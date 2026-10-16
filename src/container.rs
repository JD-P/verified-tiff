//! Local Classic TIFF/BigTIFF header and directory-entry contracts for seekable I/O.
//! The caller supplies only the requested record, plus its actual file offset.
use super::*;
use crate::native_samples::read_word;
verus! {
use crate::native_samples as ns;
#[derive(Clone,Copy,Debug)]
pub struct Header {pub little_endian:bool,pub big:bool,pub first_ifd:u64}
pub closed spec fn header_valid(s:Seq<u8>) -> bool {
    &&& s.len()>=8
    &&& (s[0]==73 && s[1]==73 || s[0]==77 && s[1]==77)
    &&& if word16(s,2,s[0]==73)==42 {true}
        else {s.len()>=16 && word16(s,2,s[0]==73)==43 && word16(s,4,s[0]==73)==8 && word16(s,6,s[0]==73)==0}
}
pub fn header(s:&[u8]) -> (r:Result<Header,Error>)
    ensures r.is_ok()==header_valid(s@),
        r.is_ok() ==> r.unwrap().little_endian==(s@[0]==73),
        r.is_ok() ==> r.unwrap().big==(word16(s@,2,s@[0]==73)==43),
        r.is_ok() ==> r.unwrap().first_ifd==ns::word(s@,if r.unwrap().big {8int} else {4int},if r.unwrap().big {8usize} else {4usize},r.unwrap().little_endian),
{
    reveal(header_valid);
    if s.len()<8 {return Err(Error::Truncated);}
    let le=if s[0]==73 && s[1]==73 {true} else if s[0]==77 && s[1]==77 {false} else {return Err(Error::Invalid);};
    let magic=read16(s,2,le)?;
    let big=if magic==42 {false} else if magic==43 {
        if s.len()<16 || read16(s,4,le)?!=8 || read16(s,6,le)?!=0 {return Err(Error::Invalid);}
        true
    } else {return Err(Error::Unsupported);};
    let first_ifd=read_word(s,if big {8} else {4},if big {8} else {4},le)?;
    Ok(Header {little_endian:le,big,first_ifd})
}
pub open spec fn field_width(kind:u16) -> int {
    if kind==1 || kind==2 || kind==6 || kind==7 {1}
    else if kind==3 || kind==8 {2}
    else if kind==4 || kind==9 || kind==11 || kind==13 {4}
    else if kind==5 || kind==10 || kind==12 || kind==16 || kind==17 || kind==18 {8}
    else {0}
}
pub fn field_size(kind:u16) -> (r:usize) ensures r==field_width(kind) {
    if kind==1 || kind==2 || kind==6 || kind==7 {1}
    else if kind==3 || kind==8 {2}
    else if kind==4 || kind==9 || kind==11 || kind==13 {4}
    else if kind==5 || kind==10 || kind==12 || kind==16 || kind==17 || kind==18 {8}
    else {0}
}
#[derive(Clone,Copy,Debug)]
pub struct Entry {pub tag:u16,pub kind:u16,pub count:u64,pub offset:u64,pub length:u64,pub inline:bool}
pub closed spec fn entry_valid(s:Seq<u8>,at:u64,big:bool,le:bool,file_len:u64) -> bool {
    let size=if big {20int} else {12int};let slot=if big {8usize} else {4usize};
    let kind=word16(s,2,le);let count=ns::word(s,4,slot,le);let len=count*field_width(kind as u16);
    let off=if len<=slot {at as int+size-slot} else {ns::word(s,size-slot,slot,le)};
    &&& s.len()>=size && at as int+size<=file_len
    &&& field_width(kind as u16)>0 && 0<=len<=u64::MAX
    &&& 0<=off<=file_len && off+len<=file_len
}
pub fn entry(s:&[u8],at:u64,big:bool,le:bool,file_len:u64) -> (r:Result<Entry,Error>)
    ensures r.is_ok()==entry_valid(s@,at,big,le,file_len),
        r.is_ok() ==> r.unwrap().tag==word16(s@,0,le) && r.unwrap().kind==word16(s@,2,le),
        r.is_ok() ==> r.unwrap().count==ns::word(s@,4,if big {8usize} else {4usize},le),
        r.is_ok() ==> r.unwrap().length as int==r.unwrap().count as int*field_width(r.unwrap().kind),
        r.is_ok() ==> r.unwrap().offset as int+r.unwrap().length<=file_len,
        r.is_ok() ==> r.unwrap().inline==(r.unwrap().length<=if big {8u64} else {4u64}),
        r.is_ok() ==> r.unwrap().offset as int==(if r.unwrap().inline {at as int+if big {12int} else {8int}} else {ns::word(s@,if big {12int} else {8int},if big {8usize} else {4usize},le)}),
{
    reveal(entry_valid);
    let size=if big {20usize} else {12usize};let slot=if big {8usize} else {4usize};
    if s.len()<size || at>file_len || size as u64>file_len-at {return Err(Error::Truncated);}
    let tag=read16(s,0,le)?;let kind=read16(s,2,le)?;let width=field_size(kind);
    if width==0 {return Err(Error::Unsupported);}
    let count=read_word(s,4,slot,le)?;
    let length=count as u128*width as u128;
    if length>u64::MAX as u128 {return Err(Error::Limit);}
    let length=length as u64;let inline=length<=slot as u64;
    let offset=if inline {at+(size-slot) as u64} else {read_word(s,size-slot,slot,le)?};
    if offset>file_len || length>file_len-offset {return Err(Error::Truncated);}
    Ok(Entry {tag,kind,count,offset,length,inline})
}
}
