#!/usr/bin/env python3
"""Check default strip chopping separately from the fixed-layout main suite.
Derive consumers by changing only the common open mode rmc -> rm. Original
main-suite consumer source and binaries remain unchanged and hash-checked.
"""
import hashlib,json,os,pathlib,random,statistics,subprocess,datetime
ROOT=pathlib.Path(__file__).resolve().parents[1];OUT=ROOT/'target/reader-benchmark'
def sha(p):return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def cmd(a):return subprocess.run(list(map(str,a)),capture_output=True,text=True,check=True,timeout=300).stdout
main=json.loads((OUT/'results.json').read_text());meta=main['metadata'];source=ROOT/'benchmarks/reader.c'
assert sha(source)==meta['benchmark_sha256']['benchmarks/reader.c']
assert sha(ROOT/'target/compat/libtiff.so.6')==meta['binary_sha256']['adapter_library']
for p,h in meta['source_sha256'].items():assert sha(ROOT/p)==h
text=source.read_text();needle='?"rmOc":"rmc"';assert text.count(needle)==1
derived=OUT/'reader-defaults.c';derived.write_text(text.replace(needle,'?"rmOc":"rm"'))
ref=OUT/'tiff-4.7.1';engines={'verified':OUT/'verified-defaults','libtiff':OUT/'reference-defaults'}
flags=['cc','-std=c11','-O3','-g','-Wall','-Wextra','-Werror']
commands=[flags+['-DVERIFIED_ADAPTER','-I'+str(ROOT/'compat/include'),derived,'-L'+str(ROOT/'target/compat'),'-Wl,-rpath,'+str(ROOT/'target/compat'),'-ltiff','-o',engines['verified']],
 [ref/'libtool','--mode=link','--tag=CC']+flags+['-I'+str(ref/'libtiff'),derived,ref/'libtiff/libtiff.la','-Wl,-rpath,'+str(ROOT/'target/lerc-reference'),'-Wl,-rpath,'+str(ROOT/'target/zstd-reference/lib'),'-o',engines['libtiff']]]
for command in commands:cmd(command)
os.sched_setaffinity(0,{meta['cpu']});records=[];rng=random.Random(20261017)
for case in json.loads((OUT/'inputs.json').read_text()):
 if not case['name'].startswith('large-strip-'):continue
 path=ROOT/case['file'];assert sha(path)==case['sha256']
 for mode in ['firstrow','crop']:
  parent=next(r for r in main['records'] if r['case']['name']==case['name'] and r['mode']==mode);correct={}
  for e,exe in engines.items():
   dump=OUT/(e+'-defaults.bin');m=json.loads(cmd([exe,path,mode,0,'memory',dump]));correct[e]={'sha256':sha(dump),'hash':m['hash']};assert correct[e]['sha256']==parent['correctness'][e]['sha256']
  samples={e:[] for e in engines};memory={e:[] for e in engines}
  for trial in range(meta['rounds']):
   order=list(engines);rng.shuffle(order)
   for e in order:
    m=json.loads(cmd([engines[e],path,mode,meta['milliseconds'],'memory']));assert m['hash']==correct[e]['hash'];samples[e].append(m)
  for trial in range(3):
   order=list(engines);rng.shuffle(order)
   for e in order:
    m=json.loads(cmd([engines[e],path,mode,0,'file']));assert m['hash']==correct[e]['hash'];memory[e].append(m)
  stats={}
  for e in engines:
   x=[m['ns_per_operation']/1e6 for m in samples[e]];q=statistics.quantiles(x,n=4,method='inclusive');stats[e]={'median_ms':statistics.median(x),'q1_ms':q[0],'q3_ms':q[2],'median_rss_mib':statistics.median(m['peak_rss_kib']/1024 for m in memory[e])}
  r={'case':case,'mode':mode,'correctness':correct,'samples':samples,'memory_samples':memory,'statistics':stats,'ratio':stats['verified']['median_ms']/stats['libtiff']['median_ms']};records.append(r)
  result={'metadata':{'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'main_results_sha256':sha(OUT/'results.json'),'script_sha256':sha(__file__),'derived_source_sha256':sha(derived),'source_change':{'from':needle,'to':'?"rmOc":"rm"'},'cpu':meta['cpu'],'seed':20261017,'rounds':meta['rounds'],'milliseconds':meta['milliseconds'],'binary_sha256':{e:sha(exe) for e,exe in engines.items()},'build_commands':[list(map(str,c)) for c in commands]},'records':records}
  (OUT/'defaults-results.json').write_text(json.dumps(result,indent=2)+'\n');print(case['name'],mode,{e:round(s['median_ms'],4) for e,s in stats.items()},flush=True)
