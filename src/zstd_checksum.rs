//! XXH64(seed=0), the optional Zstandard content checksum. This is an exact
//! integer/bit-vector model, not a cryptographic-security claim.
use super::*;
use crate::native_samples as ns;
verus! {
const P1:u64=11400714785074694791;
const P2:u64=14029467366897019727;
const P3:u64=1609587929392839161;
const P4:u64=9650029242287828579;
const P5:u64=2870177450012600261;
spec fn addm(a:u64,b:u64) -> u64 {((a as int+b as int)%18446744073709551616) as u64}
spec fn mulm(a:u64,b:u64) -> u64 {((a as int*b as int)%18446744073709551616) as u64}
spec fn rotm(a:u64,n:u32) -> u64 {(a<<n)|(a>>(64-n))}
#[inline(always)]
fn addw(a:u64,b:u64) -> (r:u64) ensures r==addm(a,b),
{(((a as u128)+(b as u128))%18446744073709551616u128) as u64}
#[inline(always)]
fn mulw(a:u64,b:u64) -> (r:u64) ensures r==mulm(a,b),
{
    assert((a as int)*(b as int)<=u128::MAX) by(nonlinear_arith) requires a<=u64::MAX,b<=u64::MAX;
    (((a as u128)*(b as u128))%18446744073709551616u128) as u64
}
#[inline(always)]
fn rot(a:u64,n:u32) -> (r:u64) requires 0<n<64,ensures r==rotm(a,n),
{(a<<n)|(a>>(64-n))}
spec fn round_model(acc:u64,word:u64) -> u64 {mulm(rotm(addm(acc,mulm(word,P2)),31),P1)}
#[inline(always)]
fn round(acc:u64,word:u64) -> (r:u64) ensures r==round_model(acc,word),
{mulw(rot(addw(acc,mulw(word,P2)),31),P1)}
spec fn merge_model(acc:u64,word:u64) -> u64 {addm(mulm(acc^round_model(0,word),P1),P4)}
#[inline(always)]
fn merge(acc:u64,word:u64) -> (r:u64) ensures r==merge_model(acc,word),
{addw(mulw(acc^round(0,word),P1),P4)}
#[verifier::opaque]
spec fn lanes(s:Seq<u8>,blocks:nat) -> [u64;4] decreases blocks {
    if blocks==0{[addm(P1,P2),P2,0,7046029288634856825u64]}else{
        let q=lanes(s,(blocks-1) as nat);let at=(blocks-1)*32;
        [round_model(q[0],ns::word(s,at,8,true) as u64),round_model(q[1],ns::word(s,at+8,8,true) as u64),
         round_model(q[2],ns::word(s,at+16,8,true) as u64),round_model(q[3],ns::word(s,at+24,8,true) as u64)]
    }
}
spec fn mix(q:[u64;4]) -> u64 {
    let h=addm(addm(addm(rotm(q[0],1),rotm(q[1],7)),rotm(q[2],12)),rotm(q[3],18));
    merge_model(merge_model(merge_model(merge_model(h,q[0]),q[1]),q[2]),q[3])
}
spec fn avalanche(h:u64) -> u64 {
    let a=mulm(h^(h>>33),P2);let b=mulm(a^(a>>29),P3);b^(b>>32)
}
#[verifier::opaque]
spec fn tail(s:Seq<u8>,p:int,len:int,h:u64) -> u64 decreases len-p {
    if p>=len{avalanche(h)}else if len-p>=8{
        let next=addm(mulm(rotm(h^round_model(0,ns::word(s,p,8,true) as u64),27),P1),P4);
        tail(s,p+8,len,next)
    }else if len-p>=4{
        let next=addm(mulm(rotm(h^mulm(ns::word(s,p,4,true) as u64,P1),23),P2),P3);
        tail(s,p+4,len,next)
    }else{
        let next=mulm(rotm(h^mulm(s[p] as u64,P5),11),P1);tail(s,p+1,len,next)
    }
}
pub(super) closed spec fn checksum(s:Seq<u8>,len:usize) -> u64 {
    let blocks=len/32;let h=if len>=32 {mix(lanes(s,blocks as nat))}else{P5};
    tail(s,blocks*32,len as int,addm(h,len as u64))
}
pub(super) fn compute(s:&[u8],len:usize) -> (r:u64)
    requires len<=s.len(),
    ensures r==checksum(s@,len),
{
    reveal(checksum);
    let blocks=len/32;let mut q=[addw(P1,P2),P2,0u64,7046029288634856825u64];let mut i=0usize;
    reveal_with_fuel(lanes,1);
    while i<blocks
        invariant i<=blocks,blocks==len/32,len<=s.len(),q==lanes(s@,i as nat),
        decreases blocks-i,
    {
        let at=i*32;
        q[0]=round(q[0],ns::read_word(s,at,8,true).unwrap());
        q[1]=round(q[1],ns::read_word(s,at+8,8,true).unwrap());
        q[2]=round(q[2],ns::read_word(s,at+16,8,true).unwrap());
        q[3]=round(q[3],ns::read_word(s,at+24,8,true).unwrap());
        i=i+1;reveal_with_fuel(lanes,1);
    }
    let mut h=if len>=32{
        let v=addw(addw(addw(rot(q[0],1),rot(q[1],7)),rot(q[2],12)),rot(q[3],18));
        merge(merge(merge(merge(v,q[0]),q[1]),q[2]),q[3])
    }else{P5};
    h=addw(h,len as u64);let mut p=blocks*32;
    let ghost goal=checksum(s@,len);
    while len-p>=8
        invariant p<=len<=s.len(),goal==checksum(s@,len),goal==tail(s@,p as int,len as int,h),
        decreases len-p,
    {
        reveal_with_fuel(tail,1);
        h=addw(mulw(rot(h^round(0,ns::read_word(s,p,8,true).unwrap()),27),P1),P4);p=p+8;
    }
    reveal_with_fuel(tail,1);
    if len-p>=4{
        h=addw(mulw(rot(h^mulw(ns::read_word(s,p,4,true).unwrap(),P1),23),P2),P3);p=p+4;
    }
    while p<len
        invariant p<=len<=s.len(),len-p<4,goal==checksum(s@,len),goal==tail(s@,p as int,len as int,h),
        decreases len-p,
    {
        reveal_with_fuel(tail,1);
        h=mulw(rot(h^mulw(s[p] as u64,P5),11),P1);p=p+1;
    }
    reveal_with_fuel(tail,1);
    let a=mulw(h^(h>>33),P2);let b=mulw(a^(a>>29),P3);b^(b>>32)
}
}
