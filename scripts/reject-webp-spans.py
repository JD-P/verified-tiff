#!/usr/bin/env python3
"""Reject incorrect lossless SIMD composition, span selection and framing."""
import os,pathlib,shutil,subprocess
root=pathlib.Path(__file__).resolve().parents[1]
changes={
 'tile-end':('webp_spans','select','let n=if rem<t.width-x{rem}else{t.width-x};','let n=t.width-x;'),
 'carry':('webp_spans','predict','out[k]=v;left=v;tl=top;k=k+1;','out[k]=v;left=v;k=k+1;'),
 'store':('webp_spans','write4','out[k+j]=v[j];','out[k+j]=v[0];'),
 'green':('webp_simd','green4','let b=[p[0][1],0,','let b=[p[0][2],0,'),
 'cross-color':('webp_simd','color4','signed_product_shift5([m[0];4],red)','signed_product_shift5([m[1];4],red)'),
 'predictor':('webp_simd','predict12','clamp_add_sub(left,top,top_left)','clamp_add_sub(left,top_left,top)'),
 'tail':('webp_spans','colors','while k<end','while k+1<end'),
}
for name,(module,function,before,after) in changes.items():
 dest=root/'target'/('reject-webp-spans-'+name);shutil.copytree(root/'src',dest,dirs_exist_ok=True)
 p=dest/(module+'.rs');s=p.read_text()
 # The colors and predictor functions both have this loop spelling.
 if name=='tail':
  start=s.index('pub(super) fn colors');end=s.index('pub(super) fn predict',start)
  body=s[start:end];assert body.count(before)==1;s=s[:start]+body.replace(before,after)+s[end:]
 else:assert s.count(before)==1;s=s.replace(before,after)
 if name=='carry':s=s.replace('pub(super) fn predict(', '#[verifier::spinoff_prover]\npub(super) fn predict(')
 p.write_text(s);log=root/'target'/('reject-webp-spans-'+name+'.log')
 with log.open('w') as f:
  r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100','--verify-only-module',module,'--verify-function',function,str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
 text=log.read_text()
 expected=['precondition not satisfied'] if name=='tile-end' else ['postcondition not satisfied','invariant not satisfied','assertion failed']
 assert r.returncode!=0 and 'Resource limit' not in text and any(x in text for x in expected),(name,text)
 print('WebP span gate: rejected broken '+name,flush=True)
