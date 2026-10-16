#!/usr/bin/env python3
"""Optional installed ImageMagick, Leptonica, and GdkPixbuf read-only smoke.
No packages are installed. Each library load runs in a fresh subprocess.
"""
import ctypes as C, ctypes.util, hashlib, json, os, pathlib, re, shutil, subprocess, sys, tempfile
ROOT=pathlib.Path(__file__).resolve().parents[2]
OUT=pathlib.Path(os.environ.get('VTIFF_COMPAT_OUTPUT',ROOT/'target/compat')).resolve()
def child(engine,path):
 if engine=='lept':
  lib=C.CDLL(ctypes.util.find_library('lept'));lib.pixReadTiff.argtypes=[C.c_char_p,C.c_int];lib.pixReadTiff.restype=C.c_void_p
  p=lib.pixReadTiff(os.fsencode(path),0)
  if not p:raise RuntimeError('Leptonica did not decode')
  for name in ['pixGetWidth','pixGetHeight','pixGetDepth']:
   fn=getattr(lib,name);fn.argtypes=[C.c_void_p];fn.restype=C.c_int
  w,h,d=lib.pixGetWidth(p),lib.pixGetHeight(p),lib.pixGetDepth(p)
  lib.pixGetPixel.argtypes=[C.c_void_p,C.c_int,C.c_int,C.POINTER(C.c_uint32)];raw=bytearray()
  for y in range(h):
   for x in range(w):
    v=C.c_uint32();assert lib.pixGetPixel(p,x,y,C.byref(v))==0;raw.extend(v.value.to_bytes(4,'little'))
  lib.pixDestroy.argtypes=[C.POINTER(C.c_void_p)];ptr=C.c_void_p(p);lib.pixDestroy(C.byref(ptr));shape=[w,h,d]
 else:
  lib=C.CDLL(ctypes.util.find_library('gdk_pixbuf-2.0'));lib.gdk_pixbuf_new_from_file.argtypes=[C.c_char_p,C.POINTER(C.c_void_p)];lib.gdk_pixbuf_new_from_file.restype=C.c_void_p
  error=C.c_void_p();p=lib.gdk_pixbuf_new_from_file(os.fsencode(path),C.byref(error))
  if not p:raise RuntimeError('GdkPixbuf did not decode')
  def get(name):
   f=getattr(lib,'gdk_pixbuf_get_'+name);f.argtypes=[C.c_void_p];f.restype=C.c_int;return f(p)
  w,h,ch,stride=get('width'),get('height'),get('n_channels'),get('rowstride')
  lib.gdk_pixbuf_get_pixels.argtypes=[C.c_void_p];lib.gdk_pixbuf_get_pixels.restype=C.c_void_p;base=lib.gdk_pixbuf_get_pixels(p)
  raw=b''.join(C.string_at(base+y*stride,w*ch) for y in range(h));shape=[w,h,ch]
  obj=C.CDLL(ctypes.util.find_library('gobject-2.0'));obj.g_object_unref.argtypes=[C.c_void_p];obj.g_object_unref(p)
 maps=pathlib.Path('/proc/self/maps').read_text();paths=sorted({line.split()[-1] for line in maps.splitlines() if '/libtiff.so' in line})
 print(json.dumps(dict(shape=shape,sha256=hashlib.sha256(raw).hexdigest(),libtiff=paths)))
