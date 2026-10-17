//! Native segment orchestration and libtiff RGBA presentation (unverified ABI).
//! Generic compression and sample transforms call verified native primitives.
//! Older specialized codecs use a bounded, one-segment TIFF view and the
//! original verified inspectors/decoders; never an external codec library.
use super::*;
use verified_tiff::adapter_geometry as geometry;
use verified_tiff::{native,native_samples as ns};

impl Handle{
    pub fn native_ok(&self)->R<()>{
        let m=&self.meta;
        if m.photo==6 && !self.jpeg_rgb && !self.ycbcr_raw(){return Err((NOTIMPL,"native subsampled YCbCr not implemented; request JPEGCOLORMODE_RGB"));}
        Ok(())
    }
    pub fn segment(&mut self,k:usize,budget:&mut Budget)->R<()>{
        if self.cache.as_ref().is_some_and(|(i,_)|*i==k){return Ok(());}
        self.cache=None;
        self.meta.segment_rows(k)?;
        let off=self.meta.offsets.get(&mut self.source,k)?;
        let n=usize::try_from(self.meta.counts.get(&mut self.source,k)?).map_err(|_|(LIMIT,"encoded segment size overflow"))?;
        let encoded=self.source.read(off,n)?;
        let output=self.decode_segment(k,encoded,budget)?;
        self.cache=Some((k,output));Ok(())
    }
    pub fn decode_segment(&mut self,k:usize,mut encoded:Buf<u8>,budget:&mut Budget)->R<Buf<u8>>{
        let m=&self.meta;let (row,rows)=self.native_layout(m.bw,m.segment_rows(k)?)?;
        let needed=product(row,rows)?;
        if needed>self.limits.max_output{return Err((LIMIT,"native segment output limit"));}
        let mem=&self.source.mem;
        let output=if matches!(m.compression,1|32773|5|8|32946|50000){
            if !(1..=128).contains(&m.bits) || !(1..=6).contains(&m.format) || (m.format==3 && !matches!(m.bits,16|24|32|64)) || (matches!(m.format,5|6) && !matches!(m.bits,16|32|64|128)){
                return Err((NOTIMPL,"native sample width/format not implemented"));
            }
            if m.predictor!=1 && m.predictor!=2 && m.predictor!=3{return Err((NOTIMPL,"sample predictor not implemented"));}
            if m.predictor==2 && (!matches!(m.bits,8|16|32|64) || matches!(m.format,5|6) || !matches!(m.compression,5|8|32946|50000)){return Err((NOTIMPL,"horizontal prediction requires byte samples and LZW/Deflate/ZSTD"));}
            if m.predictor==3 && (m.format!=3 || !matches!(m.bits,16|24|32|64) || !matches!(m.compression,5|8|32946|50000)){return Err((NOTIMPL,"floating prediction requires float16/24/32/64 and LZW/Deflate/ZSTD"));}
            if m.fill==2{ns::reverse_bits(&mut encoded);}
            let cap=if matches!(m.compression,8|32946|50000) && !m.tiled{let (a,b)=self.native_layout(m.bw,m.bh)?;product(a,b)?}else{needed};
            if cap>self.limits.max_output{return Err((LIMIT,"decoded padding limit"));}
            let mut raw=Buf::zeros(if m.compression==50000{cap}else{needed},self.limits.max_output,mem)?;
            if m.compression==50000{
                let mut scratch=Buf::zeros(verified_tiff::native_zstd::scratch_len(cap),self.limits.max_output,mem)?;
                verified_tiff::native_zstd::decode(&encoded,needed,cap,&mut raw,&mut scratch,budget).map_err(err)?;
                raw.truncate(needed);
            }else{native::decode(&encoded,m.compression,row,needed,cap,&mut raw,budget).map_err(err)?;}
            if m.predictor==3{
                let mut scratch=Buf::zeros(needed,self.limits.max_output,mem)?;let mut out=Buf::zeros(needed,self.limits.max_output,mem)?;
                verified_tiff::native_fp::decode(&raw,row,if m.planar==2{1}else{m.ch},m.bits/8,cfg!(target_endian="little"),&mut scratch,&mut out).map_err(err)?;out
            }else if m.predictor==2{
                let mut out=Buf::zeros(needed,self.limits.max_output,mem)?;
                ns::convert(&raw,row,if m.planar==2{1}else{m.ch},m.bits/8,m.le,true,cfg!(target_endian="little"),&mut out).map_err(err)?;out
            }else{
                // libtiff transports packed samples without unpacking them.
                // Its endian postprocessor applies only to these word widths.
                let bytes=if matches!(m.format,5|6){m.bits/16}else{m.bits/8};
                if m.le!=cfg!(target_endian="little") && (matches!(m.bits,16|24|32|64|128)) && bytes>1{
                    let mut out=Buf::zeros(needed,self.limits.max_output,mem)?;
                    ns::swap_components(&raw,bytes,&mut out).map_err(err)?;out
                }else{raw}
            }

        }else if m.compression==50001{self.webp(&encoded,rows,needed,budget)?
        }else if m.compression==34887{self.lerc(&encoded,rows,needed,budget)?
        }else if m.compression==2{
            if m.bits!=1||m.ch!=1||m.predictor!=1{return Err((NOTIMPL,"CCITT RLE requires one bilevel component"));}
            let count=product(m.bw,rows)?;let mut pixels=Buf::zeros(count,self.limits.max_output,mem)?;
            verified_tiff::native_rle::decode(&encoded,m.bw,rows,m.fill==2,&mut pixels,budget).map_err(err)?;
            let mut out=Buf::zeros(needed,self.limits.max_output,mem)?;
            pack_bilevel(&pixels,m.bw,rows,row,0,&mut out)?;out
        }else{self.special(k,&encoded,budget)?};
        if output.len()!=needed{return Err((NOTIMPL,"special codec native layout mismatch"));}
        Ok(output)
    }
    pub fn read(&mut self,kind:u32,a:usize,b:usize,c:usize,_d:usize,limit:usize)->R<()>{
        self.rendered=None;
        let mut budget=Budget{remaining:self.work};
        match kind{
            0=>{
                self.native_ok()?;
                if self.ycbcr_raw()&&self.sampling()?.1>1{return Err((NOTIMPL,"vertically subsampled YCbCr requires strip/tile reads"));}
                let m=&self.meta;if m.tiled{return Err((NOTIMPL,"scanline reads on tiled images not implemented"));}
                if a>=m.height || (m.planar==2 && b>=m.ch) || (m.planar==1 && b!=0){return Err((INVALID,"scanline row/sample out of range"));}
                let row=self.raw_row(m.width)?;let idx=a/m.bh+if m.planar==2{b*m.per_plane()?}else{0};let within=(a%m.bh)*row;
                let out=self.native_range(idx,within,row,&mut budget)?;self.rendered=Some(out);
            }
            1|2=>{
                self.native_ok()?;if (kind==2)!=self.meta.tiled{return Err((INVALID,"strip/tile API does not match layout"));}
                let (row,rows)=self.native_layout(self.meta.bw,self.meta.segment_rows(a)?)?;
                let n=product(row,rows)?.min(limit);
                self.rendered=Some(self.native_range(a,0,n,&mut budget)?);
            }
            3=>{
                let m=&self.meta;if a>=m.offsets.len(){return Err((INVALID,"segment index out of range"));}
                let off=self.meta.offsets.get(&mut self.source,a)?;
                let n=usize::try_from(self.meta.counts.get(&mut self.source,a)?).map_err(|_|(LIMIT,"raw segment size overflow"))?;
                if off>self.source.len||n as u64>self.source.len-off{return Err((INVALID,"raw segment outside source"));}
                self.rendered=Some(self.source.read(off,n.min(limit))?);
            }
            4=>{
                if a==0 || a>self.meta.width || b==0{return Err((INVALID,"RGBA width must fit image and height must be positive"));}
                self.rgba(0,0,a,b.min(self.meta.height),c)?;
            }
            5=>{
                let m=&self.meta;if m.tiled || a>=m.height || a%m.bh!=0{return Err((INVALID,"invalid RGBA strip row"));}
                self.rgba(0,a,m.width,m.bh.min(m.height-a),4)?;
            }
            6=>{
                let m=&self.meta;if !m.tiled || a>=m.width || b>=m.height || a%m.bw!=0 || b%m.bh!=0{return Err((INVALID,"invalid RGBA tile coordinate"));}
                self.rgba(a,b,m.bw,m.bh,4)?;
            }
            _=>return Err((NOTIMPL,"read operation not implemented")),
        }Ok(())
    }
    pub fn rgba_ok(&self)->R<()>{
        let m=&self.meta;
        if m.width==0{return Err((INVALID,"no selected image directory"));}
        if m.format!=1 || !matches!(m.bits,1|2|4|8|16) || !matches!(m.photo,0|1|2|3|5|6|8){return Err((NOTIMPL,"RGBA sample/color conversion not implemented"));}
        let base=if m.photo==5{4}else if matches!(m.photo,2|6|8){3}else{1};
        if m.ch<base || m.extra.len()>m.ch-base || (m.photo==3 && (m.ch!=1 || m.map.len()!=3*(1usize<<m.bits))){return Err((INVALID,"RGBA color/extra sample layout"));}
        if m.extra.iter().any(|v|*v>2){return Err((NOTIMPL,"extra sample meaning not implemented"));}
        if self.ycbcr_raw(){self.sampling()?;}
        if matches!(m.photo,5|8)&&!matches!(m.bits,8|16){return Err((NOTIMPL,"CMYK/Lab bit depth"));}
        if m.photo==5&&scalar(&self.data,&m.tags,332,1,m.le)?!=1{return Err((NOTIMPL,"non-CMYK separated inks"));}
        Ok(())
    }
    pub fn rgba(&mut self,x:usize,y:usize,w:usize,h:usize,orientation:usize)->R<()>{
        self.rendered=None;
        self.rgba_ok()?;
        if matches!(self.meta.photo,5|8)||self.ycbcr_raw(){return self.rgba_color(x,y,w,h,orientation);}
        let mut budget=Budget{remaining:self.work};
        if !(1..=8).contains(&orientation) || w==0 || h==0{return Err((INVALID,"invalid RGBA dimensions/orientation"));}
        let mut result=Buf::zeros(product(product(w,h)?,4)?,self.limits.max_output,&self.source.mem)?;
        let m=&self.meta;let width=m.width;let height=m.height;let bw=m.bw;let bh=m.bh;let bits=m.bits;
        let channels=m.ch;let planar=m.planar;let photo=m.photo;let source_orientation=m.orientation;
        let base=if photo==2||photo==6{3}else{1};
        let alpha=m.extra.iter().position(|v|*v==1||*v==2).map(|i|(channels-m.extra.len()+i,m.extra[i]));
        let row=m.row_bytes(bw)?;let across=(width-1)/bw+1;
        let right=x.checked_add(w).ok_or((LIMIT,"RGBA extent overflow"))?.min(width);
        let bottom=y.checked_add(h).ok_or((LIMIT,"RGBA extent overflow"))?.min(height);
        let (fx,fy)=geometry::orientation(source_orientation,orientation).map_err(err)?;
        let fast=planar==1&&matches!(bits,8|16)&&photo!=3;
        let packed=channels==1&&matches!(photo,0|1|3)&&(matches!(bits,1|2|4)||(photo==3&&bits==8));
        let mut expansion=[[0u8;32];256];
        if packed{
            let mask=(1usize<<bits)-1;
            for byte in 0..256{for j in 0..8/bits{
                let v=(byte>>(8-bits-j*bits))&mask;
                let q=if photo==3{let n=1usize<<bits;[(self.meta.map[v]>>8) as u8,(self.meta.map[n+v]>>8) as u8,(self.meta.map[2*n+v]>>8) as u8,255]}
                    else{let v=if photo==0{mask-v}else{v};let b=((v*255+mask/2)/mask) as u8;[b,b,b,255]};
                pack(&mut expansion[byte][j*4..j*4+4],q);
            }}
        }
        // Work in the requested region's coordinates, matching libtiff's strip
        // and tile orientation conventions. Padding remains zero.
        for by in y/bh..=(bottom-1)/bh{for bx in x/bw..=(right-1)/bw{
            let left=x.max(bx*bw);let top=y.max(by*bh);let rr=right.min((bx+1)*bw);let bb=bottom.min((by+1)*bh);
            for plane in 0..if planar==2{channels}else{1}{
                let segment=geometry::segment(across,(height-1)/bh+1,if planar==2{channels}else{1},bx,by,plane).map_err(err)?;let needed=product(row,self.meta.segment_rows(segment)?)?;
                let complete=left==bx*bw&&rr==width.min((bx+1)*bw)&&top==by*bh&&bb==height.min((by+1)*bh);
                let full=if complete&&self.rgba_full_segment(segment,needed)?{Some(self.native_range(segment,0,needed,&mut budget)?)}else{None};
                let cached=if full.is_none()&&self.rgba_cached_segment(segment,needed)?{self.segment(segment,&mut budget)?;self.cache.take()}else{None};
                for yy in top..bb{
                    let at=(yy-by*bh)*row;
                    let owned=if full.is_none()&&cached.is_none(){Some(self.native_range(segment,at,row,&mut budget)?)}else{None};
                    let data=if let Some(d)=&full{&d[at..at+row]}else if let Some((_,d))=&cached{&d[at..at+row]}else{&**owned.as_ref().unwrap()};
                    if packed{
                        let (dest,_)=geometry::crop_span(w,h,right-x,bottom-y,left-x,yy-y,rr-left,fx,fy).map_err(err)?;
                        packed_rgba_row(data,left-bx*bw,rr-left,bits,&expansion,fx,&mut result[dest..dest+(rr-left)*4]);
                        continue;
                    }
                    if fast{
                        let (dest,_)=geometry::crop_span(w,h,right-x,bottom-y,left-x,yy-y,rr-left,fx,fy).map_err(err)?;
                        let start=(left-bx*bw)*channels*(bits/8);let n=rr-left;
                        rgba_row(&data[start..],n,channels,bits,base==3,photo==0,alpha,fx,&mut result[dest..dest+n*4])?;
                        continue;
                    }
                    for xx in left..rr{
                    let (dest,_)=geometry::crop_span(w,h,right-x,bottom-y,xx-x,yy-y,1,fx,fy).map_err(err)?;
                    let mut rgba=if planar==2 && plane>0{unpack(&result[dest..dest+4])}else{[0,0,0,255]};
                    for c in 0..if planar==2{1}else{channels}{
                        let ci=if planar==2{plane}else{c};
                        let index=(xx-bx*bw)*if planar==2{1}else{channels}+c;
                        let v=sample(&data,0,index,bits)?;
                        if photo==3{
                            let n=1usize<<bits;for c in 0..3{rgba[c]=(self.meta.map[c*n+v as usize]>>8) as u8;}
                        }else if ci<base{
                            let max=(1u32<<bits)-1;let v=if photo==0{max-v}else{v};
                            let b=if bits==16 && base==1{(v>>8) as u8}else{((v*255+max/2)/max) as u8};
                            if base==1{rgba[0]=b;rgba[1]=b;rgba[2]=b;}else{rgba[ci]=b;}
                        }else if alpha.is_some_and(|(i,_)|ci==i){rgba[3]=if bits==16{if base==3{((v+128)/257) as u8}else{(v>>8) as u8}}else{((v*255)/((1u32<<bits)-1)) as u8};}
                    }
                    pack(&mut result[dest..dest+4],rgba);
                }}
                if cached.is_some(){self.cache=cached;}
            }
        }}
        if !fast && base==3 && alpha.is_some_and(|(_,kind)|kind==2){
            for p in result.chunks_exact_mut(4){let mut q=unpack(p);for c in 0..3{q[c]=((q[c] as u32*q[3] as u32+127)/255) as u8;}pack(p,q);}
        }
        self.rendered=Some(result);Ok(())
    }
    fn special(&mut self,k:usize,encoded:&[u8],budget:&mut Budget)->R<Buf<u8>>{
        if !matches!(self.meta.compression,3|4|6|7|50001|34676){return Err((NOTIMPL,"compression not implemented"));}
        let source=self.synthetic(k,encoded,budget)?;
        let once=if matches!(self.meta.compression,6|7){self.jpeg_once(k,&source,budget)?}else{None};
        let pixels=match once{Some(p)=>p,None=>{
            let image=inspect_raster(&source,0,&self.limits,budget).map_err(err)?;
            let mut pixels=Buf::zeros(image.output_len(),self.limits.max_output,&self.source.mem)?;
            let mut scratch=Buf::zeros(image.scratch_len(),self.limits.max_output,&self.source.mem)?;
            image.decode_with_scratch(&mut pixels,&mut scratch,budget).map_err(err)?;pixels
        }};
        let m=&self.meta;
        if m.bits==1{
            let rows=m.segment_rows(k)?;let row=m.row_bytes(m.bw)?;
            let mut out=Buf::zeros(product(row,rows)?,self.limits.max_output,&self.source.mem)?;
            pack_bilevel(&pixels,m.bw,rows,row,if m.photo==0{0xff}else{0},&mut out)?;Ok(out)
        }else if m.compression==34676 && !self.logluv16{
            let mut out=Buf::zeros(product(pixels.len(),2)?,self.limits.max_output,&self.source.mem)?;
            for (p,q) in pixels.chunks_exact(6).zip(out.chunks_exact_mut(12)){
                let l=u16::from_le_bytes([p[0],p[1]]);let u=u16::from_le_bytes([p[2],p[3]]) as u32;let v=u16::from_le_bytes([p[4],p[5]]) as u32;
                // The verified native expansion is injective for the 256
                // encoded chroma values. Recover them before float rendering.
                let xyz=if l==0||l&0x8000!=0{[0f32;3]}else{
                    let y=(std::f64::consts::LN_2/256.*(l as f64+0.5)-std::f64::consts::LN_2*64.).exp();
                    let u=((u*205/16384) as f64+0.5)/410.;let v=((v*205/16384) as f64+0.5)/410.;
                    let s=1./(6.*u-16.*v+12.);let xx=9.*u*s;let yy=4.*v*s;
                    [(xx/yy*y) as f32,y as f32,((1.-xx-yy)/yy*y) as f32]
                };
                for c in 0..3{q[c*4..c*4+4].copy_from_slice(&xyz[c].to_ne_bytes());}
            }Ok(out)
        }else if m.compression==34676 && cfg!(target_endian="big"){
            let mut out=Buf::zeros(pixels.len(),self.limits.max_output,&self.source.mem)?;
            ns::convert(&pixels,m.row_bytes(m.bw)?,m.ch,2,true,false,false,&mut out).map_err(err)?;Ok(out)
        }else{Ok(pixels)}
    }
    /// One validating entropy pass for a JPEG segment whose estimated buffers
    /// fit. The probe's work is charged whatever its outcome. An input error is
    /// final; an undersized estimate leaves the rest of the budget to the
    /// general path.
    fn jpeg_once(&self,k:usize,source:&[u8],budget:&mut Budget)->R<Option<Buf<u8>>>{
        let m=&self.meta;let Ok(rows)=m.segment_rows(k) else {return Ok(None)};
        let Some(len)=m.bw.checked_mul(rows).and_then(|n|n.checked_mul(m.ch)) else {return Ok(None)};
        // Each component has at most ceil(n/8)+3 blocks per axis (sampling <= 4).
        let blocks=(m.bw.div_ceil(8)+3).checked_mul(rows.div_ceil(8)+3).and_then(|n|n.checked_mul(m.ch));
        let Some(room)=blocks.and_then(|n|n.checked_mul(64)).and_then(|n|n.checked_add(len)) else {return Ok(None)};
        let room=room.min(self.limits.max_output);
        let Ok(mut pixels)=Buf::zeros(len,self.limits.max_output,&self.source.mem) else {return Ok(None)};
        let Ok(mut scratch)=Buf::zeros(room,self.limits.max_output,&self.source.mem) else {return Ok(None)};
        let mut probe=Budget{remaining:budget.remaining};
        let r=decode_jpeg_into(source,0,&self.limits,&mut pixels,&mut scratch,&mut probe);
        budget.remaining=probe.remaining;
        match r{
            Ok(image) if image.output_len()==len=>Ok(Some(pixels)),
            Ok(_)|Err(Error::OutputTooSmall)=>Ok(None),
            Err(e)=>Err(err(e)),
        }
    }
    fn synthetic(&mut self,k:usize,encoded:&[u8],budget:&mut Budget)->R<Buf<u8>>{
        let first_offset=self.meta.offsets.get(&mut self.source,0)?;
        let first_count=self.meta.counts.get(&mut self.source,0)?;
        let m=&self.meta;let rows=m.segment_rows(k)?;
        if m.planar!=1{return Err((NOTIMPL,"special codec planar layout not implemented"));}
        let jif=scalar(&self.data,&m.tags,513,0,m.le)?;
        let jlen=scalar(&self.data,&m.tags,514,0,m.le)? as usize;
        let cap=self.data.len().checked_add(encoded.len()).and_then(|n|n.checked_add(jlen)).and_then(|n|n.checked_add(8192)).ok_or((LIMIT,"segment container overflow"))?;
        let mut b=Builder::new(cap,&self.source.mem,m.le)?;
        b.number(256,4,m.bw as u32)?;b.number(257,4,rows as u32)?;
        b.repeated(258,m.bits as u16,m.ch)?;b.number(259,3,m.compression as u32)?;
        b.number(262,3,m.photo as u32)?;b.number(266,3,m.fill as u32)?;
        b.number(274,3,1)?;b.number(277,3,m.ch as u32)?;b.number(278,4,rows as u32)?;
        b.number(284,3,1)?;b.number(317,3,m.predictor as u32)?;
        b.repeated(339,m.format,m.ch)?;
        // WebP's output already carries the stored components. Suppress the
        // old raster API's lossy unassociation; the caller owns alpha meaning.
        if !m.extra.is_empty(){if m.compression==50001 && m.extra.len()==1{b.number(338,3,2)?;}else{b.raw_tag(&self.data,*find(&m.tags,338).as_ref().unwrap())?;}}
        for key in [292,293,347,512,515,529,530,531,532]{if key==515 && jif!=0{continue;}if let Some(t)=find(&m.tags,key){b.raw_tag(&self.data,t)?;}}
        for key in [519,520,521]{if jif!=0{continue;}if let Some(t)=find(&m.tags,key){
            if t.count>3{return Err((INVALID,"OJPEG table count"));}let mut offsets=[0u32;3];
            for i in 0..t.count{
                let at=value(&self.data,t,i,m.le)?;
                let n=if key==519{64}else{let counts=self.source.read(at,16)?;16+counts.iter().map(|v|*v as usize).sum::<usize>()};
                if n>272{return Err((INVALID,"OJPEG Huffman table size"));}
                let raw=self.source.read(at,n)?;offsets[i]=b.append(&raw)?;
            }
            b.longs(key,&offsets[..t.count])?;
        }}
        let mut renamed=None;
        if jif!=0{
            let raw=self.source.read(jif,jlen)?;
            let mut shared=Buf::new(jlen.checked_add(256).ok_or((LIMIT,"shared JPEG header overflow"))?,&self.source.mem)?;
            shared.extend(&raw)?;drop(raw);
            // An OJPEG interchange header can stop before SOS. Its first-strip
            // SOS is shared metadata: read only that bounded marker, not pixels.
            if first_count>=4{
                let first=self.source.read(first_offset,4)?;
                if first[..2]==[255,218]{let n=u16::from_be_bytes([first[2],first[3]]) as usize+2;
                    if n>256 || n as u64>first_count{return Err((INVALID,"OJPEG scan header size"));}
                    let scan=self.source.read(first_offset,n)?;shared.extend(&scan)?;
                }
            }
            let v=native::jpeg_shared(&shared,budget).map_err(err)?;
            if v[12]!=m.bw || v[13]!=m.height || v[14]!=m.ch{return Err((INVALID,"OJPEG shared frame geometry"));}
            let sub=find(&m.tags,530);let hs=if let Some(t)=sub{value(&self.data,t,0,m.le)? as usize}else{2};let vs=if let Some(t)=sub{value(&self.data,t,1,m.le)? as usize}else{2};
            if m.photo==6 && (v[15]!=hs || v[16]!=vs){return Err((INVALID,"OJPEG shared sampling"));}
            for (key,selectors,base) in [(519,20,0),(520,23,4),(521,26,8)]{
                let mut offsets=[0u32;3];
                if m.ch>3{return Err((INVALID,"OJPEG component count"));}
                for c in 0..m.ch{
                    let selector=v[selectors+c];if selector>=4{return Err((INVALID,"OJPEG table selector"));}
                    let at=v[base+selector];
                    let n=if key==519{64}else{let counts=shared.get(at..at.checked_add(16).ok_or((LIMIT,"OJPEG table range"))?).ok_or((INVALID,"OJPEG table range"))?;16+counts.iter().map(|x|*x as usize).sum::<usize>()};
                    let table=shared.get(at..at.checked_add(n).ok_or((LIMIT,"OJPEG table range"))?).ok_or((INVALID,"OJPEG table range"))?;
                    offsets[c]=b.append(table)?;
                }
                b.longs(key,&offsets[..m.ch])?;
            }
            b.number(515,3,v[29] as u32)?;
            if encoded.len()>=2 && encoded[..2]==[255,218]{
                let mut copy=Buf::new(encoded.len(),&self.source.mem)?;copy.extend(encoded)?;
                let mut ids=[0u8;3];let mut selectors=[0u8;3];
                for c in 0..m.ch{ids[c]=v[17+c] as u8;selectors[c]=(16*v[23+c]+v[26+c]) as u8;}
                native::renumber_ojpeg_scan(&mut copy,m.ch,ids,selectors).map_err(err)?;
                renamed=Some(copy);
            }
        }
        let at=b.append(renamed.as_ref().map_or(encoded,|s|&**s))?;b.number(273,4,at)?;b.number(279,4,u32::try_from(encoded.len()).map_err(|_|(LIMIT,"special segment length"))?)?;
        b.finish()
    }
}

