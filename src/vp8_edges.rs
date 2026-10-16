//! Intra-prediction boundaries, including VP8's replicated macroblock top-right.
use super::*;
verus! {
pub(super) open spec fn geometry(p:Seq<u8>,w:usize,h:usize,x:usize,y:usize,n:usize) -> bool {
    1<=w<=16384 && 1<=h<=16384 && p.len()==w as int*h && 1<=n<=16 && x as int+n<=w && y as int+n<=h
}
pub(super) open spec fn edge(p:Seq<u8>,w:usize,x:usize,y:usize,k:usize) -> u8 {
    if k==0{if y==0{127u8}else if x==0{129u8}else{p[(y-1)*w+x-1]}}
    else if k<=16{if y==0{127u8}else{p[(y-1)*w+x+k-1]}}
    else{if x==0{129u8}else{p[(y+k-17)*w+x-1]}}
}
pub(super) fn large(p:&[u8],w:usize,h:usize,x:usize,y:usize,n:usize) -> (r:[u8;33])
    requires geometry(p@,w,h,x,y,n),n==8 || n==16,
    ensures forall|k:int|0<=k<33 ==> #[trigger] r@[k]==(if k==0 || 1<=k<=n || 17<=k<17+n{edge(p@,w,x,y,k as usize)}else{0u8}),
{
    assert(w*h<=268435456) by(nonlinear_arith) requires w<=16384,h<=16384;
    if y>0 && x>0{assert((y-1)*w+x-1<p.len()) by(nonlinear_arith) requires y>0,x>0,y+n<=h,x+n<=w,n>0,p.len()==w*h,w>0;}
    let mut e=[0u8;33];e[0]=if y==0{127}else if x==0{129}else{p[(y-1)*w+x-1]};
    let mut i=0usize;
    while i<n
        invariant geometry(p@,w,h,x,y,n),n==8 || n==16,i<=n,w*h<=268435456,
            e@[0]==edge(p@,w,x,y,0),
            forall|k:int|1<=k<=i ==> #[trigger] e@[k]==edge(p@,w,x,y,k as usize),
            forall|k:int|17<=k<17+i ==> #[trigger] e@[k]==edge(p@,w,x,y,k as usize),
            forall|k:int|i<k<=16 || 17+i<=k<33 ==> #[trigger] e@[k]==0,
        decreases n-i,
    {
        if y>0{assert((y-1)*w+x+i<p.len()) by(nonlinear_arith) requires y>0,y+n<=h,x+n<=w,i<n,w>0,p.len()==w*h;}
        if x>0{assert((y+i)*w+x-1<p.len()) by(nonlinear_arith) requires x>0,y+n<=h,x+n<=w,i<n,w>0,p.len()==w*h;}
        e[1+i]=if y==0{127}else{p[(y-1)*w+x+i]};
        e[17+i]=if x==0{129}else{p[(y+i)*w+x-1]};i=i+1;
    }
    e
}
pub(super) open spec fn little(p:Seq<u8>,w:usize,x:usize,y:usize,k:usize) -> u8 {
    if k==0{edge(p,w,x,y,0)}else if k>=9{
        if x==0{129u8}else{p[(y+k-9)*w+x-1]}
    }else{
        let yy=if k>=5 && x%16==12{y-y%16}else{y as int};
        let xx=if x+k-1<w{x+k-1}else{w-1};
        if yy==0{127u8}else{p[(yy-1)*w+xx]}
    }
}
pub(super) fn small(p:&[u8],w:usize,h:usize,x:usize,y:usize) -> (e:[u8;13])
    requires geometry(p@,w,h,x,y,4),x%4==0,y%4==0,w%16==0,
    ensures forall|k:int|0<=k<13 ==> #[trigger] e@[k]==little(p@,w,x,y,k as usize),
{
    assert(w*h<=268435456) by(nonlinear_arith) requires w<=16384,h<=16384;
    let mut e=[0u8;13];let mut k=0usize;
    while k<13
        invariant geometry(p@,w,h,x,y,4),x%4==0,y%4==0,w%16==0,k<=13,w*h<=268435456,
            forall|j:int|0<=j<k ==> #[trigger] e@[j]==little(p@,w,x,y,j as usize),
        decreases 13-k,
    {
        if k==0{
            if y>0 && x>0{assert((y-1)*w+x-1<p.len()) by(nonlinear_arith) requires y>0,x>0,y+4<=h,x+4<=w,p.len()==w*h,w>0;}
            e[k]=if y==0{127}else if x==0{129}else{p[(y-1)*w+x-1]};
        }else if k>=9{
            if x>0{assert((y+k-9)*w+x-1<p.len()) by(nonlinear_arith) requires x>0,9<=k<13,y+4<=h,x+4<=w,p.len()==w*h,w>0;}
            e[k]=if x==0{129}else{p[(y+k-9)*w+x-1]};
        }else{
            let yy=if k>=5 && x%16==12{y-y%16}else{y};let xx=if x+k-1<w{x+k-1}else{w-1};
            if yy>0{assert((yy-1)*w+xx<p.len()) by(nonlinear_arith) requires 0<yy<=y,y<h,xx<w,w>0,p.len()==w*h;}
            e[k]=if yy==0{127}else{p[(yy-1)*w+xx]};
        }
        k=k+1;
    }
    e
}
}
