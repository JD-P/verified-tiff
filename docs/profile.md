# Supported TIFF profile

The [WebP SSE2 pass](webp-simd.md) preserves accepted representations, exact integer pixels, stream endings, work limits and output suffixes. It adds a documented trusted integer-operation boundary and retains scalar fallbacks.


The implementation uses TIFF 6.0 (June 3, 1992), particularly the bilevel/grayscale/RGB
descriptions, field definitions, and tiled-image extension. The specification
is linked from https://libtiff.gitlab.io/libtiff/specification/index.html and
mirrored at https://www.itu.int/itudoc/itu-t/com16/tiff-fx/docs/tiff6.pdf .
Tag lookup is modeled independently as the first matching directory entry;
the executable directory validator additionally requires ascending unique tags.

The CLI and `inspect_raster` additionally implement palettes, separate planes,
unsigned 16-bit grayscale/RGB, and [Deflate](deflate.md) for all representations. Their exact representation, memory policy,
and proof scope are specified in [extended-formats.md](extended-formats.md).
JPEG/OJPEG and YCbCr use the separate [JPEG profile](jpeg.md).
CCITT Group 3/4 bilevel input uses the separate [fax profile](fax.md).
IEEE float64 and Predictor 2 extend the caller-scratch profile as specified in [float64.md](float64.md).
Lossless WebP Compression 50001 uses the separate [VP8L subset](webp.md).
SGILog32 LogLuv uses the separate [native LogLuv16 profile](logluv.md).
The sections below describe the original `inspect`/`Image` profile.

## Original zero-scratch image representation

- Classic TIFF magic 42, II or MM byte order. BigTIFF magic 43 is unsupported.
- Positive width/height, within configured limits. SHORT and LONG dimensions
  are supported; conversion and all products are checked.
- Compression 1 (also its default), 32773 (PackBits), or 5 (TIFF LZW); unsigned 1-bit black-and-white, 8-bit
  grayscale, or interleaved 8-bit RGB, with optional 8-bit alpha on grayscale/RGB.
  PhotometricInterpretation is 0 or 1 for grayscale (one/two channels), and
  2 for RGB (three/four channels).
- BitsPerSample is a SHORT array with one value per channel: 1 for bilevel,
  or 8 for every grayscale/RGB/alpha sample. Its omitted default of 1 is accepted for a
  single-channel image. Mixed depths and 1-bit RGB are unsupported.
- FillOrder defaults to 1 (leftmost pixel in the most significant bit).
  Bilevel also supports FillOrder 2 (leftmost pixel in the least significant
  bit), independently of the file's byte order, for uncompressed data.
  PackBits, LZW, and all eight-bit data require FillOrder 1. TIFF 6.0 recommends
  FillOrder 2 only for uncompressed and CCITT bilevel data because other
  combinations can be ambiguous.
- SamplesPerPixel defaults to 1; PlanarConfiguration, Orientation,
  Predictor, and SampleFormat must have their supported value 1 when present.
  SampleFormat is a SHORT array with one value per channel.
- ExtraSamples, when present, must be SHORT with exactly one value: 1
  (associated alpha) or 2 (unassociated alpha). The alpha sample follows the
  one grayscale or three RGB samples; SamplesPerPixel is therefore 2 or 4.
  Without ExtraSamples, only one/three channels are supported. Unspecified
  extra channels (value 0), empty/multiple extras, and 1-bit alpha are unsupported.
  No color management or ICC profile transformation occurs.
- Strips use RowsPerStrip (default 2^32-1), StripOffsets, and StripByteCounts.
  The final strip needs only its actual rows, including when RowsPerStrip is
  larger than ImageLength. Each strip can be placed anywhere in the input.
- Bilevel scanlines each start on a byte boundary. Packed row storage is
  `ceil(block_width / 8)` bytes; unused trailing bits are ignored. Output is
  expanded Gray8: 0 for black, 255 for white, respecting either photometric
  polarity. `max_output` and output work count these expanded bytes.
- Tiles require nonzero TileWidth/TileLength, multiples of 16, TileOffsets,
  and TileByteCounts. Edge tiles occupy their full padded storage rectangle;
  only the logical image rectangle is returned.
