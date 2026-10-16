# Reader baseline versus libtiff 4.7.1

This report records the reader before the Deflate/LERC optimization. Its original
measurements and source hashes are preserved. See [the current before/after
comparison](reader-optimization.md) for the optimized reader.

The current verified-tiff C reader is substantially slower than libtiff 4.7.1 on this suite: 13.3× as long for full native reads and 17.7× for full RGBA reads. The strongest benefits are lower memory for large compressed crops and lazy access to large strip tables. Streaming compressed reads, LERC float reconstruction, and RGBA conversion are the largest measured bottlenecks.

Measured 2026-10-15: 36 inputs, 78 API workloads, seven timing batches per engine/workload, and three separate peak-RSS processes per engine/workload. Eight additional strip-layout workloads, six default-strip-chopping workloads, and three diagnostic CPU profiles are recorded separately. All 14 supplementary output checks also passed exactly. All 78 output checks passed: 76 matched byte-for-byte; the two textured-JPEG operations differed by at most one 8-bit channel value (the allowed bound was three). Timed and memory-run output hashes matched their preflight results.

## Main suite: fixed file layouts

The main suite disables automatic strip chopping for both engines. Default-mode partial reads are measured separately below.

| API workload group | Cases | Geometric mean relative time |
| --- | --- | --- |
| Native strips / tiles | 30 | 13.3× slower |
| Sequential scanlines | 9 | 6.5× slower |
| Full RGBA | 22 | 17.7× slower |
| Centered 128 × 128 RGBA crop | 10 | 9.5× slower |

## Selected workloads

All times are medians in milliseconds; relative time is verified / libtiff.

| Input | Operation | Verified ms | 4.7.1 ms | Relative time |
| --- | --- | --- | --- | --- |
| RGB8 deflate | native | 119.35 | 1.264 | 94.4× slower |
| RGBA8 WebP lossless | native | 36.56 | 2.915 | 12.5× slower |
| RGBA8 WebP lossy | native | 91.87 | 7.912 | 11.6× slower |
| Float32 LERC plain | native | 408.41 | 2.241 | 182.2× slower |
| Float32 LERC deflate | native | 386.93 | 3.084 | 125.5× slower |
| Float32 LERC zstd | native | 427.47 | 3.270 | 130.7× slower |
| 16 MP Gray8 single strip none | crop | 0.2170 | 15.86 | 73.1× faster |
| 16 MP Gray8 single strip lzw | crop | 157.34 | 11.07 | 14.2× slower |
| 16 MP Gray8 single strip deflate | crop | 455.60 | 3.882 | 117.3× slower |
| 16 MP Gray8 single strip deflate | firstrow | 304.64 | 0.0203 | 15000.7× slower |

## Bottlenecks and tradeoffs

Streaming threshold: with identical RGB pixels, 16-row (48 KiB decoded) Deflate strips take 11.15 ms, while 32-row (96 KiB) strips take 111.17 ms in the verified reader. libtiff takes 1.53 and 1.31 ms. Source inspection confirms native_range selects the streaming path when either decoded or compressed segment size exceeds 64 KiB. This isolates a roughly 10× slowdown at the path change; compression layout still differs, so it is not a single-instruction causal experiment.

Deflate profile: 50.0% of flat CPU samples are in native_stream::Inflater::step, 43.5% in stream::Stream::pull, and 5.0% in Input::ensure. The implementation validates the complete compressed segment before exposing requested bytes, then resumes a second decode. That policy explains why a first-row request can incur full-segment work: the 16 MP Deflate case is 304.64 ms versus 0.0203 ms. These first-row operations produce the same row but do not promise identical validation of unseen data.

LERC profile: 70.1% of samples are in lerc_numeric::pack and 20.4% in lerc_numeric::narrow. These software floating-point reconstruction/conversion routines dominate the plain float32 LERC case, which is 182.2× slower. Deflate/ZSTD-wrapped LERC remains 125.5×/130.7× slower. Those percentages are flat self samples, including inlined code and warm-up, not a call graph or proof that a proposed optimization will recover the entire gap.

RGBA profile: uncompressed RGB spends 78.3% of samples in Handle::rgba and 18.0% in native_samples::read_word. Its native read is 0.191 ms, while full RGBA is 19.06 ms; libtiff RGBA is 0.755 ms. The general per-pixel conversion loop is expensive even when decompression is absent. Full RGBA also retains an internal result in addition to the caller buffer, visible in the large-scan RSS measurements.

