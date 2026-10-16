# CPU profiling and optimization against libtiff 4.7.1

Historical v19 report. See [throughput.md](throughput.md) for the current pass.

Measured 2026-10-14T23:39:35.012541+00:00 on AMD EPYC 9V74 80-Core Processor, pinned to logical CPU 0.
The before version is the preserved v18 core; the after version is this delivery.
Both were freshly built and measured on this host. The pinned native reference
is libtiff 4.7.1 with zlib and scalar libjpeg-turbo 3.1.1 (SIMD disabled),
as in the existing numerical test setup. This is not a claim about every
distribution's libtiff build. The separately reported system RGBA reference
is identified in the raw metadata.

Across all 26 native cases, the geometric-mean speedup is **1.38×**;
across 24 C API cases it is **1.37×**. The native implementation
remains **10.07× slower** than this pinned libtiff build by geometric mean.
Each case has equal weight in these summaries; this is not an application workload mix.

## Findings and changes

1. JPEG IDCT rebuilt a 64-entry constant array inside each inner-loop lookup.
   Baseline disassembly shows the repeated stack stores; the smooth JPEG profile
   attributes 88.7% of CPU samples to the containing `decode_blocks` function.
   Borrowing immutable literal arrays allows constant promotion. The IDCT's exact
   arithmetic, rounding and clipping model is unchanged. Zig-zag and other small
   lookup tables receive the same treatment.
2. Textured JPEG exposed entropy-bit overhead after the IDCT fix. The bit helper
   now inlines and uses the existing proved byte divisor, removing repeated
   state copies and enabling simpler bit extraction. Geometry lookup borrows its
   32-word descriptor instead of taking it by value.
3. Deflate copied Huffman tables and linearly searched up to 288 code lengths
   for every symbol. Tables are now borrowed and carry a fixed ordered-symbol
   cache, built once per table. Its output is proved equal to the original pure
   `ranked` definition. Length/distance constant arrays are borrowed too.
4. Stored Deflate blocks previously consumed eight loop iterations per byte.
   Aligned eight-bit reads now load the byte directly, proved equivalent to the
   unchanged bit-field model. Checksum, bounds, partial-read behavior and framing
   rules remain in force.
5. The full first sweep exposed a LogLuv regression from 7.9 to 21.5 ms.
   Its profile attributed 57.2% of samples to an out-of-line `scatter_byte` call;
   the baseline had inlined this helper. An `inline(always)` hint restores that
   specialization without changing its body or contracts. The final full sweep
   below includes this fix. The superseded sweep is retained as
   `evidence/profile-first-pass-results.json`.

The Deflate cache adds 704 bytes of declared fields per Huffman table on this
64-bit target, two persistent tables per cursor. Construction scans 288 lengths
for each of 16 buckets. Exact compiler stack usage is not proved. There is no
core heap allocation, unsafe decoder, runtime codec dependency or new axiom.

## Native inspect plus decode

Seven shuffled process batches per engine/case, each at least 80.0 ms.
Warm-cache medians below exclude disk I/O, startup, fixture generation and output
checking. The shared virtual host is not isolated benchmark hardware. Raw
per-batch results and quartiles are in `evidence/profile-results.json`.


| Case | Before ms | After ms | Speedup | 4.7.1 ms | After / 4.7.1 |
| --- | ---: | ---: | ---: | ---: | ---: |
| gray-none | 0.034 | 0.034 | 1.01× | 0.034 | 0.99× |
| gray-packbits | 6.876 | 5.053 | 1.36× | 1.275 | 3.96× |
| gray-lzw | 15.775 | 14.901 | 1.06× | 0.925 | 16.11× |
| gray-deflate | 25.135 | 19.346 | 1.30× | 0.457 | 42.33× |
| rgb-none | 0.091 | 0.091 | 1.00× | 0.084 | 1.08× |
| rgb-lzw | 50.306 | 48.090 | 1.05× | 2.824 | 17.03× |
| rgb-deflate | 71.442 | 53.400 | 1.34× | 1.247 | 42.81× |
| rgb-jpeg | 587.086 | 80.049 | 7.33× | 3.229 | 24.79× |
| rgb-jpeg-textured | 1214.575 | 300.491 | 4.04× | 25.156 | 11.94× |
| rgb-tile-lzw | 55.781 | 62.687 | 0.89× | 2.743 | 22.86× |
| gray16-deflate | 354.284 | 81.264 | 4.36× | 8.217 | 9.89× |
| float64-lzw-p2 | 101.215 | 87.715 | 1.15× | 11.633 | 7.54× |
| noise-lzw | 52.975 | 52.492 | 1.01× | 2.180 | 24.08× |
| noise-deflate | 33.372 | 17.156 | 1.95× | 0.366 | 46.82× |
| fax3 | 15.114 | 15.037 | 1.01× | 1.735 | 8.67× |
| fax4 | 9.588 | 6.870 | 1.40× | 1.749 | 3.93× |
| webp | 48.668 | 20.359 | 2.39× | 1.245 | 16.35× |
| logluv | 7.939 | 7.848 | 1.01× | 1.124 | 6.98× |
| original-quad-lzw-compat.tiff | 10.260 | 9.517 | 1.08× | 1.305 | 7.29× |
| original-lzw-single-strip.tiff | 115.469 | 117.178 | 0.99× | 30.645 | 3.82× |
| original-testfax4.tiff | 28.312 | 26.301 | 1.08× | 11.495 | 2.29× |
| original-deflate-last-strip-extra-data.tiff | 8.037 | 5.513 | 1.46× | 0.212 | 25.97× |
| white-none | 0.013 | 0.013 | 1.01× | 0.116 | 0.11× |
| rgba-none | 1.884 | 1.879 | 1.00× | 0.040 | 46.52× |
| rgba-tile-none | 21.600 | 21.715 | 0.99× | 0.108 | 201.04× |
| bilevel-none | 4.856 | 4.601 | 1.06× | 0.304 | 15.11× |