def main():
 from coverage import scene
 from fax_fixtures import one_d,pack
 from coverage import assemble
 records=[];engines=[]
 if shutil.which('convert'):engines.append('imagemagick')
 if ctypes.util.find_library('lept'):engines.append('lept')
 if ctypes.util.find_library('gdk_pixbuf-2.0'):engines.append('gdk')
 with tempfile.TemporaryDirectory() as tmp:
  folder=pathlib.Path(tmp);fixtures={}
  for name,kw in [('zstd_rgb',dict(codec=50000)),('float_zstd',dict(codec=50000,ch=1,bits=32,fmt=3,predict=3)),('rgb',{}),('rgba',dict(ch=4,alpha=2)),('gray16',dict(ch=1,bits=16)),('cmyk',dict(ch=4,photo=5)),('lab',dict(ch=3,photo=8)),('float32',dict(ch=1,bits=32,fmt=3)),('float_predictor3',dict(ch=1,bits=32,fmt=3,predict=3))]:
   p=folder/(name+'.tif');p.write_bytes(scene(w=17,h=19,**{'codec':8,**kw})[0]);fixtures[name]=p
  for name,file in [('group4','fax-g4-witness.tif'),('rle',None)]:
   if file:fixtures[name]=ROOT/'tests/fixtures'/file
   else:
    tags=scene(w=17,h=19,ch=1,bits=1,photo=0)[3];segments=[]
    for key,ty,v in tags:
     if key==273:
      for raw in v[1]:segments.append(b''.join(pack(one_d([(raw[y*3+x//8]>>(7-x%8))&1 for x in range(17)])) for y in range(len(raw)//3)))
    tags=[(key,ty,[2] if key==259 else ('SEG',segments) if key==273 else list(map(len,segments)) if key==279 else v) for key,ty,v in tags]
    p=folder/'rle.tif';p.write_bytes(assemble([tags])[0]);fixtures[name]=p
  large=folder/'large.tif';shutil.copyfile(fixtures['rgb'],large)
  with large.open('ab') as f:f.truncate(65*1024*1024)
  fixtures['largefile']=large
  for engine in engines:
   for name,p in fixtures.items():
    if engine!='imagemagick' and name.startswith('float'):continue
    results=[]
    for ours in [False,True]:
     env=dict(os.environ)
     if ours:env['LD_LIBRARY_PATH']=str(OUT)
     else:env.pop('LD_LIBRARY_PATH',None)
     if ours and engine=='imagemagick':env['LD_DEBUG']='bindings'
     command=['convert',str(p),'-depth','8','RGBA:-'] if engine=='imagemagick' else [sys.executable,__file__,'--child',engine,str(p)]
     r=subprocess.run(command,env=env,capture_output=True,timeout=30)
     stderr=r.stderr.decode(errors='replace')
     binding=None
     if ours and engine=='imagemagick':
      binding=any(str(OUT)+'/libtiff.so.6' in line and re.search(r'TIFF(?:ClientOpen|Open|ReadEncoded|ReadScanline|ReadRGBA)',line) for line in stderr.splitlines())
      assert binding,'ImageMagick did not bind TIFF reader symbols to the adapter'
      stderr='\n'.join(line for line in stderr.splitlines() if not re.match(r'^\s*\d+:',line))
     result=dict(exit=r.returncode,stderr=stderr)
     if binding is not None:result['adapter_binding_confirmed']=binding
     if engine=='imagemagick':result.update(sha256=hashlib.sha256(r.stdout).hexdigest(),length=len(r.stdout))
     elif r.returncode==0:
      result.update(json.loads(r.stdout))
      if ours:assert any(str(OUT) in path for path in result['libtiff']),result
     results.append(result)
    baseline,actual=results;equal=baseline['exit']==actual['exit']==0 and baseline.get('sha256')==actual.get('sha256') and baseline.get('shape')==actual.get('shape')
    records.append(dict(engine=engine,fixture=name,match=equal,baseline=baseline,actual=actual))
 report=dict(engines=engines,comparisons=len(records),matched=sum(r['match'] for r in records),results=records)
 (OUT/'installed-consumers.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({k:v for k,v in report.items() if k!='results'}))
 assert all(r['match'] for r in records),[(r['engine'],r['fixture'],r['actual']) for r in records if not r['match']]
if __name__=='__main__':
 if len(sys.argv)>1 and sys.argv[1]=='--child':child(sys.argv[2],sys.argv[3])
 else:main()
