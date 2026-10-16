# WebP SSE2 integer kernels

The user explicitly authorized a trusted SSE2 interface on 2026-10-15.
The decoder selects VP8L prediction and color kernels once per row/tile span,
uses SSE2 predictor 12 and four-pixel cross-color/subtract-green transforms,
and uses four-lane VP8 inverse transforms and four-pixel YUV-to-RGBA conversion. Each butterfly intermediate is computed once per lane group;
adjacent color pixels share row geometry. DC-only transforms retain the
existing shortcut. Chroma interpolation and all parsing remain verified core
code. The scalar transform and pixel functions remain the fallback.

SSE2 is baseline on x86-64. On 32-bit x86, runtime feature detection guards the
instructions. Other architectures select scalar code. The `force-scalar`
Cargo feature selects the old core paths even on x86-64, for regression tests.
This feature changes only dispatch in the trusted dependency; both branches
of the same decoder implementation are checked by Verus. ARM/32-bit x86
execution is not established by forced-scalar testing on x86-64.

## Trusted operations

`trusted-simd/src/lib.rs` declares eleven fixed-size integer operation contracts
and an availability query. Their bodies, including architecture detection and
the unsafe intrinsics in `cpu.rs`, are **trusted, not proved**. All operands
and results are fixed arrays passed by value. No caller pointers, slices,
lengths, allocation, entropy parsing, or codec dispatch enter this boundary.

| Operation | Assumed lane semantics |
| --- | --- |
| `add`, `sub` | Exact signed i32 addition/subtraction, with each operand bounded to ±4,000,000. |
| `mul_shift16` | Floor of signed i32 × unsigned constant / 65,536; inputs bounded to ±1,000,000 and constant ≤65,535. |
| `shift3` | Arithmetic i32 right shift by three, equivalent to floor division by eight. |
| `mul_shift8` | Floor of u8 × u16 / 256, returned as nonnegative i32. |
| `pack_shift6` | Floor division by 64 followed by clipping to 0–255. |
| `add_bytes`, `add_bytes16` | Four or sixteen independent byte additions modulo 256. |
| `clamp_add_sub` | Four independent widened byte additions/subtractions, clipped to 0–255. |
| `signed_product_shift5` | Four signed-byte products with floor division by 32. |
| `wrap_bytes` | Four i32 values reduced modulo 256 and packed to bytes. |

SSE2's unsigned 32-bit multiply handles two even lanes at a time. The signed
product helper subtracts `constant * 2^32` for negative lanes, applies the
shift, and interleaves even/odd results. This preserves wide intermediates;
it does not silently adopt libwebp's narrower 16-bit transform overflow
behavior for malformed coefficients. `mul_shift8` widens bytes, shifts them
by eight, and uses unsigned high-half multiplication. The final pack shifts
i32 lanes, saturates to signed i16, then saturates to u8. Integer operations
do not depend on the floating-point rounding environment.

Registers are constructed through same-size value transmutations or four-byte
register moves. No SIMD memory access extends beyond a caller's logical
buffer. Safe core code gathers and stores values after proving image, row and
output bounds. VP8 output groups cannot cross a row; short rows and one-to-three-pixel
tails use the scalar conversion. Image geometry, clipped reads, alpha merge,
stream acceptance, checksums, work quotas, and suffix contracts are unchanged.

## Proof and testing boundary

The whole-crate proof imports the exact audited SIMD and existing FP crates.
Source hashes and imported metadata hashes are recorded separately for both.
No `--no-cheating` claim is made: imported trusted contracts are assumptions.
The source audit still rejects proof escapes and conditional implementations
in decoder sources. `trusted-simd/audit.json` pins every boundary source file.
The Rust compiler, intrinsic lowering, CPU semantics, and dispatch implementation
are trusted. Differential tests do not prove those implementations.

