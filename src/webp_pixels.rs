//! VP8L pixel stream with caller-owned pixels and entropy groups. Auxiliary
//! images use the same token/cache model and finish without byte alignment.
use super::*;
use crate::deflate_bits::Bits;
use crate::webp_container::{self as wc,Config,Group,GroupModel};
use crate::webp_token::{self as token,Token};
use crate::webp as wp;
verus! {
pub(super) type Pixel=[u8;4];
pub(super) open spec fn groups(g:Seq<Group>) -> Seq<GroupModel> {Seq::new(g.len(),|i:int|g[i].model())}
pub(super) open spec fn tile_size(bits:usize) -> usize {if bits==0{1}else if bits==1{2}else if bits==2{4}else if bits==3{8}else if bits==4{16}else if bits==5{32}else if bits==6{64}else if bits==7{128}else if bits==8{256}else{512}}
#[inline(always)]
pub(super) fn size(bits:usize) -> (r:usize) requires bits<=9,ensures r==tile_size(bits),1<=r<=512,
{if bits==0{1}else if bits==1{2}else if bits==2{4}else if bits==3{8}else if bits==4{16}else if bits==5{32}else if bits==6{64}else if bits==7{128}else if bits==8{256}else{512}}
pub(super) open spec fn sampled(n:usize,bits:usize) -> usize {((n as int+tile_size(bits)-1)/(tile_size(bits) as int)) as usize}
#[inline(always)]
pub(super) fn coordinate(n:usize,bits:usize) -> (r:usize)
    requires bits<=9,
    ensures r==n as int/(tile_size(bits) as int),r<=n,
{
    let d=size(bits);
    assert(d==1usize<<bits) by(bit_vector)
        requires bits<=9,d==if bits==0{1usize}else if bits==1{2}else if bits==2{4}else if bits==3{8}else if bits==4{16}else if bits==5{32}else if bits==6{64}else if bits==7{128}else if bits==8{256}else{512};
    assert(n>>bits==n/d && n>>bits<=n) by(bit_vector) requires bits<=9,d==1usize<<bits;
    n>>bits
}
#[inline(always)]
pub(super) fn subsample(n:usize,bits:usize) -> (r:usize)
    requires 1<=n<=16384,bits<=9,
    ensures r==sampled(n,bits),1<=r<=n,
{
    let d=size(bits);(n+d-1)/d
}
pub(super) open spec fn selection(c:Config,meta:Seq<Pixel>,bits:usize,at:int) -> Option<usize> {
    if bits==0{Some(0usize)}else{
        let size=tile_size(bits) as int;let cell=(at/(c.w as int)/size)*sampled(c.w,bits)+(at%(c.w as int))/size;
        if cell>=meta.len(){None}else{Some((meta[cell]@[0] as int*256+meta[cell]@[1] as int) as usize)}
    }
}
fn select(c:Config,meta:&[Pixel],bits:usize,at:usize) -> (r:Result<usize,Error>)
    requires 1<=c.w<=16384,1<=c.h<=16384,at<c.w*c.h,bits<=9,
    ensures r.is_ok()==selection(c,meta@,bits,at as int).is_some(),r.is_ok() ==> r.unwrap()==selection(c,meta@,bits,at as int).unwrap(),
{
    if bits==0{return Ok(0);}
    let size=size(bits);let across=subsample(c.w,bits);let row=at/c.w/size;let col=at%c.w/size;
    assert(at/c.w<c.h) by(nonlinear_arith) requires at<c.w*c.h,c.w>0;
    assert(row*across+col<=268451840) by(nonlinear_arith) requires row<16384,across<=16384,col<16384;
    let cell=row*across+col;
    if cell>=meta.len(){return Err(Error::Invalid);}
    Ok(meta[cell][0] as usize*256+meta[cell][1] as usize)
}
struct Cursor {bits:Bits,left:usize,dist:usize,pos:usize,cache:[Pixel;2048]}
impl Cursor {
    spec fn model(self,out:Seq<Pixel>) -> wp::State {wp::State {pos:self.bits.pos(),left:self.left,dist:self.dist,pixels:out.take(self.pos as int),cache:self.cache@}}
    spec fn valid(self,s:Seq<u8>,out:Seq<Pixel>,total:usize) -> bool {self.bits.valid(s) && self.pos<=total && total<=out.len()}
    fn advance(&mut self,s:&[u8],c:Config,h:&Group,total:usize,out:&mut[Pixel]) -> (r:Result<(),Error>)
        requires old(self).valid(s@,old(out)@,total),c.valid(s@),h.valid(),old(self).bits.end==c.bits.end,old(self).pos<total,
        ensures final(out).len()==old(out).len(),
            r.is_ok()==wp::step(s@,c,h.model(),total as int,old(self).model(old(out)@)).is_some(),
            r.is_ok() ==> final(self).model(final(out)@)==wp::step(s@,c,h.model(),total as int,old(self).model(old(out)@)).unwrap(),
            r.is_ok() ==> final(self).valid(s@,final(out)@,total) && final(self).pos==old(self).pos+1 && final(self).bits.end==c.bits.end && final(self).bits.pos()>=old(self).bits.pos(),
            forall|i:int|total<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
    {
        reveal(wp::step);
        let t=if self.left>0{
            if self.dist==0 || self.dist>self.pos || self.left>total-self.pos{return Err(Error::Invalid);}
            Token {kind:1,len:self.left,dist:self.dist,key:0,rgba:[0u8,0,0,0]}
        }else{token::read_token(s,&mut self.bits,h,c.w,c.cache,self.pos,total-self.pos)?};
        let value=if t.kind==0{t.rgba}else if t.kind==1{out[self.pos-t.dist]}else{self.cache[t.key]};
        let ghost prefix=out@.take(self.pos as int);out[self.pos]=value;
        if c.cache>0{self.cache[wp::hash(value,c.cache)]=value;}
        self.left=t.len-1;self.dist=t.dist;self.pos=self.pos+1;
        assert(out@.take(self.pos as int) =~= prefix.push(value));Ok(())
    }
}
#[verifier::opaque]
pub(super) open spec fn finish(s:Seq<u8>,c:Config,g:Seq<GroupModel>,meta:Seq<Pixel>,bits:usize,total:int,v:wp::State,n:nat) -> Option<wp::State> decreases n {
    if n==0{if v.left==0{Some(v)}else{None}}else{
        let choice=selection(c,meta,bits,v.pixels.len() as int);
        if choice.is_none() || choice.unwrap()>=g.len(){None}else{
            let next=wp::step(s,c,g[choice.unwrap() as int],total,v);
            if next.is_none(){None}else{finish(s,c,g,meta,bits,total,next.unwrap(),(n-1) as nat)}
        }
    }
}
pub(super) fn decode(s:&[u8],c:Config,g:&[Group],meta:&[Pixel],meta_bits:usize,out:&mut[Pixel]) -> (r:Result<Bits,Error>)
    requires c.valid(s@),meta_bits<=9,old(out).len()>=c.w as int*c.h,forall|i:int|0<=i<g.len() ==> g@[i].valid(),
    ensures final(out).len()==old(out).len(),
        r.is_ok()==finish(s@,c,groups(g@),meta@,meta_bits,c.w as int*c.h,wp::initial(c.bits.pos()),(c.w as int*c.h) as nat).is_some(),
        r.is_ok() ==> r.unwrap().valid(s@) && r.unwrap().end==c.bits.end && r.unwrap().pos()>=c.bits.pos(),
        r.is_ok() ==> r.unwrap().pos()==finish(s@,c,groups(g@),meta@,meta_bits,c.w as int*c.h,wp::initial(c.bits.pos()),(c.w as int*c.h) as nat).unwrap().pos,
        r.is_ok() ==> final(out)@.take((c.w as int*c.h) as int)==finish(s@,c,groups(g@),meta@,meta_bits,c.w as int*c.h,wp::initial(c.bits.pos()),(c.w as int*c.h) as nat).unwrap().pixels,
        forall|i:int|c.w as int*c.h<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
{
    assert(c.w*c.h<=268435456) by(nonlinear_arith) requires c.w<=16384,c.h<=16384;
    let total=c.w*c.h;let mut cur=Cursor {bits:c.bits,left:0,dist:0,pos:0,cache:[[0u8;4];2048]};
    assert forall|j:int|0<=j<2048 implies cur.cache@[j]==[0u8,0,0,0] by {
        assert(cur.cache@[j]@ =~= seq![0u8,0,0,0]);
        assert([0u8,0,0,0]@ =~= seq![0u8,0,0,0]);
    }
    assert(cur.cache@ =~= Seq::new(2048,|i:int|[0u8,0,0,0]));assert(out@.take(0) =~= Seq::<Pixel>::empty());
    let ghost goal=finish(s@,c,groups(g@),meta@,meta_bits,total as int,wp::initial(c.bits.pos()),total as nat);
    while cur.pos<total
        invariant c.valid(s@),meta_bits<=9,total==c.w as int*c.h,cur.valid(s@,out@,total),cur.bits.end==c.bits.end,cur.bits.pos()>=c.bits.pos(),
            forall|i:int|0<=i<g.len() ==> g@[i].valid(),out.len()==old(out).len(),
            forall|i:int|total<=i<out.len() ==> out@[i]==old(out)@[i],
            goal==finish(s@,c,groups(g@),meta@,meta_bits,total as int,wp::initial(c.bits.pos()),total as nat),
            goal==finish(s@,c,groups(g@),meta@,meta_bits,total as int,cur.model(out@),(total-cur.pos) as nat),
        decreases total-cur.pos,
    {
        reveal_with_fuel(finish,1);let group=select(c,meta,meta_bits,cur.pos)?;
        if group>=g.len(){return Err(Error::Invalid);}
        cur.advance(s,c,&g[group],total,out)?;
    }
    reveal_with_fuel(finish,1);if cur.left!=0{return Err(Error::Invalid);}Ok(cur.bits)
}
}
