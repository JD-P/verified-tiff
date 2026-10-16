#!/usr/bin/env python3
"""Interleave unchanged/optimized decoder and real-libtiff measurements.

Build both projects with scripts/build-benchmarks.sh, then pass the unchanged
checkout's root through --baseline-root. Uses the same in-process timers as
run.py. Before/after pixels must match exactly, including JPEG.
"""
import argparse
import datetime
import hashlib
import json
import os
import pathlib
import random
import statistics
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUT = ROOT / 'target/optimization'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(args):
    p = subprocess.run(list(map(str, args)), capture_output=True, text=True, timeout=300)
    if p.returncode:
        raise RuntimeError(f'{args}: {p.returncode}\n{p.stderr[-4000:]}')
    return json.loads(p.stdout)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline-root', type=pathlib.Path, required=True)
    parser.add_argument('--rounds', type=int, default=7)
    parser.add_argument('--milliseconds', type=float, default=80)
    parser.add_argument('--resume', action='store_true')
    parser.add_argument('--case')
    parser.add_argument('--output', type=pathlib.Path, default=OUT / 'results.json')
    args = parser.parse_args()
    assert args.rounds >= 3 and args.milliseconds > 0
    baseline = args.baseline_root.resolve()
    OUT.mkdir(exist_ok=True)
    cases = json.loads((ROOT / 'target/benchmark/inputs.json').read_text())
    sys.path.insert(0, str(ROOT / 'tests'))
    from fixtures import make_tiff
    # These cases exercise inversion, alpha conversion and the unchanged
    # padded-tile/bilevel fallbacks as well as the primary benchmark suite.
    extras = [
        ('white-none', 'WhiteIsZero Gray8 / none', dict(white=True)),
        ('rgba-none', 'RGBA8 straight alpha / none', dict(channels=4, alpha=2)),
        # The pinned libtiff's non-mapped RGBA reader rejects a valid 768-byte
        # RGB tile (reported raw buffer size 1024). RGBA tiles are 1024 bytes.
        ('rgba-tile-none', 'RGBA8 / uncompressed padded tiles', dict(channels=4, alpha=2, tiled=True)),
        ('bilevel-none', 'Bilevel / none', dict(bits=1)),
    ]
    for name, label, options in extras:
        data, expected = make_tiff(509, 513, **options)
        path = OUT / (name + '.tif')
        path.write_bytes(data)
        cases.append(dict(name=name, label=label, file=str(path.relative_to(ROOT)),
                          rgba=True, jpeg=False, origin='independent Python fixture',
                          input_bytes=len(data), input_sha256=digest(path),
                          expected_sha256=hashlib.sha256(expected[0]).hexdigest()))
    if args.case:
        cases = [c for c in cases if args.case in c['name']]
    engines = {
        'before': (baseline / 'target/release/examples/benchmark', 'total'),
        'after': (ROOT / 'target/release/examples/benchmark', 'total'),
        'combined': (ROOT / 'target/release/examples/benchmark', 'combined'),
        'libtiff': (ROOT / 'target/benchmark/libtiff', 'total'),
        'before_rgba': (baseline / 'target/benchmark/adapter', 'rgba'),
        'after_rgba': (ROOT / 'target/benchmark/adapter', 'rgba'),
        'libtiff_rgba': (ROOT / 'target/benchmark/libtiff', 'rgba'),
        'system_rgba': (ROOT / 'target/benchmark/system', 'rgba'),
    }
    cpu = min(os.sched_getaffinity(0))
    os.sched_setaffinity(0, {cpu})
    proof = json.loads((ROOT / 'target/verification-summary.json').read_text())
    assert proof['success'] and proof['is-verifying-entire-crate'] and proof['errors'] == 0
    for path, sha in proof['core_sources_sha256'].items():
        assert digest(ROOT / path) == sha
    old_proof = json.loads((baseline / 'docs/evidence/verification-summary.json').read_text())
    assert old_proof['success'] and old_proof['is-verifying-entire-crate'] and old_proof['errors'] == 0
    for path, sha in old_proof['core_sources_sha256'].items():
        assert digest(baseline / path) == sha
    compat = json.loads((ROOT / 'docs/evidence/c-api-source-sha256.json').read_text())
    old_compat = json.loads((baseline / 'docs/evidence/c-api-source-sha256.json').read_text())
    for tree, sources in [(ROOT, compat), (baseline, old_compat)]:
        for path, sha in sources.items():
            assert digest(tree / path) == sha, path
    dependencies = subprocess.check_output(['ldd', ROOT / 'target/benchmark/system'], text=True)
    dependency_hashes = {}
    for line in dependencies.splitlines():
        parts = line.split()
        if '=>' in parts and len(parts) > 2 and pathlib.Path(parts[2]).is_file():
            dependency_hashes[parts[2]] = digest(pathlib.Path(parts[2]))
    meta = dict(
        timestamp_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
        cpu_affinity=[cpu],
        cpu_model=next(s.split(':', 1)[1].strip() for s in pathlib.Path('/proc/cpuinfo').read_text().splitlines() if s.startswith('model name')),
        uname=subprocess.check_output(['uname', '-a'], text=True).strip(),
        rounds=args.rounds, target_batch_ms=args.milliseconds, shuffle_seed=20261014,
        input_output_limit_bytes=268435456, work_budget=1000000000000,
        compiler=subprocess.check_output(['cc', '--version'], text=True).splitlines()[0],
        rustc=subprocess.check_output(['bash', '-c', 'source scripts/env.sh; rustc --version'], cwd=ROOT, text=True).strip(),
        build=dict(rust='Cargo release opt-level=3', consumers='GCC -O3', pinned_libtiff='-O2', pinned_jpeg_simd=False),
        before_verified=old_proof['verified'], after_verified=proof['verified'],
        before_core_sha256=old_proof['core_sources_sha256'], after_core_sha256=proof['core_sources_sha256'],
        before_compat_sha256=old_compat, after_compat_sha256=compat,
        adapter_sources_identical=(old_compat == compat),
        binaries={e: dict(path=str(p), sha256=digest(p), mode=mode) for e, (p, mode) in engines.items()},
        harness_sha256={str(p.relative_to(ROOT)): digest(p) for p in [ROOT / 'benchmarks/compare.py', ROOT / 'benchmarks/run.py', ROOT / 'benchmarks/libtiff.c', ROOT / 'benchmarks/encode.c', ROOT / 'examples/benchmark.rs', ROOT / 'scripts/build-benchmarks.sh']},
        pinned_libtiff=subprocess.check_output([ROOT / 'target/benchmark/libtiff', '--version'], text=True).strip(),
        system_libtiff=subprocess.check_output([ROOT / 'target/benchmark/system', '--version'], text=True).strip(),
        system_dependencies=dependencies, dependency_sha256=dependency_hashes,
        distribution_packages=subprocess.check_output(['dpkg-query', '-W', 'libtiff6', 'libjpeg-turbo8', 'libwebp7', 'libdeflate0', 'zlib1g'], text=True),
        pinned_static_library_sha256=digest(ROOT / '.tools/tiff-4.7.1/libtiff/.libs/libtiff.a'),
    )
    records = []
    result_path = args.output.resolve()
    result_path.parent.mkdir(parents=True, exist_ok=True)
    if args.resume and result_path.exists():
        old = json.loads(result_path.read_text())
        for key in ['rounds', 'target_batch_ms', 'cpu_affinity', 'binaries', 'before_core_sha256', 'after_core_sha256', 'harness_sha256']:
            assert old['metadata'][key] == meta[key], key
        records = old['cases']
        meta = old['metadata']
    for case in cases:
        if any(c['name'] == case['name'] for c in records):
            continue
        path = ROOT / case['file']
        assert digest(path) == case['input_sha256']
        active = [e for e in engines if (case['rgba'] or not e.endswith('_rgba')) and (e!='combined' or 'deflate' in case['name'])]
        record = dict(case, measurements={}, correctness={}, samples={}, statistics={})
        outputs = {}
        for e in active:
            exe, mode = engines[e]
            dump = OUT / (e + '.raw')
            m = run([exe, path, mode, 0, dump])
            outputs[e] = dump.read_bytes()
            record['measurements'][e] = m
        assert outputs['before'] == outputs['after'], (case['name'], 'changed native pixels')
        record['before_after_native_exact'] = True
        if 'combined' in outputs:
            assert outputs['combined']==outputs['after'], (case['name'],'changed combined pixels')
            record['combined_native_exact']=True
        if 'expected_sha256' in case:
            assert digest(OUT / 'after.raw') == case['expected_sha256']
        if case['rgba']:
            assert outputs['before_rgba'] == outputs['after_rgba'], (case['name'], 'changed RGBA pixels')
            record['before_after_rgba_exact'] = True
        for e in active:
            reference = outputs['libtiff_rgba' if e.endswith('_rgba') else 'libtiff']
            data = outputs[e]
            assert len(data) == len(reference), (case['name'], e, 'length')
            if data == reference:
                check = dict(exact=True, max_abs_error=0)
            else:
                import numpy as np
                diff = np.abs(np.frombuffer(data, np.uint8).astype(np.int16) - np.frombuffer(reference, np.uint8).astype(np.int16))
                check = dict(exact=False, max_abs_error=int(diff.max()), mean_abs_error=float(diff.mean()))
                assert case['jpeg'] and check['max_abs_error'] <= 3, (case['name'], e, check)
            record['correctness'][e] = check
        record['width'] = record['measurements']['after']['width']
        record['height'] = record['measurements']['after']['height']
        rng = random.Random(20261014)
        for _ in range(args.rounds):
            order = active.copy()
            rng.shuffle(order)
            for e in order:
                exe, mode = engines[e]
                m = run([exe, path, mode, args.milliseconds])
                assert m['hash'] == record['measurements'][e]['hash']
                assert m['seconds'] * 1000 >= args.milliseconds * .999
                record['samples'].setdefault(e, []).append(m)
        for e, samples in record['samples'].items():
            values = [s['ns_per_image'] / 1e6 for s in samples]
            q = statistics.quantiles(values, n=4, method='inclusive')
            record['statistics'][e] = dict(median_ms=statistics.median(values), min_ms=min(values), max_ms=max(values), p25_ms=q[0], p75_ms=q[2])
        records.append(record)
        result_path.write_text(json.dumps(dict(metadata=meta, cases=records), indent=2) + '\n')
        stats = record['statistics']
        print(case['name'], {e: round(s['median_ms'], 4) for e, s in stats.items()}, 'speedup', round(stats['before']['median_ms'] / stats['after']['median_ms'], 2), flush=True)


if __name__ == '__main__':
    main()
