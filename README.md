# verified-tiff

**WARNING: This is EXPERIMENTAL software written by ChatGPT Astra, until it's undergone
adversarial review you should assume there is at least one critical exploit in here.
Use at your own risk.**

This is an attempt at a formally verified replacement library for
[libtiff](https://en.wikipedia.org/wiki/LibTIFF) written in
[Rust](https://rust-lang.org/) and [Versus](https://www.cs.cmu.edu/~csd-phd-blog/2023/rust-verification-with-verus/).
It is currently about 5x slower than the C libtiff, but can presumably be brought
up to parity with further optimization of either the library itself or Versus.
I have minimal experience with Rust and zero experience with Versus, so I'm left
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

The library is about 5x slower overall, but it's uneven and some parts of it are
slower than others, you can consult the benchmark numbers below to get a sense
of whether you'd like to try this or not:

## Full Benchmark

(AI text follows)

Measurements used nine randomized serial batches of at least 200 ms per implementation/workload, pinned to one AMD EPYC 9V74 CPU, with fresh handles and warm memory input. **114 outputs matched exactly; two JPEG outputs differed by at most one channel value.** The reference preserves the previous suite’s JPEG SIMD-disabled configuration; WebP SIMD is enabled.

Below, times are **median milliseconds per operation**. Ratio means **ours ÷ libtiff**, so **above 1 favors libtiff**. `native` reads all samples, `scan` reads every scanline, `rgba` converts the full image, and `crop` reads a centered 128×128 RGBA region. This matrix does not time OJPEG or SGILog/LogLuv.

**Main suite — all 102 results**

| Fixture | Task | Ours ms | libtiff ms | Ratio |
|---|---|---:|---:|---:|
| gray-none | native | 0.0689 | 0.0319 | 2.16× |
| gray-none | scan | 0.1992 | 0.0624 | 3.19× |
| gray-none | rgba | 1.560 | 0.5132 | 3.04× |
| rgb-none | native | 0.2014 | 0.0886 | 2.27× |
| rgb-none | rgba | 2.117 | 0.7363 | 2.87× |
| rgb-none | crop | 0.0568 | 0.0959 | 0.59× |
| gray-packbits | native | 1.312 | 1.187 | 1.11× |
| gray-packbits | scan | 1.275 | 1.224 | 1.04× |
| gray-packbits | rgba | 2.573 | 1.672 | 1.54× |
| gray-lzw | native | 5.763 | 0.9286 | 6.21× |
| gray-lzw | scan | 5.507 | 0.9500 | 5.80× |
| gray-lzw | rgba | 6.878 | 1.410 | 4.88× |
| rgb-lzw | native | 12.402 | 3.214 | 3.86× |
| rgb-lzw | rgba | 13.441 | 3.512 | 3.83× |
| rgb-lzw | crop | 3.271 | 0.4438 | 7.37× |
| gray-deflate | native | 3.595 | 0.4524 | 7.95× |
| gray-deflate | scan | 3.734 | 0.5124 | 7.29× |
| gray-deflate | rgba | 5.334 | 0.9220 | 5.78× |
| rgb-deflate | native | 7.195 | 1.230 | 5.85× |
| rgb-deflate | rgba | 7.890 | 1.724 | 4.58× |
| rgb-deflate | crop | 1.804 | 0.1756 | 10.27× |
| gray-zstd | native | 0.2134 | 0.1064 | 2.01× |
| gray-zstd | scan | 0.3049 | 0.1163 | 2.62× |
| gray-zstd | rgba | 1.597 | 0.5678 | 2.81× |
| rgb-zstd | native | 1.461 | 0.2275 | 6.42× |
| rgb-zstd | rgba | 1.978 | 0.7754 | 2.55× |
| rgb-zstd | crop | 0.0610 | 0.0543 | 1.12× |
| noise-lzw | native | 15.987 | 2.208 | 7.24× |
| noise-deflate | native | 4.968 | 0.3704 | 13.41× |
| noise-zstd | native | 0.4459 | 0.0957 | 4.66× |
| jpeg-smooth | native | 28.852 | 3.079 | 9.37× |
| jpeg-smooth | rgba | 31.466 | 3.818 | 8.24× |
| jpeg-texture | native | 134.475 | 25.620 | 5.25× |
| jpeg-texture | rgba | 129.143 | 26.288 | 4.91× |
| rgb-tiled-lzw | native | 16.072 | 2.718 | 5.91× |
| rgb-tiled-lzw | rgba | 17.316 | 3.264 | 5.31× |
| rgb-tiled-lzw | crop | 0.9499 | 0.1843 | 5.15× |
| gray16-deflate-p2 | native | 12.268 | 4.347 | 2.82× |
| gray16-deflate-p2 | scan | 15.206 | 4.437 | 3.43× |
| float32-deflate-p3 | native | 22.221 | 8.330 | 2.67× |
| float32-deflate-p3 | scan | 29.475 | 8.641 | 3.41× |
| float16-deflate-p3 | native | 11.369 | 6.859 | 1.66× |
| float16-deflate-p3 | scan | 14.838 | 6.873 | 2.16× |
| complex64-lzw | native | 37.944 | 10.423 | 3.64× |
| complex64-lzw | scan | 78.943 | 10.584 | 7.46× |
| fax-rle | native | 3.039 | 0.4592 | 6.62× |
| fax-rle | rgba | 3.771 | 0.7696 | 4.90× |
| fax-g3 | native | 7.134 | 0.4944 | 14.43× |
| fax-g3 | rgba | 7.555 | 0.7655 | 9.87× |
| fax-g4 | native | 7.072 | 0.5059 | 13.98× |
| fax-g4 | rgba | 7.616 | 0.7698 | 9.89× |
| webp-lossless | native | 14.331 | 2.704 | 5.30× |
| webp-lossless | rgba | 17.990 | 3.903 | 4.61× |
| webp-lossless | crop | 1.706 | 0.4507 | 3.79× |
| webp-lossy | native | 41.657 | 6.926 | 6.01× |
| webp-lossy | rgba | 47.461 | 8.446 | 5.62× |
| webp-lossy | crop | 5.358 | 1.179 | 4.54× |
| lerc-float-plain | native | 9.637 | 1.975 | 4.88× |
| lerc-float-deflate | native | 16.932 | 3.094 | 5.47× |
| lerc-float-zstd | native | 12.507 | 2.962 | 4.22× |
| cmyk-deflate | rgba | 28.104 | 4.328 | 6.49× |
| cmyk-deflate | crop | 2.414 | 0.2580 | 9.36× |
| lab-deflate | rgba | 43.749 | 20.089 | 2.18× |
| lab-deflate | crop | 2.311 | 0.5167 | 4.47× |
| large-strip-none | firstrow | 0.002851 | 1.099 | 0.0026× |
| large-strip-none | crop | 0.0567 | 14.541 | 0.0039× |
| large-strip-lzw | firstrow | 66.319 | 0.1727 | 383.98× |
| large-strip-lzw | crop | 101.631 | 10.878 | 9.34× |
| large-strip-deflate | firstrow | 37.558 | 0.0209 | 1800.97× |
| large-strip-deflate | crop | 49.631 | 3.690 | 13.45× |
| many-strips | open | 0.001924 | 1.106 | 0.0017× |
| many-strips | lookup | 0.002354 | 1.037 | 0.0023× |
| many-strips | open-lazy | 0.001896 | 0.004853 | 0.39× |
| many-strips | lookup-lazy | 0.002340 | 0.8212 | 0.0029× |
| original-lzw-single-strip.tiff | native | 6.840 | 1.200 | 5.70× |
| original-lzw-single-strip.tiff | rgba | 62.656 | 8.002 | 7.83× |
| original-testfax4.tiff | native | 40.897 | 1.810 | 22.59× |
| original-testfax4.tiff | rgba | 47.076 | 3.974 | 11.84× |
| original-quad-lzw-compat.tiff | native | 4.652 | 1.316 | 3.53× |
| original-quad-lzw-compat.tiff | rgba | 4.860 | 1.368 | 3.55× |
| lerc-float64-tiled | native | 11.406 | 2.298 | 4.96× |
| lerc-uint16-tiled | native | 12.780 | 2.102 | 6.08× |
| lerc-int16-tiled | native | 12.317 | 2.154 | 5.72× |
| lerc-float32-strips | native | 8.765 | 2.078 | 4.22× |
| webp-lossy-opaque | native | 38.594 | 3.766 | 10.25× |
| webp-lossy-opaque | rgba | 36.845 | 3.943 | 9.35× |
| webp-lossy-opaque | crop | 4.384 | 0.4243 | 10.33× |
| webp-lossy-texture | native | 264.781 | 40.005 | 6.62× |
| webp-lossy-texture | rgba | 267.357 | 38.066 | 7.02× |
| webp-lossy-texture | crop | 16.371 | 2.526 | 6.48× |
| webp-lossy-odd | native | 2.870 | 0.5380 | 5.33× |
| webp-lossy-odd | rgba | 3.463 | 0.6176 | 5.61× |
| webp-lossy-odd | crop | 3.100 | 0.6423 | 4.83× |
| webp-lossless-opaque | native | 13.633 | 2.462 | 5.54× |
| webp-lossless-opaque | rgba | 15.405 | 3.099 | 4.97× |
| webp-lossless-opaque | crop | 1.680 | 0.4021 | 4.18× |
| webp-lossless-texture | native | 597.569 | 12.301 | 48.58× |
| webp-lossless-texture | rgba | 613.423 | 14.526 | 42.23× |
| webp-lossless-texture | crop | 37.319 | 0.8472 | 44.05× |
| webp-lossless-odd | native | 0.9036 | 0.1860 | 4.86× |
| webp-lossless-odd | rgba | 1.202 | 0.3848 | 3.12× |
| webp-lossless-odd | crop | 0.9599 | 0.3628 | 2.65× |

**RGB Deflate strip-layout diagnostics**

| Rows per strip | Task | Ours ms | libtiff ms | Ratio |
|---|---|---:|---:|---:|
| 16 | native | 10.390 | 1.457 | 7.13× |
| 16 | crop | 1.367 | 0.2020 | 6.77× |
| 32 | native | 6.569 | 1.274 | 5.16× |
| 32 | crop | 1.694 | 0.1834 | 9.24× |
| 64 | native | 7.123 | 1.208 | 5.90× |
| 64 | crop | 1.783 | 0.1688 | 10.56× |
| 256 | native | 6.769 | 1.128 | 6.00× |
| 256 | crop | 5.530 | 0.3835 | 14.42× |

**Large-strip reads with default strip-chopping behavior**

| Compression | Task | Ours ms | libtiff ms | Ratio |
|---|---|---:|---:|---:|
| None | firstrow | 0.002665 | 0.9499 | 0.0028× |
| None | crop | 0.0522 | 13.196 | 0.0040× |
| LZW | firstrow | 64.824 | 0.1643 | 394.66× |
| LZW | crop | 100.618 | 9.963 | 10.10× |
| Deflate | firstrow | 36.310 | 0.0199 | 1820.38× |
| Deflate | crop | 46.854 | 3.394 | 13.80× |

(End AI Text)

Astra [wrote a README](YAP.md) of weird mumbly AI text that you probably don't want
to read, but I have included it for the sake of completeness.
