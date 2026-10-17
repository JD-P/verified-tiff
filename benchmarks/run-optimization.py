#!/usr/bin/env python3
"""Compare the pre-pass reader, each intermediate commit, the current reader and libtiff 4.7.1.

Uses the unchanged reader.c consumer, all 78 original workloads and six extra
WebP fixtures (lossy/lossless; opaque, alpha texture, odd size). Every verified
engine's output must agree byte-for-byte with the pre-pass reader, including
JPEG. Differences from libtiff are recorded, not hidden. Run after the full
proof gate and builds, with no concurrent CPU-heavy work. This script never
edits codecs.
"""
import argparse, datetime, hashlib, json, os, pathlib, random, statistics, subprocess

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUT = ROOT / 'target/run-optimization'
READER = ROOT / 'target/reader-benchmark'

def sha(p):
    return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()

def cmd(args):
    return subprocess.run(list(map(str, args)), capture_output=True, text=True,
                          check=True, timeout=900).stdout

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--rounds', type=int, default=7)
    parser.add_argument('--milliseconds', type=float, default=100)
    parser.add_argument('--resume', action='store_true')
    parser.add_argument('--steps', default='before', help='restored engines in commit order, starting with before')
    args = parser.parse_args()
    steps = args.steps.split(',');assert steps[0] == 'before' and 'after' not in steps
    assert args.rounds >= 2 and args.milliseconds > 0
    proof = json.loads((ROOT / 'target/verification-summary.json').read_text())
    assert proof['success'] and proof['is-verifying-entire-crate'] and proof['errors'] == 0
    for name, digest in proof['core_sources_sha256'].items():
        assert sha(ROOT / name) == digest, name
    receipts = {}
    for e in steps:
        receipts[e] = json.loads((OUT / e / 'build-receipt.json').read_text())
        assert sha(OUT / e / 'lib/libtiff.so.6') == receipts[e]['library_sha256'], e
        assert sha(ROOT / 'benchmarks/reader.c') == receipts[e]['consumer_source_sha256'], e
    chain = steps + ['after']
    engines = {e: OUT / e / 'verified' for e in steps} | {'after': READER / 'verified', 'libtiff': READER / 'reference'}
    flags = ['cc', '-std=c11', '-O3', '-g', '-Wall', '-Wextra', '-Werror']
    for e, lib in [(e, OUT/e/'lib') for e in steps] + [('after', ROOT/'target/compat')]:
        cmd(flags + ['-DVERIFIED_ADAPTER', '-I'+str(ROOT/'compat/include'), ROOT/'benchmarks/reader.c',
                     '-L'+str(lib), '-Wl,-rpath,'+str(lib), '-ltiff', '-o', engines[e]])
    cases = json.loads((READER/'inputs.json').read_text())
    for lossless in [0, 1]:
        for suffix, ch, alpha, texture, w, h, rows in [('opaque',3,0,0,1024,1024,64),('texture',4,1,1,1024,1024,64),('odd',4,0,0,257,259,259)]:
            name = 'webp-'+('lossless' if lossless else 'lossy')+'-'+suffix
            path = OUT/(name+'.tif')
            cmd([READER/'encode', path, 50001, 8, ch, alpha, texture, 1, w, h, rows, 1, 2, 0, lossless])
            cases.append(dict(name=name, label=f"WebP {'lossless' if lossless else 'lossy'} {suffix}",
                              file=str(path.relative_to(ROOT)), sha256=sha(path), bytes=path.stat().st_size,
                              modes=['native','rgba','crop'], jpeg=False))
    work = [(c, m) for c in cases for m in c['modes']]
    cpu = min(os.sched_getaffinity(0)); os.sched_setaffinity(0, {cpu})
    source = {str(p.relative_to(ROOT)): sha(p) for d in ['src', 'compat', 'trusted-float', 'trusted-simd']
              for p in sorted((ROOT/d).rglob('*')) if p.is_file() and p.suffix in ['.rs','.c','.h','.inc']}
    ref = READER/'tiff-4.7.1'
    meta = dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(), cpu=cpu,
        cpu_model=next(l.split(':',1)[1].strip() for l in pathlib.Path('/proc/cpuinfo').read_text().splitlines() if l.startswith('model name')),
        uname=cmd(['uname','-a']).strip(), cc=cmd(['cc','--version']).splitlines()[0],
        rounds=args.rounds, milliseconds=args.milliseconds, seed=20261016,
        versions={e:cmd([exe,'--version']).strip() for e,exe in engines.items()},
        timing_io='memory callbacks; fresh handle; no mmap; input loading/caller output allocation/hash excluded; decoder allocations included',
        rss_io='three separate file-backed processes; process VmHWM including output buffer',
        jpeg_simd=False, proof_sha256=sha(ROOT/'target/verification-summary.json'),
        chain=chain, step_receipts=receipts, source_sha256=source,
        benchmark_sha256={str(p.relative_to(ROOT)):sha(p) for p in
                          [ROOT/'benchmarks/reader.c', ROOT/'benchmarks/reader-encode.c', pathlib.Path(__file__)]},
        binary_sha256={e:sha(p) for e,p in engines.items()} | {e+'-library':sha(OUT/e/'lib/libtiff.so.6') for e in steps} |
            {'after-library':sha(ROOT/'target/compat/libtiff.so.6'),
             'reference-library':sha(ref/'libtiff/.libs/libtiff.a')},
        reference_linkage=cmd(['ldd',engines['libtiff']]).strip(),
        adapter_linkage=cmd(['ldd',ROOT/'target/compat/libtiff.so.6']).strip(),
        toolchain=json.loads((ROOT/'toolchain-lock.json').read_text()))
    dest = OUT/'results.json'; records = []
    if args.resume and dest.exists():
        previous = json.loads(dest.read_text())
        for key in ['rounds','milliseconds','cpu','source_sha256','benchmark_sha256','binary_sha256','proof_sha256']:
            assert previous['metadata'][key] == meta[key], key
        records = previous['records']; meta = previous['metadata']
    done = {(r['case']['name'],r['mode']) for r in records}
    for case, mode in work:
        key = (case['name'], mode)
        if key in done: continue
        assert sha(ROOT/case['file']) == case['sha256']
        def run(e, ms, io='memory', dump=None):
            return json.loads(cmd([engines[e],ROOT/case['file'],mode,ms,io]+([dump] if dump else [])))
        checks = {}; raw = {}
        r = dict(case=case,mode=mode,correctness={},samples={e:[] for e in engines},memory_samples={e:[] for e in engines})
        for e in engines:
            path = OUT/(e+'.bin'); checks[e] = run(e,0,dump=path); raw[e] = path.read_bytes()
            r['correctness'][e] = dict(sha256=sha(path),hash=checks[e]['hash'],bytes=len(raw[e]))
        for e in chain: assert raw[e] == raw['before'], (key, e, 'output changed')
        if raw['after'] == raw['libtiff']:
            r['difference'] = dict(exact=True,max_channel_error=0)
        elif len(raw['after']) == len(raw['libtiff']):
            r['difference'] = dict(exact=False,max_channel_error=max(abs(a-b) for a,b in zip(raw['after'],raw['libtiff'])))
        else:
            r['difference'] = dict(exact=False,length_after=len(raw['after']),length_libtiff=len(raw['libtiff']))
        # Per-workload seed keeps order reproducible even after resuming.
        rng = random.Random(str(meta['seed'])+repr(key))
        for _ in range(args.rounds):
            order = list(engines); rng.shuffle(order)
            for e in order:
                m = run(e,args.milliseconds); assert m['hash'] == checks[e]['hash']; r['samples'][e].append(m)
        for _ in range(3):
            order = list(engines); rng.shuffle(order)
            for e in order:
                m = run(e,0,'file'); assert m['hash'] == checks[e]['hash']; r['memory_samples'][e].append(m)
        r['statistics'] = {}
        for e in engines:
            x = [m['ns_per_operation']/1e6 for m in r['samples'][e]]
            q = statistics.quantiles(x,n=4,method='inclusive')
            rss = [m['peak_rss_kib']/1024 for m in r['memory_samples'][e]]
            r['statistics'][e] = dict(median_ms=statistics.median(x),q1_ms=q[0],q3_ms=q[2],min_ms=min(x),max_ms=max(x),
                                     median_rss_mib=statistics.median(rss),min_rss_mib=min(rss),max_rss_mib=max(rss))
        r['speedup'] = r['statistics']['before']['median_ms']/r['statistics']['after']['median_ms']
        r['step_speedup'] = {b: r['statistics'][a]['median_ms']/r['statistics'][b]['median_ms'] for a, b in zip(chain, chain[1:])}
        r['vs_libtiff'] = r['statistics']['after']['median_ms']/r['statistics']['libtiff']['median_ms']
        records.append(r); dest.write_text(json.dumps(dict(metadata=meta,records=records),indent=2)+'\n')
        print(case['name'],mode,{e:round(s['median_ms'],4) for e,s in r['statistics'].items()},f"x{r['speedup']:.2f}",flush=True)
    print('Completed',len(records),'workloads; every verified engine output byte exact',flush=True)

if __name__ == '__main__': main()
