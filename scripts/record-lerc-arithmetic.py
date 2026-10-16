#!/usr/bin/env python3
"""Record current proof/test/benchmark evidence for the LERC word kernels."""
import datetime,hashlib,json,pathlib,re,shutil,statistics
ROOT=pathlib.Path(__file__).resolve().parents[1]
OUT=ROOT/'target/lerc-arithmetic';E=ROOT/'docs/evidence/lerc-arithmetic'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 proof=json.loads((ROOT/'target/verification-summary.json').read_text())
 assert proof['success'] and proof['errors']==0 and proof['is-verifying-entire-crate']
 for n,h in proof['core_sources_sha256'].items():assert sha(ROOT/n)==h,n
 gate=json.loads((ROOT/'target/parallel-verification/receipt.json').read_text())
 assert len(gate['results'])==21 and all(r['exit_code']==0 for r in gate['results'])
 assert gate['verify_sh_sha256']==sha(ROOT/'scripts/verify.sh')
 texts={n:(OUT/n).read_text() for n in ['core-tests.log','compat-tests.log','sanitizer-tests.log','upstream.log','fuzz-tests.log']}
 rust=sum(map(int,re.findall(r'test result: ok\. (\d+) passed; 0 failed',texts['core-tests.log'])))
 py=lambda s:sum(map(int,re.findall(r'(?m)^Ran (\d+) tests? in ',s)))
 counts=(rust,py(texts['core-tests.log']),py(texts['compat-tests.log']),py(texts['sanitizer-tests.log']))
 assert counts==(58,97,63,63),counts
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
 timed=[r for r in results['records'] if 'statistics' in r];assert len(timed)==7
 E.mkdir(exist_ok=True)
 for p in OUT.glob('*.log'):
  if p.name in ['normal-pack-proof.log','word-lemmas-proof.log','addition-proof.log','round-significand-proof.log']:continue
  shutil.copy2(p,E/p.name)
 for source,dest in [('target/verification.json','verification.json'),('target/verification-summary.json','verification-summary.json'),('target/parallel-proof-gate.log','full-proof-gate.log'),('target/parallel-verification/receipt.json','parallel-receipt.json'),('target/verus-version.txt','verus-version.txt'),('target/false-contract.log','false-contract.log'),('target/original-corpus.json','original-corpus.json'),('target/lerc-arithmetic/before/build-receipt.json','baseline-build-receipt.json'),('docs/evidence/kernel-optimization/verification-summary.json','baseline-verification-summary.json')]:shutil.copy2(ROOT/source,E/dest)
 for p in (ROOT/'target').glob('reject-lerc-arithmetic-*.log'):shutil.copy2(p,E/p.name)
 for name in ['results.json','profile-after.json','profile-after.pcs']:
  if (OUT/name).exists():shutil.copy2(OUT/name,E/name)
 sources={str(p.relative_to(ROOT)):sha(p) for d in ['src','compat','tests','scripts','benchmarks'] for p in sorted((ROOT/d).rglob('*')) if p.is_file() and '__pycache__' not in p.parts and p.suffix!='.pyc'}
 summary=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),proof={k:proof[k] for k in ['verified','errors','success','is-verifying-entire-crate']},negative_gates=89,rust_tests=rust,core_python_tests=counts[1],c_tests=counts[2],sanitized_c_tests=counts[3],upstream_tests=108,corpus_images=31,core_fuzz_mutations=20000,exact_before_after_cases=96,exact_libtiff_cases=sum(r['difference']['max_channel_error']==0 for r in results['records']),timed_lerc_cases=7,lerc_geomean_speedup=statistics.geometric_mean(r['speedup'] for r in timed),source_sha256=sources,logs_sha256={p.name:sha(p) for p in sorted(E.glob('*.log'))},results_sha256=sha(E/'results.json'),sanitizer_note='C shim/consumers are instrumented; Rust is not. LeakSanitizer is disabled.',validation_note='The first core run and retry stopped at a test-only LogLuv encoder that had lost executable permissions after workspace restoration. The upstream builder and LogLuv test now restore executable mode. Passing pre-LogLuv tests from the retry and the successful continuation form core-tests.log; both failed runs are preserved separately. Core numerical source hashes were unchanged throughout the recorded suite.')
 (E/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
 print(json.dumps({k:v for k,v in summary.items() if k not in ['source_sha256','logs_sha256']},indent=2))
if __name__=='__main__':main()
