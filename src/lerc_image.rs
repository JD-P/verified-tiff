//! Image traversal and exact native-word reconstruction for Lerc2.
use super::*;
use crate::lerc_header::Header;
use crate::lerc_numeric as num;
use crate::lerc_mask as mask;
use crate::lerc_block as block;
use crate::native_samples as ns;
verus! {
#[derive(Clone,Copy)]
pub(super) struct Position {pub x:usize,pub y:usize,pub d:usize}
impl Position {
    pub(super) open spec fn valid(self,h:Header) -> bool {h.invariant() && self.x<h.width && self.y<=h.height && self.d<h.depth}
    pub(super) open spec fn tile(self,h:Header) -> block::Tile {
        block::Tile {x:self.x,y:self.y,depth:self.d,w:if h.width-self.x<h.block{(h.width-self.x) as usize}else{h.block},
            h:if h.height-self.y<h.block{(h.height-self.y) as usize}else{h.block}}
    }
    pub(super) open spec fn next(self,h:Header) -> Position {
        if self.d as int+1<h.depth{Position {x:self.x,y:self.y,d:(self.d+1) as usize}}
        else{
            let t=self.tile(h);
            if self.x as int+t.w<h.width{Position {x:(self.x+t.w) as usize,y:self.y,d:0}}
            else{Position {x:0,y:(self.y+t.h) as usize,d:0}}
        }
    }
    fn current(self,h:Header) -> (r:block::Tile)
        requires self.valid(h),self.y<h.height,
        ensures r==self.tile(h),r.valid(h),
    {
        block::Tile {x:self.x,y:self.y,depth:self.d,w:if h.width-self.x<h.block{h.width-self.x}else{h.block},
            h:if h.height-self.y<h.block{h.height-self.y}else{h.block}}
    }
    fn advance(self,h:Header) -> (r:Position)
        requires self.valid(h),self.y<h.height,
        ensures r==self.next(h),r.valid(h),
    {
        if self.d+1<h.depth{return Position {x:self.x,y:self.y,d:self.d+1};}
        let t=self.current(h);
        if self.x+t.w<h.width{Position {x:self.x+t.w,y:self.y,d:0}}
        else{Position {x:0,y:self.y+t.h,d:0}}
    }
}
#[verifier::opaque]
pub(super) closed spec fn tiles(s:Seq<u8>,h:Header,mask:Seq<u8>,max:Seq<u64>,p:Position,at:usize,out:Seq<u64>,fuel:nat) -> Option<block::ResultModel> decreases fuel {
    if p.y==h.height{Some(block::ResultModel {at,data:out})}else if fuel==0{None}else{
        let t=p.tile(h);let m=block::mode(s,at,h,t);
        if m.is_none(){None}else{
            let mode=m.unwrap();let q=if mode.flag==1{crate::lerc_bits::decoded(s,mode.quant).unwrap().1}else{Seq::<u32>::empty()};
            let b=block::finish(s,h,t,mode,q,mask,max[p.d as int],0,mode.at,0,out,256);
            if b.is_none(){None}else{tiles(s,h,mask,max,p.next(h),b.unwrap().at,b.unwrap().data,(fuel-1) as nat)}
        }
    }
}
pub(super) fn decode_tiles(s:&[u8],h:Header,valid:&[u8],maximum:&[u64;64],start:usize,out:&mut[u64]) -> (r:Result<usize,Error>)
    requires h.invariant(),start<=s.len(),(h.width as int*h.height+7)/8<=valid.len(),h.width as int*h.height*h.depth<=old(out).len(),
    ensures final(out).len()==old(out).len(),
        r.is_ok()==tiles(s@,h,valid@,maximum@,Position {x:0,y:0,d:0},start,old(out)@,(h.width as int*h.height*h.depth) as nat).is_some(),
        r.is_ok() ==> r.unwrap()==tiles(s@,h,valid@,maximum@,Position {x:0,y:0,d:0},start,old(out)@,(h.width as int*h.height*h.depth) as nat).unwrap().at,
        r.is_ok() ==> final(out)@==tiles(s@,h,valid@,maximum@,Position {x:0,y:0,d:0},start,old(out)@,(h.width as int*h.height*h.depth) as nat).unwrap().data,
        r.is_ok() ==> start<=r.unwrap()<=s.len(),
        forall|j:int|h.width as int*h.height*h.depth<=j<old(out).len() ==> final(out)@[j]==old(out)@[j],
{
    assert(h.width*h.height*h.depth<=usize::MAX) by(nonlinear_arith)
        requires h.width*h.height<=usize::MAX/512,h.depth<=64;
    let all_valid=mask::all(valid,h.width*h.height);
    let mut q=crate::lerc_bits::Array {at:0,n:0,data:[0u32;256]};
    let mut fuel=h.width*h.height*h.depth;let mut p=Position {x:0,y:0,d:0};let mut at=start;
    let ghost goal=tiles(s@,h,valid@,maximum@,p,start,out@,fuel as nat);
    loop
        invariant h.invariant(),p.valid(h),
            all_valid ==> forall|i:int|0<=i<h.width as int*h.height ==> mask::valid(valid@,i),start<=at<=s.len(),out.len()==old(out).len(),
            (h.width as int*h.height+7)/8<=valid.len(),h.width as int*h.height*h.depth<=out.len(),
            goal==tiles(s@,h,valid@,maximum@,Position {x:0,y:0,d:0},start,old(out)@,(h.width as int*h.height*h.depth) as nat),
            goal==tiles(s@,h,valid@,maximum@,p,at,out@,fuel as nat),
            forall|j:int|h.width as int*h.height*h.depth<=j<out.len() ==> out@[j]==old(out)@[j],
        decreases fuel,
    {
        reveal_with_fuel(tiles,1);
        if p.y==h.height{return Ok(at);}
        if fuel==0{return Err(Error::Invalid);}
        let tile=p.current(h);let mode=block::read_mode(s,at,h,tile,&mut q)?;
        if mode.flag!=1{assert(q.data@.take(q.n as int) =~= Seq::<u32>::empty());}
        at=block::reconstruct(s,h,tile,mode,&q,valid,maximum[p.d],all_valid,out)?;
        p=p.advance(h);fuel=fuel-1;
    }
}
pub(super) closed spec fn linear_value(s:Seq<u8>,h:Header,at:usize,k:usize,raw:bool,constant:Seq<u64>) -> Option<(usize,u64)> {
    if !raw{Some((at,constant[(k as int)%(h.depth as int)]))}else{
        let size=num::type_size(h.kind);
        if !range_ok(s.len() as int,at as int,size as int){None}else{Some(((at+size) as usize,ns::word(s,at as int,size,true) as u64))}
    }
}
#[verifier::opaque]
pub(super) closed spec fn linear(s:Seq<u8>,h:Header,mask:Seq<u8>,constant:Seq<u64>,raw:bool,k:usize,at:usize,out:Seq<u64>,fuel:nat) -> Option<block::ResultModel> decreases fuel {
    if k as int==h.width as int*h.height*h.depth{Some(block::ResultModel {at,data:out})}
    else if fuel==0{None}else if !mask::valid(mask,(k as int)/(h.depth as int)){
        linear(s,h,mask,constant,raw,(k+1) as usize,at,out,(fuel-1) as nat)
    }else{
        let v=linear_value(s,h,at,k,raw,constant);
        if v.is_none(){None}else{linear(s,h,mask,constant,raw,(k+1) as usize,v.unwrap().0,out.update(k as int,v.unwrap().1),(fuel-1) as nat)}
    }
}
pub(super) fn decode_linear(s:&[u8],h:Header,valid:&[u8],constant:&[u64;64],raw:bool,start:usize,out:&mut[u64]) -> (r:Result<usize,Error>)
    requires h.invariant(),start<=s.len(),(h.width as int*h.height+7)/8<=valid.len(),h.width as int*h.height*h.depth<=old(out).len(),
    ensures final(out).len()==old(out).len(),
        r.is_ok()==linear(s@,h,valid@,constant@,raw,0,start,old(out)@,(h.width as int*h.height*h.depth) as nat).is_some(),
        r.is_ok() ==> r.unwrap()==linear(s@,h,valid@,constant@,raw,0,start,old(out)@,(h.width as int*h.height*h.depth) as nat).unwrap().at,
        r.is_ok() ==> final(out)@==linear(s@,h,valid@,constant@,raw,0,start,old(out)@,(h.width as int*h.height*h.depth) as nat).unwrap().data,
        r.is_ok() ==> start<=r.unwrap()<=s.len(),
        forall|j:int|h.width as int*h.height*h.depth<=j<old(out).len() ==> final(out)@[j]==old(out)@[j],
{
    assert(h.width*h.height*h.depth<=usize::MAX) by(nonlinear_arith)
        requires h.width*h.height<=usize::MAX/512,h.depth<=64;
    let count=h.width*h.height*h.depth;let mut k=0usize;let mut at=start;let size=num::size(h.kind);
    let ghost goal=linear(s@,h,valid@,constant@,raw,0,start,out@,count as nat);
    while k<count
        invariant h.invariant(),size==num::type_size(h.kind),count==h.width as int*h.height*h.depth,k<=count,
            start<=at<=s.len(),out.len()==old(out).len(),count<=out.len(),(h.width as int*h.height+7)/8<=valid.len(),
            goal==linear(s@,h,valid@,constant@,raw,0,start,old(out)@,count as nat),
            goal==linear(s@,h,valid@,constant@,raw,k,at,out@,(count-k) as nat),
            forall|j:int|count<=j<out.len() ==> out@[j]==old(out)@[j],
        decreases count-k,
    {
        reveal_with_fuel(linear,1);reveal(linear_value);
        assert(k/h.depth<h.width*h.height) by(nonlinear_arith) requires k<h.width*h.height*h.depth,h.depth>0;
        if mask::is_valid(valid,k/h.depth){
            let v=if raw{let v=ns::read_word(s,at,size,true)?;at=at+size;v}else{constant[k%h.depth]};
            out[k]=v;
        }
        k=k+1;
    }
    reveal_with_fuel(linear,1);Ok(at)
}
}
