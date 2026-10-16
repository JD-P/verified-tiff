# Throughput fixes against libtiff 4.7.1

Measured 2026-10-15T01:16:32.493304+00:00 on AMD EPYC 9V74 80-Core Processor, logical CPU 0.
Both v19 and this version were freshly built with the pinned toolchain on this
host. The native reference is libtiff 4.7.1, system zlib 1.3, and scalar
libjpeg-turbo 3.1.1 (SIMD disabled). This does not represent every distribution's
configuration. The system RGBA reference is separately identified in raw metadata.

Across 26 native workloads the unchanged API is **2.11× faster** by geometric
mean. Its same-run gap to pinned libtiff falls from **10.03× to 4.75×**.
Across 24 C API RGBA workloads it is **1.62× faster**. Each case has equal
weight; these are not application workload-weighted speedups. The combined
Deflate API is reported separately and excluded from these aggregate gains.

## Changes

- LZW reads bounded 24-bit windows for both modern and legacy bit orders.
  Deflate fields and canonical Huffman loops also consume a buffered word;
  exact short-tail behavior is retained. These are not Huffman prefix tables.
- JPEG's exact integer IDCT shares symmetric terms (352 rather than 1,024
  multiplications per full block) and handles DC-only lines directly, keeping
  the original rounding/clipping. Eight-pixel runs avoid repeated geometry;
  amplitude reads consume buffered chunks and Huffman bits use shifts.
- Bilevel scatter computes the visible span once per packed byte, including
  clipped tiles, then expands bits by shifts. This removes repeated geometry
  and the measured large scanned-document regression.
- Uncompressed padded tile rows use bulk copies. Straight non-inverted alpha
  skips identity normalization. Extended interleaved output uses native copies
  or direct 16-bit sample pairs instead of per-byte general layout conversion.
- Deflate reduces Adler accumulators in bounded 256-byte batches and writes
  contiguous strip slices. The opt-in combined API validates and writes each
  strip once; tiles retain the old fallback. The ordinary API still validates
  compressed streams before returning an image descriptor.

A speculative 16-bit JPEG lookahead implementation was measured and removed:
it made textured JPEG slower. The final source uses the faster shift-based
canonical decoder. A later sweep caught out-of-line entropy/amplitude calls;
explicit inlining removes that regression. Its earlier partial sweep and CPU
samples are retained as diagnostic evidence, not final performance results.
No image model, checksum/padding policy, conservative quota,
trust boundary or exact before/after pixel result was relaxed.

## Native inspect plus decode

7 shuffled batches per engine/case, each at least 80.0 ms;
medians below. Proof/test jobs finished before timing. Disk I/O, startup, caller
buffer allocation, hashing and fixture generation are outside timing. Parsing,
validation, decoding and output conversion are inside. This shared virtual host
is not isolated benchmark hardware. Samples, quartiles and source/input/binary
hashes are in `evidence/throughput-results.json`.

| Case | Before ms | After ms | Speedup | 4.7.1 ms | After / 4.7.1 |
| --- | ---: | ---: | ---: | ---: | ---: |
| gray-none | 0.0327 | 0.0323 | 1.01× | 0.0339 | 0.95× |
| gray-packbits | 4.8114 | 4.1717 | 1.15× | 1.1987 | 3.48× |
| gray-lzw | 14.2606 | 6.9373 | 2.06× | 0.9275 | 7.48× |
| gray-deflate | 18.5252 | 6.7794 | 2.73× | 0.4369 | 15.52× |
| rgb-none | 0.0923 | 0.0947 | 0.98× | 0.0819 | 1.16× |
| rgb-lzw | 46.8229 | 22.5180 | 2.08× | 2.8080 | 8.02× |
| rgb-deflate | 52.8703 | 19.6852 | 2.69× | 1.1962 | 16.46× |
| rgb-jpeg | 78.5473 | 28.1680 | 2.79× | 3.1788 | 8.86× |
| rgb-jpeg-textured | 311.5608 | 127.5792 | 2.44× | 26.3550 | 4.84× |
| rgb-tile-lzw | 61.1795 | 32.4594 | 1.88× | 2.8262 | 11.49× |
| gray16-deflate | 85.4815 | 61.4428 | 1.39× | 8.4421 | 7.28× |
| float64-lzw-p2 | 93.5870 | 77.1911 | 1.21× | 12.0285 | 6.42× |
| noise-lzw | 56.5517 | 20.3488 | 2.78× | 2.4129 | 8.43× |
| noise-deflate | 18.1563 | 5.8412 | 3.11× | 0.3970 | 14.71× |
| fax3 | 14.2660 | 14.5228 | 0.98× | 1.8251 | 7.96× |
| fax4 | 6.9073 | 6.7762 | 1.02× | 1.8084 | 3.75× |
| webp | 19.9773 | 21.1114 | 0.95× | 1.2300 | 17.16× |
| logluv | 7.8936 | 7.7418 | 1.02× | 1.1190 | 6.92× |
| original-quad-lzw-compat.tiff | 10.0585 | 5.8668 | 1.71× | 1.2865 | 4.56× |
| original-lzw-single-strip.tiff | 119.8917 | 53.7917 | 2.23× | 30.6527 | 1.75× |
| original-testfax4.tiff | 25.8453 | 26.0761 | 0.99× | 11.5012 | 2.27× |
| original-deflate-last-strip-extra-data.tiff | 5.6373 | 2.8084 | 2.01× | 0.2162 | 12.99× |
| white-none | 0.0130 | 0.0123 | 1.06× | 0.1195 | 0.10× |
| rgba-none | 1.8890 | 0.0313 | 60.38× | 0.0415 | 0.75× |
| rgba-tile-none | 21.9019 | 0.2493 | 87.84× | 0.1054 | 2.37× |
| bilevel-none | 4.6427 | 4.6362 | 1.00× | 0.3041 | 15.25× |

