# Recorded validation

The latest [LERC throughput pass](lerc-throughput.md) has fresh whole-core proof,
100 negative gates, 60 Rust and 97 Python tests, 63 normal and 63 sanitized C
methods, 31 original fixtures, 20,000 mutation cases and all 96 exact before/after
comparisons. Evidence and same-host seven-workload timings are in
`evidence/lerc-throughput/`. The existing five trusted FP primitives are unchanged.


The current kernel optimization is documented in
[kernel-optimization.md](kernel-optimization.md). Fresh evidence in
`evidence/kernel-optimization/summary.json` records **1,672 whole-crate proof
obligations, zero errors, 84 negative gates, 56 Rust tests, 97 core Python
methods, and 63 C methods in both ordinary and ASan/UBSan runs**. All 31 original
images, 20,000 deterministic core mutations and the oracle's 108 upstream tests
pass. C sanitizer scope excludes Rust and leak checking. Every one of the 92
before/after benchmark outputs is exact, including JPEG. The 71 main decode
cases improve 1.42× in geometric mean; runtime relative to libtiff 4.7.1 falls
from 6.57× to 4.63×. No measured median slowdown exceeds 10%.

Earlier sections below are historical checkpoints, bound to their archived
sources. The interrupted overlapping verification/test run and initial missing
execute-bit failures are retained as diagnostics, not counted as passing runs.

The prior Deflate/LERC optimization is specified in
[reader-optimization.md](reader-optimization.md). Fresh results are in
`evidence/reader-optimization/summary.json`: 1,638 verified obligations,
zero proof errors, 72 rejected semantic/acceptance mutations, 52 Rust tests,
97 Python core methods and 57 C compatibility methods, the C ASan/UBSan run,
108 upstream oracle tests, 31/31 original images and 20,000 deterministic fuzz
mutations. C sanitizer instrumentation excludes Rust; LeakSanitizer cannot run
under this runner. New tests cover batch input/output boundaries, exact charging,
strict endings and checksum failures, caller-buffer preservation for complete
native reads, and LERC top-bit/float rounding boundaries. All existing codec
matrices remain. The full proof gate runs every unchanged `verify.sh` command
in 18 independent chunks; its receipt records all zero exit codes. The positive
proof still verifies the entire crate with `--no-cheating --rlimit 100`.
Earlier sections below describe historical deliveries. The prior reader-feature
run remains in `evidence/reader-summary.json` (1,631 obligations, 68 negative gates).

Historical LERC / full static WebP delivery, 2026-10-15 UTC:

- Full pinned Verus proof: **1,618 obligations, zero errors**, with all **64 negative gates** (52 preserved and 12 new), unchanged `--no-cheating --rlimit 100` settings.
- 46 Rust tests, 97 core Python methods (the main run plus the added LERC edge-case method), 45 C regression methods, and all 31 original corpus images pass.
- LERC: 864 matrix cases plus 32 extreme/depth cases against Esri; 384 TIFF layouts, 18 alpha/float-band layouts and six shared-mask layouts. Plain, Deflate and Zstandard wrappers are covered.
- WebP: 192 encoder-produced VP8L images, 50 independent transform/group streams, 140 VP8 images checked in YUV and RGBA, 64 TIFF layouts and 12 alpha/filter/partition configurations. TIFF tests compare both native reads and RGBA presentation where supported by libtiff.
- Primitive checks: 100,000 software IEEE cases, 256,000 binary arithmetic decisions, 10,000 each IDCT/Walsh cases, 10,000 intra predictors and 100,000 filter kernels.
- Malformed LERC: 207 truncation/checksum/ending failures rejected. The C mutation harness runs 2,000 existing inputs and 2,000 additional LERC/WebP mutations, checking caller buffers and failure atomicity. The original core smoke runs 20,000 mutations; the Zstandard suite retains its 1,500 mutations.
- Fresh C ASan/UBSan suites pass; Rust internals are not instrumented and LeakSanitizer is disabled under the tracer. The independently rebuilt libtiff oracle passes its own 108 tests.

