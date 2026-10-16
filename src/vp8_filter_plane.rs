//! VP8 deblocking order: outer and inner vertical edges, then horizontal edges.
use super::*;
use crate::vp8_filter_info::Filter;
use crate::vp8_filter_write as write;
use crate::vp8_edges as ed;
verus! {
pub(super) open spec fn step(p:Seq<u8>,w:usize,x:usize,y:usize,n:usize,chroma:bool,f:Filter,i:usize) -> Seq<u8> {
    let edges=n as int/4;let dir=(i as int)/(edges*n);let e=((i as int)/(n as int))%edges;let scan=(i as int)%(n as int);
    if f.limit==0 || (chroma && f.simple) || (e==0 && ((dir==0 && x==0) || (dir==1 && y==0))) || (e!=0 && !f.filter_inner){p}else{
        let base=if dir==0{(y+scan)*w+x+4*e-4}else{(y+4*e-4)*w+x+scan};
        let stride=if dir==0{1usize}else{w};
        write::filtered(p,base as usize,stride,f.simple,e==0,(f.limit+if e==0{4int}else{0int}) as u8,f.inner,f.threshold)
    }
}
fn filter_step(p:&mut[u8],w:usize,h:usize,x:usize,y:usize,n:usize,chroma:bool,f:Filter,i:usize)
    requires ed::geometry(old(p)@,w,h,x,y,n),n==8 || n==16,x%n==0,y%n==0,f.limit<=189,i<2*(n/4)*n,
    ensures final(p).len()==old(p).len(),final(p)@==step(old(p)@,w,x,y,n,chroma,f,i),
{
    let edges=n/4;let span=if n==8{16usize}else{64};
    if n==8{assert(edges==2);assert(edges*n==span) by(nonlinear_arith) requires edges==2,n==8,span==16;assert(i<2*span);}else{assert(n==16);assert(edges==4);assert(edges*n==span) by(nonlinear_arith) requires edges==4,n==16,span==64;assert(i<2*span);}
    let dir=i/span;let e=(i/n)%edges;let scan=i%n;
    if f.limit==0 || (chroma && f.simple) || (e==0 && ((dir==0 && x==0) || (dir==1 && y==0))) || (e!=0 && !f.filter_inner){return;}
    assert(dir<=1 && e<edges && scan<n);
    let base;
    let stride;
    if dir==0{
        assert(x+4*e>=4);
        let xx=x+4*e-4;let yy=y+scan;
        assert(xx+7<w && yy<h);
        assert(yy*w+xx+7<p.len()) by(nonlinear_arith) requires xx+7<w,yy<h,w>0,p.len()==w*h;
        assert(yy*w+xx<=268435455) by(nonlinear_arith) requires xx<w,yy<h,w<=16384,h<=16384;
        base=yy*w+xx;stride=1;
    }else{
        assert(y+4*e>=4);
        let xx=x+scan;let yy=y+4*e-4;
        assert(xx<w && yy+7<h);
        assert(yy*w+xx+7*w<p.len()) by(nonlinear_arith) requires xx<w,yy+7<h,w>0,p.len()==w*h;
        assert(yy*w+xx<=268435455) by(nonlinear_arith) requires xx<w,yy<h,w<=16384,h<=16384;
        base=yy*w+xx;stride=w;
    }
    assert(p.len()<=268435456) by(nonlinear_arith) requires w<=16384,h<=16384,p.len()==w*h;
    write::filter(p,base,stride,f.simple,e==0,f.limit+if e==0{4}else{0},f.inner,f.threshold);
}
#[verifier::opaque]
pub(super) open spec fn finish(p:Seq<u8>,w:usize,x:usize,y:usize,n:usize,chroma:bool,f:Filter,i:usize,fuel:nat) -> Seq<u8> decreases fuel {
    if fuel==0{p}else{finish(step(p,w,x,y,n,chroma,f,i),w,x,y,n,chroma,f,(i+1) as usize,(fuel-1) as nat)}
}
pub(super) fn filter(p:&mut[u8],w:usize,h:usize,x:usize,y:usize,n:usize,chroma:bool,f:Filter)
    requires ed::geometry(old(p)@,w,h,x,y,n),n==8 || n==16,x%n==0,y%n==0,f.limit<=189,
    ensures final(p).len()==old(p).len(),final(p)@==finish(old(p)@,w,x,y,n,chroma,f,0,(2*(n/4)*n) as nat),
{
    let total=2*(n/4)*n;let mut i=0usize;let ghost goal=finish(p@,w,x,y,n,chroma,f,0,total as nat);
    loop
        invariant ed::geometry(p@,w,h,x,y,n),n==8 || n==16,x%n==0,y%n==0,f.limit<=189,
            i<=total,total==2*(n/4)*n,p.len()==old(p).len(),
            goal==finish(old(p)@,w,x,y,n,chroma,f,0,total as nat),goal==finish(p@,w,x,y,n,chroma,f,i,(total-i) as nat),
        decreases total-i,
    {
        reveal_with_fuel(finish,1);if i==total{return;}
        filter_step(p,w,h,x,y,n,chroma,f,i);i=i+1;
    }
}
}
