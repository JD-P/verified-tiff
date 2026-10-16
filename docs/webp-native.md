# Native static WebP for TIFF

The [SSE2 kernels](webp-simd.md) now accelerate VP8 transforms and batched RGB conversion. Their composition is verified relative to eleven explicitly trusted integer lane operations and CPU dispatch; scalar fallback remains. VP8L uses span-selected prediction and four-pixel color transforms; filtering remains scalar.


Compression 50001 in the C reader supports both lossless VP8L and lossy VP8,
with opaque or alpha-bearing RGB/RGBA output. This replaces the C reader's
single-entropy-group limitation. The original whole-image Rust/CLI profile in
[webp.md](webp.md) retains its existing acceptance theorem and narrower scope.

## Supported bitstreams

The VP8L engine implements all 14 predictors, signed cross-color prediction,
subtract-green and color-index transforms, all palette packing widths, inverse
transform ordering, entropy meta-images and up to 65,536 Huffman groups. It
retains color caches, canonical Huffman parsing, zero-bit singleton leaves,
previous-nonzero length repeats, plane distances and overlapping copies.

The VP8 engine implements keyframe header and segment parsing, coefficient
probability updates, one/two/four/eight token partitions, 4x4 and 16x16 intra
prediction, segment quantizers, coefficient reconstruction, integer IDCT and
Walsh transforms, and simple/normal loop filters. YUV420 is reconstructed
before centered bilinear chroma upsampling and fixed-point BT.601 RGB
conversion. Integer rounding matches pinned libwebp's default rendering on
the independent generated-image matrix. No optional dithering or color profile
management is applied.

Static RIFF containers accept VP8L, VP8 and VP8X with ancillary metadata.
Raw and VP8L-compressed ALPH, all four alpha filters and preprocessed alpha
levels are supported. Animation/interframes are outside TIFF's independent
image-segment model. RIFF sizes, chunk ranges, duplicate image/alpha chunks,
reserved flags, padding and dimensions are checked. VP8L permits dimensions
1–16,384; VP8 permits 1–16,383 without scaling hints. VP8 partition lengths are
validated rather than recovered from truncation. VP8L consumes its complete
payload except the final byte's unused bits; VP8 permits unused arithmetic
coder padding inside its declared partitions.

TIFF samples must be contiguous 8-bit RGB or RGBA, Predictor/FillOrder 1.
Classic/BigTIFF, either byte order, strips and padded tiles are supported.
The TIFF channel count controls alpha retention; RGB TIFFs with embedded alpha
are an accepted extension tested against libwebp, since libtiff rejects them.
Native reads preserve stored samples; the existing RGBA presentation layer
handles TIFF alpha association and orientation.

## Checked models and adapter boundary

All `webp_*`, `vp8_*`, `native_webp` and `native_vp8` production modules are
inside Verus, without FFI, heap allocation, unchecked indexing or proof escape
hatches. Public contracts cover exact acceptance, model-relative pixels,
checked work and output suffix frames. VP8's explicit model composes framing,
binary arithmetic decisions, coefficient trees, contexts, prediction,
transforms and loop filtering. Its numerical theorem is about the stated
integer operations, not real-cosine conformance. Wide intermediates give
malformed coefficients defined behavior instead of C signed-overflow behavior.
Primitive reference comparisons use the native transform's defined range.

`native_webp::required_groups` decodes bounded auxiliary metadata so callers
can allocate the required table count. `decode` reparses that metadata and
checks caller-supplied storage. Pixels and transform storage are caller-owned;
`scratch_len` returns `3 * width * height + 256` RGBA slots.

`native_vp8::storage` describes padded YUV planes, macroblock-column contexts
and filters. `decode_yuv` proves the composed YUV result; `render_rgba` separately
proves pixel conversion. ALPH header validation and inverse filtering also
have checked acceptance/output contracts. The C adapter assembles a VP8L
header around compressed ALPH, selects channels and merges alpha. That assembly,
TIFF metadata/geometry, ownership/accounting and FFI remain unverified. No
end-to-end C/BigTIFF theorem is claimed. C reads stage output atomically.

Work is charged conservatively before each phase. VP8L preparation is
`64 * encoded_bytes + 3000 * pixels + 4,000,000`; rendering is
`3000 * pixels + 1,000,000 * entropy_groups`. The planning query and decode each
charge preparation. VP8 charges `64 * encoded_bytes + 1,000,000 * macroblocks +
1000 * padded_pixels + 4,000,000`; RGB conversion adds `1000 * actual_pixels`
and alpha unfiltering adds `10 * pixels`. Compressed ALPH also pays VP8L costs.
These are abstract allowances, not instruction counts or performance results.
Configure work/output/allocation limits for large segments using existing
per-open options or environment settings.

## Regression and provenance

The pinned libwebp 1.6.0 encoder/decoders are test-only. The matrices include
192 lossless images, 50 independent hand-built transform/group streams,
140 lossy images checked in both YUV and RGBA, 64 TIFF layouts and 12 alpha,
filter and partition configurations. Hand-built streams cover every predictor,
all 24 transform orders and multiple entropy groups. Primitive comparisons
include 256,000 arithmetic decisions, 10,000 each IDCT/Walsh transforms,
10,000 intra predictors and 100,000 filter kernels. Malformed-input/budget
checks and C mutation tests cover the integration boundary.

New negative gates break group selection, prediction, palette indexing,
cross-color math, binary arithmetic, RGBA output and alpha prediction, and
replace each public decoder with an always-error implementation. Original
whole-image WebP witnesses and all five old WebP negative gates are preserved.

Probability/quantizer constants and integer algorithms were translated from
the pinned upstream source; notices are in `third-party/libwebp`. The tables
can be regenerated with `scripts/vp8-tables.py`. Independent test code links
libwebp; production code does not.

Kernel optimization computes each VP8 filter decision/correction once per edge
window and adds an exact DC-only inverse-transform shortcut. VP8L common
predictors dispatch once per pixel and traverse incremental image coordinates;
power-of-two transform tiles use shifts proved equal to division. Every path
refines the unchanged integer pixel and acceptance models. The test oracle now
checks all 65,536 legal DC coefficients for both IDCT and Walsh transforms,
plus AC impulses and the existing random filter/transform matrix.
