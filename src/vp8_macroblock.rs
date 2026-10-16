//! One keyframe macroblock: mode grammar, coefficient contexts and filter state.
use super::*;
use crate::vp8_bool as ar;
use crate::vp8_modes as modes;
use crate::vp8_residual as residual;
use crate::vp8_filter_info::{self as filter,Filter};
verus! {
#[derive(Clone,Copy)]
pub struct Context {pub modes:[u8;4],pub nz:[u8;9]}
pub(super) struct ContextModel {pub modes:Seq<u8>,pub nz:Seq<u8>}
impl Context {
    pub(super) open spec fn model(self) -> ContextModel {ContextModel {modes:self.modes@,nz:self.nz@}}
    pub(super) open spec fn valid(self) -> bool {(forall|i:int|0<=i<4 ==> #[trigger] self.modes@[i]<10) && (forall|i:int|0<=i<9 ==> #[trigger] self.nz@[i]<=1)}
    pub fn empty() -> (r:Self)
        ensures forall|i:int|0<=i<4 ==> #[trigger] r.modes@[i]==0,forall|i:int|0<=i<9 ==> #[trigger] r.nz@[i]==0,
    {Self {modes:[0u8;4],nz:[0u8;9]}}
}
#[derive(Clone,Copy)]
pub(super) struct ModeData {pub prefix:modes::Prefix,pub reader:ar::Reader,pub modes:[u8;16],pub chroma:u8}
pub(super) struct ModeModel {pub prefix:modes::Prefix,pub reader:ar::Reader,pub modes:Seq<u8>,pub chroma:u8}
impl ModeData {pub(super) open spec fn model(self) -> ModeModel {ModeModel {prefix:self.prefix,reader:self.reader,modes:self.modes@,chroma:self.chroma}}}
pub(super) open spec fn mode_stage(s:Seq<u8>,h:Seq<i32>,r:ar::Reader,top:ContextModel,left:ContextModel) -> Option<ModeModel> {
    let head=modes::head(s,r,h);if head.is_none(){None}else{
        let prefix=head.unwrap().0;
        let y=if prefix.small{modes::finish(s,head.unwrap().1,top.modes,left.modes,Seq::new(16,|i:int|0u8),0,16)}else{
            let b=modes::large(s,head.unwrap().1,false);if b.is_none(){None}else{Some((b.unwrap().1,Seq::new(16,|i:int|b.unwrap().0)))}
        };
        if y.is_none(){None}else{
            let uv=modes::large(s,y.unwrap().0,true);if uv.is_none(){None}else{Some(ModeModel {prefix,reader:uv.unwrap().1,modes:y.unwrap().1,chroma:uv.unwrap().0})}
        }
    }
}
fn read_modes(s:&[u8],h:&[i32;1096],r:ar::Reader,top:Context,left:Context) -> (res:Result<ModeData,Error>)
    requires r.valid(s@),top.valid(),left.valid(),
    ensures res.is_ok()==mode_stage(s@,h@,r,top.model(),left.model()).is_some(),
        res.is_ok() ==> res.unwrap().model()==mode_stage(s@,h@,r,top.model(),left.model()).unwrap() && res.unwrap().reader.valid(s@) && res.unwrap().prefix.segment<4 && res.unwrap().chroma<4,
        res.is_ok() ==> forall|i:int|0<=i<16 ==> #[trigger] res.unwrap().modes@[i]<10 && (!res.unwrap().prefix.small ==> res.unwrap().modes@[i]<4),
{
    let (prefix,t)=modes::prefix(s,r,h)?;let mut im=[0u8;16];assert(im@ =~= Seq::new(16,|i:int|0u8));let after;
    if prefix.small{after=modes::small_modes(s,t,&top.modes,&left.modes,&mut im)?;}
    else{let (m,t)=modes::main_mode(s,t,false)?;im=[m;16];after=t;assert(im@ =~= Seq::new(16,|i:int|m));}
    let (uv,t)=modes::main_mode(s,after,true)?;Ok(ModeData {prefix,reader:t,modes:im,chroma:uv})
}
#[derive(Clone,Copy)]
pub(super) struct Block {pub modes:ModeData,pub residual:residual::Residual,pub top:Context,pub left:Context,pub filter:Filter}
pub(super) struct BlockModel {pub modes:ModeModel,pub residual:residual::Model,pub top:ContextModel,pub left:ContextModel,pub filter:Filter}
impl Block {
    pub(super) open spec fn model(self) -> BlockModel {BlockModel {modes:self.modes.model(),residual:self.residual.model(),top:self.top.model(),left:self.left.model(),filter:self.filter}}
    pub(super) open spec fn valid(self,s:Seq<u8>) -> bool {self.modes.reader.valid(s) && self.residual.valid(s) && self.top.valid() && self.left.valid() && self.modes.chroma<4 && self.filter.limit<=189 &&
        (forall|i:int|0<=i<16 ==> #[trigger] self.modes.modes@[i]<10 && (!self.modes.prefix.small ==> self.modes.modes@[i]<4))}
}
pub(super) open spec fn decoded(s:Seq<u8>,h:Seq<i32>,r:ar::Reader,token:ar::Reader,top:ContextModel,left:ContextModel) -> Option<BlockModel> {
    let mm=mode_stage(s,h,r,top,left);if mm.is_none(){None}else{
        let m=mm.unwrap();let d=residual::decoded(s,h,m.prefix.segment as usize,m.prefix.small,m.prefix.skip,token,top.nz,left.nz);
        if d.is_none(){None}else{Some(BlockModel {modes:m,residual:d.unwrap(),
            top:ContextModel {modes:Seq::new(4,|i:int|m.modes[12+i]),nz:d.unwrap().top},
            left:ContextModel {modes:Seq::new(4,|i:int|m.modes[3+4*i]),nz:d.unwrap().left},
            filter:filter::strength(h,m.prefix.segment as usize,m.prefix.small,d.unwrap().empty)})}
    }
}
pub(super) fn decode(s:&[u8],h:&[i32;1096],r:ar::Reader,token:ar::Reader,top:Context,left:Context) -> (res:Result<Block,Error>)
    requires r.valid(s@),token.valid(s@),top.valid(),left.valid(),forall|i:int|0<=i<1096 ==> -255<=#[trigger] h@[i]<=255,
    ensures res.is_ok()==decoded(s@,h@,r,token,top.model(),left.model()).is_some(),
        res.is_ok() ==> res.unwrap().model()==decoded(s@,h@,r,token,top.model(),left.model()).unwrap() && res.unwrap().valid(s@),
{
    let m=read_modes(s,h,r,top,left)?;
    let d=residual::decode(s,h,m.prefix.segment as usize,m.prefix.small,m.prefix.skip,token,top.nz,left.nz)?;
    let top=Context {modes:[m.modes[12],m.modes[13],m.modes[14],m.modes[15]],nz:d.top};
    let left=Context {modes:[m.modes[3],m.modes[7],m.modes[11],m.modes[15]],nz:d.left};
    assert(top.modes@ =~= Seq::new(4,|i:int|m.modes@[12+i]));assert(left.modes@ =~= Seq::new(4,|i:int|m.modes@[3+4*i]));
    let f=filter::prepare(h,m.prefix.segment as usize,m.prefix.small,d.empty);
    Ok(Block {modes:m,residual:d,top,left,filter:f})
}
}