Partial-read tradeoff: disabling strip chopping makes the 16 MP uncompressed crop appear 73.1× faster, but ordinary libtiff strip chopping reverses that result: verified 0.195 ms versus libtiff 0.0260 ms (7.5× slower), with 1.75 versus 2.28 MiB RSS. In the same default-mode follow-up, LZW and Deflate crops retain small memory footprints (1.93 and 1.96 MiB versus 23.59 and 18.41 MiB) but take 14.2× and 121.1× as long. Lower memory usage does not imply lower latency.

Metadata: opening the 200,000-strip file takes 1.82 µs versus 1,080 µs in ordinary rmc mode. Enabling libtiff deferred metadata (rmOc) reduces its open to 5.05 µs, so the corresponding verified 1.87 µs open is 2.7× faster. Looking up the last strip offset remains 2.24 µs versus 889.7 µs with that option. Both ordinary and deferred-mode results are included; the large ordinary-open advantage should not be generalized to optimized libtiff users.

## Controlled strip-height sweep

| Rows / strip | Decoded KiB / strip | Operation | Verified ms | 4.7.1 ms | Relative time |
| --- | --- | --- | --- | --- | --- |
| 16 | 48 | native | 11.15 | 1.528 | 7.3× slower |
| 16 | 48 | crop | 1.750 | 0.1994 | 8.8× slower |
| 32 | 96 | native | 111.17 | 1.307 | 85.1× slower |
| 32 | 96 | crop | 14.62 | 0.1849 | 79.1× slower |
| 64 | 192 | native | 114.75 | 1.222 | 93.9× slower |
| 64 | 192 | crop | 15.12 | 0.1830 | 82.6× slower |
| 256 | 768 | native | 120.08 | 1.229 | 97.7× slower |
| 256 | 768 | crop | 44.55 | 0.3819 | 116.6× slower |

The sweep is excluded from the main-suite geometric means. Every layout decoded to the same native output hash.

## Default strip chopping

These six follow-up workloads use ordinary `rm` opening instead of `rmc`; mmap stays disabled. The same libraries and outputs are used. Derived consumer source, exact one-string change, commands and binary hashes are recorded. Results are excluded from the main geometric means.

| Input | Operation | Verified ms | 4.7.1 ms | Relative time | Verified RSS MiB | 4.7.1 RSS MiB |
| --- | --- | --- | --- | --- | --- | --- |
| 16 MP Gray8 single strip none | firstrow | 0.0026 | 0.0080 | 3.2× faster | 1.602 | 2.238 |
| 16 MP Gray8 single strip none | crop | 0.1952 | 0.0260 | 7.5× slower | 1.746 | 2.281 |
| 16 MP Gray8 single strip lzw | firstrow | 100.81 | 0.1954 | 515.9× slower | 1.840 | 7.527 |
| 16 MP Gray8 single strip lzw | crop | 148.89 | 10.47 | 14.2× slower | 1.926 | 23.59 |
| 16 MP Gray8 single strip deflate | firstrow | 302.60 | 0.0196 | 15443.9× slower | 1.891 | 2.410 |
| 16 MP Gray8 single strip deflate | crop | 433.16 | 3.578 | 121.1× slower | 1.965 | 18.41 |

## Peak process memory (main suite, chopping disabled)

| Input | Operation | Verified RSS MiB | 4.7.1 RSS MiB |
| --- | --- | --- | --- |
| 16 MP Gray8 single strip none | crop | 1.742 | 34.11 |
| 16 MP Gray8 single strip lzw | crop | 1.867 | 23.64 |
| 16 MP Gray8 single strip deflate | crop | 1.973 | 18.35 |
| Original 24.3 MP LZW scan | rgba | 187.43 | 98.12 |
| Original 8.3 MP fax scan | rgba | 66.23 | 34.75 |

## Method and limits

Both implementations use the same freshly compiled C consumer and the same TIFF inputs. Every timed operation opens a fresh handle, reads, and closes it; no decoded image or segment cache survives an operation. Native means all encoded strips/tiles in stored sample form, including native float/complex words and tile padding. RGBA means TIFFReadRGBAImageOriented; crops use TIFFRGBAImageBegin/Get/End with centered offsets.