- Offset/count arrays must have exactly the computed number of segments.
  SHORT and LONG arrays are accepted. TileOffsets encoded as SHORT are a
  documented permissive extension beyond TIFF 6.0's LONG type for that field.
- Declared segment ranges must fit the input. Uncompressed segments may
  declare more bytes than their required storage; extra bytes are ignored.

## Alpha representation and conversion

TIFF 6.0 Section 18 and ExtraSamples define the input alpha convention. The
reader accepts both associated (premultiplied) and unassociated (straight)
8-bit alpha, for both grayscale polarities and RGB, with strips or tiles and
any supported compression. Alpha is opacity: 0 transparent, 255 opaque.
BitsPerSample and SampleFormat arrays include the alpha channel.

The output is interleaved straight-alpha Gray+A or RGBA. Alpha is copied
unchanged. For associated alpha `a > 0`, each stored color `v` is converted to
`min(255, floor((255*v + floor(a/2)) / a))`: nearest rounding with ties upward.
At associated alpha zero, stored colors are ignored and output color is zero.
For unassociated input, stored colors are unchanged, including hidden colors
at zero alpha. WhiteIsZero grayscale is inverted after unassociation (except
the associated-zero case); the alpha sample is never inverted.

TIFF permits associated colors greater than alpha. These files are accepted;
conversion to the chosen 8-bit straight representation clips values above 255.
It is therefore not a lossless representation of all TIFF associated colors,
and unassociation of quantized samples does not recover original colors exactly.
There is no compositing, gamma adjustment, or color-profile transformation.

Storage decoding writes native samples into caller output. A second in-place
pass normalizes colors while preserving each following alpha sample. This adds
`output_len()` units to the decode quota and requires no image scratch buffer.
The entire quota is reserved before alpha output is written. This applies to
uncompressed alpha images as well as compressed ones. `max_output` counts all
color and alpha bytes. Inspection's sufficient bound allows four channel checks.

The CLI writes P7 PAM with `MAXVAL 255`, depth 2/4, and tuple type
`GRAYSCALE_ALPHA`/`RGB_ALPHA`, preserving transparency. PAM describes straight
foreground colors: https://netpbm.sourceforge.net/doc/pam.html . Existing
non-alpha images continue to produce P5/P6. JSON distinguishes `source_alpha`
(`none`, `associated`, `unassociated`) and `output_alpha` (`none`, `straight`).

## PackBits profile

PackBits packets follow TIFF 6.0 Section 9: controls 0 through 127 copy the
following 1 through 128 bytes; controls 129 through 255 repeat the following
byte 128 through 2 times; control 128 is a no-op. The core interprets these
as unsigned control bytes to avoid signed-overflow corner cases.

Every segment must expand to exactly its declared geometric storage: packed
byte-padded rows for bilevel, and interleaved sample bytes for Gray8/RGB8/Gray+A/RGBA.
Packets may not cross a row boundary. Full edge-tile storage is required even
when most of it is outside the visible image. Short final strips use only
their actual rows. Leading/interspersed/trailing no-ops are accepted.

Missing literal payloads, missing repeat values, insufficient expansion,
overruns, row-crossing packets, and extra packets producing pixels after the
expected storage are rejected. Only no-ops may remain after exact expansion.
This is stricter than libtiff's tolerant behavior for some malformed streams.
Predictor must still be 1; horizontal differencing is not part of this codec.

Inspection charges one unit per segment plus its encoded byte count.
`decode_work_limit()` is the logical output size, an additional logical output
size when alpha is present, and the sum of encoded and expanded segment storage sizes. Checked sums must fit `usize`. The decoder
reserves this quota before writing compressed output, so insufficient work
leaves the output unchanged. Padding costs work but never becomes output.
The core has a fixed-size packet cursor and writes directly into the caller's
buffer, with no heap allocation or full-segment scratch buffer.

## LZW profile

Modern TIFF LZW follows Section 13: each strip/tile begins with Clear (256), codes
are packed most-significant-bit first, and EOI (257) terminates the stream.
Codes begin at 9 bits. The decoder switches to 10, 11, and 12 bits immediately
after defining entries 510, 1022, and 2046 respectively (TIFF early change).
Bytes are decoded independently of the TIFF header's II/MM byte order.

