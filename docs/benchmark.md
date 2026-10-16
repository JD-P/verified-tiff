# Decoder benchmark against libtiff

> Historical v17 baseline, before the optimizations. See [the current optimization report](optimization.md) for fresh same-host before/after measurements.

Measured on AMD EPYC 9V74 80-Core Processor under KVM/Linux, pinned to logical CPU 0. The runtime timestamp is 2026-10-14T20:02:00.564251+00:00. These are medians of 7 separately warmed process batches, each at least 80 ms or one full operation. Inputs and outputs are resident in memory; disk I/O and process startup are excluded.

**The current implementation is substantially slower.** Across these 22 chosen workloads, the geometric mean of the Rust core's complete-read time divided by pinned libtiff's is **39.4×** (range 3.7–1736.2×). The complete C API RGBA path is **44.5×** slower than pinned libtiff across 20 applicable cases, and **47.6×** slower than Ubuntu's system libtiff. These suite summaries do not estimate an application workload distribution.

## Complete native-image read

Rust includes inspection and decoding into reusable buffers. Libtiff includes in-memory open, metadata setup, decoding/canonical output conversion, and close. Both produce the same raster representation. Caller allocation and input loading are excluded; library-internal allocations are included. Higher ratios mean verified-tiff is slower.

| Workload | MP | Input KiB | Rust ms | libtiff 4.7.1 ms | Ratio |
| --- | ---: | ---: | ---: | ---: | ---: |
| Gray8 / none | 1.049 | 1024.3 | 50.407 | 0.034 | 1474.1× |
| Gray8 / PackBits | 1.049 | 256.3 | 13.688 | 1.230 | 11.1× |
| Gray8 / LZW | 1.049 | 239.8 | 21.843 | 0.925 | 23.6× |
| Gray8 / Deflate | 1.049 | 9.6 | 29.906 | 0.451 | 66.4× |
| RGB8 / none | 1.049 | 3072.3 | 149.358 | 0.086 | 1736.2× |
| RGB8 / LZW | 1.049 | 868.2 | 73.470 | 3.041 | 24.2× |
| RGB8 / Deflate | 1.049 | 28.4 | 84.886 | 1.228 | 69.1× |
| RGB8 / JPEG (smooth blocks) | 1.049 | 53.4 | 653.191 | 3.249 | 201.0× |
| RGB8 / tiled LZW | 1.049 | 349.4 | 65.410 | 3.335 | 19.6× |
| Gray16 / Deflate | 1.049 | 2020.0 | 464.828 | 8.303 | 56.0× |
| Float64 / LZW predictor 2 | 1.049 | 216.2 | 178.188 | 12.745 | 14.0× |
| Random Gray8 / LZW | 1.049 | 1401.1 | 62.226 | 2.329 | 26.7× |
| Random Gray8 / Deflate | 1.049 | 1024.7 | 39.655 | 0.393 | 100.9× |
| Bilevel / CCITT Group 3 | 1.049 | 46.9 | 27.187 | 1.770 | 15.4× |
| Bilevel / CCITT Group 4 | 1.049 | 16.4 | 24.379 | 1.961 | 12.4× |
| RGB8 / lossless WebP subset | 1.049 | 1.6 | 61.228 | 1.363 | 44.9× |
| Native LogLuv16 / SGILog32 | 0.262 | 28.8 | 27.781 | 1.159 | 24.0× |
| Original legacy LZW | 0.197 | 209.3 | 14.352 | 1.264 | 11.4× |
| Original 24.3 MP LZW scan | 24.336 | 74.5 | 114.058 | 30.482 | 3.7× |
| Original 8.3 MP Group 4 scan | 8.264 | 38.7 | 129.178 | 12.224 | 10.6× |
| Original Deflate final-strip case | 0.250 | 12.5 | 10.591 | 0.246 | 43.1× |
| RGB8 / JPEG (random texture) | 1.049 | 2070.2 | 1278.870 | 25.107 | 50.9× |

## Inspection versus prepared decoding

