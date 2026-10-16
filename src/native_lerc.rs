//! Allocation-free Lerc2 v4 decoding, the version emitted by libtiff's LERC
//! codec. Words contain exact native sample bits; invalid pixels remain zero
//! and their validity mask is returned separately for TIFF's alpha/NaN policy.
use super::*;
use crate::lerc_header::{self as header,Header};
use crate::lerc_planes as planes;
use crate::lerc_image as image;
use crate::lerc_byte_image as bytes;
use crate::lerc_block::ResultModel;
verus! {
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Info {pub width:usize,pub height:usize,pub depth:usize,pub datatype:usize,pub encoded_len:usize,pub samples:usize,pub mask_len:usize}
pub(super) open spec fn info(h:Header) -> Info {Info {width:h.width,height:h.height,depth:h.depth,datatype:h.kind,encoded_len:h.length,samples:(h.width as int*h.height*h.depth) as usize,mask_len:planes::mask_size(h) as usize}}
fn description(h:Header) -> (r:Info)
    requires h.invariant(),ensures r==info(h),r.samples==h.width as int*h.height*h.depth,r.mask_len==planes::mask_size(h),
{
    assert(h.width*h.height*h.depth<=usize::MAX) by(nonlinear_arith) requires h.width*h.height<=usize::MAX/512,h.depth<=64;
    Info {width:h.width,height:h.height,depth:h.depth,datatype:h.kind,encoded_len:h.length,samples:h.width*h.height*h.depth,mask_len:(h.width*h.height+7)/8}
}
pub closed spec fn metadata(s:Seq<u8>) -> Option<Info> {
    let h=header::record(s);if h.is_none(){None}else{Some(info(h.unwrap()))}
}
/// Parses bounded metadata. Checksum, mask, entropy and exact ending are
/// validated by decode, before a TIFF adapter exposes any output to its caller.
pub fn inspect(s:&[u8]) -> (r:Result<Info,Error>)
    ensures r.is_ok()==metadata(s@).is_some(),r.is_ok() ==> r.unwrap()==metadata(s@).unwrap(),
{
    reveal(metadata);let h=header::read_record(s)?;Ok(description(h))
}
pub open spec fn fee(encoded:int,samples:int,pixels:int) -> int {8*encoded+4096*samples+8*pixels+256}
pub closed spec fn work(s:Seq<u8>) -> int {
    let h=header::record(s);if h.is_none(){-1}else{let v=h.unwrap();fee(v.length as int,v.width as int*v.height*v.depth,v.width as int*v.height)}
}
fn cost(h:Header) -> (r:Result<usize,Error>)
    requires h.invariant(),
    ensures r.is_ok()==(fee(h.length as int,h.width as int*h.height*h.depth,h.width as int*h.height)<=usize::MAX),
        r.is_ok() ==> r.unwrap()==fee(h.length as int,h.width as int*h.height*h.depth,h.width as int*h.height),
{
    let d=description(h);add(add(add(mul(h.length,8)?,mul(d.samples,4096)?)?,mul(h.width*h.height,8)?)?,256)
}
pub fn work_limit(s:&[u8]) -> (r:Result<usize,Error>)
    ensures r.is_ok()==(0<=work(s@)<=usize::MAX),r.is_ok() ==> r.unwrap()==work(s@),
{reveal(work);let h=header::read_record(s)?;cost(h)}
pub(super) closed spec fn pixels(s:Seq<u8>,h:Header,mask:Seq<u8>,initial:Seq<u64>) -> Option<ResultModel> {
    if h.valid==0{Some(ResultModel {at:h.data,data:initial})}else{
        let rr=planes::ranges(s,h);
        if rr.is_none(){None}else{
            let r=rr.unwrap();
            if r.constant{
                let cc=planes::constants(r.min,h.depth,h.kind);
                if cc.is_none(){None}else{image::linear(s,h,mask,cc.unwrap(),false,0,r.at,initial,initial.len())}
            }else if r.at>=s.len(){None}else if s[r.at as int]!=0{
                image::linear(s,h,mask,Seq::new(64,|i:int|0u64),true,0,(r.at+1) as usize,initial,initial.len())
            }else{
                let entropy=h.kind<2 && h.error==0x3fe0000000000000u64;let at=r.at as int+1;
                if entropy && (at>=s.len() || s[at]>2){None}
                else if entropy && s[at]!=0{bytes::decoded(s,h,mask,(at+1) as usize,s[at]==1,initial)}
                else{image::tiles(s,h,mask,r.max,image::Position {x:0,y:0,d:0},(at+if entropy{1int}else{0int}) as usize,initial,initial.len())}
            }
        }
    }
}
fn process(s:&[u8],h:Header,valid:&[u8],out:&mut[u64]) -> (r:Result<usize,Error>)
    requires h.invariant(),h.length==s.len(),planes::mask_size(h)<=valid.len(),old(out).len()==h.width as int*h.height*h.depth,
        forall|i:int|0<=i<old(out).len() ==> old(out)@[i]==0,
    ensures final(out).len()==old(out).len(),r.is_ok()==pixels(s@,h,valid@,old(out)@).is_some(),
        r.is_ok() ==> r.unwrap()==pixels(s@,h,valid@,old(out)@).unwrap().at && final(out)@==pixels(s@,h,valid@,old(out)@).unwrap().data,
        r.is_ok() ==> r.unwrap()<=s.len(),
{
    reveal(pixels);
    if h.valid==0{return Ok(h.data);}
    let ranges=planes::read_ranges(s,h)?;
    if ranges.constant{
        let constants=planes::convert(&ranges.min,h.depth,h.kind)?;
        return image::decode_linear(s,h,valid,&constants,false,ranges.at,out);
    }
    let mut at=ranges.at;
    if at==s.len(){return Err(Error::Truncated);}
    let raw=s[at];at=at+1;
    if raw!=0{
        let constants=[0u64;64];assert(constants@ =~= Seq::new(64,|i:int|0u64));
        return image::decode_linear(s,h,valid,&constants,true,at,out);
    }
    if h.kind<2 && h.error==0x3fe0000000000000u64{
        if at==s.len() || s[at]>2{return Err(Error::Invalid);}
        let mode=s[at];at=at+1;
        if mode!=0{return bytes::decode(s,h,valid,at,mode==1,out);}
    }
    image::decode_tiles(s,h,valid,&ranges.max,at,out)
}
pub closed spec fn decoded(s:Seq<u8>,previous_mask:Seq<u8>,reuse:bool) -> Option<(Info,Seq<u64>,Seq<u8>)> {
    let hh=header::inspected(s);
    if hh.is_none(){None}else{
        let h=hh.unwrap();let mask=planes::validity(s.take(h.length as int),h,previous_mask,reuse);
        if mask.is_none(){None}else{
            let zeros=Seq::new((h.width as int*h.height*h.depth) as nat,|i:int|0u64);
            let result=pixels(s.take(h.length as int),h,mask.unwrap(),zeros);
            if result.is_none() || result.unwrap().at!=h.length{None}else{Some((info(h),result.unwrap().data,mask.unwrap()))}
        }
    }
}
/// Consumes one complete Lerc2 blob. A subsequent blob can reuse this mask
/// only when the caller explicitly selects reuse and dimensions agree.
/// Output and mask prefixes may change on failure; suffixes never change.
pub fn decode(s:&[u8],out:&mut[u64],mask:&mut[u8],reuse:bool,budget:&mut Budget) -> (r:Result<Info,Error>)
    ensures final(out).len()==old(out).len(),final(mask).len()==old(mask).len(),final(budget).remaining<=old(budget).remaining,
        r.is_ok()==(metadata(s@).is_some() && old(out).len()>=metadata(s@).unwrap().samples && old(mask).len()>=metadata(s@).unwrap().mask_len
            && decoded(s@,old(mask)@,reuse).is_some() && 0<=work(s@)<=old(budget).remaining),
        r.is_ok() ==> r.unwrap()==decoded(s@,old(mask)@,reuse).unwrap().0,
        r.is_ok() ==> final(out)@.take(r.unwrap().samples as int)==decoded(s@,old(mask)@,reuse).unwrap().1,
        r.is_ok() ==> final(mask)@.take(r.unwrap().mask_len as int)==decoded(s@,old(mask)@,reuse).unwrap().2,
        r.is_ok() ==> final(budget).remaining as int+work(s@)==old(budget).remaining,
        metadata(s@).is_some() ==> forall|j:int|metadata(s@).unwrap().samples<=j<old(out).len() ==> final(out)@[j]==old(out)@[j],
        metadata(s@).is_some() ==> forall|j:int|metadata(s@).unwrap().mask_len<=j<old(mask).len() ==> final(mask)@[j]==old(mask)@[j],
{
    reveal(metadata);reveal(work);reveal(decoded);reveal(header::inspected);reveal(planes::validity);
    let h=header::read_record(s)?;let desc=description(h);
    if out.len()<desc.samples || mask.len()<desc.mask_len{return Err(Error::OutputTooSmall);}
    let fee=cost(h)?;spend(budget,fee)?;
    let checked=header::inspect(s)?;
    assert(checked==h);
    let ghost previous=mask@;
    let (blob,_)=s.split_at(h.length);
    let (valid,_)=mask.split_at_mut(desc.mask_len);
    assert(valid@.take(desc.mask_len as int) =~= previous.take(desc.mask_len as int));
    let ghost wanted=planes::validity(blob@,h,previous,reuse);
    assert(wanted==planes::validity(blob@,h,valid@,reuse));
    planes::prepare(blob,h,reuse,valid)?;
    assert(valid@ =~= valid@.take(desc.mask_len as int));
    assert(wanted==Some(valid@));
    let (words,_)=out.split_at_mut(desc.samples);
    let mut i=0usize;
    while i<words.len()
        invariant i<=words.len(),words.len()==desc.samples,
            forall|j:int|0<=j<i ==> words@[j]==0,
        decreases words.len()-i,
    {words[i]=0;i=i+1;}
    assert(words@ =~= Seq::new(desc.samples as nat,|i:int|0u64));
    let at=process(blob,h,valid,words)?;
    if at!=h.length{return Err(Error::Invalid);}
    Ok(desc)
}
}
