// CCITT codes use MSB first logical bits; TIFF FillOrder may reverse each byte.
use super::*;
use crate::deflate_bits as db;
verus! {
#[derive(Clone,Copy)]
pub(super) struct Bits {pub pos:usize,pub end:usize,pub lsb:usize}
impl Bits {
    pub(super) open spec fn valid(self,s:Seq<u8>) -> bool {self.pos<=self.end && self.end<=8*s.len()}
}
#[verifier::opaque]
pub(super) open spec fn one(s:Seq<u8>,b:Bits) -> Option<(usize,Bits)> {
    if b.pos>=b.end {None} else {
        let n=if b.lsb!=0 {b.pos as int%8} else {7-b.pos as int%8};
        Some(((s[b.pos as int/8] as int/db::pow(n)%2) as usize,Bits {pos:(b.pos+1) as usize,..b}))
    }
}
#[inline(always)]
pub(super) fn bit(s:&[u8],b:Bits) -> (r:Result<(usize,Bits),Error>)
    requires b.valid(s@),
    ensures r.is_ok()==one(s@,b).is_some(),r.is_ok() ==> r.unwrap()==one(s@,b).unwrap(),
        r.is_ok() ==> r.unwrap().0<=1 && r.unwrap().1.valid(s@) && r.unwrap().1.end==b.end && r.unwrap().1.lsb==b.lsb && r.unwrap().1.pos==b.pos+1,
{
    reveal(one);
    if b.pos==b.end {return Err(Error::Truncated);}
    assert(b.pos/8<s.len()) by(nonlinear_arith) requires b.pos<b.end,b.end<=8*s.len();
    let n=if b.lsb!=0 {b.pos%8} else {7-b.pos%8};let d=crate::jpeg_bits::power(n);
    Ok((((s[b.pos/8] as usize)/d)%2,Bits {pos:b.pos+1,..b}))
}
#[verifier::opaque]
pub(super) open spec fn symbol(s:Seq<u8>,b:Bits,t:Seq<u16>,node:usize,fuel:nat) -> Option<(usize,Bits)> decreases fuel {
    if fuel==0 || 2*node+1>=t.len() {None} else {
        let a=one(s,b);if a.is_none() {None} else {
            let (v,q)=a.unwrap();let next=t[2*node+v] as usize;
            if next>=512 {Some(((next-512) as usize,q))}
            else if next==0 {None} else {symbol(s,q,t,next,(fuel-1) as nat)}
        }
    }
}
#[inline(always)]
pub(super) fn code(s:&[u8],b:Bits,black:bool) -> (r:Result<(usize,Bits),Error>)
    requires b.valid(s@),
    ensures r.is_ok()==symbol(s@,b,if black {crate::fax_tables::black()} else {crate::fax_tables::white()},0,13).is_some(),
        r.is_ok() ==> r.unwrap()==symbol(s@,b,if black {crate::fax_tables::black()} else {crate::fax_tables::white()},0,13).unwrap(),
        r.is_ok() ==> r.unwrap().1.valid(s@) && r.unwrap().1.end==b.end && r.unwrap().1.lsb==b.lsb && b.pos<r.unwrap().1.pos<=b.pos+13 && r.unwrap().0<=65023,
{
    let t=crate::fax_tables::table(black);let mut q=b;let mut node=0usize;let mut left=13usize;
    let ghost result=symbol(s@,b,t@,0,13);
    while left>0
        invariant q.valid(s@),q.end==b.end,q.lsb==b.lsb,b.pos<=q.pos<=b.pos+13-left,left<=13,node<=511,
            result==symbol(s@,q,t@,node,left as nat),result==symbol(s@,b,t@,0,13),
            t@==if black {crate::fax_tables::black()} else {crate::fax_tables::white()},
        decreases left,
    {
        reveal_with_fuel(symbol,1);
        if 2*node+1>=t.len() {return Err(Error::Invalid);}
        let (v,p)=bit(s,q)?;let next=t[2*node+v] as usize;
        if next>=512 {return Ok((next-512,p));}
        if next==0 {return Err(Error::Invalid);}
        q=p;node=next;left=left-1;
    }
    reveal_with_fuel(symbol,1);Err(Error::Invalid)
}
pub(super) open spec fn code_model(s:Seq<u8>,b:Bits,black:bool) -> Option<(usize,Bits)> {
    symbol(s,b,if black {crate::fax_tables::black()} else {crate::fax_tables::white()},0,13)
}
#[verifier::opaque]
pub(super) open spec fn run(s:Seq<u8>,b:Bits,black:bool,limit:usize,total:usize,fuel:nat) -> Option<(usize,Bits)> decreases fuel {
    if fuel==0 {None} else {
        let a=code_model(s,b,black);if a.is_none() {None} else {let (n,q)=a.unwrap();
            if total+n>limit {None} else if n<64 {Some(((total+n) as usize,q))}
            else {run(s,q,black,limit,(total+n) as usize,(fuel-1) as nat)}
        }
    }
}
pub(super) fn length(s:&[u8],b:Bits,black:bool,limit:usize) -> (r:Result<(usize,Bits),Error>)
    requires b.valid(s@),limit<=65536,
    ensures r.is_ok()==run(s@,b,black,limit,0,1026).is_some(),
        r.is_ok() ==> r.unwrap()==run(s@,b,black,limit,0,1026).unwrap(),
        r.is_ok() ==> r.unwrap().0<=limit && r.unwrap().1.valid(s@) && r.unwrap().1.end==b.end && r.unwrap().1.lsb==b.lsb && b.pos<r.unwrap().1.pos,
{
    let mut q=b;let mut total=0usize;let mut left=1026usize;
    let ghost result=run(s@,b,black,limit,0,1026);
    while left>0
        invariant q.valid(s@),q.end==b.end,q.lsb==b.lsb,b.pos<=q.pos,total<=limit,limit<=65536,left<=1026,
            result==run(s@,q,black,limit,total,left as nat),result==run(s@,b,black,limit,0,1026),
        decreases left,
    {
        reveal_with_fuel(run,1);
        let (n,p)=code(s,q,black)?;
        if n>limit-total {return Err(Error::Invalid);}
        if n<64 {return Ok((total+n,p));}
        total=total+n;q=p;left=left-1;
    }
    reveal_with_fuel(run,1);Err(Error::Invalid)
}
}
