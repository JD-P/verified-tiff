#!/usr/bin/env python3
"""Negative semantic/acceptance checks for the native and BigTIFF primitives."""
import pathlib,shutil,subprocess,os
root=pathlib.Path(__file__).resolve().parents[1]
changes={
 'component-swap':('native_samples.rs','out[k]=raw[at+bytes-1-k%bytes];','out[k]=raw[k];','native_samples','swap_components','assertion failed'),
 'stream-step':('native_stream.rs','let byte=c.step(s,self.cap)?;','let byte=None;','native_stream','Inflater::step','postcondition not satisfied'),
 'stream-phrase':('native_stream.rs','Ok(self.table.expand(c,out))','Ok(0usize)','native_stream','Lzw::code','postcondition not satisfied'),
 'stream-packet':('native_stream.rs','Ok((p.next,p.data,p.len,p.repeat))','Ok((p.next,p.data,0usize,p.repeat))','native_stream','packet','postcondition not satisfied'),

 'float-shuffle':('native_fp.rs','reveal(decoded);out[k]=scratch[at];','reveal(decoded);out[k]=0;','native_fp','decode','invariant not satisfied'),
 'rle-alignment':('fax_frame.rs','let n=if c.aligned{(8-b.pos%8)%8}else{0};','let n=0usize;','fax_frame','begin_line','postcondition not satisfied'),
 'predictor':('native_samples.rs','let b=if first {0u64} else {previous[c]};','let b=0u64;','native_samples','convert','assertion failed'),
 'bigoffset':('container.rs','Ok(Header {little_endian:le,big,first_ifd})','Ok(Header {little_endian:le,big,first_ifd:0})','container','header','postcondition not satisfied'),
 'sos-selector':('native.rs','s[9]=2;s[10]=34;','s[9]=2;s[10]=17;','native','renumber_ojpeg_scan','postcondition not satisfied'),
}
for name,(file,before,after,module,function,reason) in changes.items():
 dest=root/'target'/('reject-native-'+name);shutil.copytree(root/'src',dest,dirs_exist_ok=True)
 p=dest/file;s=p.read_text();assert s.count(before)==1,name;p.write_text(s.replace(before,after))
 log=root/'target'/('reject-native-'+name+'.log')
 # Pin the predictor counterexample search; seed 0 times out after ZSTD was added.
 options=['--smt-option','smt.random_seed=1'] if name=='predictor' else []
 with log.open('w') as f:r=subprocess.run([os.environ['VERUS'],*options,'--crate-type','lib','--triggers-mode','silent','--rlimit','100','--verify-only-module',module,'--verify-function',function,str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode!=0 and reason in log.read_text(),(name,log.read_text())
 print('Native semantic gate: rejected broken '+name,flush=True)
# Removing all successful native decoding must violate its exact acceptance.
dest=root/'target/reject-native-acceptance';shutil.copytree(root/'src',dest,dirs_exist_ok=True)
p=dest/'native.rs';s=p.read_text();a=s.index('pub fn decode(');a=s.index('\n{',a);depth=0
for b in range(a+1,len(s)):
 if s[b]=='{':depth+=1
 elif s[b]=='}':
  depth-=1
  if depth==0:break
p.write_text(s[:a]+'\n{reveal(accepted); Err(Error::Invalid)}'+s[b+1:])
log=root/'target/reject-native-acceptance.log'
with log.open('w') as f:r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100','--verify-only-module','native','--verify-function','decode',str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
assert r.returncode!=0 and 'postcondition not satisfied' in log.read_text(),log.read_text()
print('Native acceptance gate: rejected always-error native decoder',flush=True)
