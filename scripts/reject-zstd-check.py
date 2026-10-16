#!/usr/bin/env python3
"""Zstandard semantics and acceptance mutations must fail their proof contracts."""
import os,pathlib,shutil,subprocess
root=pathlib.Path(__file__).resolve().parents[1]
changes={
 'reverse':('zstd_bits.rs','let v=read(s,self.p-n,self.end,n)?;','let v=read(s,self.low,self.end,n)?;','zstd_bits','Reverse::read','postcondition not satisfied'),
 'spread':('zstd_table.rs','pos=(pos+size/2+size/8+3)%size;ticks=ticks-1;','pos=(pos+size/2+size/8+5)%size;ticks=ticks-1;','zstd_table','spread_symbols','invariant not satisfied'),
 'offset':('zstd_sequence_token.rs','if value>3{return Ok([value-3,previous[0],previous[1]]);}','if value>3{return Ok([value-2,previous[0],previous[1]]);}','zstd_sequence_token','resolve','postcondition not satisfied'),
 'overlap':('zstd_output.rs','out[at+i]=out[source];i=i+1;','out[at+i]=0;i=i+1;','zstd_output','append_match','invariant not satisfied'),
 'checksum':('native_zstd.rs','if (sum%4294967296u64) as u32!=wanted{return Err(Error::Invalid);}','if false{return Err(Error::Invalid);}','native_zstd','process','postcondition not satisfied'),
}
def reject(name,dest,module,function,reason):
 log=root/'target'/('reject-zstd-'+name+'.log')
 with log.open('w') as f:r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100','--verify-only-module',module,'--verify-function',function,str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode!=0 and reason in log.read_text(),(name,log.read_text())
 print('Zstandard semantic gate: rejected broken '+name,flush=True)
for name,(file,before,after,module,function,reason) in changes.items():
 dest=root/'target'/('reject-zstd-'+name);shutil.copytree(root/'src',dest,dirs_exist_ok=True)
 p=dest/file;s=p.read_text();assert s.count(before)==1,name;p.write_text(s.replace(before,after));reject(name,dest,module,function,reason)
dest=root/'target/reject-zstd-acceptance';shutil.copytree(root/'src',dest,dirs_exist_ok=True)
p=dest/'native_zstd.rs';s=p.read_text();a=s.index('pub fn decode(');a=s.index('\n{',a);depth=0
for b in range(a+1,len(s)):
 if s[b]=='{':depth+=1
 elif s[b]=='}':
  depth-=1
  if depth==0:break
p.write_text(s[:a]+'\n{proof{raw_witness();} reveal(decoded); reveal(work); Err(Error::Invalid)}'+s[b+1:]);reject('acceptance',dest,'native_zstd','decode','postcondition not satisfied')
