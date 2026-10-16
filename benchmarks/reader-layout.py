#!/usr/bin/env python3
"""Controlled RGB Deflate strip-height sweep; run after reader-bench.py.
Identical pixels, freshly opened handles, unchanged measured binaries.
This diagnostic is separate from the main suite's geometric means.
"""
import hashlib,json,os,pathlib,random,statistics,subprocess,datetime
ROOT=pathlib.Path(__file__).resolve().parents[1];OUT=ROOT/'target/reader-benchmark'
def sha(p):return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def cmd(a):return subprocess.run(list(map(str,a)),capture_output=True,text=True,check=True,timeout=300).stdout
main=json.loads((OUT/'results.json').read_text());meta=main['metadata'];cpu=meta['cpu'];os.sched_setaffinity(0,{cpu})
engines={'verified':OUT/'verified','libtiff':OUT/'reference'}
for e,p in engines.items():assert sha(p)==meta['binary_sha256'][e]
assert sha(ROOT/'target/compat/libtiff.so.6')==meta['binary_sha256']['adapter_library']
for p,h in meta['source_sha256'].items():assert sha(ROOT/p)==h
records=[];rng=random.Random(20261016);native_hash=None
for rows in [16,32,64,256]:
 path=OUT/'inputs'/('layout-rgb-deflate-'+str(rows)+'.tif')
 cmd([OUT/'encode',path,8,8,3,0,0,1,1024,1024,rows,1,2,0,1])
 for mode in ['native','crop']:
  correct={}
  for e,exe in engines.items():
   dump=OUT/(e+'-layout.bin');m=json.loads(cmd([exe,path,mode,0,'memory',dump]));correct[e]={'sha256':sha(dump),'hash':m['hash']}
  assert correct['verified']==correct['libtiff']
  if mode=='native':
   if native_hash is None:native_hash=correct['verified']['sha256']
   assert native_hash==correct['verified']['sha256']
  samples={e:[] for e in engines}
  for trial in range(meta['rounds']):
   order=list(engines);rng.shuffle(order)
   for e in order:
    m=json.loads(cmd([engines[e],path,mode,meta['milliseconds'],'memory']));assert m['hash']==correct[e]['hash'];samples[e].append(m)
  stats={}
  for e in engines:
   x=[m['ns_per_operation']/1e6 for m in samples[e]];q=statistics.quantiles(x,n=4,method='inclusive');stats[e]={'median_ms':statistics.median(x),'q1_ms':q[0],'q3_ms':q[2]}
  r={'rows_per_strip':rows,'decoded_strip_kib':rows*3,'file':str(path.relative_to(ROOT)),'file_sha256':sha(path),'file_bytes':path.stat().st_size,'mode':mode,'correctness':correct,'samples':samples,'statistics':stats,'ratio':stats['verified']['median_ms']/stats['libtiff']['median_ms']};records.append(r)
  result={'metadata':{'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'main_results_sha256':sha(OUT/'results.json'),'script_sha256':sha(__file__),'cpu':cpu,'seed':20261016,'rounds':meta['rounds'],'milliseconds':meta['milliseconds'],'encoder_sha256':sha(OUT/'encode'),'native_pixels_sha256':native_hash},'records':records}
  (OUT/'layout-results.json').write_text(json.dumps(result,indent=2)+'\n');print(rows,mode,{e:round(s['median_ms'],3) for e,s in stats.items()},flush=True)