The descriptor/handle and caller output buffers are prepared before the decode-only measurements. Every measured iteration performs decoding again. Rust inspection validates the compressed stream or sample addresses, not just the TIFF directory. Separate medians need not add exactly because they come from different batches.

| Workload | Rust inspection ms | Rust decode ms | libtiff decode ms | Decode ratio |
| --- | ---: | ---: | ---: | ---: |
| Gray8 / none | 30.466 | 21.147 | 0.029 | 738.6× |
| Gray8 / PackBits | 0.292 | 13.175 | 1.209 | 10.9× |
| Gray8 / LZW | 4.691 | 16.907 | 0.914 | 18.5× |
| Gray8 / Deflate | 7.961 | 22.069 | 0.439 | 50.3× |
| RGB8 / none | 89.577 | 61.886 | 0.079 | 780.4× |
| RGB8 / LZW | 17.855 | 53.198 | 2.880 | 18.5× |
| RGB8 / Deflate | 21.805 | 64.092 | 1.242 | 51.6× |
| RGB8 / JPEG (smooth blocks) | 12.427 | 635.742 | 3.382 | 188.0× |
| RGB8 / tiled LZW | 8.175 | 58.085 | 3.192 | 18.2× |
| Gray16 / Deflate | 216.996 | 236.067 | 8.337 | 28.3× |
| Float64 / LZW predictor 2 | 4.839 | 167.594 | 12.497 | 13.4× |
| Random Gray8 / LZW | 28.238 | 35.177 | 2.367 | 14.9× |
| Random Gray8 / Deflate | 12.031 | 27.730 | 0.391 | 70.9× |
| Bilevel / CCITT Group 3 | 6.375 | 20.868 | 1.735 | 12.0× |
| Bilevel / CCITT Group 4 | 3.898 | 18.897 | 1.867 | 10.1× |
| RGB8 / lossless WebP subset | 1.253 | 60.204 | 1.216 | 49.5× |
| Native LogLuv16 / SGILog32 | 0.039 | 27.824 | 1.209 | 23.0× |
| Original legacy LZW | 3.086 | 11.080 | 1.296 | 8.6× |
| Original 24.3 MP LZW scan | 1.546 | 114.360 | 30.911 | 3.7× |
| Original 8.3 MP Group 4 scan | 6.248 | 120.456 | 11.894 | 10.1× |
| Original Deflate final-strip case | 3.484 | 7.059 | 0.213 | 33.1× |
| RGB8 / JPEG (random texture) | 330.449 | 940.341 | 25.778 | 36.5× |

## Complete RGBA read through the C API

The identical C consumer opens a fresh in-memory handle, renders with `TIFFReadRGBAImageOriented`, and closes it. The adapter therefore cannot answer timed iterations from its previous decoded-raster cache. Float64 and LogLuv have no supported adapter RGBA conversion and are omitted.

| Workload | Adapter ms | Pinned libtiff ms | System libtiff ms | Adapter / pinned | Adapter / system |
| --- | ---: | ---: | ---: | ---: | ---: |
| Gray8 / none | 85.990 | 0.540 | 0.787 | 159.1× | 109.3× |
| Gray8 / PackBits | 17.575 | 1.687 | 1.857 | 10.4× | 9.5× |
| Gray8 / LZW | 27.520 | 1.419 | 1.608 | 19.4× | 17.1× |
| Gray8 / Deflate | 41.408 | 0.918 | 0.815 | 45.1× | 50.8× |
| RGB8 / none | 249.848 | 0.805 | 0.814 | 310.2× | 307.1× |
| RGB8 / LZW | 87.418 | 3.458 | 3.437 | 25.3× | 25.4× |
| RGB8 / Deflate | 119.147 | 1.785 | 0.893 | 66.8× | 133.5× |
| RGB8 / JPEG (smooth blocks) | 698.173 | 3.815 | 2.879 | 183.0× | 242.5× |
| RGB8 / tiled LZW | 79.835 | 4.151 | 3.936 | 19.2× | 20.3× |
| Gray16 / Deflate | 693.721 | 8.418 | 5.835 | 82.4× | 118.9× |
| Random Gray8 / LZW | 80.153 | 2.895 | 3.182 | 27.7× | 25.2× |
| Random Gray8 / Deflate | 55.456 | 1.116 | 0.925 | 49.7× | 60.0× |
| Bilevel / CCITT Group 3 | 37.831 | 0.765 | 0.755 | 49.5× | 50.1× |
| Bilevel / CCITT Group 4 | 31.793 | 0.843 | 0.882 | 37.7× | 36.0× |
| RGB8 / lossless WebP subset | 76.363 | 1.985 | 1.909 | 38.5× | 40.0× |
| Original legacy LZW | 18.703 | 1.380 | 1.443 | 13.6× | 13.0× |
| Original 24.3 MP LZW scan | 238.466 | 7.523 | 7.749 | 31.7× | 30.8× |
| Original 8.3 MP Group 4 scan | 166.041 | 4.384 | 4.250 | 37.9× | 39.1× |
| Original Deflate final-strip case | 15.216 | 0.368 | 0.332 | 41.3× | 45.9× |
| RGB8 / JPEG (random texture) | 1611.366 | 25.525 | 19.735 | 63.1× | 81.7× |

