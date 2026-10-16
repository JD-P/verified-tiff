#!/usr/bin/env python3
"""Enumerate the pinned Linux public header and stub every missing function.
Never infer success from a stub: ENOSYS + diagnostic, API failure sentinel,
and no caller-output writes. The inventory is checked against exported symbols.
"""
import json,pathlib,re,subprocess,sys
root=pathlib.Path(__file__).resolve().parent
out=pathlib.Path(sys.argv[1]);out.mkdir(parents=True,exist_ok=True)
cc=sys.argv[2] if len(sys.argv)>2 else 'cc'
text=subprocess.check_output([cc,'-E','-P','-I'+str(root/'include'),'-xc','-'],input=b'#include <tiffio.h>\n').decode()
# Remove GNU attributes with balanced parentheses.
while '__attribute__' in text:
 a=text.index('__attribute__');b=text.index('(',a);depth=0
 for e in range(b,len(text)):
  if text[e]=='(':depth+=1
  elif text[e]==')':
   depth-=1
   if not depth:break
 text=text[:a]+text[e+1:]
prototypes={}
for match in re.finditer(r'\bextern\s+([^;{}]+?)\b((?:TIFF|_TIFF|LogL|XYZtoRGB24|uv_)[A-Za-z_0-9]*)\s*\(([^;{}]*)\)\s*;',text):
 ret,name,args=match.groups();ret=' '.join(ret.split());args=' '.join(args.split());prototypes[name]=(ret,args)
assert 'TIFFOpen' in prototypes and 'TIFFWriteEncodedStrip' in prototypes
nm=subprocess.check_output(['nm','--defined-only',str(out/'tiff_compat.o')],text=True)
implemented=set(re.findall(r'\b[TW]\s+(\w+)',nm))
lines=['/* Generated from the pinned public Linux header; see api.json. */','#include "bridge.h"','#include <math.h>']
manifest=[]
stub_tests=['#include <tiffio.h>','#include <verified_tiff.h>','#include <assert.h>','#include <errno.h>','#include <math.h>','int main(void){TIFFSetErrorHandler(NULL);']
manual_stubs=set()
negative_int={'TIFFWriteScanline','uv_encode','uv_decode','LogL16fromY','LogL10fromY'}
for name,(ret,args) in sorted(prototypes.items()):
 impl=name in implemented
 record=dict(name=name,result=ret,arguments=args,status='implemented_or_explicitly_restricted' if impl and name not in manual_stubs else 'not_implemented',implementation='manual_stub' if name in manual_stubs else 'manual' if impl else 'generated_stub')
 if not impl:
  parts=[];names=[]
  if args and args!='void':
   for i,arg in enumerate(args.split(',')):
    arg=arg.strip()
    if arg=='...':parts.append(arg);continue
    # Declarations contain simple types/typedefs, pointers and optional arrays.
    arr=''
    if '[' in arg:arg,arr=arg.split('[',1);arr='['+arr
    m=re.search(r'\b([A-Za-z_]\w*)\s*$',arg)
    # Last identifier is a name iff there is a separate type before it.
    token=m.group(1) if m else ''
    base=arg[:m.start()].strip() if m else ''
    if base not in ('','const','volatile','const volatile') and token not in {'void','char','int','long','short','float','double'}:arg=base
    param=f'p{i}';parts.append(arg.strip()+' '+param+arr);names.append(param)
  signature=ret+' '+name+'('+(', '.join(parts) if parts else 'void')+')'
  if ret=='void':value=None
  elif '*' in ret or ret.endswith('Proc') or ret.endswith('Handler') or ret.endswith('HandlerExt') or ret=='TIFFExtendProc':value='NULL'
  elif ret in ('double','float'):value='NAN'
  elif ret=='tmsize_t' or name in negative_int:value='-1'
  else:value='0'
  diagnostic=('vt_not_implemented_for(p0,"'+name+'");') if parts and re.match(r'TIFF\s*\*',parts[0]) else ('vt_not_implemented("'+name+'");')
  body=''.join('(void)'+n+';' for n in names)+diagnostic+('return '+value+';' if value is not None else '')
  lines.append(signature+' {'+body+'}')
  call=name+'('+','.join('NULL' if '*' in part or '[' in part else '0' for part in parts if part!='...')+')'
  stmt=(call+';' if value is None else 'assert(isnan('+call+'));' if value=='NAN' else 'assert('+call+'=='+value+');')
  stub_tests.append('VTIFFClearError();errno=0;'+stmt+'assert(errno==ENOSYS);assert(VTIFFLastErrorCode()==ENOSYS);')
  record['failure_return']=value if value is not None else 'void (diagnostic only)'
 manifest.append(record)
stub_tests.append('return 0;}')
(out/'stub_smoke.c').write_text('\n'.join(stub_tests)+'\n')
(out/'stubs.c').write_text('\n'.join(lines)+'\n')
(out/'api.json').write_text(json.dumps(dict(target='libtiff 4.7.1 public Linux tiffio.h',functions=manifest),indent=2)+'\n')
# Preserve upstream symbol versions for public symbols; no private/internal ABI.
maptext=(root/'upstream-libtiff.map').read_text()
allnames=set(prototypes);seen=set();version_lines=[]
for block in re.finditer(r'(LIBTIFF_[\w.]+)\s*\{(.*?)\}\s*(LIBTIFF_[\w.]+)?;',maptext,re.S):
 version,body,parent=block.groups();names=[n for n in re.findall(r'^\s*([A-Za-z_]\w*)\s*;',body,re.M) if n in allnames];seen.update(names)
 if version=='LIBTIFF_4.0':
  # Header conversion routines are not present in upstream's version map.
  names+=sorted(allnames-set(re.findall(r'^\s*([A-Za-z_]\w*)\s*;',maptext,re.M)))
  names+=['VTIFFLastErrorCode','VTIFFLastError','VTIFFClearError','VTIFFOpenOptionsSetLimits']
 version_lines.append(version+' {\n'+('  global:\n'+''.join('    '+n+';\n' for n in names) if names else '')+('  local: *;\n' if version=='LIBTIFF_4.0' else '')+'}'+(' '+parent if parent else '')+';\n')
(out/'exports.map').write_text('\n'.join(version_lines))
print(f'Public API: {len(prototypes)} symbols; {sum(r["status"]!="not_implemented" for r in manifest)} implemented/restricted, {sum(r["status"]=="not_implemented" for r in manifest)} explicit ENOSYS stubs ({sum(r["implementation"]=="generated_stub" for r in manifest)} generated, {len(manual_stubs)} manual)')
