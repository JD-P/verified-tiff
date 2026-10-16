#!/usr/bin/env python3
"""Bind completed kernel validation and benchmark evidence to current sources."""
import datetime,hashlib,json,pathlib,re,shutil
ROOT=pathlib.Path(__file__).resolve().parents[1]
OUT=ROOT/'target/kernel-optimization';E=ROOT/'docs/evidence/kernel-optimization'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 proof=json.loads((ROOT/'target/verification-summary.json').read_text())
 assert proof['success'] and proof['is-verifying-entire-crate'] and proof['errors']==0
 for name,h in proof['core_sources_sha256'].items():assert sha(ROOT/name)==h,name
 receipt=json.loads((ROOT/'target/parallel-verification/receipt.json').read_text())
 assert len(receipt['results'])==20 and all(r['exit_code']==0 for r in receipt['results'])
 assert receipt['verify_sh_sha256']==sha(ROOT/'scripts/verify.sh')
 core=(OUT/'core-tests.log').read_text();compat=(OUT/'compat-tests.log').read_text();asan=(OUT/'sanitizer-tests.log').read_text()
 rust=sum(map(int,re.findall(r'test result: ok\. (\d+) passed; 0 failed',core)))
 py=lambda text:sum(map(int,re.findall(r'(?m)^Ran (\d+) tests? in ',text)))
 assert (rust,py(core),py(compat),py(asan))==(56,97,63,63),(rust,py(core),py(compat),py(asan))
 for text in [core,compat,asan]:assert not re.search(r'(?m)^(FAILED|ERROR:|FAIL:|Traceback)',text)
 assert '# PASS:  108' in (OUT/'upstream.log').read_text() or re.search(r'# PASS:\s+108', (OUT/'upstream.log').read_text())
 corpus=json.loads((ROOT/'target/original-corpus.json').read_text());assert corpus['accepted']==corpus['total']==31
 assert 'iterations=20000 ' in (OUT/'fuzz.log').read_text()
 results=json.loads((OUT/'results.json').read_text());assert len(results['records'])==92
 for name,h in results['metadata']['source_sha256'].items():assert sha(ROOT/name)==h,name
 assert results['metadata']['proof_sha256']==sha(ROOT/'target/verification-summary.json')
 assert results['metadata']['binary_sha256']['after-library']==sha(ROOT/'target/compat/libtiff.so.6')
 E.mkdir(exist_ok=True)
 for p in OUT.glob('*.log'):
  if p.name in ['build.log','focused-tests.log','reader-features.log','fp-proof.log','webp-proof.log','initial-proof.log']:continue
  shutil.copy2(p,E/p.name)
 for name in ['results.json','recheck.json']:
  if (OUT/name).exists():shutil.copy2(OUT/name,E/name)
 for source,dest in [('target/verification.json','verification.json'),('target/verification-summary.json','verification-summary.json'),('target/parallel-proof-gate.log','full-proof-gate.log'),('target/parallel-verification/receipt.json','parallel-receipt.json'),('target/false-contract.log','false-contract.log'),('target/verus-version.txt','verus-version.txt'),('target/original-corpus.json','original-corpus.json'),('target/kernel-optimization/before/build-receipt.json','baseline-build-receipt.json')]:shutil.copy2(ROOT/source,E/dest)
 for p in (ROOT/'target').glob('reject-kernel-*.log'):shutil.copy2(p,E/p.name)
 shutil.copy2(ROOT/'docs/evidence/rgba-optimization/verification-summary.json',E/'baseline-verification-summary.json')
 summary=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),proof={k:proof[k] for k in ['verified','errors','success','is-verifying-entire-crate']},negative_gates=84,rust_tests=rust,core_python_tests=py(core),c_tests=py(compat),sanitized_c_tests=py(asan),upstream_tests=108,corpus_images=31,core_fuzz_mutations=20000,source_sha256=results['metadata']['source_sha256'],logs_sha256={p.name:sha(p) for p in sorted(E.glob('*.log'))},results_sha256=sha(E/'results.json'),sanitizer_note='C shim/consumers are ASan/UBSan-instrumented; Rust is not. LeakSanitizer is disabled.',initial_run_note='A development verification run was left running while a new gate/test run started. The overlapping runs exhausted practical resources; both were interrupted and all gates/tests were rerun sequentially with three verification workers. The first complete C run also exposed a missing executable bit on a regenerated WebP oracle; the build helper now sets executable permissions explicitly, and the full C run is repeated. An upstream tiffdump test executable also needed its execute bit restored; the full 108-test upstream run then passed. Interrupted/initial logs are not passing evidence.')
 if (E/'recheck.json').exists():summary['recheck_sha256']=sha(E/'recheck.json')
 (E/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
 print('Recorded current proof, tests and all 92 exact before/after outputs.')
if __name__=='__main__':main()
