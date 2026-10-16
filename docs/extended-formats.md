# Palettes, separate planes, 16-bit and float64 samples

The CLI and `inspect_raster` support these additions with uncompressed,
PackBits, TIFF LZW, and [Deflate](deflate.md) storage, in both byte orders, with strips or tiles:

| Input | Output |
| --- | --- |
| Palette TIFF, 1-, 4-, or 8-bit unsigned indices | Interleaved RGB8 |
| Separate planes, 8-bit Gray/RGB/Gray+A/RGBA | Interleaved 8-bit samples, with straight alpha when present |
| Unsigned 16-bit grayscale or RGB, contiguous or separate | Interleaved Gray16/RGB16, big-endian sample bytes |
| IEEE float64 BlackIsZero grayscale or RGB, contiguous or separate | Interleaved binary64, little-endian bytes; NPY from the CLI |

For palettes, `PhotometricInterpretation=3`, `SamplesPerPixel=1`, and a
SHORT `ColorMap` of exactly `3 * 2^BitsPerSample` entries are required. Entries
are three consecutive red, green, and blue arrays. The output channel is the
high byte of the corresponding 16-bit map entry, matching libtiff's normal
16-to-8-bit palette conversion. This intentionally quantizes palette colors.
Packed rows start on byte boundaries; unused end-of-row bits are ignored.
The reader does not implement libtiff's heuristic for nonstandard 8-bit values
stored in a nominally 16-bit color map.

`PlanarConfiguration=2` stores all segments of channel zero, then channel one,
and so on. Every plane has the same strip/tile geometry. Offset and byte-count
arrays must contain exactly `segments_per_plane * SamplesPerPixel` entries.
Missing planes and incomplete final strips are rejected. Single-channel
separate-plane files are accepted. Palette files may specify either planar
value; with one source channel the storage is identical.

16-bit output preserves all unsigned sample bits. The CLI writes P5/P6 with
`MAXVAL 65535`, with the big-endian bytes required by these formats, regardless
of input byte order. WhiteIsZero uses `65535 - value`. Inverting both bytes
implements that exact unsigned transformation. Mixed depths, signed samples, and 16-bit alpha remain unsupported. Float64 has its own bit-preserving representation described in [float64.md](float64.md).

Palette/planar/integer additions require FillOrder 1, Predictor 1, and Orientation 1. Float64 also accepts Predictor 2 for LZW and Deflate. Existing
uncompressed bilevel grayscale still supports FillOrder 2 through the original
path. Existing codec rules remain unchanged: PackBits packets cannot cross
rows; LZW supports modern early-change MSB and legacy late-change LSB streams,
with initial Clear, exact expansion, and EOI
or the explicit exact-output sub-byte ending in [the LZW profile](profile.md#lzw-profile).
Edge tiles require full padded storage. Deflate additionally accepts bounded
extra decoded data in the short final strip; its exact policy and work quota
are in [deflate.md](deflate.md). Packed 1-bit grayscale also uses this path for
Deflate and expands to Gray8. No TIFF writer was added.

## API and memory

`inspect_raster` returns `Raster` and tries the original inspector first.
The first attempt is capped at the original profile's sufficient budget (or
available work if that bound cannot fit). Its actual work is charged even on
failure. A failed attempt is passed to the extended inspector, which validates
the file independently. Error precedence is not a compatibility guarantee. The existing
`inspect`/`Image` API remains available for its original zero-scratch profile.

```rust
use verified_tiff::{inspect_raster, raster_inspection_work_limit, Budget, Limits};
let limits = Limits::conservative();
let mut work = Budget { remaining: raster_inspection_work_limit(0, &limits)? };
let image = inspect_raster(&bytes, 0, &limits, &mut work)?;
// Use fallible allocation in applications receiving untrusted files.
let mut output = vec![0; image.output_len()];
let mut scratch = vec![0; image.scratch_len()];
let mut decode_work = Budget { remaining: image.decode_work_limit() };
image.decode_with_scratch(&mut output, &mut scratch, &mut decode_work)?;
```