Fifteen added negative gates corrupt a butterfly, transpose, rounding lane,
color coefficient, channel order, source position, row-tail guard, primitive
range precondition, tile-span end, predictor carry, batch store, green or
cross-color lane, predictor formula, or short tail. Every earlier negative gate remains required. The color-coefficient and channel-order
mutations use concrete YUV lanes to expose wrong pixels; the carry and pixel-position
mutations use isolated solver contexts. A solver-limit failure is not an accepted
negative-test result. Tests cover
all 16,777,216 byte/constant multiplication pairs, signed product/shift edges,
200,000 deterministic wide lane vectors, saturation edges, 20,000 full-range
coefficient blocks against the frozen prior scalar transform, 200,000 lossless
pixel groups against the frozen scalar predictor/color implementation, all
signed-byte product and modular-addition pairs, the independent
libwebp primitive oracle, and complete VP8/VP8L/TIFF matrices. Both automatic
SSE2 dispatch and forced-scalar builds are exercised.

The compatibility adapter remains outside the end-to-end formal claim.
ASan/UBSan cover the C shim/consumers, not Rust intrinsics or Rust core code;
LeakSanitizer is disabled. Existing original Image/Raster acceptance profiles
are unchanged.

## Scope and reproduction

VP8L `webp_spans::select` proves a common coefficient/prediction entry over a
row-bounded transform tile. The predictor loop caches left/top-left pixels and
selects the mode once per span; predictor 12 composes clipped channel arithmetic
and modular residual addition. Other modes use their original formulas within
the same span traversal. Cross-color processing reconstructs four reds first,
then uses those reconstructed reds for blue; signed multiplication/shift and
modular output match the prior scalar model. Subtract-green adds all sixteen
bytes in a four-pixel group. Scalar tails and the original complete scalar
traversal remain. Span and batch composition refine the original `finish`
model, including strict acceptance and suffix framing.

The C adapter bulk-initializes bounded scratch/output storage with the existing
accounted allocator. RGBA output uses a single safe flattened-slice copy; RGB
uses fixed-width three-byte stores. This removes per-pixel fallible pushes and
variable-length copies, while retaining allocation limits, private staging and
failed-read atomicity. These adapter changes are tested, not formally verified.
Compiler/library-generated vector memory operations are not additional
project-defined trusted codec primitives.

Entropy decoding, VP8 filtering and ALPH unfiltering retain their existing
implementations. Compressed ALPH benefits from the same lossless engine. An
initial experiment replacing isolated VP8L pixel operations with intrinsics
regressed; those operations are now composed within span kernels so mode and
coordinate selection stay outside the pixel loop. No all-kernel SIMD or
libtiff-parity claim is made.

Reproduce the baseline using `benchmarks/prepare-webp-simd-baseline.py`, which
reverses the exact source patch and checks the archived pre-SSE2 hashes.
Run `scripts/verify-parallel.py --jobs 3`, `scripts/test.sh`, both variants of
`scripts/test-compat.sh`, and the existing corpus/mutation checks. Run
`scripts/test-webp-simd-scalar.sh` for forced-scalar coverage.
`benchmarks/webp-simd.py --rounds 9 --milliseconds 200` uses the unchanged C
consumer and compares every output byte before timing. See
`docs/evidence/webp-simd/` for the recorded run and timing scope.

## Recorded verification

The 2026-10-15 whole-crate run verified **1,764 obligations with zero errors**,
conditional on the documented FP and SIMD boundaries. All **115 negative gates**
passed, including the 15 new SIMD/span mutations. The recorded source hashes
and imported metadata hashes identify this run. Positive verification used the
entire crate; the negative mutations use selected functions.

Regression coverage passed: 62 Rust tests, 97 core Python test methods, 63 C
methods and the same 63 C methods under ASan/UBSan. The SIMD primitive suite
passed its four tests, and the forced-scalar run passed seven Rust tests plus
the three complete WebP matrices (192 encoded VP8L images, 50 independent
transform/group bitstreams, 140 VP8 images). The original 31-image corpus and
20,000 deterministic core mutations passed. The pinned upstream libtiff build
also passed its own 108 tests; that is reference-build validation, not 108
additional adapter tests.

Initial harness failures and corrections are retained in the evidence. The
C test harness restores execute permission on rebuilt consumers. New mutation
checks use semantic/precondition diagnostics and reject solver-limit reports.
The original full proof remained valid while those test diagnostics were refined.

