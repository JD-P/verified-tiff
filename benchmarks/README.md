# Current kernel optimization comparison

See [kernel-optimization.md](../docs/kernel-optimization.md). After the full
proof and test gates, run in a clean checkout:

```bash
python3 benchmarks/prepare-kernel-baseline.py
python3 benchmarks/prepare-reader.py
bash benchmarks/build-reader.sh
python3 benchmarks/kernel-optimization.py
python3 benchmarks/kernel-optimization-recheck.py
```

The unchanged C consumer exercises 92 workloads, requires exact before/after
bytes including JPEG, and records randomized serial timing and RSS samples.
The baseline reverse patch reconstructs the preceding RGBA release and checks
all archived core/C hashes before building. Earlier restoration scripts first
undo this kernel pass. Run timing without concurrent builds, proofs or tests.

# Prior RGBA optimization comparison

See [docs/rgba-optimization.md](../docs/rgba-optimization.md). The RGBA pass uses
the same 92 workloads and C consumer, with the exact pre-RGBA reader preserved
as its baseline. In a fresh tree, run the full proof/test gates, then:

```bash
python3 benchmarks/prepare-rgba-baseline.py
python3 benchmarks/prepare-reader.py
bash scripts/build-compat.sh
bash benchmarks/build-reader.sh
python3 benchmarks/reader-bench.py --generate-only
python3 benchmarks/rgba-optimization.py --rounds 7 --milliseconds 100
python3 benchmarks/rgba-optimization-recheck.py
```

Run timings without concurrent CPU-heavy jobs. Results are in
`target/rgba-optimization/`; archived proof, tests and measurements are under
`docs/evidence/rgba-optimization/`. `rgba-optimization-report.py` renders them.
The new reverse patch also precedes the pre-Deflate/LERC reader restoration.
The older v17–v20 recipes below require their corresponding source release;
they do not reconstruct those releases from the current reader tree.

# Earlier Deflate/LERC optimization comparison

See [docs/reader-optimization.md](../docs/reader-optimization.md) for the current
reader, its exact pre-optimization baseline, and freshly built libtiff 4.7.1.
All 92 workloads require byte-identical before/after results, including JPEG.
Run with no concurrent CPU-heavy jobs:

```bash
bash scripts/verify.sh
python3 benchmarks/prepare-reader-baseline.py
python3 benchmarks/prepare-reader.py
bash scripts/build-compat.sh
bash benchmarks/build-reader.sh
python3 benchmarks/reader-bench.py --generate-only
python3 benchmarks/reader-optimization.py --rounds 7 --milliseconds 100
python3 benchmarks/reader-optimization-recheck.py
```

The baseline helper creates a new separate tree, applies the shipped reverse
patch with zero fuzz, checks every archived core/C source hash and builds it.
It refuses to overwrite an existing baseline tree. The runner checks current
sources against the fresh full-crate proof, checks baseline library/source
provenance, and records source, input, binary, dependency and toolchain hashes.
`--resume` rejects changed binaries, sources, proof, CPU or timing settings.
The C consumer and timing boundaries are unchanged from the earlier benchmark.
Results live in `target/reader-optimization/`; archived evidence is under
`docs/evidence/reader-optimization/`. The report generator is
`benchmarks/reader-optimization-report.py`.

The recheck selects every original median slowdown greater than 10%, runs nine
500 ms batches per engine, and preserves every original sample and aggregate.
Its separate results help identify timing variation; they never replace the main
measurements or establish statistical significance.

# Earlier C reader benchmark

