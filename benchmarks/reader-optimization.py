#!/usr/bin/env python3
"""Compare the archived reader, current verified reader and libtiff 4.7.1.

Uses the unchanged reader.c consumer, all 78 original workloads, eight RGB
strip-height diagnostics, and six default-strip-chopping checks. Before/after
outputs must agree byte-for-byte, including JPEG. Run after the full proof gate
and builds, with no concurrent CPU-heavy work. This script never edits codecs.
"""
import argparse, datetime, hashlib, json, os, pathlib, random, statistics, subprocess

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUT = ROOT / 'target/reader-optimization'
READER = ROOT / 'target/reader-benchmark'

def sha(p):
    return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()

def cmd(args):
    return subprocess.run(list(map(str, args)), capture_output=True, text=True,
                          check=True, timeout=300).stdout

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--rounds', type=int, default=7)
    parser.add_argument('--milliseconds', type=float, default=100)
    parser.add_argument('--resume', action='store_true')
    args = parser.parse_args()
    assert args.rounds >= 2 and args.milliseconds > 0
    proof = json.loads((ROOT / 'target/verification-summary.json').read_text())
    assert proof['success'] and proof['is-verifying-entire-crate'] and proof['errors'] == 0
    for name, digest in proof['core_sources_sha256'].items():
        assert sha(ROOT / name) == digest, name
    receipt = json.loads((OUT / 'before/build-receipt.json').read_text())
    assert sha(OUT / 'before/lib/libtiff.so.6') == receipt['library_sha256']
    assert sha(ROOT / 'benchmarks/reader.c') == receipt['consumer_source_sha256']
    engines = {'before': OUT / 'before/verified', 'after': READER / 'verified',
               'libtiff': READER / 'reference'}
    # Rebuild the baseline consumer with precisely the same flags/source as after.
    flags = ['cc', '-std=c11', '-O3', '-g', '-Wall', '-Wextra', '-Werror']
    cmd(flags + ['-DVERIFIED_ADAPTER', '-I'+str(ROOT/'compat/include'),
                 ROOT/'benchmarks/reader.c', '-L'+str(OUT/'before/lib'),
                 '-Wl,-rpath,'+str(OUT/'before/lib'), '-ltiff', '-o', engines['before']])
    text = (ROOT / 'benchmarks/reader.c').read_text()
    needle = '?"rmOc":"rmc"'
    assert text.count(needle) == 1
    derived = OUT / 'reader-defaults.c'
    derived.write_text(text.replace(needle, '?"rmOc":"rm"'))
    defaults = {e: OUT/(e+'-defaults') for e in engines}
    for e, lib in [('before', OUT/'before/lib'), ('after', ROOT/'target/compat')]:
        cmd(flags + ['-DVERIFIED_ADAPTER', '-I'+str(ROOT/'compat/include'), derived,
                     '-L'+str(lib), '-Wl,-rpath,'+str(lib), '-ltiff', '-o', defaults[e]])
    ref = READER/'tiff-4.7.1'
    cmd([ref/'libtool', '--mode=link', '--tag=CC'] + flags +
        ['-I'+str(ref/'libtiff'), derived, ref/'libtiff/libtiff.la',
         '-Wl,-rpath,'+str(ROOT/'target/lerc-reference'),
         '-Wl,-rpath,'+str(ROOT/'target/zstd-reference/lib'), '-o', defaults['libtiff']])
    cases = json.loads((READER/'inputs.json').read_text())
    work = [(c, m, 'main') for c in cases for m in c['modes']]
    for rows in [16, 32, 64, 256]:
        path = READER/'inputs'/('layout-rgb-deflate-'+str(rows)+'.tif')
        cmd([READER/'encode', path, 8, 8, 3, 0, 0, 1, 1024, 1024, rows, 1, 2, 0, 1])
        c = dict(name='layout-rgb-deflate-'+str(rows), label=f'RGB Deflate, {rows} rows/strip',
                 file=str(path.relative_to(ROOT)), sha256=sha(path), bytes=path.stat().st_size,
                 rows_per_strip=rows, decoded_strip_kib=rows*3)
        work.extend((c, m, 'layout') for m in ['native', 'crop'])
    work.extend((c, m, 'defaults') for c in cases if c['name'].startswith('large-strip-')
                for m in ['firstrow', 'crop'])
    cpu = min(os.sched_getaffinity(0)); os.sched_setaffinity(0, {cpu})
    source = {str(p.relative_to(ROOT)): sha(p) for d in ['src', 'compat']
              for p in sorted((ROOT/d).rglob('*')) if p.is_file() and p.suffix in ['.rs','.c','.h','.inc']}
    meta = dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(), cpu=cpu,
        cpu_model=next(l.split(':',1)[1].strip() for l in pathlib.Path('/proc/cpuinfo').read_text().splitlines() if l.startswith('model name')),
        uname=cmd(['uname','-a']).strip(), cc=cmd(['cc','--version']).splitlines()[0],
        rounds=args.rounds, milliseconds=args.milliseconds, seed=20261018,
        versions={e:cmd([exe,'--version']).strip() for e,exe in engines.items()},
        timing_io='memory callbacks; fresh handle; no mmap; load/allocation/hash excluded',
        rss_io='three separate file-backed processes; process VmHWM including output buffer',
        strip_chopping='disabled except defaults group', jpeg_simd=False,
        proof_sha256=sha(ROOT/'target/verification-summary.json'),
        baseline_receipt=receipt, source_sha256=source,
        benchmark_sha256={str(p.relative_to(ROOT)):sha(p) for p in
                          [ROOT/'benchmarks/reader.c', ROOT/'benchmarks/reader-encode.c', pathlib.Path(__file__)]},
        binary_sha256={e:sha(p) for e,p in engines.items()} | {e+'-defaults':sha(p) for e,p in defaults.items()} |
            {'before-library':sha(OUT/'before/lib/libtiff.so.6'), 'after-library':sha(ROOT/'target/compat/libtiff.so.6'),
             'reference-library':sha(ref/'libtiff/.libs/libtiff.a')},
        codec_library_sha256={str(p.relative_to(ROOT)):sha(p) for p in [ROOT/'.tools/jpeg/lib/libjpeg.a',
             ROOT/'.tools/webp/lib/libwebp.a', ROOT/'target/lerc-reference/libLerc.so', ROOT/'target/zstd-reference/lib/libzstd.so']},
        reference_config_sha256=sha(ref/'libtiff/tif_config.h'),
        reference_linkage=cmd(['ldd',engines['libtiff']]).strip(),
        adapter_linkage=cmd(['ldd',ROOT/'target/compat/libtiff.so.6']).strip(),
        toolchain=json.loads((ROOT/'toolchain-lock.json').read_text()))
    dest = OUT/'results.json'; records = []
    if args.resume and dest.exists():
        previous = json.loads(dest.read_text())
        for key in ['rounds','milliseconds','cpu','source_sha256','benchmark_sha256','binary_sha256','proof_sha256']:
            assert previous['metadata'][key] == meta[key], key
        records = previous['records']; meta = previous['metadata']
    done = {(r['case']['name'],r['mode'],r['group']) for r in records}
    for case, mode, group in work:
        key = (case['name'], mode, group)
        if key in done: continue
        assert sha(ROOT/case['file']) == case['sha256']
        binaries = defaults if group == 'defaults' else engines
        def run(e, ms, io='memory', dump=None):
            return json.loads(cmd([binaries[e],ROOT/case['file'],mode,ms,io]+([dump] if dump else [])))
        checks = {}; raw = {}
        r = dict(case=case,mode=mode,group=group,correctness={},samples={e:[] for e in engines},memory_samples={e:[] for e in engines})
        for e in engines:
            path = OUT/(e+'.bin'); checks[e] = run(e,0,dump=path); raw[e] = path.read_bytes()
            r['correctness'][e] = dict(sha256=sha(path),hash=checks[e]['hash'],bytes=len(raw[e]))
        assert raw['before'] == raw['after'], (key,'before/after output changed')
        assert len(raw['after']) == len(raw['libtiff']), (key,'length')
        if raw['after'] != raw['libtiff']:
            import numpy as np
            diff = abs(np.frombuffer(raw['after'],dtype=np.uint8).astype(np.int16)-np.frombuffer(raw['libtiff'],dtype=np.uint8).astype(np.int16))
            r['difference'] = dict(max_channel_error=int(diff.max()),mean_channel_error=float(diff.mean()))
            assert case.get('jpeg') and int(diff.max()) <= 3, (key,r['difference'])
        else: r['difference'] = dict(max_channel_error=0,mean_channel_error=0)
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
        r['vs_libtiff'] = r['statistics']['after']['median_ms']/r['statistics']['libtiff']['median_ms']
        records.append(r); dest.write_text(json.dumps(dict(metadata=meta,records=records),indent=2)+'\n')
        print(group,case['name'],mode,{e:round(s['median_ms'],4) for e,s in r['statistics'].items()},flush=True)
    print('Completed',len(records),'workloads; all before/after output bytes exact',flush=True)

if __name__ == '__main__': main()