Cases more than 5% slower in this run: webp (5.7% slower). These results remain included in the aggregate; no timing-noise exemption is assumed.


## Opt-in combined Deflate API

`decode_deflate_into` uses caller-owned output and native scratch, with a
supported-profile acceptance theorem, checked sufficient budget, exact pixels
and unchanged output on failure. Scratch may be partial on failure. The CLI
and C adapter do not automatically use it. All five inputs here use strips;
final-strip padding and checksums are still checked. See `deflate.md`.

| Case | Before ms | Ordinary now ms | Combined now ms | Before / combined | Combined / 4.7.1 |
| --- | ---: | ---: | ---: | ---: | ---: |
| gray-deflate | 18.5252 | 6.7794 | 3.8684 | 4.79× | 8.85× |
| rgb-deflate | 52.8703 | 19.6852 | 10.1690 | 5.20× | 8.50× |
| gray16-deflate | 85.4815 | 61.4428 | 30.6580 | 2.79× | 3.63× |
| noise-deflate | 18.1563 | 5.8412 | 3.3741 | 5.38× | 8.50× |
| original-deflate-last-strip-extra-data.tiff | 5.6373 | 2.8084 | 1.3853 | 4.07× | 6.41× |

## C API fresh-handle RGBA

The same consumer opens, renders and closes a handle each iteration. Adapter
allocation, inspection and conversions are included. Adapter source is
unchanged and outside the core proof boundary.

| Case | Before ms | After ms | Speedup | 4.7.1 ms | System ms |
| --- | ---: | ---: | ---: | ---: | ---: |
| gray-none | 3.5851 | 3.5120 | 1.02× | 0.5192 | 0.7117 |
| gray-packbits | 8.9095 | 7.8574 | 1.13× | 1.6651 | 1.8393 |
| gray-lzw | 23.2464 | 11.8016 | 1.97× | 1.4398 | 1.6773 |
| gray-deflate | 27.9221 | 15.1902 | 1.84× | 0.9073 | 0.8149 |
| rgb-none | 7.5129 | 7.4975 | 1.00× | 0.7508 | 0.7320 |
| rgb-lzw | 70.8945 | 34.7788 | 2.04× | 3.4039 | 3.3711 |
| rgb-deflate | 81.4043 | 43.9635 | 1.85× | 1.7069 | 0.8426 |
| rgb-jpeg | 94.0432 | 39.8410 | 2.36× | 3.9817 | 2.6344 |
| rgb-jpeg-textured | 455.5175 | 188.6268 | 2.41× | 26.2672 | 21.2589 |
| rgb-tile-lzw | 74.2847 | 41.8227 | 1.78× | 3.1775 | 3.2222 |
| gray16-deflate | 132.7960 | 98.1793 | 1.35× | 8.0467 | 5.4156 |
| noise-lzw | 87.9876 | 34.7136 | 2.53× | 3.0331 | 3.1920 |
| noise-deflate | 28.8421 | 12.9962 | 2.22× | 1.1049 | 0.9308 |
| fax3 | 23.8658 | 23.6596 | 1.01× | 0.7497 | 0.7313 |
| fax4 | 12.7648 | 12.4616 | 1.02× | 0.7833 | 0.8758 |
| webp | 32.6302 | 32.7122 | 1.00× | 1.8368 | 1.7975 |
| original-quad-lzw-compat.tiff | 14.3810 | 8.5961 | 1.67× | 1.3627 | 1.3617 |
| original-lzw-single-strip.tiff | 240.4943 | 173.6710 | 1.38× | 7.4786 | 8.0704 |
| original-testfax4.tiff | 60.2926 | 59.3607 | 1.02× | 4.0402 | 3.9659 |
| original-deflate-last-strip-extra-data.tiff | 8.3330 | 5.2412 | 1.59× | 0.3403 | 0.3054 |
| white-none | 0.8418 | 0.8505 | 0.99× | 0.1443 | 0.1756 |
| rgba-none | 3.6097 | 1.7466 | 2.07× | 0.3218 | 0.3290 |
| rgba-tile-none | 22.8667 | 2.0110 | 11.37× | 0.3930 | 0.3887 |
| bilevel-none | 5.5546 | 5.6899 | 0.98× | 0.0814 | 0.0808 |

## Validation and remaining work

The full crate verifies: 1,038 obligations, zero errors, unchanged
`--no-cheating --rlimit 100` gate. All 40 negative checks, 34 Rust API tests,
87 Python test methods, 5,616 matrix cases, 31 original TIFFs, 11 C compatibility
methods, 20,000 core mutations and 2,000 C ABI mutations pass. Separately,
upstream libtiff passes its own 108 C tests. Before/after output is exact,
including JPEG; original JPEG versus libtiff retains its documented tolerance.

Remaining gaps include canonical Huffman length walks instead of prefix
tables, per-byte Deflate stored/match processing instead of bulk runs,
compressed validation/replay outside the combined Deflate strip API, and
adapter allocation/conversion/handle costs. Bilevel expansion remains scalar, with geometry shared across eight bits.
The core has no heap allocation, unsafe decoder or new proof axiom.

The reverse patch and archived proof hashes reconstruct v19. See
`benchmarks/README.md` for reproduction. Historical timings are not used as
this run's speedup denominator.