The reference differences are explicit: big-endian LERC tests undo an extra
libtiff postdecode byte swap; shared-mask float bands and RGB TIFFs containing
WebP alpha are checked directly against Esri/libwebp where libtiff rejects
those layouts. No new performance claim, full-application certification or
end-to-end C proof is made. See [LERC](lerc.md), [WebP](webp-native.md),
[adoption status](adoption-status.md) and `evidence/codecs-summary.json`.

Initial runs exposed stale negative LERC-availability expectations and removed
execute permissions on two generated test binaries. The assertion was updated
for implemented support, permissions were restored, and the affected suites
were rerun. Initial and final logs are distinguished in the evidence directory.

The previous `adoption-*` checkpoint recorded 30 installed-consumer and 12
Pillow comparisons. Those application runs are historical; this delivery's
new codec comparisons use pinned library oracles and the C API harness.
The current `codecs-*` receipt supersedes current-version wording below.

Historical consumer-compatibility checkpoint (before Zstandard), 2026-10-15:

- Entire core: **1,076 obligations, zero errors**, pinned Verus with no cheating; all 46 negative checks pass.
- Runtime: 40 Rust tests, 87 Python methods, 5,616 existing generated matrix cases; all 31 original images (27 exact, four bounded JPEG comparisons).
- C adapter: all 34 test methods (12 original, 11 native coverage, 11 consumer additions), 198 exports, 145 implemented/restricted and 53 stubs.
- Consumer additions include 128 Predictor-3, 128 CMYK/Lab, 32 RLE and 72 YCbCr cases, typed/custom metadata, EXIF/GPS, limits, read-side fields and temporary decoder contexts.
- Installed applications: 26/26 ImageMagick/Leptonica/GdkPixbuf comparisons and 12/12 Pillow comparisons, with loader paths/bindings checked.
- Fresh C ASan/UBSan pass; Rust internals are not instrumented and LeakSanitizer is disabled under the tracer.
- Deterministic mutation smoke: 20,000 core inputs and 2,000 C-adapter inputs. This is not coverage-guided fuzzing.
- The pinned upstream 108-test result below is retained from the preceding delivery; those tests exercise upstream libtiff.

See [consumer-compatibility.md](consumer-compatibility.md) and the current section of [evidence/README.md](evidence/README.md). ZSTD/LERC and broader WebP remain outstanding; this checkpoint does not complete the codec item. No new performance measurement or end-to-end C proof is claimed.

## Historical throughput validation (v20)

Run on Linux x86-64 on 2026-10-15 UTC after the throughput changes documented
in [throughput.md](throughput.md). The entire crate was reverified.
The proof, runtime, corpus, adapter, upstream and mutation gates below were rerun
for this implementation. C-layer sanitizer evidence is explicitly historical.
GitHub Actions has not run remotely. Current logs are identified in
[evidence/README.md](evidence/README.md).

| Check | Observed result |
| --- | --- |
| Entire core, pinned Verus `--no-cheating --rlimit 100` | 1,038 verified, 0 errors; full-crate flag true |
| Negative proof and semantic checks | All 40 rejected for semantic/acceptance proof failures; five new throughput mutations included |
| Rust API tests | 34 passed; combined Deflate acceptance/mutation/budget tests, exact work limits, partial-prefix behavior and output suffix preservation included |
| Python test methods | 87 passed: 23 original, 4 LZW tail, 5 legacy LZW, 7 alpha, 5 extended, 6 Deflate, 8 JPEG, 7 fax, 4 float64, 9 WebP, 7 LogLuv, 2 upstream |
| Generated profile matrices | 5,616 cases: 5,520 previous + 96 LogLuv |
| Independent SGILog encoder | 48 libtiff-encoded inputs; exact native components |
| Additional LogLuv cases | All 256 u/v codes at six luminance boundaries/signs, packet boundaries, no-ops, pages, malformed metadata/streams and resource limits |
| Adopted upstream-derived fixtures | 31 passed: 22 unchanged originals and 9 transcodes |
| Original libtiff TIFF files | 31/31 accepted (100%): 27 exact native-output comparisons, 4 bounded JPEG comparisons |
| Upstream C libtiff suite | 108 passed, 0 failed, 0 skipped; tests libtiff itself |
| Deterministic mutation smoke | 20,000 inputs; no panic/assertion failure; 70 committed seeds |