The initial dictionary contains the 256 literal bytes. New entries are the
previous phrase followed by the first byte of the current phrase. The special
next-entry reference (often called KwKwK) expands the previous phrase followed
by its own first byte. Clear resets dictionary state and width, including
between phrases or after the expected output. Repeated Clears are accepted.
Each strip or tile has its own dictionary; phrases may cross row boundaries.

The table has 4,096 entries. Codes through 4095 are accepted; after the table
fills, another modern data code requires Clear first. EOI is also allowed immediately
after a full table. This permits the last 12-bit entry beyond the specification's
recommended encoder Clear point. The decoder never uses a 13-bit code, grows
the table, or follows a prefix cycle.

Old-style libtiff LZW is also supported. Each segment selects this variant
exactly when its first two bytes satisfy `byte[0] == 0 && (byte[1] & 1) == 1`,
the LSB-packed initial Clear signature. Its codes are least-significant-bit
first, and widths increase after defining entries 511, 1023 and 2047. The
initial Clear is still mandatory. Selection is fixed for the segment, including
after Clear resets; it is independent of TIFF byte order and is not a trial
parse or fallback after an error. Different segments may select different
variants. GIF containers, PDF streams and MSB-packed late-change hybrids are
not part of this TIFF profile.

At 4,096 entries, the legacy dictionary freezes: existing codes remain valid,
no new entries are stored, and codes stay at 12 bits until Clear. The historical
libtiff decoder can append unaddressable entries above 4095; omitting those
entries preserves output for its valid 12-bit references. This reader does not
copy libtiff's eventual limit on those unreachable entries: continued references
to the frozen table remain accepted within the geometric and work bounds.
Modern LZW retains its full-table Clear requirement. The unchanged
`quad-lzw-compat.tiff` contains 77 legacy strips, including 4,549 data codes
after saturation, and expands to 512×384 RGB pixels. All 589,824 output bytes
match an independent string-dictionary decoder and libtiff exactly.

Expansion must equal the segment's geometric storage exactly, including all
padded edge-tile bytes. Missing initial Clear, truncated data codes, future
or invalid references, and insufficient/excess output are rejected. After a
complete EOI, only that byte's unused bits (low for modern, high for legacy) may remain; their values are
ignored. Whole trailing bytes are rejected.

There is one explicit compatibility exception to requiring a complete EOI:
immediately after a data code has produced **exactly** the required storage,
the segment may end with **0–7 residual bits**. Their values are ignored; this
also covers an EOI truncated within that final byte. There must be no whole
unconsumed byte. If at least eight bits remain, parsing continues and requires
valid Clear/EOI framing; data that would expand beyond the exact size is still
rejected. A Clear resets the previous-data state, so a trailing Clear requires
a complete EOI and cannot use this exception. No pixels are synthesized or
discarded to recover damaged streams. This rule applies uniformly to LZW
strips/tiles and the original/extended representations, including alpha and
float64; it does not alter code widths, dictionary rules, or work quotas.

The unchanged libtiff `lzw-single-strip.tiff` motivates this rule: its 76,042-byte
strip expands to all 3,043,950 packed bytes for a 7,795×3,122 WhiteIsZero scan,
then has just five bits (`01000`) of a 10-bit EOI left. The independent reference
and libtiff agree on all 24,335,990 expanded Gray8 pixels. Completing that EOI
does not change the output; removing another input byte loses required pixels
and is rejected. Arbitrary sub-byte bits cannot be distinguished from padding
at this boundary; accepting them is a documented compatibility choice, not a
claim that the file contains a standards-complete end marker.

This remains stricter than libtiff paths that stop once the requested pixels
have been produced regardless of remaining encoded bytes.
FillOrder and Predictor must both be 1. Horizontal differencing is not included.

Inspection checks phrase lengths using a fixed prefix/length/first-byte table;
it does not materialize expanded pixels. Decoding uses the same table and a
fixed 4,096-byte phrase buffer, then scatters visible samples into caller output.
All arrays have constant bounds, prefix indices strictly decrease, and no
runtime recursion or heap allocation is used. No exact compiled stack-size
claim is made. Clear changes the active table length instead of clearing all
4,096 slots again; stale entries cannot be referenced.

