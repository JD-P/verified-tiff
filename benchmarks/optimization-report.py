#!/usr/bin/env python3
"""Report actual same-host before/after measurements and retain their evidence."""
import hashlib
import json
import math
from pathlib import Path
import shutil
import statistics

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / 'target/optimization/results.json'
data = json.loads(SOURCE.read_text())
meta, cases = data['metadata'], data['cases']
assert len(cases) == 26, 'A complete report requires all 26 workloads'
assert len({c['name'] for c in cases}) == 26
proof = json.loads((ROOT / 'target/verification-summary.json').read_text())
assert proof['success'] and proof['is-verifying-entire-crate'] and proof['errors'] == 0
assert proof['core_sources_sha256'] == meta['after_core_sha256']
for path, sha in meta['after_core_sha256'].items():
    assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == sha
for c in cases:
    assert hashlib.sha256((ROOT / c['file']).read_bytes()).hexdigest() == c['input_sha256']
    assert c['before_after_native_exact']
    assert c['measurements']['before']['hash'] == c['measurements']['after']['hash']
    if c['rgba']:
        assert c['before_after_rgba_exact']
        assert c['measurements']['before_rgba']['hash'] == c['measurements']['after_rgba']['hash']
    for engine, samples in c['samples'].items():
        assert len(samples) == meta['rounds']
        assert all(s['hash'] == c['measurements'][engine]['hash'] for s in samples)
        assert all(s['seconds'] * 1000 >= meta['target_batch_ms'] * .999 for s in samples)
        assert all(v['exact'] or c['jpeg'] and v['max_abs_error'] <= 3 for v in c['correctness'].values())
        assert c['statistics'][engine]['median_ms'] == statistics.median(s['ns_per_image'] / 1e6 for s in samples)


def ms(c, engine):
    return c['statistics'][engine]['median_ms']


def ratio(c, a, b):
    return ms(c, a) / ms(c, b)


def gm(values):
    return math.exp(statistics.mean(math.log(v) for v in values))


def aggregate(group, a, b):
    return gm(ratio(c, a, b) for c in group)


rgba = [c for c in cases if c['rgba']]
original = [c for c in cases if 'expected_sha256' not in c]
original_rgba = [c for c in original if c['rgba']]
batch_count = sum(len(s) for c in cases for s in c['samples'].values())
lines = [
    '# Verified decoder optimization', '',
    f"Measured {meta['timestamp_utc']} on {meta['cpu_model']}, pinned to logical CPU {meta['cpu_affinity'][0]}. "
    f"The previous v17 decoder and the optimized decoder were both freshly compiled with the same pinned toolchain and measured in interleaved batches on this host. "
    'These are new before/after measurements, not a comparison with the historical report’s timings.', '',
    '## Results', '',
    '| Workload set | Native speedup | C API RGBA speedup |',
    '| --- | ---: | ---: |',
    f"| Original 22 workloads (20 RGBA) | {aggregate(original, 'before', 'after'):.2f}× | {aggregate(original_rgba, 'before_rgba', 'after_rgba'):.2f}× |",
    f"| All 26 workloads (24 RGBA) | {aggregate(cases, 'before', 'after'):.2f}× | {aggregate(rgba, 'before_rgba', 'after_rgba'):.2f}× |", '',
    'These are geometric means of per-workload median ratios. They describe this chosen suite; '
    'they are not a claim about an average user workload. Large gains in formerly inefficient '
    'uncompressed paths strongly influence the summary. Individual timings follow.', '',
    f"Across the original 22 workloads, the native decoder’s geometric-mean time relative to pinned libtiff changes from "
    f"{aggregate(original, 'before', 'libtiff'):.2f}× to {aggregate(original, 'after', 'libtiff'):.2f}×. "
    f"The C API’s ratio against system libtiff changes from {aggregate(original_rgba, 'before_rgba', 'system_rgba'):.2f}× "
    f"to {aggregate(original_rgba, 'after_rgba', 'system_rgba'):.2f}×. Ratios above 1 mean slower than libtiff.", '',
    '## What changed', '',
    '- Uncompressed inspection validates segment storage and uses the existing coverage proof to establish every sample address. The redundant per-sample execution loop is removed.',
    '- Uncompressed 8-bit strips use verified contiguous row copies. WhiteIsZero uses a proved inversion loop; alpha keeps the existing normalization pass. Packed bilevel and tiles keep scalar decoding.',
    '- Full-width 8-bit scatter uses a simpler proved index expression, with inline helpers. This benefits basic PackBits/LZW paths without changing stream semantics.', '',
    'Only `src/lib.rs` changed among the 51 verified core files. The other core files and production C adapter are unchanged. '
    'No codec dependency, unsafe core code, external proof body, axiom, or weakened contract is added. '
    'Budget charges remain conservative and unchanged, including insufficient-budget prefix behavior for plain uncompressed decode. '
    'The bulk copy uses safe slices and the existing upstream vstd contract for `copy_from_slice`.', '',
    '## Native inspect plus decode', '',
    f"Milliseconds per image, median of {meta['rounds']} warmed batches. Both native engines include inspection/open, "
    'decode and canonical sample conversion into reusable buffers. “New/reference” is a time ratio; smaller is better.', '',
    '| Workload | Before ms | After ms | Speedup | libtiff ms | New/reference |',
    '| --- | ---: | ---: | ---: | ---: | ---: |',
]
for c in cases:
    lines.append(f"| {c['label']} ({c['width']}×{c['height']}) | {ms(c, 'before'):.4f} | {ms(c, 'after'):.4f} | {ratio(c, 'before', 'after'):.2f}× | {ms(c, 'libtiff'):.4f} | {ratio(c, 'after', 'libtiff'):.2f}× |")