## C ABI regression checks

The adapter exports all 198 public functions in libtiff 4.7.1's Linux header:
119 implemented/restricted entry points and 79 explicit ENOSYS stubs (75
generated, four manual). These are symbol counts, not a claim of complete
libtiff behavior. Its Rust/C FFI code is unverified and unchanged in this
optimization. Thirteen of the 52 verified core files changed or were added in this pass;
all core files are covered by the fresh full-crate proof.

| Adapter check | Observed result |
| --- | --- |
| C contract and Python-driven compatibility suite | 11 methods passed |
| Native matrix | 128 files; 256 exact scanline/strip comparisons with independent source samples |
| Native full tiles | 72 exact libtiff comparisons |
| Bilevel, raw, pages and LogLuv | Exact comparisons, padding and native-component checks passed |
| RGBA | 160 independent-pixel checks; 136 exact libtiff comparisons; four orientations and 12 strip/tile cases |
| Known renderer differences | 24 cropped gray/Gray+A tile cases preserve correct independent source pixels instead of the pinned libtiff renderer's row-skipping bug |
| Public exports and linkage | All 198 symbols; standard public versions and SONAME 6; no libtiff/jpeg/webp/zlib dependency |
| Existing binary | Consumer linked to system libtiff.so.6 ran unchanged with adapter via its loader path |
| C ABI mutation smoke | 2,000 inputs / 80 fixtures; 705 accepted, 529 native decodes; no assertion failure |
| Prior C ASan + UBSan (v16, not rerun for this core optimization) | All 11 methods passed; Rust internals not instrumented |
| Prior LeakSanitizer attempt | Unavailable because the environment tracer prevents /proc task inspection; leak detection disabled for the passing ASan run |

All 75 generated stubs were called and checked for ENOSYS and their failure
sentinels. Additional C contracts cover manual stubs, varargs, per-handle/TLS
diagnostics, limits, ownership, callbacks, directory rollback and output
buffer frames. See [C API scope and build instructions](c-api.md).

The 108 upstream C tests still run against upstream libtiff, not the adapter.
The separately rerun core proof, 34 Rust tests, 87 Python tests, original
corpus, and 20,000 core mutations passed. This run rebuilt the pinned test
oracles and executed all 108 upstream tests successfully. The C boundary
source did not change, so the earlier C-layer sanitizer run is retained as
historical evidence rather than claimed as a fresh sanitizer run.

## Prior decoder features preserved

The unchanged `logluv-3c-16b.tiff` contains one SGILog32 pixel. Its native
LogLuv16 containers are `(0,6913,15544)`, emitted as six little-endian bytes
in an NPY array, with explicit `color_model="logluv16"` JSON. The bytes match
both an independent rational conversion and libtiff SGILOGDATAFMT_16BIT.
This is encoded HDR component output; there is no display RGB conversion,
tone mapping or STONITS calibration. See [the exact profile](logluv.md).

Source SHA-256 is `a042e969b1f6ee47703e5187bdac782510023906cd99fdb81ad41e510f4c457d`;
native-byte SHA-256 is `8262676937f21d3b6034030a5cbb1c632929cc7aedb7275a8c280944e44f26bb`.
The original needs six output bytes, ten scratch bytes and 29 decode units.
The dedicated sufficient inspection bound is 327 units for its tight limits.
The CLI's full public inspection plus decode charges 1,336 units; exact and
one-short budgets are tested. A formal witness mirrors the same 166 input bytes.

