# Evidence provenance

## Current codec and conversion kernel optimization

`kernel-optimization/summary.json` binds the final core/C sources, fresh proof,
all 84 negative gates and the full ordinary/sanitized runtime suites. The
positive proof is whole-crate, with 1,672 obligations and zero errors; the
20-chunk receipt includes every command in `verify.sh`. Runtime evidence covers
56 Rust tests, 97 core Python methods, 63 C methods in both ordinary and
ASan/UBSan runs, 31 original images, 20,000 core mutations and 108 upstream tests.

`results.json`/`results.csv` retain all 92 three-engine comparisons and raw
samples. Every before/after output is exact, including JPEG; 90 match libtiff
exactly and two textured-JPEG results differ by at most one channel value.
No median slowdown exceeded 10%, so no longer regression rechecks were needed.
The exact prior source was freshly built, and the reverse patches restore all
archived core/C hashes for the preceding three benchmark baselines.

The initial overlapping proof/test jobs were interrupted after resource
contention; initial C/upstream runs also found missing executable permissions
on existing oracle binaries. Final complete reruns passed. Those initial logs
are retained separately and are not passing evidence. C sanitizers do not
instrument Rust; leak checking is disabled. See `../kernel-optimization.md`.

## Prior Deflate/LERC optimization

`reader-optimization/summary.json` binds the final core/C sources and fresh
validation logs. The whole-crate proof passes 1,638 obligations with zero errors;
all 72 semantic/acceptance negative gates pass. `gate-receipt.json` records 18
successful chunks containing every command from `scripts/verify.sh`, using the
same pinned verifier and `--no-cheating --rlimit 100` settings. Runtime evidence
covers 52 Rust tests, 97 core Python methods, 57 C methods in both ordinary and
C ASan/UBSan runs, 31 original corpus images, 20,000 deterministic core mutations,
and the independently built libtiff oracle's own 108 tests.

The initial sequential proof log includes a successful full positive proof;
that run was stopped to finish all gates through the parallel orchestrator.
The initial sanitizer log records the runner's LeakSanitizer/ptrace failure;
the complete rerun disables leak detection and passes. C sanitizer scope excludes
Rust internals. These initial logs are retained, not counted as passing runs.

`results.json`, `benchmark.log`, and `results.csv` record the three-engine,
92-workload comparison, exact before/after outputs, seven timing batches and
three file-backed peak-RSS samples per workload/engine. `baseline-build-receipt.json`
and `baseline-restore.log` record exact archived core/C hash reconstruction and a
fresh baseline build. The earlier bottleneck measurements remain unchanged in
`reader-benchmark/`. See `../reader-optimization.md` for findings and limitations.
The remaining sections below are historical checkpoints; their proof counts,
API profiles, and performance measurements belong to their recorded sources.

## Prior read-side API, memory, sample and crop pass

Fresh `reader-*` evidence records 1,631 full-crate obligations with zero errors,
68 negative gates, 49 Rust tests, 97 core Python test methods, 57 C test methods
(including 12 new reader methods), the 108-test upstream oracle suite, all 31
original images and 20,000 deterministic fuzz mutations. The complete C suite
also passes ASan/UBSan; Rust is not instrumented and LeakSanitizer is unavailable
under the runner. `reader-summary.json` contains hashes and scope. Initial logs
retain resolved executable-permission/harness and LeakSanitizer failures; final
runs passed. Historical deliveries below retain their original scope.

Historical LERC / static WebP delivery, 2026-10-15 UTC:

