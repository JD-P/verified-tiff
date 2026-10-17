// JPEG Annex F entropy bits: MSB first, FF/00 byte stuffing, bounded input.
use super::*;
use crate::deflate_bits as db;
verus! {
#[inline(always)]
pub(super) fn power(n:usize) -> (r:usize) requires n<=16, ensures r==db::pow(n as int),1<=r<=65536, {
    proof {crate::bit_window::powers(n as u32);}
    assert((1usize<<n)==(1u32<<(n as u32)) as usize) by(bit_vector) requires n<=16;
    1usize<<n
}

#[derive(Clone,Copy)]
pub(super) struct Bits {pub at:usize,pub end:usize,pub byte:u8,pub left:usize}
impl Bits {
    pub(super) open spec fn valid(self,s:Seq<u8>) -> bool {self.at<=self.end<=s.len() && self.left<=8}
}
#[verifier::opaque]
pub(super) open spec fn one(s:Seq<u8>,b:Bits) -> Option<(usize,Bits)> {
    if b.left>0 {Some(((b.byte as int/db::pow(b.left as int-1)%2) as usize,Bits {left:(b.left-1) as usize,..b}))}
    else if b.at>=b.end {None}
    else if s[b.at as int]==255 {
        if b.at as int+1>=b.end || s[b.at as int+1]!=0 {None}
        else {Some((1usize,Bits {at:(b.at+2) as usize,byte:255,left:7,..b}))}
    } else {Some(((s[b.at as int]/128) as usize,Bits {at:(b.at+1) as usize,byte:s[b.at as int],left:7,..b}))}
}
#[inline(always)]
pub(super) fn bit(s:&[u8],b:Bits) -> (r:Result<(usize,Bits),Error>)
    requires b.valid(s@),
    ensures r.is_ok()==one(s@,b).is_some(),r.is_ok() ==> r.unwrap()==one(s@,b).unwrap(),
        r.is_ok() ==> r.unwrap().0<=1 && r.unwrap().1.valid(s@),
{
    reveal(one);
    if b.left>0 {
        let shift=b.left-1;let byte=b.byte as usize;
        proof {shift_power(byte,shift);}
        let v=(byte>>shift)&1;
        assert(v==(byte>>shift)%2) by(bit_vector) requires v==(byte>>shift)&1;
        Ok((v,Bits {left:b.left-1,..b}))
    } else {
        if b.at==b.end {return Err(Error::Truncated);}
        let v=s[b.at];
        if v==255 {
            if b.end-b.at<2 || s[b.at+1]!=0 {return Err(Error::Invalid);}
            Ok((1,Bits {at:b.at+2,byte:255,left:7,..b}))
        } else {Ok(((v/128) as usize,Bits {at:b.at+1,byte:v,left:7,..b}))}
    }
}
#[verifier::opaque]
pub(super) open spec fn field(s:Seq<u8>,b:Bits,n:nat) -> Option<(usize,Bits)> decreases n {
    if n==0 {Some((0usize,b))} else {
        let a=field(s,b,(n-1) as nat);
        if a.is_none() {None} else {let (v,q)=a.unwrap();let c=one(s,q);
            if c.is_none() {None} else {Some(((2*v+c.unwrap().0) as usize,c.unwrap().1))}}
    }
}
pub(super) proof fn properties(s:Seq<u8>,b:Bits,n:nat)
    requires b.valid(s),n<=16,
    ensures field(s,b,n).is_some() ==> field(s,b,n).unwrap().1.valid(s) && field(s,b,n).unwrap().0<db::pow(n as int),
    decreases n,
{
    reveal_with_fuel(field,1);reveal(one);
    if n>0 {properties(s,b,(n-1) as nat);db::pow_bound(n as int-1);}
}
pub(super) proof fn join(s:Seq<u8>,b:Bits,i:nat,n:nat)
    requires b.valid(s),i+n<=16,field(s,b,i).is_some(),
    ensures field(s,b,i+n)==match field(s,field(s,b,i).unwrap().1,n) {
        None=>None,
        Some((w,r))=>Some(((field(s,b,i).unwrap().0 as int*db::pow(n as int)+w) as usize,r)),
    },
    decreases n,
{
    reveal_with_fuel(field,1);
    properties(s,b,i);let q=field(s,b,i).unwrap().1;
    let a=field(s,b,i).unwrap().0 as int;
    if n>0 {
        join(s,b,i,(n-1) as nat);properties(s,q,(n-1) as nat);properties(s,b,(i+n-1) as nat);
        db::pow_bound(n as int-1);
        if field(s,q,(n-1) as nat).is_some() {
            let (w,t)=field(s,q,(n-1) as nat).unwrap();let p=db::pow(n as int-1);
            assert(0<=a*p+w<32768) by(nonlinear_arith)
                requires 0<=a<db::pow(i as int),0<=w<p,p==db::pow(n as int-1),i+n<=16,0<=i,0<n<=16;
            assert(field(s,b,(i+n-1) as nat)==Some(((a*p+w) as usize,t)));
            reveal(one);
            if one(s,t).is_some() {
                let (c,r)=one(s,t).unwrap();assert(c<=1);
                assert(field(s,b,i+n)==Some(((2*(a*p+w)+c) as usize,r)));
                assert(field(s,q,n)==Some(((2*w+c) as usize,r)));
                assert(2*(a*p+w)+c==a*(2*p)+(2*w+c)) by(nonlinear_arith);
            }
        }
    }
}
pub(super) proof fn shift_power(value:usize,shift:usize)
    requires shift<=16,
    ensures (value>>shift) as int==value as int/db::pow(shift as int),
{
    crate::bit_window::powers(shift as u32);
    assert((1usize<<shift)==(1u32<<(shift as u32)) as usize) by(bit_vector) requires shift<=16;
    assert((value>>shift)==value/(1usize<<shift)) by(bit_vector) requires shift<=16;
}
pub(super) proof fn buffered(s:Seq<u8>,b:Bits,n:u32)
    requires b.valid(s),n<=b.left,
    ensures field(s,b,n as nat)==Some((((b.byte as u32>>((b.left-n) as u32))&(((1u32<<n)-1) as u32)) as usize,
        Bits {left:(b.left-n) as usize,..b})),
    decreases n,
{
    reveal_with_fuel(field,1);reveal(one);
    crate::bit_window::powers(n);
    let byte=b.byte as u32;let left=b.left as u32;
    if n==0 {
        assert(((byte>>left)&(((1u32<<n)-1) as u32))==0) by(bit_vector) requires n==0,left<=8;
    } else {
        buffered(s,b,(n-1) as u32);crate::bit_window::powers((left-n) as u32);
        assert(((byte>>((left-n) as u32))&(((1u32<<n)-1) as u32)) as u32
            ==2*((byte>>((left-n+1) as u32))&(((1u32<<((n-1) as u32))-1) as u32))
                +(byte/(1u32<<((left-n) as u32)))%2) by(bit_vector)
            requires byte<=255,0<n<=left<=8;
        assert(((byte>>((left-n+1) as u32))&(((1u32<<((n-1) as u32))-1) as u32))<=255) by(bit_vector)
            requires byte<=255,0<n<=left<=8;
    }
}
#[inline(always)]
pub(super) fn read(s:&[u8],b:Bits,n:usize) -> (r:Result<(usize,Bits),Error>)
    requires b.valid(s@),n<=16,
    ensures r.is_ok()==field(s@,b,n as nat).is_some(),
        r.is_ok() ==> r.unwrap()==field(s@,b,n as nat).unwrap(),
        r.is_ok() ==> r.unwrap().1.valid(s@) && r.unwrap().0<db::pow(n as int),
{
    let mut q=b;let mut v=0usize;let mut i=0usize;
    reveal_with_fuel(field,1);
    while i<n
        invariant i<=n,n<=16,b.valid(s@),q.valid(s@),v<db::pow(i as int),
            field(s@,b,i as nat)==Some((v,q)),
        decreases n-i,
    {
        if q.left>0 {
            let take=if q.left<n-i {q.left} else {n-i};
            let shift=q.left-take;
            let factor=power(take);
            let c=((q.byte as usize)>>shift)&(factor-1);
            let qb=q.byte;let qleft=q.left;
            proof {
                buffered(s@,q,take as u32);properties(s@,q,take as nat);
                crate::bit_window::powers(take as u32);
                assert(c as u32==((qb as u32>>((qleft-take) as u32))&(((1u32<<(take as u32))-1) as u32))) by(bit_vector)
                    requires qleft<=8,0<take<=qleft,shift==qleft-take,factor==(1u32<<(take as u32)) as usize,
                        c==((qb as usize)>>shift)&((factor-1) as usize);
                assert(c<factor) by(bit_vector)
                    requires factor>=1,factor<=256,c==((qb as usize)>>shift)&((factor-1) as usize),
                        factor==(1u32<<(take as u32)) as usize,take<=8;
                join(s@,b,i as nat,take as nat);
                properties(s@,b,(i+take) as nat);
            }
            let next=Bits {left:q.left-take,..q};
            assert(v as int*factor+c<65536) by(nonlinear_arith)
                requires v<db::pow(i as int),c<factor,factor as int==db::pow(take as int),i+take<=16,0<=i,0<take<=8;
            v=v*factor+c;q=next;i=i+take;
            continue;
        }
        let a=bit(s,q);
        if a.is_err() {
            proof {field_failure(s@,b,i as nat,n as nat);}
            return Err(Error::Invalid);
        }
        let (c,p)=a.unwrap();
        proof {crate::deflate_bits::pow_bound(i as int);}
        v=v*2+c;q=p;i=i+1;
        reveal_with_fuel(field,1);
    }
    Ok((v,q))
}
pub(super) proof fn pow_split(l:int)
    requires 0<=l<=8,
    ensures db::pow(l)*db::pow(8-l)==256,
{
    if l==0 {} else if l==1 {} else if l==2 {} else if l==3 {} else if l==4 {}
    else if l==5 {} else if l==6 {} else if l==7 {} else {}
}
pub(super) proof fn one_bit(s:Seq<u8>,b:Bits)
    ensures one(s,b).is_some() ==> one(s,b).unwrap().0<=1,
{reveal(one);}
/// A shorter read is a prefix of a successful longer read.
pub(super) proof fn prefix(s:Seq<u8>,b:Bits,i:nat,n:nat)
    requires i<=n,field(s,b,n).is_some(),
    ensures field(s,b,i).is_some(),
    decreases n,
{
    reveal_with_fuel(field,1);
    if i<n {prefix(s,b,i,(n-1) as nat);}
}
/// The first `i` bits of an eight-bit read are its high bits.
pub(super) proof fn high(s:Seq<u8>,b:Bits,i:nat)
    requires b.valid(s),i<=8,field(s,b,8).is_some(),
    ensures field(s,b,i).is_some(),
        field(s,b,i).unwrap().0 as int==field(s,b,8).unwrap().0 as int/db::pow(8-i as int),
{
    prefix(s,b,i,8);properties(s,b,i);
    let (a,q)=field(s,b,i).unwrap();
    join(s,b,i,(8-i) as nat);properties(s,q,(8-i) as nat);db::pow_bound(8-i as int);
    let (w,_)=field(s,q,(8-i) as nat).unwrap();let p=db::pow(8-i as int);
    assert(field(s,b,8).unwrap().0 as int==a as int*p+w) by {
        assert(0<=a as int*p+w<65536) by(nonlinear_arith) requires 0<=a<db::pow(i as int),0<=w<p,p==db::pow(8-i as int),i<=8;
    }
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(a as int*p+w,p,a as int,w as int);
}
/// Reading at a byte boundary consumes the next entropy byte; an FF 00 pair
/// supplies FF.
pub(super) proof fn fresh(s:Seq<u8>,b:Bits,nb:u8,adv:usize,m:nat)
    requires b.valid(s),b.left==0,1<=m<=8,b.at<b.end,
        (adv==1 && s[b.at as int]!=255 && nb==s[b.at as int]) || (adv==2 && s[b.at as int]==255 && b.at+1<b.end && s[b.at as int+1]==0 && nb==255),
    ensures field(s,b,m)==Some(((nb as int/db::pow(8-m as int)) as usize,Bits {at:(b.at+adv) as usize,byte:nb,left:(8-m) as usize,..b})),
{
    reveal_with_fuel(field,2);reveal(one);
    let b1=Bits {at:(b.at+adv) as usize,byte:nb,left:7,..b};
    assert(one(s,b)==Some(((nb/128) as usize,b1)));
    assert(field(s,b,1)==Some(((nb/128) as usize,b1)));
    join(s,b,1,(m-1) as nat);buffered(s,b1,(m-1) as u32);
    let k=(m-1) as u32;let x=nb as u32;
    crate::bit_window::powers(k);crate::bit_window::powers((7-k) as u32);
    let y=x>>((7-k) as u32);let lo=y&(((1u32<<k)-1) as u32);let top=x/128;
    assert(top*(1u32<<k)+lo==y && y==x/(1u32<<((7-k) as u32))) by(bit_vector) requires x<=255,k<=7,y==x>>((7-k) as u32),lo==y&(((1u32<<k)-1) as u32),top==x/128;
    assert(field(s,b1,k as nat)==Some((lo as usize,Bits {left:(7-k) as usize,..b1})));
    assert(top as int*db::pow(k as int)+lo as int==nb as int/db::pow(8-m as int));
}
/// Eight bits from the buffered remainder and the next entropy byte, and the
/// reader state after any shorter prefix of them.
pub(super) proof fn window(s:Seq<u8>,b:Bits,nb:u8,adv:usize,n:nat)
    requires b.valid(s),b.left<8,1<=n<=8,b.at<b.end,
        (adv==1 && s[b.at as int]!=255 && nb==s[b.at as int]) || (adv==2 && s[b.at as int]==255 && b.at+1<b.end && s[b.at as int+1]==0 && nb==255),
    ensures field(s,b,8).is_some(),field(s,b,n).is_some(),
        field(s,b,8).unwrap().0 as int==(b.byte as int%db::pow(b.left as int))*db::pow(8-b.left as int)+nb as int/db::pow(b.left as int),
        field(s,b,n).unwrap().1==(if n<=b.left {Bits {left:(b.left-n) as usize,..b}}
            else {Bits {at:(b.at+adv) as usize,byte:nb,left:(8-(n-b.left)) as usize,..b}}),
{
    let l=b.left as u32;let x=b.byte as u32;
    buffered(s,b,l);crate::bit_window::powers(l);
    let q0=Bits {left:0,..b};
    assert(((x>>((l-l) as u32))&(((1u32<<l)-1) as u32))==x%(1u32<<l)) by(bit_vector) requires l<8;
    assert(field(s,b,b.left as nat)==Some(((b.byte as int%db::pow(b.left as int)) as usize,q0)));
    join(s,b,b.left as nat,(8-b.left) as nat);fresh(s,q0,nb,adv,(8-b.left) as nat);
    assert(0<=(b.byte as int%db::pow(b.left as int))*db::pow(8-b.left as int)+nb as int/db::pow(b.left as int)<65536) by(nonlinear_arith)
        requires 0<=b.byte as int%db::pow(b.left as int)<db::pow(b.left as int),0<=nb as int/db::pow(b.left as int)<=255,
            db::pow(b.left as int)*db::pow(8-b.left as int)==256,db::pow(b.left as int)>=1;
    if n<=b.left {buffered(s,b,n as u32);}
    else {join(s,b,b.left as nat,(n-b.left) as nat);fresh(s,q0,nb,adv,(n-b.left) as nat);}
    prefix(s,b,n,8);
}
/// Up to eight bits past the buffered remainder come from the next entropy byte.
pub(super) proof fn spill(s:Seq<u8>,b:Bits,nb:u8,adv:usize,n:nat)
    requires b.valid(s),b.left<8,b.left<n<=b.left+8,n<=16,b.at<b.end,
        (adv==1 && s[b.at as int]!=255 && nb==s[b.at as int]) || (adv==2 && s[b.at as int]==255 && b.at+1<b.end && s[b.at as int+1]==0 && nb==255),
    ensures field(s,b,n)==Some((((b.byte as int%db::pow(b.left as int))*db::pow(n-b.left as int)+nb as int/db::pow(8-(n-b.left) as int)) as usize,
        Bits {at:(b.at+adv) as usize,byte:nb,left:(8-(n-b.left)) as usize,..b})),
{
    let l=b.left as u32;let x=b.byte as u32;
    buffered(s,b,l);crate::bit_window::powers(l);
    let q0=Bits {left:0,..b};
    assert(((x>>((l-l) as u32))&(((1u32<<l)-1) as u32))==x%(1u32<<l)) by(bit_vector) requires l<8;
    assert(field(s,b,b.left as nat)==Some(((b.byte as int%db::pow(b.left as int)) as usize,q0)));
    join(s,b,b.left as nat,(n-b.left) as nat);fresh(s,q0,nb,adv,(n-b.left) as nat);
}
/// `read` from the buffered byte and, when it is data, the next entropy byte.
#[inline(always)]
pub(super) fn read_fast(s:&[u8],b:Bits,n:usize) -> (r:Result<(usize,Bits),Error>)
    requires b.valid(s@),n<=16,
    ensures r.is_ok()==field(s@,b,n as nat).is_some(),
        r.is_ok() ==> r.unwrap()==field(s@,b,n as nat).unwrap(),
        r.is_ok() ==> r.unwrap().1.valid(s@) && r.unwrap().0<db::pow(n as int),
{
    let left=b.left;
    if n<=left {
        let shift=left-n;let v=((b.byte as usize)>>shift)&(power(n)-1);
        proof {
            buffered(s@,b,n as u32);properties(s@,b,n as nat);crate::bit_window::powers(n as u32);
            let y=b.byte;
            assert(((y as usize)>>shift)&(((1usize<<n) as usize-1) as usize)==(((y as u32>>(shift as u32))&(((1u32<<(n as u32))-1) as u32)) as usize)) by(bit_vector) requires shift<=8,n<=8;
            assert((1usize<<n)==(1u32<<(n as u32)) as usize) by(bit_vector) requires n<=8;
        }
        return Ok((v,Bits {left:shift,..b}));
    }
    if left<8 && n<=left+8 && b.at<b.end {
        let x=s[b.at];
        let (nb,adv)=if x!=255 {(x,1usize)} else if b.end-b.at>=2 && s[b.at+1]==0 {(255u8,2usize)} else {(0u8,0usize)};
        if adv!=0 {
            let m=n-left;
            proof {
                spill(s@,b,nb,adv,n as nat);properties(s@,b,n as nat);shift_power(nb as usize,(8-m) as usize);
                crate::bit_window::powers(left as u32);db::pow_bound(m as int);
                let y=b.byte as usize;let l=left;
                assert(y&(((1usize<<l) as usize-1) as usize)==y%(1usize<<l)) by(bit_vector) requires l<8;
                assert((1usize<<left)==(1u32<<(left as u32)) as usize) by(bit_vector) requires left<8;
            }
            let hi=(b.byte as usize)&(power(left)-1);
            assert(hi as int*db::pow(m as int)<65536) by(nonlinear_arith) requires 0<=hi<db::pow(left as int),db::pow(left as int)<=128,db::pow(m as int)<=256;
            let v=hi*power(m)+((nb as usize)>>(8-m));
            return Ok((v,Bits {at:b.at+adv,byte:nb,left:8-m,end:b.end}));
        }
    }
    read(s,b,n)
}
proof fn field_failure(s:Seq<u8>,b:Bits,i:nat,n:nat)
    requires i<n,field(s,b,i).is_some(),one(s,field(s,b,i).unwrap().1).is_none(),
    ensures field(s,b,n).is_none(),decreases n-i,
{
    reveal_with_fuel(field,1);
    if n>i+1 {field_failure(s,b,i,(n-1) as nat);}
}
}
