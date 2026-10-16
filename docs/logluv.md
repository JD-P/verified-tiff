# SGILog32 and native LogLuv16

`inspect_raster` and `inspect_logluv_tiff` support the unchanged libtiff 4.7.1
fixture `logluv-3c-16b.tiff` and the explicitly bounded profile below. The core
implements SGILog32 run-length decoding and native 16-bit component expansion.
It does not convert HDR color to display RGB. Its behavior was checked against
pinned libtiff's `libtiff/tif_luv.c`, especially `LogLuvDecode32`,
`Luv32toLuv48`, and the SGILog data-format setup/close routines. That source
is a compatibility reference and test dependency, not production decoder code.

## Metadata and geometry

- Classic TIFF, both byte orders, bounded selection from the main IFD chain.
- Compression 34676 (`SGILog`), PhotometricInterpretation 32845 (`LogLuv`),
  SamplesPerPixel 3, explicit BitsPerSample `[16,16,16]` and explicit
  SampleFormat `[2,2,2]`. SHORT and LONG encodings of these arrays are accepted.
- Contiguous samples, Orientation 1, FillOrder 1, Predictor 1, no ExtraSamples.
  The usual default of 1 is accepted for those four scalar fields.
- Strips or tiles. Each storage block dimension is 1..16384; tile dimensions
  must also be multiples of 16. Image dimensions obey configured limits.
  Strips use actual rows in the final strip; tiles include full edge padding.
  RowsPerStrip defaults to 2^32-1 and is clamped to image height before the
  block-dimension check. Mixed strip/tile tags are rejected.
- Offset/count arrays must match the computed number of segments and their
  declared ranges must lie within the input. SHORT and LONG are accepted.
- The shared `fax_meta` directory reader permits unsorted entries and zero-tag
  tombstones, rejects duplicate recognized fields, and checks typed payload
  ranges. Ignored metadata, including color calibration, is not applied.

This profile excludes LogL grayscale (photometric 32844), SGILog24
(compression 34677), separate planes, alpha, BigTIFF, and general signed
16-bit grayscale/RGB. Adding native LogLuv does not add those representations
to other codecs.

## Encoded bytes and strict termination

Each stored pixel has four encoded component bytes: high and low luminance
code bytes, then eight-bit u and v codes. For each row, SGILog compresses the
four planes separately, in that order. Each plane has exactly block-width
bytes after expansion. File byte order affects TIFF metadata, not this
byte-plane ordering.

An unsigned packet control means:

| Control | Action |
| --- | --- |
| 0 | No-op; consumes the control byte only |
| 1..127 | Copy that many following literal bytes |
| 128..255 | Repeat the following byte `control - 126` times (2..129) |

Packets must fit both their encoded segment and the remaining plane in the
current row. Each segment must expand to exactly four bytes per storage pixel.
Leading, interspersed, and trailing zero controls are allowed and charged to
the input/work limits. Missing payloads, short expansion, plane-crossing runs,
overexpansion, and trailing non-no-op packets are rejected. Unlike libtiff's
permissive decoder, this profile does not clip an overlong run to a row/plane.
The inspector validates the complete declared segment before returning an
image; decoding cannot use early stopping to bypass trailing-data validation.

## Output interpretation

Output is row-major, interleaved native LogLuv16: three little-endian 16-bit
containers `(L, u, v)` per pixel, matching libtiff's
`TIFFTAG_SGILOGDATAFMT=SGILOGDATAFMT_16BIT` result. It occupies six bytes per
pixel. Padding outside the visible image is discarded.

- `L` preserves the encoded 16-bit luminance word exactly. This word uses a
  sign bit and logarithmic magnitude; interpreting the container as an `i16`
  does **not** make its numeric value linear luminance or ordinary signed
  log luminance. In particular, negative luminance retains its encoded bits.
- For each eight-bit chromaticity code `c`, the component is
  `floor((c + 1/2) * 32768 / 410)`, implemented exactly as
  `((2*c + 1) * 8192) / 205` with integer arithmetic. Values range from 39 to
  20420. The proof covers this rational conversion, without floating rounding.
- There is no XYZ/RGB conversion, tone mapping, gamma correction, or STONITS
  calibration. These are encoded native components, not display colors.
  Exact output means preservation/expansion of the stored quantized codes;
  it does not recover color precision lost before compression.

`Raster::sample_format()` returns `SampleFormat::SignedInteger` and
`Raster::color_model()` returns `ColorModel::LogLuv`. The latter prevents
three channels from being mistaken for RGB. `ColorModel` also identifies
existing Gray and RGB output. Source and output depth are both 16; alpha is
absent and planar configuration is 1.

