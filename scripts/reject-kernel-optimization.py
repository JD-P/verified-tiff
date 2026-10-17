#!/usr/bin/env python3
"""Reject semantic mutations of the optimized codec and coordinate kernels."""
import os,pathlib,shutil,subprocess
root=pathlib.Path(__file__).resolve().parents[1]
changes={
 'lerc-round64':('lerc_numeric.rs','        return r;\n    }\n    let q=v>>n;','        return r^1u128;\n    }\n    let q=v>>n;','lerc_numeric','round'),
 'lerc-narrow':('lerc_numeric.rs','        return out;','        return out^1u32;','lerc_numeric','narrow_software'),
 'vp8-dc':('vp8_transform.rs','let out=[v;16];','let out=[0i32;16];','vp8_transform','transform'),
 'vp8-filter':('vp8_filter_kernel.rs','            out[3]=pr::clip(p0+a2);out[4]=pr::clip(q0-a1);','            out[3]=0;out[4]=pr::clip(q0-a1);','vp8_filter_kernel','filter'),
 'webp-coordinates':('webp_inverse.rs','else{x=x+1;}','else{x=x;}','webp_inverse','apply'),
 'fp-coordinates':('native_fp.rs','else{local=local+1;}','else{local=local;}','native_fp','decode'),
 'match-output':('native_stream.rs','let byte=c.match_byte(s,cap);\n        dst[i]=byte;','let byte=c.match_byte(s,cap);\n        dst[i]=0;','native_stream','copy_run'),
 'match-work':('native_stream.rs','charged together.\n                spend(budget,2*n)?;','charged together.\n                spend(budget,n)?;','native_stream','Inflater::pull'),
}
for name,(file,before,after,module,function) in changes.items():
 dest=root/'target'/('reject-kernel-'+name);shutil.copytree(root/'src',dest,dirs_exist_ok=True)
 p=dest/file;s=p.read_text();assert s.count(before)==1,(name,s.count(before));p.write_text(s.replace(before,after))
 log=root/'target'/('reject-kernel-'+name+'.log')
 with log.open('w') as f:r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100','--verify-only-module',module,'--verify-function',function,str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
 text=log.read_text()
 assert r.returncode!=0 and any(reason in text for reason in ['postcondition not satisfied','invariant not satisfied','assertion failed']),(name,text)
 print('Kernel gate: rejected broken '+name,flush=True)
