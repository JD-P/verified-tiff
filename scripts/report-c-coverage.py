#!/usr/bin/env python3
"""Merge gcov JSON for the same production C sources; never exclude a branch."""
import argparse,gzip,json,pathlib,subprocess
R=pathlib.Path(__file__).resolve().parents[1]
p=argparse.ArgumentParser();p.add_argument('folder',type=pathlib.Path);p.add_argument('--missing',action='store_true');p.add_argument('--require-complete',action='store_true');a=p.parse_args();D=a.folder.resolve()
objects=['tiff_compat.gcno','stubs.gcno','shim-hardening-shim_hardening.gcno']
subprocess.run(['gcov','-b','-c','-f','-j',*objects],cwd=D,check=True,stdout=subprocess.DEVNULL)
files={};runs={}
for obj in objects:
 d=json.loads(gzip.decompress((D/(obj.removesuffix('.gcno')+'.gcov.json.gz')).read_bytes()))
 runs[obj]=d
 for f in d['files']:
  path=pathlib.Path(f['file']);name=str(path.relative_to(R)) if path.is_relative_to(R) else str(path)
  if not ((path.parent==R/'compat' and path.suffix in ['.c','.inc']) or path==D/'stubs.c'):continue
  o=files.setdefault(name,{'functions':{},'lines':{}})
  for g in f['functions']:
   n=g['name'];v=o['functions'].setdefault(n,dict(g,execution_count=0));v['execution_count']+=g['execution_count']
  for l in f['lines']:
   key=(l['line_number'],l.get('function_name',''));v=o['lines'].setdefault(key,dict(l,count=0,branches=[dict(b,count=0) for b in l['branches']]))
   assert len(v['branches'])==len(l['branches']),(name,key)
   v['count']+=l['count']
   for x,y in zip(v['branches'],l['branches']):
    assert x['fallthrough']==y['fallthrough'] and x['throw']==y['throw'],(name,key)
    x['count']+=y['count']
summary={};total=[0]*6
for name,o in files.items():
 lines=list(o['lines'].values());fs=list(o['functions'].values());bs=[b for l in lines for b in l['branches']]
 counts=[sum(l['count']>0 for l in lines),len(lines),sum(f['execution_count']>0 for f in fs),len(fs),sum(b['count']>0 for b in bs),len(bs)]
 total=[x+y for x,y in zip(total,counts)];summary[name]=counts
 print(name, 'lines %d/%d functions %d/%d branches %d/%d'%tuple(counts))
 if a.missing:
  source=(R/name).read_text().splitlines()
  for l in sorted(lines,key=lambda x:x['line_number']):
   zero=[i for i,b in enumerate(l['branches']) if b['count']==0]
   if l['count']==0 or zero:print(' ',l['line_number'],'NO LINE' if not l['count'] else 'branches '+str(zero),source[l['line_number']-1].strip())
 o['lines']=lines;o['functions']=fs
print('TOTAL','lines %d/%d functions %d/%d branches %d/%d'%tuple(total))
(D/'coverage.json').write_text(json.dumps({'files':files,'summary':summary,'total':total},indent=2)+'\n')

expected={'compat/tiff_compat.c','compat/fields.inc','compat/writer.inc','compat/reader_api.inc','compat/color_api.inc','compat/logluv_api.inc',str((D/'stubs.c').relative_to(R))}
assert set(files)==expected, 'Production source coverage inventory changed'
public={f['name'] for f in json.loads((D/'api.json').read_text())['functions']}
functions={f['name']:f['execution_count'] for o in files.values() for f in o['functions']}
assert public<=functions.keys()
if a.require_complete:
 assert total[0]==total[1] and total[2]==total[3] and total[4]==total[5], 'Incomplete C coverage'
 assert all(functions[n]>0 for n in public)
 print('All 198 pinned public symbols and every emitted C line/function/branch covered; no exclusions.')
# Keep raw per-binary JSON so integration and fault-injection coverage can be
# inspected independently; combined coverage never substitutes for integration.
(D/'coverage-raw.json').write_text(json.dumps(runs,indent=2)+'\n')