Work accounting is the same as PackBits: inspection charges one segment unit
plus encoded bytes; decode reserves logical output, alpha normalization when
present, and encoded and expanded storage bytes. The abstract segment unit covers fixed-size dictionary setup,
and code processing has a bounded 12-bit cost. The quota is a constant-factor
work bound, not a count of instructions or literal array writes. Decoding
rejects insufficient work before writing any output.

## Directory and metadata policy

Directories and payloads can precede or follow one another. Read-only ranges
may overlap or be shared. Entries must be strictly ascending by tag identifier;
duplicate and unsorted entries are rejected. Mixed strip/tile layouts are
rejected. The scalar enum tags require SHORT; dimensions and RowsPerStrip
also permit LONG.

Known TIFF field types 1 through 12 and IFD type 13 have checked widths and
payload ranges. Unknown field types are unsupported. Unknown/private tags of
known type are range-validated and otherwise ignored. Inline values are used
at payload size <=4; larger values follow their file offsets. No metadata
payload is copied or interpreted as a nested directory.

Page selection follows exactly the requested number of main-IFD links, limited
by both max_directories and work. It does not walk the rest of the file or prove
global acyclicity. A cyclic chain can return the same directory for another
in-budget page index; all such requests still terminate. Page zero of a file
with an unvisited malformed later directory can decode successfully. This is
an image-selection contract, not whole-file certification.

The core deliberately does not enforce every TIFF alignment or metadata
recommendation. All ranges are accessed bytewise, including unaligned ranges.
See `src/lib.rs` for the checked supported-field conditions and declarative
metadata/pixel specifications. Missing required fields and unsupported features
use structured errors; error precedence is not a stable compatibility promise.

## Not implemented by the original whole-image API/CLI

The newer [C reader/native primitives](native-compatibility.md) add BigTIFF,
SubIFDs, seekable segment I/O, float32/signed/higher-bit-alpha native samples,
integer prediction and libtiff orientation handling. The exclusions in this
section and the complete acceptance theorem below describe the original
`Image`/`Raster` API and CLI. They do not describe the expanded C pipeline.


Other compression codecs outside the separate JPEG, fax, WebP and LogLuv profiles,
BigTIFF, 1-bit or higher-depth alpha, other bit
depths beyond the documented palette and unsigned 16-bit and IEEE float64 extensions, float32 samples, Predictor 3, CMYK, orientation transformations,
recursive SubIFDs/EXIF, writing TIFF, and streaming/mmap I/O. A partial
read-only [C ABI adapter](c-api.md) is now available outside the verified core;
its supported operations and proof boundary are explicit.
The converter writes PGM/PPM/PAM for integers and NPY for float64. `--inspect`
validates complete segment storage and derives sample addressability for
uncompressed images; it performs full stream validation for
PackBits/LZW/Deflate/JPEG/OJPEG/fax/WebP/SGILog32 images.

Uncompressed 8-bit strips decode by contiguous row copies, with byte inversion
for WhiteIsZero and the existing alpha normalization pass when needed. Packed
bilevel and tiled inputs retain the scalar decode path. These implementation
choices preserve the profile, exact pixels, buffer frames and work quotas.
The conservative inspection quota still charges expanded output length even
though inspection no longer reads every uncompressed sample. Plain
uncompressed decode with insufficient work retains its partial-prefix writes;
compressed and alpha decode still reserve their full quota before writes.

## Formal acceptance contract

`supported_profile(bytes, page, limits)` in `src/lib.rs` formalizes this
selected-image profile: valid visited directories and field encodings,
supported scalar/array values, positive dimensions, permitted storage layout,
complete declared segment ranges, valid row-contained PackBits packets or
TIFF LZW streams where applicable, and configured/machine resource bounds.
All arithmetic in the predicate uses mathematical integers; conversions and
intermediate storage products must fit the target's `usize` where required.
The predicate does not execute the parser or require successful sample
preflight. A separate lemma derives per-sample addressability from segment
storage, including full padded tiles and the final short strip.

`inspect` is proved to succeed whenever this predicate holds and initial work
is at least `inspection_work_bound(page, limits)`. That sufficient bound is
available to ordinary Rust through `inspection_work_limit`. It is conservative;
inspection can also succeed with a smaller budget. The existing successful
metadata/pixel and malformed-input safety guarantees remain in force.

