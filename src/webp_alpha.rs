//! ALPH header validation and byte-exact inverse filtering.
use super::*;
verus! {
#[derive(Clone,Copy,Debug)]
pub struct Header {pub compressed:bool,pub filter:usize,pub preprocessed:bool}
pub closed spec fn header(s:Seq<u8>) -> Option<Header> {
    if s.len()<=1 || s[0]%4>1 || s[0]>=32{None}else{Some(Header {compressed:s[0]%4==1,filter:((s[0] as int/4)%4) as usize,preprocessed:s[0]>=16})}
}
pub fn inspect(s:&[u8]) -> (res:Result<Header,Error>)
    ensures res.is_ok()==header(s@).is_some(),res.is_ok() ==> res.unwrap()==header(s@).unwrap() && res.unwrap().filter<4,
{
    reveal(header);if s.len()<=1 || s[0]%4>1 || s[0]>=32{return Err(Error::Invalid);}
    Ok(Header {compressed:s[0]%4==1,filter:(s[0]/4%4) as usize,preprocessed:s[0]>=16})
}
pub(super) open spec fn predictor(p:Seq<u8>,w:usize,filter:usize,i:usize) -> u8 {
    if filter==0 || i==0{0u8}else if i<w{p[(i as int)-1]}else if i%w==0{p[(i as int)-w]}
    else if filter==1{p[(i as int)-1]}else if filter==2{p[(i as int)-w]}
    else{crate::vp8_predict::clipped(p[(i as int)-1] as int+p[(i as int)-w] as int-p[(i as int)-w-1] as int)}
}
fn predict(p:&[u8],w:usize,filter:usize,i:usize) -> (r:u8)
    requires w>0,filter<4,i<p.len(),
    ensures r==predictor(p@,w,filter,i),
{
    if filter==0 || i==0{0}else if i<w{p[i-1]}else if i%w==0{p[i-w]}
    else if filter==1{p[i-1]}else if filter==2{p[i-w]}
    else{proof{if i==w{vstd::arithmetic::div_mod::lemma_mod_self_0(w as int);}}assert(i>w);crate::vp8_predict::clip(p[i-1] as i32+p[i-w] as i32-p[i-w-1] as i32)}
}
#[verifier::opaque]
pub closed spec fn finish(p:Seq<u8>,w:usize,filter:usize,i:usize,fuel:nat) -> Seq<u8> decreases fuel {
    if fuel==0{p}else{
        let v=((p[i as int] as int+predictor(p,w,filter,i) as int)%256) as u8;
        finish(p.update(i as int,v),w,filter,(i+1) as usize,(fuel-1) as nat)
    }
}
pub fn unfilter(p:&mut[u8],w:usize,h:usize,filter:usize,budget:&mut Budget) -> (res:Result<(),Error>)
    ensures final(p).len()==old(p).len(),final(budget).remaining<=old(budget).remaining,
        res.is_ok()==(1<=w<=16384 && 1<=h<=16384 && filter<4 && old(p).len()>=w as int*h && 10*w*h<=old(budget).remaining),
        res.is_ok() ==> final(p)@==finish(old(p)@,w,filter,0,(w*h) as nat) && final(budget).remaining as int+10*w*h==old(budget).remaining,
        forall|i:int|w as int*h<=i<old(p).len() ==> final(p)@[i]==old(p)@[i],
{
    if w==0 || w>16384 || h==0 || h>16384 || filter>=4{return Err(Error::Invalid);}
    assert(w*h<=268435456) by(nonlinear_arith) requires w<=16384,h<=16384;
    let total=w*h;if p.len()<total{return Err(Error::OutputTooSmall);}
    assert(total*10==10*w*h) by(nonlinear_arith) requires total==w*h;
    let fee=mul(total,10)?;spend(budget,fee)?;
    let ghost goal=finish(p@,w,filter,0,total as nat);let mut i=0usize;
    loop
        invariant 1<=w<=16384,1<=h<=16384,filter<4,i<=total,total==w*h,p.len()>=total,p.len()==old(p).len(),
            budget.remaining as int+10*w*h==old(budget).remaining,
            goal==finish(old(p)@,w,filter,0,total as nat),goal==finish(p@,w,filter,i,(total-i) as nat),
            forall|j:int|total<=j<p.len() ==> p@[j]==old(p)@[j],
        decreases total-i,
    {
        reveal_with_fuel(finish,1);if i==total{return Ok(());}
        let pred=predict(p,w,filter,i);p[i]=((p[i] as u16+pred as u16)%256) as u8;i=i+1;
    }
}
}
