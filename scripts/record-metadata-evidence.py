#!/usr/bin/env python3
"""Record fresh proof, regression, sanitizer and C coverage runs for metadata fixes."""
import datetime,hashlib,json,pathlib,re,shutil,subprocess
R=pathlib.Path(__file__).resolve().parents[1]
E=R/'docs/evidence/metadata-contracts'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(p):return json.loads((R/p).read_text())
proof=read('target/verification-summary.json')
assert proof['success'] and proof['errors']==0 and proof['is-verifying-entire-crate']
assert proof['verified']==1809
for group in [proof['core_sources_sha256'],proof['trusted_float']['source_sha256'],proof['trusted_simd']['source_sha256']]:
 for n,h in group.items():assert sha(R/n)==h,n
receipt=read('target/parallel-verification/receipt.json')
assert receipt['verify_sh_sha256']==sha(R/'scripts/verify.sh')
assert len(receipt['results'])==26 and all(r['exit_code']==0 for r in receipt['results'])
gates=(R/'target/parallel-proof-gate.log').read_text()
assert len(re.findall(r'(?m)^[^\n]*: (?:correctly )?rejected ',gates))==128
for name in ['ink-count','ink-acceptance','ink-length']:
 assert 'Adapter proof gate: correctly rejected broken '+name in gates
coverage=read('target/metadata-coverage/coverage.json')
assert coverage['total']==[1351,1351,242,242,1455,1455]
logs={'coverage-suite.log':'target/metadata-coverage/suite.log',
 'coverage-gate.log':'target/metadata-coverage-gate.log','optimized-suite.log':'target/metadata-compat.log',
 'sanitized-suite.log':'target/metadata-sanitized.log','core-tests.log':'target/metadata-core-tests.log',
 'fuzz.log':'target/metadata-fuzz.log','source-audit.log':'target/metadata-audit.log',
 'proof-gate.log':'target/parallel-proof-gate.log'}
for name,path in logs.items():
 text=(R/path).read_text()
 assert not re.search(r'(?mi)^(?:FAILED|ERROR:|Traceback|error:)|runtime error:|AddressSanitizer:|libgcov profiling error',text),name
 if name.endswith('suite.log'):
  assert sum(map(int,re.findall(r'(?m)^Ran (\d+) tests? in ',text)))==70,name
  assert text.count('C shim hardening assertions passed')==1,name
 if name=='core-tests.log':
  assert sum(map(int,re.findall(r'(?m)^Ran (\d+) tests? in ',text)))==97
  assert sum(map(int,re.findall(r'test result: ok\. (\d+) passed;',text)))==71
assert 'every emitted C line/function/branch covered; no exclusions.' in (R/logs['coverage-gate.log']).read_text()
assert 'iterations=20000' in (R/logs['fuzz.log']).read_text()
subprocess.run(['python3','compat/generate_fields.py','--check'],cwd=R,check=True)
E.mkdir(exist_ok=True,parents=True)
for name,path in logs.items():shutil.copy2(R/path,E/name)
for n in ['coverage.json','coverage-raw.json','tiff_compat.gcov.json.gz','stubs.gcov.json.gz','shim-hardening-shim_hardening.gcov.json.gz','api.json']:
 shutil.copy2(R/'target/metadata-coverage'/n,E/n)
for n in ['verification.json','verification-summary.json']:
 shutil.copy2(R/'target'/n,E/n)
shutil.copy2(R/'target/parallel-verification/receipt.json',E/'proof-receipt.json')
sources={str(f.relative_to(R)):sha(f) for directory in ['src','compat','tests','scripts','trusted-float','trusted-simd'] for f in sorted((R/directory).rglob('*')) if f.is_file() and '__pycache__' not in f.parts and f.suffix!='.pyc'}
sources.update({n:sha(R/n) for n in ['Cargo.toml','Cargo.lock','rust-toolchain.toml','toolchain-lock.json']})
summary={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
 'coverage':{'functions':[242,242],'lines':[1351,1351],'branches':[1455,1455],'public_api':[198,198],'exclusions':[],
 'scope':'Linux LP64 production C; real-backend integration plus boundary fault injection and defensive internal-state tests'},
 'compiler':subprocess.check_output(['gcc','--version'],text=True).splitlines()[0],
 'optimized_python_tests':70,'sanitized_python_tests':70,'core_python_tests':97,'rust_tests':71,
 'fuzz_iterations':20000,'negative_proof_gates':128,
 'boundary_harness':'passed optimized, under sanitizers, and with gcov; C allocation ledger returned to zero',
 'sanitizers':['address','undefined','float-cast-overflow'],
 'leak_sanitizer':'Unavailable in this container; disabled, with explicit C allocation ledger checked instead.',
 'core_proof':{'verified':proof['verified'],'errors':0,'reused_unchanged_sources':False,'entire_crate':True,
 'receipt':'docs/evidence/metadata-contracts/verification-summary.json'},
 'source_sha256':sources,'evidence_sha256':{str(f.relative_to(E)):sha(f) for f in sorted(E.iterdir()) if f.is_file() and f.name!='summary.json'}}
(E/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps({k:v for k,v in summary.items() if not k.endswith('sha256')},indent=2))
