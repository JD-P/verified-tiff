# CCITT Group 3 and Group 4

`inspect_fax_tiff` and the public `inspect_raster` reader accept Classic TIFF
Compression 3 (T.4 Group 3) and 4 (T.6 Group 4). Output is expanded Gray8, with
exact lossless pixels. Both TIFF byte orders, both FillOrder values, strips,
padded edge tiles, short final strips, and main-directory page selection work.
The original `inspect` API retains its original profile.

## Fields and representation

Input must have one unsigned one-bit sample and PhotometricInterpretation 0
or 1. Omitted BitsPerSample/SampleFormat use their defaults of 1. Orientation,
PlanarConfiguration and Predictor must be 1; ExtraSamples is unsupported.
Group 3 accepts T4Options 0, 1, 4 or 5: 1D, mixed 1D/2D, and optional EOL
byte alignment. Group 4 requires T6Options 0 and no nonzero T4Options.
Uncompressed extension modes and Compression 2 CCITT RLE are unsupported.

Coded row width is at most 65,536. There are at most 65,538 stored run endpoints
per row, including zero-length runs. Configured image dimensions, input,
output, directory, tag and segment limits also apply. Tile dimensions must be
positive multiples of 16; full padded tiles are decoded. A short final strip
contains exactly its actual rows. Every segment starts with an all-white
reference row; references never carry across strips or tiles.

Recognized metadata fields may be unordered, and entirely zero directory
entries are ignored, as in the separate JPEG compatibility parser. Duplicate
recognized fields, malformed tombstones, invalid tag ranges, and inconsistent
strip/tile arrays are rejected. These policies are part of the fax profile,
not changes to the original inspector's sorted-directory profile.

## Stream rules and compatibility

The codebooks include terminating lengths 0–63 and makeup lengths 64–2560.
Repeated makeup codes handle long runs. Each completed run must fit the
remaining row width. Group 4 and mixed Group 3 implement pass, horizontal,
and vertical offsets −3 through +3. Mixed Group 3 begins with a 1D row.
The decoder does not enforce an encoder's preferred interval between 1D rows.

Every row must be complete. Invalid codes, premature markers, missing
horizontal runs, backward/out-of-width positions, partial rows, and storage
limit violations are rejected. No missing pixels are filled or clipped.
Input reads require only the bits actually consumed: no lookahead bytes or
implicit zero bytes beyond the declared range are needed.

Group 3 EOL consists of 11–18 zero bits followed by one; the upper allowance
covers up to seven byte-alignment fill bits. When T4Options requests alignment,
the EOL's one must end a byte. Group 4 has no per-row EOL. After the exact rows,
either zero padding to the end of the current byte or a complete terminator
is accepted: six EOLs for Group 3 RTC (including 1D tags in mixed mode), or two
exact 12-bit EOLs for Group 4 EOFB. A terminator may be followed by at most
seven zero pad bits. Other trailing data is rejected.

Two compatibility choices are explicit:

* For pure 1D Group 3 with T4Options 0 only, a segment whose first eleven bits
  are not all zero is interpreted as marker-free 1D data. This is selected
  before decoding; a failed marked stream is never retried using this rule.
  All declared rows must still decode exactly and only final zero pad bits
  may remain. This supports `testfax3_bug54_1dnoEOL.tif` without general
  resynchronization or damaged-row repair.
* 2D reference state retains duplicate run endpoints, including a horizontal
  pair of zero-length runs. Reducing the reference to visible pixels loses
  this information and changes decoding of `testfax4.tiff`. In 1D rows,
  completed zero/zero pairs before row end are normalized, matching libtiff's
  longstanding run representation. Both rules are specified in the pure model
  and tested with explicit streams, not special-cased fixture names.

`testfax3_bug_513.tiff` needs no recovery: its last code ends at the last
available bit, which the bit reader handles exactly.

## Resources and API

No codec library, unsafe code, FFI, heap allocation or runtime recursion is
introduced in the core. Two fixed arrays of 65,538 `u32` endpoints occupy
524,304 bytes of array storage. They alternate roles without copying whole
rows. This is not a proof of exact compiled stack usage or total process RSS.

The caller supplies scratch equal to the largest expanded segment, one byte
per padded pixel. It is independently limited by `max_output`. Logical output
is also one byte per pixel. `scratch_len()` and `output_len()` report these
sizes. Scratch and output suffixes beyond the reported sizes are preserved.
Insufficient buffers or decode budget are rejected before either is modified.

Each segment's conservative work quota is
`64 * encoded_bytes + 4 * expanded_pixels + 64 * rows + 1`.
Inspection charges this quota before codec validation; decoding reserves the
sum before writing. These are abstract work units, not machine instructions.
Codewords and endpoint searches have fixed bounds, input consumption bounds
row transitions, and rendering/scatter is linear in stored pixels. The
source-level interpretation of the quota as a constant-factor work bound is
audited; exact instruction counts and elapsed time are not formally proved.

`fax_inspection_work_limit(page, limits)` computes the checked sufficient bound
`(page + 1) * (max_tags + 1) + 128 + max_segments * (64 * max_input + 68 * max_output + 1)`.
The public raster bound includes all four inspectors. Failed earlier probes
are capped at their own sufficient bounds and charged for actual work.
A particular file can succeed with substantially less than these conservative
bounds; the CLI retains its explicit shared work limit.

## Proofs and tests

The entire core verification includes the CCITT tries and bit order, exact
code/run acceptance, row state transitions, framing and termination, whole
segment acceptance, rendering, TIFF geometry/scatter, buffer frames, and work
conservation. `supported_fax_profile` describes bytes, metadata and pure stream
models. Every member is accepted by `inspect_fax_tiff` with its sufficient
budget. `supported_raster_profile` includes it, preserving the previous three
profiles and the original smaller-budget guarantee.

Two concrete 101-byte TIFFs prove whole-profile non-vacuity for Groups 3 and 4;
matching Rust fixtures decode to one white pixel. Negative proof gates replace
the stream/image readers with always-error implementations, remove pixel
writes, and remove public fax dispatch; each must fail its postconditions.

`tests/fax.py` covers independently generated encodings, independent libtiff
encodings, exact known pixels, both byte orders/fill orders/polarities,
strips/tiles, pages, makeup runs, zero runs, termination, malformed streams and
resource limits. All three original fax files are compared byte for byte with
libtiff. `tests/fax_api.rs` checks quotas and both buffer suffixes. The mutation
smoke target includes fax seeds and reports successful fax inspection/decode.

The mathematical codebook and run model still require independent review
against T.4/T.6 and TIFF 6.0. Differential comparisons are evidence of observed
compatibility, not proof that every prose-standard interpretation is correct.