## What explains the gap

Source inspection shows several concrete costs. This is an explanation consistent with the timing breakdown, not a sampled CPU profile or a measured speedup from an optimization:

- The basic uncompressed path calls `sample` for every output byte during inspection and again during decoding. Each call recomputes geometry using divisions/remainders, reads offset/count metadata, and checks ranges. Ordinary uncompressed libtiff reads can largely copy whole runs of bytes.
- The C adapter inspects at open and reinspects before its first decode, then allocates canonical/scratch/render buffers, converts the canonical image to RGBA, and copies it to the caller. The extra inspection is visible in cases where validation is expensive.
- Compressed core paths use conservative scalar implementations and per-byte/per-pixel work and scatter operations. The measured JPEG and other codec gaps remain even after removing initial inspection.
- Proofs and ghost code are erased. These numbers measure this implementation and adapter, not an intrinsic runtime cost of formal verification or Rust. A verified segment-wise copy path and reuse of the inspected descriptor are concrete optimization candidates, but no such changes or projected speedups were made in this task.

## Correctness and scope

All timed batches were checked against their engine's validated output hash. Lossless comparisons were byte-exact; the only non-exact comparisons were: RGB8 / JPEG (random texture): libtiff_decode maximum absolute difference 1/255, mean 0.020825. RGB8 / JPEG (random texture): adapter_rgba maximum absolute difference 1/255, mean 0.015619. Independent libtiff writers generated the main TIFF fixtures; the existing independent Python writers generated the supported WebP and LogLuv subsets. Four upstream fixtures were used unchanged.

The core and production C adapter source hashes match the previous delivered version. The benchmark adds standalone consumers, fixture generation and reporting only. The prior 899-obligation proof result is unchanged; a new proof run was not needed for these benchmark-only additions.

The primary reference is real libtiff 4.7.1, optimized at `-O2`, with pinned JPEG/WebP dependencies and system zlib. The pinned libjpeg-turbo build intentionally disables SIMD for numerical regression tests. The system baseline is Ubuntu libtiff 4.5.1 plus distribution codecs, including libjpeg-turbo and libdeflate. Different versions and codec configurations confound any SIMD-only attribution. Rust uses Cargo release (`opt-level=3`), and the C benchmark consumers use GCC `-O3`. No native-CPU specialization or sanitizer is enabled.

All measurements are single-threaded, warm-cache and in a shared VM. Small cached uncompressed images particularly favor highly optimized memory copies; cold storage, larger working sets, concurrency, peak RSS, and allocation counts are not measured. The selected synthetic patterns and four real regressions are not representative of every TIFF workload. The generous finite benchmark limits are 256 MiB per buffer and 10^12 core work units; checks remain enabled. Default limits would reject some of these large workloads.

## Variation: median [25th, 75th percentile] in milliseconds

