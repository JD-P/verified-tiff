#!/usr/bin/env python3
import json,sys,pathlib,hashlib
data=json.load(open(sys.argv[1]))
# Exact report keys are checked against the pinned verifier during project setup.
results=data.get('verification-results',{})
if (results.get('encountered-error') or results.get('errors',0)!=0 or results.get('verified',0)<1
    or results.get('success') is not True or results.get('is-verifying-entire-crate') is not True):
    raise SystemExit('Verification did not report a nonempty successful core proof run')
root=pathlib.Path(__file__).resolve().parents[1]
lock=json.loads((root/'toolchain-lock.json').read_text())
if data.get('verus',{}).get('commit')!=lock['verus_commit']:
    raise SystemExit('Unexpected verifier commit')
trusted=json.loads((root/'target/trusted-float/build.json').read_text())
for n,h in trusted['source_sha256'].items():
    assert hashlib.sha256((root/n).read_bytes()).hexdigest()==h,n
simd=json.loads((root/'target/trusted-simd/build.json').read_text())
for n,h in simd['source_sha256'].items():
    assert hashlib.sha256((root/n).read_bytes()).hexdigest()==h,n
summary=dict(results,trusted_float=trusted,trusted_simd=simd,core_sources_sha256={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted((root/'src').glob('*.rs')) if p.name!='main.rs'},source_sha256=hashlib.sha256((root/'src/lib.rs').read_bytes()).hexdigest(),verus=data['verus'])
(root/'target/verification-summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
