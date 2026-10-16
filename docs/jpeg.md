# JPEG, legacy OJPEG, and YCbCr

The reader implements an allocation-free, Verus-verified sequential JPEG decoder
and TIFF integration. Neither libjpeg nor libtiff is linked into the Rust core.
`inspect_raster` selects this path automatically; `inspect_jpeg_tiff` exposes it
directly. `inspect_jpeg` also accepts a standalone JPEG with an explicit caller
choice of RGB versus YCbCr interpretation. The standalone entry point has
verified successful decoding; the whole-inspection completeness theorem is
provided for the TIFF/JPEG entry point.

## Supported profile

- Classic TIFF, either byte order, Compression 7 (JPEG) or 6 (old JPEG).
- Eight-bit unsigned BlackIsZero grayscale, RGB, or YCbCr; one or three
  contiguous channels, orientation 1, predictor 1, fill order 1, no alpha.
- Sequential Huffman JPEG, SOF0 or eight-bit SOF1, one complete interleaved SOS.
  Eight-bit nonzero quantization tables; DC categories through 11 and AC
  categories through 10. Accumulated quantized DC lies in -2048..2047.
- TIFF YCbCr sampling factors 1, 2, or 4 in either direction, with their
  product at most 4. Chroma components have sampling 1x1. Grayscale MCU
  geometry is 8x8, regardless of redundant frame sampling factors.
- Strips, including short final strips, and padded tiles with dimensions
  divisible by 16. Segment dimensions must agree with the TIFF geometry.
  A missing RowsPerStrip means one strip. Per-segment dimensions are at most
  65535; logical image dimensions also obey caller limits.
- Modern JPEG accepts complete per-segment streams or abbreviated streams
  sharing a table-only JPEGTables SOI/EOI sequence. Per-segment tables can
  override shared definitions. APP and COM metadata are skipped by length.
- OJPEG accepts raw entropy with per-component JPEGQTables/JPEGDCTables/
  JPEGACTables offsets, or a JPEGInterchangeFormat header containing the
  frame/tables with an SOS in that header or the first strip. Complete
  per-segment headers are also understood. This describes specific legacy
  layouts, not every historical OJPEG variation.
- DRI restart intervals and exact cyclic RST0..RST7 markers are supported.
  A restart resets every component's DC predictor. Each TIFF segment begins
  with zero predictors and a fresh restart sequence.
- Modern streams require all-one final pad bits, EOI, and no trailing bytes.
  Legacy segments permit arbitrary final pad bits and an implicit end at the
  declared byte count; explicit EOI must still end that range exactly.
- To read the legacy corpus, this path permits unordered directories and
  all-zero 12-byte tombstones. Duplicate recognized fields are rejected.
  Other nonzero entries still require known TIFF field types and valid ranges.
  Duplicate ignored fields have no effect. The other TIFF profiles retain
  their ascending, unique-entry policy.

Progressive, arithmetic-coded, lossless, multiple/non-interleaved scans, 12-bit
JPEG, CMYK/YCCK, alpha, custom luma coefficients, other reference ranges,
color management, EXIF orientation, and JPEG 2000 are outside this profile.

## Output and numerical policy

Output is Gray8 or RGB8. Each dequantized 8x8 coefficient block passes through
a separable fixed-point inverse DCT, using constants
`round(2^14 * C(u) * cos((2*x+1)*u*pi/16))`. The two sums use exact signed integer
arithmetic, followed by level shift, nearest rounding, and clipping to 0..255.
All intermediate bounds and the exact integer result are proved. The proof
does not establish a real-cosine approximation bound or IEEE 1180 conformance.

Chroma uses nearest replication. YCbCr coefficients are the default TIFF luma
weights 299/1000, 587/1000, 114/1000; the executable green conversion uses the
specified six-decimal fixed-point coefficients. ReferenceBlackWhite accepts
the default `[0,255,128,255,128,255]` or limited range
`[16,235,128,240,128,240]`, represented by supported integer/rational tags.
Conversion, rounding, and clipping are exact relative to this integer model.
ChromaPositioning must be its supported value 1. This is a documented nearest
sampling policy, not a claim to reproduce libtiff's filtered RGBA renderer.

