//! A macroblock's luma or chroma plane as an ordered fold of 4x4 writes.
use super::*;
use crate::vp8_edges as ed;
use crate::vp8_render as render;
use crate::vp8_predict as pr;
verus! {
pub(super) open spec fn edges(p:Seq<u8>,w:usize,x:usize,y:usize,n:usize) -> Seq<u8> {
    Seq::new(33,|k:int|if k==0 || 1<=k<=n || 17<=k<17+n{ed::edge(p,w,x,y,k as usize)}else{0u8})
}
pub(super) open spec fn step(p:Seq<u8>,w:usize,x:usize,y:usize,n:usize,small:bool,mode:usize,modes:Seq<u8>,e:Seq<u8>,dc:u8,c:Seq<Seq<i32>>,first:usize,i:usize) -> Seq<u8> {
    let row=n/4;let ox=i%row*4;let oy=i/row*4;
    let pred=render::predictions(p,w,(x+ox) as usize,(y+oy) as usize,small,if small{modes[i as int] as usize}else{mode},e,dc,ox as usize,oy as usize);
    render::paint(p,w,(x+ox) as usize,(y+oy) as usize,pred,c[first+i],16)
}
#[verifier::opaque]
pub(super) open spec fn finish(p:Seq<u8>,w:usize,x:usize,y:usize,n:usize,small:bool,mode:usize,modes:Seq<u8>,e:Seq<u8>,dc:u8,c:Seq<Seq<i32>>,first:usize,i:usize,fuel:nat) -> Seq<u8> decreases fuel {
    if fuel==0{p}else{finish(step(p,w,x,y,n,small,mode,modes,e,dc,c,first,i),w,x,y,n,small,mode,modes,e,dc,c,first,(i+1) as usize,(fuel-1) as nat)}
}
pub(super) open spec fn decoded(p:Seq<u8>,w:usize,x:usize,y:usize,n:usize,small:bool,mode:usize,modes:Seq<u8>,c:Seq<Seq<i32>>,first:usize) -> Seq<u8> {
    let e=edges(p,w,x,y,n);let dc=pr::dc(e,n,y>0,x>0);
    finish(p,w,x,y,n,small,mode,modes,e,dc,c,first,0,(n*n/16) as nat)
}
pub(super) fn decode(p:&mut[u8],w:usize,h:usize,x:usize,y:usize,n:usize,small:bool,mode:usize,modes:&[u8;16],c:&[[i32;16];24],first:usize)
    requires ed::geometry(old(p)@,w,h,x,y,n),n==8 || n==16,first as int+n*n/16<=24,mode<4,
        small ==> n==16 && x%16==0 && y%16==0 && w%16==0,
        forall|j:int|0<=j<16 ==> #[trigger] modes@[j]<10,
        forall|j:int,k:int|0<=j<24 && 0<=k<16 ==> -32768<=#[trigger] c[j][k]<=32767,
    ensures final(p).len()==old(p).len(),final(p)@==decoded(old(p)@,w,x,y,n,small,mode,modes@,Seq::new(24,|j:int|c[j]@),first),
{
    let e=ed::large(p,w,h,x,y,n);assert(e@ =~= edges(p@,w,x,y,n));
    let dc=pr::average(&e,n,y>0,x>0);let ghost cc=Seq::new(24,|j:int|c[j]@);let ghost before=p@;
    let total=n*n/16;let row=n/4;let mut i=0usize;
    let ghost goal=decoded(p@,w,x,y,n,small,mode,modes@,cc,first);
    loop
        invariant i<=total,total==n*n/16,row==n/4,n==8 || n==16,first as int+total<=24,mode<4,
            ed::geometry(p@,w,h,x,y,n),before==old(p)@,p.len()==before.len(),
            small ==> n==16 && x%16==0 && y%16==0 && w%16==0,
            cc==Seq::new(24,|j:int|c[j]@),e@==edges(before,w,x,y,n),dc==pr::dc(e@,n,y>0,x>0),
            goal==decoded(before,w,x,y,n,small,mode,modes@,cc,first),
            goal==finish(p@,w,x,y,n,small,mode,modes@,e@,dc,cc,first,i,(total-i) as nat),
            forall|j:int|0<=j<16 ==> #[trigger] modes@[j]<10,
            forall|j:int,k:int|0<=j<24 && 0<=k<16 ==> -32768<=#[trigger] c[j][k]<=32767,
        decreases total-i,
    {
        reveal_with_fuel(finish,1);if i==total{return;}
        let ox=i%row*4;let oy=i/row*4;let bx=x+ox;let by=y+oy;
        assert(ox<=12 && oy<=12 && ox+4<=n && oy+4<=n);
        if small{assert(bx%4==0 && by%4==0);}
        let m=if small{modes[i] as usize}else{mode};let ghost previous=p@;
        render::block(p,w,h,bx,by,small,m,&e,dc,ox,oy,&c[first+i]);
        assert(p@==step(previous,w,x,y,n,small,mode,modes@,e@,dc,cc,first,i));i=i+1;
    }
}
}
