//! Native, bounded VP8 keyframe decoding for static WebP TIFF segments.
//! Planar output retains macroblock padding; the RGBA renderer crops it.
use super::*;
use crate::webp_riff as riff;
use crate::vp8_frame as frame;
use crate::vp8_image as image;
use crate::vp8_postfilter as post;
pub use crate::vp8_macroblock::Context;
pub use crate::vp8_filter_info::Filter;
verus! {
#[derive(Clone,Copy,Debug)]
pub struct Info {pub width:usize,pub height:usize,pub y_len:usize,pub chroma_len:usize,pub bytes:usize,pub columns:usize,pub blocks:usize}
pub open spec fn dimensions(w:usize,h:usize) -> bool {1<=w<=16383 && 1<=h<=16383}
pub open spec fn shape(w:usize,h:usize) -> Info {
    let x=(w+15)/16*16;let y=(h+15)/16*16;let a=x*y;let c=(x/2)*(y/2);
    Info {width:x as usize,height:y as usize,y_len:a as usize,chroma_len:c as usize,bytes:(a+2*c) as usize,columns:(x/16) as usize,blocks:((x/16)*(y/16)) as usize}
}
pub fn storage(w:usize,h:usize) -> (res:Result<Info,Error>)
    ensures res.is_ok()==dimensions(w,h),res.is_ok() ==> res.unwrap()==shape(w,h) && 16<=res.unwrap().width<=16384 && 16<=res.unwrap().height<=16384 && res.unwrap().bytes<=402653184,
        res.is_ok() ==> {let v=res.unwrap();w<=v.width && h<=v.height && v.width%16==0 && v.height%16==0 && v.y_len==v.width as int*v.height && v.chroma_len==(v.width/2)*(v.height/2) && v.bytes==v.y_len as int+2*v.chroma_len && v.columns==v.width/16 && v.blocks==(v.width/16)*(v.height/16)},
{
    if w==0 || h==0 || w>16383 || h>16383{return Err(Error::Invalid);}
    let x=(w+15)/16*16;let y=(h+15)/16*16;
    assert(x*y<=268435456) by(nonlinear_arith) requires x<=16384,y<=16384;
    assert((x/2)*(y/2)<=67108864) by(nonlinear_arith) requires x<=16384,y<=16384;
    assert((x/16)*(y/16)<=1048576) by(nonlinear_arith) requires x<=16384,y<=16384;
    let a=x*y;let c=(x/2)*(y/2);
    Ok(Info {width:x,height:y,y_len:a,chroma_len:c,bytes:a+2*c,columns:x/16,blocks:(x/16)*(y/16)})
}
pub open spec fn cost(bytes:int,w:usize,h:usize) -> int {64*bytes+1000000*shape(w,h).blocks+1000*shape(w,h).y_len+4000000}
pub fn work_limit(bytes:usize,w:usize,h:usize) -> (res:Result<usize,Error>)
    ensures res.is_ok()==(dimensions(w,h) && cost(bytes as int,w,h)<=usize::MAX),res.is_ok() ==> res.unwrap()==cost(bytes as int,w,h),
{
    let info=storage(w,h)?;add(add(add(mul(bytes,64)?,mul(info.blocks,1000000)?)?,mul(info.y_len,1000)?)?,4000000)
}
pub(super) open spec fn empty_filter() -> Filter {Filter {limit:0,inner:0,threshold:0,filter_inner:false,simple:false}}
pub(super) open spec fn initial(f:frame::Model,info:Info) -> image::Model {
    image::Model {reader:f.reader,parts:f.parts,left:image::empty_context(),columns:Seq::new(info.columns as nat,|i:int|image::empty_context()),
        filters:Seq::new(info.blocks as nat,|i:int|empty_filter()),y:Seq::new(info.y_len as nat,|i:int|0u8),u:Seq::new(info.chroma_len as nat,|i:int|0u8),v:Seq::new(info.chroma_len as nat,|i:int|0u8)}
}
pub closed spec fn decoded(s:Seq<u8>,w:usize,h:usize) -> Option<Seq<u8>> {
    if !dimensions(w,h){None}else{
        let r=riff::frame(s,w,h);
        if r.is_none() || r.unwrap().kind!=2{None}else{
            let f=frame::parsed(s,r.unwrap().at,r.unwrap().end);
            if f.is_none(){None}else{
                let info=shape(w,h);let m=image::finish(s,f.unwrap().header,info.width,f.unwrap().count,initial(f.unwrap(),info),0,info.blocks as nat);
                if m.is_none(){None}else{
                    let m=m.unwrap();let p=post::finish(post::Model {y:m.y,u:m.u,v:m.v},info.width,m.filters,0,info.blocks as nat);
                    Some(p.y+p.u+p.v)
                }
            }
        }
    }
}
fn zero(p:&mut[u8])
    ensures final(p).len()==old(p).len(),forall|i:int|0<=i<final(p).len() ==> #[trigger] final(p)@[i]==0,
{
    let mut i=0usize;while i<p.len()
        invariant i<=p.len(),p.len()==old(p).len(),forall|j:int|0<=j<i ==> #[trigger] p@[j]==0,
        decreases p.len()-i,
    {p[i]=0;i=i+1;}
}
fn initialize_columns(cols:&mut[Context])
    ensures final(cols).len()==old(cols).len(),forall|j:int|0<=j<final(cols).len() ==> #[trigger] final(cols)@[j].model()==image::empty_context(),forall|j:int|0<=j<final(cols).len() ==> #[trigger] final(cols)@[j].valid(),
{
    let mut i=0usize;
    while i<cols.len()
        invariant i<=cols.len(),cols.len()==old(cols).len(),forall|j:int|0<=j<i ==> #[trigger] cols@[j].model()==image::empty_context(),forall|j:int|0<=j<i ==> #[trigger] cols@[j].valid(),
        decreases cols.len()-i,
    {
        let c=Context::empty();assert(c.modes@ =~= image::empty_context().modes);assert(c.nz@ =~= image::empty_context().nz);cols[i]=c;i=i+1;
    }
}
fn initialize_filters(fs:&mut[Filter])
    ensures final(fs).len()==old(fs).len(),forall|j:int|0<=j<final(fs).len() ==> #[trigger] final(fs)@[j]==empty_filter(),
{
    let mut i=0usize;while i<fs.len()
        invariant i<=fs.len(),fs.len()==old(fs).len(),forall|j:int|0<=j<i ==> #[trigger] fs@[j]==empty_filter(),
        decreases fs.len()-i,
    {fs[i]=Filter::empty();i=i+1;}
}
/// Decodes padded Y, U, V planes in that order. Scratch prefixes can change on
/// errors; output and scratch suffixes are preserved. No allocation occurs.
pub fn decode_yuv(s:&[u8],w:usize,h:usize,pixels:&mut[u8],columns:&mut[Context],filters:&mut[Filter],budget:&mut Budget) -> (res:Result<Info,Error>)
    ensures final(pixels).len()==old(pixels).len(),final(columns).len()==old(columns).len(),final(filters).len()==old(filters).len(),final(budget).remaining<=old(budget).remaining,
        res.is_ok()==(dimensions(w,h) && old(pixels).len()>=shape(w,h).bytes && old(columns).len()>=shape(w,h).columns && old(filters).len()>=shape(w,h).blocks &&
            decoded(s@,w,h).is_some() && cost(s.len() as int,w,h)<=old(budget).remaining),
        res.is_ok() ==> res.unwrap()==shape(w,h) && final(pixels)@.take(shape(w,h).bytes as int)==decoded(s@,w,h).unwrap() && final(budget).remaining as int+cost(s.len() as int,w,h)==old(budget).remaining,
        forall|i:int|shape(w,h).bytes<=i<old(pixels).len() ==> final(pixels)@[i]==old(pixels)@[i],
        forall|i:int|shape(w,h).columns<=i<old(columns).len() ==> final(columns)@[i]==old(columns)@[i],
        forall|i:int|shape(w,h).blocks<=i<old(filters).len() ==> final(filters)@[i]==old(filters)@[i],
{
    reveal(decoded);let info=storage(w,h)?;
    if pixels.len()<info.bytes || columns.len()<info.columns || filters.len()<info.blocks{return Err(Error::OutputTooSmall);}
    let fee=work_limit(s.len(),w,h)?;spend(budget,fee)?;
    let r=riff::inspect(s,w,h)?;if r.kind!=2{return Err(Error::Unsupported);}
    let f=frame::parse(s,r.at,r.end)?;
    let (area,_)=pixels.split_at_mut(info.bytes);let (cols,_)=columns.split_at_mut(info.columns);let (fs,_)=filters.split_at_mut(info.blocks);
    zero(area);initialize_columns(cols);initialize_filters(fs);
    let (yp,uv)=area.split_at_mut(info.y_len);let (up,vp)=uv.split_at_mut(info.chroma_len);
    let left=Context::empty();assert(left.modes@ =~= image::empty_context().modes);assert(left.nz@ =~= image::empty_context().nz);
    let st=image::State {reader:f.reader,parts:f.parts,left};
    assert(yp@ =~= Seq::new(info.y_len as nat,|i:int|0u8));assert(up@ =~= Seq::new(info.chroma_len as nat,|i:int|0u8));assert(vp@ =~= Seq::new(info.chroma_len as nat,|i:int|0u8));
    assert(image::snapshot(st,cols@,fs@,yp@,up@,vp@).columns =~= initial(f.model(),info).columns);assert(fs@ =~= initial(f.model(),info).filters);
    let _=image::decode(s,&f.header,info.width,info.height,f.count,st,cols,fs,yp,up,vp)?;
    post::filter(info.width,info.height,fs,yp,up,vp);Ok(info)
}
pub closed spec fn rgba_sample(p:Seq<u8>,w:usize,h:usize,k:usize) -> [u8;4] {
    let info=shape(w,h);crate::vp8_color::sample(p,info.width,info.height,w,h,k)
}
/// Crops padded YUV planes and uses centered bilinear chroma reconstruction.
/// Alpha is initialized to 255; a separate ALPH plane may subsequently replace it.
pub fn render_rgba(p:&[u8],w:usize,h:usize,out:&mut[[u8;4]],budget:&mut Budget) -> (res:Result<(),Error>)
    ensures final(out).len()==old(out).len(),final(budget).remaining<=old(budget).remaining,
        res.is_ok()==(dimensions(w,h) && p.len()>=shape(w,h).bytes && old(out).len()>=w as int*h && 1000*w*h<=old(budget).remaining),
        res.is_ok() ==> final(budget).remaining as int+1000*w*h==old(budget).remaining,
        res.is_ok() ==> forall|i:int|0<=i<w as int*h ==> #[trigger] final(out)@[i]==rgba_sample(p@,w,h,i as usize),
        forall|i:int|w as int*h<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
{
    reveal(rgba_sample);let info=storage(w,h)?;
    assert(w*h<=268435456) by(nonlinear_arith) requires w<=16383,h<=16383;
    let n=w*h;if p.len()<info.bytes || out.len()<n{return Err(Error::OutputTooSmall);}
    assert(n*1000==1000*w*h) by(nonlinear_arith) requires n==w*h;
    let fee=mul(n,1000)?;spend(budget,fee)?;
    assert(info.bytes==info.width as int*info.height+2*(info.width/2)*(info.height/2)) by(nonlinear_arith) requires info.bytes==info.y_len+2*info.chroma_len,info.y_len==info.width as int*info.height,info.chroma_len==(info.width/2)*(info.height/2);
    let mut i=0usize;
    while i<n
        invariant dimensions(w,h),n==w*h,i<=n,info==shape(w,h),w<=info.width<=16384,h<=info.height<=16384,info.width%16==0,info.height%16==0,
            info.bytes==info.width as int*info.height+2*(info.width/2)*(info.height/2),p.len()>=info.bytes,out.len()==old(out).len(),out.len()>=n,
            budget.remaining as int+1000*w*h==old(budget).remaining,
            forall|j:int|0<=j<i ==> #[trigger] out@[j]==rgba_sample(p@,w,h,j as usize),
            forall|j:int|n<=j<out.len() ==> out@[j]==old(out)@[j],
        decreases n-i,
    {
        if verified_tiff_simd::available() && w-i%w>=4 {
            proof{
                vstd::arithmetic::div_mod::lemma_fundamental_div_mod(i as int,w as int);
                let q=i as int/w as int;let r=i as int%w as int;
                assert(q<h) by(nonlinear_arith) requires i==w*q+r,r>=0,i<n,n==w*h,w>0;
                assert(i+4<=n) by(nonlinear_arith) requires i==w*q+r,q<h,r+4<=w,n==w*h,w>0;
            }
            let batch=crate::vp8_color::pixels4(p,info.width,info.height,w,h,i);
            out[i]=batch[0];out[i+1]=batch[1];out[i+2]=batch[2];out[i+3]=batch[3];i=i+4;
        }else{out[i]=crate::vp8_color::pixel(p,info.width,info.height,w,h,i);i=i+1;}
    }
    Ok(())
}

}
