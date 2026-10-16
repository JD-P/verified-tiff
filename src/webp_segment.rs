use super::*;
use crate::webp_format::Format;
use crate::webp_container::{self as wc,Config};
use crate::webp as codec;
verus! {
pub(super) open spec fn rows(f:Format,k:int) -> int {
    if f.tiled {f.bh as int} else if f.h as int-k*f.bh<f.bh {f.h as int-k*f.bh} else {f.bh as int}
}
pub(super) fn height(f:Format,k:usize) -> (r:usize)
    requires f.valid(),k<f.segments,ensures r as int==rows(f,k as int),1<=r<=f.bh,
{
    if f.tiled {f.bh} else {
        assert((f.w as int-1)/(f.w as int)==0) by(nonlinear_arith) requires f.w>0;
        assert(f.across==1);
        assert(f.segments==(f.h as int-1)/(f.bh as int)+1) by(nonlinear_arith)
            requires f.across==1,f.segments==f.across as int*((f.h as int-1)/(f.bh as int)+1);
        assert(k as int*f.bh<f.h) by(nonlinear_arith) requires f.bh>0,k<(f.h as int-1)/(f.bh as int)+1;
        let remaining=f.h-k*f.bh;if remaining<f.bh {remaining} else {f.bh}
    }
}
#[derive(Clone,Copy)]
pub(super) struct Plan {pub config:Config,pub pixels:usize,pub bytes:usize,pub work:usize}
impl Plan {
    pub(super) open spec fn valid(self,s:Seq<u8>,f:Format,k:int,l:Limits) -> bool {
        self.config.valid(s) && self.config.w==f.bw && self.config.h==rows(f,k) &&
        self.pixels==4*(f.bw as int)*rows(f,k) && self.pixels<=l.max_output && self.bytes==f.bw as int*rows(f,k)*f.ch && self.bytes<=self.pixels
    }
}
pub(super) open spec fn fee(count:int,pixels:int) -> int {64*count+3000*pixels+1000000}
#[verifier::opaque]
pub(super) open spec fn plan(s:Seq<u8>,f:Format,k:int,l:Limits) -> Option<Plan> {
    let off=tag_value(s,f.offsets,k,f.le);let count=tag_value(s,f.counts,k,f.le);let h=rows(f,k);let pixels=4*(f.bw as int)*h;
    let c=wc::container(s,off,count,f.bw as int,h);
    if !value_valid(s,f.offsets,k) || !value_valid(s,f.counts,k) || c.is_none() || pixels>l.max_output || fee(count,pixels)>usize::MAX {None}
    else {Some(Plan {config:c.unwrap(),pixels:pixels as usize,bytes:(f.bw as int*h*f.ch) as usize,work:fee(count,pixels) as usize})}
}
pub(super) fn read(s:&[u8],f:Format,k:usize,l:&Limits) -> (r:Result<Plan,Error>)
    requires f.valid(),k<f.segments,
    ensures r.is_ok()==plan(s@,f,k as int,*l).is_some(),
        r.is_ok() ==> r.unwrap()==plan(s@,f,k as int,*l).unwrap() && r.unwrap().valid(s@,f,k as int,*l),
        r.is_ok() ==> r.unwrap().work as int<=64*s.len()+3000*(l.max_output as int)+1000000,
{
    reveal(plan);
    let off=value(s,f.offsets,k,f.le)?;let count=value(s,f.counts,k,f.le)?;let rows=height(f,k);
    let config=wc::open(s,off,count,f.bw,rows)?;
    assert(4*(f.bw as int)*rows<=1073741824) by(nonlinear_arith) requires f.bw<=16384,rows<=16384;
    assert(0<=f.bw as int*rows<=4*(f.bw as int)*rows && 0<=f.bw as int*rows*f.ch<=4*(f.bw as int)*rows) by(nonlinear_arith) requires f.bw>=1,rows>=1,f.ch<=4;
    let pixels=4*f.bw*rows;let bytes=f.bw*rows*f.ch;
    if pixels>l.max_output {return Err(Error::Limit);}
    let work=add(add(mul(count,64)?,mul(pixels,3000)?)?,1000000)?;
    proof {reveal(wc::container);}
    Ok(Plan {config,pixels,bytes,work})
}
pub(super) open spec fn accepted(s:Seq<u8>,f:Format,k:int,l:Limits) -> bool {
    let p=plan(s,f,k,l);p.is_some() && codec::result(s,p.unwrap().config).is_some()
}
pub(super) fn validate(s:&[u8],f:Format,k:usize,l:&Limits,budget:&mut Budget) -> (r:Result<Plan,Error>)
    requires f.valid(),k<f.segments,s.len()<=l.max_input,
    ensures final(budget).remaining<=old(budget).remaining,
        old(budget).remaining as int-final(budget).remaining<=64*(l.max_input as int)+3000*(l.max_output as int)+1000000,
        accepted(s@,f,k as int,*l) && old(budget).remaining as int>=64*(l.max_input as int)+3000*(l.max_output as int)+1000000 ==> r.is_ok(),
        r.is_ok() ==> accepted(s@,f,k as int,*l) && r.unwrap()==plan(s@,f,k as int,*l).unwrap() && r.unwrap().valid(s@,f,k as int,*l),
{
    let p=read(s,f,k,l)?;spend(budget,p.work)?;let mut empty=[0u8;0];
    codec::process(s,p.config,&mut empty,false)?;Ok(p)
}
pub(super) open spec fn mapped(v:Seq<u8>,ch:int,alpha:int,i:int) -> u8 {
    let p=i/ch;let c=i%ch;
    if c==3 {v[4*p+3]} else {straight_color(v[4*p+c],v[4*p+3],alpha==1,false)}
}
fn convert(out:&mut [u8],count:usize,ch:usize,alpha:usize)
    requires (ch==3 || ch==4),4*(count as int)<=old(out).len(),
    ensures final(out).len()==old(out).len(),
        forall|i:int| 0<=i<count as int*ch ==> final(out)@[i]==mapped(old(out)@,ch as int,alpha as int,i),
        forall|i:int| count as int*ch<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
{
    assert(count as int*ch<=4*(count as int)) by(nonlinear_arith) requires count>=0,ch<=4;
    let bytes=count*ch;let mut i=0usize;
    while i<bytes
        invariant i<=bytes,bytes as int==count as int*ch,ch==3 || ch==4,4*(count as int)<=out.len(),out.len()==old(out).len(),
            forall|k:int| 0<=k<i ==> out@[k]==mapped(old(out)@,ch as int,alpha as int,k),
            forall|k:int| i<=k<out.len() ==> out@[k]==old(out)@[k],
        decreases bytes-i,
    {
        let p=i/ch;let c=i%ch;
        assert(i as int==p as int*ch+c && c<ch) by(nonlinear_arith) requires p==i/ch,c==i%ch,ch>0;
        assert(p<count && i<=4*(p as int)+c<4*(count as int) && i<=4*(p as int)+3<4*(count as int)) by(nonlinear_arith)
            requires i as int==p as int*ch+c,0<=c<ch,ch==3 || ch==4,i<count as int*ch;
        let source=4*p+c;let a=out[4*p+3];let v=out[source];
        let value=if c==3 {v} else {convert_color(v,a,alpha==1,false)};
        out[i]=value;i=i+1;
    }
}
pub(super) open spec fn byte(s:Seq<u8>,f:Format,k:int,l:Limits,i:int) -> u8 {
    let p=plan(s,f,k,l).unwrap();let n=i/(f.ch as int);let c=i%(f.ch as int);
    let v=codec::transformed(s,p.config,4*n+c);let a=codec::transformed(s,p.config,4*n+3);
    if c==3 {v} else {straight_color(v,a,f.alpha==1,false)}
}
pub(super) fn decode(s:&[u8],f:Format,k:usize,l:&Limits,out:&mut [u8]) -> (r:Result<(),Error>)
    requires f.valid(),k<f.segments,accepted(s@,f,k as int,*l),old(out).len()>=plan(s@,f,k as int,*l).unwrap().pixels,
    ensures r.is_ok(),final(out).len()==old(out).len(),
        forall|i:int| 0<=i<plan(s@,f,k as int,*l).unwrap().bytes ==> final(out)@[i]==byte(s@,f,k as int,*l,i),
        forall|i:int| plan(s@,f,k as int,*l).unwrap().pixels<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
{
    let p=read(s,f,k,l)?;codec::process(s,p.config,out,true)?;
    let ghost raw=out@;
    assert(p.pixels==4*(p.config.w as int)*p.config.h);
    codec::inverse(p.config,out);
    assert forall|i:int| 0<=i<p.pixels implies out@[i]==codec::transformed(s@,p.config,i) by {
        assert((i-i%4+1)/4==i/4 && (i-i%4+1)%4==1) by(nonlinear_arith) requires i>=0;
        let area=p.config.w as int*p.config.h;
        assert(p.pixels==4*area) by(nonlinear_arith) requires p.pixels==4*(p.config.w as int)*p.config.h,area==p.config.w as int*p.config.h;
        vstd::arithmetic::div_mod::lemma_mod_multiples_basic(area,4);
        assert(0<=i-i%4+1<p.pixels) by(nonlinear_arith) requires 0<=i<p.pixels,p.pixels%4==0;
        assert(raw[i]==codec::flat(codec::result(s@,p.config).unwrap().pixels,i));
        assert(raw[i-i%4+1]==codec::flat(codec::result(s@,p.config).unwrap().pixels,i-i%4+1));
    }
    let ghost transformed=out@;
    let count=p.pixels/4;
    assert(count as int==f.bw as int*rows(f,k as int)) by(nonlinear_arith) requires p.pixels==4*(f.bw as int)*rows(f,k as int),count==p.pixels/4;
    assert(p.pixels==4*(count as int) && p.bytes==count as int*f.ch) by(nonlinear_arith) requires count as int==f.bw as int*rows(f,k as int),p.pixels==4*(f.bw as int)*rows(f,k as int),p.bytes==f.bw as int*rows(f,k as int)*f.ch;
    convert(out,count,f.ch,f.alpha);
    assert forall|i:int| 0<=i<p.bytes implies out@[i]==byte(s@,f,k as int,*l,i) by {
        let n=i/(f.ch as int);let c=i%(f.ch as int);
        assert(0<=4*n+c<p.pixels && 0<=4*n+3<p.pixels) by(nonlinear_arith)
            requires f.ch>=1,f.ch<=4,0<=i<count as int*f.ch,n==i/(f.ch as int),c==i%(f.ch as int),p.pixels==4*(count as int);
        assert(transformed[4*n+c]==codec::transformed(s@,p.config,4*n+c));assert(transformed[4*n+3]==codec::transformed(s@,p.config,4*n+3));
    }
    Ok(())
}
}
