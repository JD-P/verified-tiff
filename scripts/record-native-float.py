#!/usr/bin/env python3
"""Record current proof/test/benchmark evidence for the native hardware floating point."""
import datetime,hashlib,json,pathlib,re,shutil,statistics
ROOT=pathlib.Path(__file__).resolve().parents[1]
OUT=ROOT/'target/native-float';E=ROOT/'docs/evidence/native-float'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 proof=json.loads((ROOT/'target/verification-summary.json').read_text())
 assert proof['success'] and proof['errors']==0 and proof['is-verifying-entire-crate']
 for n,h in proof['core_sources_sha256'].items():assert sha(ROOT/n)==h,n
 for n,h in proof['trusted_float']['source_sha256'].items():assert sha(ROOT/n)==h,n
 gate=json.loads((ROOT/'target/parallel-verification/receipt.json').read_text())
 assert len(gate['results'])==22 and all(r['exit_code']==0 for r in gate['results'])
 assert gate['verify_sh_sha256']==sha(ROOT/'scripts/verify.sh')
 texts={n:(OUT/n).read_text() for n in ['core-tests.log','compat-tests.log','sanitizer-tests.log','upstream.log','fuzz-tests.log']}
 rust=sum(map(int,re.findall(r'test result: ok\. (\d+) passed; 0 failed',texts['core-tests.log'])))
 py=lambda s:sum(map(int,re.findall(r'(?m)^Ran (\d+) tests? in ',s)))
 counts=(rust,py(texts['core-tests.log']),py(texts['compat-tests.log']),py(texts['sanitizer-tests.log']))
 assert counts==(59,97,63,63),counts
 for n in ['core-tests.log','compat-tests.log','sanitizer-tests.log']:
  assert not re.search(r'(?m)^(FAILED|ERROR:|FAIL:|Traceback)',texts[n]),n
 assert re.search(r'# PASS:\s+108',texts['upstream.log'])
 assert 'iterations=20000 ' in texts['fuzz-tests.log']
 corpus=json.loads((ROOT/'target/original-corpus.json').read_text());assert corpus['accepted']==corpus['total']==31
 results=json.loads((OUT/'results.json').read_text());assert len(results['records'])==96
 for n,h in results['metadata']['source_sha256'].items():assert sha(ROOT/n)==h,n
 assert results['metadata']['proof_sha256']==sha(ROOT/'target/verification-summary.json')
 assert results['metadata']['binary_sha256']['after-library']==sha(ROOT/'target/compat/libtiff.so.6')
 for r in results['records']:assert r['correctness']['before']==r['correctness']['after']
 assert 'source audit rejected injected assume' in texts.get('audit-negative.log',(OUT/'audit-negative.log').read_text())
 timed=[r for r in results['records'] if 'statistics' in r];assert len(timed)==7
 E.mkdir(exist_ok=True)
 for name in ['core-tests.log','compat-tests.log','sanitizer-tests.log','upstream.log','fuzz-tests.log','numeric-tests.log','numeric-proof.log','instructions-build.log','baseline-restore.log','build-before.log','audit-negative.log','compat-tests-incomplete.log','compat-tests-prefix.log','lerc_compat-continuation.log','webp_compat-continuation.log','codec_malformed-continuation.log']:
  shutil.copy2(OUT/name,E/name)
 shutil.copy2(OUT/'instructions.txt',E/'instructions.txt')
 original=(OUT/'restored-baseline-source/src/lerc_numeric.rs').read_text();specs=[];names=[]
 for match in re.finditer(r'pub\(super\) open spec fn (\w+)',original):
  start=original.index('{',match.start());depth=1;end=start+1
  while depth:
   depth+=(original[end]=='{')-(original[end]=='}');end+=1
  specs.append(original[match.start():end].replace('pub(super)','pub',1));names.append(match[1])
 expected='//! Pure bit specifications moved unchanged from lerc_numeric.rs.\nuse vstd::prelude::*;\nverus! {\n'+'\n'.join(specs)+'\n}\n'
 assert expected==(ROOT/'trusted-float/src/model.rs').read_text()
 model=dict(unchanged_functions=names,model_sha256=sha(ROOT/'trusted-float/src/model.rs'))
 (E/'model-move.json').write_text(json.dumps(model,indent=2)+'\n')
 shutil.copy2(ROOT/'target/trusted-float/build.json',E/'trusted-build.json')
 shutil.copy2(ROOT/'trusted-float/audit.json',E/'trusted-audit.json')
 for source,dest in [('target/verification.json','verification.json'),('target/verification-summary.json','verification-summary.json'),('target/parallel-proof-gate.log','full-proof-gate.log'),('target/parallel-verification/receipt.json','parallel-receipt.json'),('target/verus-version.txt','verus-version.txt'),('target/false-contract.log','false-contract.log'),('target/original-corpus.json','original-corpus.json'),('target/native-float/before/build-receipt.json','baseline-build-receipt.json'),('docs/evidence/lerc-arithmetic/verification-summary.json','baseline-verification-summary.json')]:shutil.copy2(ROOT/source,E/dest)
 for p in (ROOT/'target').glob('reject-native-float-*.log'):shutil.copy2(p,E/p.name)
 for name in ['results.json','profile-after.json','profile-after.pcs']:
  if (OUT/name).exists():shutil.copy2(OUT/name,E/name)
 sources={str(p.relative_to(ROOT)):sha(p) for d in ['src','compat','tests','scripts','benchmarks','trusted-float'] for p in sorted((ROOT/d).rglob('*')) if p.is_file() and '__pycache__' not in p.parts and p.suffix!='.pyc'}
 summary=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),proof={k:proof[k] for k in ['verified','errors','success','is-verifying-entire-crate']},negative_gates=95,rust_tests=rust,core_python_tests=counts[1],c_tests=counts[2],sanitized_c_tests=counts[3],upstream_tests=108,corpus_images=31,core_fuzz_mutations=20000,exact_before_after_cases=96,exact_libtiff_cases=sum(r['difference']['max_channel_error']==0 for r in results['records']),timed_lerc_cases=7,lerc_geomean_speedup=statistics.geometric_mean(r['speedup'] for r in timed),source_sha256=sources,logs_sha256={p.name:sha(p) for p in sorted(E.glob('*.log'))},results_sha256=sha(E/'results.json'),sanitizer_note='C shim/consumers are instrumented; Rust is not. LeakSanitizer is disabled.',validation_note='Full decoder proof is conditional on five explicitly trusted hardware-FP contracts. The native primitive bodies are not proved. Full --no-cheating is not claimed; source auditing and six new semantic/precondition gates enforce the documented boundary. Numeric tests compare hardware dispatch directly against the preserved software reference. Normal C-suite output stopped at the start of LERC despite a zero exit status; the completed 56-test prefix is preserved, and the seven LERC/WebP/malformed tests were rerun with captured output and explicit zero exit statuses. The combined normal log contains all 63 methods. The sanitizer log is one complete 63-method run.',trusted_float=proof['trusted_float'])
 (E/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
 print(json.dumps({k:v for k,v in summary.items() if k not in ['source_sha256','logs_sha256']},indent=2))
if __name__=='__main__':main()
