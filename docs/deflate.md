# Deflate TIFF decoding

`inspect_raster` and the CLI accept TIFF Compression **8** (Adobe Deflate) and
**32946** (the older Deflate identifier). Both mean a separate **zlib-wrapped**
Deflate stream for each strip or tile. Raw Deflate and gzip are not accepted.
The original `inspect`/`Image` API retains its original compression profile.

The new codec works with all currently supported raster representations:
1-bit grayscale, 8-bit grayscale/RGB and alpha, 1/4/8-bit palettes, separate
planes, and unsigned 16-bit grayscale/RGB. It supports both TIFF byte orders,
strips and padded tiles. Deflate fields have their own RFC-defined bit order,
independent of TIFF endianness. FillOrder, Predictor, and Orientation must be 1.
Predictor 2, 16-bit alpha, BigTIFF, JPEG, and CCITT remain unsupported.

## Stream and final-strip policy

The reader implements RFC 1951 stored, fixed-Huffman, and dynamic-Huffman
blocks, canonical code assignment, literal/length and distance codes, and
LZ77 overlapping copies. A fixed 32,768-byte circular buffer retains history
across blocks. Each strip/tile starts a new history. Distances must refer to
already produced bytes and fit the window declared by the zlib header.

The zlib CMF/FLG check, compression method, window size, and Adler-32 checksum
are validated. Preset dictionaries are rejected. Truncation, reserved block
and data codes, oversubscribed trees, invalid repeats, missing EOB, and invalid
back-references are rejected. Code-length trees must be complete; literal and
distance alphabets also permit the RFC's one-symbol, one-bit exception. An
empty distance alphabet is accepted only when no distance is decoded.
The final byte's unused bits are ignored. Extra bytes after the zlib checksum,
including concatenated streams, are rejected within a declared TIFF segment.

Every full strip and tile must expand to its exact storage size. A short final
strip may expand past its logical rows, up to
`row_bytes * min(RowsPerStrip, ImageLength)`. Extra expanded bytes are validated
and checksummed but produce no output pixels. Thus omitted/oversized
RowsPerStrip does not authorize arbitrarily large padding beyond the image.
A segment's expansion cap must fit `max_output`, including tile padding.

The original `deflate-last-strip-extra-data.tiff` has 500 x 500 Gray8 pixels
and RowsPerStrip=16. Its last stream expands to 8,000 bytes; only the first
2,000 are logical image data. The reader validates all 8,000 bytes and emits
the same 250,000 image bytes as libtiff.

## Memory and work

The allocation-free core borrows the immutable source and uses caller-owned
native-image scratch, followed by the existing verified pixel conversion.
Scratch and output are each independently limited by `max_output`. Fixed
Huffman state and the 32 KiB history are additional stack state; this is not a
claim about exact compiled stack size or process RSS. There is no runtime
recursion, unsafe codec, C call, or new Cargo dependency.

For a segment with encoded byte count `C` and expansion cap `B`, the interpreter
allows `F = 8*C + 2*B + 64` state transitions. Inspection reserves `F+1` work
units before validation. The successful descriptor's decode quota is:

`scratch_len + output_len + sum(F + 1)`

All arithmetic is checked. The entire decode quota is reserved before either
caller buffer is modified. Undersized buffers or insufficient decode work
leave both buffers unchanged. Suffixes beyond the reported lengths are
preserved. Padding counts toward the cap and work reservation even if unused.
The quota is a conservative, constant-factor cost model, not an instruction
count. Huffman tables now carry a fixed 288-entry `u16` symbol-order cache and
16 `usize` bucket offsets. Stable counting order is built with 16 passes over
288 lengths, once per table; symbols then use an indexed lookup after at most
15 code bits. Dynamic headers still contain at most 320 code lengths. These
are fixed helper bounds, not new input-dependent allocations. The cache adds
704 bytes of declared fields per table on a 64-bit target (two persistent
tables per cursor); compiler temporaries and exact stack usage are not proved.

## Checked proof scope

All four new core modules are included in the full-crate Verus gate:

- `deflate_bits.rs`: LSB-first integer fields, MSB-first canonical Huffman
  decoding, code counts, tree validity, and exact successful parse results.
  The cache is proved equal to the original pure `ranked` search, including
  empty/single-symbol alphabets and missing ranks. Aligned byte reads are proved
  equal to eight modeled bit reads. Neither shortcut changes stream acceptance.
- `deflate_header.rs`: dynamic code-length order and repeat expansion,
  fixed/dynamic tables, and parsing success equivalence to the pure model.
- `deflate.rs`: a pure state machine over input bytes and a mathematical
  byte-string history. The executable step refines that model. The circular
  buffer retains exactly the available 32 KiB suffix, including overlapping
  copies and wraparound; Adler accumulators match the modeled history.
  Validation succeeds exactly when the bounded model returns a complete
  checksummed stream. A concrete stored-stream witness establishes that the
  model is inhabited.
- `deflate_plane.rs`: expansion limits, original-byte stream selection,
  logical sample coverage, padding exclusion, and scatter correctness.
  Replaying a validated stream terminates successfully and produces the
  modeled native bytes, which compose with the existing output conversion.

