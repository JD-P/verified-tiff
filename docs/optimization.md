# Verified decoder optimization

This is the historical v17-to-v18 optimization pass. See [profiling.md](profiling.md)
for the current JPEG/Deflate profiling pass and v18-to-v19 measurements.

Measured 2026-10-14T22:19:23.795613+00:00 on AMD EPYC 9V74 80-Core Processor, pinned to logical CPU 0. The previous v17 decoder and the optimized decoder were both freshly compiled with the same pinned toolchain and measured in interleaved batches on this host. These are new before/after measurements, not a comparison with the historical report’s timings.

## Results

| Workload set | Native speedup | C API RGBA speedup |
| --- | ---: | ---: |
| Original 22 workloads (20 RGBA) | 2.76× | 1.64× |
| All 26 workloads (24 RGBA) | 3.71× | 2.19× |

These are geometric means of per-workload median ratios. They describe this chosen suite; they are not a claim about an average user workload. Large gains in formerly inefficient uncompressed paths strongly influence the summary. Individual timings follow.

Across the original 22 workloads, the native decoder’s geometric-mean time relative to pinned libtiff changes from 39.51× to 14.32×. The C API’s ratio against system libtiff changes from 47.97× to 29.17×. Ratios above 1 mean slower than libtiff.

## What changed

- Uncompressed inspection validates segment storage and uses the existing coverage proof to establish every sample address. The redundant per-sample execution loop is removed.
- Uncompressed 8-bit strips use verified contiguous row copies. WhiteIsZero uses a proved inversion loop; alpha keeps the existing normalization pass. Packed bilevel and tiles keep scalar decoding.
- Full-width 8-bit scatter uses a simpler proved index expression, with inline helpers. This benefits basic PackBits/LZW paths without changing stream semantics.

Only `src/lib.rs` changed among the 51 verified core files. The other core files and production C adapter are unchanged. No codec dependency, unsafe core code, external proof body, axiom, or weakened contract is added. Budget charges remain conservative and unchanged, including insufficient-budget prefix behavior for plain uncompressed decode. The bulk copy uses safe slices and the existing upstream vstd contract for `copy_from_slice`.

## Native inspect plus decode

Milliseconds per image, median of 7 warmed batches. Both native engines include inspection/open, decode and canonical sample conversion into reusable buffers. “New/reference” is a time ratio; smaller is better.

| Workload | Before ms | After ms | Speedup | libtiff ms | New/reference |
| --- | ---: | ---: | ---: | ---: | ---: |
| Gray8 / none (1024×1024) | 52.4301 | 0.0344 | 1522.84× | 0.0350 | 0.98× |
| Gray8 / PackBits (1024×1024) | 15.0589 | 7.1831 | 2.10× | 1.2817 | 5.60× |
| Gray8 / LZW (1024×1024) | 21.8648 | 14.9502 | 1.46× | 0.9271 | 16.13× |
| Gray8 / Deflate (1024×1024) | 29.0313 | 23.9284 | 1.21× | 0.4249 | 56.32× |
| RGB8 / none (1024×1024) | 145.1315 | 0.0911 | 1592.57× | 0.0825 | 1.10× |
| RGB8 / LZW (1024×1024) | 71.1110 | 52.0636 | 1.37× | 2.8895 | 18.02× |
| RGB8 / Deflate (1024×1024) | 86.2285 | 70.8862 | 1.22× | 1.2309 | 57.59× |
| RGB8 / JPEG (smooth blocks) (1024×1024) | 632.6304 | 592.7494 | 1.07× | 3.2263 | 183.72× |
| RGB8 / JPEG (random texture) (1024×1024) | 1320.6608 | 1290.6077 | 1.02× | 24.8418 | 51.95× |
| RGB8 / tiled LZW (1024×1024) | 60.2411 | 57.6428 | 1.05× | 3.0699 | 18.78× |
| Gray16 / Deflate (1024×1024) | 429.2340 | 358.1692 | 1.20× | 8.1853 | 43.76× |
| Float64 / LZW predictor 2 (1024×1024) | 158.0390 | 104.9965 | 1.51× | 12.4818 | 8.41× |
| Random Gray8 / LZW (1024×1024) | 59.9542 | 51.7281 | 1.16× | 2.1642 | 23.90× |
| Random Gray8 / Deflate (1024×1024) | 37.7515 | 33.4028 | 1.13× | 0.3706 | 90.14× |
| Bilevel / CCITT Group 3 (1024×1024) | 26.8552 | 14.4929 | 1.85× | 1.7258 | 8.40× |
| Bilevel / CCITT Group 4 (1024×1024) | 21.4454 | 9.4214 | 2.28× | 1.7486 | 5.39× |
| RGB8 / lossless WebP subset (1024×1024) | 59.1103 | 47.8766 | 1.23× | 1.2248 | 39.09× |
| Native LogLuv16 / SGILog32 (512×512) | 26.5692 | 7.9630 | 3.34× | 1.1286 | 7.06× |
| Original legacy LZW (512×384) | 13.9441 | 10.5009 | 1.33× | 1.3180 | 7.97× |
| Original 24.3 MP LZW scan (7795×3122) | 112.4059 | 113.7793 | 0.99× | 29.9325 | 3.80× |
| Original 8.3 MP Group 4 scan (2453×3369) | 124.8459 | 28.9269 | 4.32× | 12.1116 | 2.39× |
| Original Deflate final-strip case (500×500) | 10.0936 | 8.1076 | 1.24× | 0.2199 | 36.87× |
| WhiteIsZero Gray8 / none (509×513) | 11.7374 | 0.0132 | 892.53× | 0.1203 | 0.11× |
| RGBA8 straight alpha / none (509×513) | 51.4661 | 2.0287 | 25.37× | 0.0437 | 46.42× |
| RGBA8 / uncompressed padded tiles (509×513) | 53.6305 | 22.7083 | 2.36× | 0.1149 | 197.67× |
| Bilevel / none (509×513) | 11.4888 | 4.6281 | 2.48× | 0.3008 | 15.39× |

