use super::*;
use crate::jpeg_header as jh;
use crate::jpeg_container as jc;
use crate::jpeg_format::Format;
use crate::jpeg_bits::Bits;
verus! {
pub(super) open spec fn rows(f:Format,k:int) -> int {
    if f.tiled {f.bh as int} else if f.h as int-k*f.bh<f.bh {f.h as int-k*f.bh} else {f.bh as int}
}
pub(super) fn height(f:Format,k:usize) -> (r:usize)
    requires f.valid(),k<f.segments,ensures r as int==rows(f,k as int),1<=r<=65535,
{
    if f.tiled {f.bh} else {
        assert((f.w as int-1)/(f.w as int)==0) by(nonlinear_arith) requires f.w>0;
        assert(f.across==1 && f.segments==(f.h as int-1)/(f.bh as int)+1);
        assert(k as int*f.bh<f.h) by(nonlinear_arith) requires f.bh>0,k<(f.h as int-1)/(f.bh as int)+1;
        let remaining=f.h-k*f.bh;if remaining<f.bh {remaining} else {f.bh}
    }
}
pub(super) open spec fn base_ready(s:Seq<u8>,f:Format,v:Seq<usize>) -> bool {
    jh::shape(v) && jh::tables_ok(s,v) && (f.compression==6 ==> jh::scan_ok(s,v) && v[14]==f.ch && v[15]==f.hs && v[16]==f.vs)
}
pub(super) open spec fn adjusted(f:Format,v:Seq<usize>,k:int) -> Seq<usize> {
    if f.compression==7 {v.update(30,0).update(31,0)} else {v.update(12,f.bw).update(13,rows(f,k) as usize).update(31,0)}
}
#[verifier::opaque]
pub(super) open spec fn header_model(s:Seq<u8>,f:Format,base:Seq<usize>,k:int) -> Option<jh::Model> {
    if !value_valid(s,f.offsets,k) || !value_valid(s,f.counts,k) {None} else {
        let p=tag_value(s,f.offsets,k,f.le);let n=tag_value(s,f.counts,k,f.le);let v=adjusted(f,base,k);
        if !range_ok(s.len() as int,p,n) || p+n>usize::MAX || 2*n+1>usize::MAX || n==0 {None}
        else if f.compression==7 || (n>=2 && s[p]==255 && s[p+1]==216) {jc::header_model(s,p,n,v.update(30,0))}
        else if n>=2 && s[p]==255 && s[p+1]==218 {
            jh::finish(s,jh::Model {v,at:p as usize,end:(p+n) as usize,sub:(p+n) as usize,kind:0},(2*n+1) as nat)
        } else {Some(jh::Model {v:v.update(31,1),at:p as usize,end:(p+n) as usize,sub:(p+n) as usize,kind:0})}
    }
}
pub(super) struct Model {pub v:Seq<usize>,pub bits:Bits}
#[derive(Clone,Copy)]
pub(super) struct Plan {pub v:[usize;32],pub bits:Bits}
impl Plan {
    pub(super) open spec fn model(self) -> Model {Model {v:self.v@,bits:self.bits}}
    pub(super) open spec fn valid(self,s:Seq<u8>,f:Format,k:int) -> bool {
        jh::scan_ok(s,self.v@) && self.bits.valid(s) && self.v@[12]==f.bw && self.v@[13]==rows(f,k) && self.v@[14]==f.ch && self.v@[15]==f.hs && self.v@[16]==f.vs
    }
}
#[verifier::opaque]
pub(super) open spec fn plan(s:Seq<u8>,f:Format,base:Seq<usize>,k:int) -> Option<Model> {
    let a=header_model(s,f,base,k);if a.is_none() {None} else {
        let h=a.unwrap();let v=h.v;
        if v[31]!=1 || v[12]!=f.bw || v[13]!=rows(f,k) || v[14]!=f.ch || v[15]!=f.hs || v[16]!=f.vs {None}
        else {Some(Model {v,bits:Bits {at:h.at,end:h.end,byte:0,left:0}})}
    }
}
pub(super) fn read(s:&[u8],f:Format,base:[usize;32],k:usize) -> (r:Result<Plan,Error>)
    requires f.valid(),k<f.segments,base_ready(s@,f,base@),
    ensures r.is_ok()==plan(s@,f,base@,k as int).is_some(),
        r.is_ok() ==> r.unwrap().valid(s@,f,k as int) && r.unwrap().model()==plan(s@,f,base@,k as int).unwrap(),
{
    reveal(plan);reveal(header_model);
    let p=value(s,f.offsets,k,f.le)?;let n=value(s,f.counts,k,f.le)?;
    if !checked_range(s.len(),p,n) || n==0 {return Err(Error::Truncated);}
    let end=add(p,n)?;let fuel=add(mul(n,2)?,1)?;
    let ht=height(f,k);let mut v=base;
    if f.compression==7 {v[30]=0;v[31]=0;} else {v[12]=f.bw;v[13]=ht;v[31]=0;}
    assert(v@==adjusted(f,base@,k as int));
    let h=if f.compression==7 || (n>=2 && s[p]==255 && s[p+1]==216) {
        v[30]=0;jc::read_header(s,p,n,v)?
    } else if n>=2 && s[p]==255 && s[p+1]==218 {
        jh::parse(s,jh::Header {v,at:p,end,sub:end,kind:0},fuel)?
    } else {
        v[31]=1;jh::Header {v,at:p,end,sub:end,kind:0}
    };
    if h.v[31]!=1 || h.v[12]!=f.bw || h.v[13]!=ht || h.v[14]!=f.ch || h.v[15]!=f.hs || h.v[16]!=f.vs {return Err(Error::Unsupported);}
    Ok(Plan {v:h.v,bits:Bits {at:h.at,end:h.end,byte:0,left:0}})
}
}
