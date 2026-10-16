//! VP8 macroblock segmentation, skip and intra-mode parsing.
use super::*;
use crate::vp8_bool as ar;
use crate::vp8_tree as tr;
use crate::vp8_fields::{self as fields,Field};
use crate::vp8_tables::BMODE;
verus! {
pub(super) const SEG_TREE:[i16;6]=[1,2,0,-1,-2,-3];
pub(super) const Y_TREE:[i16;6]=[1,2,0,-2,-3,-1];
pub(super) const UV_TREE:[i16;6]=[0,1,-2,2,-3,-1];
pub(super) const B_TREE:[i16;18]=[0,1,-1,2,-2,3,4,6,-3,5,-4,-5,-6,7,-7,8,-8,-9];
#[derive(Clone,Copy)]
pub(super) struct Prefix {pub segment:u8,pub skip:bool,pub small:bool}
pub(super) open spec fn head(s:Seq<u8>,r:ar::Reader,h:Seq<i32>) -> Option<(Prefix,ar::Reader)> {
    let seg=if h[3]!=0{tr::walk(s,r,seq![h[14] as u8,h[15] as u8,h[16] as u8],SEG_TREE@,0,32)}else{Some((0u8,r))};
    if seg.is_none(){None}else{
        let skip=if h[1094]!=0{ar::symbol(s,seg.unwrap().1,h[1095] as u8)}else{Some((false,seg.unwrap().1))};
        if skip.is_none(){None}else{
            let b=ar::symbol(s,skip.unwrap().1,145);
            if b.is_none(){None}else{Some((Prefix {segment:seg.unwrap().0,skip:skip.unwrap().0,small:!b.unwrap().0},b.unwrap().1))}
        }
    }
}
pub(super) fn prefix(s:&[u8],r:ar::Reader,h:&[i32;1096]) -> (res:Result<(Prefix,ar::Reader),Error>)
    requires r.valid(s@),
    ensures res.is_ok()==head(s@,r,h@).is_some(),
        res.is_ok() ==> res.unwrap()==head(s@,r,h@).unwrap() && res.unwrap().0.segment<4 && res.unwrap().1.valid(s@),
{
    let (segment,t)=if h[3]!=0 {tr::decode(s,r,&[h[14] as u8,h[15] as u8,h[16] as u8],&SEG_TREE,0)?}else{(0u8,r)};
    assert(segment<4);
    let (skip,t)=if h[1094]!=0{ar::bit(s,t,h[1095] as u8)?}else{(false,t)};
    let (large,t)=ar::bit(s,t,145)?;Ok((Prefix {segment,skip,small:!large},t))
}
pub(super) open spec fn large(s:Seq<u8>,r:ar::Reader,chroma:bool) -> Option<(u8,ar::Reader)> {
    tr::walk(s,r,if chroma{seq![142u8,114,183]}else{seq![156u8,163,128]},if chroma{UV_TREE@}else{Y_TREE@},0,32)
}
pub(super) fn main_mode(s:&[u8],r:ar::Reader,chroma:bool) -> (res:Result<(u8,ar::Reader),Error>)
    requires r.valid(s@),
    ensures res.is_ok()==large(s@,r,chroma).is_some(),res.is_ok() ==> res.unwrap()==large(s@,r,chroma).unwrap() && res.unwrap().0<4 && res.unwrap().1.valid(s@),
{
    if chroma{tr::decode(s,r,&[142u8,114,183],&UV_TREE,0)}else{tr::decode(s,r,&[156u8,163,128],&Y_TREE,0)}
}
pub(super) open spec fn probabilities(top:u8,left:u8) -> Seq<u8> {Seq::new(9,|j:int|BMODE[(top as int*10+left)*9+j])}
fn probs(top:u8,left:u8) -> (out:[u8;9])
    requires top<10,left<10,
    ensures out@==probabilities(top,left),
{
    let at=(top as usize*10+left as usize)*9;
    let mut out=[0u8;9];let mut i=0usize;
    while i<9
        invariant top<10,left<10,at==(top as usize*10+left as usize)*9,i<=9,forall|j:int|0<=j<i ==> #[trigger] out@[j]==BMODE[at+j],
        decreases 9-i,
    {out[i]=BMODE[at+i];i=i+1;}
    assert(out@ =~= probabilities(top,left));out
}
pub(super) open spec fn bmode(s:Seq<u8>,r:ar::Reader,top:u8,left:u8) -> Option<(u8,ar::Reader)> {tr::walk(s,r,probabilities(top,left),B_TREE@,0,32)}
pub(super) fn block_mode(s:&[u8],r:ar::Reader,top:u8,left:u8) -> (res:Result<(u8,ar::Reader),Error>)
    requires r.valid(s@),top<10,left<10,
    ensures res.is_ok()==bmode(s@,r,top,left).is_some(),res.is_ok() ==> res.unwrap()==bmode(s@,r,top,left).unwrap() && res.unwrap().0<10 && res.unwrap().1.valid(s@),
{
    let p=probs(top,left);tr::decode(s,r,&p,&B_TREE,0)
}
#[verifier::opaque]
pub(super) open spec fn finish(s:Seq<u8>,r:ar::Reader,top:Seq<u8>,left:Seq<u8>,modes:Seq<u8>,i:usize,fuel:nat) -> Option<(ar::Reader,Seq<u8>)> decreases fuel {
    if i==16{Some((r,modes))}else if fuel==0{None}else{
        let up=if i<4{top[i as int]}else{modes[(i as int)-4]};
        let l=if i%4==0{left[(i as int)/4]}else{modes[(i as int)-1]};
        let v=bmode(s,r,up,l);
        if v.is_none(){None}else{finish(s,v.unwrap().1,top,left,modes.update(i as int,v.unwrap().0),(i+1) as usize,(fuel-1) as nat)}
    }
}
pub(super) fn small_modes(s:&[u8],r:ar::Reader,top:&[u8;4],left:&[u8;4],modes:&mut[u8;16]) -> (res:Result<ar::Reader,Error>)
    requires r.valid(s@),forall|j:int|0<=j<4 ==> #[trigger] top@[j]<10 && left@[j]<10,
    ensures res.is_ok()==finish(s@,r,top@,left@,old(modes)@,0,16).is_some(),
        res.is_ok() ==> res.unwrap()==finish(s@,r,top@,left@,old(modes)@,0,16).unwrap().0 && final(modes)@==finish(s@,r,top@,left@,old(modes)@,0,16).unwrap().1 && res.unwrap().valid(s@),
        res.is_ok() ==> forall|j:int|0<=j<16 ==> #[trigger] final(modes)@[j]<10,
{
    let ghost goal=finish(s@,r,top@,left@,modes@,0,16);let mut t=r;let mut i=0usize;
    loop
        invariant goal==finish(s@,r,top@,left@,old(modes)@,0,16),goal==finish(s@,t,top@,left@,modes@,i,(16-i) as nat),
            i<=16,t.valid(s@),forall|j:int|0<=j<4 ==> #[trigger] top@[j]<10 && left@[j]<10,
            forall|j:int|0<=j<i ==> #[trigger] modes@[j]<10,
        decreases 16-i,
    {
        reveal_with_fuel(finish,1);if i==16{return Ok(t);}
        assert(top@[(i/4) as int]<10);
        let up=if i<4{top[i]}else{modes[i-4]};let l=if i%4==0{left[i/4]}else{modes[i-1]};
        let (v,after)=block_mode(s,t,up,l)?;t=after;modes[i]=v;i=i+1;
    }
}
}