## C API fresh-handle RGBA

The same C consumer opens, renders and closes a new in-memory handle each iteration. Internal allocations and adapter conversion are included; the decoded-image cache is not reused between iterations. Float64 and native LogLuv are excluded from RGBA.

| Workload | Before ms | After ms | Speedup | Pinned libtiff ms | System libtiff ms |
| --- | ---: | ---: | ---: | ---: | ---: |
| Gray8 / none | 85.6442 | 3.8111 | 22.47× | 0.5428 | 0.7099 |
| Gray8 / PackBits | 18.3812 | 10.9793 | 1.67× | 1.8210 | 1.9585 |
| Gray8 / LZW | 27.7222 | 23.9931 | 1.16× | 1.4204 | 1.5791 |
| Gray8 / Deflate | 39.9052 | 35.1832 | 1.13× | 0.8834 | 0.8067 |
| RGB8 / none | 243.5247 | 7.2390 | 33.64× | 0.7125 | 0.7167 |
| RGB8 / LZW | 83.8781 | 80.7902 | 1.04× | 3.5935 | 3.4530 |
| RGB8 / Deflate | 125.0026 | 104.3005 | 1.20× | 1.9452 | 0.8710 |
| RGB8 / JPEG (smooth blocks) | 651.7427 | 625.5292 | 1.04× | 3.8239 | 2.6854 |
| RGB8 / JPEG (random texture) | 1664.6569 | 1587.2383 | 1.05× | 25.2823 | 20.0344 |
| RGB8 / tiled LZW | 69.9728 | 74.8481 | 0.93× | 3.2303 | 3.3546 |
| Gray16 / Deflate | 619.8888 | 616.4936 | 1.01× | 8.0310 | 5.2445 |
| Random Gray8 / LZW | 74.6874 | 92.4541 | 0.81× | 2.6914 | 2.9786 |
| Random Gray8 / Deflate | 51.1875 | 47.2936 | 1.08× | 1.0289 | 0.8802 |
| Bilevel / CCITT Group 3 | 36.7655 | 24.2261 | 1.52× | 0.7634 | 0.7391 |
| Bilevel / CCITT Group 4 | 28.4131 | 16.4693 | 1.73× | 0.7388 | 0.8383 |
| RGB8 / lossless WebP subset | 73.2103 | 59.8854 | 1.22× | 1.8447 | 1.8396 |
| Original legacy LZW | 20.1647 | 16.0027 | 1.26× | 1.4539 | 1.4309 |
| Original 24.3 MP LZW scan | 225.7227 | 227.1002 | 0.99× | 7.4093 | 7.4975 |
| Original 8.3 MP Group 4 scan | 163.4361 | 62.8245 | 2.60× | 4.0461 | 4.2127 |
| Original Deflate final-strip case | 13.7619 | 12.7271 | 1.08× | 0.3394 | 0.3083 |
| WhiteIsZero Gray8 / none | 20.9032 | 0.8427 | 24.80× | 0.1441 | 0.1773 |
| RGBA8 straight alpha / none | 86.7658 | 3.8852 | 22.33× | 0.3458 | 0.3498 |
| RGBA8 / uncompressed padded tiles | 86.6333 | 24.2754 | 3.57× | 0.4300 | 0.4172 |
| Bilevel / none | 18.8841 | 5.5016 | 3.43× | 0.0809 | 0.0810 |

