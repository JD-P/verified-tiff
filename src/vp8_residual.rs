//! Context propagation and residual reconstruction for one VP8 macroblock.
use super::*;
use crate::vp8_bool as ar;
use crate::vp8_coeff as co;
use crate::vp8_quant as q;
use crate::vp8_transform as tx;
verus! {
#[derive(Clone,Copy)]
pub(super) struct Residual {pub reader:ar::Reader,pub top:[u8;9],pub left:[u8;9],pub dc:[i32;16],pub blocks:[[i32;16];24],pub empty:bool}
pub(super) struct Model {pub reader:ar::Reader,pub top:Seq<u8>,pub left:Seq<u8>,pub dc:Seq<i32>,pub blocks:Seq<Seq<i32>>,pub empty:bool}
impl Residual {
    pub(super) open spec fn model(self) -> Model {Model {reader:self.reader,top:self.top@,left:self.left@,dc:self.dc@,blocks:Seq::new(24,|i:int|self.blocks[i]@),empty:self.empty}}
    pub(super) open spec fn valid(self,s:Seq<u8>) -> bool {
        self.reader.valid(s) && (forall|i:int|0<=i<9 ==> #[trigger] self.top@[i]<=1) && (forall|i:int|0<=i<9 ==> #[trigger] self.left@[i]<=1) &&
        (forall|i:int|0<=i<16 ==> -32768<=#[trigger] self.dc@[i]<=32767) &&
        (forall|i:int,j:int|0<=i<24 && 0<=j<16 ==> -32768<=#[trigger] self.blocks[i][j]<=32767)
    }
}
pub(super) open spec fn initial(r:ar::Reader,top:Seq<u8>,left:Seq<u8>) -> Model {Model {reader:r,top,left,dc:Seq::new(16,|i:int|0i32),blocks:Seq::new(24,|i:int|Seq::new(16,|j:int|0i32)),empty:true}}
pub(super) fn start(r:ar::Reader,top:[u8;9],left:[u8;9]) -> (v:Residual)
    ensures v.model()==initial(r,top@,left@),v.reader==r,v.top==top,v.left==left,v.empty,
        forall|i:int|0<=i<16 ==> #[trigger] v.dc@[i]==0,forall|i:int,j:int|0<=i<24 && 0<=j<16 ==> #[trigger] v.blocks[i][j]==0,
{let v=Residual {reader:r,top,left,dc:[0i32;16],blocks:[[0i32;16];24],empty:true};assert forall|i:int|0<=i<24 implies v.model().blocks[i]==initial(r,top@,left@).blocks[i] by {assert(v.blocks[i]@ =~= Seq::new(16,|j:int|0i32));}
 assert(v.model().blocks =~= initial(r,top@,left@).blocks);assert(v.dc@ =~= initial(r,top@,left@).dc);v}
pub(super) open spec fn dc_stage(s:Seq<u8>,h:Seq<i32>,segment:usize,small:bool,m:Model) -> Option<Model> {
    if small{Some(m)}else{
        let state=co::State {reader:m.reader,n:0,ctx:(m.top[8]+m.left[8]) as usize,zero:false,done:false};
        let d=co::finish(s,h,1,q::factor(h,segment,1,false),q::factor(h,segment,1,true),state,Seq::new(16,|i:int|0i32),17);
        if d.is_none(){None}else{
            let nz=if d.unwrap().0.n>0{1u8}else{0u8};
            Some(Model {reader:d.unwrap().0.reader,top:m.top.update(8,nz),left:m.left.update(8,nz),
                dc:Seq::new(16,|i:int|tx::sample(d.unwrap().1,i,true)),..m})
        }
    }
}
fn dc(s:&[u8],h:&[i32;1096],segment:usize,small:bool,m:Residual) -> (res:Result<Residual,Error>)
    requires m.valid(s@),segment<4,forall|i:int|0<=i<1096 ==> -255<=#[trigger] h@[i]<=255,
    ensures res.is_ok()==dc_stage(s@,h@,segment,small,m.model()).is_some(),
        res.is_ok() ==> res.unwrap().model()==dc_stage(s@,h@,segment,small,m.model()).unwrap() && res.unwrap().valid(s@),
{
    if small{return Ok(m);}
    let mut c=[0i32;16];assert(c@ =~= Seq::new(16,|i:int|0i32));let q0=q::scale(h,segment,1,false);let q1=q::scale(h,segment,1,true);
    let (n,r)=co::decode(s,m.reader,h,1,(m.top[8]+m.left[8]) as usize,0,q0,q1,&mut c)?;
    let d=tx::transform(&c,true);let mut v=m;v.reader=r;v.top[8]=if n>0{1}else{0};v.left[8]=if n>0{1}else{0};v.dc=d;
    assert(v.dc@ =~= Seq::new(16,|i:int|tx::sample(c@,i,true)));
    assert(v.model().blocks =~= m.model().blocks);Ok(v)
}
pub(super) open spec fn coordinates(i:usize) -> (usize,usize,usize) {
    if i<16{((i%4) as usize,(i/4) as usize,0usize)}else{
        let base=4+(i-16)/4*2;((base+i%2) as usize,(base+(i%4)/2) as usize,2usize)
    }
}
fn location(i:usize) -> (r:(usize,usize,usize))
    requires i<24,
    ensures r==coordinates(i),r.0<8,r.1<8,r.2==0 || r.2==2,
{
    if i<16{(i%4,i/4,0)}else{let base=4+(i-16)/4*2;(base+i%2,base+(i%4)/2,2)}
}
pub(super) open spec fn advance(s:Seq<u8>,h:Seq<i32>,segment:usize,small:bool,m:Model,i:usize) -> Option<Model> {
    let xy=coordinates(i);let first=if i<16 && !small{1usize}else{0usize};let kind=if i>=16{2usize}else if small{3usize}else{0usize};
    let c=Seq::new(16,|k:int|if k==0 && i<16{m.dc[i as int]}else{0i32});
    let state=co::State {reader:m.reader,n:first,ctx:(m.top[xy.0 as int]+m.left[xy.1 as int]) as usize,zero:false,done:false};
    let d=co::finish(s,h,kind,q::factor(h,segment,xy.2,false),q::factor(h,segment,xy.2,true),state,c,17);
    if d.is_none(){None}else{
        let n=d.unwrap().0.n;let nz=if n>first{1u8}else{0u8};
        Some(Model {reader:d.unwrap().0.reader,top:m.top.update(xy.0 as int,nz),left:m.left.update(xy.1 as int,nz),
            blocks:m.blocks.update(i as int,d.unwrap().1),empty:m.empty && n<=1 && d.unwrap().1[0]==0,..m})
    }
}
fn block(s:&[u8],h:&[i32;1096],segment:usize,small:bool,m:Residual,i:usize) -> (res:Result<Residual,Error>)
    requires m.valid(s@),segment<4,i<24,forall|j:int|0<=j<1096 ==> -255<=#[trigger] h@[j]<=255,
    ensures res.is_ok()==advance(s@,h@,segment,small,m.model(),i).is_some(),
        res.is_ok() ==> res.unwrap().model()==advance(s@,h@,segment,small,m.model(),i).unwrap() && res.unwrap().valid(s@),
{
    let (x,y,plane)=location(i);let first=if i<16 && !small{1usize}else{0usize};let kind=if i>=16{2usize}else if small{3usize}else{0usize};
    let mut c=[0i32;16];if i<16{c[0]=m.dc[i];}
    assert(c@ =~= Seq::new(16,|k:int|if k==0 && i<16{m.dc@[i as int]}else{0i32}));
    let q0=q::scale(h,segment,plane,false);let q1=q::scale(h,segment,plane,true);
    let (n,r)=co::decode(s,m.reader,h,kind,(m.top[x]+m.left[y]) as usize,first,q0,q1,&mut c)?;
    let mut v=m;v.reader=r;v.top[x]=if n>first{1}else{0};v.left[y]=if n>first{1}else{0};v.blocks[i]=c;v.empty=v.empty && n<=1 && c[0]==0;
    assert(v.model().blocks =~= m.model().blocks.update(i as int,c@));Ok(v)
}
#[verifier::opaque]
pub(super) open spec fn finish(s:Seq<u8>,h:Seq<i32>,segment:usize,small:bool,m:Model,i:usize,fuel:nat) -> Option<Model> decreases fuel {
    if i==24{Some(m)}else if fuel==0{None}else{
        let next=advance(s,h,segment,small,m,i);if next.is_none(){None}else{finish(s,h,segment,small,next.unwrap(),(i+1) as usize,(fuel-1) as nat)}
    }
}
pub(super) open spec fn decoded(s:Seq<u8>,h:Seq<i32>,segment:usize,small:bool,skip:bool,r:ar::Reader,top:Seq<u8>,left:Seq<u8>) -> Option<Model> {
    let m=initial(r,top,left);
    if skip{Some(Model {top:Seq::new(9,|i:int|if i==8 && small{top[i]}else{0u8}),left:Seq::new(9,|i:int|if i==8 && small{left[i]}else{0u8}),..m})}
    else{let d=dc_stage(s,h,segment,small,m);if d.is_none(){None}else{finish(s,h,segment,small,d.unwrap(),0,24)}}
}
pub(super) fn decode(s:&[u8],h:&[i32;1096],segment:usize,small:bool,skip:bool,r:ar::Reader,top:[u8;9],left:[u8;9]) -> (res:Result<Residual,Error>)
    requires r.valid(s@),segment<4,forall|j:int|0<=j<1096 ==> -255<=#[trigger] h@[j]<=255,
        forall|i:int|0<=i<9 ==> #[trigger] top@[i]<=1,forall|i:int|0<=i<9 ==> #[trigger] left@[i]<=1,
    ensures res.is_ok()==decoded(s@,h@,segment,small,skip,r,top@,left@).is_some(),
        res.is_ok() ==> res.unwrap().model()==decoded(s@,h@,segment,small,skip,r,top@,left@).unwrap() && res.unwrap().valid(s@),
{
    let mut m=start(r,top,left);assert(m.valid(s@));
    if skip{
        let mut i=0usize;
        while i<9
            invariant i<=9,m.reader==r,m.dc@==initial(r,top@,left@).dc,m.model().blocks==initial(r,top@,left@).blocks,m.empty,
                forall|j:int|0<=j<9 ==> #[trigger] m.top@[j]==(if j<i && (j!=8 || !small){0u8}else{top@[j]}),
                forall|j:int|0<=j<9 ==> #[trigger] m.left@[j]==(if j<i && (j!=8 || !small){0u8}else{left@[j]}),
                forall|j:int|0<=j<9 ==> #[trigger] top@[j]<=1,forall|j:int|0<=j<9 ==> #[trigger] left@[j]<=1,r.valid(s@),
            decreases 9-i,
        {if i!=8 || !small{m.top[i]=0;m.left[i]=0;}i=i+1;}
        assert(m.top@ =~= decoded(s@,h@,segment,small,skip,r,top@,left@).unwrap().top);
        assert(m.left@ =~= decoded(s@,h@,segment,small,skip,r,top@,left@).unwrap().left);
        assert(m.model().blocks =~= initial(r,top@,left@).blocks);return Ok(m);
    }
    m=dc(s,h,segment,small,m)?;
    let ghost goal=decoded(s@,h@,segment,small,skip,r,top@,left@);let mut i=0usize;
    loop
        invariant i<=24,m.valid(s@),segment<4,!skip,goal==decoded(s@,h@,segment,small,skip,r,top@,left@),
            goal==finish(s@,h@,segment,small,m.model(),i,(24-i) as nat),
            forall|j:int|0<=j<1096 ==> -255<=#[trigger] h@[j]<=255,
        decreases 24-i,
    {
        reveal_with_fuel(finish,1);if i==24{return Ok(m);}
        m=block(s,h,segment,small,m,i)?;i=i+1;
    }
}
}
