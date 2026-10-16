#!/usr/bin/env python3
"""Import the explicitly trusted FP crate, then verify the decoder normally.

This does not remove/replace any verifier flags. --no-cheating is incompatible
with these imported assumptions and is not claimed. The dependency's five
primitive contracts are assumed; its build log is NOT a proof of their bodies.
"""
import fcntl,hashlib,json,os,pathlib,subprocess,sys
root=pathlib.Path(__file__).resolve().parents[1]
real=root/'.tools/verus-x86-linux/verus'
if any(a in sys.argv for a in ['--version','--help','-h']):os.execv(real,[str(real),*sys.argv[1:]])
imports=[]
for name,modules,trust in [
 ('float',['lib.rs','model.rs'],'Five native primitive contracts are assumed; bodies are not verified.'),
 ('simd',['lib.rs','cpu.rs'],'Eleven fixed-size SIMD integer operation contracts and CPU dispatch are trusted; bodies are not verified.')]:
 folder=root/('target/trusted-'+name);folder.mkdir(parents=True,exist_ok=True)
 sources=[root/('trusted-'+name)/'src'/m for m in modules]
 paths=sources+[root/'toolchain-lock.json',pathlib.Path(__file__),root/'.tools/verus-x86-linux/version.json']
 key=hashlib.sha256(b''.join(p.read_bytes() for p in paths)).hexdigest()
 rlib=folder/('libverified_tiff_'+name+'.rlib');vir=folder/(name+'.vir');receipt=folder/'build.json'
 with (folder/'build.lock').open('w') as lock:
  fcntl.flock(lock,fcntl.LOCK_EX)
  old=json.loads(receipt.read_text()) if receipt.exists() else {}
  if old.get('key')!=key or not rlib.exists() or not vir.exists():
   args=[str(real),'--crate-type','lib','--crate-name','verified_tiff_'+name,'--compile','--export',str(vir),str(sources[0]),'-o',str(rlib)]
   with (folder/'build.log').open('w') as log:
    result=subprocess.run(args,cwd=root,stdout=log,stderr=subprocess.STDOUT)
   if result.returncode:
    sys.stderr.write((folder/'build.log').read_text());sys.exit(result.returncode)
   receipt.write_text(json.dumps(dict(key=key,trust=trust,source_sha256={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sources},rlib_sha256=hashlib.sha256(rlib.read_bytes()).hexdigest(),vir_sha256=hashlib.sha256(vir.read_bytes()).hexdigest()),indent=2)+'\n')
 imports+=['--extern','verified_tiff_'+name+'='+str(rlib),'--import','verified_tiff_'+name+'='+str(vir)]
os.execv(real,[str(real),*imports,*sys.argv[1:]])
