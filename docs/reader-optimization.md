# Deflate and LERC reader optimization

RGB Deflate native reads improved 9.76× and plain float32 LERC improved 6.94× on this suite. Both remain slower than libtiff 4.7.1: 9.68× and 29.16× as long, respectively.

All 92 before/after outputs match byte-for-byte, including JPEG. Against libtiff, 90 match exactly; the two textured-JPEG operations retain the previous maximum difference of one 8-bit channel value. Every timed and memory sample matches its own preflight hash. The eight layout and six default-mode checks are excluded from main-suite geometric means.

## Selected results

All times are medians in milliseconds; after / 4.7.1 above one means slower.

| Input / operation | Before ms | After ms | Speedup | 4.7.1 ms | After / 4.7.1 |
| --- | --- | --- | --- | --- | --- |
| RGB8 deflate / native | 116.92 | 11.98 | 9.76× | 1.238 | 9.68× |
| RGB8 deflate / rgba | 131.35 | 41.70 | 3.15× | 1.894 | 22.01× |
| RGB8 deflate / crop | 15.92 | 3.839 | 4.15× | 0.2050 | 18.72× |
| Float32 LERC plain / native | 408.33 | 58.85 | 6.94× | 2.018 | 29.16× |
| Float32 LERC deflate / native | 415.76 | 62.80 | 6.62× | 3.088 | 20.33× |
| Float32 LERC zstd / native | 407.14 | 63.68 | 6.39× | 3.054 | 20.85× |
| 16 MP Gray8 single strip deflate / crop (default chopping) | 478.30 | 105.57 | 4.53× | 4.065 | 25.97× |
| 16 MP Gray8 single strip deflate / firstrow (default chopping) | 312.76 | 65.08 | 4.81× | 0.0200 | 3255.25× |

## Main-suite geometric means

| Operation | Cases | Before / 4.7.1 | After / 4.7.1 | Before / after |
| --- | --- | --- | --- | --- |
| native | 30 | 13.24× | 8.56× | 1.55× |
| scan | 9 | 6.68× | 4.72× | 1.41× |
| rgba | 22 | 17.80× | 15.66× | 1.14× |
| crop | 10 | 9.73× | 5.44× | 1.79× |

Median slowdowns greater than 10% in this run: gray-deflate / scan (main): 1.20×, rgb-zstd / native (main): 1.22×, noise-lzw / native (main): 1.14×, jpeg-smooth / rgba (main): 1.16×, rgb-tiled-lzw / native (main): 1.20×, complex64-lzw / native (main): 1.24×, fax-rle / native (main): 1.14×, fax-g3 / native (main): 1.16×, many-strips / open (main): 1.15×, original-quad-lzw-compat.tiff / native (main): 1.22×, layout-rgb-deflate-16 / crop (layout): 1.27×, large-strip-lzw / firstrow (defaults): 1.14×. These are measured differences, not statistical significance claims. Every such workload was rechecked in nine randomized 500 ms batches per engine. Slowdowns greater than 10% remaining in the longer run: gray-deflate / scan: 1.12×, layout-rgb-deflate-16 / crop: 1.13×. All original results remain in the main table and geometric means; the longer batches are separate evidence.

Longer-batch rechecks (speedup below one means slower):

| Workload | Main speedup | Recheck before ms | Recheck after ms | Recheck speedup |
| --- | --- | --- | --- | --- |
| gray-deflate / scan (main) | 0.833× | 3.511 | 3.936 | 0.892× |
| rgb-zstd / native (main) | 0.822× | 1.555 | 1.617 | 0.961× |
| noise-lzw / native (main) | 0.876× | 60.74 | 48.95 | 1.241× |
| jpeg-smooth / rgba (main) | 0.860× | 54.99 | 55.74 | 0.987× |
| rgb-tiled-lzw / native (main) | 0.835× | 19.43 | 17.34 | 1.120× |
| complex64-lzw / native (main) | 0.807× | 122.23 | 126.33 | 0.968× |
| fax-rle / native (main) | 0.878× | 8.139 | 7.847 | 1.037× |
| fax-g3 / native (main) | 0.862× | 17.59 | 17.88 | 0.984× |
| many-strips / open (main) | 0.869× | 0.0018 | 0.0019 | 0.986× |
| original-quad-lzw-compat.tiff / native (main) | 0.818× | 5.470 | 5.290 | 1.034× |
| layout-rgb-deflate-16 / crop (layout) | 0.786× | 1.695 | 1.923 | 0.882× |
| large-strip-lzw / firstrow (defaults) | 0.877× | 99.49 | 98.89 | 1.006× |

## Changes and proof boundary

Deflate now binds an input window once per bounded batch, with a checked refinement of the original transitions. Two work units per transition preserve the former adapter-loop-plus-core-step charge.

Complete native Deflate segment requests decode and validate once into private staging. Checksum, strict ending and permitted final-strip padding are checked before application storage changes. Partial reads retain a validation pass and bounded replay; whole-buffer requests avoid an extra result copy.

LERC normalization replaces bit-by-bit top-bit searches with standard leading-zero operations, proved equal to the existing top model. Software IEEE rounding, acceptance, sample semantics and conservative work quotas are unchanged. No native codec runtime dependency was introduced.

