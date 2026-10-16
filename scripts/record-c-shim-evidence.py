#!/usr/bin/env python3
"""Record completed C hardening runs and explicitly reused unchanged core proof."""
import datetime,hashlib,json,pathlib,re,shutil,subprocess
R=pathlib.Path(__file__).resolve().parents[1];E=R/'docs/evidence/c-shim-hardening'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(p):return json.loads((R/p).read_text())
coverage=read('target/c-coverage/coverage.json')
assert coverage['total']==[1352,1352,242,242,1459,1459]
proof=read('docs/evidence/adapter-proofs/verification-summary.json')
assert proof['success'] and proof['errors']==0 and proof['is-verifying-entire-crate']
for group in [proof['core_sources_sha256'],proof['trusted_float']['source_sha256'],proof['trusted_simd']['source_sha256']]:
 for n,h in group.items():assert sha(R/n)==h,n
logs={'coverage-suite.log':'target/c-coverage/suite.log','coverage-gate.log':'target/c-coverage-final.log','optimized-suite.log':'target/shim-compat-final.log','sanitized-suite.log':'target/shim-sanitizers-final.log','source-audit.log':'target/shim-source-audit.log'}
for name,path in logs.items():
 text=(R/path).read_text()
 assert not re.search(r'(?mi)^(?:FAILED|ERROR:|Traceback|error:)|runtime error:|AddressSanitizer:|libgcov profiling error',text),name
 if name.endswith('suite.log'):
  assert sum(map(int,re.findall(r'(?m)^Ran (\d+) tests? in ',text)))==64,name
  assert text.count('C shim hardening assertions passed')==1,name
assert 'every emitted C line/function/branch covered; no exclusions.' in (R/logs['coverage-gate.log']).read_text()
E.mkdir(exist_ok=True,parents=True)
for name,path in logs.items():shutil.copy2(R/path,E/name)
for n in ['coverage.json','coverage-raw.json','tiff_compat.gcov.json.gz','stubs.gcov.json.gz','shim-hardening-shim_hardening.gcov.json.gz','api.json']:
 shutil.copy2(R/'target/c-coverage'/n,E/n)
shutil.copy2(R/'target/c-shim-before.json',E/'before-coverage.json')
sources={str(f.relative_to(R)):sha(f) for directory in ['src','compat','tests','scripts','trusted-float','trusted-simd'] for f in sorted((R/directory).rglob('*')) if f.is_file() and '__pycache__' not in f.parts and f.suffix!='.pyc'}
sources.update({n:sha(R/n) for n in ['Cargo.toml','Cargo.lock','rust-toolchain.toml','toolchain-lock.json']})
summary={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'coverage':{'functions':[242,242],'lines':[1352,1352],'branches':[1459,1459],'public_api':[198,198],'exclusions':[],'scope':'Linux LP64 production C only; real-backend integration plus boundary fault injection and defensive internal-state tests'},'compiler':subprocess.check_output(['gcc','--version'],text=True).splitlines()[0],'gcov':subprocess.check_output(['gcov','--version'],text=True).splitlines()[0],'optimized_python_tests':64,'sanitized_python_tests':64,'boundary_harness':'passed at -O2, under sanitizers, and at -O0 with gcov; C-owned allocation count returns to zero','sanitizers':['address','undefined','float-cast-overflow'],'leak_sanitizer':'Unavailable: container process/thread inspection fails; explicit C allocation ledger checked instead.','core_proof':{'verified':proof['verified'],'errors':0,'reused_unchanged_sources':True,'receipt':'docs/evidence/adapter-proofs/verification-summary.json'},'source_sha256':sources,'evidence_sha256':{str(f.relative_to(E)):sha(f) for f in sorted(E.iterdir()) if f.is_file() and f.name!='summary.json'}}
(E/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps({k:v for k,v in summary.items() if not k.endswith('sha256')},indent=2))