- `codecs-summary.json` binds final source hashes, logs, tools and results.
- `codecs-proof.log`, `codecs-gate-receipt.json`, `codecs-verification.json` and `codecs-verification-summary.json`: full-crate proof, 1,618 obligations, zero errors, all 64 negative gates. The parallel runner executes every command from `verify.sh`; positive verification is still whole-crate. `reject-codecs-*.log` records the 12 new semantic/acceptance failures.
- `codecs-tests.log` and `codecs-lerc-edge.log`: 46 Rust tests and 97 core Python methods in total; all previous suites and new LERC/WebP matrices. The edge-case method was added after the main run and executed separately against the same built decoder.
- `codecs-compat.log`: all 45 C regression methods, native/RGBA reference comparisons, 207 malformed LERC cases, 2,000 previous and 2,000 new-codec C mutations.
- `codecs-sanitizers.log`: fresh complete C ASan/UBSan suite on current sources. Rust is not instrumented; LeakSanitizer is disabled. Initial `codecs-tests-initial.log`, `codecs-compat-initial.log` and `codecs-sanitizers-initial.log` preserve the resolved executable-permission/stale-test failures.
- `codecs-fuzz.log`: 20,000 deterministic core mutations. `codecs-corpus.log`: all 31 original images. These are smoke tests, not coverage-guided fuzzing.
- `codecs-upstream.log`: 108 tests against independently built libtiff itself. Test dependencies are pinned libtiff 4.7.1, libwebp 1.6.0, Esri LERC 4.0.0 and Zstandard 1.5.7.
- `codecs-linkage.txt` and `codecs-c-api-source-sha256.json`: 198 exports, 145 implemented/restricted APIs, 53 stubs, with no native production codec-library dependencies.

Exact new format and numerical boundaries are in `../lerc.md` and
`../webp-native.md`. The original whole-image profiles and acceptance witnesses
remain intact. C/BigTIFF integration is tested, not end-to-end verified.
Older logs and installed-consumer results below remain historical evidence.


Historical adoption delivery, 2026-10-15 UTC:

- `adoption-summary.json` ties the final sources, test counts and log hashes together.
- `adoption-proof.log`, `adoption-verification.json` and `adoption-verification-summary.json` record the full 1,215-obligation proof and negative gates on current core sources.
- `adoption-tests.log`, `adoption-corpus.log`, `adoption-fuzz.log` record core runtime and mutation results; `adoption-compat.log` records all four C regression suites.
- `adoption-sanitizers.log` records all four C ASan/UBSan suites. `adoption-sanitizers-initial.log` retains an earlier environment-level executable-permission failure, resolved before the final run. Rust is not instrumented; LeakSanitizer is disabled.
- `adoption-installed.log` and `adoption-installed-consumers.json` record 30 installed-consumer and 12 Pillow matches. ImageMagick reader-symbol binding is now explicitly checked as well.
- `adoption-upstream.log` records a fresh 108-test run against upstream libtiff; this is not a full-application test of our adapter.
- `adoption-linkage.txt` records that the replacement library has no production codec-library dependency.
- `c-api-api.json`, `c-api-source-sha256.json` and `compat/api.json` are updated for the final adapter. There are 145 implemented/restricted functions and 53 stubs among 198 public exports.

That checkpoint completed priorities 1–5 and 7 plus RLE, Predictor 3 and Zstandard. The current codec delivery below closes its LERC/static-WebP gaps. See `../adoption-status.md`.

Historical consumer-compatibility checkpoint (v22), task date 2026-10-15:

- `verification.json`, `verification-summary.json`, `proof-gate.log` and all `reject-*.log`: fresh full-crate proof, 1,076 obligations, zero errors; all 46 negative checks pass. `consumer-gate-receipt.json` records all 15 successful chunks. The two added checks corrupt Predictor 3 shuffling and RLE row alignment.
- `consumer-core-tests.log`, `consumer-corpus.log`, `consumer-fuzz.log`: 40 Rust tests, 87 Python methods, existing 5,616-case matrix, all 31 original images and 20,000 deterministic mutations.
- `consumer-compat.log`: all 34 C regression methods, new independent format/metadata matrices, temporary decoder workflows and 2,000 adapter mutations.
- `consumer-asan.log`: fresh C ASan/UBSan pass. Rust internals are not instrumented; LeakSanitizer remains disabled under the environment tracer.
- `consumer-installed.log`, `installed-consumers.json`: 26/26 installed ImageMagick/Leptonica/GdkPixbuf comparisons with actual adapter loading; `consumer-pillow.log`: 12/12 installed Pillow comparisons with loader bindings checked. The GDAL sequence is exercised by a dedicated harness, not by an installed GDAL.
- `c-api-api.json`, map and source hashes: 145 implemented/restricted APIs, 53 stubs, 198 public exports. No production native codec dependencies. `consumer-summary.json` records source and log hashes.
- `consumer-baseline-*` preserves the prior proof and adapter hashes. Older `api-five-*` results are historical. The 108-test pinned libtiff suite was not rerun for this checkpoint; it tests libtiff itself.
- No new speed measurements or end-to-end C/BigTIFF theorem. Codec work is partial: RLE and Predictor 3 are implemented; ZSTD, LERC and broader WebP remain unsupported. See `../consumer-compatibility.md`.

