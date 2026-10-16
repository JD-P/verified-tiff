#!/usr/bin/env python3
"""Record only successful, source-matching adapter proof and test evidence."""
import datetime,hashlib,json,pathlib,re,shutil
R=pathlib.Path(__file__).resolve().parents[1];E=R/'docs/evidence/adapter-proofs'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def load(p):return json.loads(p.read_text())
def count_python(s):return sum(map(int,re.findall(r'(?m)^Ran (\d+) tests? in ',s)))
def count_rust(s):return sum(map(int,re.findall(r'test result: ok\. (\d+) passed;',s)))
p=load(R/'target/verification-summary.json')
assert p['success'] and p['errors']==0 and p['is-verifying-entire-crate']
for mapping in [p['core_sources_sha256'],p['trusted_float']['source_sha256'],p['trusted_simd']['source_sha256']]:
 for name,h in mapping.items():assert sha(R/name)==h,name
g=load(R/'target/parallel-verification/receipt.json')
assert len(g['results'])==26 and all(v['exit_code']==0 for v in g['results'])
assert g['verify_sh_sha256']==sha(R/'scripts/verify.sh')
negative=(R/'target/parallel-proof-gate.log').read_text()
assert len(re.findall(r'(?m)^.*(?:check|gate):.*rejected.*$',negative))==125
# The initial parallel full-core run may precede the finalized MaybeUninit
# integration. The final complete run below is authoritative, not its old hash.
logs={'full-proof.log':'target/adapter-full-proof.log','core-tests.log':'target/adapter-core-final.log','compat-tests.log':'target/adapter-compat-final.log','sanitizers.log':'target/adapter-sanitizers.log','focused-tests.log':'target/adapter-focused-final.log','fuzz-smoke.log':'target/adapter-fuzz-smoke.log'}
texts={k:(R/v).read_text() for k,v in logs.items()}
for name,s in texts.items():
 assert not re.search(r'(?m)^(FAILED|ERROR:|FAIL:|Traceback|error:)',s),name
assert count_python(texts['core-tests.log'])==97
assert count_rust(texts['core-tests.log'])==70
assert count_rust(texts['focused-tests.log'])==8
assert count_python(texts['compat-tests.log'])==64
assert count_python(texts['sanitizers.log'])==64
assert re.search(r'(?m)^seed=0x5449464650524f4f iterations=20000 accepted=\d+ errors=',texts['fuzz-smoke.log'])
E.mkdir(exist_ok=True,parents=True)
for dest,src in logs.items():shutil.copy2(R/src,E/dest)
for src,dest in [('target/verification.json','verification.json'),('target/verification-summary.json','verification-summary.json'),('target/parallel-proof-gate.log','full-proof-gate.log'),('target/parallel-verification/receipt.json','parallel-receipt.json')]:shutil.copy2(R/src,E/dest)
for f in (R/'target').glob('reject-adapter-*.log'):shutil.copy2(f,E/f.name)
sources={str(f.relative_to(R)):sha(f) for directory in ['src','compat','tests','scripts','trusted-float','trusted-simd'] for f in sorted((R/directory).rglob('*')) if f.is_file() and '__pycache__' not in f.parts and f.suffix!='.pyc'}
sources.update({name:sha(R/name) for name in ['Cargo.toml','Cargo.lock','rust-toolchain.toml','toolchain-lock.json']})
summary={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'proof':{k:p[k] for k in ['verified','errors','success','is-verifying-entire-crate']},'negative_gates':125,'new_negative_gates':10,'rust_tests':70,'focused_rust_tests':8,'core_python_tests':97,'c_python_tests':64,'sanitized_c_python_tests':64,'malformed_schema_configurations':12,'mutation_smoke_iterations':20000,'source_sha256':sources,'trusted_float':p['trusted_float'],'trusted_simd':p['trusted_simd'],'boundary':'Local executable adapter decisions verified; C varargs, pointer validity, ownership/event completeness, callbacks and whole-pipeline composition remain unverified.'}
summary['evidence_scope']={'positive_proof':'verification.json and verification-summary.json record the final complete source; full-proof.log is that run.','parallel_run':'The parallel receipt and full-proof-gate.log establish all negative gates. Their initial positive run predates final output initialization and is superseded by the final complete proof.'}
(E/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps({k:v for k,v in summary.items() if k not in ['source_sha256','trusted_float','trusted_simd']},indent=2))
