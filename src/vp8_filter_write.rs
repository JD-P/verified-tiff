//! Checked strided read/modify/write of the eight samples at a VP8 filter edge.
use super::*;
use crate::vp8_filter_kernel as kernel;
verus! {
pub(super) open spec fn samples(p:Seq<u8>,base:usize,step:usize) -> Seq<u8> {Seq::new(8,|i:int|p[base+i*step])}
#[verifier::opaque]
pub(super) open spec fn scatter(p:Seq<u8>,base:usize,step:usize,out:Seq<u8>,count:usize) -> Seq<u8> {
    Seq::new(p.len(),|j:int|if j>=base && (j-base)%(step as int)==0 && (j-base)/(step as int)<count{out[(j-base)/(step as int)]}else{p[j]})
}
proof fn next(p:Seq<u8>,base:usize,step:usize,out:Seq<u8>,i:usize)
    requires step>0,base as int+7*step<p.len(),out.len()==8,i<8,
    ensures scatter(p,base,step,out,i).update(base+i*step,out[i as int])==scatter(p,base,step,out,(i+1) as usize),
{
    reveal(scatter);
    let at=base+i*step;
    assert(at<p.len()) by(nonlinear_arith) requires i<8,step>0,base+7*step<p.len(),at==base+i*step;
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(at-base,step as int,i as int,0);
    assert forall|j:int|0<=j<p.len() implies
        scatter(p,base,step,out,i).update(at,out[i as int])[j]==#[trigger] scatter(p,base,step,out,(i+1) as usize)[j] by {
        if j>=base && (j-base)%(step as int)==0 && (j-base)/(step as int)==i{
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(j-base,step as int);
            assert(j==at) by(nonlinear_arith) requires j-base==step*((j-base)/(step as int))+(j-base)%(step as int),(j-base)%(step as int)==0,(j-base)/(step as int)==i,at==base+i*step;
        }
    }
    assert(scatter(p,base,step,out,i).update(at,out[i as int]) =~= scatter(p,base,step,out,(i+1) as usize));
}
pub(super) open spec fn filtered(p:Seq<u8>,base:usize,step:usize,simple:bool,edge:bool,limit:u8,inner:u8,threshold:u8) -> Seq<u8> {
    let e=samples(p,base,step);
    scatter(p,base,step,Seq::new(8,|k:int|kernel::sample(e,simple,edge,limit,inner,threshold,k as usize)),8)
}
pub(super) fn filter(p:&mut[u8],base:usize,step:usize,simple:bool,edge:bool,limit:u8,inner:u8,threshold:u8)
    requires 0<step<=16384,base as int+7*step<old(p).len(),old(p).len()<=268435456,
    ensures final(p).len()==old(p).len(),final(p)@==filtered(old(p)@,base,step,simple,edge,limit,inner,threshold),
{
    let mut e=[0u8;8];let mut i=0usize;
    while i<8
        invariant i<=8,0<step<=16384,base as int+7*step<p.len(),p.len()<=268435456,
            forall|j:int|0<=j<i ==> #[trigger] e@[j]==p@[base+j*step],
        decreases 8-i,
    {
        assert(base+i*step<p.len()) by(nonlinear_arith) requires i<8,step>0,base+7*step<p.len();
        e[i]=p[base+i*step];i=i+1;
    }
    assert(e@ =~= samples(p@,base,step));let out=kernel::filter(&e,simple,edge,limit,inner,threshold);
    assert(out@ =~= Seq::new(8,|k:int|kernel::sample(e@,simple,edge,limit,inner,threshold,k as usize)));
    let ghost before=p@;let ghost goal=filtered(p@,base,step,simple,edge,limit,inner,threshold);let mut i=0usize;
    proof{reveal(scatter);assert(p@ =~= scatter(before,base,step,out@,0));}
    while i<8
        invariant i<=8,0<step<=16384,base as int+7*step<p.len(),p.len()<=268435456,p.len()==before.len(),before==old(p)@,
            p@==scatter(before,base,step,out@,i),goal==filtered(before,base,step,simple,edge,limit,inner,threshold),
            goal==scatter(before,base,step,out@,8),
        decreases 8-i,
    {
        assert(base+i*step<p.len()) by(nonlinear_arith) requires i<8,step>0,base+7*step<p.len();
        proof{next(before,base,step,out@,i);}
        p[base+i*step]=out[i];i=i+1;
    }
}
}
