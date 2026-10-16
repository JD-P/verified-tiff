#!/usr/bin/env python3
"""Current C ABI vs pinned libtiff 4.7.1. No production source modifications.
Build dependencies/consumers before running. Preflight each complete output,
then randomize seven serial process batches per engine/workload on one CPU.
"""
import argparse,datetime,hashlib,json,os,pathlib,random,statistics,subprocess,sys,time
ROOT=pathlib.Path(__file__).resolve().parents[1];OUT=ROOT/'target/reader-benchmark'
sys.path[:0]=[str(ROOT/'compat/tests'),str(ROOT/'tests')]
def sha(p):return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def cmd(argv,timeout=300):
 r=subprocess.run(list(map(str,argv)),capture_output=True,text=True,timeout=timeout)
 if r.returncode:raise RuntimeError(f'{argv}: {r.returncode}\n{r.stderr[-3000:]}')
 return r.stdout

def generate():
 d=OUT/'inputs';d.mkdir(exist_ok=True);cases=[]
 def add(name,label,path,modes,origin='generated',**extra):
  cases.append(dict(name=name,label=label,file=str(path.relative_to(ROOT)),modes=modes,origin=origin,bytes=path.stat().st_size,sha256=sha(path),**extra))
 def make(name,label,codec=1,bits=8,ch=1,noise=0,tiled=0,pred=1,w=1024,h=1024,rows=64,fmt=1,photo=None,wrapper=0,lossless=1,modes=None):
  if photo is None:photo=0 if bits==1 else 2 if ch>=3 else 1
  if modes is None:modes=['native','rgba'] if fmt==1 else ['native']
  p=d/(name+'.tif');cmd([OUT/'encode',p,codec,bits,ch,noise,tiled,pred,w,h,rows,fmt,photo,wrapper,lossless]);add(name,label,p,modes,codec=codec,tiled=bool(tiled),w=w,h=h,jpeg=codec==7)
 for codec,name in [(1,'none'),(32773,'packbits'),(5,'lzw'),(8,'deflate'),(50000,'zstd')]:
  make('gray-'+name,'Gray8 '+name,codec=codec,modes=['native','scan','rgba'])
  if codec!=32773:make('rgb-'+name,'RGB8 '+name,codec=codec,ch=3,modes=['native','rgba','crop'])
 for codec,name in [(5,'lzw'),(8,'deflate'),(50000,'zstd')]:make('noise-'+name,'Random Gray8 '+name,codec=codec,noise=1,modes=['native'])
 for noise in [0,1]:make('jpeg-'+('texture' if noise else 'smooth'),'RGB8 JPEG '+('texture' if noise else 'smooth'),codec=7,ch=3,noise=noise)
 make('rgb-tiled-lzw','RGB8 tiled LZW',codec=5,ch=3,tiled=1,modes=['native','rgba','crop'])
 make('gray16-deflate-p2','Gray16 Deflate Predictor 2',codec=8,bits=16,pred=2,modes=['native','scan'])
 make('float32-deflate-p3','Float32 Deflate Predictor 3',codec=8,bits=32,fmt=3,pred=3,modes=['native','scan'])
 make('float16-deflate-p3','Float16 Deflate Predictor 3',codec=8,bits=16,fmt=3,pred=3,modes=['native','scan'])
 make('complex64-lzw','Complex float32 pairs LZW',codec=5,bits=64,fmt=6,modes=['native','scan'])
 for codec,name in [(2,'rle'),(3,'g3'),(4,'g4')]:make('fax-'+name,'Bilevel CCITT '+name,codec=codec,bits=1)
 for lossless,name in [(1,'lossless'),(0,'lossy')]:make('webp-'+name,'RGBA8 WebP '+name,codec=50001,ch=4,lossless=lossless)
 for wrapper,name in [(0,'plain'),(1,'deflate'),(2,'zstd')]:make('lerc-float-'+name,'Float32 LERC '+name,codec=34887,bits=32,fmt=3,tiled=1,wrapper=wrapper)
 for photo,name,ch in [(5,'cmyk',4),(8,'lab',3)]:make(name+'-deflate',name.upper()+'8 Deflate',codec=8,ch=ch,photo=photo,modes=['rgba','crop'])
 # Large complete strips stress initial row/crop validation and bounded memory.
 for codec,name in [(1,'none'),(5,'lzw'),(8,'deflate')]:make('large-strip-'+name,'16 MP Gray8 single strip '+name,codec=codec,w=4096,h=4096,rows=4096,modes=['firstrow','crop'])
 from coverage import assemble
 n=200000;p=d/'many-strips.tif';p.write_bytes(assemble([[(256,4,[1]),(257,4,[n]),(258,3,[8]),(259,3,[1]),(262,3,[1]),(273,4,('SEG',[b'I']*n)),(277,3,[1]),(278,4,[1]),(279,4,[1]*n)]],dedup=True)[0]);add('many-strips','200,000 strip entries',p,['open','lookup','open-lazy','lookup-lazy'])
 for name,label in [('lzw-single-strip.tiff','Original 24.3 MP LZW scan'),('testfax4.tiff','Original 8.3 MP fax scan'),('quad-lzw-compat.tiff','Original legacy LZW')]:
  p=ROOT/'.tools/tiff-4.7.1/test/images'/name;add('original-'+name,label,p,['native','rgba'],origin='libtiff 4.7.1 original corpus')
 (OUT/'inputs.json').write_text(json.dumps(cases,indent=2)+'\n');return cases

