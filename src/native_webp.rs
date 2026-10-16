//! Native static WebP decoding with caller-owned pixel, transform and entropy
//! storage. Existing whole-image TIFF profiles retain their original scope.
use super::*;
use crate::webp_riff as riff;
use crate::webp_prepare as prep;
use crate::webp_pixels as pix;
use crate::webp_inverse as inverse;
use crate::webp_container::Config;

use crate::webp as wp;
pub use crate::webp_container::Group as EntropyGroup;
pub use crate::webp_alpha::{Header as AlphaHeader,inspect as inspect_alpha,unfilter as unfilter_alpha};
verus! {
#[derive(Clone,Copy,Debug)]
pub struct Container {pub lossless:bool,pub alpha_start:usize,pub alpha_end:usize}
pub closed spec fn container(s:Seq<u8>,w:usize,h:usize) -> Option<Container> {
    let f=riff::frame(s,w,h);if f.is_none(){None}else{Some(Container {lossless:f.unwrap().kind==1,alpha_start:f.unwrap().alpha,alpha_end:f.unwrap().alpha_end})}
}
pub fn inspect(s:&[u8],w:usize,h:usize) -> (res:Result<Container,Error>)
    ensures res.is_ok()==container(s@,w,h).is_some(),res.is_ok() ==> res.unwrap()==container(s@,w,h).unwrap() && res.unwrap().alpha_start<=res.unwrap().alpha_end<=s.len(),
{
    reveal(container);let f=riff::inspect(s,w,h)?;Ok(Container {lossless:f.kind==1,alpha_start:f.alpha,alpha_end:f.alpha_end})
}
pub type Pixel=[u8;4];
pub open spec fn dimensions(w:usize,h:usize) -> bool {1<=w<=16384 && 1<=h<=16384}
pub open spec fn storage(w:usize,h:usize) -> int {3*w*h+256}
pub fn scratch_len(w:usize,h:usize) -> (r:Result<usize,Error>)
    ensures r.is_ok()==dimensions(w,h),r.is_ok() ==> r.unwrap()==storage(w,h),
{
    if w==0 || w>16384 || h==0 || h>16384{return Err(Error::Invalid);}
    assert(w*h<=268435456) by(nonlinear_arith) requires w<=16384,h<=16384;
    let n=w*h;
    assert(3*n==3*w*h) by(nonlinear_arith) requires n==w*h;
    Ok(3*n+256)
}
pub open spec fn preparation_fee(bytes:int,pixels:int) -> int {64*bytes+3000*pixels+4000000}
fn prepare_cost(bytes:usize,w:usize,h:usize) -> (r:Result<usize,Error>)
    requires dimensions(w,h),
    ensures r.is_ok()==(preparation_fee(bytes as int,w as int*h)<=usize::MAX),r.is_ok() ==> r.unwrap()==preparation_fee(bytes as int,w as int*h),
{
    assert(w*h<=268435456) by(nonlinear_arith) requires w<=16384,h<=16384;
    add(add(mul(bytes,64)?,mul(w*h,3000)?)?,4000000)
}
pub open spec fn rendering_fee(pixels:int,groups:int) -> int {3000*pixels+1000000*groups}
fn render_cost(w:usize,h:usize,groups:usize) -> (r:Result<usize,Error>)
    requires dimensions(w,h),
    ensures r.is_ok()==(rendering_fee(w as int*h,groups as int)<=usize::MAX),r.is_ok() ==> r.unwrap()==rendering_fee(w as int*h,groups as int),
{
    assert(w*h<=268435456) by(nonlinear_arith) requires w<=16384,h<=16384;
    add(mul(w*h,3000)?,mul(groups,1000000)?)
}
pub fn work_limit(bytes:usize,w:usize,h:usize,groups:usize) -> (r:Result<usize,Error>)
    ensures r.is_ok()==(dimensions(w,h) && preparation_fee(bytes as int,w as int*h)+rendering_fee(w as int*h,groups as int)<=usize::MAX),
        r.is_ok() ==> r.unwrap()==preparation_fee(bytes as int,w as int*h)+rendering_fee(w as int*h,groups as int),
{
    if w==0 || w>16384 || h==0 || h>16384{return Err(Error::Invalid);}
    add(prepare_cost(bytes,w,h)?,render_cost(w,h,groups)?)
}
pub(super) open spec fn prepared(s:Seq<u8>,w:usize,h:usize) -> Option<prep::Model> {
    let f=riff::frame(s,w,h);
    if f.is_none() || f.unwrap().kind!=1{None}else{
        let b=crate::deflate_bits::Bits {at:f.unwrap().at,bit:0,end:f.unwrap().end};
        prep::prepared(s,b,w,h,storage(w,h) as usize)
    }
}
pub closed spec fn group_requirement(s:Seq<u8>,w:usize,h:usize) -> Option<usize> {
    let p=prepared(s,w,h);if p.is_none(){None}else{Some(p.unwrap().groups)}
}
/// This planning pass decodes only small auxiliary images. It does not allocate
/// entropy tables. The following decode revalidates caller-modifiable scratch.
pub fn required_groups(s:&[u8],w:usize,h:usize,scratch:&mut[Pixel],budget:&mut Budget) -> (r:Result<usize,Error>)
    ensures final(scratch).len()==old(scratch).len(),final(budget).remaining<=old(budget).remaining,
        r.is_ok()==(dimensions(w,h) && old(scratch).len()>=storage(w,h) && group_requirement(s@,w,h).is_some() && preparation_fee(s.len() as int,w as int*h)<=old(budget).remaining),
        r.is_ok() ==> r.unwrap()==group_requirement(s@,w,h).unwrap() && 1<=r.unwrap()<=65536,
        r.is_ok() ==> final(budget).remaining as int+preparation_fee(s.len() as int,w as int*h)==old(budget).remaining,
        forall|i:int|storage(w,h)<=i<old(scratch).len() ==> final(scratch)@[i]==old(scratch)@[i],
{
    reveal(group_requirement);let cap=scratch_len(w,h)?;
    if scratch.len()<cap{return Err(Error::OutputTooSmall);}
    let fee=prepare_cost(s.len(),w,h)?;spend(budget,fee)?;
    let f=riff::inspect(s,w,h)?;if f.kind!=1{return Err(Error::Unsupported);}
    let bits=crate::deflate_bits::Bits {at:f.at,bit:0,end:f.end};let (data,_)=scratch.split_at_mut(cap);
    let p=prep::prepare(s,bits,w,h,data)?;Ok(p.groups)
}
pub closed spec fn decoded(s:Seq<u8>,w:usize,h:usize) -> Option<Seq<Pixel>> {
    let pp=prepared(s,w,h);
    if pp.is_none(){None}else{
        let p=pp.unwrap();let gg=prep::group_finish(s,p.bits,p.cache,Seq::empty(),p.groups as nat);
        if gg.is_none(){None}else{
            let g=gg.unwrap();let c=Config {bits:g.bits,w:p.transforms.width,h,subtract:false,cache:p.cache};
            let meta=p.data.subrange(p.meta_start as int,p.meta_start as int+p.meta_count);
            let d=pix::finish(s,c,g.tables,meta,p.meta_bits,c.w as int*h,wp::initial(g.bits.pos()),(c.w as int*h) as nat);
            if d.is_none() || (d.unwrap().pos+7)/8!=c.bits.end{None}else{
                let initial=d.unwrap().pixels+Seq::new((w as int*h-d.unwrap().pixels.len()) as nat,|i:int|[0u8,0,0,0]);
                inverse::all(p.transforms.transforms,p.data,initial,p.transforms.transforms.len())
            }
        }
    }
}
pub closed spec fn work(s:Seq<u8>,w:usize,h:usize) -> int {
    let p=prepared(s,w,h);if p.is_none(){-1}else{preparation_fee(s.len() as int,w as int*h)+rendering_fee(w as int*h,p.unwrap().groups as int)}
}
/// Produces stored RGBA components. TIFF alpha association and RGB channel
/// selection are adapter responsibilities. Scratch may change on failure.
pub fn decode(s:&[u8],w:usize,h:usize,tables:&mut[EntropyGroup],scratch:&mut[Pixel],out:&mut[Pixel],budget:&mut Budget) -> (r:Result<(),Error>)
    ensures final(out).len()==old(out).len(),final(scratch).len()==old(scratch).len(),final(tables).len()==old(tables).len(),final(budget).remaining<=old(budget).remaining,
        r.is_ok()==(dimensions(w,h) && old(out).len()>=w as int*h && old(scratch).len()>=storage(w,h) && group_requirement(s@,w,h).is_some() && old(tables).len()>=group_requirement(s@,w,h).unwrap()
            && decoded(s@,w,h).is_some() && 0<=work(s@,w,h)<=old(budget).remaining),
        r.is_ok() ==> final(out)@.take((w as int*h) as int)==decoded(s@,w,h).unwrap(),
        r.is_ok() ==> final(budget).remaining as int+work(s@,w,h)==old(budget).remaining,
        forall|i:int|w as int*h<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
        forall|i:int|storage(w,h)<=i<old(scratch).len() ==> final(scratch)@[i]==old(scratch)@[i],
{
    reveal(group_requirement);reveal(decoded);reveal(work);
    let cap=scratch_len(w,h)?;
    assert(w*h<=268435456) by(nonlinear_arith) requires w<=16384,h<=16384;
    let total=w*h;
    if out.len()<total || scratch.len()<cap{return Err(Error::OutputTooSmall);}
    let fee=prepare_cost(s.len(),w,h)?;spend(budget,fee)?;
    let f=riff::inspect(s,w,h)?;if f.kind!=1{return Err(Error::Unsupported);}
    let bits=crate::deflate_bits::Bits {at:f.at,bit:0,end:f.end};
    let (area,_)=scratch.split_at_mut(cap);let p=prep::prepare(s,bits,w,h,area)?;
    let ghost plan=p.model(area@);
    if tables.len()<p.groups{return Err(Error::OutputTooSmall);}
    let extra=render_cost(w,h,p.groups)?;spend(budget,extra)?;
    let (groups,_)=tables.split_at_mut(p.groups);let after=prep::read_groups(s,p.bits,p.cache,p.groups,groups)?;
    assert(groups@.take(p.groups as int) =~= groups@);
    let (output,_)=out.split_at_mut(total);let mut i=0usize;
    while i<total
        invariant i<=total,output.len()==total,forall|j:int|0<=j<i ==> output@[j]==[0u8,0,0,0],
        decreases total-i,
    {output[i]=[0u8,0,0,0];i=i+1;}
    let c=Config {bits:after,w:p.transforms.width,h,subtract:false,cache:p.cache};
    assert(c.w*h<=total) by(nonlinear_arith) requires c.w<=w,total==w*h,h>0;
    let (data,_)=area.split_at(p.used);let (_,meta)=data.split_at(p.meta_start);
    assert(data@ =~= plan.data);
    assert(meta@ =~= plan.data.subrange(plan.meta_start as int,plan.meta_start as int+plan.meta_count));
    assert(pix::groups(groups@)==prep::group_finish(s@,plan.bits,plan.cache,Seq::empty(),plan.groups as nat).unwrap().tables);
    let end=pix::decode(s,c,groups,meta,p.meta_bits,output)?;
    if end.at!=end.end && (end.bit==0 || end.end-end.at!=1){return Err(Error::Invalid);}
    assert(output@ =~= output@.take((c.w as int*h) as int)+Seq::new((total as int-c.w as int*h) as nat,|i:int|[0u8,0,0,0]));
    assert forall|j:int|0<=j<p.transforms.count implies p.transforms.transforms@[j].width as int*p.transforms.transforms@[j].height<=output.len() by {
        assert(p.transforms.transforms@[j].width as int*h<=w as int*h) by(nonlinear_arith) requires p.transforms.transforms@[j].width<=w,h>0;
    }
    inverse::inverse(&p.transforms.transforms,p.transforms.count,data,output)?;
    Ok(())
}
}
