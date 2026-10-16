#!/usr/bin/env python3
"""Optional real-consumer smoke test; uses an already installed Pillow.
Force Pillow's libtiff decoder and interpose only in isolated subprocesses.
LD_DEBUG confirms TIFFClientOpen/Ext resolves to our compatibility library.
No system files, packages or loader settings are changed.
"""
import json,os,pathlib,subprocess,sys,tempfile
ROOT=pathlib.Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'tests'))
from extended import make_extended
import PIL
OUT=pathlib.Path(os.environ.get('VTIFF_COMPAT_OUTPUT',ROOT/'target/compat')).resolve()
child='''from PIL import Image,TiffImagePlugin
import sys,json
TiffImagePlugin.READ_LIBTIFF=True
with Image.open(sys.argv[1]) as im:
 im.load()
 open(sys.argv[2],"wb").write(im.tobytes())
 print(json.dumps(dict(mode=im.mode,size=im.size)))
'''
with tempfile.TemporaryDirectory() as d:
 p=pathlib.Path(d);cases=0;bound=False
 for opt in [dict(ch=1,bits=8),dict(ch=3,bits=8),dict(ch=4,bits=8,alpha=2),dict(ch=4,bits=8,alpha=1),dict(ch=1,bits=4,palette=True),dict(ch=1,bits=16)]:
  for tiled in [False,True]:
   src=p/'input.tif';src.write_bytes(make_extended(w=17,h=19,le=True,tiled=tiled,compression=5,**opt)[0])
   env=os.environ.copy();env.pop('LD_PRELOAD',None);env.pop('LD_DEBUG',None)
   a=subprocess.run([sys.executable,'-c',child,str(src),str(p/'reference.raw')],env=env,capture_output=True,timeout=30)
   assert a.returncode==0,a.stderr
   env['LD_PRELOAD']=str(OUT/'libtiff.so.6');env['LD_DEBUG']='bindings'
   b=subprocess.run([sys.executable,'-c',child,str(src),str(p/'ours.raw')],env=env,capture_output=True,timeout=30)
   assert b.returncode==0,(opt,tiled,b.stderr.decode()[-6000:])
   assert a.stdout==b.stdout,(a.stdout,b.stdout)
   assert (p/'reference.raw').read_bytes()==(p/'ours.raw').read_bytes(),(opt,tiled)
   bindings=[line for line in b.stderr.decode().splitlines() if 'symbol `TIFF' in line and str(OUT/'libtiff.so.6') in line]
   assert any('TIFFClientOpen' in line for line in bindings),bindings
   bound=True;cases+=1
 print(json.dumps(dict(pillow_version=PIL.__version__,forced_libtiff=True,loader_binding_confirmed=bound,exact_files=cases)))