def main():
 p=argparse.ArgumentParser();p.add_argument('--generate',action='store_true');p.add_argument('--generate-only',action='store_true');p.add_argument('--validate-only',action='store_true');p.add_argument('--resume',action='store_true');p.add_argument('--rounds',type=int,default=7);p.add_argument('--milliseconds',type=float,default=100);a=p.parse_args()
 cases=generate() if a.generate or a.generate_only or not (OUT/'inputs.json').exists() else json.loads((OUT/'inputs.json').read_text())
 if a.generate_only:return
 cpu=min(os.sched_getaffinity(0));os.sched_setaffinity(0,{cpu});engines={'verified':OUT/'verified','libtiff':OUT/'reference'}
 proof=json.loads((ROOT/'docs/evidence/reader-verification-summary.json').read_text())
 for n,h in proof['core_sources_sha256'].items():assert sha(ROOT/n)==h,n
 meta={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'cpu':cpu,'cpu_model':next(l.split(':',1)[1].strip() for l in pathlib.Path('/proc/cpuinfo').read_text().splitlines() if l.startswith('model name')),'uname':cmd(['uname','-a']).strip(),'cc':cmd(['cc','--version']).splitlines()[0],'versions':{e:cmd([exe,'--version']).strip() for e,exe in engines.items()},'rounds':a.rounds,'milliseconds':a.milliseconds,'seed':20261015,'timing_io':'memory callbacks, no mmap; fresh handle per operation; input loading/output allocation/hash excluded','rss_io':'separate fresh process, file-backed pread/read, mmap disabled; output allocation included','jpeg_simd':False,'source_sha256':{str(p.relative_to(ROOT)):sha(p) for d in ['src','compat'] for p in sorted((ROOT/d).rglob('*')) if p.is_file() and p.suffix in ['.rs','.c','.h','.inc']},'benchmark_sha256':{str(p.relative_to(ROOT)):sha(p) for p in [ROOT/'benchmarks/reader.c',ROOT/'benchmarks/reader-encode.c',ROOT/'benchmarks/reader-bench.py']},'binary_sha256':{e:sha(p) for e,p in engines.items()}|{'adapter_library':sha(ROOT/'target/compat/libtiff.so.6'),'reference_library':sha(OUT/'tiff-4.7.1/libtiff/.libs/libtiff.a')},'reference_linkage':cmd(['ldd',OUT/'reference']).strip(),'adapter_linkage':cmd(['ldd',ROOT/'target/compat/libtiff.so.6']).strip(),'reference_config_sha256':sha(OUT/'tiff-4.7.1/libtiff/tif_config.h'),'reference_configure_log_sha256':sha(OUT/'configure.log'),'codec_library_sha256':{str(p.relative_to(ROOT)):sha(p) for p in [ROOT/'.tools/jpeg/lib/libjpeg.a',ROOT/'.tools/webp/lib/libwebp.a',ROOT/'target/lerc-reference/libLerc.so',ROOT/'target/zstd-reference/lib/libzstd.so']},'toolchain':json.loads((ROOT/'toolchain-lock.json').read_text()),'source_archive_sha256':sha(ROOT.parent/'output/verified-tiff-0.1.0.tar')}
 records=[];destination=OUT/('preflight.json' if a.validate_only else 'results.json')
 if a.resume and destination.exists():
  previous=json.loads(destination.read_text())
  for key in ['rounds','milliseconds','cpu','source_sha256','benchmark_sha256','binary_sha256']:assert previous['metadata'][key]==meta[key],key
  records=previous['records'];meta=previous['metadata']
 done={(r['case']['name'],r['mode']) for r in records};rng=random.Random(meta['seed'])
 def run(case,mode,e,ms,io='memory',dump=None):return json.loads(cmd([engines[e],ROOT/case['file'],mode,ms,io]+([dump] if dump else [])))
 for case in cases:
  assert sha(ROOT/case['file'])==case['sha256']
  for mode in case['modes']:
   if (case['name'],mode) in done:continue
   record={'case':case,'mode':mode,'correctness':{},'samples':{e:[] for e in engines},'memory_samples':{e:[] for e in engines}}
   raw={};checks={}
   for e in engines:
    path=OUT/(e+'.bin');checks[e]=run(case,mode,e,0,dump=path);raw[e]=path.read_bytes();record['correctness'][e]={'sha256':sha(path),'hash':checks[e]['hash'],'bytes':len(raw[e])}
   assert len(raw['verified'])==len(raw['libtiff']),(case['name'],mode,'length')
   if raw['verified']!=raw['libtiff']:
    import numpy as np
    diff=abs(np.frombuffer(raw['verified'],dtype=np.uint8).astype(np.int16)-np.frombuffer(raw['libtiff'],dtype=np.uint8).astype(np.int16));maximum=int(diff.max());record['difference']={'max_channel_error':maximum,'mean_channel_error':float(diff.mean())}
    assert case.get('jpeg') and maximum<=3,(case['name'],mode,record['difference'])
   else:record['difference']={'max_channel_error':0,'mean_channel_error':0}
   if not a.validate_only:
    for trial in range(a.rounds):
     order=list(engines);rng.shuffle(order)
     for e in order:
      m=run(case,mode,e,a.milliseconds);assert m['hash']==checks[e]['hash'];record['samples'][e].append(m)
    for trial in range(3):
     order=list(engines);rng.shuffle(order)
     for e in order:
      m=run(case,mode,e,0,'file');assert m['hash']==checks[e]['hash'];record['memory_samples'][e].append(m)
    record['statistics']={}
    for e in engines:
     values=[m['ns_per_operation']/1e6 for m in record['samples'][e]];q=statistics.quantiles(values,n=4,method='inclusive');rss=[m['peak_rss_kib']/1024 for m in record['memory_samples'][e]]
     record['statistics'][e]={'median_ms':statistics.median(values),'q1_ms':q[0],'q3_ms':q[2],'min_ms':min(values),'max_ms':max(values),'median_rss_mib':statistics.median(rss),'min_rss_mib':min(rss),'max_rss_mib':max(rss)}
    record['ratio']=record['statistics']['verified']['median_ms']/record['statistics']['libtiff']['median_ms']
   records.append(record);destination.write_text(json.dumps({'metadata':meta,'records':records},indent=2)+'\n')
   print(case['name'],mode,'OK' if a.validate_only else {e:round(s['median_ms'],4) for e,s in record['statistics'].items()},flush=True)
 print('Completed',len(records),'workloads',flush=True)
if __name__=='__main__':main()