Successful inspection therefore gives the existing `Raster` guarantees:
source metadata correspondence, exact modeled output pixels, successful decode
with sufficient buffers/quota, and buffer framing. The full original
`supported_profile` inspection-acceptance theorem is preserved. Its layout
parsing was factored into a separately verified helper to keep solver work
bounded, with unchanged runtime checks and work accounting.

The whole-inspection acceptance theorem now covers the extended path,
including Deflate: `supported_extended_profile` plus its sufficient inspection
budget implies successful inspection, and `supported_raster_profile` covers
the public wrapper. The Deflate plane validator proves acceptance and exact
quota summation; concrete full-TIFF Gray8 and bilevel witnesses connect the
stream model to the inspector's profile. See [extended-formats.md](extended-formats.md).

## References and tests

Primary format specifications:
- https://www.rfc-editor.org/rfc/rfc1950.html
- https://www.rfc-editor.org/rfc/rfc1951.html

`tests/deflate.py` covers all representations and both compression IDs, block
kinds, multi-block streams, circular-buffer wraparound, maximum distance,
malformed headers/trees/repeats/back-references, checksums, truncation,
expansion caps, padded final strips, pages, and budget boundaries. The runtime
API tests check exact quotas and both buffer suffixes. A negative proof check
replaces the Deflate validator with an always-error body and requires Verus
to reject its stream acceptance postcondition. The original libtiff
regression is committed unchanged with provenance and hashes.

Committed generated API/fuzz seeds can be regenerated with
`python3 tests/deflate.py --write-seeds`. They are independent of the upstream
fixture; `scripts/upstream.py` retains the pinned libtiff source baseline.

## Combined validation and decoding

`decode_deflate_into(input, page, limits, output, scratch, budget)` accepts the
Deflate subset of the existing extended profile. Its public acceptance theorem
uses `supported_deflate_profile`; the checked `deflate_decode_work_limit`
returns a sufficient work budget (or `Limit` on arithmetic overflow). Buffers
of `limits.max_output` bytes are sufficient; callers may pass smaller buffers
when they already know the actual output and scratch lengths. Successful
output has exactly the same pixel model as `inspect_extended` followed by
`decode_with_scratch`.

For strips, each stream is validated and decoded once into its native scratch
slice. The entire zlib stream, checksum and permitted final-strip padding are
still checked. Tiles currently use the existing validate/decode fallback.
Other compression tags return `Unsupported`. Failure leaves the final output
unchanged but may have written a scratch prefix. Successful calls preserve
both buffer suffixes; errors preserve scratch beyond `limits.max_output`.
The original inspection API still validates compressed data before returning.

The work accounting deliberately retains the conservative validation-plus-
decode charge; fewer executed instructions do not weaken resource bounds.
The combined sufficient bound is twice the extended inspection bound plus
`2 * limits.max_output`. This is a guaranteed upper bound, not actual runtime.
No heap allocation, unsafe operation, external decoder or new proof assumption
is introduced. Applications must explicitly use the combined API to receive
this whole-image API's one-pass benefit; the existing CLI retains its API.
Separately, complete native Deflate segment reads through the C adapter now
decode and validate once into private staging, preserving failure atomicity.

The shared Deflate cursor now reduces Adler-32 accumulators every 256 bytes,
with proved overflow bounds and equivalence modulo 65521. Buffered fields and
canonical Huffman decoding read a bounded word when enough input remains;
short-tail fallbacks retain the original stream semantics. Ordinary strip
decoding also writes contiguous slices, avoiding per-byte scatter geometry.

## Resumable native batches

`native_stream::Inflater::pull` rebinds the caller's bounded input window once
and runs multiple existing `Cursor::step` transitions. Its pure `batch` relation
stops at output capacity, validated completion, or a nonfinal-window refill
boundary. Exact success/failure, emitted bytes, final model, cursor, work and
output-suffix contracts are checked. Every transition costs two units, retaining
the former adapter-loop-plus-step charge. The scalar API and its proofs remain.

The adapter supplies consecutive 64 KiB windows and keeps 1024 bytes of lookahead
before starting a transition in a nonfinal window. Input continuity remains a
tested adapter responsibility. Full native segment requests stage all requested
bytes, check the checksum, exact ending and permitted short-final-strip padding,
and only then return. Partial requests still validate the complete segment and
replay into bounded row buffers. Work-limit reservations are unchanged. Independent
stored/fixed/dynamic zlib fixtures test window/output boundaries, strict endings,
work exhaustion, output caps and unchanged suffixes; C tests check unchanged
caller storage after failed partial and complete reads.

The native batch loop now groups consecutive mode-3 match emissions. A checked
`Cursor::match_byte` refines exactly one original state-machine transition; the
inner run preserves overlapping/ring-wrapped history, checksum updates, output
suffixes and two-unit charging. Completion, control transitions, output-cap
failures and input-window refill behavior retain the original batch model. This
is scalar run specialization, not an unchecked bulk copy.
