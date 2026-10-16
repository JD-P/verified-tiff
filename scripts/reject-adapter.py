#!/usr/bin/env python3
"""Semantic mutations must fail checked adapter contracts, not just parsing."""
import os,pathlib,shutil,subprocess
root=pathlib.Path(__file__).resolve().parents[1]
changes={
 'ink-count':('adapter_meta','ink_names','    Ok(count)','    Ok(1u16)'),
 'ink-acceptance':('adapter_meta','ink_names','    Ok(count)','    Err(Error::Invalid)'),
 'ink-length':('adapter_meta','ink_names','s.len()>65535','s.len()>=65535'),
 'schema':('adapter_meta','get_plan','if count_width==2&&n<=65535 {3}','if count_width==2&&n<=65535 {4}'),
 'narrowing':('adapter_meta','convert_integer','    Ok(n as u64)','    Ok(0u64)'),
 'allocation':('adapter_bounds','claim','if total!=0&&next>total','if false'),
 'release':('adapter_bounds','release','else{Ok(used-n)}','else{Ok(used)}'),
 'crop':('adapter_geometry','crop_span','let tx=if fx{vw-x-n}else{x};','let tx=x;'),
 'window':('adapter_window','compact','data.copy_within(at..len,0);','data.copy_within(0..len-at,0);'),
 'commit':('adapter_bounds','commit_uninit','MaybeUninit::new(source[i])','MaybeUninit::new(0)'),
 'color':('adapter_color','f32_index','if v>cap as u64{cap}else{v as u32}','if v>cap as u64{0}else{v as u32}'),
 'row':('adapter_bounds','row_range','let first=start-start%row;','let first=0usize;'),
 'source':('adapter_bounds','source_offset','Ok(at)','Ok(base)'),
}
for name,(module,function,before,after) in changes.items():
 dest=root/'target'/('reject-adapter-'+name);shutil.copytree(root/'src',dest,dirs_exist_ok=True)
 p=dest/(module+'.rs');s=p.read_text();assert s.count(before)==1,(name,s.count(before));p.write_text(s.replace(before,after))
 log=root/'target'/('reject-adapter-'+name+'.log')
 with log.open('w') as f:r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100','--verify-only-module',module,'--verify-function',function,str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode!=0 and ('postcondition not satisfied' in log.read_text() or name=='commit' and 'invariant not satisfied' in log.read_text()),(name,log.read_text())
 print('Adapter proof gate: correctly rejected broken '+name,flush=True)
