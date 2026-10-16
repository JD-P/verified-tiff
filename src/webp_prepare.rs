//! VP8L metadata preparation and bounded entropy-group storage.
use super::*;
use crate::deflate_bits::{self as db,Bits};
use crate::webp_container::{self as wc,Group,GroupModel};
use crate::webp_pixels::{self as pix,Pixel};
use crate::webp_transforms::{self as tr};
use crate::webp_transforms::*;
use crate::webp_aux as aux;
verus! {
#[verifier::opaque]
pub(super) open spec fn maximum(s:Seq<Pixel>,n:nat) -> usize decreases n {
    if n==0{1}else{
        let v=(s[n as int-1]@[0] as int*256+s[n as int-1]@[1] as int+1) as usize;
        let old=maximum(s,(n-1) as nat);if v>old{v}else{old}
    }
}
fn group_count(s:&[Pixel],start:usize,count:usize) -> (r:usize)
    requires start as int+count<=s.len(),
    ensures r==maximum(s@.subrange(start as int,start as int+count),count as nat),1<=r<=65536,
{
    let mut i=0usize;let mut max=1usize;reveal_with_fuel(maximum,1);
    while i<count
        invariant i<=count,start as int+count<=s.len(),1<=max<=65536,
            max==maximum(s@.subrange(start as int,start as int+count),i as nat),
        decreases count-i,
    {
        let v=s[start+i][0] as usize*256+s[start+i][1] as usize+1;
        if v>max{max=v;}i=i+1;reveal_with_fuel(maximum,1);
    }max
}
pub(super) struct Model {pub transforms:tr::Model,pub bits:Bits,pub cache:usize,pub meta_bits:usize,pub meta_start:usize,pub meta_count:usize,pub groups:usize,pub data:Seq<Pixel>}
#[verifier::opaque]
pub(super) open spec fn metadata(s:Seq<u8>,t:tr::Model,h:usize,cap:usize) -> Option<Model> {
    let cache=aux::cache(s,t.bits.pos(),t.bits.end as int);
    if cache.is_none(){None}else{
        let (p,cb)=cache.unwrap();
        if p>=t.bits.end as int*8{None}else if db::field(s,p,1)==0{
            Some(Model {transforms:t,bits:position(p+1,t.bits.end),cache:cb,meta_bits:0,meta_start:t.data.len() as usize,meta_count:0,groups:1,data:t.data})
        }else{
            let bits=(db::field(s,p+1,3)+2) as usize;let w=pix::sampled(t.width,bits);let hh=pix::sampled(h,bits);
            if p+4>t.bits.end as int*8 || t.data.len()+w as int*hh>cap{None}else{
                let data=aux::image(s,position(p+4,t.bits.end),w,hh,false);
                if data.is_none(){None}else{
                    let (end,image)=data.unwrap();
                    Some(Model {transforms:t,bits:position(end,t.bits.end),cache:cb,meta_bits:bits,meta_start:t.data.len() as usize,
                        meta_count:(w as int*hh) as usize,groups:maximum(image,image.len()),data:t.data+image})
                }
            }
        }
    }
}
pub(super) open spec fn prepared(s:Seq<u8>,bits:Bits,w:usize,h:usize,cap:usize) -> Option<Model> {
    let t=tr::finish(s,bits,w,h,0,Seq::empty(),Seq::empty(),cap,5);
    if t.is_none(){None}else{metadata(s,t.unwrap(),h,cap)}
}
pub(super) struct Prepared {pub transforms:tr::Prepared,pub bits:Bits,pub cache:usize,pub meta_bits:usize,pub meta_start:usize,pub meta_count:usize,pub groups:usize,pub used:usize}
impl Prepared {
    pub(super) open spec fn model(self,data:Seq<Pixel>) -> Model {Model {transforms:self.transforms.model(data),bits:self.bits,cache:self.cache,meta_bits:self.meta_bits,meta_start:self.meta_start,meta_count:self.meta_count,groups:self.groups,data:data.take(self.used as int)}}
    pub(super) open spec fn valid(self,s:Seq<u8>,w:usize,h:usize,cap:usize) -> bool {
        self.bits.valid(s) && self.cache<=11 && self.meta_bits<=9 && 1<=self.groups<=65536 && self.used<=cap &&
        self.meta_start==self.transforms.used && self.used==self.meta_start as int+self.meta_count &&
        self.transforms.count<=4 && 1<=self.transforms.width<=w &&
        forall|i:int|0<=i<self.transforms.count ==> self.transforms.transforms@[i].valid() && self.transforms.transforms@[i].width<=w && self.transforms.transforms@[i].height==h
    }
}
pub(super) fn prepare(s:&[u8],bits:Bits,w:usize,h:usize,out:&mut[Pixel]) -> (r:Result<Prepared,Error>)
    requires bits.valid(s@),1<=w<=16384,1<=h<=16384,
    ensures final(out).len()==old(out).len(),r.is_ok()==prepared(s@,bits,w,h,old(out).len()).is_some(),
        r.is_ok() ==> r.unwrap().model(final(out)@)==prepared(s@,bits,w,h,old(out).len()).unwrap(),
        r.is_ok() ==> r.unwrap().valid(s@,w,h,old(out).len()) && r.unwrap().bits.end==bits.end && r.unwrap().bits.pos()>=bits.pos(),
        r.is_ok() ==> forall|i:int|r.unwrap().used<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
{
    reveal(metadata);let transforms=tr::prepare(s,bits,w,h,out)?;
    let ghost original=transforms.model(out@);let ghost prior=out@.take(transforms.used as int);
    let mut b=transforms.bits;let cache=aux::read_cache(s,&mut b)?;let start=transforms.used;
    if b.read(s,1)?==0{
        assert(b==position(b.pos(),b.end)) by(nonlinear_arith) requires b.bit<8,b.pos()==b.at as int*8+b.bit;
        return Ok(Prepared {transforms,bits:b,cache,meta_bits:0,meta_start:start,meta_count:0,groups:1,used:start});
    }
    let meta_bits=b.read(s,3)?+2;let x=pix::subsample(transforms.width,meta_bits);let y=pix::subsample(h,meta_bits);
    assert(b==position(b.pos(),b.end)) by(nonlinear_arith) requires b.bit<8,b.pos()==b.at as int*8+b.bit;
    assert(x*y<=268435456) by(nonlinear_arith) requires x<=16384,y<=16384;
    let count=x*y;if count>out.len()-start{return Err(Error::OutputTooSmall);}
    let before=b;let next=aux::append(s,b,x,y,false,start,out)?;
    let groups=group_count(out,start,count);
    assert(out@.take((start+count) as int) =~= prior+aux::image(s@,before,x,y,false).unwrap().1);
    assert(out@.take(start as int) =~= prior);
    assert(transforms.model(out@)==original);
    assert(next==position(next.pos(),next.end)) by(nonlinear_arith) requires next.bit<8,next.pos()==next.at as int*8+next.bit;
    Ok(Prepared {transforms,bits:next,cache,meta_bits,meta_start:start,meta_count:count,groups,used:start+count})
}
pub(super) struct Groups {pub bits:Bits,pub tables:Seq<GroupModel>}
#[verifier::opaque]
pub(super) open spec fn group_finish(s:Seq<u8>,bits:Bits,cache:usize,prefix:Seq<GroupModel>,n:nat) -> Option<Groups> decreases n {
    if n==0{Some(Groups {bits,tables:prefix})}else{
        let g=wc::group(s,bits.pos(),bits.end as int,cache as int);
        if g.is_none(){None}else{
            let (p,t)=g.unwrap();group_finish(s,position(p,bits.end),cache,prefix.push(t),(n-1) as nat)
        }
    }
}
pub(super) fn read_groups(s:&[u8],bits:Bits,cache:usize,count:usize,out:&mut[Group]) -> (r:Result<Bits,Error>)
    requires bits.valid(s@),cache<=11,1<=count<=65536,count<=old(out).len(),
    ensures final(out).len()==old(out).len(),r.is_ok()==group_finish(s@,bits,cache,Seq::empty(),count as nat).is_some(),
        r.is_ok() ==> r.unwrap()==group_finish(s@,bits,cache,Seq::empty(),count as nat).unwrap().bits && pix::groups(final(out)@.take(count as int))==group_finish(s@,bits,cache,Seq::empty(),count as nat).unwrap().tables,
        r.is_ok() ==> r.unwrap().valid(s@) && r.unwrap().end==bits.end && r.unwrap().pos()>=bits.pos(),
        r.is_ok() ==> forall|i:int|0<=i<count ==> final(out)@[i].valid(),
        forall|i:int|count<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
{
    let mut b=bits;let mut k=0usize;let ghost goal=group_finish(s@,bits,cache,Seq::empty(),count as nat);
    assert(pix::groups(out@.take(0)) =~= Seq::<GroupModel>::empty());
    while k<count
        invariant b.valid(s@),b.end==bits.end,b.pos()>=bits.pos(),cache<=11,k<=count<=65536,count<=out.len(),out.len()==old(out).len(),
            forall|i:int|0<=i<k ==> out@[i].valid(),forall|i:int|count<=i<out.len() ==> out@[i]==old(out)@[i],
            goal==group_finish(s@,bits,cache,Seq::empty(),count as nat),goal==group_finish(s@,b,cache,pix::groups(out@.take(k as int)),(count-k) as nat),
        decreases count-k,
    {
        reveal_with_fuel(group_finish,1);let ghost before=pix::groups(out@.take(k as int));
        let g=wc::read_group(s,&mut b,cache)?;
        assert(b==position(b.pos(),b.end)) by(nonlinear_arith) requires b.bit<8,b.pos()==b.at as int*8+b.bit;
        out[k]=g;k=k+1;assert(pix::groups(out@.take(k as int)) =~= before.push(g.model()));
    }
    reveal_with_fuel(group_finish,1);Ok(b)
}
}