The pre-optimization reader was measured against a fresh pinned libtiff 4.7.1 build in
[docs/reader-benchmark.md](../docs/reader-benchmark.md). This comparison uses
the same C consumer for both implementations: native strips/tiles, sequential
scanlines, full RGBA, 128 × 128 crops, first-row reads, and large strip tables.
It includes LERC, ZSTD, full static WebP, Predictor 3, float16, complex samples,
and the latest bounded streaming/crop paths. The 78 main workloads, eight
layout diagnostics, six default-strip-chopping checks, and three CPU profiles are preserved in
`docs/evidence/reader-benchmark/`, with source/input/binary hashes and build logs.

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
```

Each operation opens a fresh handle. Timings use shared in-memory callback
code; separate file-backed processes measure peak RSS. Both disable mmap and
strip chopping. The large-table fixture also measures libtiff's `O` deferred
metadata option. Lossless outputs must agree exactly; JPEG allows the existing
maximum three-channel-value bound (the recorded maximum is one). All timed
and RSS-run hashes must match their preflight output. CPU affinity and
`/proc/self/status` make this runner Linux-specific. NumPy is used only when
checking a nonidentical JPEG output. Do not run CPU-heavy jobs concurrently.

`reader-bench.py --validate-only` performs output checks without timing;
`--resume` requires matching sources, binaries, CPU, and settings. Fresh runs
write to `target/reader-benchmark/` and never overwrite the archived evidence.
`reader-report.py` renders the recorded results to Markdown/CSV and a standalone
HTML report; it defaults to the archived evidence. Full methodology, raised
resource quotas, compiler flags, codec versions, and limitations are in the report.
The pinned JPEG reference has SIMD disabled. Production sources are unchanged;
hashes match the prior proof record. This comparison adds no proof claims.

# Historical decoder benchmarks

The reports and commands below belong to earlier source versions and API
profiles. Use the C reader benchmark above for the current adapter.

## Decoder performance benchmark

This harness measures the decoder and C adapter against real libtiff.
The current profiling report is `docs/profiling.md`, with raw batches,
input/source/binary hashes and versions in `docs/evidence/profile-results.json`.
`docs/optimization.md` retains the earlier v17-to-v18 pass.
`docs/benchmark.md` and `benchmark-results.json` retain the historical v17 baseline.

## Reproduce

```bash
python3 scripts/bootstrap.py
python3 scripts/upstream.py
bash scripts/verify.sh
bash scripts/build-benchmarks.sh
python3 benchmarks/prepare-baseline.py --version 18
(cd target/optimization-baseline && bash scripts/build-benchmarks.sh)
python3 benchmarks/run.py --generate-only
python3 benchmarks/profile-batch.py --baseline-root target/optimization-baseline
python3 benchmarks/compare.py --baseline-root target/optimization-baseline --rounds 7 --milliseconds 80 --output target/profile-results.json
python3 benchmarks/profile-report.py
```

The baseline helper creates a separate tree using the shipped reverse patch;
it checks every original core file against the archived v18 proof hashes.
Use `--version 17` for the older baseline, or `--version 20` for the last
throughput delivery. The v20 reverse patch restores that delivery's C adapter
and scripts as well as its core. The helper verifies its archived hashes.
It never edits the current decoder. Both versions compile with the same pinned
toolchain and benchmark consumers. `compare.py` checks the current source
against a fresh successful full-crate proof in `target/verification-summary.json`.
Each tree's C adapter hashes must match its own evidence. The v21 native/C
changes are recorded separately; the harness no longer claims identical adapters.
All before/after output bytes must agree exactly, including JPEG, before timing.

For the separate current-version inspect/decode/total comparison, use
`benchmarks/run.py --rounds 7 --milliseconds 80`, then `benchmarks/report.py`.
That replaces the historical single-version report; it is not needed for the
before/after comparison above. Refresh the recorded proof summary if modifying
the core before using that single-version harness.

The system comparison needs Ubuntu/Debian's `libtiff.so.6` (or an ABI-compatible
system installation). The pinned reference is libtiff 4.7.1 plus the project's
pinned test codec dependencies. Python NumPy is needed only to diagnose or
accept bounded JPEG pixel differences. The harness also uses Linux CPU affinity.
Use `--case substring` for a subset, and `--resume` to continue an interrupted
run with identical binaries/settings. Results are checkpointed after each case.
Do not run unrelated CPU-heavy jobs during the measurements.

## What is timed

- `rust_inspect`: the public `inspect_raster` validation path only.
- `rust_decode`: `Raster::decode_with_scratch`, using an already inspected
  descriptor and preallocated output/scratch. Every iteration really decodes;
  it does not use the C adapter's segment cache.
- `rust_total`: inspect plus decode into reusable buffers.
- `libtiff_decode`: encoded-strip/tile reads on an already opened TIFF, with
  output converted to the same canonical representation as the Rust raster.
- `libtiff_total`: a new in-memory `TIFFClientOpen`, metadata setup, native
  decoding/canonical conversion, and `TIFFClose`. Output buffers are reused.
- `libtiff_rgba`, `adapter_rgba`, `system_rgba`: identical C consumer opening
  a fresh in-memory TIFF, calling `TIFFReadRGBAImageOriented`, and closing it.
  Caller output is reused; internal library allocation/conversion is timed.

`compare.py` uses `before`/`after` for native inspect+decode and
`before_rgba`/`after_rgba` for fresh-handle RGBA operations. `libtiff` and
`libtiff_rgba` refer to the pinned reference; `system_rgba` is the installed
distribution library. Both versions are measured anew and interleaved on the
same host; the historical report's wall times are not used as the baseline.

Disk I/O, process startup, fixture encoding, output dumping, checksum
calculation, and test comparisons are outside the timing window. Native
bilevel reference reads expand packed bits to Gray8; Gray16 reference reads
convert host containers to the Rust API's big-endian samples. Float64 and
LogLuv retain their exact native encodings. There are no palette or planar
native benchmarks in this version. The prepared TIFF used to obtain geometry
is retained outside the timed fresh-open iterations; this harness does not
measure peak memory consumption.

Each child process warms its code/data paths, calibrates a batch, and runs
at least the requested duration (or one complete operation if longer). Seven
independent process batches per engine/case are shuffled reproducibly and
pinned to one available logical CPU. Report medians and interquartile ranges;
the shared virtual host is not an isolated hardware benchmark machine. These
are warm-cache throughput/latency measurements, not cold storage performance.

The Rust and C adapter use release builds, normal safety checks, 256 MiB
input/output limits and a 10^12 work-unit quota. The large scan's RGBA output
exceeds the default 64 MiB, and large WebP's conservative quota exceeds the
default work allowance, hence the explicit larger bounds. No verifier,
proof checking, ghost state or solver runs in these binaries. No decoder or
production adapter source is modified by the timing harness.

The pinned libtiff build uses its normal optimized `-O2` configuration;
Rust uses Cargo release's `opt-level=3`. Consumers are built with GCC `-O3`.
The pinned libjpeg-turbo reference intentionally has SIMD disabled for the
project's numerical tests. The additional distribution-library RGBA column
shows a production library configuration, including its codec implementations.
It also differs in libtiff version; it is not a controlled SIMD-only experiment.

## Workloads and checks

The generated 1024×1024 fixtures use smooth block/gradient patterns, seeded
random bytes, and a synthetic scan pattern. They cover none, PackBits, LZW,
Deflate, JPEG, tiled LZW, Gray16, Float64 with horizontal prediction, Group 3/4,
and the supported single-group lossless WebP subset. LogLuv is 512×512.
JPEG includes smooth blocks and random texture so DC-only blocks do not define
its reported behavior. Four unchanged upstream fixtures include the 24.3 MP
LZW scan and 8.3 MP fax scan. Inputs are not a representative workload survey.
Geometric means are descriptive summaries of this chosen suite only.

The before/after comparison adds four independent 509×513 fixtures covering
WhiteIsZero, straight RGBA, padded uncompressed RGBA tiles and packed bilevel, for
26 native and 24 RGBA workloads. These additional cases exercise optimized
and fallback paths. Their native pixels also match the independent generator.

Native output is compared byte-for-byte between Rust and libtiff before
measurement; RGBA is compared between the three C consumers. A JPEG case may
use a separately recorded maximum difference of three channel values, while
lossless cases require exact equality. Each timed batch's output checksum must
match its corresponding preflight output. The final report states whether
any JPEG tolerance was actually used. Inputs and raw batch results are hashed
and preserved in the evidence; generated input files can be reproduced from
the independent libtiff writer and the existing Python fixture writers.

The recorded optimization report combines 24 initial cases and two supplemental
cases using `merge-comparisons.py`. That helper requires identical sources,
binaries, libraries, affinity and timing settings and rejects duplicate cases;
it preserves the complete metadata of every run. It does not relax the strict
`--resume` check. The initial in-memory libtiff RGBA preflight rejected a valid
768-byte RGB tile with an allocated-buffer-size error; the final tiled fixture
uses RGBA8/1024-byte tiles. The report documents this reference limitation.
The normal reproduction command above measures all 26 cases in one run.

## CPU profiles

On Linux x86-64, profile the same release binaries with the test-only sampler:

```bash
python3 benchmarks/profile.py --output target/profile/before-rgb-jpeg.json target/optimization-baseline/target/release/examples/benchmark target/benchmark/inputs/rgb-jpeg.tif total 3000
python3 benchmarks/profile.py --output target/profile/after-rgb-jpeg.json target/release/examples/benchmark target/benchmark/inputs/rgb-jpeg.tif total 3000
python3 benchmarks/profile.py --output target/profile/libtiff-rgb-jpeg.json target/benchmark/libtiff target/benchmark/inputs/rgb-jpeg.tif total 3000
```

The sampler preloads `sample.c`, requests a 1 ms `ITIMER_PROF` interval and
records interrupted instruction addresses with SIGPROF. Actual delivery is
limited by Linux timer granularity. It is intended only for these single-threaded
consumers. Raw PCs and executable load biases are retained next to JSON summaries;
`nm` resolves function self samples, not inclusive costs or a call graph. Inlined
code is attributed to its containing function. Stripped library interiors may
remain unresolved. Profiles include initialization, preflight, warmup and the
timed batch; profile-run timings are diagnostic, not reported speedups. Use
`compare.py` without `LD_PRELOAD` for controlled performance comparisons.

## v20 throughput comparison

Use `prepare-baseline.py --version 19 --destination target/baseline-v19-restored`
to reconstruct the exact previous core and benchmark example. The reverse
patch removes the new bit-window module and combined-API harness call. The
restorer checks all archived v19 core hashes. Versions 17 and 18 remain
available through the ordered reverse-patch chain.

Build that baseline and the current tree with `scripts/build-benchmarks.sh`.
After the current full proof gate, run:

```bash
python3 benchmarks/compare.py --baseline-root target/baseline-v19-restored   --rounds 7 --milliseconds 80 --output target/throughput-results.json
```

The `combined` engine is additional and appears only for the five Deflate
cases. Its caller buffers are preallocated outside the timer, just like the
existing native pipeline. Metadata parsing, validation, decompression and
output conversion are timed; output dumps and hashes are outside timing.
Its output is compared directly with the ordinary pipeline and libtiff.
Do not fold these opt-in API measurements into the unchanged-API geomean.
The CLI and C ABI measurements still use ordinary inspect-then-decode.

## LERC throughput comparison

`prepare-lerc-throughput-baseline.py` restores the preceding native-FP source
using the archived reverse patch and checks its source hashes before building.
`lerc-throughput.py --rounds 9 --milliseconds 200` checks all 96 outputs and times
seven LERC workloads, including wrappers, float64 and signed/unsigned integers.
Current full-crate proof hashes are required. Run serially without other heavy
work; `reader.c` is unchanged. Caller output allocation is excluded, while
allocations inside the decoder remain timed.
