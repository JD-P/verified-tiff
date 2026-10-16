//! Typed Lerc2 values and reduced offset types, with exact stored-bit output.
use super::*;
use crate::native_samples as ns;
use crate::lerc_numeric as num;
verus! {
#[verifier::opaque]
pub(super) open spec fn number(s:Seq<u8>,at:usize,kind:usize) -> Option<u64> {
    if kind>=8 || !range_ok(s.len() as int,at as int,num::type_size(kind) as int){None}
    else{Some(num::value(ns::word(s,at as int,num::type_size(kind),true) as u64,kind))}
}
pub(super) fn read(s:&[u8],at:usize,kind:usize) -> (r:Result<u64,Error>)
    ensures r.is_ok()==number(s@,at,kind).is_some(),r.is_ok() ==> r.unwrap()==number(s@,at,kind).unwrap(),
        r.is_ok() ==> kind<8 && at as int+num::type_size(kind)<=s.len(),
{
    reveal(number);if kind>=8{return Err(Error::Invalid);}
    let bytes=num::size(kind);let raw=ns::read_word(s,at,bytes,true)?;
    Ok(num::number(raw,kind))
}
#[verifier::opaque]
pub(super) open spec fn storage(v:u64,kind:usize) -> Option<u64> {
    if kind>=8{None}else if kind==7{Some(v)}else if kind==6{Some(num::narrowed(v) as u64)}else{num::integer(v,kind)}
}
#[inline(always)]
pub(super) fn stored(v:u64,kind:usize) -> (r:Result<u64,Error>)
    ensures r.is_ok()==storage(v,kind).is_some(),r.is_ok() ==> r.unwrap()==storage(v,kind).unwrap(),
{
    reveal(storage);
    if kind>=8{return Err(Error::Invalid);}
    if kind==7{Ok(v)}else if kind==6{Ok(num::narrow(v) as u64)}else{num::to_integer(v,kind)}
}
pub(super) open spec fn reduced(kind:usize,code:usize) -> Option<usize> {
    let t=if kind==2 || kind==4{kind as int-code}else if kind==3 || kind==5{kind as int-2*code}
        else if kind==6{if code==0{6int}else if code==1{2int}else{1int}}
        else if kind==7{if code==0{7int}else{8-2*code}}else{kind as int};
    if kind>=8 || code>3 || t<0 || t>=8{None}else{Some(t as usize)}
}
pub(super) fn offset_kind(kind:usize,code:usize) -> (r:Result<usize,Error>)
    ensures r.is_ok()==reduced(kind,code).is_some(),r.is_ok() ==> r.unwrap()==reduced(kind,code).unwrap() && r.unwrap()<8,
{
    if kind>=8 || code>3{return Err(Error::Invalid);}
    let t=if kind==2 || kind==4{kind as i32-code as i32}else if kind==3 || kind==5{kind as i32-2*code as i32}
        else if kind==6{if code==0{6i32}else if code==1{2i32}else{1i32}}
        else if kind==7{if code==0{7i32}else{8-2*code as i32}}else{kind as i32};
    if t<0 || t>=8{return Err(Error::Invalid);}Ok(t as usize)
}
}
