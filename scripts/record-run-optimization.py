#!/usr/bin/env python3
"""Record matching whole-crate proof, regression and run-optimization benchmark evidence."""
import datetime,hashlib,json,pathlib,re,shutil,statistics,subprocess
import importlib.util
R=pathlib.Path(__file__).resolve().parents[1]
RUN=R/'target/run-evidence';BENCH=R/'target/run-optimization';E=R/'docs/evidence/run-optimization'
STEPS=['before','jcc','webp','deflate','strips','fax','jpeg','bilevel','rgba','packbits','lzw','lzwstream','rgbarows',
       'lookahead','render','onepass','packed','phrases','spans','predict','vp8bool','unshuffle','pack8',
       'vp8filter','prefix','storedslice','matches','stored']
spec=importlib.util.spec_from_file_location('report',R/'benchmarks/run-optimization-report.py')
report=importlib.util.module_from_spec(spec);spec.loader.exec_module(report)
def sha(p):return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def read(p):return json.loads((R/p).read_text())
def main():
 proof=read('target/verification-summary.json')
 assert proof['success'] and proof['errors']==0 and proof['is-verifying-entire-crate']
 for group in [proof['core_sources_sha256'],proof['trusted_float']['source_sha256'],proof['trusted_simd']['source_sha256']]:
  for n,h in group.items():assert sha(R/n)==h,n
 receipt=read('target/parallel-verification/receipt.json')
 assert receipt['verify_sh_sha256']==sha(R/'scripts/verify.sh')
 assert len(receipt['results'])==27 and all(r['exit_code']==0 for r in receipt['results'])
 gates=(R/'target/parallel-proof-gate.log').read_text()
 rejected=len(re.findall(r'(?m)^[^\n]*: (?:correctly )?rejected ',gates))
 assert rejected==173,rejected
 # Every run gate is new in this pass.
 run=re.findall(r"(?m)^ '([a-z0-9-]+)':",(R/'scripts/reject-run-optimization.py').read_text())
 assert len(run)==45,len(run)
 for name in run:assert 'Run gate: rejected broken '+name+'\n' in gates,name
 retargeted={'Kernel gate':['match-output','match-work','fp-coordinates'],'Native semantic gate':['float-shuffle'],'RGBA gate':['pixels']}
 for kind,names in retargeted.items():
  for name in names:assert kind+': rejected broken '+name+'\n' in gates,name
 texts={n:(RUN/n).read_text() for n in ['core-tests.log','compat-tests.log','sanitizer-tests.log','upstream.log','fuzz.log','corpus.log','build-after.log','prepare-steps.log']}
 bad=r'(?mi)^(?:FAILED|ERROR:|FAIL:|Traceback|error:)|runtime error:|AddressSanitizer:'
 for n in ['core-tests.log','compat-tests.log','sanitizer-tests.log','fuzz.log','corpus.log']:
  assert not re.search(bad,texts[n]),n
  assert texts[n].rstrip().endswith('exit=0'),n
 py=lambda s:sum(map(int,re.findall(r'(?m)^Ran (\d+) tests? in ',s)))
 rust=sum(map(int,re.findall(r'test result: ok\. (\d+) passed; 0 failed',texts['core-tests.log'])))
 counts=(rust,py(texts['core-tests.log']),py(texts['compat-tests.log']),py(texts['sanitizer-tests.log']))
 assert counts==(72,97,70,70),counts
 for n in ['compat-tests.log','sanitizer-tests.log']:assert texts[n].count('C shim hardening assertions passed')==1,n
 assert re.search(r'# PASS:\s+108',texts['upstream.log']) and re.search(r'# FAIL:\s+0',texts['upstream.log'])
 assert 'iterations=20000' in texts['fuzz.log']
 corpus=read('target/original-corpus.json');assert corpus['accepted']==corpus['total']==31
 results=json.loads((BENCH/'results.json').read_text());meta=results['metadata'];records=results['records']
 assert len(records)==96,len(records)
 for n,h in meta['source_sha256'].items():assert sha(R/n)==h,n
 assert meta['proof_sha256']==sha(R/'target/verification-summary.json')
 assert meta['binary_sha256']['after-library']==sha(R/'target/compat/libtiff.so.6')
 assert meta['binary_sha256']['before-library']==sha(BENCH/'before/lib/libtiff.so.6')
 assert meta['chain']==STEPS+['after'],meta['chain']
 for r in records:
  for e in meta['chain']:assert r['correctness'][e]==r['correctness']['before'],(r['case']['name'],r['mode'],e)
 ir=json.loads((BENCH/'instructions.json').read_text())
 assert ir['metadata']['results_sha256']==sha(BENCH/'results.json')
 for e,h in ir['metadata']['binary_sha256'].items():assert sha(R/'target/reader-benchmark/verified' if e=='after' else BENCH/e/'verified')==h,e
 step_ir={}
 for a,b in zip(meta['chain'][1:],meta['chain'][2:]):
  t=[r['case']['file']+'|'+r['mode'] for r in records if report.TARGETS[b](r)]
  step_ir[b]=statistics.geometric_mean(ir['counts'][a+'|'+k]/ir['counts'][b+'|'+k] for k in t)
 # Map each measured tree to the branch commit with identical built paths
 # (src, compat, scripts, tests, ...); commits may also touch docs.
 built=['.cargo','src','compat','examples','trusted-float','trusted-simd','scripts','tests','fuzz','Cargo.toml','Cargo.lock','rust-toolchain.toml','toolchain-lock.json']
 key=lambda t:tuple(subprocess.run(['git','rev-parse','--verify','-q',t+':'+p],cwd=R,capture_output=True,text=True).stdout.strip() for p in built)
 commits={key(c):c for c in subprocess.check_output(['git','rev-list','9a64643^..HEAD'],cwd=R,text=True).split()}
 commits={t:commits.get(key(t)) for t in (meta['step_receipts'][e]['tree'] for e in STEPS)}
 for e in STEPS:
  receipt=meta['step_receipts'][e];assert commits[receipt['tree']],e
  assert sha(BENCH/e/'lib/libtiff.so.6')==receipt['library_sha256'],e
 # The measured and verified code is exactly the committed code.
 subprocess.run(['git','diff','--quiet','HEAD','--','.cargo','src','compat','trusted-float','trusted-simd','scripts/verify.sh','scripts/reject-run-optimization.py'],cwd=R,check=True)
 base=read('docs/evidence/metadata-contracts/verification-summary.json')
 before=json.loads((BENCH/'before/build-receipt.json').read_text())
 # The restored baseline is exactly the source the previous fresh proof covered.
 for n,h in base['core_sources_sha256'].items():assert before['source_sha256'][n]==h,n
 E.mkdir(parents=True,exist_ok=True)
 for n in texts:shutil.copy2(RUN/n,E/n)
 for src,dest in [('target/verification.json','verification.json'),('target/verification-summary.json','verification-summary.json'),
                  ('target/parallel-proof-gate.log','full-proof-gate.log'),('target/parallel-verification/receipt.json','parallel-receipt.json'),
                  ('target/verus-version.txt','verus-version.txt'),('target/false-contract.log','false-contract.log'),
                  ('target/original-corpus.json','original-corpus.json'),
                  ('target/run-optimization/benchmark.log','benchmark.log'),('target/run-optimization/instructions.json','instructions.json'),
                  ('docs/evidence/metadata-contracts/verification-summary.json','baseline-verification-summary.json')]:
  shutil.copy2(R/src,E/dest)
 # Receipts keep each step build log's hash; the logs themselves stay out.
 for e in STEPS:shutil.copy2(BENCH/e/'build-receipt.json',E/f'{e}-build-receipt.json')
 for p in [R/'target'/f'reject-run-{n}.log' for n in run]+[R/'target'/n for n in ['reject-kernel-match-output.log','reject-kernel-match-work.log',
     'reject-kernel-fp-coordinates.log','reject-native-float-shuffle.log','reject-rgba-pixels.log']]:
  shutil.copy2(p,E/p.name)
 # Per-sample timings stay out of the repository; every statistic the report
 # and summary use is kept, and the full file's hash is recorded.
 keep=['case','mode','correctness','difference','statistics','speedup','step_speedup','vs_libtiff']
 (E/'results.json').write_text(json.dumps(dict(metadata=meta,full_results_sha256=sha(BENCH/'results.json'),
     records=[{k:r[k] for k in keep} for r in records]),indent=1)+'\n')
 sources={str(p.relative_to(R)):sha(p) for d in ['src','compat','tests','scripts','benchmarks','trusted-float','trusted-simd']
          for p in sorted((R/d).rglob('*')) if p.is_file() and '__pycache__' not in p.parts and p.suffix!='.pyc'}
 sources.update({n:sha(R/n) for n in ['Cargo.toml','Cargo.lock','rust-toolchain.toml','toolchain-lock.json']})
 speed=[r['speedup'] for r in records]
 summary=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
  proof={k:proof[k] for k in ['verified','errors','success','is-verifying-entire-crate']},
  negative_gates=rejected,new_negative_gates=len(run),retargeted_negative_gates=retargeted,
  rust_tests=rust,core_python_tests=counts[1],c_tests=counts[2],sanitized_c_tests=counts[3],upstream_tests=108,
  corpus_images=31,core_fuzz_mutations=20000,workloads=len(records),exact_before_after_cases=len(records),
  exact_libtiff_cases=sum(r['difference']['exact'] for r in records),
  geomean_speedup=statistics.geometric_mean(speed),
  step_commits={e:commits[meta['step_receipts'][e]['tree']] for e in STEPS},
  geomean_step_speedup={b:statistics.geometric_mean(r['step_speedup'][b] for r in records) for b in meta['chain'][1:]},
  geomean_after_vs_libtiff=statistics.geometric_mean(r['vs_libtiff'] for r in records),
  geomean_before_vs_libtiff=statistics.geometric_mean(r['statistics']['before']['median_ms']/r['statistics']['libtiff']['median_ms'] for r in records),
  source_sha256=sources,logs_sha256={p.name:sha(p) for p in sorted(E.glob('*.log'))},results_sha256=sha(E/'results.json'),
  geomean_step_instructions=step_ir,
  sanitizer_note='C shim/consumers are instrumented with ASan/UBSan; Rust is not. LeakSanitizer is disabled.',
  coverage_note='No C source changed, so the C coverage gate was not rerun; the shim hardening assertions ran in both C suites.',
  validation_note='The whole-crate proof keeps the same five trusted FP and eleven trusted SSE2 contracts. No new trusted operation. Every before/after output is exact, including JPEG. The adapter changes (bilevel packing, packed RGBA runs, CMYK/Lab conversion spans (pixel placement uses the verified adapter_geometry::span_pixel), whole-segment and streamed LZW, and dispatch to the verified fast paths) are unverified Rust covered by the C integration tests. The jcc step only adds the LLVM branch-alignment flag to Cargo builds; rust_verify checks the same source.',
  trusted_float=proof['trusted_float'],trusted_simd=proof['trusted_simd'])
 (E/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
 print(json.dumps({k:v for k,v in summary.items() if k not in ['source_sha256','logs_sha256','trusted_float','trusted_simd']},indent=2))
if __name__=='__main__':main()