| Workload | Rust complete read | libtiff complete read | Adapter RGBA |
| --- | ---: | ---: | ---: |
| Gray8 / none | 50.407 [49.498, 51.307] | 0.034 [0.033, 0.039] | 85.990 [83.999, 92.360] |
| Gray8 / PackBits | 13.688 [13.474, 13.864] | 1.230 [1.217, 1.240] | 17.575 [17.447, 18.190] |
| Gray8 / LZW | 21.843 [21.585, 23.495] | 0.925 [0.920, 0.933] | 27.520 [27.218, 27.624] |
| Gray8 / Deflate | 29.906 [29.195, 32.935] | 0.451 [0.441, 0.469] | 41.408 [39.866, 42.721] |
| RGB8 / none | 149.358 [147.831, 152.605] | 0.086 [0.085, 0.092] | 249.848 [246.964, 251.597] |
| RGB8 / LZW | 73.470 [71.659, 76.238] | 3.041 [2.890, 3.099] | 87.418 [86.293, 89.873] |
| RGB8 / Deflate | 84.886 [83.886, 85.714] | 1.228 [1.218, 1.234] | 119.147 [117.351, 124.759] |
| RGB8 / JPEG (smooth blocks) | 653.191 [634.516, 668.840] | 3.249 [3.207, 3.546] | 698.173 [655.646, 706.819] |
| RGB8 / tiled LZW | 65.410 [64.443, 71.374] | 3.335 [3.151, 4.620] | 79.835 [77.739, 85.653] |
| Gray16 / Deflate | 464.828 [446.829, 469.276] | 8.303 [8.181, 8.582] | 693.721 [653.708, 699.741] |
| Float64 / LZW predictor 2 | 178.188 [172.613, 180.343] | 12.745 [12.381, 13.088] | — |
| Random Gray8 / LZW | 62.226 [61.439, 63.018] | 2.329 [2.268, 2.449] | 80.153 [74.216, 81.546] |
| Random Gray8 / Deflate | 39.655 [38.745, 40.382] | 0.393 [0.387, 0.405] | 55.456 [53.860, 56.482] |
| Bilevel / CCITT Group 3 | 27.187 [26.832, 27.526] | 1.770 [1.714, 1.789] | 37.831 [37.344, 38.474] |
| Bilevel / CCITT Group 4 | 24.379 [23.534, 25.308] | 1.961 [1.912, 1.996] | 31.793 [30.879, 33.624] |
| RGB8 / lossless WebP subset | 61.228 [59.146, 62.764] | 1.363 [1.303, 1.415] | 76.363 [75.129, 76.800] |
| Native LogLuv16 / SGILog32 | 27.781 [26.985, 28.408] | 1.159 [1.140, 1.190] | — |
| Original legacy LZW | 14.352 [14.206, 14.886] | 1.264 [1.262, 1.290] | 18.703 [18.573, 19.098] |
| Original 24.3 MP LZW scan | 114.058 [111.990, 121.057] | 30.482 [29.898, 30.814] | 238.466 [235.005, 250.611] |
| Original 8.3 MP Group 4 scan | 129.178 [124.190, 135.190] | 12.224 [11.690, 13.217] | 166.041 [161.225, 174.725] |
| Original Deflate final-strip case | 10.591 [10.485, 10.787] | 0.246 [0.237, 0.249] | 15.216 [14.857, 15.355] |
| RGB8 / JPEG (random texture) | 1278.870 [1265.559, 1337.997] | 25.107 [24.536, 26.913] | 1611.366 [1603.499, 1669.477] |

## Reproduce and inspect

See [the harness instructions](../benchmarks/README.md). Raw samples and source/input/binary/dependency hashes are in [benchmark-results.json](evidence/benchmark-results.json). Generated inputs are reproducible; the benchmark does not require shipping their uncompressed pixel buffers. All numbers above are computed from the recorded measurements.

Compiler: `cc (Ubuntu 13.3.0-6ubuntu2~24.04) 13.3.0`. System package versions:

```
libdeflate0:amd64	1.19-1build1.1
libjpeg-turbo8:amd64	2.1.5-2ubuntu2
libtiff6:amd64	4.5.1+git230720-4ubuntu2.5
libwebp7:amd64	1.3.2-0.4build3
zlib1g:amd64	1:1.3.dfsg-3.1ubuntu2.1
```