Timing uses identical in-memory callbacks with mmap and strip chopping disabled (mode rmc). Input loading, caller-output allocation, process startup, output hashing, fixture encoding, and correctness comparison are outside the timer. Internal library allocations, validation, conversion, and close are inside. One warm-up/calibration operation precedes batches of at least 100 ms, or a complete operation if longer. Engine order is shuffled reproducibly; processes run serially pinned to logical CPU 0. These are warm-cache measurements on a shared AMD EPYC 9V74 virtual host, not cold-storage or isolated-hardware measurements.

Peak RSS is the median of three separate file-backed processes with mmap disabled, read from /proc/self/status VmHWM before hashing. It includes executable/runtime state, metadata setup, caller buffers, and library allocations; it is not exact library heap usage. RSS baselines differ between binaries. File-backed RSS runs are separate from the in-memory timing runs.

The adapter is rebuilt in Cargo release mode (opt-level=3) using pinned Rust 1.98.1. The freshly configured libtiff 4.7.1 reference uses its normal GCC -g -O2 build; both C consumers use -O3. Reference codecs: libjpeg-turbo 3.1.1, libwebp 1.6.0, Zstandard 1.5.7, LERC 4.0.0, and runtime zlib 1.3. libdeflate is disabled. JPEG SIMD is disabled in this pinned test reference; this is not a SIMD-enabled production JPEG comparison. Configure/build logs, dynamic linkage, and binary hashes are retained.

Adapter limits are explicitly raised: 16 GiB file size, 1 GiB output and single-allocation limits, 2 GiB cumulative allocation allowance, and 10^12 work units. These are performance measurements with adequate quotas, not measurements of default deployment limits. The production adapter and core are unchanged and their hashes match the prior reader delivery and its 1,631-obligation proof record. No new full proof run is claimed for this benchmark-only change.

Most generated images are 1024 × 1024, with 64-row strips or 128 × 128 tiles. Patterns include smooth blocks, seeded random bytes, and a synthetic document. Large-strip inputs are 4096 × 4096. Three original libtiff corpus files cover a 24.3 MP LZW scan, an 8.3 MP fax scan, and legacy LZW. The metadata fixture has 200,000 strips. This selected suite is not a representative application workload survey; geometric means weight each listed workload equally. They are not whole-application performance or coverage certification.

## Reproduce

```bash
python3 scripts/bootstrap.py
python3 scripts/upstream.py
python3 benchmarks/prepare-reader.py
bash scripts/build-compat.sh
bash benchmarks/build-reader.sh
python3 benchmarks/reader-bench.py --generate-only
python3 benchmarks/reader-bench.py --rounds 7 --milliseconds 100
python3 benchmarks/reader-layout.py
python3 benchmarks/reader-defaults.py
# Optional diagnostic profiling; never use profiled timings as benchmark results:
python3 benchmarks/profile.py --output target/reader-benchmark/profile-lerc.json target/reader-benchmark/verified target/reader-benchmark/inputs/lerc-float-plain.tif native 4000 memory
python3 benchmarks/profile.py --output target/reader-benchmark/profile-deflate.json target/reader-benchmark/verified target/reader-benchmark/inputs/rgb-deflate.tif native 4000 memory
python3 benchmarks/profile.py --output target/reader-benchmark/profile-rgba.json target/reader-benchmark/verified target/reader-benchmark/inputs/rgb-none.tif rgba 4000 memory
```

Recorded evidence is in `docs/evidence/reader-benchmark/`: raw samples, input hashes, source/binary hashes, build logs, profiles, and CSV. Fresh runs write to `target/reader-benchmark/` and do not overwrite the recorded evidence. Use `--validate-only` for correctness preflight, or `--resume` only with unchanged binaries and settings. The original source archive hash in the results identifies the pre-benchmark delivery; this delivery adds harnesses and evidence without changing production code.

## Every workload

IQR is the 25th–75th percentile across seven process batches. RSS is from three independent file-backed processes.