## Recorded throughput

Measured on AMD EPYC 9V74 80-Core Processor, Linux x86-64, pinned logical CPU 0. Each workload has nine randomized serial before/after/libtiff batches of at least 200 ms per engine. These are generated fixtures and targeted reader workloads, not application certification. The unchanged C consumer uses fresh TIFF handles and memory callbacks. Input loading, caller-output allocation and hashing are outside timing; decoder allocations are included. Raw samples and interquartile ranges are in `evidence/webp-simd/results.json`.

All **116 before/after comparisons are byte-exact**, including JPEG. Of these, 114 also match libtiff exactly; the two JPEG comparisons retain their existing bounded difference. All 24 timed WebP workloads match libtiff exactly.

| WebP scope (12 workloads each) | Geometric mean speedup over previous reader | Current runtime / libtiff 4.7.1 runtime |
| --- | ---: | ---: |
| Lossless | 1.55× | 7.53× |
| Lossy | 1.24× | 6.56× |

Each category comprises smooth RGBA, smooth RGB, tiled noisy RGBA, and odd-size RGBA, each tested through native, full RGBA and cropped reads. The 257×259 fixture uses one exact-height strip. The speedups combine SSE2, span traversal, and adapter initialization/output changes; they do not isolate the contribution of intrinsics. This is a WebP-only timing aggregate, not an overall TIFF speedup.

| Fixture, native read | Before (ms) | After (ms) | libtiff (ms) | Speedup |
| --- | ---: | ---: | ---: | ---: |
| webp-lossless | 27.571 | 15.620 | 2.811 | 1.77× |
| webp-lossy | 62.154 | 45.624 | 7.406 | 1.36× |
| webp-lossy-opaque | 47.815 | 38.902 | 3.887 | 1.23× |
| webp-lossy-texture | 293.097 | 274.621 | 40.609 | 1.07× |
| webp-lossy-odd | 3.636 | 2.829 | 0.531 | 1.29× |
| webp-lossless-opaque | 27.041 | 15.397 | 2.787 | 1.76× |
| webp-lossless-texture | 622.480 | 598.426 | 12.944 | 1.04× |
| webp-lossless-odd | 1.731 | 0.910 | 0.190 | 1.90× |

The original lossless fixture's full RGBA read changes from 32.39 to 19.68 ms (1.65×); native decoding changes from 27.57 to 15.62 ms (1.77×). Cropped reads are also included in the raw results.

The noisy lossless native case improves only 4% and remains about 46× libtiff. Its fresh flat profile places 48.9% of samples in `webp_huffman::Huff::read` and 47.9% in unresolved libc routines; inverse color transforms account for about 0.1%. The Huffman API still passes large tables by value, and canonical decoding remains scalar. The flat profile does not attribute every libc sample to a particular call site. This is a different bottleneck from the smooth fixture that motivated this pass.

On the smooth lossless fixture, the current flat profile places 34.4% in pixel/token decoding, 31.8% in prediction spans and 22.5% in color spans. Lossy samples still include unvectorized filter work and chroma interpolation. Profiles include startup, preflight and warmup; their timings are diagnostic, not the benchmark measurements.

### Isolated transform diagnostic

The transform-only harness compares the current function with the frozen previous scalar source, alternating nine 150 ms batches per engine. It checks every block before timing; it excludes TIFF, entropy and rendering.

| Kernel | Previous (ns/block) | Current (ns/block) | Speedup |
| --- | ---: | ---: | ---: |
| DCT | 138.95 | 21.44 | 6.48× |
| WALSH | 56.71 | 16.51 | 3.44× |
| DC | 3.79 | 12.33 | 0.31× |

The retained DC-only shortcut is slower in this isolated binary (12.33 versus 3.79 ns/block). The full-image measurements above include the current implementation; all 24 measured WebP workloads improve, but this pass does not claim that every individual kernel improves.

The archive includes exact reverse-baseline reconstruction, source and binary hashes, both trusted-boundary hashes, complete proof/test evidence, raw benchmark samples, profiles, and the emitted SSE2 kernel assembly.
