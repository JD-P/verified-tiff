#!/usr/bin/env python3
"""Semantic negative gates for LERC word extraction and traversal shortcuts."""
import os,pathlib,shutil,subprocess
root=pathlib.Path(__file__).resolve().parents[1]
changes={
 'word':('lerc_word','read','Ok(v as u32)','Ok((v as u32)^1u32)','postcondition not satisfied'),
 'unpack':('lerc_bits','decode_into','data[k]=value;','data[k]=value^1u32;','invariant not satisfied'),
 'float':('lerc_block','reconstruct_float','out[dest]=bits;','out[dest]=bits^1u64;','invariant not satisfied'),
 'coordinates':('lerc_block','target_xy','(p,p*h.depth+t.depth)','(p,(p*h.depth+t.depth)^1usize)','postcondition not satisfied'),
 'mask':('lerc_mask','all','if mask[i]!=255{return false;}','if false{return false;}','invariant not satisfied'),
}
for name,(module,function,before,after,diagnostic) in changes.items():
 dest=root/'target'/('reject-lerc-throughput-'+name);shutil.copytree(root/'src',dest,dirs_exist_ok=True)
 p=dest/(module+'.rs');s=p.read_text();assert s.count(before)==1;p.write_text(s.replace(before,after))
 log=root/'target'/('reject-lerc-throughput-'+name+'.log')
 with log.open('w') as f:
  r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100','--verify-only-module',module,'--verify-function',function,str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode!=0 and diagnostic in log.read_text(),(name,log.read_text())
 print('LERC throughput gate: rejected broken '+name,flush=True)
