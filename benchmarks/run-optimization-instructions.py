#!/usr/bin/env python3
"""Count instructions (callgrind Ir, one decode) for every commit on the workloads it targets.

Counts are deterministic for these engines, so they separate each commit's own
effect from this host's code-layout and run-to-run timing noise. The jump-padding
commit only adds alignment padding and is not counted. Run after
run-optimization.py; writes target/run-optimization/instructions.json.
"""
import hashlib, importlib.util, json, pathlib, re, subprocess
from concurrent.futures import ThreadPoolExecutor

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUT = ROOT / 'target/run-optimization'
spec = importlib.util.spec_from_file_location('report', ROOT/'benchmarks/run-optimization-report.py')
report = importlib.util.module_from_spec(spec); spec.loader.exec_module(report)

def sha(p):
    return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()

def engine(e):
    return ROOT/'target/reader-benchmark/verified' if e == 'after' else OUT/e/'verified'

def count(job):
    e, file, mode = job
    run = subprocess.run(['valgrind', '--tool=callgrind', '--callgrind-out-file=/dev/null', str(engine(e)),
                          str(ROOT/file), mode, '0', 'memory'], capture_output=True, text=True, check=True, timeout=1800)
    return '|'.join(job), int(re.search(r'Collected : (\d+)', run.stderr).group(1))

def main():
    results = json.loads((OUT/'results.json').read_text())
    chain = results['metadata']['chain']; jobs = set()
    for a, b in zip(chain, chain[1:]):
        if b == 'jcc': continue
        for r in results['records']:
            if report.TARGETS[b](r):
                assert sha(ROOT/r['case']['file']) == r['case']['sha256']
                jobs |= {(a, r['case']['file'], r['mode']), (b, r['case']['file'], r['mode'])}
    # Counts do not depend on CPU contention, so four run at once.
    with ThreadPoolExecutor(4) as pool:
        counts = dict(pool.map(count, sorted(jobs)))
    meta = dict(tool=subprocess.run(['valgrind', '--version'], capture_output=True, text=True).stdout.strip(),
                results_sha256=sha(OUT/'results.json'),
                binary_sha256={e: sha(engine(e)) for e in sorted({j[0] for j in jobs})})
    (OUT/'instructions.json').write_text(json.dumps(dict(metadata=meta, counts=dict(sorted(counts.items()))), indent=1)+'\n')
    print('Counted', len(counts), 'engine/workload pairs')

if __name__ == '__main__': main()
