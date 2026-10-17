#!/usr/bin/env python3
"""Require meaningful failures for RGBA pixels, alpha, acceptance and frames."""
import os,pathlib,shutil,subprocess
root=pathlib.Path(__file__).resolve().parents[1]
changes={
 'pixels':('dst[d]=if le {r} else {a};','dst[d]=if le {0} else {a};','row','invariant not satisfied'),
 'alpha':('if premul {((v as u32*a as u32+127)/255) as u8}','if premul {((v as u32*a as u32+128)/255) as u8}','associate','postcondition not satisfied'),
 'frame':('    Ok(())','    if out.len()>4*n {out[4*n]=0;}\n    Ok(())','row','postcondition not satisfied'),
}
for name in [*changes,'acceptance']:
 dest=root/'target'/('reject-rgba-'+name);shutil.copytree(root/'src',dest,dirs_exist_ok=True)
 p=dest/'native_rgba.rs';s=p.read_text()
 if name in changes:
  before,after,function,reason=changes[name];assert s.count(before)==1,name;p.write_text(s.replace(before,after))
 else:
  function='row';reason='postcondition not satisfied';a=s.index('pub fn row(');a=s.index('\n{',a);depth=0
  for b in range(a+1,len(s)):
   if s[b]=='{':depth+=1
   elif s[b]=='}':
    depth-=1
    if depth==0:break
  p.write_text(s[:a]+'\n{Err(Error::Invalid)}'+s[b+1:])
 log=root/'target'/('reject-rgba-'+name+'.log')
 with log.open('w') as f:r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100','--verify-only-module','native_rgba','--verify-function',function,str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode!=0 and reason in log.read_text(),(name,log.read_text())
 print('RGBA gate: rejected broken '+name,flush=True)