lines += ['', '## C API fresh-handle RGBA', '',
    'The same C consumer opens, renders and closes a new in-memory handle each iteration. Internal allocations and adapter conversion are included; '
    'the decoded-image cache is not reused between iterations. Float64 and native LogLuv are excluded from RGBA.', '',
    '| Workload | Before ms | After ms | Speedup | Pinned libtiff ms | System libtiff ms |',
    '| --- | ---: | ---: | ---: | ---: | ---: |']
for c in rgba:
    lines.append(f"| {c['label']} | {ms(c, 'before_rgba'):.4f} | {ms(c, 'after_rgba'):.4f} | {ratio(c, 'before_rgba', 'after_rgba'):.2f}× | {ms(c, 'libtiff_rgba'):.4f} | {ms(c, 'system_rgba'):.4f} |")
slower = [(c, a, b) for c in cases for a, b in
          [('before', 'after')] + ([('before_rgba', 'after_rgba')] if c['rgba'] else [])
          if ratio(c, b, a) > 1.05]
if slower:
    lines += ['', '### Slower measured cases', '',
        'These median increases exceed 5%. The optimization does not improve every workload.', '',
        '| Workload | Path | Before ms | After ms | Increase |', '| --- | --- | ---: | ---: | ---: |']
    for c, a, b in slower:
        lines.append(f"| {c['label']} | {'C API RGBA' if b.endswith('_rgba') else 'Native'} | {ms(c, a):.4f} | {ms(c, b):.4f} | {(ratio(c, b, a)-1)*100:.1f}% |")
    lines += ['', 'A separate five-batch probe at 160 ms per batch compared the old decoder, '
        'this implementation, and an isolated variant without the two new inline hints. '
        'Removing those hints did not eliminate the random-data LZW C API slowdown, so '
        'that variant was not adopted. The probe is retained in `evidence/optimization-inlining-probe.json`; '
        'its timings are not substituted into the tables or geometric means. '
        'This adapter regression remains a performance issue for future work.']
lines += ['', '## Correctness and verification', '',
    f"All {len(cases)} before/after native outputs and {len(rgba)} before/after RGBA outputs are byte-for-byte identical, including JPEG. "
    'The four added uncompressed fixtures also match independent Python-generated pixels. Lossless comparisons with libtiff are exact.', '']
differences = [(c, e, v) for c in cases for e, v in c['correctness'].items() if not v['exact']]
if differences:
    lines += ['The only non-exact reference comparisons are JPEG:', '',
              '| Workload | Engine | Maximum channel error | Mean absolute error |', '| --- | --- | ---: | ---: |']
    for c, engine, v in differences:
        lines.append(f"| {c['label']} | {engine} | {v['max_abs_error']} | {v['mean_abs_error']:.6f} |")
else:
    lines += ['JPEG comparisons are also exact; no tolerance was needed.']
