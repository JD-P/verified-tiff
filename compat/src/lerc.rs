//! TIFF's LERC adaptation: optional wrappers, band arrangement and validity
//! presentation. All compressed bytes are decoded by verified core APIs.
use super::*;
use verified_tiff::{native,native_lerc,native_zstd};

impl Handle {
    pub fn lerc(&self,encoded:&[u8],rows:usize,needed:usize,budget:&mut Budget)->R<Buf<u8>>{
        let m=&self.meta;let mem=&self.source.mem;
        if m.predictor!=1 || m.fill!=1{return Err((NOTIMPL,"LERC requires Predictor and FillOrder 1"));}
        let kind=match (m.format,m.bits){
            (2,8)=>0,(1|4,8)=>1,(2,16)=>2,(1|4,16)=>3,(2,32)=>4,(1|4,32)=>5,(3,32)=>6,(3,64)=>7,
            _=>return Err((NOTIMPL,"LERC sample type not supported")),
        };
        let channels=if m.planar==2{1}else{m.ch};
        let extra=match find(&m.tags,50674){
            Some(tag)=>{
                if tag.count!=2{return Err((INVALID,"LERC parameters require two values"));}
                let version=value(&self.data,tag,0,m.le)?;
                if version!=4{return Err((NOTIMPL,"LERC codec version must be 4"));}
                value(&self.data,tag,1,m.le)?
            },None=>0,
        };
        let mut unwrapped;
        let source=if extra==0{encoded}else{
            let cap=needed.checked_add(needed/3).and_then(|n|n.checked_add(256*channels)).ok_or((LIMIT,"LERC wrapper size overflow"))?;
            unwrapped=Buf::zeros(cap,self.limits.max_output,mem)?;
            let n=match extra{
                1=>native::inflate(encoded,cap,&mut unwrapped,budget).map_err(err)?,
                2=>{
                    let mut scratch=Buf::zeros(native_zstd::scratch_len(cap),self.limits.max_output,mem)?;
                    native_zstd::decode(encoded,0,cap,&mut unwrapped,&mut scratch,budget).map_err(err)?
                },_=>return Err((NOTIMPL,"LERC additional compression not supported")),
            };
            unwrapped.truncate(n);&unwrapped
        };
        let first=native_lerc::inspect(source).map_err(err)?;
        let alpha=m.planar==1 && kind==1 && m.extra.last()==Some(&2) && first.depth+1==channels;
        let banded=m.format==3 && channels>1 && first.depth==1;
        let bands=if banded{channels}else{1};let depth=if banded{1}else if alpha{channels-1}else{channels};
        let pixels=product(m.bw,rows)?;let count=product(pixels,depth)?;
        if product(count,8)?>self.limits.max_output{return Err((LIMIT,"LERC word scratch limit"));}
        let mut words=Buf::filled(count,0u64,mem)?;
        let mask_len=pixels.checked_add(7).ok_or((LIMIT,"LERC mask size overflow"))?/8;
        let mut mask=Buf::zeros(mask_len,self.limits.max_output,mem)?;
        let mut out=Buf::zeros(needed,self.limits.max_output,mem)?;let bytes=m.bits/8;let mut at=0usize;
        for band in 0..bands{
            let data=&source[at..];let info=native_lerc::inspect(data).map_err(err)?;
            if info.width!=m.bw || info.height!=rows || info.depth!=depth || info.datatype!=kind || info.samples!=count{
                return Err((INVALID,"LERC header does not match TIFF segment"));
            }
            native_lerc::decode(data,&mut words,&mut mask,band>0,budget).map_err(err)?;
            if needed!=product(product(pixels,channels)?,bytes)?{return Err((INVALID,"LERC output layout mismatch"));}
            match bytes {
                1=>pack::<1>(&words,&mask,&mut out,pixels,depth,channels,band,banded,alpha,kind),
                2=>pack::<2>(&words,&mask,&mut out,pixels,depth,channels,band,banded,alpha,kind),
                4=>pack::<4>(&words,&mask,&mut out,pixels,depth,channels,band,banded,alpha,kind),
                8=>pack::<8>(&words,&mask,&mut out,pixels,depth,channels,band,banded,alpha,kind),
                _=>unreachable!(),
            }
            at=at.checked_add(info.encoded_len).ok_or((LIMIT,"LERC blob size overflow"))?;
        }
        if at!=source.len(){return Err((INVALID,"unexpected trailing LERC bands or bytes"));}
        Ok(out)
    }
}

// Fixed-width stores allow LLVM to combine/vectorize packing. Application output
// remains private until the entire segment (all bands and endings) has passed.
#[inline(always)]
fn store<const N:usize>(dst:&mut[u8],bits:u64){
    let raw=bits.to_ne_bytes();
    if cfg!(target_endian="little"){dst.copy_from_slice(&raw[..N]);}
    else{dst.copy_from_slice(&raw[8-N..]);}
}
fn pack<const N:usize>(words:&[u64],mask:&[u8],out:&mut[u8],pixels:usize,depth:usize,channels:usize,band:usize,banded:bool,alpha:bool,kind:usize){
    if !banded && !alpha && mask.iter().all(|&b|b==255){
        for (dst,&bits) in out.chunks_exact_mut(N).zip(words){store::<N>(dst,bits);}
        return;
    }
    for p in 0..pixels{
        let valid=(mask[p/8]>>(7-p%8))&1!=0;
        for c in 0..depth{
            let bits=if !valid && kind==6{0x7fc00000u64}else if !valid && kind==7{0x7ff8000000000000u64}else{words[p*depth+c]};
            let target=(p*channels+if banded{band}else{c})*N;
            store::<N>(&mut out[target..target+N],bits);
        }
        if alpha{out[p*channels+channels-1]=if valid{255}else{0};}
    }
}