The CLI writes NPY 1.0 with `descr='<i2'` and shape `(height,width,3)`, regardless
of the filename extension. Its JSON reports `sample_format="signed_integer"`,
`color_model="logluv16"`, and `output_byte_order="little"`. NPY alone records
dtype and shape, not color interpretation: retain the JSON or source metadata
with the array. For example:

```bash
target/release/verified-tiff tests/upstream/logluv-3c-16b.tiff logluv.npy >logluv.json
```

The original fixture has packed word `0x000056c2`; its native containers are
`(0,6913,15544)`, with little-endian bytes `00 00 01 1b b8 3c`.

## Resource policy and checked claims

The core has no allocation, codec FFI, unsafe code, floating-point operations,
or runtime recursion. It uses a constant-size packet cursor and caller-owned
scratch. Scratch holds the native result (six bytes per storage pixel) and
expanded byte planes (four bytes per storage pixel), totaling ten bytes per
storage pixel for the largest segment. Both total output and scratch must
individually fit `max_output`; the limit is not their combined allocation.

For each segment of `A` storage pixels and `C` encoded bytes, inspection and
decode each reserve `C + 20*A + 1` work units. This conservatively covers
packet traversal, raw expansion, native conversion, scatter, and a segment
charge. Decode reserves the sum before writing. Output/scratch that are too
small or insufficient decode work leave both buffers and the budget unchanged.
Suffixes beyond `output_len()` and `scratch_len()` are preserved. These are
abstract work units, not an elapsed-time or machine-instruction proof.

The dedicated sufficient inspection bound is:

```
(page + 1) * (max_tags + 1) + 128
+ max_segments * (max_input + 2*max_output + 1)
```

`logluv_inspection_work_limit` succeeds iff this bound fits `usize`. The public
raster bound adds it to the five earlier profile bounds; earlier probes are
capped independently and charge their actual work. The original smaller-budget
acceptance guarantee remains intact.

The same full-crate Verus run checks all five `logluv*.rs` modules:

- `sg_valid` is a pure packet/expansion/boundary model; `sg_byte` specifies
  expanded bytes. The validator has a success equivalence, and the cursor
  refines that byte model with decreasing encoded and output positions.
- Metadata, segment size/range, and stream predicates define
  `supported_logluv_profile` over the original input. It implies successful
  inspection with the sufficient bound, without assuming parser success.
- Conversion establishes exact native bytes; segment geometry composes them
  with the full-image scatter model. Decode succeeds with the reported
  buffers/quota and produces the specified output with buffer suffix frames.
- A concrete proof witness mirrors the entire unchanged 166-byte original.
  Its dedicated sufficient inspection budget is 327, output is six bytes,
  scratch ten bytes, and decode work 29 units. A Rust test uses the same bytes
  and exact budgets.
- Five negative proof gates must reject an always-error stream validator,
  always-error TIFF inspector, removed conversion writes, zero chroma
  conversion, and removed public dispatch. All earlier gates remain required.

The proof is relative to these mathematical models and the pinned toolchain;
translation of format behavior into the model still needs independent review.
The CLI's NPY/JSON and C/Python test helpers are outside the formal boundary.

## Regression coverage

`tests/logluv.py` checks 96 combinations of byte order, strips/tiles, dimensions,
packet style and source patterns against independent rational components and
live libtiff. It also checks all 256 u/v codes at six luminance boundaries/signs,
literal/repeat boundaries, no-ops, multipage selection, malformed streams,
unsupported metadata, limits, and the original fixture. A separate test uses
libtiff's encoder to produce 48 files from independently specified packed
words, then compares both decoders to the rational expected output. The
encoder helper compensates libtiff 4.7.1's generic pre-encode RAW-word swapping
when file/host byte orders differ: `TIFFWriteEncodedStrip` applies the swap,
while `LogLuvEncode32` expects native packed integers. No compressed bytes or
upstream source are modified.

`tests/logluv_api.rs` checks dedicated/public APIs, multipage decoding, exact
quotas, undersized buffers, suffix preservation, the original witness, and
checked budget overflow. Four LogLuv inputs are deterministic mutation seeds.
Native decoding of this fixture completes the **31/31 original first-main-image**
corpus: 27 exact native-output comparisons and four bounded JPEG comparisons.
That denominator does not measure libtiff's C API tests, writer support, all
pages or SubIFDs, or support for every TIFF variant.