| Workload | Operation | Verified ms | 4.7.1 ms | Relative time | Verified IQR ms | 4.7.1 IQR ms | Verified RSS MiB | 4.7.1 RSS MiB |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| gray-none | native | 0.0673 | 0.0335 | 2.0× slower | 0.0671–0.0752 | 0.0329–0.0338 | 2.688 | 3.191 |
| gray-none | scan | 0.2053 | 0.0672 | 3.1× slower | 0.2022–0.2108 | 0.0627–0.0701 | 2.633 | 3.230 |
| gray-none | rgba | 12.39 | 0.5612 | 22.1× slower | 11.69–13.04 | 0.5441–0.6077 | 9.438 | 6.297 |
| rgb-none | native | 0.1908 | 0.0864 | 2.2× slower | 0.1874–0.1987 | 0.0808–0.0869 | 4.805 | 5.172 |
| rgb-none | rgba | 19.06 | 0.7550 | 25.2× slower | 19.04–20.18 | 0.7339–0.7682 | 9.559 | 6.496 |
| rgb-none | crop | 0.3280 | 0.1050 | 3.1× slower | 0.3143–0.3412 | 0.1039–0.1139 | 1.742 | 2.480 |
| gray-packbits | native | 1.439 | 1.257 | 1.1× slower | 1.366–1.480 | 1.207–1.268 | 2.781 | 3.270 |
| gray-packbits | scan | 1.291 | 1.211 | 1.1× slower | 1.268–1.313 | 1.190–1.288 | 2.730 | 3.242 |
| gray-packbits | rgba | 12.15 | 1.674 | 7.3× slower | 12.05–12.38 | 1.662–1.682 | 9.605 | 6.324 |
| gray-lzw | native | 6.652 | 0.9674 | 6.9× slower | 6.323–7.030 | 0.9211–0.9876 | 2.797 | 3.383 |
| gray-lzw | scan | 6.579 | 1.011 | 6.5× slower | 6.465–6.697 | 1.005–1.052 | 2.742 | 3.414 |
| gray-lzw | rgba | 18.30 | 1.531 | 12.0× slower | 18.08–18.65 | 1.412–1.621 | 9.559 | 6.414 |
| rgb-lzw | native | 34.29 | 2.889 | 11.9× slower | 33.36–36.00 | 2.846–2.952 | 5.086 | 5.410 |
| rgb-lzw | rgba | 53.04 | 3.647 | 14.5× slower | 52.52–53.78 | 3.596–3.703 | 9.699 | 6.500 |
| rgb-lzw | crop | 4.494 | 0.4267 | 10.5× slower | 4.377–4.871 | 0.4122–0.4745 | 1.934 | 2.500 |
| gray-deflate | native | 3.542 | 0.5101 | 6.9× slower | 3.449–3.585 | 0.4644–0.5129 | 2.926 | 3.316 |
| gray-deflate | scan | 3.490 | 0.5733 | 6.1× slower | 3.436–3.707 | 0.5276–0.5967 | 2.855 | 3.344 |
| gray-deflate | rgba | 15.89 | 1.020 | 15.6× slower | 15.36–17.56 | 0.9658–1.095 | 9.703 | 6.375 |
| rgb-deflate | native | 119.35 | 1.264 | 94.4× slower | 117.52–123.45 | 1.244–1.343 | 5.098 | 5.336 |
| rgb-deflate | rgba | 138.41 | 1.877 | 73.7× slower | 137.27–145.37 | 1.842–1.987 | 9.684 | 6.355 |
| rgb-deflate | crop | 14.84 | 0.1803 | 82.3× slower | 14.62–15.01 | 0.1781–0.1822 | 1.957 | 2.469 |
| gray-zstd | native | 0.2200 | 0.1168 | 1.9× slower | 0.2161–0.2272 | 0.1037–0.1170 | 3.004 | 3.492 |
| gray-zstd | scan | 0.3247 | 0.1289 | 2.5× slower | 0.3229–0.3521 | 0.1194–0.1345 | 2.992 | 3.508 |
| gray-zstd | rgba | 12.96 | 0.6276 | 20.7× slower | 11.60–14.25 | 0.5899–0.6642 | 9.812 | 6.555 |
| rgb-zstd | native | 1.670 | 0.2405 | 6.9× slower | 1.578–1.710 | 0.2355–0.2620 | 5.090 | 5.543 |
| rgb-zstd | rgba | 19.20 | 0.8199 | 23.4× slower | 18.97–19.28 | 0.7826–0.8360 | 10.05 | 6.797 |
| rgb-zstd | crop | 0.3604 | 0.0571 | 6.3× slower | 0.3420–0.3654 | 0.0557–0.0602 | 2.254 | 2.789 |
| noise-lzw | native | 45.79 | 2.327 | 19.7× slower | 45.57–47.14 | 2.262–3.070 | 2.871 | 3.422 |
| noise-deflate | native | 43.47 | 0.4246 | 102.4× slower | 42.33–43.92 | 0.3999–0.4374 | 2.883 | 3.359 |
| noise-zstd | native | 0.5087 | 0.1059 | 4.8× slower | 0.4971–0.5175 | 0.1033–0.1117 | 2.801 | 3.543 |
| jpeg-smooth | native | 31.90 | 3.527 | 9.0× slower | 31.64–33.76 | 3.476–3.655 | 5.453 | 5.719 |
| jpeg-smooth | rgba | 52.51 | 4.628 | 11.3× slower | 51.40–56.16 | 4.157–4.773 | 10.34 | 6.703 |
| jpeg-texture | native | 136.63 | 27.41 | 5.0× slower | 133.42–141.81 | 26.96–27.97 | 5.477 | 5.684 |
| jpeg-texture | rgba | 144.43 | 25.97 | 5.6× slower | 142.74–152.12 | 25.82–28.43 | 10.44 | 6.855 |
| rgb-tiled-lzw | native | 19.94 | 2.970 | 6.7× slower | 18.37–24.53 | 2.879–3.410 | 4.750 | 5.379 |
| rgb-tiled-lzw | rgba | 42.50 | 4.100 | 10.4× slower | 39.82–44.22 | 3.608–5.018 | 9.605 | 6.418 |
| rgb-tiled-lzw | crop | 1.544 | 0.2106 | 7.3× slower | 1.495–1.573 | 0.2084–0.2160 | 1.840 | 2.469 |
| gray16-deflate-p2 | native | 91.47 | 4.661 | 19.6× slower | 87.97–93.46 | 4.529–5.029 | 4.051 | 4.398 |
| gray16-deflate-p2 | scan | 89.45 | 4.792 | 18.7× slower | 88.51–91.99 | 4.686–4.987 | 3.891 | 4.430 |
| float32-deflate-p3 | native | 212.68 | 9.275 | 22.9× slower | 209.14–216.63 | 9.161–9.513 | 6.543 | 6.324 |
| float32-deflate-p3 | scan | 215.97 | 9.191 | 23.5× slower | 208.06–222.60 | 9.101–9.855 | 5.902 | 6.359 |
| float16-deflate-p3 | native | 104.29 | 6.976 | 15.0× slower | 101.22–105.18 | 6.896–7.081 | 4.137 | 4.309 |
| float16-deflate-p3 | scan | 92.75 | 6.655 | 13.9× slower | 92.17–98.47 | 6.604–7.001 | 3.840 | 4.363 |
| complex64-lzw | native | 146.02 | 11.18 | 13.1× slower | 125.52–148.54 | 11.02–11.59 | 10.66 | 10.38 |
| complex64-lzw | scan | 113.23 | 10.63 | 10.7× slower | 111.52–114.39 | 10.47–11.03 | 9.852 | 10.40 |
| fax-rle | native | 9.347 | 0.5602 | 16.7× slower | 8.879–9.722 | 0.5113–0.6060 | 2.375 | 2.480 |
| fax-rle | rgba | 19.26 | 0.8198 | 23.5× slower | 18.73–20.71 | 0.7883–0.8660 | 10.20 | 6.375 |
| fax-g3 | native | 16.81 | 0.4874 | 34.5× slower | 16.39–17.62 | 0.4858–0.4994 | 2.500 | 2.488 |
| fax-g3 | rgba | 27.21 | 0.7631 | 35.7× slower | 26.79–27.41 | 0.7519–0.7704 | 10.23 | 6.371 |
| fax-g4 | native | 9.580 | 0.5290 | 18.1× slower | 9.376–9.736 | 0.5260–0.5489 | 2.461 | 2.484 |
| fax-g4 | rgba | 20.37 | 0.8625 | 23.6× slower | 19.51–20.98 | 0.8323–0.8821 | 10.21 | 6.371 |
| webp-lossless | native | 36.56 | 2.915 | 12.5× slower | 36.02–37.29 | 2.818–3.096 | 6.949 | 6.898 |
| webp-lossless | rgba | 63.34 | 4.491 | 14.1× slower | 62.03–65.21 | 4.394–4.531 | 10.90 | 7.016 |
| webp-lossy | native | 91.87 | 7.912 | 11.6× slower | 89.13–96.69 | 7.767–8.279 | 7.332 | 7.035 |
| webp-lossy | rgba | 132.18 | 9.136 | 14.5× slower | 118.53–144.33 | 9.032–9.715 | 11.25 | 7.215 |
| lerc-float-plain | native | 408.41 | 2.241 | 182.2× slower | 397.26–428.07 | 2.157–2.654 | 5.828 | 6.770 |
| lerc-float-deflate | native | 386.93 | 3.084 | 125.5× slower | 380.09–408.33 | 3.036–3.205 | 5.984 | 6.902 |
| lerc-float-zstd | native | 427.47 | 3.270 | 130.7× slower | 415.85–434.00 | 3.240–3.301 | 6.008 | 7.102 |
| cmyk-deflate | rgba | 186.32 | 4.488 | 41.5× slower | 176.75–197.40 | 4.322–4.669 | 9.699 | 6.496 |
| cmyk-deflate | crop | 19.06 | 0.2618 | 72.8× slower | 18.75–19.47 | 0.2555–0.2658 | 1.930 | 2.434 |
| lab-deflate | rgba | 212.45 | 20.39 | 10.4× slower | 205.27–215.03 | 20.25–20.70 | 9.730 | 6.402 |
| lab-deflate | crop | 16.20 | 0.5402 | 30.0× slower | 16.12–16.57 | 0.5366–0.5557 | 1.996 | 2.434 |
| large-strip-none | firstrow | 0.0029 | 1.441 | 505.6× faster | 0.0028–0.0031 | 1.359–1.565 | 1.602 | 18.12 |
| large-strip-none | crop | 0.2170 | 15.86 | 73.1× faster | 0.2123–0.2253 | 15.49–16.07 | 1.742 | 34.11 |
| large-strip-lzw | firstrow | 101.00 | 0.1872 | 539.6× slower | 98.68–110.59 | 0.1844–0.1944 | 1.840 | 7.531 |
| large-strip-lzw | crop | 157.34 | 11.07 | 14.2× slower | 154.18–159.49 | 11.03–12.38 | 1.867 | 23.64 |
| large-strip-deflate | firstrow | 304.64 | 0.0203 | 15000.7× slower | 294.86–307.78 | 0.0194–0.0209 | 1.898 | 2.418 |
| large-strip-deflate | crop | 455.60 | 3.882 | 117.3× slower | 447.73–468.22 | 3.670–3.982 | 1.973 | 18.35 |
| many-strips | open | 0.0018 | 1.080 | 592.2× faster | 0.0017–0.0018 | 1.069–1.100 | 1.598 | 5.895 |
| many-strips | lookup | 0.0023 | 1.166 | 504.5× faster | 0.0023–0.0024 | 1.144–1.197 | 1.621 | 5.875 |
| many-strips | open-lazy | 0.0019 | 0.0050 | 2.7× faster | 0.0019–0.0019 | 0.0049–0.0051 | 1.609 | 2.164 |
| many-strips | lookup-lazy | 0.0022 | 0.8897 | 396.5× faster | 0.0022–0.0027 | 0.8648–0.9603 | 1.590 | 5.125 |
| original-lzw-single-strip.tiff | native | 17.00 | 1.338 | 12.7× slower | 16.56–18.88 | 1.168–1.453 | 7.449 | 5.352 |
| original-lzw-single-strip.tiff | rgba | 291.22 | 8.954 | 32.5× slower | 282.22–321.84 | 8.094–10.28 | 187.43 | 98.12 |
| original-testfax4.tiff | native | 48.18 | 1.936 | 24.9× slower | 47.10–48.89 | 1.878–2.058 | 18.93 | 3.367 |
| original-testfax4.tiff | rgba | 135.95 | 4.292 | 31.7× slower | 132.72–160.36 | 4.147–4.967 | 66.23 | 34.75 |
| original-quad-lzw-compat.tiff | native | 5.136 | 1.273 | 4.0× slower | 5.039–5.420 | 1.234–1.339 | 2.258 | 2.961 |
| original-quad-lzw-compat.tiff | rgba | 9.380 | 1.499 | 6.3× slower | 9.243–9.593 | 1.461–1.552 | 3.059 | 3.129 |
