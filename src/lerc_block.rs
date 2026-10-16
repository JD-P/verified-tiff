//! Bounded micro-block reconstruction for TIFF's Lerc2 version 4.
use super::*;
use crate::lerc_header::Header;
use crate::lerc_numeric as num;
use crate::lerc_samples as sample;
use crate::lerc_mask as mask;
use crate::lerc_bits as packed;
use crate::native_samples as ns;
verus! {
#[derive(Clone,Copy)]
pub(super) struct Tile {pub x:usize,pub y:usize,pub w:usize,pub h:usize,pub depth:usize}
impl Tile {
    pub(super) open spec fn valid(self,h:Header) -> bool {
        h.invariant() && 0<self.w<=16 && 0<self.h<=16 && self.x as int+self.w<=h.width && self.y as int+self.h<=h.height && self.depth<h.depth
    }
    pub(super) open spec fn pixel(self,h:Header,k:int) -> int {(self.y+k/(self.w as int))*h.width+self.x+k%(self.w as int)}
    pub(super) open spec fn target(self,h:Header,k:int) -> int {self.pixel(h,k)*h.depth+self.depth}
}
#[derive(Clone,Copy)]
pub(super) struct Mode {pub at:usize,pub flag:usize,pub offset:u64,pub quant:usize,pub next:usize}
pub(super) closed spec fn mode(s:Seq<u8>,at:usize,h:Header,t:Tile) -> Option<Mode> {
    if at>=s.len(){None}else{
        let flag=s[at as int] as usize;
        if (flag/4)%16!=(t.x/8)%16{None}else{
            let f=flag%4;
            if f==0 || f==2{Some(Mode {at:(at+1) as usize,flag:f,offset:0,quant:0,next:(at+1) as usize})}
            else{
                let kind=sample::reduced(h.kind,flag/64);
                if kind.is_none(){None}else{
                    let k=kind.unwrap();let value=sample::number(s,(at+1) as usize,k);
                    if value.is_none() || !num::finite(value.unwrap()){None}else{
                        let next=(at+1+num::type_size(k)) as usize;
                        if f==3{Some(Mode {at:next,flag:f,offset:value.unwrap(),quant:0,next})}
                        else{
                            let q=packed::decoded(s,next);
                            if q.is_none(){None}else{Some(Mode {at:q.unwrap().0,flag:f,offset:value.unwrap(),quant:next,next:q.unwrap().0})}
                        }
                    }
                }
            }
        }
    }
}
pub(super) fn read_mode(s:&[u8],at:usize,h:Header,t:Tile,q:&mut packed::Array) -> (r:Result<Mode,Error>)
    requires at<=s.len(),h.invariant(),t.valid(h),
    ensures r.is_ok()==mode(s@,at,h,t).is_some(),r.is_ok() ==> r.unwrap()==mode(s@,at,h,t).unwrap(),
        r.is_ok() ==> at<r.unwrap().at<=s.len() && r.unwrap().flag<4 && final(q).n<=256,
        r.is_ok() ==> r.unwrap().flag!=1 ==> final(q).n==0,
        r.is_ok() ==> r.unwrap().flag==1 ==> packed::decoded(s@,r.unwrap().quant)==Some((r.unwrap().at,final(q).data@.take(final(q).n as int))),
{
    reveal(mode);if at==s.len(){return Err(Error::Truncated);}
    let flag=s[at] as usize;
    if (flag/4)%16!=(t.x/8)%16{return Err(Error::Invalid);}
    let f=flag%4;
    q.n=0;
    if f==0 || f==2{return Ok(Mode {at:at+1,flag:f,offset:0,quant:0,next:at+1});}
    let kind=sample::offset_kind(h.kind,flag/64)?;let offset=sample::read(s,at+1,kind)?;
    if (offset/0x10000000000000u64)%2048==2047{return Err(Error::Invalid);}
    let size=num::size(kind);
    let next=at+1+size;
    if f==3{return Ok(Mode {at:next,flag:f,offset,quant:0,next});}
    let (end,n)=packed::decode_into(s,next,&mut q.data)?;q.at=end;q.n=n;
    Ok(Mode {at:end,flag:f,offset,quant:next,next:end})
}
pub(super) struct Step {pub value:u64,pub at:usize,pub q:usize}
pub(super) closed spec fn value(s:Seq<u8>,h:Header,m:Mode,q:Seq<u32>,at:usize,index:usize,maximum:u64) -> Option<Step> {
    if m.flag==0{
        let size=num::type_size(h.kind);
        if !range_ok(s.len() as int,at as int,size as int){None}else{Some(Step {value:ns::word(s,at as int,size,true) as u64,at:(at+size) as usize,q:index})}
    }else if m.flag==2{Some(Step {value:0,at,q:index})}
    else{
        let v=if m.flag==3{m.offset}else{
            let sum=num::sum(m.offset,num::scaled(h.error,q[index as int]));
            if num::less(sum,maximum){sum}else{maximum}
        };
        if m.flag==1 && index>=q.len(){None}else{
            let bits=sample::storage(v,h.kind);
            if bits.is_none(){None}else{Some(Step {value:bits.unwrap(),at,q:if m.flag==1{(index+1) as usize}else{index}})}
        }
    }
}
#[inline(always)]
fn read_value(s:&[u8],h:Header,m:Mode,q:&packed::Array,at:usize,index:usize,maximum:u64) -> (r:Result<Step,Error>)
    requires at<=s.len(),h.invariant(),m.flag<4,q.n<=256,index<=q.n,
    ensures r.is_ok()==value(s@,h,m,q.data@.take(q.n as int),at,index,maximum).is_some(),
        r.is_ok() ==> r.unwrap()==value(s@,h,m,q.data@.take(q.n as int),at,index,maximum).unwrap(),
        r.is_ok() ==> at<=r.unwrap().at<=s.len() && index<=r.unwrap().q<=q.n,
{
    reveal(value);
    if m.flag==0{
        let size=num::size(h.kind);let v=ns::read_word(s,at,size,true)?;
        return Ok(Step {value:v,at:at+size,q:index});
    }
    if m.flag==2{return Ok(Step {value:0,at,q:index});}
    if m.flag==1 && index>=q.n{return Err(Error::Invalid);}
    let v=if m.flag==3{m.offset}else{num::minimum(num::plus(m.offset,num::scale(h.error,q.data[index])),maximum)};
    let bits=sample::stored(v,h.kind)?;
    Ok(Step {value:bits,at,q:if m.flag==1{index+1}else{index}})
}
pub(super) struct ResultModel {pub at:usize,pub data:Seq<u64>}
#[verifier::opaque]
pub(super) closed spec fn finish(s:Seq<u8>,h:Header,t:Tile,m:Mode,q:Seq<u32>,mask:Seq<u8>,maximum:u64,k:usize,at:usize,index:usize,out:Seq<u64>,fuel:nat) -> Option<ResultModel> decreases fuel {
    if k as int==t.w as int*t.h{
        if m.flag==1 && index!=q.len(){None}else{Some(ResultModel {at,data:out})}
    }else if fuel==0{None}else{
        let pixel=t.pixel(h,k as int);let target=t.target(h,k as int);
        if !mask::valid(mask,pixel){finish(s,h,t,m,q,mask,maximum,(k+1) as usize,at,index,out,(fuel-1) as nat)}
        else{
            let v=value(s,h,m,q,at,index,maximum);
            if v.is_none(){None}else{
                let step=v.unwrap();finish(s,h,t,m,q,mask,maximum,(k+1) as usize,step.at,step.q,out.update(target,step.value),(fuel-1) as nat)
            }
        }
    }
}
fn target_xy(h:Header,t:Tile,row:usize,col:usize) -> (r:(usize,usize))
    requires t.valid(h),row<t.h,col<t.w,
    ensures r.0==t.pixel(h,row as int*t.w+col),r.1==t.target(h,row as int*t.w+col),r.0<h.width*h.height,r.1<h.width*h.height*h.depth,
{
    proof {vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(row as int*t.w+col,t.w as int,row as int,col as int);}
    let yy=t.y+row;let xx=t.x+col;
    assert(yy*h.width+xx<h.width*h.height) by(nonlinear_arith)
        requires yy<h.height,xx<h.width,h.width>0;
    let p=yy*h.width+xx;
    assert(p*h.depth+t.depth<h.width*h.height*h.depth<=usize::MAX) by(nonlinear_arith)
        requires p<h.width*h.height,t.depth<h.depth,h.depth<=64,h.width*h.height<=usize::MAX/512;
    (p,p*h.depth+t.depth)
}
pub(super) fn reconstruct(s:&[u8],h:Header,t:Tile,m:Mode,q:&packed::Array,valid:&[u8],maximum:u64,all_valid:bool,out:&mut[u64]) -> (r:Result<usize,Error>)
    requires t.valid(h),m.at<=s.len(),m.flag<4,q.n<=256,
        all_valid ==> forall|p:int|0<=p<h.width as int*h.height ==> mask::valid(valid@,p),
        (h.width as int*h.height+7)/8<=valid.len(),h.width as int*h.height*h.depth<=old(out).len(),
    ensures final(out).len()==old(out).len(),
        r.is_ok()==finish(s@,h,t,m,q.data@.take(q.n as int),valid@,maximum,0,m.at,0,old(out)@,256).is_some(),
        r.is_ok() ==> r.unwrap()==finish(s@,h,t,m,q.data@.take(q.n as int),valid@,maximum,0,m.at,0,old(out)@,256).unwrap().at,
        r.is_ok() ==> final(out)@==finish(s@,h,t,m,q.data@.take(q.n as int),valid@,maximum,0,m.at,0,old(out)@,256).unwrap().data,
        r.is_ok() ==> m.at<=r.unwrap()<=s.len(),
        forall|j:int|h.width as int*h.height*h.depth<=j<old(out).len() ==> final(out)@[j]==old(out)@[j],
{
    if m.flag==1 && h.kind==6{
        if all_valid{return reconstruct_float(s,h,t,m,q,valid,maximum,true,6,out);}
        return reconstruct_float(s,h,t,m,q,valid,maximum,false,6,out);
    }
    if m.flag==1 && h.kind==7{
        if all_valid{return reconstruct_float(s,h,t,m,q,valid,maximum,true,7,out);}
        return reconstruct_float(s,h,t,m,q,valid,maximum,false,7,out);
    }
    let mut k=0usize;let mut at=m.at;let mut index=0usize;let mut row=0usize;let mut col=0usize;
    let ghost goal=finish(s@,h,t,m,q.data@.take(q.n as int),valid@,maximum,0,m.at,0,out@,256);
    assert(t.w*t.h<=256) by(nonlinear_arith) requires t.w<=16,t.h<=16;
    while k<t.w*t.h
        invariant t.valid(h),t.w*t.h<=256,k<=t.w*t.h,
            row<=t.h,col<t.w,k==row as int*t.w+col,
            all_valid ==> forall|p:int|0<=p<h.width as int*h.height ==> mask::valid(valid@,p),m.at<=at<=s.len(),m.flag<4,index<=q.n<=256,
            (h.width as int*h.height+7)/8<=valid.len(),h.width as int*h.height*h.depth<=out.len(),out.len()==old(out).len(),
            goal==finish(s@,h,t,m,q.data@.take(q.n as int),valid@,maximum,0,m.at,0,old(out)@,256),
            goal==finish(s@,h,t,m,q.data@.take(q.n as int),valid@,maximum,k,at,index,out@,(256-k) as nat),
            forall|j:int|h.width as int*h.height*h.depth<=j<out.len() ==> out@[j]==old(out)@[j],
        decreases 256-k,
    {
        reveal_with_fuel(finish,1);
        assert(row<t.h) by(nonlinear_arith) requires k<t.w*t.h,k==row*t.w+col,t.w>0,col>=0;
        let (p,dest)=target_xy(h,t,row,col);
        if all_valid || mask::is_valid(valid,p){
            let v=read_value(s,h,m,q,at,index,maximum)?;
            out[dest]=v.value;at=v.at;index=v.q;
        }
        k=k+1;
        if col+1==t.w{assert((row as int+1)*t.w==row as int*t.w+t.w) by(nonlinear_arith);col=0;row=row+1;}else{col=col+1;}
    }
    reveal_with_fuel(finish,1);
    if m.flag==1 && index!=q.n{return Err(Error::Invalid);}
    Ok(at)
}
#[inline(always)]
fn reconstruct_float(s:&[u8],h:Header,t:Tile,m:Mode,q:&packed::Array,valid:&[u8],maximum:u64,all_valid:bool,kind:usize,out:&mut[u64]) -> (r:Result<usize,Error>)
    requires t.valid(h),m.at<=s.len(),m.flag==1,q.n<=256,kind==h.kind,kind==6 || kind==7,
        all_valid ==> forall|p:int|0<=p<h.width as int*h.height ==> mask::valid(valid@,p),
        (h.width as int*h.height+7)/8<=valid.len(),h.width as int*h.height*h.depth<=old(out).len(),
    ensures final(out).len()==old(out).len(),
        r.is_ok()==finish(s@,h,t,m,q.data@.take(q.n as int),valid@,maximum,0,m.at,0,old(out)@,256).is_some(),
        r.is_ok() ==> r.unwrap()==finish(s@,h,t,m,q.data@.take(q.n as int),valid@,maximum,0,m.at,0,old(out)@,256).unwrap().at,
        r.is_ok() ==> final(out)@==finish(s@,h,t,m,q.data@.take(q.n as int),valid@,maximum,0,m.at,0,old(out)@,256).unwrap().data,
        r.is_ok() ==> m.at<=r.unwrap()<=s.len(),
        forall|j:int|h.width as int*h.height*h.depth<=j<old(out).len() ==> final(out)@[j]==old(out)@[j],
{
    let mut k=0usize;let mut at=m.at;let mut index=0usize;let mut row=0usize;let mut col=0usize;
    let ghost goal=finish(s@,h,t,m,q.data@.take(q.n as int),valid@,maximum,0,m.at,0,out@,256);
    assert(t.w*t.h<=256) by(nonlinear_arith) requires t.w<=16,t.h<=16;
    while k<t.w*t.h
        invariant kind==h.kind,kind==6 || kind==7,m.flag==1,t.valid(h),t.w*t.h<=256,k<=t.w*t.h,
            row<=t.h,col<t.w,k==row as int*t.w+col,
            all_valid ==> forall|p:int|0<=p<h.width as int*h.height ==> mask::valid(valid@,p),m.at<=at<=s.len(),m.flag<4,index<=q.n<=256,
            (h.width as int*h.height+7)/8<=valid.len(),h.width as int*h.height*h.depth<=out.len(),out.len()==old(out).len(),
            goal==finish(s@,h,t,m,q.data@.take(q.n as int),valid@,maximum,0,m.at,0,old(out)@,256),
            goal==finish(s@,h,t,m,q.data@.take(q.n as int),valid@,maximum,k,at,index,out@,(256-k) as nat),
            forall|j:int|h.width as int*h.height*h.depth<=j<out.len() ==> out@[j]==old(out)@[j],
        decreases 256-k,
    {
        reveal_with_fuel(finish,1);
        assert(row<t.h) by(nonlinear_arith) requires k<t.w*t.h,k==row*t.w+col,t.w>0,col>=0;
        let (p,dest)=target_xy(h,t,row,col);
        if all_valid || mask::is_valid(valid,p){
            reveal(value);reveal(sample::storage);
            if index>=q.n{return Err(Error::Invalid);}
            let v=num::minimum(num::plus(m.offset,num::scale(h.error,q.data[index])),maximum);
            let bits=if kind==6{num::narrow(v) as u64}else{v};
            out[dest]=bits;index=index+1;
        }
        k=k+1;
        if col+1==t.w{assert((row as int+1)*t.w==row as int*t.w+t.w) by(nonlinear_arith);col=0;row=row+1;}else{col=col+1;}
    }
    reveal_with_fuel(finish,1);
    if m.flag==1 && index!=q.n{return Err(Error::Invalid);}
    Ok(at)
}
}
