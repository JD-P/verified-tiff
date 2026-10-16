#!/usr/bin/env python3
"""Render the v18-to-v19 profiling/benchmark evidence as a repository report."""
import argparse
import json
import math
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def geometric(values):
    return math.exp(sum(math.log(v) for v in values) / len(values))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--results', type=Path, default=ROOT / 'target/profile-results.json')
    parser.add_argument('--profiles', type=Path, default=ROOT / 'target/profile')
    parser.add_argument('--output', type=Path, default=ROOT / 'docs/profiling.md')
    args = parser.parse_args()
    result = json.loads(args.results.read_text())
    cases, meta = result['cases'], result['metadata']
    resume = args.results.with_name('profile-resume.json')
    resume_note = ''
    if resume.exists():
        count = json.loads(resume.read_text())['completed_cases_at_restart']
        resume_note = f'The sweep resumed after {count} complete cases, with identical binaries, source hashes, settings and CPU affinity required by the harness.'
    assert len(cases) == 26, 'report requires the complete case set'
    assert all(c['before_after_native_exact'] for c in cases)
    assert all(c.get('before_after_rgba_exact', True) for c in cases)
    native = geometric([c['statistics']['before']['median_ms'] / c['statistics']['after']['median_ms'] for c in cases])
    rgba = [c for c in cases if 'after_rgba' in c['statistics']]
    adapter = geometric([c['statistics']['before_rgba']['median_ms'] / c['statistics']['after_rgba']['median_ms'] for c in rgba])
    gap = geometric([c['statistics']['after']['median_ms'] / c['statistics']['libtiff']['median_ms'] for c in cases])
    text = [f'''# CPU profiling and optimization against libtiff 4.7.1

Measured {meta['timestamp_utc']} on {meta['cpu_model']}, pinned to logical CPU {meta['cpu_affinity'][0]}.
The before version is the preserved v18 core; the after version is this delivery.
Both were freshly built and measured on this host. The pinned native reference
is libtiff 4.7.1 with zlib and scalar libjpeg-turbo 3.1.1 (SIMD disabled),
as in the existing numerical test setup. This is not a claim about every
distribution's libtiff build. The separately reported system RGBA reference
is identified in the raw metadata.

Across all {len(cases)} native cases, the geometric-mean speedup is **{native:.2f}×**;
across {len(rgba)} C API cases it is **{adapter:.2f}×**. The native implementation
remains **{gap:.2f}× slower** than this pinned libtiff build by geometric mean.
Each case has equal weight in these summaries; this is not an application workload mix.

## Findings and changes

1. JPEG IDCT rebuilt a 64-entry constant array inside each inner-loop lookup.
   Baseline disassembly shows the repeated stack stores; the smooth JPEG profile
   attributes 88.7% of CPU samples to the containing `decode_blocks` function.
   Borrowing immutable literal arrays allows constant promotion. The IDCT's exact
   arithmetic, rounding and clipping model is unchanged. Zig-zag and other small
   lookup tables receive the same treatment.
2. Textured JPEG exposed entropy-bit overhead after the IDCT fix. The bit helper
   now inlines and uses the existing proved byte divisor, removing repeated
   state copies and enabling simpler bit extraction. Geometry lookup borrows its
   32-word descriptor instead of taking it by value.
3. Deflate copied Huffman tables and linearly searched up to 288 code lengths
   for every symbol. Tables are now borrowed and carry a fixed ordered-symbol
   cache, built once per table. Its output is proved equal to the original pure
   `ranked` definition. Length/distance constant arrays are borrowed too.
4. Stored Deflate blocks previously consumed eight loop iterations per byte.
   Aligned eight-bit reads now load the byte directly, proved equivalent to the
   unchanged bit-field model. Checksum, bounds, partial-read behavior and framing
   rules remain in force.
5. The full first sweep exposed a LogLuv regression from 7.9 to 21.5 ms.
   Its profile attributed 57.2% of samples to an out-of-line `scatter_byte` call;
   the baseline had inlined this helper. An `inline(always)` hint restores that
   specialization without changing its body or contracts. The final full sweep
   below includes this fix. The superseded sweep is retained as
   `evidence/profile-first-pass-results.json`.

The Deflate cache adds 704 bytes of declared fields per Huffman table on this
64-bit target, two persistent tables per cursor. Construction scans 288 lengths
for each of 16 buckets. Exact compiler stack usage is not proved. There is no
core heap allocation, unsafe decoder, runtime codec dependency or new axiom.

## Native inspect plus decode

Seven shuffled process batches per engine/case, each at least {meta['target_batch_ms']} ms.
Warm-cache medians below exclude disk I/O, startup, fixture generation and output
checking. The shared virtual host is not isolated benchmark hardware. Raw
per-batch results and quartiles are in `evidence/profile-results.json`.
{resume_note}

| Case | Before ms | After ms | Speedup | 4.7.1 ms | After / 4.7.1 |
| --- | ---: | ---: | ---: | ---: | ---: |''']
    for c in cases:
        s = c['statistics']
        b, a, ref = [s[e]['median_ms'] for e in ('before', 'after', 'libtiff')]
        text.append(f"| {c['name']} | {b:.3f} | {a:.3f} | {b/a:.2f}× | {ref:.3f} | {a/ref:.2f}× |")
    text += ['''
## C API fresh-handle RGBA

The same C consumer opens, renders and closes an in-memory TIFF on every iteration.
Adapter allocation, conversion and inspection are included. The adapter itself
was not changed and remains outside the formal proof.

| Case | Before ms | After ms | Speedup | 4.7.1 ms | System ms |
| --- | ---: | ---: | ---: | ---: | ---: |''']
    for c in rgba:
        s = c['statistics']
        b, a, ref, system = [s[e]['median_ms'] for e in ('before_rgba', 'after_rgba', 'libtiff_rgba', 'system_rgba')]
        text.append(f"| {c['name']} | {b:.3f} | {a:.3f} | {b/a:.2f}× | {ref:.3f} | {system:.3f} |")
    text += ['''
## CPU profiles

Flat SIGPROF samples of the actual release binaries, requesting a 1 ms process
CPU timer. Kernel timer granularity limits delivery; this is not a call graph.
Samples include startup, preflight and warmup. Inlined functions appear under
their containing symbol, and some stripped runtime-library symbols remain
unresolved. All retained profiles have zero dropped samples. Profile-run timings
are diagnostic only; the tables above use separate runs without the sampler.
Raw PCs, load biases, sample counts and executable hashes are retained under
`evidence/profile/`. Disassembly corroborates the repeated constants and copies.

| Workload / version | Samples | Three largest function self-sample shares |
| --- | ---: | --- |''']
    for name in ('rgb-jpeg', 'rgb-jpeg-textured', 'gray16-deflate', 'noise-deflate'):
        for phase in ('before', 'after', 'libtiff'):
            p = json.loads((args.profiles / f'{phase}-{name}.json').read_text())
            top = '; '.join(f"`{f['function'].split('::',1)[1]}` {f['percent']:.1f}%" for f in p['functions'][:3])
            text.append(f"| {name} / {phase} | {p['samples']} | {top} |")
    regressions = []
    for c in cases:
        for before, after, view in [('before', 'after', 'Native'), ('before_rgba', 'after_rgba', 'C API')]:
            stats = c['statistics']
            if after in stats:
                b, a = stats[before]['median_ms'], stats[after]['median_ms']
                if a > 1.05*b:
                    regressions.append(f"| {c['name']} | {view} | {b:.3f} | {a:.3f} | +{100*(a/b-1):.1f}% |")
    text += ['''
## Measured tradeoffs

Cases more than 5% slower in this sweep are listed explicitly. Tiled RGB LZW
regresses with the stronger shared scatter inlining hint, while LogLuv returns
to baseline speed and several other paths improve. The native LZW tile quartiles
do not overlap in this run. This is a measured cost of the delivered choice,
not a claim of uniform improvement. A future change should specialize the
LogLuv/scatter call sites without penalizing tiled LZW, then remeasure all paths.

| Case | Interface | Before ms | After ms | Change |
| --- | --- | ---: | ---: | ---: |''', *regressions]
    text += ['''
## Validation and remaining work

The complete crate verifies 924 obligations with zero errors and all 35 negative
gates reject their mutations. The full 32 Rust / 87 Python test methods, generated
matrices, 31 original TIFF corpus files, 11 adapter methods, 2,000 adapter
mutations and 20,000 core mutations pass. The separately built libtiff passes its
own 108 tests. Those upstream tests test libtiff, not this library.

Every benchmark's before/after output is byte-for-byte identical, including JPEG;
lossless cases match libtiff exactly and JPEG comparisons stay within the existing
maximum channel error of 3. No acceptance policy, mathematical pixel model,
resource quota or output frame was relaxed. The C boundary did not change;
earlier sanitizer evidence is explicitly historical.

The remaining profiles point to JPEG Huffman decoding and MCU-to-image addressing,
Deflate bit decoding/state transitions, and LZW bit reads (49.1% of baseline
random-LZW C API samples). Tiled uncompressed data and WebP still have large
relative gaps. These remain future optimization targets, not completed work.
See every row above for regressions as well as improvements; small changes on
unmodified paths can reflect code layout and host variability.

Reproduction commands are in `../benchmarks/README.md`. The reverse patch restores
all 51 v18 core files with archived hashes; the v17 baseline remains reconstructible.
`evidence/profile-baseline-verification.json` is the historical v18 proof summary;
`evidence/verification-summary.json` and `proof-gate.log` describe this source.
''']
    args.output.write_text('\n'.join(text))
    print(args.output)


if __name__ == '__main__':
    main()