The full core proof passed 1,638 obligations with zero errors; all 72 negative gates rejected their mutations. 52 Rust tests, 97 core Python methods, 57 C methods, the complete C ASan/UBSan suite, 31 original images and 20,000 deterministic core mutations passed. The separate libtiff oracle passed its own 108 tests. C sanitizers do not instrument Rust; LeakSanitizer is disabled because it cannot run under this runner.

Partial Deflate reads still validate the entire compressed segment before exposing any bytes. Their first-row time is therefore not equivalent to libtiff's early-return validation policy. Large LZW reads, RGBA conversion and complete-segment LERC decoding retain earlier costs. Peak RSS is a process measurement and small differences may be noise. Results are specific to these layouts, inputs, codec builds and host; the reader is still experimental. The formal claims cover the core models and local batch refinement; input-window continuity, private-buffer commit and TIFF/C composition remain tested adapter code.

## Identical RGB pixels, different strip heights

| Input / operation | Before ms | After ms | Speedup | 4.7.1 ms | After / 4.7.1 |
| --- | --- | --- | --- | --- | --- |
| RGB Deflate, 16 rows/strip / native | 12.80 | 13.24 | 0.97× | 1.596 | 8.30× |
| RGB Deflate, 16 rows/strip / crop | 1.817 | 2.313 | 0.79× | 0.2444 | 9.46× |
| RGB Deflate, 32 rows/strip / native | 124.18 | 13.23 | 9.39× | 1.437 | 9.21× |
| RGB Deflate, 32 rows/strip / crop | 15.25 | 3.171 | 4.81× | 0.1846 | 17.18× |
| RGB Deflate, 64 rows/strip / native | 121.16 | 14.66 | 8.26× | 1.221 | 12.01× |
| RGB Deflate, 64 rows/strip / crop | 15.60 | 3.568 | 4.37× | 0.1946 | 18.34× |
| RGB Deflate, 256 rows/strip / native | 122.58 | 12.43 | 9.86× | 1.183 | 10.51× |
| RGB Deflate, 256 rows/strip / crop | 48.89 | 9.673 | 5.05× | 0.4018 | 24.08× |

## Ordinary strip chopping

| Input / operation | Before ms | After ms | Speedup | 4.7.1 ms | After / 4.7.1 |
| --- | --- | --- | --- | --- | --- |
| 16 MP Gray8 single strip none / firstrow (default chopping) | 0.0025 | 0.0025 | 1.02× | 0.0079 | 0.31× |
| 16 MP Gray8 single strip none / crop (default chopping) | 0.1949 | 0.1955 | 1.00× | 0.0263 | 7.45× |
| 16 MP Gray8 single strip lzw / firstrow (default chopping) | 105.96 | 120.89 | 0.88× | 0.1765 | 685.08× |
| 16 MP Gray8 single strip lzw / crop (default chopping) | 148.66 | 153.15 | 0.97× | 10.08 | 15.20× |
| 16 MP Gray8 single strip deflate / firstrow (default chopping) | 312.76 | 65.08 | 4.81× | 0.0200 | 3255.25× |
| 16 MP Gray8 single strip deflate / crop (default chopping) | 478.30 | 105.57 | 4.53× | 4.065 | 25.97× |

Peak process RSS, MiB:

| Input / operation | Before | After | 4.7.1 |
| --- | --- | --- | --- |
| 16 MP Gray8 single strip none / firstrow | 1.63 | 1.62 | 2.24 |
| 16 MP Gray8 single strip none / crop | 1.74 | 1.71 | 2.30 |
| 16 MP Gray8 single strip lzw / firstrow | 1.85 | 1.82 | 7.43 |
| 16 MP Gray8 single strip lzw / crop | 1.92 | 1.90 | 23.65 |
| 16 MP Gray8 single strip deflate / firstrow | 1.89 | 1.89 | 2.41 |
| 16 MP Gray8 single strip deflate / crop | 1.98 | 2.03 | 18.39 |

## Method and reproduction

Measured 2026-10-15T11:13:47.844401+00:00 on AMD EPYC 9V74 80-Core Processor, CPU 0, with 7 randomized serial batches of at least 100 ms per engine/workload. The exact prior sources and current sources were freshly built with the same pinned toolchain. Each operation uses the unchanged C consumer and a fresh TIFF handle. Timing uses common in-memory callbacks, excluding file loading, output allocation, hashing and process startup. Separate file-backed processes measure peak RSS three times; RSS includes caller buffers and the process, not just library allocations. The main suite and layout sweep disable mmap and strip chopping; the default group enables ordinary strip chopping. The reference is freshly built libtiff 4.7.1, zlib rather than libdeflate, with pinned JPEG SIMD disabled, LERC 4.0.0, WebP 1.6.0 and Zstandard 1.5.7. Quotas are raised identically to the original benchmark.

See [benchmarks/README.md](../benchmarks/README.md) for commands. Raw batches, exact-output hashes, proof/gate receipts, source hashes and regression logs are in `docs/evidence/reader-optimization/`. The shipped reverse patch and baseline restorer check every archived core/C source before building. The original comparison remains in [reader-benchmark.md](reader-benchmark.md).
