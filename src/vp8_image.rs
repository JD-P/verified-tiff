//! Bounded raster-order macroblock reconstruction from independent VP8 partitions.
use super::*;
use crate::vp8_bool as ar;
use crate::vp8_macroblock::{self as mb,Context,ContextModel};
use crate::vp8_filter_info::Filter;
use crate::vp8_plane as plane;
verus! {
#[derive(Clone,Copy)]
pub(super) struct State {pub reader:ar::Reader,pub parts:[ar::Reader;8],pub left:Context}
impl State {pub(super) open spec fn valid(self,s:Seq<u8>) -> bool {self.reader.valid(s) && self.left.valid() && (forall|i:int|0<=i<8 ==> #[trigger] self.parts@[i].valid(s))}}
pub(super) struct Model {pub reader:ar::Reader,pub parts:Seq<ar::Reader>,pub left:ContextModel,pub columns:Seq<ContextModel>,pub filters:Seq<Filter>,pub y:Seq<u8>,pub u:Seq<u8>,pub v:Seq<u8>}
pub(super) open spec fn snapshot(st:State,cols:Seq<Context>,filters:Seq<Filter>,y:Seq<u8>,u:Seq<u8>,v:Seq<u8>) -> Model {
    Model {reader:st.reader,parts:st.parts@,left:st.left.model(),columns:Seq::new(cols.len(),|i:int|cols[i].model()),filters,y,u,v}
}
pub(super) open spec fn empty_context() -> ContextModel {ContextModel {modes:Seq::new(4,|i:int|0u8),nz:Seq::new(9,|i:int|0u8)}}
pub(super) open spec fn geometry(w:usize,h:usize,cols:int,filters:int,y:int,u:int,v:int) -> bool {
    16<=w<=16384 && 16<=h<=16384 && w%16==0 && h%16==0 && cols==w/16 && filters==(w/16)*(h/16) && y==w as int*h && u==(w/2)*(h/2) && v==u
}
pub(super) open spec fn step(s:Seq<u8>,header:Seq<i32>,w:usize,count:usize,m:Model,i:usize) -> Option<Model> {
    let width=w/16;let x=i%width;let y=i/width;let part=y%count;
    let left=if x==0{empty_context()}else{m.left};
    let b=mb::decoded(s,header,m.reader,m.parts[part as int],m.columns[x as int],left);
    if b.is_none(){None}else{
        let b=b.unwrap();let mode=if b.modes.prefix.small{0usize}else{b.modes.modes[0] as usize};
        Some(Model {reader:b.modes.reader,parts:m.parts.update(part as int,b.residual.reader),left:b.left,columns:m.columns.update(x as int,b.top),filters:m.filters.update(i as int,b.filter),
            y:plane::decoded(m.y,w,(x*16) as usize,(y*16) as usize,16,b.modes.prefix.small,mode,b.modes.modes,b.residual.blocks,0),
            u:plane::decoded(m.u,(w/2) as usize,(x*8) as usize,(y*8) as usize,8,false,b.modes.chroma as usize,b.modes.modes,b.residual.blocks,16),
            v:plane::decoded(m.v,(w/2) as usize,(x*8) as usize,(y*8) as usize,8,false,b.modes.chroma as usize,b.modes.modes,b.residual.blocks,20)})
    }
}
fn decode_step(s:&[u8],header:&[i32;1096],w:usize,h:usize,count:usize,st:State,cols:&mut[Context],filters:&mut[Filter],yp:&mut[u8],up:&mut[u8],vp:&mut[u8],i:usize) -> (res:Result<State,Error>)
    requires geometry(w,h,old(cols).len() as int,old(filters).len() as int,old(yp).len() as int,old(up).len() as int,old(vp).len() as int),
        st.valid(s@),1<=count<=8,i<old(filters).len(),
        forall|j:int|0<=j<1096 ==> -255<=#[trigger] header@[j]<=255,
        forall|j:int|0<=j<old(cols).len() ==> #[trigger] old(cols)@[j].valid(),
        forall|j:int|0<=j<old(filters).len() ==> #[trigger] old(filters)@[j].limit<=189,
    ensures final(cols).len()==old(cols).len(),final(filters).len()==old(filters).len(),final(yp).len()==old(yp).len(),final(up).len()==old(up).len(),final(vp).len()==old(vp).len(),
        res.is_ok()==step(s@,header@,w,count,snapshot(st,old(cols)@,old(filters)@,old(yp)@,old(up)@,old(vp)@),i).is_some(),
        res.is_ok() ==> snapshot(res.unwrap(),final(cols)@,final(filters)@,final(yp)@,final(up)@,final(vp)@)==step(s@,header@,w,count,snapshot(st,old(cols)@,old(filters)@,old(yp)@,old(up)@,old(vp)@),i).unwrap() && res.unwrap().valid(s@),
        forall|j:int|0<=j<final(cols).len() ==> #[trigger] final(cols)@[j].valid(),forall|j:int|0<=j<final(filters).len() ==> #[trigger] final(filters)@[j].limit<=189,
{
    let width=w/16;let x=i%width;let y=i/width;let part=y%count;
    proof{vstd::arithmetic::div_mod::lemma_fundamental_div_mod(i as int,width as int);}
    assert(y<h/16) by(nonlinear_arith) requires i==width*y+x,0<=x<width,i<width*(h/16),width>0;
    assert(x*16+16<=w && y*16+16<=h);
    let left=if x==0{Context::empty()}else{st.left};
    if x==0{assert(left.modes@ =~= empty_context().modes);assert(left.nz@ =~= empty_context().nz);}
    let b=mb::decode(s,header,st.reader,st.parts[part],cols[x],left)?;
    let ghost prev=snapshot(st,cols@,filters@,yp@,up@,vp@);
    let mode=if b.modes.prefix.small{0usize}else{b.modes.modes[0] as usize};
    plane::decode(yp,w,h,x*16,y*16,16,b.modes.prefix.small,mode,&b.modes.modes,&b.residual.blocks,0);
    plane::decode(up,w/2,h/2,x*8,y*8,8,false,b.modes.chroma as usize,&b.modes.modes,&b.residual.blocks,16);
    plane::decode(vp,w/2,h/2,x*8,y*8,8,false,b.modes.chroma as usize,&b.modes.modes,&b.residual.blocks,20);
    cols[x]=b.top;filters[i]=b.filter;let mut next=st;next.reader=b.modes.reader;next.parts[part]=b.residual.reader;next.left=b.left;
    assert(snapshot(next,cols@,filters@,yp@,up@,vp@).columns =~= prev.columns.update(x as int,b.top.model()));Ok(next)
}
#[verifier::opaque]
pub(super) open spec fn finish(s:Seq<u8>,header:Seq<i32>,w:usize,count:usize,m:Model,i:usize,fuel:nat) -> Option<Model> decreases fuel {
    if fuel==0{Some(m)}else{let next=step(s,header,w,count,m,i);if next.is_none(){None}else{finish(s,header,w,count,next.unwrap(),(i+1) as usize,(fuel-1) as nat)}}
}
pub(super) fn decode(s:&[u8],header:&[i32;1096],w:usize,h:usize,count:usize,st:State,cols:&mut[Context],filters:&mut[Filter],yp:&mut[u8],up:&mut[u8],vp:&mut[u8]) -> (res:Result<State,Error>)
    requires geometry(w,h,old(cols).len() as int,old(filters).len() as int,old(yp).len() as int,old(up).len() as int,old(vp).len() as int),st.valid(s@),1<=count<=8,
        forall|j:int|0<=j<1096 ==> -255<=#[trigger] header@[j]<=255,
        forall|j:int|0<=j<old(cols).len() ==> #[trigger] old(cols)@[j].valid(),forall|j:int|0<=j<old(filters).len() ==> #[trigger] old(filters)@[j].limit<=189,
    ensures final(cols).len()==old(cols).len(),final(filters).len()==old(filters).len(),final(yp).len()==old(yp).len(),final(up).len()==old(up).len(),final(vp).len()==old(vp).len(),
        res.is_ok()==finish(s@,header@,w,count,snapshot(st,old(cols)@,old(filters)@,old(yp)@,old(up)@,old(vp)@),0,old(filters).len() as nat).is_some(),
        res.is_ok() ==> snapshot(res.unwrap(),final(cols)@,final(filters)@,final(yp)@,final(up)@,final(vp)@)==finish(s@,header@,w,count,snapshot(st,old(cols)@,old(filters)@,old(yp)@,old(up)@,old(vp)@),0,old(filters).len() as nat).unwrap() && res.unwrap().valid(s@),
        forall|j:int|0<=j<final(cols).len() ==> #[trigger] final(cols)@[j].valid(),forall|j:int|0<=j<final(filters).len() ==> #[trigger] final(filters)@[j].limit<=189,
{
    let total=filters.len();let mut i=0usize;let mut state=st;
    let ghost initial=snapshot(st,cols@,filters@,yp@,up@,vp@);let ghost goal=finish(s@,header@,w,count,initial,0,total as nat);
    loop
        invariant i<=total,total==filters.len(),geometry(w,h,cols.len() as int,filters.len() as int,yp.len() as int,up.len() as int,vp.len() as int),state.valid(s@),1<=count<=8,
            cols.len()==old(cols).len(),filters.len()==old(filters).len(),yp.len()==old(yp).len(),up.len()==old(up).len(),vp.len()==old(vp).len(),
            initial==snapshot(st,old(cols)@,old(filters)@,old(yp)@,old(up)@,old(vp)@),goal==finish(s@,header@,w,count,initial,0,total as nat),
            goal==finish(s@,header@,w,count,snapshot(state,cols@,filters@,yp@,up@,vp@),i,(total-i) as nat),
            forall|j:int|0<=j<1096 ==> -255<=#[trigger] header@[j]<=255,
            forall|j:int|0<=j<cols.len() ==> #[trigger] cols@[j].valid(),forall|j:int|0<=j<filters.len() ==> #[trigger] filters@[j].limit<=189,
        decreases total-i,
    {
        reveal_with_fuel(finish,1);if i==total{return Ok(state);}
        state=decode_step(s,header,w,h,count,state,cols,filters,yp,up,vp,i)?;i=i+1;
    }
}
}
