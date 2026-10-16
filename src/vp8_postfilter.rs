//! Raster-order frame filtering, keeping the three color planes separate.
use super::*;
use crate::vp8_filter_info::Filter;
use crate::vp8_filter_plane as plane;
verus! {
pub(super) struct Model {pub y:Seq<u8>,pub u:Seq<u8>,pub v:Seq<u8>}
pub(super) open spec fn step(m:Model,w:usize,filters:Seq<Filter>,i:usize) -> Model {
    let width=w/16;let x=i%width;let y=i/width;let f=filters[i as int];
    Model {y:plane::finish(m.y,w,(x*16) as usize,(y*16) as usize,16,false,f,0,128),
        u:plane::finish(m.u,(w/2) as usize,(x*8) as usize,(y*8) as usize,8,true,f,0,32),
        v:plane::finish(m.v,(w/2) as usize,(x*8) as usize,(y*8) as usize,8,true,f,0,32)}
}
#[verifier::opaque]
pub(super) open spec fn finish(m:Model,w:usize,filters:Seq<Filter>,i:usize,fuel:nat) -> Model decreases fuel {
    if fuel==0{m}else{finish(step(m,w,filters,i),w,filters,(i+1) as usize,(fuel-1) as nat)}
}
pub(super) fn filter(w:usize,h:usize,filters:&[Filter],yp:&mut[u8],up:&mut[u8],vp:&mut[u8])
    requires 16<=w<=16384,16<=h<=16384,w%16==0,h%16==0,filters.len()==(w/16)*(h/16),
        old(yp).len()==w as int*h,old(up).len()==(w/2)*(h/2),old(vp).len()==old(up).len(),
        forall|i:int|0<=i<filters.len() ==> #[trigger] filters@[i].limit<=189,
    ensures final(yp).len()==old(yp).len(),final(up).len()==old(up).len(),final(vp).len()==old(vp).len(),
        (Model {y:final(yp)@,u:final(up)@,v:final(vp)@})==finish(Model {y:old(yp)@,u:old(up)@,v:old(vp)@},w,filters@,0,filters.len() as nat),
{
    let ghost initial=Model {y:yp@,u:up@,v:vp@};let ghost goal=finish(initial,w,filters@,0,filters.len() as nat);let mut i=0usize;
    loop
        invariant i<=filters.len(),16<=w<=16384,16<=h<=16384,w%16==0,h%16==0,filters.len()==(w/16)*(h/16),
            yp.len()==w as int*h,up.len()==(w/2)*(h/2),vp.len()==up.len(),yp.len()==old(yp).len(),up.len()==old(up).len(),vp.len()==old(vp).len(),
            initial==(Model {y:old(yp)@,u:old(up)@,v:old(vp)@}),goal==finish(initial,w,filters@,0,filters.len() as nat),
            goal==finish(Model {y:yp@,u:up@,v:vp@},w,filters@,i,(filters.len()-i) as nat),
            forall|j:int|0<=j<filters.len() ==> #[trigger] filters@[j].limit<=189,
        decreases filters.len()-i,
    {
        reveal_with_fuel(finish,1);if i==filters.len(){return;}
        let width=w/16;let x=i%width;let y=i/width;
        proof{vstd::arithmetic::div_mod::lemma_fundamental_div_mod(i as int,width as int);}
        assert(y<h/16) by(nonlinear_arith) requires i==width*y+x,0<=x<width,i<width*(h/16),width>0;
        let f=filters[i];
        plane::filter(yp,w,h,x*16,y*16,16,false,f);
        plane::filter(up,w/2,h/2,x*8,y*8,8,true,f);
        plane::filter(vp,w/2,h/2,x*8,y*8,8,true,f);i=i+1;
    }
}
}
