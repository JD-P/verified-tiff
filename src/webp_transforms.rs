//! VP8L transform descriptors and nonrecursive auxiliary-image preparation.
use super::*;
use crate::deflate_bits::{self as db,Bits};
use crate::webp_pixels::{self as pix,Pixel};
use crate::webp_aux as aux;
verus! {
#[derive(Clone,Copy)]
pub(super) struct Transform {pub kind:usize,pub bits:usize,pub width:usize,pub height:usize,pub offset:usize,pub colors:usize}
impl Transform {
    pub(super) open spec fn valid(self) -> bool {self.kind<4 && 1<=self.width<=16384 && 1<=self.height<=16384 &&
        (self.kind<2 ==> 2<=self.bits<=9) && (self.kind==2 ==> self.bits==0) && (self.kind==3 ==> self.bits<=3 && 1<=self.colors<=256)}
}
pub(super) open spec fn blank() -> Transform {Transform {kind:4,bits:0,width:0,height:0,offset:0,colors:0}}
pub(super) fn empty() -> (r:Transform) ensures r==blank(),{Transform {kind:4,bits:0,width:0,height:0,offset:0,colors:0}}
#[derive(Clone,Copy)]
pub(super) struct Header {pub transform:Transform,pub x:usize,pub y:usize,pub next_width:usize,pub next:Bits}
pub(super) open spec fn position(p:int,end:usize) -> Bits {Bits {at:(p/8) as usize,bit:(p%8) as usize,end}}
#[verifier::opaque]
pub(super) open spec fn header(s:Seq<u8>,bits:Bits,w:usize,h:usize,offset:usize) -> Option<Header> {
    let p=bits.pos();let kind=db::field(s,p,2);let q=p+2;
    if q>bits.end as int*8{None}else if kind<2{
        let nb=(db::field(s,q,3)+2) as usize;
        if q+3>bits.end as int*8{None}else{Some(Header {transform:Transform {kind:kind as usize,bits:nb,width:w,height:h,offset,colors:0},
            x:pix::sampled(w,nb),y:pix::sampled(h,nb),next_width:w,next:position(q+3,bits.end)})}
    }else if kind==2{Some(Header {transform:Transform {kind:2,bits:0,width:w,height:h,offset,colors:0},x:0,y:0,next_width:w,next:position(q,bits.end)})}
    else{
        let n=(db::field(s,q,8)+1) as usize;let nb=if n>16{0usize}else if n>4{1usize}else if n>2{2usize}else{3usize};
        if q+8>bits.end as int*8{None}else{Some(Header {transform:Transform {kind:3,bits:nb,width:w,height:h,offset,colors:n},
            x:n,y:1,next_width:pix::sampled(w,nb),next:position(q+8,bits.end)})}
    }
}
pub(super) fn read_header(s:&[u8],bits:Bits,w:usize,h:usize,offset:usize) -> (r:Result<Header,Error>)
    requires bits.valid(s@),1<=w<=16384,1<=h<=16384,
    ensures r.is_ok()==header(s@,bits,w,h,offset).is_some(),r.is_ok() ==> r.unwrap()==header(s@,bits,w,h,offset).unwrap(),
        r.is_ok() ==> r.unwrap().transform.valid() && r.unwrap().next.valid(s@) && r.unwrap().next.end==bits.end && r.unwrap().next.pos()>=bits.pos(),
        r.is_ok() ==> 1<=r.unwrap().next_width<=w,
        r.is_ok() ==> r.unwrap().transform.width==w && r.unwrap().transform.height==h && r.unwrap().transform.offset==offset,
        r.is_ok() ==> r.unwrap().transform.kind==2 ==> r.unwrap().x==0 && r.unwrap().y==0,
        r.is_ok() ==> r.unwrap().transform.kind!=2 ==> 1<=r.unwrap().x<=16384 && 1<=r.unwrap().y<=16384,
        r.is_ok() ==> r.unwrap().transform.kind==3 ==> r.unwrap().y==1 && r.unwrap().x<=256,
{
    reveal(header);let mut b=bits;let kind=b.read(s,2)?;
    let (nb,n,x,y,nw)=if kind<2{
        let nb=b.read(s,3)?+2;(nb,0usize,pix::subsample(w,nb),pix::subsample(h,nb),w)
    }else if kind==2{(0usize,0usize,0usize,0usize,w)}else{
        let n=b.read(s,8)?+1;let nb=if n>16{0usize}else if n>4{1usize}else if n>2{2usize}else{3usize};
        (nb,n,n,1usize,pix::subsample(w,nb))
    };
    assert(b==position(b.pos(),b.end)) by(nonlinear_arith) requires b.bit<8,b.pos()==b.at as int*8+b.bit;
    Ok(Header {transform:Transform {kind,bits:nb,width:w,height:h,offset,colors:n},x,y,next_width:nw,next:b})
}
pub(super) open spec fn flag(kind:usize) -> usize {if kind==0{1}else if kind==1{2}else if kind==2{4}else{8}}
fn bit(kind:usize) -> (r:usize) requires kind<4,ensures r==flag(kind),r==1 || r==2 || r==4 || r==8,
{if kind==0{1}else if kind==1{2}else if kind==2{4}else{8}}
pub(super) struct Model {pub bits:Bits,pub width:usize,pub transforms:Seq<Transform>,pub data:Seq<Pixel>}
#[verifier::opaque]
pub(super) open spec fn finish(s:Seq<u8>,bits:Bits,w:usize,h:usize,seen:usize,transforms:Seq<Transform>,data:Seq<Pixel>,cap:usize,fuel:nat) -> Option<Model> decreases fuel {
    let p=bits.pos();
    if fuel==0 || p>=bits.end as int*8{None}else if db::field(s,p,1)==0{Some(Model {bits:position(p+1,bits.end),width:w,transforms,data})}
    else if transforms.len()>=4{None}else{
        let hh=header(s,position(p+1,bits.end),w,h,data.len() as usize);
        if hh.is_none(){None}else{
            let r=hh.unwrap();let tag=flag(r.transform.kind);
            if ((seen as int)/(tag as int))%2!=0 || data.len()+r.x as int*r.y>cap{None}else{
                let img=if r.transform.kind==2{Some((r.next.pos(),Seq::<Pixel>::empty()))}else{aux::image(s,r.next,r.x,r.y,r.transform.kind==3)};
                if img.is_none(){None}else{
                    let (end,pixels)=img.unwrap();finish(s,position(end,bits.end),r.next_width,h,(seen+tag) as usize,transforms.push(r.transform),data+pixels,cap,(fuel-1) as nat)
                }
            }
        }
    }
}
pub(super) struct Prepared {pub bits:Bits,pub width:usize,pub count:usize,pub transforms:[Transform;4],pub used:usize}
impl Prepared {pub(super) open spec fn model(self,scratch:Seq<Pixel>) -> Model {Model {bits:self.bits,width:self.width,transforms:self.transforms@.take(self.count as int),data:scratch.take(self.used as int)}}}
pub(super) fn prepare(s:&[u8],bits:Bits,w:usize,h:usize,out:&mut[Pixel]) -> (r:Result<Prepared,Error>)
    requires bits.valid(s@),1<=w<=16384,1<=h<=16384,
    ensures final(out).len()==old(out).len(),
        r.is_ok()==finish(s@,bits,w,h,0,Seq::empty(),Seq::empty(),old(out).len(),5).is_some(),
        r.is_ok() ==> r.unwrap().model(final(out)@)==finish(s@,bits,w,h,0,Seq::empty(),Seq::empty(),old(out).len(),5).unwrap(),
        r.is_ok() ==> r.unwrap().count<=4 && r.unwrap().used<=old(out).len() && 1<=r.unwrap().width<=w && r.unwrap().bits.valid(s@) && r.unwrap().bits.end==bits.end && r.unwrap().bits.pos()>=bits.pos(),
        r.is_ok() ==> forall|i:int|0<=i<r.unwrap().count ==> r.unwrap().transforms@[i].valid() && r.unwrap().transforms@[i].width<=w && r.unwrap().transforms@[i].height==h,
        r.is_ok() ==> forall|i:int|r.unwrap().used<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
{
    let mut b=bits;let mut width=w;let mut seen=0usize;let mut count=0usize;let mut used=0usize;
    let mut transforms=[empty();4];
    assert(transforms@.take(0) =~= Seq::<Transform>::empty());assert(out@.take(0) =~= Seq::<Pixel>::empty());
    let ghost goal=finish(s@,bits,w,h,0,Seq::empty(),Seq::empty(),out.len(),5);
    loop
        invariant bits.valid(s@),b.valid(s@),b.end==bits.end,b.pos()>=bits.pos(),1<=width<=w<=16384,1<=h<=16384,
            count<=4,seen<=15,used<=out.len(),out.len()==old(out).len(),
            forall|i:int|0<=i<count ==> transforms@[i].valid() && transforms@[i].width<=w && transforms@[i].height==h,
            forall|i:int|used<=i<out.len() ==> out@[i]==old(out)@[i],
            goal==finish(s@,bits,w,h,0,Seq::empty(),Seq::empty(),old(out).len(),5),
            goal==finish(s@,b,width,h,seen,transforms@.take(count as int),out@.take(used as int),out.len(),(5-count) as nat),
        decreases 5-count,
    {
        reveal_with_fuel(finish,1);
        if b.read(s,1)?==0{
            assert(b==position(b.pos(),b.end)) by(nonlinear_arith) requires b.bit<8,b.pos()==b.at as int*8+b.bit;
            return Ok(Prepared {bits:b,width,count,transforms,used});
        }
        if count==4{return Err(Error::Invalid);}
        assert(b==position(b.pos(),b.end)) by(nonlinear_arith) requires b.bit<8,b.pos()==b.at as int*8+b.bit;
        let hd=read_header(s,b,width,h,used)?;let tag=bit(hd.transform.kind);
        if (seen/tag)%2!=0{return Err(Error::Invalid);}
        assert(seen+tag<=15) by(nonlinear_arith) requires seen<=15,(seen/tag)%2==0,tag==1 || tag==2 || tag==4 || tag==8;
        assert(hd.x*hd.y<=268435456) by(nonlinear_arith) requires hd.x<=16384,hd.y<=16384;
        let n=hd.x*hd.y;if n>out.len()-used{return Err(Error::OutputTooSmall);}
        let ghost prior=out@.take(used as int);
        let ghost old_transforms=transforms@.take(count as int);
        b=if hd.transform.kind==2{hd.next}else{aux::append(s,hd.next,hd.x,hd.y,hd.transform.kind==3,used,out)?};
        assert(b==position(b.pos(),b.end)) by(nonlinear_arith) requires b.bit<8,b.pos()==b.at as int*8+b.bit;
        let ghost added=if hd.transform.kind==2{Seq::<Pixel>::empty()}else{aux::image(s@,hd.next,hd.x,hd.y,hd.transform.kind==3).unwrap().1};
        assert(out@.take((used+n) as int) =~= prior+added);
        transforms[count]=hd.transform;
        assert(transforms@.take((count+1) as int) =~= old_transforms.push(hd.transform));
        count=count+1;seen=seen+tag;width=hd.next_width;used=used+n;
    }
}
}
