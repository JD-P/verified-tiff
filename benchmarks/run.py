#!/usr/bin/env python3
"""Reproducible, single-threaded in-memory decoder benchmark. No decoder edits.
Each timed batch runs inside a warmed process; Python startup/file loading,
fixture generation, output writing and correctness checks are not timed.
"""
import argparse,datetime,hashlib,json,math,os,pathlib,random,statistics,subprocess,sys
ROOT=pathlib.Path(__file__).resolve().parents[1]
OUT=ROOT/'target/benchmark'
sys.path.insert(0,str(ROOT/'tests'))

def command(args,**kw):
 p=subprocess.run(list(map(str,args)),capture_output=True,text=True,timeout=300,**kw)
 if p.returncode:raise RuntimeError(f'{args}: exit {p.returncode}\n{p.stderr[-4000:]}')
 return p.stdout

def generate():
 fixtures=OUT/'inputs';fixtures.mkdir(exist_ok=True)
 cases=[]
 def add(name,label,path,rgba=True,jpeg=False,origin='generated'):
  cases.append(dict(name=name,label=label,file=str(path.relative_to(ROOT)),rgba=rgba,jpeg=jpeg,origin=origin,input_bytes=path.stat().st_size,input_sha256=hashlib.sha256(path.read_bytes()).hexdigest()))
 configs=[
 ('gray-none','Gray8 / none',1,8,1,0,0,1),
 ('gray-packbits','Gray8 / PackBits',32773,8,1,0,0,1),
 ('gray-lzw','Gray8 / LZW',5,8,1,0,0,1),
 ('gray-deflate','Gray8 / Deflate',8,8,1,0,0,1),
 ('rgb-none','RGB8 / none',1,8,3,0,0,1),
 ('rgb-lzw','RGB8 / LZW',5,8,3,0,0,1),
 ('rgb-deflate','RGB8 / Deflate',8,8,3,0,0,1),
 ('rgb-jpeg','RGB8 / JPEG (smooth blocks)',7,8,3,0,0,1),
 ('rgb-jpeg-textured','RGB8 / JPEG (random texture)',7,8,3,1,0,1),
 ('rgb-tile-lzw','RGB8 / tiled LZW',5,8,3,0,1,1),
 ('gray16-deflate','Gray16 / Deflate',8,16,1,0,0,1),
 ('float64-lzw-p2','Float64 / LZW predictor 2',5,64,1,0,0,2),
 ('noise-lzw','Random Gray8 / LZW',5,8,1,1,0,1),
 ('noise-deflate','Random Gray8 / Deflate',8,8,1,1,0,1),
 ('fax3','Bilevel / CCITT Group 3',3,1,1,0,0,1),
 ('fax4','Bilevel / CCITT Group 4',4,1,1,0,0,1)]
 for name,label,*args in configs:
  path=fixtures/(name+'.tif');command([OUT/'encode',path,*args]);add(name,label,path,rgba=args[1]!=64,jpeg=args[0]==7)
 from webp_fixtures import encode,wrap
 w=h=1024
 raw=bytes(v for y in range(h) for x in range(w) for v in ((x//64%4)*60,(y//64%4)*60,((x//64+y//64)%4)*60))
 blob=encode(raw,w,ch=3,subtract=True,cache=4,copies=True)
 path=fixtures/'webp.tif';path.write_bytes(wrap(blob,w,h,ch=3,alpha=0));add('webp','RGB8 / lossless WebP subset',path)
 from logluv_fixtures import make
 data,_,_=make(w=512,h=512,rows=64,pattern='constant')
 path=fixtures/'logluv.tif';path.write_bytes(data);add('logluv','Native LogLuv16 / SGILog32',path,rgba=False)
 for name,label in [('quad-lzw-compat.tiff','Original legacy LZW'),('lzw-single-strip.tiff','Original 24.3 MP LZW scan'),('testfax4.tiff','Original 8.3 MP Group 4 scan'),('deflate-last-strip-extra-data.tiff','Original Deflate final-strip case')]:
  path=ROOT/'.tools/tiff-4.7.1/test/images'/name;add('original-'+name,label,path,origin='libtiff 4.7.1 test/images, unchanged')
 (OUT/'inputs.json').write_text(json.dumps(cases,indent=2)+'\n')
 return cases

def engine_paths():
 return {'rust_inspect':(ROOT/'target/release/examples/benchmark','inspect'),'rust_decode':(ROOT/'target/release/examples/benchmark','decode'),'rust_total':(ROOT/'target/release/examples/benchmark','total'),
 'libtiff_decode':(OUT/'libtiff','decode'),'libtiff_total':(OUT/'libtiff','total'),'libtiff_rgba':(OUT/'libtiff','rgba'),'adapter_rgba':(OUT/'adapter','rgba'),'system_rgba':(OUT/'system','rgba')}

def main():
 parser=argparse.ArgumentParser();parser.add_argument('--generate',action='store_true');parser.add_argument('--generate-only',action='store_true');parser.add_argument('--validate-only',action='store_true');parser.add_argument('--rounds',type=int,default=7);parser.add_argument('--milliseconds',type=float,default=80);parser.add_argument('--case');parser.add_argument('--resume',action='store_true');args=parser.parse_args()
 cases=generate() if args.generate or args.generate_only or not (OUT/'inputs.json').exists() else json.loads((OUT/'inputs.json').read_text())
 if args.generate_only:return
 if args.case:cases=[c for c in cases if args.case in c['name']]
 available=os.sched_getaffinity(0);cpu=min(available);os.sched_setaffinity(0,{cpu})
 engines=engine_paths()
 def run(case,engine,ms,dump=None):
  exe,mode=engines[engine];argv=[exe,ROOT/case['file'],mode,ms]+([dump] if dump else [])
  return json.loads(command(argv))
 metadata={'timestamp_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'cpu_affinity':[cpu],'cpu_model':next(s.split(':',1)[1].strip() for s in pathlib.Path('/proc/cpuinfo').read_text().splitlines() if s.startswith('model name')),
 'uname':command(['uname','-a']).strip(),'compiler':command(['cc','--version']).splitlines()[0], 'pinned_libtiff':command([OUT/'libtiff','--version']).strip(),'system_libtiff':command([OUT/'system','--version']).strip(),
 'rounds':args.rounds,'target_batch_ms':args.milliseconds,'shuffle_seed':20261014,'input_output_limit_bytes':268435456,'rust_and_adapter_work_budget':1000000000000,'pinned_jpeg_simd':False,
 'binary_sha256':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in {v[0] for v in engines.values()}},
 'core_source_sha256':json.loads((ROOT/'docs/evidence/verification-summary.json').read_text())['core_sources_sha256']}
 for path,digest in metadata['core_source_sha256'].items():assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==digest
 records=[]
 if args.resume and (OUT/'results.json').exists():
  previous=json.loads((OUT/'results.json').read_text())
  for key in ['rounds','target_batch_ms','binary_sha256','core_source_sha256','cpu_affinity']:
   assert previous['metadata'][key]==metadata[key],('resume mismatch',key)
  records=previous['cases'];metadata=previous['metadata']
 completed={r['name'] for r in records}
 for case in cases:
  if case['name'] in completed:continue
  record=dict(case,samples={},correctness={});rows=[]
  active=[e for e in engines if case['rgba'] or not e.endswith('rgba')]
  for engine in ['rust_decode','libtiff_decode']+(['libtiff_rgba','adapter_rgba','system_rgba'] if case['rgba'] else []):
   output=OUT/(engine+'.raw');measurement=run(case,engine,0,output);rows.append((engine,output.read_bytes(),measurement))
   record.setdefault('measurements',{})[engine]=measurement
  native=rows[0][1]
  for engine,raw,m in rows[1:]:
   reference=native if engine=='libtiff_decode' else rows[2][1]
   assert len(raw)==len(reference),(case['name'],engine,len(raw),len(reference))
   if raw==reference:check={'exact':True,'max_abs_error':0}
   else:
    import numpy as np
    diff=np.abs(np.frombuffer(raw,dtype=np.uint8).astype(np.int16)-np.frombuffer(reference,dtype=np.uint8).astype(np.int16))
    check={'exact':False,'max_abs_error':int(diff.max()),'mean_abs_error':float(diff.mean())}
    assert case['jpeg'] and check['max_abs_error']<=3,(case['name'],engine,check)
   record['correctness'][engine]=check
  record['width']=rows[0][2]['width'];record['height']=rows[0][2]['height'];record['decoded_bytes']=len(native)
  if args.validate_only:
   print(case['name'],'OK',record['width'],record['height'],record['correctness'],flush=True)
   records.append(record);continue
  rng=random.Random(20261014)
  for trial in range(args.rounds):
   order=active.copy();rng.shuffle(order)
   for engine in order:
    m=run(case,engine,args.milliseconds)
    assert m['hash']==record['measurements']['rust_decode' if engine.startswith('rust_') else 'libtiff_decode' if engine.startswith('libtiff_') and engine!='libtiff_rgba' else engine]['hash'],(case['name'],engine,'unstable output')
    record['samples'].setdefault(engine,[]).append(m)
  record['statistics']={}
  for engine,samples in record['samples'].items():
   values=[s['ns_per_image']/1e6 for s in samples];q=statistics.quantiles(values,n=4,method='inclusive')
   record['statistics'][engine]={'median_ms':statistics.median(values),'min_ms':min(values),'max_ms':max(values),'p25_ms':q[0],'p75_ms':q[2],'megapixels_per_second':record['width']*record['height']/1000/statistics.median(values)}
  print(case['name'],{e:round(s['median_ms'],3) for e,s in record['statistics'].items()},flush=True)
  records.append(record)
  (OUT/'results.json').write_text(json.dumps(dict(metadata=metadata,cases=records),indent=2)+'\n')
 if args.validate_only:(OUT/'correctness.json').write_text(json.dumps(dict(metadata=metadata,cases=records),indent=2)+'\n')

if __name__=='__main__':main()