### Slower measured cases

These median increases exceed 5%. The optimization does not improve every workload.

| Workload | Path | Before ms | After ms | Increase |
| --- | --- | ---: | ---: | ---: |
| RGB8 / tiled LZW | C API RGBA | 69.9728 | 74.8481 | 7.0% |
| Random Gray8 / LZW | C API RGBA | 74.6874 | 92.4541 | 23.8% |

A separate five-batch probe at 160 ms per batch compared the old decoder, this implementation, and an isolated variant without the two new inline hints. Removing those hints did not eliminate the random-data LZW C API slowdown, so that variant was not adopted. The probe is retained in `evidence/optimization-inlining-probe.json`; its timings are not substituted into the tables or geometric means. This adapter regression remains a performance issue for future work.

## Correctness and verification

All 26 before/after native outputs and 24 before/after RGBA outputs are byte-for-byte identical, including JPEG. The four added uncompressed fixtures also match independent Python-generated pixels. Lossless comparisons with libtiff are exact.

The only non-exact reference comparisons are JPEG:

| Workload | Engine | Maximum channel error | Mean absolute error |
| --- | --- | ---: | ---: |
| RGB8 / JPEG (random texture) | before | 1 | 0.020825 |
| RGB8 / JPEG (random texture) | after | 1 | 0.020825 |
| RGB8 / JPEG (random texture) | before_rgba | 1 | 0.015619 |
| RGB8 / JPEG (random texture) | after_rgba | 1 | 0.015619 |

The complete optimized crate passed 915 verification obligations with zero errors and the full-crate flag set. All 33 negative proof checks failed for their intended semantic postconditions, including two new checks that omit the bulk copy or corrupt the shortcut index. The 32 Rust tests, 87 Python methods, 5,616 generated matrix cases, 31 original TIFF fixtures, 11 adapter test methods, 20,000 core mutations and 2,000 adapter mutations passed. The independently built upstream libtiff suite passed all 108 tests; that suite tests libtiff itself. See [validation.md](validation.md) for exact scope and retained logs.

## Method and limits

There are 1,218 measured batches: 7 per engine/workload, each at least 80 ms or one complete operation if longer. Engine order is reproducibly shuffled. Each process warms and calibrates before timing, and every batch’s output hash matches its preflight. Disk I/O, process startup, fixture creation, dumping and output comparison are excluded.

The host is a shared VM, not an isolated benchmark machine. Small differences in unchanged paths may reflect scheduling, code layout or measurement variation. The following intervals show the middle 50% of each native batch distribution; all samples, minima/maxima and RGBA intervals are retained in the raw JSON.

