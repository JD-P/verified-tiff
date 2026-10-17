// JPEG canonical Huffman tables, independently defined from their raw counts.
use super::*;
use crate::jpeg_bits::*;
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
/// The canonical walk over the high bits of an eight-bit window.
pub(super) open spec fn walk(s:Seq<u8>,p:int,w:int,n:nat,f:int,index:int) -> Option<(u8,nat)> decreases 9-n {
    if n<1 || n>8 {None} else {
        let c=w/db::pow(8-n as int);let len=s[p+n-1] as int;
        if c<f {None} else if c<f+len {Some((s[p+16+index+c-f],n))}
        else {walk(s,p,w,n+1,2*(f+len),index+len)}
    }
}
/// Nonzero entries are `length*256+symbol` for codes of at most eight bits.
pub(super) open spec fn fast_ok(s:Seq<u8>,h:Huff,t:Seq<u16>) -> bool {
    t.len()==256 && forall|w:int| 0<=w<256 && #[trigger] t[w]!=0 ==> walk(s,h.base as int,w,1,0,0)==Some(((t[w]%256) as u8,(t[w]/256) as nat))
}
proof fn walk_symbol(s:Seq<u8>,b:Bits,p:int,n:nat,f:int,index:int)
    requires b.valid(s),1<=n<=8,field(s,b,8).is_some(),walk(s,p,field(s,b,8).unwrap().0 as int,n,f,index).is_some(),
    ensures ({
        let (sym,len)=walk(s,p,field(s,b,8).unwrap().0 as int,n,f,index).unwrap();
        n<=len<=8 && field(s,b,(n-1) as nat).is_some() && field(s,b,len).is_some() &&
        symbol(s,field(s,b,(n-1) as nat).unwrap().1,p,n,field(s,b,(n-1) as nat).unwrap().0 as int,f,index)==Some((sym,field(s,b,len).unwrap().1))
    }),
    decreases 9-n,
{
    let w=field(s,b,8).unwrap().0 as int;
    crate::jpeg_bits::high(s,b,(n-1) as nat);crate::jpeg_bits::high(s,b,n);
    crate::jpeg_bits::properties(s,b,(n-1) as nat);crate::jpeg_bits::properties(s,b,n);db::pow_bound(n as int-1);
    reveal_with_fuel(field,1);reveal_with_fuel(symbol,1);
    let (v,q)=field(s,b,(n-1) as nat).unwrap();
    let a=crate::jpeg_bits::one(s,q);
    crate::jpeg_bits::one_bit(s,q);
    assert(a.is_some());
    let (bit,r)=a.unwrap();
    assert(field(s,b,n)==Some(((2*v+bit) as usize,r)));
    let c=w/db::pow(8-n as int);let len=s[p+n-1] as int;
    assert(c==2*v+bit);
    if c>=f+len {
        walk_symbol(s,b,p,n+1,2*(f+len),index+len);
    }
}
proof fn pow_add(a:int,b:int)
    requires 0<=a,0<=b,a+b<=16,
    ensures db::pow(a+b)==db::pow(a)*db::pow(b),
    decreases b,
{
    db::pow_bound(a+b);db::pow_bound(b);
    if b>0 {pow_add(a,b-1);db::pow_bound(a+b-1);db::pow_bound(b-1);
        assert(db::pow(a)*db::pow(b)==2*(db::pow(a)*db::pow(b-1))) by(nonlinear_arith) requires db::pow(b)==2*db::pow(b-1);}
}
proof fn first_nonneg(s:Seq<u8>,p:int,n:nat) ensures 0<=first(s,p,n), decreases n, {if n>1 {first_nonneg(s,p,(n-1) as nat);}}
/// A code value at length `n` has a prefix at every shorter length `m` that
/// lies past all codes of length `m`.
proof fn past(s:Seq<u8>,p:int,c:int,m:nat,n:nat)
    requires 1<=m<n<=16,0<=first(s,p,n)<=c,0<=p,p+16<=s.len(),
    ensures c/db::pow(n-m as int)>=first(s,p,m)+s[p+m-1] as int,
    decreases n-m,
{
    db::pow_bound(n-m as int);db::pow_bound(n-m-1 as int);
    assert(first(s,p,n)==2*(first(s,p,(n-1) as nat)+s[p+n-2] as int));
    vstd::arithmetic::div_mod::lemma_div_is_ordered(first(s,p,n),c,2);
    if m+1<n {
        first_nonneg(s,p,(n-1) as nat);
        past(s,p,c/2,m,(n-1) as nat);
        vstd::arithmetic::div_mod::lemma_div_denominator(c,2,db::pow(n-1-m as int));
    }
}
proof fn walk_fill(s:Seq<u8>,p:int,w:int,c:int,m:nat,n:nat)
    requires table_ok(s,p,s.len() as int),1<=m<=n<=8,0<=w<256,w/db::pow(8-n as int)==c,
        first(s,p,n)<=c<first(s,p,n)+s[p+n-1] as int,
    ensures walk(s,p,w,m,first(s,p,m),count(s,p,(m-1) as nat))==Some((s[p+16+count(s,p,(n-1) as nat)+c-first(s,p,n)],n)),
    decreases n-m,
{
    if m<n {
        db::pow_bound(8-n as int);db::pow_bound(n-m as int);pow_add(8-n as int,n-m as int);
        vstd::arithmetic::div_mod::lemma_div_denominator(w,db::pow(8-n as int),db::pow(n-m as int));
        past(s,p,c,m,n);
        assert(first(s,p,m+1)==2*(first(s,p,m)+s[p+m-1] as int));
        walk_fill(s,p,w,c,m+1,n);
    }
}
/// Lookahead entries for each eight-bit window; zero falls back to `decode`.
/// Codes of at most eight bits fill their window ranges in canonical order.
pub(super) fn fast(s:&[u8],h:Huff) -> (t:[u16;256])
    requires h.valid(s@),
    ensures fast_ok(s@,h,t@),
{
    let slen=s.len();let p=h.base;
    let mut t=[0u16;256];let mut n=1usize;let mut f=0usize;let mut index=0usize;
    proof {counts_monotone(s@,p as int,0,16);}
    while n<=8
        invariant 1<=n<=9,h.valid(s@),p==h.base,t@.len()==256,s@.len()<=usize::MAX,
            p as int+16+count(s@,p as int,16)<=s@.len(),0<=count(s@,p as int,16),
            f as int==first(s@,p as int,n as nat),f<db::pow(n as int),index as int==count(s@,p as int,(n-1) as nat),
            forall|j:int| 0<=j<256 && #[trigger] t@[j]!=0 ==> walk(s@,p as int,j,1,0,0)==Some(((t@[j]%256) as u8,(t@[j]/256) as nat)),
        decreases 9-n,
    {
        proof {db::pow_bound(n as int);db::pow_bound(8-n as int);pow_add(8-n as int,n as int);counts_monotone(s@,p as int,n as nat,16);}
        let len=s[p+n-1] as usize;let shift=8-n;let unit=crate::jpeg_bits::power(shift);
        assert(f+len<db::pow(n as int));
        let mut i=0usize;
        while i<len
            invariant 1<=n<=8,i<=len,len==s@[p+n-1],h.valid(s@),p==h.base,t@.len()==256,s@.len()<=usize::MAX,
                p as int+16+count(s@,p as int,16)<=s@.len(),count(s@,p as int,n as nat)<=count(s@,p as int,16),
                f as int==first(s@,p as int,n as nat),f+len<db::pow(n as int),index as int==count(s@,p as int,(n-1) as nat),
                shift==8-n,unit as int==db::pow(shift as int),db::pow(n as int)*unit==256,
                forall|j:int| 0<=j<256 && #[trigger] t@[j]!=0 ==> walk(s@,p as int,j,1,0,0)==Some(((t@[j]%256) as u8,(t@[j]/256) as nat)),
            decreases len-i,
        {
            let c=f+i;let sym=s[p+16+index+i];let entry=(n*256+sym as usize) as u16;
            assert(entry%256==sym as int && entry/256==n as int);
            assert((c as int+1)*unit<=256) by(nonlinear_arith) requires c+1<=db::pow(n as int),db::pow(n as int)*unit==256,unit>=1;
            let lo=c*unit;let hi=lo+unit;let mut w=lo;
            while w<hi
                invariant lo<=w<=hi,hi==lo+unit,lo as int==c as int*unit,hi<=256,unit as int==db::pow(shift as int),shift==8-n,1<=n<=8,
                    h.valid(s@),p==h.base,t@.len()==256,c==f+i,i<len,len==s@[p+n-1],f as int==first(s@,p as int,n as nat),
                    index as int==count(s@,p as int,(n-1) as nat),sym==s@[p+16+index+c-f],entry%256==sym as int,entry/256==n as int,
                    forall|j:int| 0<=j<256 && #[trigger] t@[j]!=0 ==> walk(s@,p as int,j,1,0,0)==Some(((t@[j]%256) as u8,(t@[j]/256) as nat)),
                decreases hi-w,
            {
                proof {
                    db::pow_bound(shift as int);
                    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(w as int,unit as int,c as int,w as int-lo as int);
                    walk_fill(s@,p as int,w as int,c as int,1,n as nat);
                }
                t[w]=entry;w=w+1;
            }
            i=i+1;
        }
        f=(f+len)*2;index=index+len;n=n+1;
    }
    t
}
/// `decode` through an eight-bit lookahead table when the next entropy byte
/// is data; a missing entry or a marker uses the bit-serial decoder.
#[inline(always)]
pub(super) fn decode_fast(s:&[u8],b:Bits,h:Huff,t:&[u16;256]) -> (r:Result<(u8,Bits),Error>)
    requires b.valid(s@),h.valid(s@),fast_ok(s@,h,t@),
    ensures r.is_ok()==symbol(s@,b,h.base as int,1,0,0,0).is_some(),
        r.is_ok() ==> r.unwrap()==symbol(s@,b,h.base as int,1,0,0,0).unwrap() && r.unwrap().1.valid(s@),
{
    if b.left<8 && b.at<b.end {
        let x=s[b.at];
        let (nb,adv)=if x!=255 {(x,1usize)} else if b.end-b.at>=2 && s[b.at+1]==0 {(255u8,2usize)} else {(0u8,0usize)};
        if adv!=0 {
            let left=b.left;
            proof {
                crate::jpeg_bits::window(s@,b,nb,adv,8);crate::jpeg_bits::shift_power(nb as usize,left);
                crate::bit_window::powers(left as u32);crate::bit_window::powers((8-left) as u32);
                let y=b.byte as usize;let l=left;
                assert(y&(((1usize<<l) as usize-1) as usize)==y%(1usize<<l)) by(bit_vector) requires l<8;
                assert((1usize<<left)==(1u32<<(left as u32)) as usize) by(bit_vector) requires left<8;
                crate::jpeg_bits::properties(s@,b,8);
            }
            let hi=(b.byte as usize)&(crate::jpeg_bits::power(left)-1);
            proof {crate::jpeg_bits::pow_split(left as int);}
            assert(hi as int*db::pow(8-left as int)<=32768) by(nonlinear_arith)
                requires 0<=hi<db::pow(left as int),db::pow(left as int)*db::pow(8-left as int)==256,db::pow(8-left as int)<=256;
            let w=hi*crate::jpeg_bits::power(8-left)+((nb as usize)>>left);
            let e=t[w];
            if e!=0 {
                let len=(e>>8) as usize;let sym=(e&255) as u8;
                assert(e>>8==e/256 && e&255==e%256) by(bit_vector);
                proof {reveal_with_fuel(field,1);walk_symbol(s@,b,h.base as int,1,0,0);crate::jpeg_bits::window(s@,b,nb,adv,len as nat);crate::jpeg_bits::properties(s@,b,len as nat);}
                let q=if len<=left {Bits {left:left-len,..b}} else {Bits {at:b.at+adv,byte:nb,left:8-(len-left),end:b.end}};
                return Ok((sym,q));
            }
        }
    }
    decode(s,b,h)
}
}