Independent scalar libjpeg-turbo integer-IDCT comparisons bound observed
channel differences to 3 in the generated suite. The four original libtiff
fixtures also differ by at most 3 from native-component libtiff decoding with
matching nearest chroma and independent color conversion. These are measured
results for the tested corpus, not a universal numerical theorem. The
one-pixel zero-coefficient witness produces exactly 128.

Generated TIFF comparisons decode the writer's original JPEG segments with
libjpeg before wrapping, then scatter and convert them independently. This
avoids libtiff's legacy RGB-to-YCbCr reinterpretation and raw-adapter differences
for some sampling layouts. Unchanged upstream files additionally exercise
libtiff's TIFF parsing and OJPEG reconstruction, independently of our writer.
Both references are separate test programs, never production dependencies.

## Proof and resource policy

All `jpeg*.rs` core modules are verified in the full crate. The chain covers:

1. Raw marker lengths, quantization and canonical Huffman tables, MSB-first
   entropy bits and FF00 stuffing, with exact local success conditions.
2. DC differences, AC runs/EOB/ZRL, zigzag positions, dequantization, restart
   handling and exact stream termination, refined from byte-level pure models.
3. Every block's integer IDCT pixels, subsampled coordinates, color conversion,
   strip/tile scatter, and preservation of output/scratch bytes outside the
   declared regions.
4. Source-derived TIFF metadata and segment descriptors, whole-image acceptance
   for `supported_jpeg_profile`, and the three-profile public raster union.

The model contains unbounded mathematical coefficient histories as erased
proof state. Runtime validation keeps fixed tables/predictors and one block;
decoding uses caller-owned scratch for block pixels and one segment's converted
pixels. No input-sized internal heap allocation or runtime recursion occurs.

For a segment containing B coefficient blocks and P converted bytes, scratch
is `64*B + P`. The maximum segment scratch and full logical output are each
bounded separately by `max_output`. Their sum can exceed that individual cap.
Standalone JPEG uses `64*B` scratch and work `3136*B + P`. TIFF decode work is
summed over segments as `2*encoded_bytes + 1 + 3136*B + 2*P`, accounting for
header replay and copying. Padding consumes work even when it is cropped.
Work is a conservative abstract quota, not a literal instruction count.

Decode accepts exactly when output, scratch, and its reported work quota are
sufficient. It reserves the quota before writing; insufficient buffers/work
preserve the caller buffers. Use `scratch_len()` and `decode_work_limit()`.

The checked sufficient inspection budget is:

```
(page + 1) * (max_tags + 1) + 4*max_input + 128
+ max_segments * (2*max_input + 1 + 3138*max_output)
```

`jpeg_inspection_work_limit` returns that exact value or overflow. It is
conservative, not a minimum. `raster_inspection_work_limit` sums the original,
extended, and JPEG sufficient bounds. Failed original/extended probes are
independently capped and charged; the original smaller-budget success theorem
is preserved. Inspection validates all selected segment entropy without
materializing full image pixels.

The concrete 271-byte witness is in `jpeg_witness.rs` and
`tests/jpeg_witness.py`. Its raw-directory, header, entropy and resource facts
establish a member of the whole JPEG profile. Negative verification checks
reject an always-error scan validator, an always-error TIFF/JPEG inspector,
a constant-zero IDCT, and removal of the public JPEG fallback.

## Reuse for JPEG 2000

The reusable work includes bounded marker parsing patterns, borrowed table
validation, resource arithmetic, component geometry, caller-scratch APIs,
color conversion, proof composition, and differential-test infrastructure.
JPEG 2000 needs separate wavelet transforms, tier-1 arithmetic coding,
code-block/packet parsing, and its own acceptance/pixel model. The JPEG DCT and
Huffman decoder are not a JPEG 2000 implementation.

References: [TIFF Technical Note 2](https://libtiff.gitlab.io/libtiff/specification/technote2.html),
[ITU-T T.81](https://www.w3.org/Graphics/JPEG/itu-t81.pdf), and the checksum-pinned
libtiff 4.7.1 and libjpeg-turbo 3.1.1 sources used for differential tests.
