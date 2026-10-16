# RGBA row and segment optimization

Uncompressed RGB full RGBA improved 9.22×, from 19.449 to 2.109 ms. Across the 22 full-RGBA cases, the geometric-mean improvement is 2.05×; the new reader takes 7.48× as long as libtiff 4.7.1.

| Input / operation | Before ms | After ms | 4.7.1 ms | Speedup | After / 4.7.1 |
| --- | --- | --- | --- | --- | --- |
| RGB8 none / rgba | 19.4492 | 2.1091 | 0.7924 | 9.22× | 2.66× |
| Gray8 none / rgba | 12.6444 | 1.8672 | 0.6007 | 6.77× | 3.11× |
| RGB8 deflate / rgba | 44.1366 | 14.2895 | 1.9702 | 3.09× | 7.25× |
| RGB8 JPEG smooth / rgba | 56.0151 | 34.3605 | 3.9237 | 1.63× | 8.76× |
| RGBA8 WebP lossless / rgba | 61.7104 | 38.1881 | 4.2239 | 1.62× | 9.04× |

## Main-suite geometric means

| Operation | Cases | Before / 4.7.1 | After / 4.7.1 | Speedup |
| --- | --- | --- | --- | --- |
| native | 30 | 8.30× | 8.38× | 0.99× |
| scan | 9 | 4.61× | 4.61× | 1.00× |
| rgba | 22 | 15.34× | 7.48× | 2.05× |
| crop | 10 | 5.30× | 3.16× | 1.67× |

Main-run median slowdowns exceeding 10%: fax-rle/native (main): 1.20×. Every such workload was rechecked with nine randomized 500 ms batches per engine. None remained above 10%.

## Changes and proof boundary

A new allocation-free Verus row kernel handles contiguous unsigned 8/16-bit RGB and grayscale, both grayscale polarities, native byte order, extra channels, selected alpha and horizontal mirroring. Common layouts specialize once per row. Direct sample loads replace generic read_word calls; RGB unassociated-alpha multiplication is fused into the row loop. Existing grayscale alpha presentation is preserved.

The adapter borrows complete cached segments directly instead of allocating and copying a native row for each conversion. Rows still write to private RGBA storage; the final caller-buffer commit remains failure-atomic.

Completely covered Deflate segments can use the existing one-pass native decoder when the decoded segment is at most 1 MiB and conservative single/cumulative allocation checks admit its scratch. Otherwise reads retain validation and bounded replay. This optional optimization preserves the old conservative work quota; it does not relax checksum, ending, crop or allocation checks.

The new core proves exact row acceptance, output components and rounding, byte order, horizontal reversal, unchanged output on invalid arguments, suffix preservation and bounded termination. The C/FFI orchestration, allocation admission and caller-buffer commit remain tested ordinary Rust/C, not an end-to-end verified TIFF pipeline. No new trusted contract, unsafe core code or runtime native codec dependency was added. Packed bilevel/palette, separate-plane and CMYK/Lab/raw-YCbCr conversion still use existing conversion paths. Partial compressed reads still validate complete streams before exposing pixels.

## Validation

The complete crate passed 1,648 proof obligations with zero errors and all 76 negative gates. All 55 Rust tests, 97 core Python methods and 61 C test methods passed; the complete C suite also passed ASan/UBSan. The original 31-image corpus, 20,000 deterministic core mutations and the oracle's own 108 upstream tests passed. C sanitizers do not instrument Rust; LeakSanitizer is disabled in this environment.

All 92 before/after outputs match byte-for-byte, including JPEG. Against libtiff, 90 match exactly; two textured-JPEG cases retain a maximum difference of one 8-bit channel value. Independent new tests cover every 16-bit narrowing value, all 65,536 byte-value/alpha pairs, extra-channel selection, invalid arguments and output guards, mirrored crops, predicted full Deflate reads in both byte orders, tight cumulative-memory fallback and a corrupt later strip leaving the entire caller buffer untouched.

Expanded tests also preserve the independently expected pixels for nonzero-column 16-bit crops. Both the prior and new adapter agree. Pinned 4.7.1 gtStripContig adds col_offset*samplesperpixel to a byte buffer without multiplying by bytes per sample; its output differs for these cases. Those cases therefore use an independent pixel oracle instead of adopting that offset behavior.

## Method and reproduction

Measured 2026-10-15T12:01:11.221857+00:00 on AMD EPYC 9V74 80-Core Processor, pinned CPU 0. Each of the 92 workloads uses 7 randomized serial batches of at least 100 ms per engine, plus three separate file-backed RSS processes. The identical C consumer opens a fresh handle per operation; timing uses common memory callbacks and excludes file loading, output allocation, hashing and startup. The exact preserved baseline, current reader and pinned libtiff 4.7.1 were rebuilt. The reference uses zlib rather than libdeflate; its pinned JPEG build has SIMD disabled. LERC/WebP/Zstandard versions and all source, input, binary and proof hashes are in results.json. Main-suite geometric means exclude eight strip-height and six default-strip-chopping diagnostics. RSS includes caller storage and process overhead. Results are specific to this suite and host.

Run `benchmarks/prepare-rgba-baseline.py`, the full proof/test gates, `benchmarks/prepare-reader.py`, `benchmarks/build-reader.sh`, then `benchmarks/rgba-optimization.py`. Raw measurements, full proof/gate logs, test logs and source hashes are in `docs/evidence/rgba-optimization/`; `results.csv` contains every workload. The source reverse patch restores and hash-checks the exact pre-RGBA reader. Earlier Deflate/LERC results remain in [reader-optimization.md](reader-optimization.md).
