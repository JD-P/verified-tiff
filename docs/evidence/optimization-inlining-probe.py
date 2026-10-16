import json,pathlib,subprocess,os,random,statistics
r=pathlib.Path('.').resolve();os.sched_setaffinity(0,{min(os.sched_getaffinity(0))})
roots={'old':r/'target/baseline/verified-tiff','current':r,'variant':r/'target/variant-no-inline'}
cases=json.load(open('target/benchmark/inputs.json'));records=[]
for c in cases:
 if c['name'] not in ['gray-packbits','gray-lzw','rgb-lzw','rgb-tile-lzw','noise-lzw']:continue
 rec={'name':c['name'],'samples':{},'hashes':{}}
 engines={key+'-'+mode:(root/'target/release/examples/benchmark' if mode=='total' else root/'target/benchmark/adapter',mode) for key,root in roots.items() for mode in ['total','rgba']}
 rng=random.Random(42)
 for _ in range(5):
  order=list(engines);rng.shuffle(order)
  for e in order:
   exe,mode=engines[e];p=subprocess.run([exe,r/c['file'],mode,'160'],capture_output=True,text=True,check=True)
   m=json.loads(p.stdout);rec['samples'].setdefault(e,[]).append(m['ns_per_image']/1e6)
   rec['hashes'].setdefault(mode,set()).add(m['hash'])
 assert all(len(s)==1 for s in rec['hashes'].values())
 rec['hashes']={k:list(v) for k,v in rec['hashes'].items()};rec['medians']={e:statistics.median(s) for e,s in rec['samples'].items()}
 print(c['name'],rec['medians'],flush=True);records.append(rec)
 pathlib.Path('target/probe-inlining.json').write_text(json.dumps(records,indent=2)+'\n')
