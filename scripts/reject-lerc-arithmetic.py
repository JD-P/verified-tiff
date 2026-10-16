#!/usr/bin/env python3
"""Require the new LERC word kernels to reject incorrect numerical results."""
import os,pathlib,shutil,subprocess
root=pathlib.Path(__file__).resolve().parents[1]
changes={
 'jam':('    r\n}\nproof fn align_word','    r^1u64\n}\nproof fn align_word','jam64'),
 'round':('        return r;\n    }\n    let wide=round(v,n);','        return r^1u64;\n    }\n    let wide=round(v,n);','round_significand'),
 'pack':('    out\n}\n#[inline(always)]\npub(super) fn pack','    out^1u64\n}\n#[inline(always)]\npub(super) fn pack','pack_normal'),
 'add':('return pack_normal(sg,m as u128,e-8,n);','return pack_normal(sg,m as u128,e-8,n)^1u64;','plus_software'),
 'scale':('pack(sg,m*(q as u128),e+1)','pack(sg,m*(q as u128),e)','scale_software'),
}
for name,(before,after,function) in changes.items():
 dest=root/'target'/('reject-lerc-arithmetic-'+name)
 shutil.copytree(root/'src',dest,dirs_exist_ok=True)
 p=dest/'lerc_numeric.rs';s=p.read_text();assert s.count(before)==1,(name,s.count(before));p.write_text(s.replace(before,after))
 log=root/'target'/('reject-lerc-arithmetic-'+name+'.log')
 with log.open('w') as f:
  r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100','--verify-only-module','lerc_numeric','--verify-function',function,str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode!=0 and 'postcondition not satisfied' in log.read_text(),(name,log.read_text())
 print('LERC arithmetic gate: rejected broken '+name,flush=True)
