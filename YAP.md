The [metadata consistency pass](docs/metadata-contracts.md) adds complete pinned
codec-tag schemas, fixes the InkNames getter ABI, and derives NumberOfInks from
verified name validation. The fresh whole-crate proof has 1,809 verified
obligations; C function/line/branch coverage remains 100%.

The [C shim hardening pass](docs/c-shim-hardening.md) fixes metadata-registration rollback and color/allocation edge cases, and adds a reproducible 100% C function/line/branch coverage gate using integration and fault-injection tests.


The [adapter proof extension](docs/adapter-proofs.md) checks metadata dispatch and integer conversion, crop/segment placement, source-window conservation, allocation/range accounting, C output initialization and numerical table indexing. These executable helpers are used by the C reader; the remaining foreign-pointer and orchestration boundary is explicit.

The [WebP SSE2 pass](docs/webp-simd.md) adds span-selected VP8L prediction/color kernels, four-lane VP8 transforms and batched color conversion through an explicitly trusted integer-operation boundary, with scalar fallback.
The focused 24-workload benchmark records 1.55× lossless and 1.24× lossy geometric-mean speedups over the previous reader, with exact output comparisons. These are WebP measurements; remaining libtiff gaps and per-kernel limitations are documented in that report.


An experimental Rust reader for **uncompressed, PackBits, LZW, Deflate, JPEG, OJPEG, CCITT Group 3/4, a lossless WebP subset, and SGILog32 LogLuv in Classic TIFF**, with a
Verus-verified decoding core and a command-line converter to PGM/PPM/PAM/NPY.
It supports both byte orders, 1-bit black-and-white, 8-bit grayscale and RGB with optional alpha, strips, tiles, and
selection of an image from the main directory chain. The CLI and `inspect_raster`
also support **1/4/8-bit palette images, separate planes, and unsigned 16-bit
grayscale/RGB**, with all four compression modes. It also supports **IEEE float64 grayscale/RGB**, including horizontal Predictor 2 with LZW/Deflate; see [float64 and exact output bits](docs/float64.md). Deflate includes the 8 and 32946 identifiers.
JPEG/OJPEG additionally support eight-bit grayscale, RGB, and subsampled YCbCr.
Fax compression supports one-bit scans; see [fax support and proof scope](docs/fax.md).
SGILog32 supports native LogLuv16 components as NPY; see [LogLuv output and proof scope](docs/logluv.md).
WebP supports single-group VP8L RGB/RGBA with optional subtract-green and color caches; see [WebP scope and exclusions](docs/webp.md).
See [JPEG support and proof scope](docs/jpeg.md).
See [Deflate behavior and proof scope](docs/deflate.md). 16-bit output
preserves sample precision. See [extended formats and API](docs/extended-formats.md).

The latest [LERC throughput pass](docs/lerc-throughput.md) replaces per-bit unpacking, specializes reconstruction and removes block-array copies and per-sample packing calls. It retains the [five explicitly trusted hardware floating-point primitives](docs/native-float.md).

The core is allocation-free, uses borrowed immutable input and caller-owned
output, and has no project-owned unsafe code. Decoder logic is verified relative
to the documented [floating-point](docs/native-float.md) and
[SSE2 integer-operation](docs/webp-simd.md) boundaries; their primitive bodies
and CPU dispatch are trusted, not proved.
File I/O and allocation live in separate, unverified CLI and C ABI adapters.

A **partial reader-oriented libtiff C interface** is available in `compat/`. It exports
the 198 functions in the pinned Linux public header; supported reads use the
verified core and unsupported operations report `ENOSYS`. Build it with
`bash scripts/build-compat.sh`. See [C ABI support, restrictions, and linking](docs/c-api.md).
The C reader additionally supports **BigTIFF, image SubIFDs, seekable independent
segments, native palette/associated-alpha/padded-tile samples, signed integers,
float32 and higher-bit alpha, and integer horizontal prediction**. Ordinary
DPI/ICC/string metadata, read-mode flags, memory options and source orientations
are supported. These additions use new verified record/segment/word primitives;
[their composition in the C/FFI adapter is not formally verified](docs/native-compatibility.md).
The C reader now also exposes custom/anonymous tags, EXIF/GPS directories,
read-side setters, float LogLuv XYZ, CMYK/Lab/non-JPEG YCbCr display conversion,
CCITT RLE and floating Predictor 3, plus GDAL-style user-buffer/temporary-metadata
contexts. See [consumer compatibility](docs/consumer-compatibility.md).
The original whole-image Rust API and CLI retain their earlier profiles.

