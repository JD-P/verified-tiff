#!/usr/bin/env python3
"""Trusted lane semantics do not excuse wrong composition or preconditions."""
import os,pathlib,shutil,subprocess
root=pathlib.Path(__file__).resolve().parents[1]
changes={
 'butterfly':('vp8_transform','pass4','[add(ac,v),add(amc,u),sub(amc,u),sub(ac,v)]','[sub(ac,v),add(amc,u),sub(amc,u),sub(ac,v)]'),
 'transpose':('vp8_transform','transpose','[t[0][0],t[1][0],t[2][0],t[3][0]]','[t[0][0],t[2][0],t[1][0],t[3][0]]'),
 'rounding':('vp8_transform','rounded_rows','verified_tiff_simd::shift3(h[0])','verified_tiff_simd::shift3(h[1])'),
 'color-rounding':('vp8_color','rgb4','mul_shift8(y,19077)','mul_shift8(y,19076)'),
 'color-order':('vp8_color','rgb4','[red[0],green[0],blue[0],255]','[blue[0],green[0],red[0],255]'),
 'color-position':('vp8_color','pixels4','yy[j]=at(p,0,pw,ph,x+j,y);','yy[j]=at(p,0,pw,ph,x,y);'),
 'color-tail':('native_vp8','render_rgba','&& w-i%w>=4','&& true'),
 'lane-range':('vp8_transform','pass4','0<=limit<=200000,0<=bias<=4','0<=limit<=4000000,0<=bias<=4'),
}
for name,(module,function,before,after) in changes.items():
 dest=root/'target'/('reject-webp-simd-'+name);shutil.copytree(root/'src',dest,dirs_exist_ok=True)
 p=dest/(module+'.rs');s=p.read_text();assert s.count(before)==1;s=s.replace(before,after)
 # Concrete YUV lanes expose the one-unit rounding error without an expensive
 # unconstrained counterexample search. The original contract is unchanged.
 if function=='rgb4':
  signature='fn rgb4(y:[u8;4],u:[u8;4],v:[u8;4]) -> (r:[[u8;4];4])'
  s=s.replace(signature, '#[verifier::spinoff_prover]\n'+signature+'\n    requires y==[27u8,27,27,27],u==[128u8,128,128,128],v==[156u8,156,156,156],')
 if name=='color-position':s=s.replace('pub(super) fn pixels4(', '#[verifier::spinoff_prover]\npub(super) fn pixels4(')
 p.write_text(s)
 log=root/'target'/('reject-webp-simd-'+name+'.log')
 with log.open('w') as f:
  r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100','--verify-only-module',module,'--verify-function',function,str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
 diagnostic=log.read_text()
 expected=['precondition not satisfied'] if name=='lane-range' else ['postcondition not satisfied','invariant not satisfied','assertion failed']
 if name=='color-tail':expected+=['precondition not satisfied']
 assert r.returncode!=0 and 'Resource limit' not in diagnostic and any(x in diagnostic for x in expected),(name,diagnostic)
 print('WebP SIMD gate: rejected broken '+name,flush=True)
