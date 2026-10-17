#!/usr/bin/env python3
"""Reject semantic mutations of the run and WebP table paths."""
import os,pathlib,shutil,subprocess
root=pathlib.Path(__file__).resolve().parents[1]
changes={
 'stream-stored-output':('native_stream.rs','let byte=c.stored_byte(s,cap);\n        dst[i]=byte;','let byte=c.stored_byte(s,cap);\n        dst[i]=0;','native_stream','stored_run'),
 'stream-stored-work':('native_stream.rs','two work units each.\n                spend(budget,2*n)?;','two work units each.\n                spend(budget,n)?;','native_stream','Inflater::pull'),
 'segment-match-output':('deflate.rs','let byte=c.match_byte(s,cap);\n        dst[i]=byte;','let byte=c.match_byte(s,cap);\n        dst[i]=0;','deflate','copy_into'),
 'segment-stored-output':('deflate.rs','let byte=c.stored_byte(s,cap);\n        dst[i]=byte;','let byte=c.stored_byte(s,cap);\n        dst[i]=0;','deflate','stored_into'),
 'stored-byte':('deflate.rs','let byte=s[self.bits.at];self.bits.at=self.bits.at+1;','let byte=0u8;self.bits.at=self.bits.at+1;','deflate','Cursor::stored_byte'),
 'webp-symbol-order':('webp_huffman.rs','symbols[at]=i as u16;next[l]=at+1;','symbols[at]=i as u16;next[l]=at;','webp_huffman','ordered'),
 'webp-unused-lengths':('webp_huffman.rs','counts[0]=counts[0]+(2328-alphabet);','counts[0]=counts[0];','webp_huffman','Huff::build'),
 'webp-window':('webp_huffman.rs','code=code*2+bit;let count=self.counts[n];','code=code*2+(bit^1);let count=self.counts[n];','webp_huffman','Huff::read_window'),
 'webp-cache-shift':('webp.rs','let r=mixed>>shift;','let r=mixed>>(shift-1);','webp','hash'),
}
for name,(file,before,after,module,function) in changes.items():
 dest=root/'target'/('reject-run-'+name);shutil.copytree(root/'src',dest,dirs_exist_ok=True)
 p=dest/file;s=p.read_text();assert s.count(before)==1,(name,s.count(before));p.write_text(s.replace(before,after))
 log=root/'target'/('reject-run-'+name+'.log')
 with log.open('w') as f:r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100','--verify-only-module',module,'--verify-function',function,str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
 text=log.read_text()
 assert r.returncode!=0 and any(reason in text for reason in ['postcondition not satisfied','invariant not satisfied','assertion failed']),(name,text[-3000:])
 print('Run gate: rejected broken '+name,flush=True)
