# verified-tiff

**WARNING: This is EXPERIMENTAL software written by ChatGPT Astra, until it's undergone
adversarial review you should assume there is at least one critical exploit in here.
Use at your own risk.**

This is an attempt at a formally verified replacement library for
[libtiff](https://en.wikipedia.org/wiki/LibTIFF) written in
[Rust](https://rust-lang.org/) and [Verus](https://www.cs.cmu.edu/~csd-phd-blog/2023/rust-verification-with-verus/).
It is currently about 2x slower than the C libtiff, but can presumably be brought
up to parity with further optimization of either the library itself or Verus.
I have minimal experience with Rust and zero experience with Verus, so I'm left
in the strange position of having this artifact but not really knowing if it's any
good or not. I guess I'll find out.

Right now it implements most of the reader/decoder side of libtiff, but doesn't
implement writing tiff files at all. I do plan to add this in a future update, but
since my motivation was to mitigate the further discovery of zero days in libtiff
and these are of most consequence in e.g. zero click phone exploits it seemed more
important to complete a somewhat performant version of the reader API than spend
tokens on writing right away.

I have limited tokens and this project requires quite a few of them, so I am
interested in patches from others. I am particularly interested in performance
improvements, hardening, bug fixes and test coverage (especially in the C shim,
which is currently the weak point of the whole library), and implementing more of
the libtiff API. My goal is to eventually reach full parity with the C libtiff.

The library is about 2x slower overall, but it's uneven and some parts of it are
slower than others, you can consult the benchmark numbers below to get a sense
of whether you'd like to try this or not:

## Full Benchmark

(AI text follows)

Measurements used seven randomized serial rounds of at least 100 ms per implementation/workload, pinned to one CPU of a 4 vCPU KVM guest on an Intel Xeon at 2.80 GHz (Cascade Lake), with fresh handles and warm memory input. Across the 89 decode workloads the reader takes 2.32× as long as libtiff 4.7.1 (geometric mean). **94 outputs matched libtiff exactly; the two textured-JPEG outputs differed by at most one channel value.** The reference keeps JPEG SIMD disabled; WebP SIMD is enabled.

Below, times are **median milliseconds per operation**. Ratio means **ours ÷ libtiff**, so **above 1 favors libtiff**. `native` reads all samples, `scan` reads every scanline, `rgba` converts the full image, `crop` reads a centered 128×128 RGBA region, and `firstrow` reads one row of a 16 MP strip. This matrix does not time OJPEG or SGILog/LogLuv.

**Main suite — all 96 results**

| Fixture | Task | Ours ms | libtiff ms | Ratio |
|---|---|---:|---:|---:|
| gray-none | native | 0.1336 | 0.07857 | 1.70× |
| gray-none | scan | 0.3598 | 0.1186 | 3.03× |
| gray-none | rgba | 3.331 | 1.058 | 3.15× |
| rgb-none | native | 0.4444 | 0.2870 | 1.55× |
| rgb-none | rgba | 3.473 | 1.599 | 2.17× |
| rgb-none | crop | 0.08109 | 0.2128 | 0.38× |
| gray-packbits | native | 2.880 | 3.396 | 0.85× |
| gray-packbits | scan | 2.889 | 3.017 | 0.96× |
| gray-packbits | rgba | 5.066 | 4.272 | 1.19× |
| gray-lzw | native | 2.981 | 1.974 | 1.51× |
| gray-lzw | scan | 2.873 | 2.611 | 1.10× |
| gray-lzw | rgba | 5.508 | 2.811 | 1.96× |
| rgb-lzw | native | 9.977 | 5.739 | 1.74× |
| rgb-lzw | rgba | 12.976 | 6.980 | 1.86× |
| rgb-lzw | crop | 1.132 | 0.8673 | 1.31× |
| gray-deflate | native | 1.535 | 0.6572 | 2.34× |
| gray-deflate | scan | 1.522 | 0.7075 | 2.15× |
| gray-deflate | rgba | 4.118 | 1.421 | 2.90× |
| rgb-deflate | native | 4.035 | 1.974 | 2.04× |
| rgb-deflate | rgba | 6.921 | 2.959 | 2.34× |
| rgb-deflate | crop | 1.259 | 0.2690 | 4.68× |
| gray-zstd | native | 0.3633 | 0.1747 | 2.08× |
| gray-zstd | scan | 0.5940 | 0.2112 | 2.81× |
| gray-zstd | rgba | 3.548 | 1.118 | 3.17× |
| rgb-zstd | native | 3.056 | 0.4075 | 7.50× |
| rgb-zstd | rgba | 3.405 | 1.695 | 2.01× |
| rgb-zstd | crop | 0.09261 | 0.1142 | 0.81× |
| noise-lzw | native | 12.087 | 5.494 | 2.20× |
| noise-deflate | native | 2.250 | 0.7089 | 3.17× |
| noise-zstd | native | 1.139 | 0.2005 | 5.68× |
| jpeg-smooth | native | 21.214 | 9.001 | 2.36× |
| jpeg-smooth | rgba | 25.309 | 6.629 | 3.82× |
| jpeg-texture | native | 77.873 | 34.888 | 2.23× |
| jpeg-texture | rgba | 71.360 | 38.579 | 1.85× |
| rgb-tiled-lzw | native | 5.568 | 9.310 | 0.60× |
| rgb-tiled-lzw | rgba | 12.548 | 8.366 | 1.50× |
| rgb-tiled-lzw | crop | 0.4611 | 0.4564 | 1.01× |
| gray16-deflate-p2 | native | 8.109 | 4.181 | 1.94× |
| gray16-deflate-p2 | scan | 11.106 | 4.190 | 2.65× |
| float32-deflate-p3 | native | 25.042 | 9.426 | 2.66× |
| float32-deflate-p3 | scan | 28.741 | 10.700 | 2.69× |
| float16-deflate-p3 | native | 12.504 | 8.266 | 1.51× |
| float16-deflate-p3 | scan | 19.535 | 8.582 | 2.28× |
| complex64-lzw | native | 35.214 | 22.078 | 1.59× |
| complex64-lzw | scan | 31.910 | 22.477 | 1.42× |
| fax-rle | native | 2.363 | 0.9248 | 2.55× |
| fax-rle | rgba | 4.131 | 1.474 | 2.80× |
| fax-g3 | native | 5.512 | 1.019 | 5.41× |
| fax-g3 | rgba | 5.996 | 1.354 | 4.43× |
| fax-g4 | native | 3.150 | 1.162 | 2.71× |
| fax-g4 | rgba | 5.458 | 1.671 | 3.27× |
| webp-lossless | native | 17.172 | 3.972 | 4.32× |
| webp-lossless | rgba | 24.207 | 7.023 | 3.45× |
| webp-lossy | native | 55.620 | 17.350 | 3.21× |
| webp-lossy | rgba | 64.349 | 19.296 | 3.33× |
| lerc-float-plain | native | 18.996 | 5.774 | 3.29× |
| lerc-float-deflate | native | 25.405 | 8.183 | 3.10× |
| lerc-float-zstd | native | 26.318 | 8.097 | 3.25× |
| cmyk-deflate | rgba | 13.528 | 6.457 | 2.10× |
| cmyk-deflate | crop | 1.424 | 0.3272 | 4.35× |
| lab-deflate | rgba | 59.777 | 45.787 | 1.31× |
| lab-deflate | crop | 2.220 | 1.048 | 2.12× |
| large-strip-none | firstrow | 0.004255 | 6.256 | 0.00068× |
| large-strip-none | crop | 0.09409 | 39.491 | 0.0024× |
| large-strip-lzw | firstrow | 70.010 | 1.124 | 62.28× |
| large-strip-lzw | crop | 108.080 | 24.093 | 4.49× |
| large-strip-deflate | firstrow | 26.204 | 0.03176 | 825.01× |
| large-strip-deflate | crop | 26.419 | 9.406 | 2.81× |
| many-strips | open | 0.004087 | 3.266 | 0.0013× |
| many-strips | lookup | 0.003965 | 3.792 | 0.001× |
| many-strips | open-lazy | 0.003425 | 0.008338 | 0.41× |
| many-strips | lookup-lazy | 0.004075 | 2.114 | 0.0019× |
| original-lzw-single-strip.tiff | native | 8.938 | 1.722 | 5.19× |
| original-lzw-single-strip.tiff | rgba | 117.764 | 16.784 | 7.02× |
| original-testfax4.tiff | native | 27.184 | 3.062 | 8.88× |
| original-testfax4.tiff | rgba | 40.468 | 8.610 | 4.70× |
| original-quad-lzw-compat.tiff | native | 2.288 | 1.939 | 1.18× |
| original-quad-lzw-compat.tiff | rgba | 2.571 | 2.209 | 1.16× |
| webp-lossy-opaque | native | 46.659 | 6.222 | 7.50× |
| webp-lossy-opaque | rgba | 59.103 | 7.776 | 7.60× |
| webp-lossy-opaque | crop | 7.057 | 0.7165 | 9.85× |
| webp-lossy-texture | native | 169.193 | 68.136 | 2.48× |
| webp-lossy-texture | rgba | 177.401 | 65.556 | 2.71× |
| webp-lossy-texture | crop | 9.494 | 4.217 | 2.25× |
| webp-lossy-odd | native | 4.048 | 1.210 | 3.35× |
| webp-lossy-odd | rgba | 6.072 | 1.378 | 4.41× |
| webp-lossy-odd | crop | 5.428 | 1.516 | 3.58× |
| webp-lossless-opaque | native | 16.461 | 3.496 | 4.71× |
| webp-lossless-opaque | rgba | 21.200 | 4.745 | 4.47× |
| webp-lossless-opaque | crop | 2.017 | 0.6412 | 3.15× |
| webp-lossless-texture | native | 44.469 | 21.831 | 2.04× |
| webp-lossless-texture | rgba | 46.309 | 25.607 | 1.81× |
| webp-lossless-texture | crop | 2.626 | 1.438 | 1.83× |
| webp-lossless-odd | native | 1.229 | 0.2581 | 4.76× |
| webp-lossless-odd | rgba | 1.252 | 0.6719 | 1.86× |
| webp-lossless-odd | crop | 1.148 | 0.6983 | 1.64× |

(End AI Text)

Astra [wrote a README](YAP.md) of weird mumbly AI text that you probably don't want
to read, but I have included it for the sake of completeness.
