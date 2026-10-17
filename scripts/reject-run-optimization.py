#!/usr/bin/env python3
"""Reject semantic mutations of the run, strip-copy, fax cursor, WebP table, RGBA, PackBits, LZW, JPEG lookahead, one-pass JPEG, WebP prefix table, whole-match and stored Deflate, LZW chunk copy and float unshuffle paths."""
import os,pathlib,shutil,subprocess
root=pathlib.Path(__file__).resolve().parents[1]
changes={
 'stream-stored-output':('native_stream.rs','let byte=c.stored_byte(s,src,i,cap);\n        dst[i]=byte;','let byte=c.stored_byte(s,src,i,cap);\n        dst[i]=0;','native_stream','stored_run'),
 'stream-stored-work':('native_stream.rs','two work units each.\n                spend(budget,2*n)?;','two work units each.\n                spend(budget,n)?;','native_stream','Inflater::pull'),
 'segment-match-output':('deflate.rs','let byte=c.match_byte(s,cap);\n        dst[i]=byte;','let byte=c.match_byte(s,cap);\n        dst[i]=0;','deflate','copy_into'),
 'segment-stored-output':('deflate.rs','let byte=c.stored_byte(s,src,i,cap);\n        dst[i]=byte;','let byte=c.stored_byte(s,src,i,cap);\n        dst[i]=0;','deflate','stored_into'),
 'stored-byte':('deflate.rs','let byte=src[i];self.bits.at=self.bits.at+1;','let byte=0u8;self.bits.at=self.bits.at+1;','deflate','Cursor::stored_byte'),
 'fax-strip':('fax_tiff.rs','copy_byte_run(pixels,0,output,base,pixels.len(),f.photo==0);','copy_byte_run(pixels,0,output,base,pixels.len(),f.photo!=0);','fax_tiff','scatter_segment'),
 'jpeg-strip':('jpeg_tiff.rs','copy_byte_run(pixels,0,output,base,pixels.len(),false);','copy_byte_run(pixels,0,output,base,pixels.len(),true);','jpeg_tiff','scatter_segment'),
 'fax-bit-order':('fax_bits.rs','let n=if b.lsb!=0 {b.pos%8} else {7-b.pos%8};','let n=if b.lsb==0 {b.pos%8} else {7-b.pos%8};','fax_bits','bit'),
 'webp-symbol-order':('webp_huffman.rs','symbols[at]=i as u16;next[l]=at+1;','symbols[at]=i as u16;next[l]=at;','webp_huffman','ordered'),
 'webp-unused-lengths':('webp_huffman.rs','counts[0]=counts[0]+(2328-alphabet);','counts[0]=counts[0];','webp_huffman','Huff::build'),
 'webp-window':('webp_huffman.rs','code=code*2+bit;let count=self.counts[n];','code=code*2+(bit^1);let count=self.counts[n];','webp_huffman','Huff::read_window'),
 'webp-cache-shift':('webp.rs','let r=mixed>>shift;','let r=mixed>>(shift-1);','webp','hash'),
 'rgba-last-byte':('native_rgba.rs','dst[d+3]=if le {a} else {r};','dst[d+3]=if le {r} else {r};','native_rgba','row'),
 'packbits-repeat':('lib.rs','fill_byte_run(output, at, n, s[self.data]);','fill_byte_run(output, at, n, 0);','root','PbCursor::take'),
 'packbits-literal':('lib.rs','copy_byte_run(s, self.data, output, at, n, false);','copy_byte_run(s, self.data, output, at, n, true);','root','PbCursor::take'),
 'packbits-literal-advance':('lib.rs','self.data = self.data + n;\n        }\n        self.run = self.run - n;','self.data = self.data + n - 1;\n        }\n        self.run = self.run - n;','root','PbCursor::take'),
 'fill-byte-run':('lib.rs','target[i]=v;i=i+1;','target[i]=0;i=i+1;','root','fill_byte_run'),
 'lzw-tail-acceptance':('lib.rs','        if i==needed && table.prev!=256 && bits.at_tail() { return Ok(()); }\n','','root','lz_decode'),
 'lzw-tail-soundness':('lib.rs','if i==needed && table.prev!=256 && bits.at_tail() { return Ok(()); }','if table.prev!=256 && bits.at_tail() { return Ok(()); }','root','lz_decode'),
 'lzw-literal':('lib.rs','output[i]=c as u8;','output[i]=0;','root','lz_decode'),
 'lzw-entry-position':('lib.rs','if created { pos[slot]=last as u32; }','if created { pos[slot]=i as u32; }','root','lz_decode'),
 'lzw-repeat-tail':('lib.rs','output[i+len-1]=output[last];','output[i+len-1]=0;','root','lz_decode'),
 'copy-within':('lib.rs','let b=output[src+j];output[dst+j]=b;','let b=output[src+j];output[dst+j]=0;','root','copy_within'),
 'lzw-skip-length':('native_stream.rs','else{self.table.data(c,self.legacy)}\n','else{self.table.data(c,self.legacy)?;Ok(1)}\n','native_stream','Lzw::skip'),
 'rgba-row-guard':('native_rgba.rs','if at+ch>src.len()||','if at+ch>=src.len()||','native_rgba','row'),
 'jpeg-table-entry':('jpeg_huffman.rs','let entry=(n*256+sym as usize) as u16;','let entry=((n+1)*256+sym as usize) as u16;','jpeg_huffman','fast'),
 'jpeg-window-symbol':('jpeg_huffman.rs','let sym=(e&255) as u8;','let sym=(e&127) as u8;','jpeg_huffman','decode_fast'),
 'jpeg-window-state':('jpeg_huffman.rs','let q=if len<=left {','let q=if len<left {','jpeg_huffman','decode_fast'),
 'jpeg-window-stuffing':('jpeg_huffman.rs','Bits {at:b.at+adv,byte:nb,left:8-(len-left),end:b.end}','Bits {at:b.at+1,byte:nb,left:8-(len-left),end:b.end}','jpeg_huffman','decode_fast'),
 'jpeg-amplitude':('jpeg_bits.rs','let v=hi*power(m)+((nb as usize)>>(8-m));','let v=hi*power(m)+((nb as usize)>>(8-m))/2;','jpeg_bits','read_fast'),
 'jpeg-amplitude-stuffing':('jpeg_bits.rs','Ok((v,Bits {at:b.at+adv,byte:nb,left:8-m,end:b.end}))','Ok((v,Bits {at:b.at+1,byte:nb,left:8-m,end:b.end}))','jpeg_bits','read_fast'),
 'jpeg-capture-sample':('jpeg_scan.rs','{output[i*64+j]=values[j];j=j+1;}\n        assert forall|k:int| 0<=k<(i as int+1)*64 implies output@[k]==crate::jpeg_transform::sample(c.history','{output[i*64+j]=values[0];j=j+1;}\n        assert forall|k:int| 0<=k<(i as int+1)*64 implies output@[k]==crate::jpeg_transform::sample(c.history','jpeg_scan','decode_checked'),
 'jpeg-capture-ending':('jpeg_scan.rs','    if c.end(s,legacy) {\n        assert(result==Some(c.model()));\n        Ok(())','    if true {\n        Ok(())','jpeg_scan','decode_checked'),
 'jpeg-one-pass-accept':('jpeg_tiff.rs','    if taken {\n        spend(budget,image.work)?;','    if taken {\n        return Err(Error::Limit);','jpeg_tiff','decode_jpeg_into'),
 'span-pixel':('adapter_geometry.rs','if fx {(n-1-i)*4} else {i*4}','if fx {(n-i)*4} else {i*4}','adapter_geometry','span_pixel'),
 'fp-unshuffle-plane':('native_fp.rs','let plane=if out_le{bytes-1-r}else{r};','let plane=if out_le{r}else{r};','native_fp','unshuffle'),
 'store-byte':('native_samples.rs','if bytes==1 {out[at]=(v%256) as u8;return;}','if bytes==1 {out[at]=(v%255) as u8;return;}','native_samples','store'),
 'webp-fast-entry':('webp_huffman.rs','return (n*4096+sym) as u16;','return ((n+1)*4096+sym) as u16;','webp_huffman','Huff::entry'),
 'webp-fast-symbol':('webp_huffman.rs','let len=(e/4096) as usize;let sym=(e%4096) as usize;','let len=(e/4096) as usize;let sym=(e/4096) as usize;','webp_huffman','Huff::read_window'),
 'block-ring':('deflate.rs','let (_,tail)=ring.split_at_mut(d0);','let (_,tail)=ring.split_at_mut(0);','deflate','Cursor::append_block'),
 'block-checksum':('deflate.rs','a=a+byte as usize;sum=sum+a;','a=a+byte as usize;sum=sum+byte as usize;','deflate','Cursor::append_block'),
 'stored-block-source':('deflate.rs','let (_,tail)=s.split_at(self.bits.at);','let (_,tail)=s.split_at(self.bits.at+1);','deflate','Cursor::stored_block'),
 'stored-block-advance':('deflate.rs','self.left=self.left-n;self.bits.at=self.bits.at+n;','self.left=self.left-n;self.bits.at=self.bits.at+n-1;','deflate','Cursor::stored_block'),
 'copy-ahead':('lib.rs','w[0]=b0;w[1]=b1;','w[0]=b1;w[1]=b1;','root','copy_ahead'),
 'lzw-copy-limit':('lib.rs','copy_ahead(output,pos[c] as usize,i,len,needed);','copy_ahead(output,pos[c] as usize,i,len,output.len());','root','lz_decode'),
}
for name,(file,before,after,module,function) in changes.items():
 dest=root/'target'/('reject-run-'+name);shutil.copytree(root/'src',dest,dirs_exist_ok=True)
 p=dest/file;s=p.read_text();assert s.count(before)==1,(name,s.count(before));p.write_text(s.replace(before,after))
 log=root/'target'/('reject-run-'+name+'.log')
 with log.open('w') as f:r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100',*(['--verify-root'] if module=='root' else ['--verify-only-module',module]),'--verify-function',function,str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
 text=log.read_text()
 assert r.returncode!=0 and any(reason in text for reason in ['postcondition not satisfied','invariant not satisfied','assertion failed']),(name,text[-3000:])
 print('Run gate: rejected broken '+name,flush=True)
