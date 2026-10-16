#!/usr/bin/env python3
"""Native arithmetic is trusted; its verified dispatch must not invent results."""
import os,pathlib,shutil,subprocess
root=pathlib.Path(__file__).resolve().parents[1]
changes={
 'scale':('hw::mul_twice_u32(a,q)','hw::mul_twice_u32(a,q)^1u64','scale'),
 'add':('hw::add(a,b)','hw::add(a,b)^1u64','plus'),
 'narrow':('hw::narrow(v)','hw::narrow(v)^1u32','narrow'),
 'widen':('return hw::widen(v as u32);','return hw::widen(v as u32)^1u64;','number'),
 'integer':('hw::from_magnitude(mag as u32,neg)','hw::from_magnitude(mag as u32,neg)^1u64','number'),
}
for name,(before,after,function) in changes.items():
 dest=root/'target'/('reject-native-float-'+name)
 shutil.copytree(root/'src',dest,dirs_exist_ok=True)
 p=dest/'lerc_numeric.rs';s=p.read_text();assert s.count(before)==1;p.write_text(s.replace(before,after))
 log=root/'target'/('reject-native-float-'+name+'.log')
 with log.open('w') as f:
  r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100','--verify-only-module','lerc_numeric','--verify-function',function,str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode!=0 and 'postcondition not satisfied' in log.read_text(),(name,log.read_text())
 print('Native FP dispatch gate: rejected broken '+name,flush=True)

# Passing a non-finite operand to a finite-only primitive must fail its
# precondition, rather than inheriting an invalid trusted equality.
dest=root/'target/reject-native-float-finite'
shutil.copytree(root/'src',dest,dirs_exist_ok=True)
p=dest/'lerc_numeric.rs';s=p.read_text();before='if (a/0x10000000000000u64)%2048!=2047 && (b/0x10000000000000u64)%2048!=2047 {hw::add(a,b)}'
assert s.count(before)==1;p.write_text(s.replace(before,'if true {hw::add(a,b)}'))
log=root/'target/reject-native-float-finite.log'
with log.open('w') as f:
 r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100','--verify-only-module','lerc_numeric','--verify-function','plus',str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
assert r.returncode!=0 and 'precondition not satisfied' in log.read_text(),log.read_text()
print('Native FP dispatch gate: rejected missing finite-operand check',flush=True)