| Workload | Before native Q1–Q3 ms | After native Q1–Q3 ms |
| --- | ---: | ---: |
| Gray8 / none | 51.0175–54.3167 | 0.0339–0.0360 |
| Gray8 / PackBits | 14.1586–15.9657 | 6.9254–7.5945 |
| Gray8 / LZW | 21.7003–22.2890 | 14.8209–15.1278 |
| Gray8 / Deflate | 28.5602–29.4291 | 23.8494–24.1886 |
| RGB8 / none | 143.4447–146.1668 | 0.0904–0.0933 |
| RGB8 / LZW | 69.9239–73.8232 | 50.6013–52.9537 |
| RGB8 / Deflate | 83.5661–90.3256 | 70.0351–74.2938 |
| RGB8 / JPEG (smooth blocks) | 619.1082–651.0890 | 586.5306–618.1981 |
| RGB8 / JPEG (random texture) | 1280.4531–1371.7929 | 1230.2696–1328.6506 |
| RGB8 / tiled LZW | 57.3053–61.3070 | 55.6706–62.9047 |
| Gray16 / Deflate | 425.5450–447.2992 | 355.0482–361.6949 |
| Float64 / LZW predictor 2 | 155.6223–162.5422 | 100.8937–105.3947 |
| Random Gray8 / LZW | 58.9888–62.3188 | 51.4032–52.6966 |
| Random Gray8 / Deflate | 37.3691–37.8773 | 33.1194–33.9937 |
| Bilevel / CCITT Group 3 | 26.6631–27.1149 | 14.3453–15.5597 |
| Bilevel / CCITT Group 4 | 21.2028–22.7662 | 9.3487–9.4743 |
| RGB8 / lossless WebP subset | 58.3061–60.6938 | 47.4022–48.5165 |
| Native LogLuv16 / SGILog32 | 26.3024–26.7033 | 7.8772–7.9759 |
| Original legacy LZW | 13.8844–14.3869 | 10.4657–10.6397 |
| Original 24.3 MP LZW scan | 109.0675–114.7801 | 112.6836–115.4372 |
| Original 8.3 MP Group 4 scan | 123.0512–127.4526 | 28.1615–29.8373 |
| Original Deflate final-strip case | 9.8460–10.2665 | 7.9941–8.2028 |
| WhiteIsZero Gray8 / none | 11.6101–12.2863 | 0.0127–0.0132 |
| RGBA8 straight alpha / none | 50.8660–54.2480 | 2.0179–2.2322 |
| RGBA8 / uncompressed padded tiles | 52.7708–54.6934 | 22.3276–23.8371 |
| Bilevel / none | 11.4063–11.6623 | 4.5804–4.6762 |

Rust uses Cargo release with normal safety checks; consumers use GCC `-O3` and pinned libtiff 4.7.1 uses `-O2`. Pinned libjpeg-turbo has SIMD disabled for numerical regression tests. The system RGBA reference is distribution libtiff 4.5.1 and its installed codecs; it is a useful production configuration but not a controlled SIMD-only comparison. Both Rust versions use the same 256 MiB input/output limits and 10^12 work quota. No solver, proof or ghost state executes in the decoder.

The main generated inputs are 1024×1024; native LogLuv is 512×512. Four unchanged upstream regressions include large scans. The four extra 509×513 cases cover WhiteIsZero, straight RGBA, padded RGBA tiles and bilevel. This is warm-cache single-thread performance. Cold storage, concurrency, allocations, peak memory, and representative real-world frequencies are not measured. There are no native palette/planar benchmarks in this suite.

Remaining work includes tiled row copies, bilevel expansion, wider-sample paths, individual codec loops and adapter inspection/allocation overhead. The measured improvement does not establish general parity with libtiff, or full libtiff API/profile compatibility. The C adapter remains outside the formal proof boundary.

The initial padded RGB8 tile preflight failed in the pinned libtiff in-memory RGBA reader with “Expected 768, got 1024”, although native decoding matched the independent pixels. The measured tiled case therefore uses RGBA8 with 1024-byte tiles. This is an explicit reference limitation; the decoder’s existing RGB tile correctness tests remain passing.

## Reproduce

Follow [benchmarks/README.md](../benchmarks/README.md). The baseline helper restores the exact previous core from the shipped reverse patch and checks every file against its archived proof hashes. The comparison requires a fresh full-crate proof summary for the optimized source. Raw results are in [optimization-results.json](evidence/optimization-results.json), including compiler, library, source, binary and input hashes.

Compiler: `cc (Ubuntu 13.3.0-6ubuntu2~24.04) 13.3.0`. Rust: `rustc 1.98.1 (48a229cea 2026-09-01)`.

```
libdeflate0:amd64	1.19-1build1.1
libjpeg-turbo8:amd64	2.1.5-2ubuntu2
libtiff6:amd64	4.5.1+git230720-4ubuntu2.5
libwebp7:amd64	1.3.2-0.4build3
zlib1g:amd64	1:1.3.dfsg-3.1ubuntu2.1
```

The 24 initial workloads and two supplemental workloads were measured in separate runs on this host. `merge-comparisons.py` required identical core/adapter sources, engine binaries, libraries, CPU affinity and measurement settings, and rejected duplicate cases. Every run’s complete metadata and its case list are retained under `measurement_runs` in the JSON. The original harness is preserved as `evidence/optimization-initial-harness.py`; the supplemental harness changes the tiled fixture and adds an output-path option. No samples were selected or discarded to improve the reported ratios, and the strict single-run resume guard remains intact.
