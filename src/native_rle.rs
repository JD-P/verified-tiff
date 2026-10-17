//! CCITT Modified Huffman (TIFF Compression 2), byte-aligned rows, both
//! FillOrders. Reuses the verified CCITT run and row models. Padding is zero;
//! no damaged-row recovery, EOL markers, or whole trailing bytes are accepted.
use super::*;
use crate::{fax as fc,fax_bits::Bits,fax_frame::Config};
verus! {
pub closed spec fn accepted(s:Seq<u8>,w:usize,h:usize,lsb:bool) -> bool {
    let c=Config{group4:false,mixed:false,aligned:true,missing:true};
    let b=Bits{pos:0,end:(8*s.len()) as usize,lsb:if lsb {1usize} else {0usize}};
    0<w<=65536 && h>0 && w as int*h<=usize::MAX && 8*s.len()<usize::MAX
        && work_bound(s.len() as int,w as int*h,h as int)<=usize::MAX
        && fc::finish(s,c,w,fc::initial(b,w),h as nat).is_some()
}
pub open spec fn work_bound(n:int,pixels:int,rows:int)->int{64*n+4*pixels+64*rows+1}
pub closed spec fn pixel(s:Seq<u8>,w:usize,h:usize,lsb:bool,k:int)->u8{
    fc::output_byte(s,Config{group4:false,mixed:false,aligned:true,missing:true},w,Bits{pos:0,end:(8*s.len()) as usize,lsb:if lsb {1usize} else {0usize}},h,k)
}
/// Expanded bits: 0 for a white run, 255 for a black run. TIFF presentation
/// polarity is a separate step. Failure preserves the output suffix.
pub fn decode(s:&[u8],w:usize,h:usize,lsb:bool,out:&mut [u8],budget:&mut Budget)->(r:Result<(),Error>)
    ensures r.is_ok()==(accepted(s@,w,h,lsb)&&old(out).len()>=w as int*h&&old(budget).remaining as int>=work_bound(s.len() as int,w as int*h,h as int)),
        final(out).len()==old(out).len(),final(budget).remaining<=old(budget).remaining,
        r.is_ok() ==> final(budget).remaining as int+work_bound(s.len() as int,w as int*h,h as int)==old(budget).remaining,
        r.is_ok() ==> forall|k:int| 0<=k<w as int*h ==> final(out)@[k]==pixel(s@,w,h,lsb,k),
        forall|k:int| w as int*h<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
{
    reveal(accepted);reveal(pixel);
    if w==0||w>65536||h==0{return Err(Error::Unsupported);}
    let pixels=mul(w,h)?;let end=mul(s.len(),8)?;if end==usize::MAX{return Err(Error::Limit);}
    let quota=add(add(add(mul(s.len(),64)?,mul(pixels,4)?)?,mul(h,64)?)?,1)?;
    if out.len()<pixels{return Err(Error::OutputTooSmall);}
    spend(budget,quota)?;
    fc::process(s,Config{group4:false,mixed:false,aligned:true,missing:true},w,h,Bits{pos:0,end,lsb:if lsb {1usize} else {0usize}},out,true)
}
}