lines += ['', f"The complete optimized crate passed {proof['verified']} verification obligations with zero errors and the full-crate flag set. "
    'All 33 negative proof checks failed for their intended semantic postconditions, including two new checks that omit the bulk copy or corrupt the shortcut index. '
    'The 32 Rust tests, 87 Python methods, 5,616 generated matrix cases, 31 original TIFF fixtures, 11 adapter test methods, '
    '20,000 core mutations and 2,000 adapter mutations passed. The independently built upstream libtiff suite passed all 108 tests; '
    'that suite tests libtiff itself. See [validation.md](validation.md) for exact scope and retained logs.', '',
    '## Method and limits', '',
    f"There are {batch_count:,} measured batches: {meta['rounds']} per engine/workload, each at least {meta['target_batch_ms']:g} ms or one complete operation if longer. "
    'Engine order is reproducibly shuffled. Each process warms and calibrates before timing, and every batch’s output hash matches its preflight. '
    'Disk I/O, process startup, fixture creation, dumping and output comparison are excluded.', '',
    'The host is a shared VM, not an isolated benchmark machine. Small differences in unchanged paths may reflect scheduling, '
    'code layout or measurement variation. The following intervals show the middle 50% of each native batch distribution; '
    'all samples, minima/maxima and RGBA intervals are retained in the raw JSON.', '',
    '| Workload | Before native Q1–Q3 ms | After native Q1–Q3 ms |', '| --- | ---: | ---: |']
for c in cases:
    a, b = c['statistics']['before'], c['statistics']['after']
    lines.append(f"| {c['label']} | {a['p25_ms']:.4f}–{a['p75_ms']:.4f} | {b['p25_ms']:.4f}–{b['p75_ms']:.4f} |")
lines += ['',
    'Rust uses Cargo release with normal safety checks; consumers use GCC `-O3` and pinned libtiff 4.7.1 uses `-O2`. '
    'Pinned libjpeg-turbo has SIMD disabled for numerical regression tests. The system RGBA reference is distribution libtiff 4.5.1 '
    'and its installed codecs; it is a useful production configuration but not a controlled SIMD-only comparison. '
    'Both Rust versions use the same 256 MiB input/output limits and 10^12 work quota. No solver, proof or ghost state executes in the decoder.', '',
    'The main generated inputs are 1024×1024; native LogLuv is 512×512. Four unchanged upstream regressions include large scans. '
    'The four extra 509×513 cases cover WhiteIsZero, straight RGBA, padded RGBA tiles and bilevel. '
    'This is warm-cache single-thread performance. Cold storage, concurrency, allocations, peak memory, and representative real-world frequencies are not measured. '
    'There are no native palette/planar benchmarks in this suite.', '',
    'Remaining work includes tiled row copies, bilevel expansion, wider-sample paths, individual codec loops and adapter inspection/allocation overhead. '
    'The measured improvement does not establish general parity with libtiff, or full libtiff API/profile compatibility. '
    'The C adapter remains outside the formal proof boundary.', '',
    'The initial padded RGB8 tile preflight failed in the pinned libtiff in-memory RGBA reader '
    'with “Expected 768, got 1024”, although native decoding matched the independent pixels. '
    'The measured tiled case therefore uses RGBA8 with 1024-byte tiles. This is an explicit '
    'reference limitation; the decoder’s existing RGB tile correctness tests remain passing.', '',
    '## Reproduce', '',
    'Follow [benchmarks/README.md](../benchmarks/README.md). The baseline helper restores the exact previous core from the shipped reverse patch '
    'and checks every file against its archived proof hashes. The comparison requires a fresh full-crate proof summary for the optimized source. '
    'Raw results are in [optimization-results.json](evidence/optimization-results.json), including compiler, library, source, binary and input hashes.', '',
    f"Compiler: `{meta['compiler']}`. Rust: `{meta['rustc']}`.", '', '```', meta['distribution_packages'].strip(), '```', '']
if 'measurement_runs' in meta:
    lines += ['The 24 initial workloads and two supplemental workloads were measured in '
        'separate runs on this host. `merge-comparisons.py` required identical core/adapter '
        'sources, engine binaries, libraries, CPU affinity and measurement settings, and '
        'rejected duplicate cases. Every run’s complete metadata and its case list are '
        'retained under `measurement_runs` in the JSON. The original harness is preserved '
        'as `evidence/optimization-initial-harness.py`; the supplemental harness changes '
        'the tiled fixture and adds an output-path option. No samples were selected or discarded '
        'to improve the reported ratios, and the strict single-run resume guard remains intact.', '']
(ROOT / 'docs/optimization.md').write_text('\n'.join(lines))
shutil.copyfile(SOURCE, ROOT / 'docs/evidence/optimization-results.json')
print(f'Reported {len(cases)} workloads / {batch_count} measured batches')
