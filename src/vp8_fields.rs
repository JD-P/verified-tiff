//! Conditional VP8 header fields and their arithmetic-stream semantics.
use super::*;
use crate::vp8_bool as ar;
verus! {
#[derive(Clone,Copy)]
pub(super) struct Field {pub kind:usize,pub n:usize,pub prob:u8,pub default:i32}
impl Field {
    pub(super) open spec fn valid(self) -> bool {self.kind<=4 && self.n<=8 && -255<=self.default<=255}
}
pub(super) open spec fn field(s:Seq<u8>,r:ar::Reader,f:Field) -> Option<(i32,ar::Reader)> {
    if f.kind==0{Some((f.default,r))}else if f.kind==1{
        let b=ar::symbol(s,r,f.prob);if b.is_none(){None}else{Some((if b.unwrap().0{1i32}else{0i32},b.unwrap().1))}
    }else{
        let opt=if f.kind>=3{ar::symbol(s,r,f.prob)}else{Some((true,r))};
        if opt.is_none(){None}else if !opt.unwrap().0{Some((f.default,opt.unwrap().1))}else{
            let v=ar::bits(s,opt.unwrap().1,f.n as nat,0);
            if v.is_none(){None}else if f.kind==4{
                let sign=ar::symbol(s,v.unwrap().1,128);
                if sign.is_none(){None}else{Some((if sign.unwrap().0{(-(v.unwrap().0 as int)) as i32}else{v.unwrap().0 as i32},sign.unwrap().1))}
            }else{Some((v.unwrap().0 as i32,v.unwrap().1))}
        }
    }
}
pub(super) fn parse(s:&[u8],r:ar::Reader,f:Field) -> (out:Result<(i32,ar::Reader),Error>)
    requires r.valid(s@),f.valid(),
    ensures out.is_ok()==field(s@,r,f).is_some(),
        out.is_ok() ==> out.unwrap()==field(s@,r,f).unwrap() && out.unwrap().1.valid(s@) && -255<=out.unwrap().0<=255,
{
    if f.kind==0{return Ok((f.default,r));}
    if f.kind==1{let (v,t)=ar::bit(s,r,f.prob)?;return Ok((if v{1}else{0},t));}
    let mut t=r;
    if f.kind>=3{let (present,after)=ar::bit(s,t,f.prob)?;t=after;if !present{return Ok((f.default,t));}}
    let (v,t)=ar::read(s,t,f.n)?;
    assert(v<=255);
    if f.kind==4{let (negative,t)=ar::bit(s,t,128)?;Ok((if negative{-(v as i32)}else{v as i32},t))}
    else{Ok((v as i32,t))}
}
}
