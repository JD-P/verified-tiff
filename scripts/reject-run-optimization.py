#!/usr/bin/env python3
"""Reject semantic mutations of the run, strip-copy, fax cursor, WebP table, RGBA and PackBits paths."""
import os,pathlib,shutil,subprocess
root=pathlib.Path(__file__).resolve().parents[1]
changes={
 'stream-stored-output':('native_stream.rs','let byte=c.stored_byte(s,cap);\n        dst[i]=byte;','let byte=c.stored_byte(s,cap);\n        dst[i]=0;','native_stream','stored_run'),
 'stream-stored-work':('native_stream.rs','two work units each.\n                spend(budget,2*n)?;','two work units each.\n                spend(budget,n)?;','native_stream','Inflater::pull'),
 'segment-match-output':('deflate.rs','let byte=c.match_byte(s,cap);\n        dst[i]=byte;','let byte=c.match_byte(s,cap);\n        dst[i]=0;','deflate','copy_into'),
 'segment-stored-output':('deflate.rs','let byte=c.stored_byte(s,cap);\n        dst[i]=byte;','let byte=c.stored_byte(s,cap);\n        dst[i]=0;','deflate','stored_into'),
 'stored-byte':('deflate.rs','let byte=s[self.bits.at];self.bits.at=self.bits.at+1;','let byte=0u8;self.bits.at=self.bits.at+1;','deflate','Cursor::stored_byte'),
 'fax-strip':('fax_tiff.rs','copy_byte_run(pixels,0,output,base,pixels.len(),f.photo==0);','copy_byte_run(pixels,0,output,base,pixels.len(),f.photo!=0);','fax_tiff','scatter_segment'),
 'jpeg-strip':('jpeg_tiff.rs','copy_byte_run(pixels,0,output,base,pixels.len(),false);','copy_byte_run(pixels,0,output,base,pixels.len(),true);','jpeg_tiff','scatter_segment'),
 'fax-bit-order':('fax_bits.rs','let n=if b.lsb!=0 {b.pos%8} else {7-b.pos%8};','let n=if b.lsb==0 {b.pos%8} else {7-b.pos%8};','fax_bits','bit'),
 'webp-symbol-order':('webp_huffman.rs','symbols[at]=i as u16;next[l]=at+1;','symbols[at]=i as u16;next[l]=at;','webp_huffman','ordered'),
 'webp-unused-lengths':('webp_huffman.rs','counts[0]=counts[0]+(2328-alphabet);','counts[0]=counts[0];','webp_huffman','Huff::build'),
 'webp-window':('webp_huffman.rs','code=code*2+bit;let count=self.counts[n];','code=code*2+(bit^1);let count=self.counts[n];','webp_huffman','Huff::read_window'),
 'webp-cache-shift':('webp.rs','let r=mixed>>shift;','let r=mixed>>(shift-1);','webp','hash'),
 'rgba-last-byte':('native_rgba.rs','out[d+3]=if le {a} else {r};','out[d+3]=if le {r} else {r};','native_rgba','row'),
 'packbits-repeat':('lib.rs','fill_byte_run(output, at, n, s[self.data]);','fill_byte_run(output, at, n, 0);','root','PbCursor::take'),
 'packbits-literal':('lib.rs','copy_byte_run(s, self.data, output, at, n, false);','copy_byte_run(s, self.data, output, at, n, true);','root','PbCursor::take'),
 'packbits-literal-advance':('lib.rs','self.data = self.data + n;\n        }\n        self.run = self.run - n;','self.data = self.data + n - 1;\n        }\n        self.run = self.run - n;','root','PbCursor::take'),
 'fill-byte-run':('lib.rs','target[i]=v;i=i+1;','target[i]=0;i=i+1;','root','fill_byte_run'),
}
for name,(file,before,after,module,function) in changes.items():
 dest=root/'target'/('reject-run-'+name);shutil.copytree(root/'src',dest,dirs_exist_ok=True)
 p=dest/file;s=p.read_text();assert s.count(before)==1,(name,s.count(before));p.write_text(s.replace(before,after))
 log=root/'target'/('reject-run-'+name+'.log')
 with log.open('w') as f:r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100',*(['--verify-root'] if module=='root' else ['--verify-only-module',module]),'--verify-function',function,str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
 text=log.read_text()
 assert r.returncode!=0 and any(reason in text for reason in ['postcondition not satisfied','invariant not satisfied','assertion failed']),(name,text[-3000:])
 print('Run gate: rejected broken '+name,flush=True)