The core allocates nothing. Scratch contains decoded native planes without
tile padding; conversion then reads that immutable raster and writes output.
The two buffers have separate Rust borrows and cannot alias in safe Rust.
Each buffer is bounded by `Limits::max_output`; total caller image storage is
at most twice that limit. Source input and the fixed codec tables are separate.
`scratch_len()` is zero for the original path. `decode_into` remains a
zero-scratch convenience and returns `OutputTooSmall` for an extended raster.

Extended decode reserves its entire quota before writing either buffer.
For none/PackBits/LZW this is:
`scratch_len + output_len + sum(encoded_segment_bytes + expanded_segment_bytes)`.
The sum includes uncompressed data and padded storage. The conversion's
constant number of reads per sample fits this abstract, constant-factor work
model. Buffer suffixes are preserved. Insufficient work or either undersized
buffer leaves both buffers unchanged. The CLI allocates both buffers fallibly.

`bits_per_sample()` describes the source; `output_bits_per_sample()` is 8, 16 or 64. `sample_format()` distinguishes unsigned integer, signed native LogLuv16 containers, and IEEE floating-point output; `color_model()` distinguishes Gray, RGB, and LogLuv; `predictor()` reports the transform. `channels()` describes output, so palette images report three. JSON also
reports `planar_configuration` and `scratch_bytes`.

## Checked proof scope

All new executable core code is inside Verus and is included in the full-crate
`--no-cheating` run:

- The extended scalar validator has source correspondence and a local
  acceptance theorem with a sufficient lookup budget.
- Every successfully inspected image has verified source metadata, complete
  segment validation, and addressable native samples. The original segment
  coverage, PackBits, and LZW proofs are reused with byte-oriented plane geometry.
- Plane decoding, buffer slicing/composition, packed palette lookup, planar
  interleaving, endian conversion, polarity, and straight-alpha conversion
  have checked pixel specifications. Decoding succeeds with the reported
  quota and sufficiently sized output and scratch buffers, and writes exactly
  `decoded_byte` while preserving both suffixes.
- `supported_extended_profile(bytes, page, limits)` describes valid visited
  directories, supported source fields, palette maps, native/output geometry,
  per-plane segment arrays, codec streams, and resource bounds. It does not
  call an executable inspector or assume per-output-byte addressability.
  Every file satisfying it is accepted by `inspect_extended` with the proved
  sufficient budget. Segment coverage supplies sample addressability.
- `supported_raster_profile` is the union of the original, extended, JPEG, fax, WebP and LogLuv
  profiles. `inspect_raster` accepts every file in that union with its proved
  sufficient budget, while preserving the original profile's existing smaller
  budget guarantee and zero-scratch path.
- The Deflate plane validator has local acceptance, bounded inspection work,
  and exact decode-quota summation, composed with the stream acceptance proof.
- Five concrete 200-byte TIFF witnesses prove that the extended profile is
  inhabited for palette, unsigned 16-bit, separate RGB planes, Deflate Gray8,
  and Deflate bilevel. The same fixtures are decoded by runtime regressions.
  Always-error inspector mutations and removal of the public wrapper's
  extended fallback must fail verification.

## Sufficient inspection budgets

Write `D = (page + 1) * (max_tags + 1)`, `T = max_tags`,
`S = max_segments`, `I = max_input`, and `O = max_output`.
The extended bound is `D + 20*T + 4 + 4*S*(8*I + 2*O + 65)`.
It covers directory traversal, 20 metadata scans, four sample-format checks,
and at most four planes of segment validation, including Deflate transition
quotas. Since `max_segments` already bounds the total across planes, the
factor of four deliberately overestimates work; overlapping input ranges
are also permitted. This is sufficient, not a necessary budget.

`extended_inspection_work_limit` computes this bound with checked arithmetic
for direct `inspect_extended` callers. `raster_inspection_work_limit` adds the
original, JPEG, fax, WebP and LogLuv bounds to cover all six inspection attempts.
Both calculators succeed exactly when their mathematical bound fits `usize`.
A `Limit` result from a calculator does not mean that a specific image cannot
be inspected with an explicit smaller budget. Decoding still needs its own
reported `decode_work_limit()` quota.

