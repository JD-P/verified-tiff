//! Zstandard bit fields. Forward probability descriptions and reverse entropy
//! streams share one little-endian mathematical field relation.
use super::*;
verus! {
pub(super) open spec fn pow(n:int) -> int {
    if n<=0 {1}
    else if n==1 {2}
    else if n==2 {4}
    else if n==3 {8}
    else if n==4 {16}
    else if n==5 {32}
    else if n==6 {64}
    else if n==7 {128}
    else if n==8 {256}
    else if n==9 {512}
    else if n==10 {1024}
    else if n==11 {2048}
    else if n==12 {4096}
    else if n==13 {8192}
    else if n==14 {16384}
    else if n==15 {32768}
    else if n==16 {65536}
    else if n==17 {131072}
    else if n==18 {262144}
    else if n==19 {524288}
    else if n==20 {1048576}
    else if n==21 {2097152}
    else if n==22 {4194304}
    else if n==23 {8388608}
    else if n==24 {16777216}
    else if n==25 {33554432}
    else if n==26 {67108864}
    else if n==27 {134217728}
    else if n==28 {268435456}
    else if n==29 {536870912}
    else if n==30 {1073741824}
    else if n==31 {2147483648}
    else if n==32 {4294967296}
    else {8589934592}
}
pub(super) fn power(n:usize) -> (r:u64)
    requires n<=32,
    ensures r==pow(n as int),1<=r<=4294967296,
{
    match n {
        0=>1u64,
        1=>2u64,
        2=>4u64,
        3=>8u64,
        4=>16u64,
        5=>32u64,
        6=>64u64,
        7=>128u64,
        8=>256u64,
        9=>512u64,
        10=>1024u64,
        11=>2048u64,
        12=>4096u64,
        13=>8192u64,
        14=>16384u64,
        15=>32768u64,
        16=>65536u64,
        17=>131072u64,
        18=>262144u64,
        19=>524288u64,
        20=>1048576u64,
        21=>2097152u64,
        22=>4194304u64,
        23=>8388608u64,
        24=>16777216u64,
        25=>33554432u64,
        26=>67108864u64,
        27=>134217728u64,
        28=>268435456u64,
        29=>536870912u64,
        30=>1073741824u64,
        31=>2147483648u64,
        32=>4294967296u64,
        _=>1,
    }
}
pub(super) proof fn power_step(n:int)
    requires 0<=n<32,
    ensures pow(n+1)==2*pow(n),1<=pow(n)<=2147483648,
{}
#[verifier::opaque]
pub(super) open spec fn value(s:Seq<u8>,p:int,n:nat) -> int decreases n {
    if n==0 {0} else {value(s,p,(n-1) as nat)+pow(n as int-1)*((s[(p+n-1)/8] as int/power2((p+n-1)%8))%2)}
}
pub(super) fn read(s:&[u8],p:usize,end:usize,n:usize) -> (r:Result<u32,Error>)
    requires end<=s.len(),end<=usize::MAX/8,p<=end*8,n<=32,
    ensures r.is_ok()==(p as int+n<=end as int*8),
        r.is_ok() ==> r.unwrap() as int==value(s@,p as int,n as nat),
        r.is_ok() ==> r.unwrap()<pow(n as int),
{
    if n>end*8-p {return Err(Error::Truncated);}
    let mut v=0u64;let mut i=0usize;let mut weight=1u64;
    reveal_with_fuel(value,1);
    while i<n
        invariant n<=32,p as int+n<=end as int*8,end<=s.len(),end<=usize::MAX/8,
            i<=n,v as int==value(s@,p as int,i as nat),v<weight,weight==pow(i as int),weight<=4294967296,
        decreases n-i,
    {
        let at=p+i;
        let b=(s[at/8]/bit_divisor(at%8))%2;
        proof {power_step(i as int);}
        assert(v as int+weight*b<=4294967295) by(nonlinear_arith) requires v<weight,weight<=2147483648,b<=1;
        v=v+weight*(b as u64);weight=weight*2;i=i+1;
        reveal_with_fuel(value,1);
    }
    Ok(v as u32)
}
#[derive(Clone,Copy)]
pub(super) struct Forward {pub p:usize,pub end:usize}
impl Forward {
    pub(super) open spec fn valid(self,s:Seq<u8>) -> bool {self.end<=s.len() && self.end<=usize::MAX/8 && self.p<=self.end as int*8}
    pub(super) fn read(&mut self,s:&[u8],n:usize) -> (r:Result<u32,Error>)
        requires old(self).valid(s@),n<=32,
        ensures final(self).valid(s@),final(self).end==old(self).end,
            r.is_ok()==(old(self).p as int+n<=old(self).end as int*8),
            r.is_ok() ==> r.unwrap()==value(s@,old(self).p as int,n as nat) && r.unwrap()<pow(n as int),
            r.is_ok() ==> final(self).p==old(self).p+n,
            r.is_err() ==> *final(self)==*old(self),
    {
        let v=read(s,self.p,self.end,n)?;self.p=self.p+n;Ok(v)
    }
}
pub(super) open spec fn top(v:u8) -> int {
    if v>=128 {7} else if v>=64 {6} else if v>=32 {5} else if v>=16 {4}
    else if v>=8 {3} else if v>=4 {2} else if v>=2 {1} else {0}
}
#[derive(Clone,Copy)]
pub(super) struct Reverse {pub low:usize,pub p:usize,pub end:usize}
impl Reverse {
    pub(super) open spec fn valid(self,s:Seq<u8>) -> bool {self.low<=self.p<=self.end as int*8 && self.end<=s.len() && self.end<=usize::MAX/8}
    pub(super) fn new(s:&[u8],start:usize,end:usize) -> (r:Result<Self,Error>)
        requires end<=s.len(),end<=usize::MAX/8,start<=end,
        ensures r.is_ok()==(start<end && s@[end as int-1]!=0),
            r.is_ok() ==> r.unwrap().valid(s@) && r.unwrap().low==start*8 && r.unwrap().end==end,
            r.is_ok() ==> r.unwrap().p as int==(end as int-1)*8+top(s@[end as int-1]),
    {
        if start==end {return Err(Error::Truncated);}
        let v=s[end-1];if v==0{return Err(Error::Invalid);}
        let n=if v>=128 {7usize}else if v>=64{6}else if v>=32{5}else if v>=16{4}
            else if v>=8{3}else if v>=4{2}else if v>=2{1}else{0};
        Ok(Self {low:start*8,p:(end-1)*8+n,end})
    }
    pub(super) fn read(&mut self,s:&[u8],n:usize) -> (r:Result<u32,Error>)
        requires old(self).valid(s@),n<=32,
        ensures final(self).valid(s@),final(self).end==old(self).end,final(self).low==old(self).low,
            r.is_ok()==(n<=old(self).p-old(self).low),
            r.is_ok() ==> r.unwrap()==value(s@,old(self).p as int-n,n as nat) && r.unwrap()<pow(n as int),
            r.is_ok() ==> final(self).p+n==old(self).p,
            r.is_err() ==> *final(self)==*old(self),
    {
        if n>self.p-self.low {return Err(Error::Truncated);}
        let v=read(s,self.p-n,self.end,n)?;self.p=self.p-n;Ok(v)
    }
}
}
