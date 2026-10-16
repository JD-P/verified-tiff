#!/usr/bin/env python3
"""Run the unchanged verify.sh gate in independent chunks, including full proof.

Only orchestration differs. Every command and required failure diagnostic comes
from verify.sh. Positive verification remains a full-crate run with the audited FP and SSE2 boundaries.
"""
import argparse,concurrent.futures,hashlib,json,pathlib,re,subprocess,time
ROOT=pathlib.Path(__file__).resolve().parents[1]
def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--jobs',type=int,default=3)
    a=p.parse_args();assert 1<=a.jobs<=4
    source=(ROOT/'scripts/verify.sh').read_text()
    starts=[m.start() for m in re.finditer(r'(?m)^(?:python3 scripts/reject-|VERUS="\$VERUS" python3 scripts/reject-)',source)]
    assert len(starts)==25, 'verify.sh gate structure changed; review chunk boundaries'
    cuts=[0,*starts,len(source)];folder=ROOT/'target/parallel-verification';folder.mkdir(exist_ok=True)
    chunks=[]
    for i,(b,e) in enumerate(zip(cuts,cuts[1:])):
        body=source[b:e]
        if i==0:body=body.replace('source "$(dirname "$0")/env.sh"','source scripts/env.sh')
        else:body='set -euo pipefail\nsource scripts/env.sh\ncd "$TIFF_ROOT"\n'+body
        chunks.append(body)
    def run(item):
        i,body=item;log=folder/f'{i:02}.log';start=time.monotonic()
        with log.open('w') as f:r=subprocess.run(['bash','-c',body],cwd=ROOT,stdout=f,stderr=subprocess.STDOUT)
        return dict(chunk=i,exit_code=r.returncode,seconds=time.monotonic()-start,log=str(log.relative_to(ROOT)))
    with concurrent.futures.ThreadPoolExecutor(max_workers=a.jobs) as pool:
        records=list(pool.map(run,enumerate(chunks)))
    (ROOT/'target/parallel-proof-gate.log').write_text(''.join((ROOT/r['log']).read_text() for r in records))
    (folder/'receipt.json').write_text(json.dumps(dict(verify_sh_sha256=hashlib.sha256(source.encode()).hexdigest(),jobs=a.jobs,results=records),indent=2)+'\n')
    failed=[r for r in records if r['exit_code']]
    if failed:raise SystemExit('Gate failures: '+str(failed))
    print('Full verification and every verify.sh negative gate passed ('+str(len(chunks))+' independent chunks).')
if __name__=='__main__':main()
