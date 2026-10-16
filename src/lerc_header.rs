//! Lerc2 version 4 TIFF header and checksum. No dimensions or lengths are
//! trusted before their complete bounded record has been checked.
use super::*;
use crate::native_samples as ns;
use crate::lerc_numeric as num;
use crate::lerc_checksum as check;
verus! {
#[derive(Clone,Copy)]
pub(super) struct Header {
    pub width:usize,pub height:usize,pub depth:usize,pub valid:usize,pub block:usize,
    pub length:usize,pub kind:usize,pub error:u64,pub min:u64,pub max:u64,
    pub mask:usize,pub data:usize,
}
impl Header {
    pub(super) open spec fn invariant(self) -> bool {
        0<self.width<=2147483647 && 0<self.height<=2147483647 && 0<self.depth<=64 && self.kind<8 &&
        self.width as int*self.height<=usize::MAX/512 && self.valid<=self.width as int*self.height &&
        0<self.block<=16 && 70<=self.data<=self.length && self.data==70+self.mask && self.length<=usize::MAX/8 &&
        num::finite(self.error) && !num::sign(self.error) && self.error<0x7fe0000000000000u64 &&
        num::finite(self.min) && num::finite(self.max) && !num::less(self.max,self.min)
    }
}
pub(super) closed spec fn record(s:Seq<u8>) -> Option<Header> {
    if s.len()<70 || s.len()>usize::MAX/8 || s.take(6)!=seq![76u8,101u8,114u8,99u8,50u8,32u8] || word32(s,6,true)!=4{None}else{
        let h=Header {height:word32(s,14,true) as usize,width:word32(s,18,true) as usize,depth:word32(s,22,true) as usize,
            valid:word32(s,26,true) as usize,block:word32(s,30,true) as usize,length:word32(s,34,true) as usize,
            kind:word32(s,38,true) as usize,error:ns::word(s,42,8,true) as u64,min:ns::word(s,50,8,true) as u64,max:ns::word(s,58,8,true) as u64,
            mask:word32(s,66,true) as usize,data:(70+word32(s,66,true)) as usize};
        if !h.invariant() || h.length>s.len() || ((h.valid==0 || h.valid==h.width as int*h.height) && h.mask!=0){None}else{Some(h)}
    }
}
pub(super) fn read_record(s:&[u8]) -> (r:Result<Header,Error>)
    ensures r.is_ok()==record(s@).is_some(),r.is_ok() ==> r.unwrap()==record(s@).unwrap() && r.unwrap().invariant() && r.unwrap().length<=s.len(),
{
    reveal(record);
    if s.len()<70 || s.len()>usize::MAX/8{return Err(Error::Truncated);}
    if s[0]!=76 || s[1]!=101 || s[2]!=114 || s[3]!=99 || s[4]!=50 || s[5]!=32 || read32(s,6,true)?!=4{return Err(Error::Unsupported);}
    assert(s@.take(6) =~= seq![76u8,101u8,114u8,99u8,50u8,32u8]);
    let height=read32(s,14,true)? as usize;let width=read32(s,18,true)? as usize;let depth=read32(s,22,true)? as usize;
    let valid=read32(s,26,true)? as usize;let block=read32(s,30,true)? as usize;let length=read32(s,34,true)? as usize;
    let kind=read32(s,38,true)? as usize;let error=ns::read_word(s,42,8,true)?;let min=ns::read_word(s,50,8,true)?;let max=ns::read_word(s,58,8,true)?;
    let mask=read32(s,66,true)? as usize;
    if width==0 || width>2147483647 || height==0 || height>2147483647 || depth==0 || depth>64 || kind>=8 || block==0 || block>16{return Err(Error::Invalid);}
    let pixels=mul(width,height)?;
    if pixels>usize::MAX/512 || valid>pixels || length<70 || length>s.len() || mask>length-70{return Err(Error::Invalid);}
    if (error/0x10000000000000u64)%2048==2047 || error>=0x7fe0000000000000u64 || (min/0x10000000000000u64)%2048==2047 || (max/0x10000000000000u64)%2048==2047{return Err(Error::Invalid);}
    if num::minimum(max,min)!=min{return Err(Error::Invalid);}
    if (valid==0 || valid==pixels) && mask!=0{return Err(Error::Invalid);}
    let h=Header {width,height,depth,valid,block,length,kind,error,min,max,mask,data:70+mask};
    Ok(h)
}
#[verifier::opaque]
pub(super) open spec fn inspected(s:Seq<u8>) -> Option<Header> {
    let h=record(s);
    if h.is_none(){None}else{
        let v=h.unwrap();if check::checksum(s.take(v.length as int),14)!=word32(s,10,true){None}else{h}
    }
}
pub(super) fn inspect(s:&[u8]) -> (r:Result<Header,Error>)
    ensures r.is_ok()==inspected(s@).is_some(),r.is_ok() ==> r.unwrap()==inspected(s@).unwrap() && r.unwrap().invariant() && r.unwrap().length<=s.len(),
{
    reveal(inspected);let h=read_record(s)?;
    let (blob,_)=s.split_at(h.length);let sum=check::compute(blob,14);
    if sum!=read32(s,10,true)?{return Err(Error::Invalid);}
    Ok(h)
}
}
