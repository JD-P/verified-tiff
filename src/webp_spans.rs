//! VP8L kernels selected once per row/tile span, refining the scalar model.
use super::*;
use crate::webp_pixels::{self as pix,Pixel};
use crate::webp_transforms::Transform;
use crate::webp_inverse as inv;
use crate::webp_predict as pred;
use crate::webp_simd as simd;
verus! {
#[verifier::spinoff_prover]
pub(super) fn select(t:Transform,data:&[Pixel],k:usize,x:usize,y:usize) -> (r:Result<(usize,Pixel),Error>)
    requires t.valid(),t.kind<2,k<t.width*t.height,x==k%t.width,y==k/t.width,
    ensures r.is_ok()==inv::coefficient(t,data@,k).is_some(),
        r.is_ok() ==> 1<=r.unwrap().0<=t.width-x && k+r.unwrap().0<=t.width*t.height,
        r.is_ok() ==> forall|j:usize|k<=j<k+r.unwrap().0 ==> #[trigger] inv::coefficient(t,data@,j)==Some(r.unwrap().1),
{
    reveal(inv::coefficient);
    let m=inv::matrix(t,data,k,x,y)?;let side=pix::size(t.bits);
    let rem=side-x%side;let n=if rem<t.width-x{rem}else{t.width-x};
    proof{
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod(k as int,t.width as int);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod(x as int,side as int);
    }
    assert(y<t.height) by(nonlinear_arith) requires k==y*t.width+x,x>=0,k<t.width*t.height,t.width>0;
    assert(k+n<=t.width*t.height) by(nonlinear_arith) requires k==y*t.width+x,y<t.height,n<=t.width-x,t.width>0;
    assert(inv::coefficient(t,data@,k)==Some(m));
    assert forall|p:usize|k<=p<k+n implies #[trigger] inv::coefficient(t,data@,p)==Some(m) by {
        let j=p as int-k as int;let q=x as int/side as int;let r=x as int%side as int;
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(x as int+j,side as int,q,r+j);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(p as int,t.width as int,y as int,x as int+j);
        assert(p/t.width==y && p%t.width==x+j);
        assert((p%t.width)/side==x/side);
        assert(k/t.width==y && k%t.width==x);
    }
    Ok((n,m))
}
proof fn independent(t:Transform,data:Seq<Pixel>,a:Seq<Pixel>,b:Seq<Pixel>,k:usize)
    requires t.valid(),t.kind==1 || t.kind==2,k<a.len(),k<b.len(),a[k as int]==b[k as int],
    ensures inv::value(t,data,a,k)==inv::value(t,data,b,k),
{reveal(inv::value);}
fn write4(t:Transform,data:&[Pixel],out:&mut[Pixel],k:usize,v:[Pixel;4])
    requires t.valid(),t.kind==1 || t.kind==2,k+4<=t.width*t.height,t.width*t.height<=old(out).len(),
        forall|j:int|0<=j<4 ==> #[trigger] inv::value(t,data@,old(out)@,(k+j) as usize)==Some(v@[j]),
    ensures final(out).len()==old(out).len(),
        inv::finish(t,data@,old(out)@,k,(t.width*t.height-k) as nat)==inv::finish(t,data@,final(out)@,(k+4) as usize,(t.width*t.height-k-4) as nat),
        forall|i:int|0<=i<old(out).len() && (i<k || i>=k+4) ==> final(out)@[i]==old(out)@[i],
{
    let ghost before=out@;let ghost goal=inv::finish(t,data@,out@,k,(t.width*t.height-k) as nat);
    let mut j=0usize;
    while j<4
        invariant j<=4,t.valid(),t.kind==1 || t.kind==2,k+4<=t.width*t.height,t.width*t.height<=out.len(),out.len()==before.len(),
            goal==inv::finish(t,data@,before,k,(t.width*t.height-k) as nat),
            goal==inv::finish(t,data@,out@,(k+j) as usize,(t.width*t.height-k-j) as nat),
            forall|n:int|j<=n<4 ==> #[trigger] inv::value(t,data@,out@,(k+n) as usize)==Some(v@[n]),
            forall|i:int|0<=i<out.len() && (i<k || i>=k+j) ==> out@[i]==before[i],
        decreases 4-j,
    {
        reveal_with_fuel(inv::finish,1);reveal(inv::value);
        let ghost previous=out@;
        assert(inv::value(t,data@,previous,(k+j) as usize)==Some(v@[j as int]));
        out[k+j]=v[j];
        assert forall|n:int|j+1<=n<4 implies #[trigger] inv::value(t,data@,out@,(k+n) as usize)==Some(v@[n]) by {
            assert(inv::value(t,data@,previous,(k+n) as usize)==Some(v@[n]));
            independent(t,data@,previous,out@,(k+n) as usize);
        }
        j=j+1;
    }
}
pub(super) fn colors(t:Transform,data:&[Pixel],out:&mut[Pixel],start:usize,n:usize,m:Pixel)
    requires t.valid(),t.kind==1 || t.kind==2,1<=n,start+n<=t.width*t.height,t.width*t.height<=old(out).len(),
        t.kind==1 ==> forall|j:usize|start<=j<start+n ==> #[trigger] inv::coefficient(t,data@,j)==Some(m),
    ensures final(out).len()==old(out).len(),
        inv::finish(t,data@,old(out)@,start,(t.width*t.height-start) as nat)==inv::finish(t,data@,final(out)@,(start+n) as usize,(t.width*t.height-start-n) as nat),
        forall|i:int|0<=i<old(out).len() && (i<start || i>=start+n) ==> final(out)@[i]==old(out)@[i],
{
    let end=start+n;let mut k=start;let ghost before=out@;
    let ghost goal=inv::finish(t,data@,out@,start,(t.width*t.height-start) as nat);
    while k<end
        invariant t.valid(),t.kind==1 || t.kind==2,start<=k<=end,end==start+n,end<=t.width*t.height,t.width*t.height<=out.len(),out.len()==before.len(),
            t.kind==1 ==> forall|j:usize|start<=j<end ==> #[trigger] inv::coefficient(t,data@,j)==Some(m),
            goal==inv::finish(t,data@,before,start,(t.width*t.height-start) as nat),
            goal==inv::finish(t,data@,out@,k,(t.width*t.height-k) as nat),
            forall|i:int|0<=i<out.len() && (i<start || i>=k) ==> out@[i]==before[i],
        decreases end-k,
    {
        reveal(inv::value);
        if end-k>=4 {
            let p=[out[k],out[k+1],out[k+2],out[k+3]];
            let v=if t.kind==2{simd::green4(p)}else{simd::color4(p,m)};
            assert forall|j:int|0<=j<4 implies #[trigger] inv::value(t,data@,out@,(k+j) as usize)==Some(v@[j]) by {
                if t.kind==1{assert(inv::coefficient(t,data@,(k+j) as usize)==Some(m));}
            }
            write4(t,data,out,k,v);k=k+4;
        }else{
            reveal_with_fuel(inv::finish,1);
            let v=if t.kind==2{pred::add_green(out[k])}else{pred::inverse_color(out[k],m)};
            if t.kind==1{assert(inv::coefficient(t,data@,k)==Some(m));}
            assert(inv::value(t,data@,out@,k)==Some(v));
            out[k]=v;k=k+1;
        }
    }
}
pub(super) fn predict(t:Transform,data:&[Pixel],out:&mut[Pixel],start:usize,n:usize,x:usize,y:usize,m:Pixel)
    requires t.valid(),t.kind==0,1<=n,0<x<t.width,0<y<t.height,start==y*t.width+x,n<=t.width-x,
        t.width*t.height<=old(out).len(),m[1]%16<=13,
        forall|j:usize|start<=j<start+n ==> #[trigger] inv::coefficient(t,data@,j)==Some(m),
    ensures final(out).len()==old(out).len(),
        inv::finish(t,data@,old(out)@,start,(t.width*t.height-start) as nat)==inv::finish(t,data@,final(out)@,(start+n) as usize,(t.width*t.height-start-n) as nat),
        forall|i:int|0<=i<old(out).len() && (i<start || i>=start+n) ==> final(out)@[i]==old(out)@[i],
{
    assert(start+n<=t.width*t.height) by(nonlinear_arith) requires start==y*t.width+x,y<t.height,n<=t.width-x,t.width>0;
    assert(start>t.width) by(nonlinear_arith) requires start==y*t.width+x,y>0,x>0,t.width>0;
    let end=start+n;let mut k=start;let mode=(m[1]%16) as usize;
    let mut left=out[start-1];let mut tl=out[start-t.width-1];
    let ghost before=out@;let ghost goal=inv::finish(t,data@,out@,start,(t.width*t.height-start) as nat);
    while k<end
        invariant t.valid(),t.kind==0,0<x<t.width,0<y<t.height,start==y*t.width+x,
            start<=k<=end,end==start+n,n<=t.width-x,end<=t.width*t.height,start>t.width,
            t.width*t.height<=out.len(),out.len()==before.len(),mode==m[1]%16,mode<=13,
            left==out@[k-1],tl==out@[k-t.width-1],
            forall|j:usize|start<=j<end ==> #[trigger] inv::coefficient(t,data@,j)==Some(m),
            goal==inv::finish(t,data@,before,start,(t.width*t.height-start) as nat),
            goal==inv::finish(t,data@,out@,k,(t.width*t.height-k) as nat),
            forall|i:int|0<=i<out.len() && (i<start || i>=k) ==> out@[i]==before[i],
        decreases end-k,
    {
        reveal(inv::value);reveal_with_fuel(inv::finish,1);
        proof{vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(k as int,t.width as int,y as int,(x+k-start) as int);}
        let top=out[k-t.width];
        let v=if mode==12{simd::predict12(out[k],left,top,tl)}else{
            pred::add_pixel(out[k],pred::predict(mode,left,top,tl,out[k-t.width+1]))
        };
        assert(inv::coefficient(t,data@,k)==Some(m));
        assert(k%t.width==x+k-start && k>=t.width);
        assert(inv::value(t,data@,out@,k)==Some(v));
        out[k]=v;left=v;tl=top;k=k+1;
    }
}
}
