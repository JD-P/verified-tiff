// VP8L single-group lossless stream. Runtime validation needs no raster history:
// control flow depends on symbols and copy distances, never on pixel values.
use super::*;
use crate::deflate_bits::Bits;
use crate::webp_container::{self as wc,Config,Group,GroupModel};
use crate::webp_token::{self as wt,Token};
verus! {
pub(super) open spec fn argb(p:[u8;4]) -> int {p@[2] as int+256*p@[1] as int+65536*p@[0] as int+16777216*p@[3] as int}
pub(super) open spec fn bucket(p:[u8;4],cb:int) -> int {
    if cb==0 {0} else {((argb(p)*506832829)%4294967296)/(4294967296int/wc::cache_size(cb))}
}
proof fn bucket_divisor(cb:usize)
    requires 1<=cb<=11,
    ensures 4294967296int/wc::cache_size(cb as int)==vstd::arithmetic::power2::pow2((32-cb) as nat),
        wc::cache_size(cb as int)==2 || wc::cache_size(cb as int)==4 || wc::cache_size(cb as int)==8 || wc::cache_size(cb as int)==16
        || wc::cache_size(cb as int)==32 || wc::cache_size(cb as int)==64 || wc::cache_size(cb as int)==128 || wc::cache_size(cb as int)==256
        || wc::cache_size(cb as int)==512 || wc::cache_size(cb as int)==1024 || wc::cache_size(cb as int)==2048,
{
    vstd::arithmetic::power2::lemma2_to64();
    if cb==1 {assert(4294967296int/2int==2147483648int) by(compute_only);} else if cb==2 {assert(4294967296int/4int==1073741824int) by(compute_only);} else if cb==3 {assert(4294967296int/8int==536870912int) by(compute_only);} else if cb==4 {assert(4294967296int/16int==268435456int) by(compute_only);} else if cb==5 {assert(4294967296int/32int==134217728int) by(compute_only);} else if cb==6 {assert(4294967296int/64int==67108864int) by(compute_only);} else if cb==7 {assert(4294967296int/128int==33554432int) by(compute_only);} else if cb==8 {assert(4294967296int/256int==16777216int) by(compute_only);} else if cb==9 {assert(4294967296int/512int==8388608int) by(compute_only);} else if cb==10 {assert(4294967296int/1024int==4194304int) by(compute_only);} else {assert(4294967296int/2048int==2097152int) by(compute_only);}
}
pub(super) fn hash(p:[u8;4],cb:usize) -> (r:usize)
    requires 1<=cb<=11,ensures r as int==bucket(p,cb as int),r<wc::cache_size(cb as int),r<2048,
{
    let color=p[2] as u64+256*(p[1] as u64)+65536*(p[0] as u64)+16777216*(p[3] as u64);
    assert(color<=4294967295);
    let sz=wc::size(cb);
    // The divisor is the power of two 2^(32-cb), so the bucket is a shift.
    let shift=(32-cb) as u64;
    let ghost divisor=4294967296int/(sz as int);
    proof {
        bucket_divisor(cb);
        assert(divisor>0 && divisor*sz==4294967296) by(nonlinear_arith)
            requires sz==2 || sz==4 || sz==8 || sz==16 || sz==32 || sz==64 || sz==128 || sz==256 || sz==512 || sz==1024 || sz==2048,divisor==4294967296int/(sz as int);
    }
    let mixed=(color*506832829)%4294967296;
    let r=mixed>>shift;
    proof {vstd::bits::lemma_u64_shr_is_div(mixed,shift);}
    assert(r as int==mixed as int/divisor);
    assert(mixed as int/divisor<sz) by(nonlinear_arith) requires mixed<4294967296,divisor>0,divisor*sz==4294967296;
    r as usize
}
pub(super) struct State {pub pos:int,pub left:usize,pub dist:usize,pub pixels:Seq<[u8;4]>,pub cache:Seq<[u8;4]>}
pub(super) open spec fn initial(p:int) -> State {State {pos:p,left:0,dist:0,pixels:Seq::empty(),cache:Seq::new(2048,|i:int|[0u8,0,0,0])}}
pub(super) open spec fn push(v:State,p:[u8;4],cb:int) -> State {
    State {pixels:v.pixels.push(p),cache:if cb==0 {v.cache} else {v.cache.update(bucket(p,cb),p)},..v}
}
pub(super) open spec fn paint(v:State,t:Token,cb:int,q:int) -> State {
    let p=if t.kind==0 {t.rgba} else if t.kind==1 {v.pixels[v.pixels.len() as int-t.dist]} else {v.cache[t.key as int]};
    push(State {pos:q,left:(t.len-1) as usize,dist:t.dist,..v},p,cb)
}
#[verifier::opaque]
pub(super) open spec fn step(s:Seq<u8>,c:Config,h:GroupModel,total:int,v:State) -> Option<State> {
    if v.pixels.len()>=total {None} else if v.left>0 {
        if v.dist==0 || v.dist>v.pixels.len() || v.left>total-v.pixels.len() {None}
        else {Some(paint(v,Token {kind:1,len:v.left,dist:v.dist,key:0,rgba:[0u8,0,0,0]},c.cache as int,v.pos))}
    } else {
        let t=wt::token(s,v.pos,c.bits.end as int,h,c.w as int,c.cache as int,v.pixels.len() as int,total-v.pixels.len());
        if t.is_none() {None} else {Some(paint(v,t.unwrap().1,c.cache as int,t.unwrap().0))}
    }
}
#[verifier::opaque]
pub(super) open spec fn finish(s:Seq<u8>,c:Config,h:GroupModel,total:int,v:State,n:nat) -> Option<State> decreases n {
    if n==0 {if v.left==0 && (v.pos+7)/8==c.bits.end {Some(v)} else {None}}
    else {let next=step(s,c,h,total,v);if next.is_none() {None} else {finish(s,c,h,total,next.unwrap(),(n-1) as nat)}}
}
pub(super) open spec fn flat(p:Seq<[u8;4]>,k:int) -> u8 {p[k/4]@[k%4]}
fn read_pixel(out:&[u8],i:usize) -> (r:[u8;4])
    requires (i as int+1)*4<=out.len(),
    ensures forall|j:int| 0<=j<4 ==> r@[j]==out@[i as int*4+j],
{
    let at=i*4;[out[at],out[at+1],out[at+2],out[at+3]]
}
fn write_pixel(out:&mut [u8],i:usize,p:[u8;4])
    requires (i as int+1)*4<=old(out).len(),
    ensures final(out).len()==old(out).len(),
        forall|k:int| 0<=k<old(out).len() ==> final(out)@[k]==(if i as int*4<=k<(i as int+1)*4 {p@[k-i as int*4]} else {old(out)@[k]}),
{
    let at=i*4;out[at]=p[0];out[at+1]=p[1];out[at+2]=p[2];out[at+3]=p[3];
}
struct Cursor {bits:Bits,left:usize,dist:usize,produced:usize,cache:[[u8;4];2048],history:Ghost<Seq<[u8;4]>>,colors:Ghost<Seq<[u8;4]>>}
impl Cursor {
    spec fn model(&self) -> State {State {pos:self.bits.pos(),left:self.left,dist:self.dist,pixels:self.history@,cache:self.colors@}}
    spec fn valid(&self,s:Seq<u8>,out:Seq<u8>,total:int,emit:bool) -> bool {
        self.bits.valid(s) && self.produced==self.history@.len() && self.produced<=total && self.colors@.len()==2048 &&
        (emit ==> out.len()>=4*total && self.cache@==self.colors@ && forall|k:int| 0<=k<4*self.produced ==> out[k]==flat(self.history@,k))
    }
    fn advance(&mut self,s:&[u8],c:Config,h:&Group,total:usize,out:&mut [u8],emit:bool) -> (r:Result<(),Error>)
        requires old(self).valid(s@,old(out)@,total as int,emit),c.valid(s@),h.valid(),old(self).bits.end==c.bits.end,old(self).produced<total,
        ensures r.is_ok()==step(s@,c,h.model(),total as int,old(self).model()).is_some(),
            r.is_ok() ==> final(self).model()==step(s@,c,h.model(),total as int,old(self).model()).unwrap(),
            r.is_ok() ==> final(self).valid(s@,final(out)@,total as int,emit) && final(self).bits.end==c.bits.end && final(self).produced==old(self).produced+1,
            final(out).len()==old(out).len(),
            forall|k:int| 4*total<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
    {
        let ghost produced_before=self.produced;
        reveal(step);
        let t=if self.left>0 {
            if self.dist==0 || self.dist>self.produced || self.left>total-self.produced {return Err(Error::Invalid);}
            Token {kind:1,len:self.left,dist:self.dist,key:0,rgba:[0u8,0,0,0]}
        } else {wt::read_token(s,&mut self.bits,h,c.w,c.cache,self.produced,total-self.produced)?};
        let ghost value=if t.kind==0 {t.rgba} else if t.kind==1 {self.history@[self.produced as int-t.dist]} else {self.colors@[t.key as int]};
        if emit {
            let p=if t.kind==0 {t.rgba} else if t.kind==1 {
                let q=read_pixel(out,self.produced-t.dist);
                assert(q==value) by {
                    assert forall|j:int| 0<=j<4 implies q@[j]==value@[j] by {
                        let k=(self.produced as int-t.dist)*4+j;
                        assert(0<=k<4*self.produced && k/4==self.produced as int-t.dist && k%4==j) by(nonlinear_arith)
                            requires t.dist>=1,t.dist<=self.produced,0<=j<4,k==(self.produced as int-t.dist)*4+j;
                        assert(out@[k]==flat(self.history@,k));
                    };assert(q@==value@);
                }q
            } else {self.cache[t.key]};
            assert(p==value);
            write_pixel(out,self.produced,p);
            if c.cache>0 {let index=hash(p,c.cache);self.cache[index]=p;}
        }
        self.left=t.len-1;self.dist=t.dist;
        proof {self.history=Ghost(self.history@.push(value));if c.cache>0 {self.colors=Ghost(self.colors@.update(bucket(value,c.cache as int),value));}}
        self.produced=self.produced+1;
        assert forall|k:int| emit && 0<=k<4*self.produced implies out@[k]==flat(self.history@,k) by {
            if k<4*old(self).produced {
                assert(0<=k/4<produced_before) by(nonlinear_arith) requires 0<=k<4*produced_before;
                assert(out@[k]==old(out)@[k]);
            } else {
                assert(k/4==produced_before && k%4==k-4*produced_before) by(nonlinear_arith)
                    requires 4*produced_before<=k<4*(produced_before+1);
            }
        }
        Ok(())
    }
}
#[verifier::opaque]
pub(super) open spec fn result(s:Seq<u8>,c:Config) -> Option<State> {
    let h=wc::group(s,c.bits.pos(),c.bits.end as int,c.cache as int);
    if h.is_none() {None} else {finish(s,c,h.unwrap().1,c.w as int*c.h,initial(h.unwrap().0),(c.w as int*c.h) as nat)}
}
pub(super) fn process(s:&[u8],c:Config,out:&mut [u8],emit:bool) -> (r:Result<(),Error>)
    requires c.valid(s@),emit ==> old(out).len()>=4*(c.w as int)*c.h,
    ensures r.is_ok()==result(s@,c).is_some(),final(out).len()==old(out).len(),
        r.is_ok() && emit ==> forall|k:int| 0<=k<4*(c.w as int)*c.h ==> final(out)@[k]==flat(result(s@,c).unwrap().pixels,k),
        forall|k:int| 4*(c.w as int)*c.h<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
{
    reveal(result);let mut bits=c.bits;let h=wc::read_group(s,&mut bits,c.cache)?;
    assert(c.w as int*c.h<=268435456) by(nonlinear_arith) requires c.w<=16384,c.h<=16384;
    let total=c.w*c.h;let mut cur=Cursor {bits,left:0,dist:0,produced:0,cache:[[0u8;4];2048],history:Ghost(Seq::empty()),colors:Ghost(Seq::new(2048,|i:int|[0u8,0,0,0]))};
    assert forall|j:int| 0<=j<2048 implies cur.cache@[j]==cur.colors@[j] by {
        assert(cur.cache@[j]@ =~= seq![0u8,0,0,0]);assert(cur.colors@[j]@ =~= seq![0u8,0,0,0]);
    }
    assert(cur.cache@ =~= cur.colors@);assert(cur.model()==initial(bits.pos()));
    assert(4*(c.w as int)*c.h==4*(total as int)) by(nonlinear_arith) requires total==c.w as int*c.h;
    let ghost expected=result(s@,c);
    while cur.produced<total
        invariant c.valid(s@),h.valid(),total==c.w as int*c.h,4*(c.w as int)*c.h==4*(total as int),cur.valid(s@,out@,total as int,emit),cur.bits.end==c.bits.end,
            out.len()==old(out).len(),emit ==> out.len()>=4*total,
            forall|k:int| 4*total<=k<out.len() ==> out@[k]==old(out)@[k],
            expected==result(s@,c),expected==finish(s@,c,h.model(),total as int,cur.model(),(total-cur.produced) as nat),
        decreases total-cur.produced,
    {
        reveal_with_fuel(finish,1);cur.advance(s,c,&h,total,out,emit)?;
    }
    reveal_with_fuel(finish,1);
    if cur.left!=0 || (cur.bits.at!=cur.bits.end && (cur.bits.bit==0 || cur.bits.end-cur.bits.at!=1)) {return Err(Error::Invalid);}
    Ok(())
}
pub(super) open spec fn transformed(s:Seq<u8>,c:Config,k:int) -> u8 {
    let p=result(s,c).unwrap().pixels[k/4];let channel=k%4;
    if c.subtract && (channel==0 || channel==2) {((p@[channel] as int+p@[1] as int)%256) as u8} else {p@[channel]}
}
pub(super) fn inverse(c:Config,out:&mut [u8])
    requires c.w>=1,c.h>=1,4*(c.w as int)*c.h<=old(out).len(),
    ensures final(out).len()==old(out).len(),
        forall|k:int| 0<=k<old(out).len() ==> final(out)@[k]==(if k<4*(c.w as int)*c.h && c.subtract && (k%4==0 || k%4==2) {((old(out)@[k] as int+old(out)@[k-k%4+1] as int)%256) as u8} else {old(out)@[k]}),
{
    assert(0<=c.w as int*c.h<=out.len() && 4*(c.w as int*c.h)==4*(c.w as int)*c.h) by(nonlinear_arith)
        requires c.w>=1,c.h>=1,4*(c.w as int)*c.h<=out.len();
    let total=c.w*c.h;let mut i=0usize;
    while i<total
        invariant i<=total,total as int==c.w as int*c.h,4*(c.w as int)*c.h==4*(total as int),4*total<=out.len(),out.len()==old(out).len(),
            forall|k:int| 0<=k<out.len() ==> out@[k]==(if k<4*i && c.subtract && (k%4==0 || k%4==2) {((old(out)@[k] as int+old(out)@[k-k%4+1] as int)%256) as u8} else {old(out)@[k]}),
        decreases total-i,
    {
        let at=4*i;
        if c.subtract {let green=out[at+1] as u16;out[at]=((out[at] as u16+green)%256) as u8;out[at+2]=((out[at+2] as u16+green)%256) as u8;}
        i=i+1;
    }
}
}
