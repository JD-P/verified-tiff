//! Bounded VP8 probability trees and category suffixes.
use super::*;
use crate::vp8_bool as ar;
verus! {
#[verifier::opaque]
pub(super) open spec fn walk(s:Seq<u8>,r:ar::Reader,p:Seq<u8>,tree:Seq<i16>,node:usize,fuel:nat) -> Option<(u8,ar::Reader)> decreases fuel {
    if node>=p.len() || fuel==0{None}else{
        let b=ar::symbol(s,r,p[node as int]);
        if b.is_none(){None}else{
            let child=tree[2*node+if b.unwrap().0{1int}else{0int}];
            if child<=0 {if child< -255{None}else{Some(((-child) as u8,b.unwrap().1))}}
            else{walk(s,b.unwrap().1,p,tree,child as usize,(fuel-1) as nat)}
        }
    }
}
pub(super) fn decode(s:&[u8],r:ar::Reader,p:&[u8],tree:&[i16],root:usize) -> (out:Result<(u8,ar::Reader),Error>)
    requires r.valid(s@),p.len()<=32,tree.len()==2*p.len(),
    ensures out.is_ok()==walk(s@,r,p@,tree@,root,32).is_some(),
        out.is_ok() ==> out.unwrap()==walk(s@,r,p@,tree@,root,32).unwrap() && out.unwrap().1.valid(s@),
        out.is_ok() ==> exists|j:int|0<=j<tree.len() && tree@[j] as int==-(out.unwrap().0 as int),
{
    let ghost goal=walk(s@,r,p@,tree@,root,32);let mut node=root;let mut t=r;let mut fuel=32usize;
    loop
        invariant goal==walk(s@,r,p@,tree@,root,32),t.valid(s@),fuel<=32,p.len()<=32,tree.len()==2*p.len(),goal==walk(s@,t,p@,tree@,node,fuel as nat),
        decreases fuel,
    {
        reveal_with_fuel(walk,1);if node>=p.len() || fuel==0{return Err(Error::Invalid);}
        let (b,after)=ar::bit(s,t,p[node])?;t=after;
        let child=tree[2*node+if b{1}else{0}];
        if child<=0{if child< -255{return Err(Error::Invalid);}return Ok(((-child) as u8,t));}
        node=child as usize;fuel=fuel-1;
    }
}
#[verifier::opaque]
pub(super) open spec fn suffix(s:Seq<u8>,r:ar::Reader,p:Seq<u8>,i:usize,v:u32,fuel:nat) -> Option<(u32,ar::Reader)> decreases fuel {
    if i==p.len(){Some((v,r))}else if fuel==0{None}else{
        let b=ar::symbol(s,r,p[i as int]);if b.is_none(){None}else{
            suffix(s,b.unwrap().1,p,(i+1) as usize,(2*v+if b.unwrap().0{1int}else{0int}) as u32,(fuel-1) as nat)
        }
    }
}
pub(super) fn extra(s:&[u8],r:ar::Reader,p:&[u8]) -> (out:Result<(u32,ar::Reader),Error>)
    requires r.valid(s@),p.len()<=11,
    ensures out.is_ok()==suffix(s@,r,p@,0,0,p.len() as nat).is_some(),
        out.is_ok() ==> out.unwrap()==suffix(s@,r,p@,0,0,p.len() as nat).unwrap() && out.unwrap().1.valid(s@) && out.unwrap().0<crate::zstd_bits::pow(p.len() as int),
{
    let ghost goal=suffix(s@,r,p@,0,0,p.len() as nat);let mut t=r;let mut i=0usize;let mut v=0u32;
    loop
        invariant goal==suffix(s@,r,p@,0,0,p.len() as nat),t.valid(s@),i<=p.len()<=11,v<crate::zstd_bits::pow(i as int),goal==suffix(s@,t,p@,i,v,(p.len()-i) as nat),
        decreases p.len()-i,
    {
        reveal_with_fuel(suffix,1);if i==p.len(){return Ok((v,t));}
        let (b,after)=ar::bit(s,t,p[i])?;t=after;
        proof{crate::zstd_bits::power_step(i as int);}
        v=2*v+if b{1}else{0};i=i+1;
    }
}
}