The current section supersedes current-version wording in historical sections below.


Historical reader/API delivery (v21), task date 2026-10-15:

- `verification.json`, `verification-summary.json`, `proof-gate.log`,
  `verus-version.txt`, `false-contract.log`, and all `reject-*.log` are fresh:
  1,064 obligations, zero errors, all 44 negative gates. Four new checks cover
  native prediction, BigTIFF offsets, SOS remapping and native acceptance.
  `api-five-gate-receipt.json` records all 15 successful independent chunks of
  the full `verify.sh` gate. Positive verification remains whole-crate,
  `--no-cheating --rlimit 100`, with exact source hashes.
- `api-five-tests.log`: 38 Rust tests and 87 Python tests; all existing generated
  image matrices. `api-five-corpus.log`/`original-corpus.json`: all 31 original
  first-main images, 27 exact and four JPEG comparisons within the specified
  tolerance. The original whole-image profiles remain unchanged.
- `api-five-compat.log`: 12 original/expanded C test methods plus 11 new methods.
  Includes 640 broader native-format files (544 also matched to pinned libtiff),
  32 RGBA16-alpha cases, all 64 source/output orientation pairs, 16 oriented
  strip/tile cases, packed FillOrder cases, native palette/associated-alpha/
  padded tiles, metadata/options, sparse BigTIFF >4 GiB, SubIFDs, lazy callback
  I/O, read-wide work limits and output frames. The three original OJPEG native
  rasters, after cropping tile padding, equal the original Rust pipeline.
- `api-five-sanitizers.log`: fresh ASan/UBSan runs of both C suites and 2,000
  adapter mutations on final adapter sources. C shim and consumers are
  instrumented; Rust is not. LeakSanitizer remains disabled under this tracer.
- `api-five-pillow.log`: installed Pillow 12.3.0, forced libtiff decoding,
  actual loader binding to our library and 12 exact normal-decoder comparisons.
- `api-five-fuzz.log`: 20,000 deterministic core mutations. Neither mutation
  suite is coverage-guided fuzzing.
- `api-five-upstream.log`/`libtiff-test-suite.log`: fresh pinned libtiff rebuild
  and 108 passing upstream tests, which test libtiff itself.
- `c-api-api.json`, export audit/map and `c-api-source-sha256.json` describe the
  current adapter: 136 implemented/restricted entry points, 62 explicit stubs,
  198 public symbols, no production native codec dependencies.
- `api-five-summary.json` ties these logs, binaries, harnesses and sources to
  this delivery. `api-five-baseline-verification.json` and
  `api-five-baseline-c-api-source-sha256.json` preserve v20 evidence.
  `api-five-baseline-restore.log` records its exact reconstruction using the
  new reverse patch. There are **no new speed measurements** in this delivery.

The current list above supersedes any current-version wording in historical
sections below. Older performance reports remain tied to their recorded source
hashes; their times are not measurements of the expanded C adapter. The new
BigTIFF/C pipeline has local verified primitives, not an end-to-end acceptance
proof; see `../native-compatibility.md`.

Historical throughput run (v20), 2026-10-15:

- `verification.json`, `verification-summary.json`, `proof-gate.log`,
  `verus-version.txt`, `false-contract.log`, and every `reject-*.log`: fresh
  full-crate verification, 1,038 obligations and 40 negative gates. Five new
  gates reject broken word extraction, IDCT DC handling, native identity copy, bilevel span truncation
  and always-error combined Deflate decoding. The final run used
  `scripts/verify-parallel.py` to execute every unchanged `verify.sh` command
  in independent chunks; all 14 chunk exit codes are recorded in
  `throughput-gate-receipt.json`. The positive proof still covers the whole crate.
- `throughput-tests.log`, `throughput-corpus.log`, `throughput-compat.log`,
  `throughput-fuzz.log`: current runtime, corpus, C API and mutation results.
  `original-corpus.json` and codec comparison JSON files are fresh.
- `throughput-upstream.log`, `libtiff-test-suite.log`: all 108 tests in the
  separately rebuilt libtiff 4.7.1 suite; these exercise libtiff itself.