// Unverified adapter packing of 0/255 bilevel samples, MSB first. For a u8
// sample v/255 is exactly v==255. `flip` inverts stored bits; padding stays zero.
fn pack_bilevel(pixels:&[u8],bw:usize,rows:usize,row:usize,flip:u8,out:&mut[u8])->R<()>{
    if bw==0 || row<bw.div_ceil(8) || pixels.len()<product(rows,bw)? || out.len()<product(rows,row)?{return Err((INVALID,"bilevel raster size"));}
    let bit=|p:u8,i:usize|((p==255) as u8)<<(7-i);
    for (line,dest) in pixels.chunks_exact(bw).take(rows).zip(out.chunks_exact_mut(row)){
        let mut groups=line.chunks_exact(8);let mut bytes=dest.iter_mut();
        for (g,d) in (&mut groups).zip(&mut bytes){*d=pack8(g)^flip;}
        let rest=groups.remainder();
        if !rest.is_empty(){if let Some(d)=bytes.next(){
            let mut b=0u8;for (i,&p) in rest.iter().enumerate(){b|=bit(p,i);}
            *d=(b^flip)&(0xffu8<<(8-rest.len()));
        }}
    }
    Ok(())
}

// Eight samples per word: a byte's high bit survives exactly when it is 255
// (0x7f-masked add, then OR, then NOT), and one multiply gathers the eight
// flags MSB-first without carries.
#[inline(always)]
fn pack8(g:&[u8])->u8{
    let x=u64::from_le_bytes([g[0],g[1],g[2],g[3],g[4],g[5],g[6],g[7]]);
    const L:u64=0x7f7f7f7f7f7f7f7f;
    let m=!x;let t=!(((m&L)+L)|m|L);
    ((t>>7).wrapping_mul(0x8040201008040201)>>56) as u8
}

