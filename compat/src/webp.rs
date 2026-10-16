//! Native WebP workspace ownership, ALPH framing and TIFF channel presentation.
use super::*;
use verified_tiff::{native_webp,native_vp8};

impl Handle {
    fn webp_lossless(&self,encoded:&[u8],w:usize,h:usize,budget:&mut Budget)->R<Buf<[u8;4]>>{
        let mem=&self.source.mem;let pixels=product(w,h)?;
        let count=native_webp::scratch_len(w,h).map_err(err)?;
        if product(count,4)?>self.limits.max_output{return Err((LIMIT,"WebP transform scratch limit"));}
        let mut scratch=Buf::filled(count,[0u8;4],mem)?;
        let groups=native_webp::required_groups(encoded,w,h,&mut scratch,budget).map_err(err)?;
        let mut tables=Buf::new(groups,mem)?;for _ in 0..groups{tables.push(native_webp::EntropyGroup::empty())?;}
        let mut rgba=Buf::filled(pixels,[0u8;4],mem)?;
        native_webp::decode(encoded,w,h,&mut tables,&mut scratch,&mut rgba,budget).map_err(err)?;
        Ok(rgba)
    }
    fn webp_lossy(&self,encoded:&[u8],w:usize,h:usize,container:native_webp::Container,budget:&mut Budget)->R<Buf<[u8;4]>>{
        let mem=&self.source.mem;let pixels=product(w,h)?;
        let mut rgba=Buf::filled(pixels,[0u8;4],mem)?;
        {
            let info=native_vp8::storage(w,h).map_err(err)?;
            let mut yuv=Buf::zeros(info.bytes,self.limits.max_output,mem)?;
            let mut columns=Buf::new(info.columns,mem)?;for _ in 0..info.columns{columns.push(native_vp8::Context::empty())?;}
            let mut filters=Buf::new(info.blocks,mem)?;for _ in 0..info.blocks{filters.push(native_vp8::Filter::empty())?;}
            native_vp8::decode_yuv(encoded,w,h,&mut yuv,&mut columns,&mut filters,budget).map_err(err)?;
            native_vp8::render_rgba(&yuv,w,h,&mut rgba,budget).map_err(err)?;
        }
        if container.alpha_start!=0{
            let alpha=&encoded[container.alpha_start..container.alpha_end];
            let header=native_webp::inspect_alpha(alpha).map_err(err)?;
            let mut mask=Buf::zeros(pixels,self.limits.max_output,mem)?;
            if header.compressed{
                // ALPH stores the ordinary VP8L image stream without its five
                // byte image header. Assemble that framing for the same core.
                let chunk=alpha.len().checked_add(4).ok_or((LIMIT,"WebP alpha framing overflow"))?;
                let total=chunk.checked_add(20+chunk%2).ok_or((LIMIT,"WebP alpha framing overflow"))?;
                let size=u32::try_from(chunk).map_err(|_|(LIMIT,"WebP alpha chunk too large"))?;
                let riff=u32::try_from(total-8).map_err(|_|(LIMIT,"WebP alpha container too large"))?;
                let mut stream=Buf::zeros(total,self.limits.max_output,mem)?;
                stream[..4].copy_from_slice(b"RIFF");stream[4..8].copy_from_slice(&riff.to_le_bytes());stream[8..16].copy_from_slice(b"WEBPVP8L");
                stream[16..20].copy_from_slice(&size.to_le_bytes());stream[20]=47;
                let dimensions=(w-1) as u32|(((h-1) as u32)<<14);stream[21..25].copy_from_slice(&dimensions.to_le_bytes());
                stream[25..25+alpha.len()-1].copy_from_slice(&alpha[1..]);
                let decoded=self.webp_lossless(&stream,w,h,budget)?;
                for (dst,src) in mask.iter_mut().zip(decoded.iter()){*dst=src[1];}
            }else{
                if alpha.len()-1!=pixels{return Err((INVALID,"WebP alpha plane size mismatch"));}
                mask.copy_from_slice(&alpha[1..]);
            }
            native_webp::unfilter_alpha(&mut mask,w,h,header.filter,budget).map_err(err)?;
            for (pixel,alpha) in rgba.iter_mut().zip(mask.iter()){pixel[3]=*alpha;}
        }
        Ok(rgba)
    }
    pub fn webp(&self,encoded:&[u8],rows:usize,needed:usize,budget:&mut Budget)->R<Buf<u8>>{
        let m=&self.meta;
        if m.bits!=8 || !matches!(m.format,1|4) || m.photo!=2 || m.planar!=1 || !matches!(m.ch,3|4) || m.predictor!=1 || m.fill!=1{
            return Err((NOTIMPL,"WebP requires contiguous 8-bit RGB or RGBA"));
        }
        let container=native_webp::inspect(encoded,m.bw,rows).map_err(err)?;
        let rgba=if container.lossless{self.webp_lossless(encoded,m.bw,rows,budget)?}else{self.webp_lossy(encoded,m.bw,rows,container,budget)?};
        let mut out=Buf::zeros(needed,self.limits.max_output,&self.source.mem)?;
        let bytes=product(rgba.len(),m.ch)?;
        if bytes>out.len(){return Err((INVALID,"WebP sample size mismatch"));}
        if m.ch==4{out[..bytes].copy_from_slice(rgba.as_flattened());}
        else{for (dst,pixel) in out[..bytes].chunks_exact_mut(3).zip(rgba.iter()){
            dst[0]=pixel[0];dst[1]=pixel[1];dst[2]=pixel[2];
        }}
        Ok(out)
    }
}
