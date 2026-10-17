use super::*;
use crate::deflate_bits::{self as db,Bits};
use crate::webp_huffman as wh;
use crate::webp_header as hd;
verus! {
#[derive(Clone,Copy)]
pub(super) struct Config {pub bits:Bits,pub w:usize,pub h:usize,pub subtract:bool,pub cache:usize}
impl Config {
    pub(super) open spec fn valid(self,s:Seq<u8>) -> bool {self.bits.valid(s) && 1<=self.w<=16384 && 1<=self.h<=16384 && self.cache<=11}
}
#[verifier::opaque]
pub(super) open spec fn container(s:Seq<u8>,off:int,count:int,w:int,h:int) -> Option<Config> {
    if !range_ok(s.len() as int,off,count) || count<26 {None} else {
        let size=word32(s,off+16,true);let end=off+20+size;let v=word32(s,off+21,true);
        if word32(s,off,true)!=1179011410 || word32(s,off+4,true)!=count-8 || word32(s,off+8,true)!=1346520407 ||
            word32(s,off+12,true)!=1278758998 || size<6 || size+20+size%2!=count || (size%2==1 && s[end]!=0) ||
            s[off+20]!=47 || v/536870912!=0 || v%16384+1!=w || (v/16384)%16384+1!=h {None}
        else {
            let p=(off+25)*8;let sub=db::field(s,p,1)==1;let q=p+if sub {4int} else {1int};
            if p+1>end*8 || (sub && (p+4>end*8 || db::field(s,p+1,2)!=2 || db::field(s,p+3,1)!=0)) {None}
            else {
                let flag=db::field(s,q,1);let cb=if flag==0 {0int} else {db::field(s,q+1,4)};let r=q+if flag==0 {1int} else {5int};
                if q+1>end*8 || r+1>end*8 || (flag!=0 && (cb==0 || cb>11)) || db::field(s,r,1)!=0 {None}
                else {Some(Config {bits:Bits {at:((r+1)/8) as usize,bit:((r+1)%8) as usize,end:end as usize},w:w as usize,h:h as usize,subtract:sub,cache:cb as usize})}
            }
        }
    }
}
pub(super) fn open(s:&[u8],off:usize,count:usize,w:usize,h:usize) -> (r:Result<Config,Error>)
    requires 1<=w<=16384,1<=h<=16384,
    ensures r.is_ok()==container(s@,off as int,count as int,w as int,h as int).is_some(),
        r.is_ok() ==> r.unwrap()==container(s@,off as int,count as int,w as int,h as int).unwrap() && r.unwrap().valid(s@),
{
    reveal(container);
    if !checked_range(s.len(),off,count) || count<26 {return Err(Error::Truncated);}
    let size=read32(s,off+16,true)? as usize;
    if read32(s,off,true)?!=1179011410 || read32(s,off+4,true)? as usize!=count-8 || read32(s,off+8,true)?!=1346520407 ||
        read32(s,off+12,true)?!=1278758998 {return Err(Error::Unsupported);}
    if size<6 || size>count-20 || size!=count-20-size%2 {return Err(Error::Invalid);}
    let end=off+20+size;
    if size%2==1 && s[end]!=0 {return Err(Error::Invalid);}
    let v=read32(s,off+21,true)? as usize;
    if s[off+20]!=47 || v/536870912!=0 || v%16384+1!=w || (v/16384)%16384+1!=h {return Err(Error::Unsupported);}
    let mut bits=Bits {at:off+25,bit:0,end};
    let subtract=bits.read(s,1)?!=0;
    if subtract {if bits.read(s,2)?!=2 || bits.read(s,1)?!=0 {return Err(Error::Unsupported);}}
    let cache=if bits.read(s,1)?==0 {0usize} else {let n=bits.read(s,4)?;if n==0 || n>11 {return Err(Error::Invalid);}n};
    if bits.read(s,1)?!=0 {return Err(Error::Unsupported);}
    assert(bits.at as int==bits.pos()/8 && bits.bit as int==bits.pos()%8) by(nonlinear_arith)
        requires bits.bit<8,bits.pos()==bits.at as int*8+bits.bit;
    Ok(Config {bits,w,h,subtract,cache})
}
pub(super) struct GroupModel {pub g:wh::HModel,pub r:wh::HModel,pub b:wh::HModel,pub a:wh::HModel,pub d:wh::HModel}
#[derive(Clone,Copy)]
pub struct Group {pub(super) g:wh::Huff,pub(super) r:wh::Huff,pub(super) b:wh::Huff,pub(super) a:wh::Huff,pub(super) d:wh::Huff}
impl Group {
    /// Fixed-size workspace. The decoder overwrites it before reading tables.
    pub fn empty() -> Self {
        let h=wh::Huff {lens:Ghost(Seq::new(2328,|i:int| 0u8)),counts:[0usize;16],offsets:[0usize;16],symbols:[0u16;2328]};
        Self {g:h,r:h,b:h,a:h,d:h}
    }
    pub(super) open spec fn valid(&self) -> bool {self.g.valid() && self.r.valid() && self.b.valid() && self.a.valid() && self.d.valid()}
    pub(super) open spec fn model(&self) -> GroupModel {GroupModel {g:self.g.model(),r:self.r.model(),b:self.b.model(),a:self.a.model(),d:self.d.model()}}
}
pub(super) open spec fn cache_size(cb:int) -> int {if cb==0 {0} else {db::pow(cb)}}
#[inline(always)]
pub(super) fn size(cb:usize) -> (r:usize) requires cb<=11,ensures r as int==cache_size(cb as int),r<=2048, {
    if cb==0 {0} else {
        proof {crate::bit_window::powers(cb as u32);}
        assert((1usize<<cb)==(1u32<<(cb as u32)) as usize) by(bit_vector) requires 1<=cb<=11;
        1usize<<cb
    }
}
#[verifier::opaque]
pub(super) open spec fn group(s:Seq<u8>,p:int,end:int,cb:int) -> Option<(int,GroupModel)> {
    let g=hd::table(s,p,end,280+cache_size(cb));if g.is_none() {None} else {
    let r=hd::table(s,g.unwrap().0,end,256);if r.is_none() {None} else {
    let b=hd::table(s,r.unwrap().0,end,256);if b.is_none() {None} else {
    let a=hd::table(s,b.unwrap().0,end,256);if a.is_none() {None} else {
    let d=hd::table(s,a.unwrap().0,end,40);if d.is_none() {None} else {
    Some((d.unwrap().0,GroupModel {g:g.unwrap().1,r:r.unwrap().1,b:b.unwrap().1,a:a.unwrap().1,d:d.unwrap().1}))
    }}}}}
}
pub(super) fn read_group(s:&[u8],bits:&mut Bits,cb:usize) -> (r:Result<Group,Error>)
    requires old(bits).valid(s@),cb<=11,
    ensures final(bits).valid(s@),final(bits).end==old(bits).end,final(bits).pos()>=old(bits).pos(),
        r.is_ok()==group(s@,old(bits).pos(),old(bits).end as int,cb as int).is_some(),
        r.is_ok() ==> r.unwrap().valid(),
        r.is_ok() ==> (final(bits).pos(),r.unwrap().model())==group(s@,old(bits).pos(),old(bits).end as int,cb as int).unwrap(),
{
    reveal(group);let n=size(cb);
    let g=hd::read_table(s,bits,280+n)?;let r=hd::read_table(s,bits,256)?;let b=hd::read_table(s,bits,256)?;
    let a=hd::read_table(s,bits,256)?;let d=hd::read_table(s,bits,40)?;Ok(Group {g,r,b,a,d})
}
}