## C API fresh-handle RGBA

The same C consumer opens, renders and closes an in-memory TIFF on every iteration.
Adapter allocation, conversion and inspection are included. The adapter itself
was not changed and remains outside the formal proof.

| Case | Before ms | After ms | Speedup | 4.7.1 ms | System ms |
| --- | ---: | ---: | ---: | ---: | ---: |
| gray-none | 3.530 | 3.611 | 0.98× | 0.545 | 0.704 |
| gray-packbits | 10.875 | 9.285 | 1.17× | 1.853 | 1.924 |
| gray-lzw | 26.394 | 24.027 | 1.10× | 1.491 | 1.784 |
| gray-deflate | 37.249 | 29.653 | 1.26× | 0.957 | 0.832 |
| rgb-none | 7.497 | 7.626 | 0.98× | 0.793 | 0.768 |
| rgb-lzw | 79.351 | 72.329 | 1.10× | 3.538 | 3.371 |
| rgb-deflate | 102.507 | 81.230 | 1.26× | 1.757 | 0.868 |
| rgb-jpeg | 608.414 | 93.173 | 6.53× | 3.722 | 2.835 |
| rgb-jpeg-textured | 1576.458 | 438.973 | 3.59× | 25.305 | 19.842 |
| rgb-tile-lzw | 71.192 | 75.041 | 0.95× | 3.269 | 3.143 |
| gray16-deflate | 605.928 | 125.385 | 4.83× | 8.109 | 5.195 |
| noise-lzw | 90.321 | 81.325 | 1.11× | 2.644 | 2.895 |
| noise-deflate | 47.503 | 27.022 | 1.76× | 1.055 | 0.882 |
| fax3 | 24.620 | 24.777 | 0.99× | 0.777 | 0.808 |
| fax4 | 16.765 | 13.129 | 1.28× | 0.765 | 0.819 |
| webp | 60.223 | 32.758 | 1.84× | 1.879 | 1.873 |
| original-quad-lzw-compat.tiff | 15.738 | 14.360 | 1.10× | 1.343 | 1.356 |
| original-lzw-single-strip.tiff | 234.940 | 241.112 | 0.97× | 8.222 | 7.730 |
| original-testfax4.tiff | 64.380 | 59.399 | 1.08× | 4.059 | 4.086 |
| original-deflate-last-strip-extra-data.tiff | 12.610 | 8.813 | 1.43× | 0.338 | 0.304 |
| white-none | 0.838 | 0.844 | 0.99× | 0.139 | 0.176 |
| rgba-none | 3.560 | 3.491 | 1.02× | 0.317 | 0.315 |
| rgba-tile-none | 23.026 | 23.315 | 0.99× | 0.383 | 0.390 |
| bilevel-none | 5.532 | 5.505 | 1.00× | 0.083 | 0.080 |

## CPU profiles

Flat SIGPROF samples of the actual release binaries, requesting a 1 ms process
CPU timer. Kernel timer granularity limits delivery; this is not a call graph.
Samples include startup, preflight and warmup. Inlined functions appear under
their containing symbol, and some stripped runtime-library symbols remain
unresolved. All retained profiles have zero dropped samples. Profile-run timings
are diagnostic only; the tables above use separate runs without the sampler.
Raw PCs, load biases, sample counts and executable hashes are retained under
`evidence/profile/`. Disassembly corroborates the repeated constants and copies.

