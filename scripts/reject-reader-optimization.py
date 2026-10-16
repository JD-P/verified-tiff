#!/usr/bin/env python3
"""Semantic and acceptance gates for LERC normalization / Deflate batches."""
import os,pathlib,shutil,subprocess
root=pathlib.Path(__file__).resolve().parents[1]
changes={
 'highest':('lerc_numeric.rs','v>>((64-zeros) as u64)==0;\n    proof {top_shift(v as u128,n,128);reveal_with_fuel(top,1);}\n    n','v>>((64-zeros) as u64)==0;\n    proof {top_shift(v as u128,n,128);reveal_with_fuel(top,1);}\n    0u32','lerc_numeric','highest64','postcondition not satisfied'),
 'batch-output':('native_stream.rs','                output[written]=byte;written=written+1;\n','                output[written]=0;written=written+1;\n','native_stream','Inflater::pull','assertion failed'),
 'batch-work':('native_stream.rs','            spend(budget,2)?;','            spend(budget,1)?;','native_stream','Inflater::pull','invariant not satisfied'),
}
for name in [*changes,'batch-acceptance']:
 dest=root/'target'/('reject-reader-opt-'+name);shutil.copytree(root/'src',dest,dirs_exist_ok=True)
 if name in changes:
  file,before,after,module,function,reason=changes[name];p=dest/file;s=p.read_text();assert s.count(before)==1,name;p.write_text(s.replace(before,after))
 else:
  module='native_stream';function='Inflater::pull';reason='postcondition not satisfied';p=dest/'native_stream.rs';s=p.read_text();a=s.index('    pub fn pull(');a=s.index('\n    {',a);depth=0
  for b in range(a+1,len(s)):
   if s[b]=='{':depth+=1
   elif s[b]=='}':
    depth-=1
    if depth==0:break
  p.write_text(s[:a]+'\n    {reveal(Inflater::batch_result); Err(Error::Invalid)}'+s[b+1:])
 log=root/'target'/('reject-reader-opt-'+name+'.log')
 with log.open('w') as f:r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100','--verify-only-module',module,'--verify-function',function,str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode!=0 and reason in log.read_text(),(name,log.read_text())
 print('Reader optimization gate: rejected broken '+name,flush=True)
