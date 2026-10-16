//! Prediction, inverse-transform addition, and exact plane write frames.
use super::*;
use crate::vp8_predict as pr;
use crate::vp8_transform as tx;
use crate::vp8_edges as ed;
verus! {
pub(super) open spec fn predictions(p:Seq<u8>,w:usize,x:usize,y:usize,small:bool,mode:usize,e:Seq<u8>,dc:u8,ox:usize,oy:usize) -> Seq<u8> {
    let se=Seq::new(13,|k:int|ed::little(p,w,x,y,k as usize));
    Seq::new(16,|k:int|if small{pr::small(se,mode,k as usize)}else{pr::large(e,mode,(ox+k%4) as usize,(oy+k/4) as usize,dc)})
}
#[verifier::opaque]
pub(super) open spec fn paint_values(p:Seq<u8>,w:usize,x:usize,y:usize,pred:Seq<u8>,delta:Seq<i32>,limit:usize) -> Seq<u8> {
    Seq::new(p.len(),|j:int|{
        let xx=j%(w as int);let yy=j/(w as int);let k=(yy-y)*4+xx-x;
        if x<=xx<x+4 && y<=yy<y+4 && k<limit{pr::clipped(pred[k] as int+delta[k] as int)}else{p[j]}
    })
}
pub(super) open spec fn paint(p:Seq<u8>,w:usize,x:usize,y:usize,pred:Seq<u8>,c:Seq<i32>,limit:usize) -> Seq<u8> {
    paint_values(p,w,x,y,pred,Seq::new(16,|k:int|tx::sample(c,k,false)),limit)
}
proof fn painted_step(p:Seq<u8>,w:usize,h:usize,x:usize,y:usize,pred:Seq<u8>,delta:Seq<i32>,k:usize)
    requires ed::geometry(p,w,h,x,y,4),pred.len()==16,delta.len()==16,k<16,
    ensures paint_values(p,w,x,y,pred,delta,k).update((y+k/4)*w+(x+k%4),pr::clipped(pred[k as int] as int+delta[k as int] as int))==paint_values(p,w,x,y,pred,delta,(k+1) as usize),
{
    reveal(paint_values);
    let at=(y+k/4)*w+(x+k%4);
    assert(at<p.len()) by(nonlinear_arith) requires y+4<=h,x+4<=w,k/4<4,k%4<4,w>0,p.len()==w as int*h,at==(y+k/4)*w+(x+k%4);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(at,w as int,y+k/4,x+k%4);
    assert forall|j:int|0<=j<p.len() implies
        paint_values(p,w,x,y,pred,delta,k).update(at,pr::clipped(pred[k as int] as int+delta[k as int] as int))[j]==#[trigger] paint_values(p,w,x,y,pred,delta,(k+1) as usize)[j] by {
        if x<=j%(w as int)<x+4 && y<=j/(w as int)<y+4 && (j/(w as int)-y)*4+j%(w as int)-x==k {
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(k as int,4,j/(w as int)-y,j%(w as int)-x);
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(j,w as int);
            assert(j/(w as int)==y+k/4 && j%(w as int)==x+k%4);
            assert(j==at) by(nonlinear_arith) requires j==w*(j/(w as int))+j%(w as int),j/(w as int)==y+k/4,j%(w as int)==x+k%4,at==(y+k/4)*w+(x+k%4);
        }
    }
    assert(paint_values(p,w,x,y,pred,delta,k).update(at,pr::clipped(pred[k as int] as int+delta[k as int] as int)) =~= paint_values(p,w,x,y,pred,delta,(k+1) as usize));
}
fn write(p:&mut[u8],w:usize,h:usize,x:usize,y:usize,pred:&[u8;16],delta:&[i32;16])
    requires ed::geometry(old(p)@,w,h,x,y,4),forall|j:int|0<=j<16 ==> -1250000<=#[trigger] delta@[j]<=1250000,
    ensures final(p).len()==old(p).len(),final(p)@==paint_values(old(p)@,w,x,y,pred@,delta@,16),
{
    let ghost before=p@;let mut k=0usize;
    proof{reveal(paint_values);assert(p@ =~= paint_values(before,w,x,y,pred@,delta@,0));}
    while k<16
        invariant k<=16,ed::geometry(p@,w,h,x,y,4),ed::geometry(before,w,h,x,y,4),p@==paint_values(before,w,x,y,pred@,delta@,k),before==old(p)@,
            forall|j:int|0<=j<16 ==> -1250000<=#[trigger] delta@[j]<=1250000,
        decreases 16-k,
    {
        let xx=x+k%4;let yy=y+k/4;
        assert(yy*w+xx<p.len()) by(nonlinear_arith) requires yy<h,xx<w,w>0,p.len()==w*h;
        assert(yy*w+xx<=268435455) by(nonlinear_arith) requires yy<h,xx<w,w<=16384,h<=16384;
        let at=yy*w+xx;let v=pr::clip(pred[k] as i32+delta[k]);
        proof{painted_step(before,w,h,x,y,pred@,delta@,k);}
        p[at]=v;k=k+1;
    }
}
pub(super) fn block(p:&mut[u8],w:usize,h:usize,x:usize,y:usize,small:bool,mode:usize,e:&[u8;33],dc:u8,ox:usize,oy:usize,c:&[i32;16])
    requires ed::geometry(old(p)@,w,h,x,y,4),ox<=12,oy<=12,
        small ==> mode<10 && x%4==0 && y%4==0 && w%16==0,!small ==> mode<4,
        forall|j:int|0<=j<16 ==> -32768<=#[trigger] c@[j]<=32767,
    ensures final(p).len()==old(p).len(),final(p)@==paint(old(p)@,w,x,y,predictions(old(p)@,w,x,y,small,mode,e@,dc,ox,oy),c@,16),
{
    let se=if small{ed::small(p,w,h,x,y)}else{[0u8;13]};
    let mut pred=[0u8;16];let mut k=0usize;
    let ghost expected=predictions(p@,w,x,y,small,mode,e@,dc,ox,oy);
    while k<16
        invariant k<=16,ox<=12,oy<=12,small ==> mode<10,!small ==> mode<4,
            expected==predictions(p@,w,x,y,small,mode,e@,dc,ox,oy),
            small ==> forall|j:int|0<=j<13 ==> #[trigger] se@[j]==ed::little(p@,w,x,y,j as usize),
            forall|j:int|0<=j<k ==> #[trigger] pred@[j]==expected[j],
        decreases 16-k,
    {
        if small{assert(se@ =~= Seq::new(13,|j:int|ed::little(p@,w,x,y,j as usize)));}
        pred[k]=if small{pr::predict4(&se,mode,k)}else{pr::predict(e,mode,ox+k%4,oy+k/4,dc)};k=k+1;
    }
    assert(pred@ =~= expected);let delta=tx::transform(c,false);
    assert(delta@ =~= Seq::new(16,|j:int|tx::sample(c@,j,false)));
    write(p,w,h,x,y,&pred,&delta);
}
}
