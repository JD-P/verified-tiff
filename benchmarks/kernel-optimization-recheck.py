#!/usr/bin/env python3
"""Longer independent batches for >10% median slowdowns in the main run.

Retains every original measurement; does not replace or cherry-pick samples.
Run after kernel-optimization.py, without concurrent CPU-heavy work.
"""
import datetime,hashlib,json,os,pathlib,random,statistics,subprocess
ROOT=pathlib.Path(__file__).resolve().parents[1];OUT=ROOT/'target/kernel-optimization'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    data=json.loads((OUT/'results.json').read_text());meta=data['metadata'];assert len(data['records'])==92
    os.sched_setaffinity(0,{meta['cpu']})
    for n,h in meta['source_sha256'].items():assert sha(ROOT/n)==h,n
    for e,p in [('before-library',OUT/'before/lib/libtiff.so.6'),('after-library',ROOT/'target/compat/libtiff.so.6')]:assert sha(p)==meta['binary_sha256'][e]
    records=[]
    for r in data['records']:
        if r['speedup']>=1/1.1:continue
        engines={e:(OUT/(e+'-defaults') if r['group']=='defaults' else OUT/'before/verified' if e=='before' else ROOT/'target/reader-benchmark/verified') for e in ['before','after']}
        for e,p in engines.items():assert sha(p)==meta['binary_sha256'][e+('-defaults' if r['group']=='defaults' else '')]
        assert sha(ROOT/r['case']['file'])==r['case']['sha256']
        samples={e:[] for e in engines};rng=random.Random(20261021+len(records))
        for _ in range(9):
            order=list(engines);rng.shuffle(order)
            for e in order:
                q=subprocess.run([str(engines[e]),str(ROOT/r['case']['file']),r['mode'],'500','memory'],capture_output=True,text=True,check=True,timeout=300)
                m=json.loads(q.stdout);assert m['hash']==r['correctness'][e]['hash'];samples[e].append(m)
        stats={}
        for e in engines:
            x=[m['ns_per_operation']/1e6 for m in samples[e]];q=statistics.quantiles(x,n=4,method='inclusive')
            stats[e]=dict(median_ms=statistics.median(x),q1_ms=q[0],q3_ms=q[2],min_ms=min(x),max_ms=max(x))
        result=dict(case=r['case']['name'],mode=r['mode'],group=r['group'],main_speedup=r['speedup'],samples=samples,statistics=stats,speedup=stats['before']['median_ms']/stats['after']['median_ms']);records.append(result)
        (OUT/'recheck.json').write_text(json.dumps(dict(metadata=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),main_results_sha256=sha(OUT/'results.json'),script_sha256=sha(pathlib.Path(__file__)),rounds=9,milliseconds=500,selection='every original median slowdown greater than 10%',cpu=meta['cpu']),records=records),indent=2)+'\n')
        print(result['case'],r['mode'],r['group'],'main',round(r['speedup'],3),'recheck',round(result['speedup'],3),flush=True)
    print('Rechecked',len(records),'original slowdowns; retained all main measurements.',flush=True)
if __name__=='__main__':main()
