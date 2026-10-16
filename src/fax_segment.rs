use super::*;
use crate::fax_format::Format;
use crate::fax_bits::Bits;
use crate::fax_frame::{self as ff,Config};
use crate::fax as fc;
verus! {
pub(super) open spec fn rows(f:Format,k:int) -> int {
    if f.tiled {f.bh as int} else if f.h as int-k*f.bh<f.bh {f.h as int-k*f.bh} else {f.bh as int}
}
pub(super) fn height(f:Format,k:usize) -> (r:usize)
    requires f.valid(),k<f.segments,ensures r as int==rows(f,k as int),1<=r<=f.bh,
{
    if f.tiled {f.bh} else {
        assert((f.w as int-1)/(f.w as int)==0) by(nonlinear_arith) requires f.w>0;
        assert(f.across==1 && f.segments==(f.h as int-1)/(f.bh as int)+1);
        assert(k as int*f.bh<f.h) by(nonlinear_arith) requires f.bh>0,k<(f.h as int-1)/(f.bh as int)+1;
        let remaining=f.h-k*f.bh;if remaining<f.bh {remaining} else {f.bh}
    }
}
#[derive(Clone,Copy)]
pub(super) struct Plan {pub bits:Bits,pub config:Config,pub rows:usize,pub pixels:usize,pub work:usize}
impl Plan {
    pub(super) open spec fn valid(self,s:Seq<u8>,f:Format,k:int,l:Limits) -> bool {
        self.bits.valid(s) && self.bits.end<usize::MAX && self.rows==rows(f,k) && self.rows>0 &&
        self.pixels==f.bw as int*self.rows && self.pixels<=l.max_output
    }
}
pub(super) open spec fn fee(count:int,pixels:int,rows:int) -> int {64*count+4*pixels+64*rows+1}
#[verifier::opaque]
pub(super) open spec fn plan(s:Seq<u8>,f:Format,k:int,l:Limits) -> Option<Plan> {
    let off=tag_value(s,f.offsets,k,f.le);let count=tag_value(s,f.counts,k,f.le);let h=rows(f,k);let pixels=f.bw as int*h;
    if !value_valid(s,f.offsets,k) || !value_valid(s,f.counts,k) || !range_ok(s.len() as int,off,count) || count==0 ||
        8*(off+count)>=usize::MAX || pixels>l.max_output || fee(count,pixels,h)>usize::MAX {None}
    else {let bits=Bits {pos:(8*off) as usize,end:(8*(off+count)) as usize,lsb:f.lsb};
        let missing=f.compression==3 && !f.mixed && !f.aligned && !ff::zero_bits(s,bits,11);
        Some(Plan {bits,config:Config {group4:f.compression==4,mixed:f.mixed,aligned:f.aligned,missing},rows:h as usize,pixels:pixels as usize,work:fee(count,pixels,h) as usize})
    }
}
pub(super) fn read(s:&[u8],f:Format,k:usize,l:&Limits) -> (r:Result<Plan,Error>)
    requires f.valid(),k<f.segments,
    ensures r.is_ok()==plan(s@,f,k as int,*l).is_some(),
        r.is_ok() ==> r.unwrap()==plan(s@,f,k as int,*l).unwrap() && r.unwrap().valid(s@,f,k as int,*l),
        r.is_ok() ==> r.unwrap().work as int<=64*s.len()+68*(l.max_output as int)+1,
{
    reveal(plan);
    let off=value(s,f.offsets,k,f.le)?;let count=value(s,f.counts,k,f.le)?;
    if !checked_range(s.len(),off,count) || count==0 {return Err(Error::Truncated);}
    let end=mul(add(off,count)?,8)?;if end==usize::MAX {return Err(Error::Limit);}
    assert(off as int*8<=end) by(nonlinear_arith) requires end as int==(off as int+count)*8;
    let start=mul(off,8)?;let rows=height(f,k);let pixels=mul(f.bw,rows)?;
    if pixels>l.max_output {return Err(Error::Limit);}
    let work=add(add(add(mul(count,64)?,mul(pixels,4)?)?,mul(rows,64)?)?,1)?;
    assert(rows<=pixels) by(nonlinear_arith) requires pixels as int==f.bw as int*rows,f.bw>=1;
    assert(work as int<=64*s.len()+68*(l.max_output as int)+1);
    let bits=Bits {pos:start,end,lsb:f.lsb};
    let missing=f.compression==3 && !f.mixed && !f.aligned && !ff::zeros(s,bits,11);
    Ok(Plan {bits,config:Config {group4:f.compression==4,mixed:f.mixed,aligned:f.aligned,missing},rows,pixels,work})
}
pub(super) open spec fn accepted(s:Seq<u8>,f:Format,k:int,l:Limits) -> bool {
    let p=plan(s,f,k,l);p.is_some() && fc::finish(s,p.unwrap().config,f.bw,fc::initial(p.unwrap().bits,f.bw),p.unwrap().rows as nat).is_some()
}
pub(super) fn validate(s:&[u8],f:Format,k:usize,l:&Limits,budget:&mut Budget) -> (r:Result<Plan,Error>)
    requires f.valid(),k<f.segments,s.len()<=l.max_input,
    ensures final(budget).remaining<=old(budget).remaining,
        old(budget).remaining as int-final(budget).remaining<=64*(l.max_input as int)+68*(l.max_output as int)+1,
        accepted(s@,f,k as int,*l) && old(budget).remaining as int>=64*(l.max_input as int)+68*(l.max_output as int)+1 ==> r.is_ok(),
        r.is_ok() ==> accepted(s@,f,k as int,*l) && r.unwrap()==plan(s@,f,k as int,*l).unwrap() && r.unwrap().valid(s@,f,k as int,*l),
{
    let p=read(s,f,k,l)?;spend(budget,p.work)?;let mut empty=[0u8;0];
    fc::process(s,p.config,f.bw,p.rows,p.bits,&mut empty,false)?;Ok(p)
}
pub(super) fn decode(s:&[u8],f:Format,k:usize,l:&Limits,output:&mut [u8]) -> (r:Result<(),Error>)
    requires f.valid(),k<f.segments,accepted(s@,f,k as int,*l),old(output).len()>=plan(s@,f,k as int,*l).unwrap().pixels,
    ensures r.is_ok(),final(output).len()==old(output).len(),
        forall|i:int| 0<=i<plan(s@,f,k as int,*l).unwrap().pixels ==> final(output)@[i]==byte(s@,f,k as int,*l,i),
        forall|i:int| plan(s@,f,k as int,*l).unwrap().pixels<=i<old(output).len() ==> final(output)@[i]==old(output)@[i],
{
    let p=read(s,f,k,l)?;fc::process(s,p.config,f.bw,p.rows,p.bits,output,true)
}
pub(super) open spec fn byte(s:Seq<u8>,f:Format,k:int,l:Limits,i:int) -> u8 {
    let p=plan(s,f,k,l).unwrap();fc::output_byte(s,p.config,f.bw,p.bits,p.rows,i)
}
}
