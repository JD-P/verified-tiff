use super::*;
use crate::logluv_format::Format;
use crate::logluv as codec;
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
pub(super) struct Plan {pub off:usize,pub count:usize,pub raw:usize,pub pixels:usize,pub bytes:usize,pub work:usize}
impl Plan {
    pub(super) open spec fn valid(self,s:Seq<u8>,f:Format,k:int,l:Limits) -> bool {
        range_ok(s.len() as int,self.off as int,self.count as int) &&
        self.raw==4*(f.bw as int)*rows(f,k) && self.pixels==10*(f.bw as int)*rows(f,k) &&
        self.pixels<=l.max_output && self.bytes==6*(f.bw as int)*rows(f,k) && self.raw as int+self.bytes==self.pixels
    }
}
pub(super) open spec fn fee(count:int,pixels:int) -> int {count+2*pixels+1}
#[verifier::opaque]
pub(super) open spec fn plan(s:Seq<u8>,f:Format,k:int,l:Limits) -> Option<Plan> {
    let off=tag_value(s,f.offsets,k,f.le);let count=tag_value(s,f.counts,k,f.le);let h=rows(f,k);let area=f.bw as int*h;
    if !value_valid(s,f.offsets,k) || !value_valid(s,f.counts,k) || !range_ok(s.len() as int,off,count)
        || 10*area>l.max_output || fee(count,10*area)>usize::MAX {None}
    else {Some(Plan {off:off as usize,count:count as usize,raw:(4*area) as usize,pixels:(10*area) as usize,
        bytes:(6*area) as usize,work:fee(count,10*area) as usize})}
}
pub(super) fn read(s:&[u8],f:Format,k:usize,l:&Limits) -> (r:Result<Plan,Error>)
    requires f.valid(),k<f.segments,
    ensures r.is_ok()==plan(s@,f,k as int,*l).is_some(),
        r.is_ok() ==> r.unwrap()==plan(s@,f,k as int,*l).unwrap() && r.unwrap().valid(s@,f,k as int,*l),
        r.is_ok() ==> r.unwrap().work as int<=s.len()+2*(l.max_output as int)+1,
{
    reveal(plan);
    let off=value(s,f.offsets,k,f.le)?;let count=value(s,f.counts,k,f.le)?;let rows=height(f,k);
    if !checked_range(s.len(),off,count) {return Err(Error::Truncated);}
    assert(0<f.bw as int*rows<=268435456) by(nonlinear_arith) requires 1<=f.bw<=16384,1<=rows<=16384;
    let area=f.bw*rows;let raw=4*area;let bytes=6*area;let pixels=10*area;
    if pixels>l.max_output {return Err(Error::Limit);}
    let work=add(add(count,mul(pixels,2)?)?,1)?;
    assert(raw as int==4*(f.bw as int)*rows && bytes as int==6*(f.bw as int)*rows && pixels as int==10*(f.bw as int)*rows && raw as int+bytes==pixels) by(nonlinear_arith)
        requires area as int==f.bw as int*rows,raw==4*area,bytes==6*area,pixels==10*area;
    Ok(Plan {off,count,raw,bytes,pixels,work})
}
pub(super) open spec fn accepted(s:Seq<u8>,f:Format,k:int,l:Limits) -> bool {
    let p=plan(s,f,k,l);p.is_some() && codec::sg_valid(s,p.unwrap().off as int,p.unwrap().off as int+p.unwrap().count,f.bw as int,p.unwrap().raw as int)
}
pub(super) fn validate(s:&[u8],f:Format,k:usize,l:&Limits,budget:&mut Budget) -> (r:Result<Plan,Error>)
    requires f.valid(),k<f.segments,s.len()<=l.max_input,
    ensures final(budget).remaining<=old(budget).remaining,
        old(budget).remaining as int-final(budget).remaining<=l.max_input as int+2*(l.max_output as int)+1,
        accepted(s@,f,k as int,*l) && old(budget).remaining as int>=l.max_input as int+2*(l.max_output as int)+1 ==> r.is_ok(),
        r.is_ok() ==> accepted(s@,f,k as int,*l) && r.unwrap()==plan(s@,f,k as int,*l).unwrap() && r.unwrap().valid(s@,f,k as int,*l),
{
    let p=read(s,f,k,l)?;spend(budget,p.work)?;let mut local=Budget {remaining:p.count};
    codec::sg_validate(s,p.off,p.count,f.bw,p.raw,&mut local)?;Ok(p)
}
pub(super) open spec fn uv(v:u8) -> int {((2*(v as int)+1)*8192)/205}
fn expand_uv(v:u8) -> (r:u16) ensures r as int==uv(v),r<=20420, {
    (((2*(v as u32)+1)*8192)/205) as u16
}
pub(super) open spec fn source(w:int,i:int) -> int {
    let p=i/6;let c=i%6;
    (p/w)*(4*w)+(if c<2 {1-c} else {1+c/2})*w+p%w
}
pub(super) open spec fn native(raw:Seq<u8>,w:int,i:int) -> u8 {
    let v=raw[source(w,i)];let c=i%6;
    if c<2 {v} else if c%2==0 {(uv(v)%256) as u8} else {(uv(v)/256) as u8}
}
proof fn source_bounds(w:int,h:int,i:int)
    requires w>0,h>0,0<=i<6*w*h,
    ensures 0<=source(w,i)<4*w*h,
{
    let p=i/6;let c=i%6;quotient_remainder(i,6);
    assert(0<=p<w*h) by(nonlinear_arith) requires i==p*6+c,0<=c<6,0<=i<6*w*h;
    quotient_remainder(p,w);
    assert(0<=p/w<h) by(nonlinear_arith) requires p==w*(p/w)+p%w,0<=p<w*h,w>0,0<=p%w<w;
    assert(0<=source(w,i)<4*w*h) by(nonlinear_arith)
        requires source(w,i)==(p/w)*(4*w)+(if c<2 {1-c} else {1+c/2})*w+p%w,
            0<=c<6,0<=p/w<h,0<=p%w<w,w>0;
}
fn convert(raw:&[u8],w:usize,h:usize,out:&mut [u8])
    requires w>0,h>0,raw.len()==4*(w as int)*h,old(out).len()==6*(w as int)*h,
    ensures final(out).len()==old(out).len(),
        forall|i:int| 0<=i<final(out).len() ==> final(out)@[i]==native(raw@,w as int,i),
{
    assert(4*(w as int)<=raw.len()) by(nonlinear_arith) requires h>=1,raw.len()==4*(w as int)*h,w>=1;
    let mut i=0usize;
    while i<out.len()
        invariant 4*(w as int)<=raw.len(),w>0,h>0,raw.len()==4*(w as int)*h,out.len()==6*(w as int)*h,out.len()==old(out).len(),i<=out.len(),
            forall|k:int| 0<=k<i ==> out@[k]==native(raw@,w as int,k),
        decreases out.len()-i,
    {
        proof {source_bounds(w as int,h as int,i as int);}
        let p=i/6;let c=i%6;let plane=if c<2 {1-c} else {1+c/2};
        let at=(p/w)*(4*w)+plane*w+p%w;
        let v=raw[at];let u=expand_uv(v);
        out[i]=if c<2 {v} else if c%2==0 {(u%256) as u8} else {(u/256) as u8};i=i+1;
    }
}
pub(super) open spec fn raw_bytes(s:Seq<u8>,p:Plan) -> Seq<u8> {
    Seq::new(p.raw as nat,|i:int| codec::sg_byte(s,p.off as int,p.off as int+p.count,i))
}
pub(super) open spec fn byte(s:Seq<u8>,f:Format,k:int,l:Limits,i:int) -> u8 {
    native(raw_bytes(s,plan(s,f,k,l).unwrap()),f.bw as int,i)
}
pub(super) fn decode(s:&[u8],f:Format,k:usize,l:&Limits,out:&mut [u8]) -> (r:Result<(),Error>)
    requires f.valid(),k<f.segments,accepted(s@,f,k as int,*l),old(out).len()>=plan(s@,f,k as int,*l).unwrap().pixels,
    ensures r.is_ok(),final(out).len()==old(out).len(),
        forall|i:int| 0<=i<plan(s@,f,k as int,*l).unwrap().bytes ==> final(out)@[i]==byte(s@,f,k as int,*l,i),
        forall|i:int| plan(s@,f,k as int,*l).unwrap().pixels<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
{
    let p=read(s,f,k,l)?;let h=height(f,k);
    let (all,_tail)=out.split_at_mut(p.pixels);let (result,raw)=all.split_at_mut(p.bytes);
    codec::decode(s,p.off,p.count,f.bw,p.raw,raw);
    assert(raw@ =~= raw_bytes(s@,p));
    convert(raw,f.bw,h,result);
    Ok(())
}
}
