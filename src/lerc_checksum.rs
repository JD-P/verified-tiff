//! Lerc2 Fletcher32, including the big-endian word order and odd last byte.
use super::*;
verus! {
pub(super) open spec fn fold(v:int) -> int {v%65536+v/65536}
pub(super) open spec fn word(s:Seq<u8>,p:int) -> int {256*s[p] as int+if p+1<s.len(){s[p+1] as int}else{0int}}
#[verifier::opaque]
pub(super) open spec fn state(s:Seq<u8>,start:int,n:nat) -> (int,int) decreases n {
    if n==0{(65535,65535)}else{
        let old=state(s,start,(n-1) as nat);let a=fold(old.0+word(s,start+2*(n-1)));
        (a,fold(old.1+a))
    }
}
pub(super) closed spec fn checksum(s:Seq<u8>,start:usize) -> u32 {
    let r=state(s,start as int,((s.len()-start+1)/2) as nat);(r.1*65536+r.0) as u32
}
pub(super) fn compute(s:&[u8],start:usize) -> (r:u32)
    requires start<=s.len(),s.len()<usize::MAX,
    ensures r==checksum(s@,start),
{
    let count=(s.len()-start+1)/2;let mut i=0usize;let mut a=65535u32;let mut b=65535u32;
    reveal_with_fuel(state,1);
    while i<count
        invariant start<=s.len(),s.len()<usize::MAX,count==(s.len()-start+1)/2,i<=count,
            a<=65535,b<=65535,state(s@,start as int,i as nat)==(a as int,b as int),
        decreases count-i,
    {
        let p=start+2*i;let w=256*(s[p] as u32)+if p+1<s.len(){s[p+1] as u32}else{0u32};
        let sum=a+w;a=sum%65536+sum/65536;
        let sum=b+a;b=sum%65536+sum/65536;
        i=i+1;reveal_with_fuel(state,1);
    }
    reveal(checksum);b*65536+a
}
}