**Verification scope:** the checked contracts establish core safety, decreasing
loops, correspondence of accepted metadata to file fields, correct output for
accepted images, and acceptance of every file satisfying the formal
`supported_profile` predicate with a proved sufficient inspection budget.
That predicate is defined directly over the original bytes and metadata.
It does not call the parser or assume that pixel preflight succeeds.
Decoding then succeeds with sufficient output space and the work quota reported
by `image.decode_work_limit()`. This covers the limited profile below; it is
not a full libtiff replacement. The TIFF model itself still needs human review.
The extended formats and Deflate additionally have a complete inspection
acceptance theorem against `supported_extended_profile`, with a checked
sufficient-budget calculator. JPEG has its own whole-image acceptance theorem
and checked budget calculator. `supported_raster_profile` is the union of the original, extended, JPEG, fax, WebP, and LogLuv
profiles and covers the public `inspect_raster` pipeline.
See [the proof ledger](docs/verification.md) and [extended proof scope](docs/extended-formats.md#checked-proof-scope).

## Build and run

The supplied bootstrap targets Linux x86-64 and installs its tools inside
`.tools/`. It needs Python 3.11+, a C compiler/linker, network access, and about
3 GB of toolchain/download storage plus build outputs. Verification scripts also use `rg`.
No global Rust installation is required or changed.

```bash
python3 scripts/bootstrap.py
bash scripts/verify.sh
source scripts/env.sh
cargo build --locked --release --bin verified-tiff
target/release/verified-tiff input.tif output.ppm
target/release/verified-tiff input.tif --inspect
target/release/verified-tiff input.tif page2.pgm --page 2
target/release/verified-tiff input-alpha.tif output.pam
target/release/verified-tiff input-float64.tif output.npy
target/release/verified-tiff input-logluv.tif output.npy >logluv.json
```

The output is P5 (Gray8/Gray16), P6 (RGB8/RGB16), or P7 PAM (8-bit Gray+A/RGBA), regardless of
the output filename's extension. Float64 input produces NPY 1.0 with little-endian binary64 samples and no precision loss. LogLuv input produces little-endian native 16-bit components in NPY; these are encoded HDR components, not display RGB. Alpha input uses one trailing 8-bit sample,
marked as associated (premultiplied) or unassociated (straight) by ExtraSamples.
Output uses straight alpha; alpha bytes are preserved. Associated colors are
unassociated with nearest rounding and clipping to 0..255; color is zero at
zero associated alpha. This conversion can lose precision and clips extended
colors above the representable range. See the exact policy in the profile. WhiteIsZero grayscale is inverted into normal Gray8. Page numbers
start at zero. Packed 1-bit pixels expand to Gray8 values 0 (black) or 255
(white). Uncompressed black-and-white images support both FillOrder values. Byte-padded
scanlines and omitted BitsPerSample (TIFF's default of 1) are also supported.
The CLI prints JSON dimensions, source `bits_per_sample`, `compression`, `source_alpha`, `output_alpha`, output
size, numeric-container `sample_format`, `color_model`, `predictor`, `output_byte_order`, the decode work quota, and charged work.
It returns exit status 2 for decoding, resource, argument, or I/O errors.
The requested output path is overwritten only after decoding succeeds.

## Run the checks

```bash
# Required for the complete differential suite: independent upstream oracles.
python3 scripts/upstream.py
python3 tests/corpus.py # after building the CLI

bash scripts/verify.sh
bash scripts/test.sh
bash scripts/test-compat.sh # needs system libtiff.so.6 for the existing-binary test
bash scripts/fuzz-smoke.sh 20000
# Longer deterministic mutation run:
bash scripts/fuzz-smoke.sh 1000000
```

`scripts/upstream.py` downloads checksum-pinned libtiff 4.7.1, runs its own
test suite, and builds the test-only oracles, including pinned libjpeg-turbo
3.1.1 and libwebp 1.6.0. The native-codec suites additionally build pinned Esri LERC 4.0.0 and Zstandard 1.5.7. WebP differential tests require these live oracles. It needs `make`, `cc`, and CMake (the script installs a pinned CMake
locally with pip when needed);
optional codec development libraries determine the upstream
configuration. The Rust reader never links to libtiff.

The tests check hand-generated pixel arrays and committed expected hashes,
then compare the supported profile against independent decoders. WebP tests
require the live libwebp/libtiff oracles; run `scripts/upstream.py` first.
Test output states which live differential comparisons were performed. The verifier fails closed
on missing tools, false contracts, timeouts, partial-crate verification, or an
unexpected verifier commit. It also checks that replacing inspection with an
always-error body violates the acceptance contract. Separate negative checks
cover the original inspector, extended inspector, public raster wrapper,
Deflate stream validator, LZW ending acceptance and rejection, legacy LZW selection/width/full-table semantics, JPEG scan/image acceptance, the JPEG public fallback,
IDCT pixel semantics, fax stream/image/pixel/public-dispatch semantics, float64 bit/prediction/acceptance semantics, WebP stream/image/transform/cache/public-dispatch semantics, and LogLuv stream/image/conversion/chroma/public-dispatch semantics. Normal `cargo build` compiles code but does not
perform formal verification.

The pinned toolchain is Verus `0.2026.10.04.426d8b0`, Rust `1.98.1`, matching
`vstd`, and bundled Z3 `4.16.0`. Exact revisions and checksums are in
`toolchain-lock.json` and `Cargo.lock`.

## Rust API

For the original whole-image profiles, use `raster_inspection_work_limit(page, &limits)`
for a proved sufficient inspection budget, then `inspect_raster`, allocate the reported
`output_len()` and `scratch_len()`, and call `decode_with_scratch`. See the
[complete example and memory limits](docs/extended-formats.md#api-and-memory).
The API below remains available for the original zero-scratch profile.


```rust
use verified_tiff::{inspect, inspection_work_limit, Budget, Limits};

let limits = Limits::conservative();
let mut budget = Budget { remaining: inspection_work_limit(0, &limits)? };
let image = inspect(&bytes, 0, &limits, &mut budget)?;
// Allocate fallibly in applications handling untrusted files.
let mut pixels = Vec::new();
pixels.try_reserve_exact(image.output_len())?;
pixels.resize(image.output_len(), 0);
let mut decode_budget = Budget { remaining: image.decode_work_limit() };
image.decode_into(&mut pixels, &mut decode_budget)?;
```

`Image` borrows its source bytes and has private fields. It cannot be
constructed with arbitrary geometry or switched to a different source buffer
through the safe API. An error during decoding can leave a partially written
output prefix; the suffix beyond `output_len()` is preserved. Too-small output
is rejected without writing or spending decode work.
`image.bits_per_sample()` reports the source depth; `image.output_len()` always
counts expanded 8-bit samples, including alpha and packed 1-bit expansion.
`image.alpha_mode()` reports `AlphaMode::{None, Associated, Unassociated}`
from the source; `image.has_alpha()` identifies two/four-channel output.
`decode_into` always returns straight alpha when present.
PackBits (`Compression=32773`) and TIFF LZW (`Compression=5`) work for
1-bit bilevel, 8-bit grayscale/RGB, and 8-bit Gray+A/RGBA with strips or tiles. Both require
FillOrder 1 and Predictor 1. The broader float64 path separately supports Predictor 2 with LZW/Deflate.
FillOrder 2 remains supported for uncompressed bilevel images. Compressed
streams are validated before an `Image` is returned. Decode streams storage
bytes into the caller's output, using a fixed cursor for PackBits or a fixed
4,096-entry dictionary and 4,096-byte phrase buffer for LZW. There is no heap
allocation or decompressed-segment buffer in the core.

LZW supports both modern TIFF packing and old-style libtiff LSB packing with
late width changes and a frozen full dictionary. The unchanged
`quad-lzw-compat.tiff` decodes exactly. Original fixture coverage is now
**31/31 (100%)**: 27 exact native-output comparisons and four bounded JPEG comparisons, including native LogLuv16 for `logluv-3c-16b.tiff`. This measures the first main image of each original TIFF fixture, not the full libtiff API test suite.

LZW also accepts an omitted or incomplete EOI immediately after exact storage
expansion when only 0–7 residual bits remain. This explicit compatibility rule
covers the unchanged `lzw-single-strip.tiff` regression; incomplete pixels and
whole junk bytes are rejected. See [the precise ending policy](docs/profile.md#lzw-profile).

## Resource and compatibility policy

Default limits are 64 MiB input, 64 MiB output, 65,536 per dimension, 1,024
directories per selected-page traversal, 4,096 tags per directory, and
1,048,576 strips/tiles. The CLI's default work budget is 100 million abstract
units. All are adjustable with the documented `--help` options. The default
work budget can reject an otherwise size-compliant large image; raise it
explicitly when needed.

For the Rust API, `inspection_work_limit(page, &limits)` computes the proved
conservative sufficient inspection budget:
`(page + 1) * (max_tags + 1) + 19 * max_tags + 4 + max_segments * (max_input + 1) + max_output`.
It returns `Limit` if that calculation cannot fit in `usize`; tighter configured
limits can reduce this bound. The bound is sufficient, not a minimum: a
particular file may use much less. The segment term allows all segments to
refer to overlapping input ranges, so it is intentionally very conservative.
Decoding needs an additional `image.decode_work_limit()` units. The base decode quota is one unit per output byte, plus one additional
unit per output byte for alpha normalization when present. Compressed decoding
adds the sum of encoded and expanded storage bytes for every segment, including
tile padding. All alpha and compressed paths reserve their quota before writing;
insufficient work leaves output unchanged. This quota bounds streaming work; it is not an instruction count.
The CLI continues to use its explicitly configured shared work budget.
For 1-bit images, output limits and per-sample work apply to the expanded
`width * height` bytes, not the smaller packed input size.

Uncompressed inspection validates complete segment storage and derives sample
addressability from the coverage proof, without reading every output sample.
Its conservative output-sized work charge is retained. Compressed
inspection validates stream structure and exact expansion sizes, including
PackBits row boundaries and LZW dictionary references/end markers. The
segment-coverage proof establishes sample validity. Compressed
decoding visits each segment once, expanding bytes without rereading earlier
packets. A work unit represents a bounded operation or conservative storage
quota, not a nanosecond or CPU instruction. See [the profile](docs/profile.md)
for exact exclusions and [the trust boundary](docs/trusted-boundary.md).

Project code is MIT-licensed. CCITT codebook data retains its upstream notice
in `src/fax_tables.rs`. Imported test images retain upstream notices in
`tests/upstream/`; toolchain and dependency licenses remain their own.

See [the recorded validation results](docs/validation.md) for this version's
checks and [the next implementation tasks](docs/next-steps.md) to continue.

## Performance

Earlier throughput work added bounded word reads for LZW/Deflate, buffered JPEG bit
consumption, exact IDCT factorization, block-row JPEG rendering, padded tile-row
copies, shared bilevel span geometry, and alpha/native-layout shortcuts. Deflate uses bounded checksum
batches and direct strip writes. The opt-in `decode_deflate_into` API combines
strip validation and decoding; existing inspection guarantees stay unchanged.

The [RGBA optimization](docs/rgba-optimization.md) adds verified, specialized
8/16-bit RGB/gray row conversion, fuses RGB alpha presentation, borrows cached
segments, and admits complete Deflate staging under explicit memory checks.
Private output staging preserves failed-read atomicity; tight budgets and large
segments retain bounded replay. Exact before/after results and all 92 workloads
are recorded with the full proof and test evidence.

The earlier [Deflate/LERC optimization](docs/reader-optimization.md) batches
streaming Deflate transitions, removes replay for complete native Deflate reads
using private staging, and accelerates LERC's software-IEEE normalization with a
proved top-bit refinement. Partial reads retain full validation before exposure.
Its same-host comparison includes all 78 original API workloads, eight strip-height
diagnostics and six default-strip-chopping checks, with exact before/after outputs.
The [earlier C reader benchmark](docs/reader-benchmark.md) records the bottlenecks
before this change. The [benchmark harness](benchmarks/README.md) documents
reproduction. The [v20 throughput results](docs/throughput.md) retain the
earlier before/after comparison; their sources precede the reader additions.
The [profiling report](docs/profiling.md) records the v18-to-v19 pass;
[earlier optimizations](docs/optimization.md) and the
[original benchmark](docs/benchmark.md) remain historical evidence.

The C reader also supports dictionary-free Zstandard (Compression 50000),
including Predictors 2 and 3, through the separate verified native API; see
[docs/zstd.md](docs/zstd.md). It also supports **LERC v4**, including Deflate/Zstandard wrappers, and **static WebP VP8/VP8L with alpha**, using separate verified native engines. See [LERC](docs/lerc.md) and [full static WebP](docs/webp-native.md). Native LERC/libwebp libraries are test-only dependencies.


The [read-side additions](docs/reader-features.md) include lazy strip/tile tables,
bounded range reads for uncompressed/PackBits/LZW/Deflate, cropped RGBA output,
reader extension hooks, and packed/half/24-bit/complex native sample transport.
The original whole-image proof scope remains unchanged.