The extended profile and public raster union now have equivalent
whole-inspection acceptance and checked sufficient-budget contracts. See
[extended-formats.md](extended-formats.md#sufficient-inspection-budgets) for
those predicates, formulas, and the bounded fallback policy.

## C reader additions

The separate C reader now has local verified Predictor 3 and CCITT RLE APIs,
typed/custom metadata, EXIF/GPS directories, additional RGBA colors and
temporary metadata contexts. These additions do not broaden the whole-image
profile or its acceptance theorem described above. See
[consumer-compatibility.md](consumer-compatibility.md) for the exact boundary.

## Additional native C codecs

The C reader accepts Lerc2 v4 and static WebP VP8/VP8L with alpha through new
verified native APIs. See [LERC](lerc.md) and [WebP](webp-native.md) for exact
formats, limits, work and sample policies. The original whole-image/CLI profile
above and its supported-profile theorem retain their stated scope.


## Additional read-side APIs and scientific samples

The C reader adds lazy strile tables, bounded uncompressed/PackBits/LZW/Deflate
range reads, cropped RGBA output, custom-directory/tag hooks, public numerical
color helpers, packed integer widths, IEEE16/24 and complex native samples.
See [reader-features.md](reader-features.md) for the exact combinations and
remaining complete-segment fallbacks. The original whole-image profile above
is unchanged; the new core contracts are local step/sample refinements.

The [Deflate/LERC optimization](reader-optimization.md) changes execution cost,
not accepted formats or resource limits. Deflate batches refine the existing
transition model; complete native segment requests use private staging to avoid
replay. Partial requests retain bounded memory and full validation before exposure.
LERC's top-bit search retains its original integer/software-IEEE semantics.

The [RGBA optimization](rgba-optimization.md) specializes contiguous native
8/16-bit RGB/gray rows without changing accepted formats or presentation pixels.
Its row model proves exact components and buffer frames. Segment selection,
memory admission and TIFF/C composition remain outside that local proof.

The [kernel optimization](kernel-optimization.md) changes execution strategy,
not accepted formats: packed RGBA lookup tables, Lab gamma caching, complete
LZW staging, Deflate match runs, fax codebook borrowing, scalar specialization,
VP8/VP8L kernels, LERC software rounding and Predictor 3 coordinate traversal.
Strict stream endings and conservative work/allocation limits are preserved.

The [LERC word-arithmetic pass](lerc-arithmetic.md) preserves all accepted
representations and exact numerical models. The normal word path and wide
fallback are proved equivalent to the previous sample calculation; no fused
multiply-add or host floating-point conversion is introduced into the core.

The current [hardware FP pass](native-float.md) keeps those numerical models
and uses five explicitly trusted scalar primitives for finite LERC arithmetic.
WebP also uses the eleven trusted integer lane operations and CPU dispatch in
[the SSE2 boundary](webp-simd.md), with the original scalar fallback.
Decoder acceptance, sample contracts and resource limits remain unchanged.

The [LERC throughput pass](lerc-throughput.md) preserves the native acceptance,
exact bit models, strict endings, conservative work and suffix contracts. It
changes execution and temporary storage only; no new codec or trusted operation
is introduced.

## Current adapter contracts

The [adapter proof extension](adapter-proofs.md) supersedes earlier statements that all metadata conversion, crop placement, window bookkeeping and allocation arithmetic are unverified. The listed executable decisions now have checked contracts. C pointer validity, varargs extraction, ownership/event matching, I/O correctness and complete pipeline composition remain outside those local proofs. No new trusted primitive or format is introduced.


## Codec metadata and ink consistency

The [metadata consistency pass](metadata-contracts.md) adds the missing codec
and fax reception schemas and fixes the special InkNames getter ABI. Image
InkNames payloads use a verified exact acceptance/counting helper, and the
adapter derives NumberOfInks before publishing directory state. The fresh
whole-crate run has 1,809 verified obligations and zero errors. The counting
contract does not turn C varargs, schema extraction or directory publication
into end-to-end verified behavior; those remain covered by integration and
sanitizer tests. All emitted C function/line/branch coverage remains 100%.