The pure SGILog packet model specifies exact row/plane expansion, zero
no-ops and strict rejection of missing/extra data. Its constant-state cursor,
integer chroma expansion, whole-TIFF acceptance, exact native pixels, scratch
sizes, budget sums and buffer frames are verified. Five additional negative
gates reject always-error stream/TIFF inspection, missing conversion writes,
zero chroma conversion and removed public dispatch. All 40 current negative
gates, including the five new throughput checks, pass.

The 96-case matrix covers both byte orders, strips/tiles, short final strips,
padded edges, literal/repeated/no-op packets and varied/constant patterns.
Separate libtiff-encoded inputs exercise the independent encoder; its test
helper compensates the pinned writer's generic raw-word byte swapping before
encoding, as documented in logluv.md. No compressed bytes or libtiff source
are modified. The independently built encoder and native oracle are test-only.

Earlier JPEG, fax, float64, WebP and both LZW compatibility regressions remain
passing. The original JPEG differences remain bounded by maximum channel
error 3 and mean absolute error below 0.1 for the specified integer transform
and nearest chroma. This is observed corpus agreement, not a universal
numerical-error theorem. Exact comparisons include the stored native LogLuv
codes; they do not claim lossless recovery of pre-quantization HDR colors.

The 31/31 fraction measures the first main image of each original TIFF in
libtiff 4.7.1's test/images. It does not count all assertions, pages, SubIFDs,
writer features or API functions. The separate 108-test C suite tests libtiff
itself. All originals in this measured corpus are accepted, while the reader
still implements the explicitly documented restricted TIFF profiles.

The deterministic mutation run uses 70 committed seeds and reports:

```
seed=0x5449464650524f4f iterations=20000 accepted=325 errors=[3326, 7046, 37, 4267, 0, 4999] jpeg_accepted=14 jpeg_decoded=3 fax_accepted=9 fax_decoded=8 float_accepted=21 float_decoded=15 webp_accepted=14 webp_decoded=8 logluv_accepted=32 logluv_decoded=32
```

Error counts are Truncated, Invalid, Unsupported, Limit, OutputTooSmall and
PageNotFound. This is mutation smoke testing, not coverage-guided fuzzing.
The LogLuv seeds reached both inspection and decoding. Proof claims remain
relative to the documented mathematical models and trusted toolchain; the
models still need independent specification review.

All 52 verified core source hashes are recorded in
`evidence/verification-summary.json`; root source SHA-256 is
`af1c247fd3a8350cd9ef319df90200588847e543f77655e12a963640e149ac19`. Current proof, negative, runtime,
upstream, corpus and mutation evidence is retained in `evidence/`.

## Performance optimization and comparison

Current same-host measurements are in [throughput.md](throughput.md) and
`evidence/throughput-results.json`: 26 native and 24 C API cases, seven shuffled
batches per engine. Five additional Deflate measurements use the opt-in
combined API; those gains are excluded from the ordinary API aggregate.
Every before/after output is exact, including JPEG. Inputs and binaries are
hashed. The preserved v19 source was freshly built for the denominator;
historical host timings are not reused.

The word readers, tile/native copies, direct strip writes, checksum batches,
JPEG factorization/rendering and combined acceptance theorem retain the original
models and conservative quotas. The combined API leaves output unchanged on
failure while permitting partial scratch. Tests cover exact/one-short budgets,
undersized buffers, suffix frames, malformed streams and original final-strip
padding. See [the proof ledger](verification.md) and [Deflate contracts](deflate.md).

Archived v19 proof/source hashes are in
`evidence/throughput-baseline-verification.json`. Reverse-patch restoration was
checked for v19 and through the chain to v17. Earlier profiling/optimization
reports and sanitizer runs remain explicitly historical.