// Presentation-only byte expansion, in the same unverified adapter boundary
// as palette selection and the former scalar packed-pixel conversion.
fn packed_rgba_row(data:&[u8],start:usize,n:usize,bits:usize,table:&[[u8;32];256],reverse:bool,out:&mut[u8]){
    let per=8/bits;let mut x=0;
    if !reverse{
        while x<n&&(start+x)%per!=0{let p=start+x;let j=p%per*4;out[x*4..x*4+4].copy_from_slice(&table[data[p/per] as usize][j..j+4]);x+=1;}
        // Whole source bytes expand to fixed-size table rows; no per-byte division.
        let whole=(n-x)/per*per;let (first,dst)=((start+x)/per,&mut out[x*4..(x+whole)*4]);
        match per{8=>packed_run::<32>(data,first,table,dst),4=>packed_run::<16>(data,first,table,dst),
            2=>packed_run::<8>(data,first,table,dst),_=>packed_run::<4>(data,first,table,dst)}
        x+=whole;
    }
    while x<n{let p=start+x;let j=p%per*4;let to=if reverse{n-1-x}else{x}*4;out[to..to+4].copy_from_slice(&table[data[p/per] as usize][j..j+4]);x+=1;}
}
fn packed_run<const N:usize>(data:&[u8],first:usize,table:&[[u8;32];256],out:&mut[u8]){
    let bytes=&data[first..first+out.len()/N];
    for (chunk,b) in out.chunks_exact_mut(N).zip(bytes){chunk.copy_from_slice(&table[*b as usize][..N]);}
}
fn rgba_row(raw:&[u8],n:usize,ch:usize,bits:usize,rgb:bool,invert:bool,alpha:Option<(usize,u16)>,reverse:bool,out:&mut[u8])->R<()>{
    use verified_tiff::native_rgba::row;
    let le=cfg!(target_endian="little");let a=alpha.map_or(ch,|(a,_)|a);let premul=rgb&&alpha.is_some_and(|(_,kind)|kind==2);
    // Constant layout arguments specialize the hot loops; unusual extra-channel
    // layouts use the same proved kernel with a dynamic stride.
    let r=match (bits,ch,rgb,invert,a,reverse){
        (8,3,true,false,3,false)=>row(raw,n,3,false,true,false,3,false,le,false,out),
        (8,4,true,false,3,false)=>row(raw,n,4,false,true,false,3,premul,le,false,out),
        (8,1,false,false,1,false)=>row(raw,n,1,false,false,false,1,false,le,false,out),
        (8,1,false,true,1,false)=>row(raw,n,1,false,false,true,1,false,le,false,out),
        (16,3,true,false,3,false)=>row(raw,n,3,true,true,false,3,false,le,false,out),
        (16,1,false,false,1,false)=>row(raw,n,1,true,false,false,1,false,le,false,out),
        _=>row(raw,n,ch,bits==16,rgb,invert,a,premul,le,reverse,out),
    };r.map_err(err)
}
pub fn sample(s:&[u8],row:usize,n:usize,bits:usize)->R<u32>{
    if bits<8{let bit=n*bits;Ok(((s[row+bit/8]>>(8-bits-bit%8))&((1u8<<bits)-1)) as u32)}
    else{ns::read_word(s,row+n*(bits/8),bits/8,cfg!(target_endian="little")).map(|v|v as u32).map_err(err)}
}
pub fn pack(p:&mut[u8],v:[u8;4]){p.copy_from_slice(&u32::from_le_bytes(v).to_ne_bytes());}
fn unpack(p:&[u8])->[u8;4]{u32::from_ne_bytes([p[0],p[1],p[2],p[3]]).to_le_bytes()}
struct Builder{bytes:Buf<u8>,tags:[(u16,u16,u32,[u8;4]);48],n:usize,le:bool}
impl Builder{
    fn new(cap:usize,mem:&Mem,le:bool)->R<Self>{let mut bytes=Buf::new(cap,mem)?;bytes.extend(&[0;590])?;Ok(Self{bytes,tags:[(0,0,0,[0;4]);48],n:0,le})}
    fn append(&mut self,s:&[u8])->R<u32>{let at=u32::try_from(self.bytes.len()).map_err(|_|(LIMIT,"synthetic segment offset"))?;self.bytes.extend(s)?;Ok(at)}
    fn tag(&mut self,key:u16,kind:u16,count:u32,s:&[u8])->R<()>{
        if self.n==48{return Err((LIMIT,"synthetic tag count"));}let mut v=[0;4];if s.len()<=4{v[..s.len()].copy_from_slice(s);}else{let at=self.append(s)?;v=if self.le{at.to_le_bytes()}else{at.to_be_bytes()};}
        self.tags[self.n]=(key,kind,count,v);self.n+=1;Ok(())
    }
    fn number(&mut self,key:u16,kind:u16,v:u32)->R<()>{
        if kind==3{let v=v as u16;self.tag(key,kind,1,&if self.le{v.to_le_bytes()}else{v.to_be_bytes()})}else{self.tag(key,kind,1,&if self.le{v.to_le_bytes()}else{v.to_be_bytes()})}
    }
    fn repeated(&mut self,key:u16,v:u16,n:usize)->R<()>{let mut s=[0u8;128];for chunk in s[..2*n].chunks_exact_mut(2){chunk.copy_from_slice(&if self.le{v.to_le_bytes()}else{v.to_be_bytes()});}self.tag(key,3,n as u32,&s[..2*n])}
    fn longs(&mut self,key:u16,v:&[u32])->R<()>{let mut s=[0;12];for (i,v) in v.iter().enumerate(){s[4*i..4*i+4].copy_from_slice(&if self.le{v.to_le_bytes()}else{v.to_be_bytes()});}self.tag(key,4,v.len() as u32,&s[..v.len()*4])}
    fn raw_tag(&mut self,s:&[u8],t:Tag)->R<()>{let n=product(t.count,container::field_size(t.kind))?;self.tag(t.key,t.kind,u32::try_from(t.count).map_err(|_|(LIMIT,"tag count overflow"))?,&s[t.at..t.at+n])}
    fn finish(mut self)->R<Buf<u8>>{
        self.tags[..self.n].sort_unstable_by_key(|t|t.0);
        self.bytes[..8].copy_from_slice(if self.le{b"II\x2a\0\x08\0\0\0"}else{b"MM\0\x2a\0\0\0\x08"});
        self.bytes[8..10].copy_from_slice(&if self.le{(self.n as u16).to_le_bytes()}else{(self.n as u16).to_be_bytes()});
        for (i,(key,kind,count,v)) in self.tags[..self.n].iter().enumerate(){let p=10+12*i;
            self.bytes[p..p+2].copy_from_slice(&if self.le{key.to_le_bytes()}else{key.to_be_bytes()});self.bytes[p+2..p+4].copy_from_slice(&if self.le{kind.to_le_bytes()}else{kind.to_be_bytes()});self.bytes[p+4..p+8].copy_from_slice(&if self.le{count.to_le_bytes()}else{count.to_be_bytes()});self.bytes[p+8..p+12].copy_from_slice(v);
        }Ok(self.bytes)
    }
}
