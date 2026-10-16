// New-style JPEGTables and legacy OJPEG table/header reconstruction.
use super::*;
use crate::jpeg_header as jh;
use crate::jpeg_format::Format;
use crate::jpeg_huffman as hf;
use crate::jpeg as j;
verus! {
pub(super) open spec fn header_model(s:Seq<u8>,off:int,count:int,v:Seq<usize>) -> Option<jh::Model> {
    if !range_ok(s.len() as int,off,count) || off+count>usize::MAX || 2*count+1>usize::MAX {None} else {
        let m=j::start(s,off,off+count,v);if m.is_none() {None} else {jh::finish(s,m.unwrap(),(2*count+1) as nat)}
    }
}
pub(super) fn read_header(s:&[u8],off:usize,count:usize,v:[usize;32]) -> (r:Result<jh::Header,Error>)
    requires jh::shape(v@),jh::tables_ok(s@,v@),
    ensures r.is_ok()==header_model(s@,off as int,count as int,v@).is_some(),
        r.is_ok() ==> r.unwrap().valid(s@) && r.unwrap().model()==header_model(s@,off as int,count as int,v@).unwrap(),
{
    if !checked_range(s.len(),off,count) {return Err(Error::Truncated);}
    let end=add(off,count)?;let fuel=add(mul(count,2)?,1)?;j::header(s,off,end,v,fuel)
}
pub(super) open spec fn pointer_model(s:Seq<u8>,t:Tag,c:int,n:int,le:bool) -> Option<usize> {
    if t.count!=n || !value_valid(s,t,c) || tag_value(s,t,c,le)==0 {None} else {Some(tag_value(s,t,c,le) as usize)}
}
fn pointer(s:&[u8],t:Tag,c:usize,n:usize,le:bool) -> (r:Result<usize,Error>)
    ensures r.is_ok()==pointer_model(s@,t,c as int,n as int,le).is_some(),r.is_ok() ==> r.unwrap()==pointer_model(s@,t,c as int,n as int,le).unwrap(),
{
    if t.count!=n {return Err(Error::Invalid);}let v=value(s,t,c,le)?;if v==0 {return Err(Error::Invalid);}Ok(v)
}
#[verifier::opaque]
pub(super) open spec fn component_model(s:Seq<u8>,f:Format,c:int,v:Seq<usize>) -> Option<Seq<usize>> {
    let q=pointer_model(s,f.q,c,f.ch as int,f.le);let dc=pointer_model(s,f.dc,c,f.ch as int,f.le);let ac=pointer_model(s,f.ac,c,f.ch as int,f.le);
    if q.is_none() || dc.is_none() || ac.is_none() {None} else {
        let a=q.unwrap();let b=dc.unwrap();let d=ac.unwrap();
        if !jh::quant_ok(s,a as int,s.len() as int) || !hf::table_ok(s,b as int,s.len() as int) || !hf::table_ok(s,d as int,s.len() as int) {None}
        else {Some(v.update(c,a).update(4+c,b).update(8+c,d))}
    }
}
fn legacy_component(s:&[u8],f:Format,c:usize,v:[usize;32]) -> (r:Result<[usize;32],Error>)
    requires c<3,jh::shape(v@),jh::tables_ok(s@,v@),
    ensures r.is_ok()==component_model(s@,f,c as int,v@).is_some(),
        r.is_ok() ==> r.unwrap()@==component_model(s@,f,c as int,v@).unwrap() && jh::shape(r.unwrap()@) && jh::tables_ok(s@,r.unwrap()@),
{
    reveal(component_model);
    let q=pointer(s,f.q,c,f.ch,f.le)?;let dc=pointer(s,f.dc,c,f.ch,f.le)?;let ac=pointer(s,f.ac,c,f.ch,f.le)?;
    jh::quant(s,q,s.len())?;hf::table(s,dc,s.len())?;hf::table(s,ac,s.len())?;
    let mut out=v;out[c]=q;out[4+c]=dc;out[8+c]=ac;Ok(out)
}
pub(super) open spec fn config(f:Format,v:Seq<usize>) -> Seq<usize> {
    v.update(12,f.bw).update(13,f.bh).update(14,f.ch).update(15,f.hs).update(16,f.vs)
        .update(17,0).update(18,1).update(19,2).update(20,0).update(21,1).update(22,2)
        .update(23,0).update(24,1).update(25,2).update(26,0).update(27,1).update(28,2).update(29,f.restart).update(30,1).update(31,1)
}
fn configure(f:Format,v:[usize;32]) -> (r:[usize;32])
    requires f.valid(),
    ensures r@==config(f,v@),jh::shape(r@),jh::frame_ok(r@),
{
    let mut a=v;a[12]=f.bw;a[13]=f.bh;a[14]=f.ch;a[15]=f.hs;a[16]=f.vs;
    a[17]=0;a[18]=1;a[19]=2;a[20]=0;a[21]=1;a[22]=2;a[23]=0;a[24]=1;a[25]=2;a[26]=0;a[27]=1;a[28]=2;a[29]=f.restart;a[30]=1;a[31]=1;a
}
pub(super) open spec fn first_scan_model(s:Seq<u8>,f:Format,v:Seq<usize>) -> Option<Seq<usize>> {
    if v[31]==1 {Some(v)} else if !value_valid(s,f.offsets,0) || !value_valid(s,f.counts,0) {None} else {
        let p=tag_value(s,f.offsets,0,f.le);let n=tag_value(s,f.counts,0,f.le);
        if !range_ok(s.len() as int,p,n) || p+n>usize::MAX || 2*n+1>usize::MAX || n<2 || s[p]!=255 || s[p+1]!=218 {None} else {
            let m=jh::Model {v:v.update(31,0),at:p as usize,end:(p+n) as usize,sub:(p+n) as usize,kind:0};
            let a=jh::finish(s,m,(2*n+1) as nat);if a.is_none() || a.unwrap().v[31]!=1 {None} else {Some(a.unwrap().v)}
        }
    }
}
fn first_scan(s:&[u8],f:Format,v:[usize;32]) -> (r:Result<[usize;32],Error>)
    requires jh::shape(v@),jh::tables_ok(s@,v@),v@[31]==1 ==> jh::scan_ok(s@,v@),
    ensures r.is_ok()==first_scan_model(s@,f,v@).is_some(),r.is_ok() ==> r.unwrap()@==first_scan_model(s@,f,v@).unwrap() && jh::shape(r.unwrap()@) && jh::scan_ok(s@,r.unwrap()@),
        r.is_ok() ==> r.unwrap()@[14]==v@[14] && r.unwrap()@[15]==v@[15] && r.unwrap()@[16]==v@[16],
{
    if v[31]==1 {return Ok(v);}
    let p=value(s,f.offsets,0,f.le)?;let n=value(s,f.counts,0,f.le)?;
    if !checked_range(s.len(),p,n) || n<2 || s[p]!=255 || s[p+1]!=218 {return Err(Error::Invalid);}
    let end=add(p,n)?;let fuel=add(mul(n,2)?,1)?;let mut a=v;a[31]=0;
    let h=jh::parse(s,jh::Header {v:a,at:p,end,sub:end,kind:0},fuel)?;
    if h.v[31]!=1 {return Err(Error::Invalid);}
    reveal_with_fuel(jh::finish,2);reveal(jh::step);reveal(jh::scan);
    Ok(h.v)
}
#[verifier::opaque]
pub(super) open spec fn base_model(s:Seq<u8>,f:Format) -> Option<Seq<usize>> {
    let zero=Seq::new(32,|i:int| 0usize);
    if f.compression==7 {
        if f.shared.kind==0 {Some(zero)} else if f.shared.kind!=7 {None} else {
            let a=header_model(s,f.shared.data as int,f.shared.count as int,zero);
            if a.is_none() || a.unwrap().v[31]!=2 || a.unwrap().v[30]!=0 {None} else {Some(a.unwrap().v.update(31,0))}
        }
    } else if f.shared.kind!=0 {None}
    else if f.interchange>0 {
        let a=header_model(s,f.interchange as int,f.length as int,zero);
        if a.is_none() {None} else {let v=a.unwrap().v;
            if v[30]!=1 || v[12]!=f.bw || v[13]!=f.h || v[14]!=f.ch || v[15]!=f.hs || v[16]!=f.vs {None}
            else {first_scan_model(s,f,v)}
        }
    } else if f.length!=0 {None}
    else {
        let a=component_model(s,f,0,zero);
        if a.is_none() {None} else {
            let b=if f.ch==1 {a} else {let b=component_model(s,f,1,a.unwrap());if b.is_none() {None} else {component_model(s,f,2,b.unwrap())}};
            if b.is_none() {None} else {Some(config(f,b.unwrap()))}
        }
    }
}
pub(super) fn base(s:&[u8],f:Format) -> (r:Result<[usize;32],Error>)
    requires f.valid(),
    ensures r.is_ok()==base_model(s@,f).is_some(),
        r.is_ok() ==> r.unwrap()@==base_model(s@,f).unwrap() && jh::shape(r.unwrap()@) && jh::tables_ok(s@,r.unwrap()@),
        r.is_ok() && f.compression==6 ==> jh::scan_ok(s@,r.unwrap()@) && r.unwrap()@[14]==f.ch && r.unwrap()@[15]==f.hs && r.unwrap()@[16]==f.vs,
{
    reveal(base_model);
    let zero=j::empty();assert(zero@ =~= Seq::new(32,|i:int| 0usize));
    assert(jh::shape(zero@) && jh::tables_ok(s@,zero@));
    if f.compression==7 {
        if f.shared.kind==0 {return Ok(zero);}if f.shared.kind!=7 {return Err(Error::Invalid);}
        let h=read_header(s,f.shared.data,f.shared.count,zero)?;
        if h.v[31]!=2 || h.v[30]!=0 {return Err(Error::Invalid);}let mut v=h.v;v[31]=0;Ok(v)
    } else {
        if f.shared.kind!=0 {return Err(Error::Invalid);}
        if f.interchange>0 {
            let h=read_header(s,f.interchange,f.length,zero)?;
            if h.v[30]!=1 || h.v[12]!=f.bw || h.v[13]!=f.h || h.v[14]!=f.ch || h.v[15]!=f.hs || h.v[16]!=f.vs {return Err(Error::Unsupported);}
            first_scan(s,f,h.v)
        } else {
            if f.length!=0 {return Err(Error::Invalid);}
            let mut v=legacy_component(s,f,0,zero)?;
            if f.ch==3 {v=legacy_component(s,f,1,v)?;v=legacy_component(s,f,2,v)?;}
            let out=configure(f,v);reveal(component_model);
            assert(jh::scan_ok(s@,out@));Ok(out)
        }
    }
}
}
