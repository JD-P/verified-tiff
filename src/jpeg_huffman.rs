// JPEG canonical Huffman tables, independently defined from their raw counts.
use super::*;
use crate::jpeg_bits::{Bits,bit};
use crate::deflate_bits as db;
verus! {
pub(super) open spec fn count(s:Seq<u8>,p:int,n:nat) -> int decreases n {
    if n==0 {0} else {count(s,p,(n-1) as nat)+s[p+n-1] as int}
}
pub(super) open spec fn first(s:Seq<u8>,p:int,n:nat) -> int decreases n {
    if n<=1 {0} else {2*(first(s,p,(n-1) as nat)+s[p+n-2] as int)}
}
pub(super) open spec fn table_ok(s:Seq<u8>,p:int,end:int) -> bool {
    &&& 0<=p && p+16<=end<=s.len()
    &&& 1<=count(s,p,16)<=256
    &&& p+16+count(s,p,16)<=end
    &&& forall|n:nat| 1<=n<=16 ==> 0<=#[trigger] first(s,p,n) && first(s,p,n)+(s[p+n-1] as int) < db::pow(n as int)
}
#[derive(Clone,Copy)]
pub(super) struct Huff {pub base:usize}
impl Huff {
    pub(super) open spec fn valid(self,s:Seq<u8>) -> bool {table_ok(s,self.base as int,s.len() as int)}
}
pub(super) fn table(s:&[u8],p:usize,end:usize) -> (r:Result<(Huff,usize),Error>)
    requires end<=s.len(),
    ensures r.is_ok()==table_ok(s@,p as int,end as int),
        r.is_ok() ==> r.unwrap().0.base==p && r.unwrap().0.valid(s@),
        r.is_ok() ==> r.unwrap().1 as int==p+16+count(s@,p as int,16),
        r.is_ok() ==> r.unwrap().1<=end,
{
    if !checked_range(end,p,16) {return Err(Error::Truncated);}
    let mut n=1usize;let mut f=0usize;let mut total=0usize;
    while n<=16
        invariant 1<=n<=17,p as int+16<=end<=s.len(),
            total as int==count(s@,p as int,(n-1) as nat),total<=256,
            f as int==first(s@,p as int,n as nat),f<=131072,n<=16 ==> f<db::pow(n as int),
            forall|j:nat| 1<=j<n ==> 0<=#[trigger] first(s@,p as int,j) && first(s@,p as int,j)+(s@[p+j-1] as int) < db::pow(j as int),
        decreases 17-n,
    {
        let c=s[p+n-1] as usize;
        let cap=crate::jpeg_bits::power(n);
        if f+c>=cap {return Err(Error::Invalid);}
        if total+c>256 {
            proof {counts_monotone(s@,p as int,n as nat,16);}
            return Err(Error::Invalid);
        }
        total=total+c;
        assert(count(s@,p as int,n as nat)==total);
        if n<16 {proof {db::pow_bound(n as int);} f=(f+c)*2;}
        else {f=(f+c)*2;}
        n=n+1;
        // At the final iteration first(17) can exceed 65536; no further lookup.
    }
    if total==0 || total>end-(p+16) {return Err(Error::Invalid);}
    Ok((Huff {base:p},p+16+total))
}
proof fn counts_monotone(s:Seq<u8>,p:int,a:nat,b:nat)
    requires a<=b,p>=0,p+b<=s.len(),
    ensures 0<=count(s,p,a)<=count(s,p,b),decreases b,
{
    if b>0 {counts_monotone(s,p,if a<b {a} else {(a-1) as nat},(b-1) as nat);}
}
#[verifier::opaque]
pub(super) open spec fn symbol(s:Seq<u8>,b:Bits,p:int,n:nat,code:int,f:int,index:int) -> Option<(u8,Bits)> decreases 17-n {
    if n>16 {None} else {
        let a=crate::jpeg_bits::one(s,b);
        if a.is_none() {None} else {
            let (v,q)=a.unwrap();let c=2*code+v;let len=s[p+n-1] as int;
            if c<f {None} else if c<f+len {Some((s[p+16+index+c-f],q))}
            else {symbol(s,q,p,n+1,c,2*(f+len),index+len)}
        }
    }
}
#[inline(always)]
pub(super) fn decode(s:&[u8],b:Bits,h:Huff) -> (r:Result<(u8,Bits),Error>)
    requires b.valid(s@),h.valid(s@),
    ensures r.is_ok()==symbol(s@,b,h.base as int,1,0,0,0).is_some(),
        r.is_ok() ==> r.unwrap()==symbol(s@,b,h.base as int,1,0,0,0).unwrap() && r.unwrap().1.valid(s@),
{
    let slen=s.len();
    let mut q=b;let mut n=1usize;let mut code=0usize;let mut f=0usize;let mut index=0usize;
    let ghost expected=symbol(s@,b,h.base as int,1,0,0,0);
    while n<=16
        invariant 1<=n<=17,h.valid(s@),q.valid(s@),s@.len()<=usize::MAX,
            expected==symbol(s@,b,h.base as int,1,0,0,0),
            code<db::pow(n as int-1),f<=131072,index<=256,
            f as int==first(s@,h.base as int,n as nat),index as int==count(s@,h.base as int,(n-1) as nat),
            expected==symbol(s@,q,h.base as int,n as nat,code as int,f as int,index as int),
        decreases 17-n,
    {
        reveal_with_fuel(symbol,1);
        let (v,next)=bit(s,q)?;
        proof {db::pow_bound(n as int-1);counts_monotone(s@,h.base as int,n as nat,16);}
        let c=code*2+v;let len=s[h.base+(n-1)] as usize;
        assert(f+len<db::pow(n as int));
        if c<f {return Err(Error::Invalid);}
        if c<f+len {
            return Ok((s[h.base+16+index+(c-f)],next));
        }
        q=next;code=c;f=(f+len)*2;index=index+len;n=n+1;
    }
    reveal_with_fuel(symbol,1);Err(Error::Invalid)
}
}