| Workload / version | Samples | Three largest function self-sample shares |
| --- | ---: | --- |
| rgb-jpeg / before | 1252 | `verified_tiff::jpeg_scan::decode_blocks` 88.7%; `verified_tiff::jpeg_layout::source` 3.5%; `<verified_tiff::jpeg::JpegSegment>::decode_with_scratch` 1.8% |
| rgb-jpeg / after | 1421 | `verified_tiff::jpeg_scan::decode_blocks` 32.1%; `verified_tiff::jpeg_layout::source` 29.4%; `<verified_tiff::jpeg::JpegSegment>::decode_with_scratch` 11.6% |
| rgb-jpeg / libtiff | 1418 | `jpeg_idct_islow` 52.4%; `null_convert` 21.5%; `decode_mcu` 14.7% |
| rgb-jpeg-textured / before | 1870 | `verified_tiff::jpeg_scan::decode_blocks` 41.7%; `verified_tiff::jpeg_block::block` 19.8%; `verified_tiff::jpeg_huffman::decode` 17.4% |
| rgb-jpeg-textured / after | 1005 | `verified_tiff::jpeg_huffman::decode` 42.5%; `verified_tiff::jpeg_block::block` 32.0%; `verified_tiff::jpeg_layout::source` 9.5% |
| rgb-jpeg-textured / libtiff | 799 | `decode_mcu` 68.2%; `jpeg_idct_islow` 25.7%; `null_convert` 2.1% |
| gray16-deflate / before | 1073 | `<verified_tiff::deflate::Cursor>::step` 78.2%; `[unresolved]` 15.2%; `verified_tiff::scatter_byte` 3.0% |
| gray16-deflate / after | 844 | `<verified_tiff::deflate::Cursor>::step` 82.9%; `<verified_tiff::extended::image::ExtendedImage>::decode_with_scratch` 5.2%; `verified_tiff::deflate_plane::decode` 4.5% |
| gray16-deflate / libtiff | 1304 | `[unresolved]` 78.1%; `native` 11.1%; `adler32_z@@ZLIB_1.2.9` 7.2% |
| noise-deflate / before | 846 | `<verified_tiff::deflate::Cursor>::step` 70.4%; `verified_tiff::scatter_byte` 9.7%; `<verified_tiff::raster_layout::RasterLayout>::convert` 8.2% |
| noise-deflate / after | 1628 | `<verified_tiff::deflate::Cursor>::step` 61.8%; `verified_tiff::deflate_plane::decode` 13.4%; `<verified_tiff::extended::image::ExtendedImage>::decode_with_scratch` 12.1% |
| noise-deflate / libtiff | 756 | `adler32_z@@ZLIB_1.2.9` 82.0%; `[unresolved]` 15.1%; `TIFFFindField` 0.7% |

## Measured tradeoffs

Cases more than 5% slower in this sweep are listed explicitly. Tiled RGB LZW
regresses with the stronger shared scatter inlining hint, while LogLuv returns
to baseline speed and several other paths improve. The native LZW tile quartiles
do not overlap in this run. This is a measured cost of the delivered choice,
not a claim of uniform improvement. A future change should specialize the
LogLuv/scatter call sites without penalizing tiled LZW, then remeasure all paths.

| Case | Interface | Before ms | After ms | Change |
| --- | --- | ---: | ---: | ---: |
| rgb-tile-lzw | Native | 55.781 | 62.687 | +12.4% |
| rgb-tile-lzw | C API | 71.192 | 75.041 | +5.4% |

## Validation and remaining work

The complete crate verifies 924 obligations with zero errors and all 35 negative
gates reject their mutations. The full 32 Rust / 87 Python test methods, generated
matrices, 31 original TIFF corpus files, 11 adapter methods, 2,000 adapter
mutations and 20,000 core mutations pass. The separately built libtiff passes its
own 108 tests. Those upstream tests test libtiff, not this library.

Every benchmark's before/after output is byte-for-byte identical, including JPEG;
lossless cases match libtiff exactly and JPEG comparisons stay within the existing
maximum channel error of 3. No acceptance policy, mathematical pixel model,
resource quota or output frame was relaxed. The C boundary did not change;
earlier sanitizer evidence is explicitly historical.

The remaining profiles point to JPEG Huffman decoding and MCU-to-image addressing,
Deflate bit decoding/state transitions, and LZW bit reads (49.1% of baseline
random-LZW C API samples). Tiled uncompressed data and WebP still have large
relative gaps. These remain future optimization targets, not completed work.
See every row above for regressions as well as improvements; small changes on
unmodified paths can reflect code layout and host variability.

Reproduction commands are in `../benchmarks/README.md`. The reverse patch restores
all 51 v18 core files with archived hashes; the v17 baseline remains reconstructible.
`evidence/profile-baseline-verification.json` is the historical v18 proof summary;
`evidence/verification-summary.json` and `proof-gate.log` describe this source.