- `throughput-build.log`, `throughput-results.json`, `throughput-run.log`:
  final release builds and uninstrumented seven-batch comparisons. The
  additional `combined` engine is opt-in and excluded from ordinary API gains.
- `throughput-baseline-verification.json`: archived v19 proof and source hashes.
  `throughput-baseline-restore.log` and `throughput-v17-restore.log`: verified
  reconstruction of exact earlier sources with the shipped patch chain.
- `throughput-first-pass-results.json` is a superseded partial sweep that
  exposed a textured-JPEG regression. `throughput-profile/` records diagnostic
  CPU samples and disassembly of the out-of-line entropy helpers. Sampling
  overlapped the abandoned sweep; its instrumented timing is not a speedup.
- `throughput-second-pass-results.json` is the superseded sweep after the
  JPEG fix; it exposed the large bilevel LZW scatter regression. The final
  byte-span mapping addresses it. `throughput-profile/lzw-single-regressed.*`
  and `lzw-regressed.asm` record its per-byte address overhead.
- C adapter sources are unchanged; the v16 sanitizer evidence remains
  historical. No fresh sanitizer run is claimed for this core-only pass.

The current evidence above supersedes the current-version claims in the older
run descriptions below. Historical profiling/optimization logs and measurements
remain attached to their recorded versions.


Historical profiling run (v19), 2026-10-14:

- `verification.json`, `verification-summary.json`, `proof-gate.log`,
  `verus-version.txt`, `false-contract.log`, and every `reject-*.log`: full
  924-obligation proof and 35 negative gates, including corrupted Huffman-cache
  and aligned stored-byte shortcuts.
- `profile-tests.log`, `profile-corpus.log`, `profile-compat.log`,
  `profile-fuzz.log`: fresh runtime, original-corpus, adapter and mutation runs.
  Current `original-corpus.json` and codec `*-comparison.json` files come from
  these runs. Earlier per-feature logs remain historical.
- `profile-upstream.log`, `libtiff-test-suite.log`: freshly rebuilt pinned
  libtiff 4.7.1, all 108 upstream tests passing. These test libtiff itself.
- `profile-build.log`: final release benchmark/adapter build.
- `profile-results.json`, `profile-run.log`: all 26 native / 24 RGBA cases,
  seven interleaved batches per engine/case, exact before/after outputs,
  pinned 4.7.1 reference and separately identified system RGBA reference.
- `profile-first-pass-results.json`, `profile-first-pass-run.log`,
  `profile-first-pass-resume.json`: superseded complete sweep exposing the
  LogLuv inlining regression. The final sweep above includes its fix.
- `profile/`: flat CPU samples, raw PC/load-bias records, baseline/final
  disassembly and hashes. `probe*`, `regressed-*` and `first-pass-*` files are intermediate diagnostic variants,
  not the delivered implementation or the reported uninstrumented speedups.
  The sampler and resolver used for these profiles are copied into this directory.
- `profile-baseline-verification.json`: archived v18 proof/hashes (915 obligations).
  `profile-baseline-restore.log` confirms exact reconstruction of all 51 v18 files;
  `profile-v17-restore.log` also checks the older v17 restoration.

Historical v18 optimization run, 2026-10-14:

- `optimization-verification.json`, `optimization-proof-gate.log`: v18 full
  915-obligation proof and 33 negative gates.
- `optimization-tests.log`, `optimization-corpus.log`, `optimization-compat.log`,
  `optimization-fuzz.log`: v18 regression runs.
- `optimization-build-final.log`, `optimization-upstream.log`: v18 builds/reference.
- `optimization-results.json`, `optimization-run.log`: v17-to-v18 measurements.
- `optimization-inlining-probe.*`: v18 diagnostic variants, not current speedups.
- `optimization-baseline-restore.log`, `optimization-baseline-verification.json`:
  v17 reconstruction and archived 899-obligation proof/hashes.
- `benchmark-*`: original v17 performance measurements.
- `compat-sanitizers-final.log`, `c-api-sanitizer-scope.txt`: earlier v16 C-layer
  sanitizer run. The adapter is unchanged; sanitizers were not rerun here.
- Other per-feature and unprefixed logs record earlier development runs. The
  `profile-*` logs above are authoritative for current runtime validation.

All proof claims exclude the CLI/C adapter and remain relative to the explicit
models and trusted toolchain documented in `../trusted-boundary.md`.