The mathematical models still need review against TIFF itself. Independent
expected samples and the libtiff oracle test agreement with the intended
standard; no C decoder is linked into the reader.

## Coverage measurement

`python3 tests/corpus.py` enumerates all 31 unchanged TIFF files in libtiff
4.7.1's `test/images` and compares every accepted first main image with the
independent oracle. Palette/planar/16-bit support increased acceptance from
11 to 17 files (35.5% to 54.8%). Deflate raised this to 18/31 (58.1%); JPEG/OJPEG now raises it to
22/31 (71.0%). CCITT Group 3/4 raises it to **25/31 (80.6%)**,
with 21 exact comparisons and four bounded JPEG differences. Float64/Predictor 2 raises this to **27/31 (87.1%)**, with 23 exact and four bounded JPEG comparisons. The WebP subset raises this to **28/31 (90.3%)**, with 24 exact and four bounded JPEG comparisons. LZW sub-byte termination compatibility raises this to **29/31 (93.5%)**, with 25 exact and four bounded JPEG comparisons. Legacy LZW raises this to **30/31 (96.8%)**, with 26 exact and four bounded JPEG comparisons. Native LogLuv16 completes **31/31 (100%)**, with 27 exact native-output comparisons and four bounded JPEG comparisons.
This does not count all pages, SubIFDs, or regression assertions.

Newly accepted originals are `palette-1c-1b.tiff`, `palette-1c-4b.tiff`,
`palette-1c-8b.tiff`, `minisblack-2c-8b-alpha.tiff`,
`minisblack-1c-16b.tiff`, and `rgb-3c-16b.tiff`. Original source and decoded
pixel hashes, metadata, and all rejection results are recorded in
`evidence/original-corpus.json`.

The five formal witness fixtures can be regenerated with
`python3 tests/acceptance_witness.py`. Their proof-only byte models are in
`src/extended_witness.rs`; the Rust API regression uses the matching committed
files and independent expected pixels.

## JPEG integration update

The public raster union and its sufficient budget now also include
`supported_jpeg_profile` and `jpeg_inspection_work_bound`. The original and
extended API contracts and bounds above are unchanged. Public inspection
caps and charges both earlier probes before the independent JPEG inspector.
At that stage, original-file compatibility reached 22/31 (71.0%): 18 exact pixel comparisons
and four JPEG/OJPEG comparisons with a measured maximum channel error of 3. Current coverage including native LogLuv16 is 31/31 (100%): 27 exact native-output comparisons and four bounded JPEG comparisons.
See [jpeg.md](jpeg.md) for the distinct numerical and resource policies.

## Float64 integration update

The extended profile now includes IEEE binary64 grayscale/RGB, both input byte
orders, contiguous/separate planes, strips/tiles, Predictor 1 for all four
lossless codecs and Predictor 2 for LZW/Deflate. Acceptance bounds and the
caller-scratch pipeline remain intact. Two additional concrete witnesses
cover float64 Predictor 1/2. See [float64.md](float64.md) for bit semantics,
NPY output, exact exclusions and the new negative gates.

## WebP integration update

The public union includes `supported_webp_profile`; its fifth inspector is
independently bounded and charged. The four earlier probes are capped at their
respective sufficient budgets. WebP uses per-segment RGBA scratch, followed by
RGB/alpha conversion and scattering; the original extended planar pipeline
remains unchanged. See [webp.md](webp.md) for the single-group VP8L profile,
resource formula, concrete witness and negative gates.

## LogLuv integration update

The public union includes `supported_logluv_profile`. The WebP probe is now
capped independently before the sixth LogLuv inspector. LogLuv uses ten bytes
of scratch per storage pixel and emits three native 16-bit components in
little-endian order; the CLI uses NPY and labels `color_model` as `logluv16`.
See [logluv.md](logluv.md) for the exact encoding, bounds, exclusions and proofs.
This completes the 31/31 original first-main-image corpus, with native component
comparison rather than display RGB for the LogLuv fixture.
