# Codec and conversion kernel optimization

Across the 71 main decode workloads, the geometric-mean speedup is 1.42×. The new reader takes 4.63× as long as libtiff 4.7.1, down from 6.57× for the rebuilt previous reader.

| Input / operation | Before ms | After ms | 4.7.1 ms | Speedup | After / 4.7.1 |
| --- | --- | --- | --- | --- | --- |
| Original 24.3 MP LZW scan / rgba | 315.3282 | 82.7021 | 8.4895 | 3.81× | 9.74× |
| LAB8 Deflate / rgba | 142.8764 | 43.6673 | 24.4616 | 3.27× | 1.79× |
| CMYK8 Deflate / rgba | 73.7465 | 29.0216 | 5.1546 | 2.54× | 5.63× |
| Random Gray8 lzw / native | 51.7944 | 18.4678 | 2.5172 | 2.80× | 7.34× |
| Bilevel CCITT g3 / native | 18.8838 | 7.8905 | 0.5162 | 2.39× | 15.29× |
| RGB8 deflate / native | 14.4611 | 8.1041 | 1.4295 | 1.78× | 5.67× |
| Float32 Deflate Predictor 3 / native | 65.6062 | 26.0639 | 9.0776 | 2.52× | 2.87× |
| Float32 LERC plain / native | 60.6793 | 47.4856 | 2.2670 | 1.28× | 20.95× |
| RGBA8 WebP lossless / native | 38.3482 | 31.5311 | 3.2777 | 1.22× | 9.62× |
| RGBA8 WebP lossy / native | 96.4023 | 71.0537 | 8.2031 | 1.36× | 8.66× |
| RGB8 JPEG texture / native | 133.7817 | 130.6413 | 26.0610 | 1.02× | 5.01× |
| RGB8 none / rgba | 2.1491 | 2.1046 | 0.7762 | 1.02× | 2.71× |

## Main decode geometric means

| Operation | Cases | Before / 4.7.1 | After / 4.7.1 | Speedup |
| --- | --- | --- | --- | --- |
| native | 30 | 8.25× | 5.96× | 1.38× |
| scan | 9 | 4.67× | 3.53× | 1.32× |
| rgba | 22 | 7.76× | 5.02× | 1.55× |
| crop | 10 | 3.12× | 2.30× | 1.35× |

No main-run median slowdown exceeded 10%.

## Changes

Packed grayscale and palette RGBA: expand a source byte through a fixed lookup table, retaining partial-byte crops, both fill orders, orientation and output padding. The fixed table occupies 8 KiB. This removes generic sample extraction and per-pixel scaling from the common loop.

Lab and CMYK: build the exact existing 1,501-entry Lab gamma table once per RGBA request; read contiguous 8/16-bit samples directly; admit full private segment staging for complete color reads. Separate-plane and raw YCbCr behavior remains unchanged.

LZW: specialize/in-line bit and dictionary operations, and decode complete native segments once into private storage before checking exact endings. Optional complete RGBA staging uses the existing 1 MiB cap and cumulative/single allocation admission. Partial reads retain full validation and bounded replay.

Deflate: group consecutive match emissions, avoiding state-machine dispatch per copied byte. Every emitted byte still updates the verified history and checksum, charges the same two work units and obeys output/input boundaries.

Fax and Predictor 3: borrow fixed fax codebooks instead of returning array values, specialize scalar bit operations, and replace repeated predictor row division with proved incremental coordinates. Common single-channel sample widths specialize once per call.

WebP: calculate VP8 filter decisions and corrections once per edge window; use an exact DC-only transform shortcut; dispatch common VP8L predictors once per pixel and traverse incremental coordinates with power-of-two tile shifts. Scalar color helpers are inlined.

LERC: split wide rounding through 64-bit words and narrow normal float32 values directly, proving both against the original software IEEE model. Subnormals and exceptional values retain the original path. No native floating-point codec shortcut is used.

## Remaining gaps and proof boundary

JPEG entropy decoding and validation/replay remain a substantial gap. A buffered-prefix experiment showed no measurable gain and was removed; this pass retains only small scalar inlining/power changes. Fax still traverses code trees and repacks expanded pixels; WebP remains scalar; LERC still performs software IEEE reconstruction. Partial compressed reads still validate the complete strip before returning pixels. This pass improves the measured kernels without claiming parity with libtiff or eliminating all bottlenecks.

Core shortcuts refine the existing acceptance, numerical, output, frame and conservative-work models. No proof bypass, unsafe core code or runtime native codec dependency was added. Packed RGBA/Lab tables, color loads, staging admission, input-window continuity and the final caller-buffer commit remain tested ordinary Rust/C outside the formal core boundary. Existing whole-image acceptance profiles are unchanged.

## Validation

The complete crate passed 1,672 obligations with zero errors and all 84 negative gates. 56 Rust tests, 97 core Python methods and 63 C methods passed, as did the complete C ASan/UBSan suite, all 31 original corpus images, 20,000 deterministic core mutations and the independent oracle's own 108 tests. C sanitizers do not instrument Rust; LeakSanitizer is disabled.

All 92 before/after outputs match byte-for-byte, including JPEG. Against libtiff, 90 match exactly; the two textured-JPEG cases retain a maximum difference of one channel value. New tests cover all 65,536 legal VP8 DC values for both transforms, AC impulses, LERC ties/carries and exponent boundaries, packed byte values/crops/fill orders, and single-channel floating predictors. Existing stream tests compare exact work and bytes against scalar transitions across window, output, overlap, ring-wrap and failure boundaries.

Nonzero-column packed and 16-bit crops use independent expected pixels: pinned libtiff 4.7.1 applies col_offset*samplesperpixel to a byte buffer in gtStripContig, which does not represent those sample offsets. Both the previous and current adapter preserve the independently expected result.

## Measurement and reproduction

Measured 2026-10-15T13:25:31.271627+00:00 on AMD EPYC 9V74 80-Core Processor, pinned CPU 0; 7 randomized serial batches of at least 100 ms per engine/workload, plus three separate file-backed RSS processes. The unchanged C consumer opens a fresh handle per operation. Memory-callback timing excludes file loading, output allocation, hashing and startup. The exact previous source, current reader and pinned libtiff 4.7.1 were rebuilt. The reference uses zlib, not libdeflate, and its JPEG build has SIMD disabled. WebP uses the reference build's enabled CPU routines. Main decode geometric means exclude metadata, first-row, strip-height and default-strip-chopping diagnostics. These are host- and workload-specific measurements.

Reproduce with benchmarks/prepare-kernel-baseline.py, the full proof/test gates, benchmarks/prepare-reader.py, benchmarks/build-reader.sh and benchmarks/kernel-optimization.py. Run benchmarks/kernel-optimization-recheck.py for all >10% median slowdowns. The reverse patch restores the exact prior core/C source hashes. Full logs, source/binary/proof hashes, raw timings, correctness hashes and results.csv are archived in docs/evidence/kernel-optimization/.
