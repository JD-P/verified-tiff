// Bounded sequential scan interpreter. Mathematical history contains coefficients;
// the executable keeps only predictors and a single block.
use super::*;
use crate::jpeg_bits::Bits;
use crate::jpeg_header::*;
use crate::jpeg_huffman::{Huff,fast};
use crate::jpeg_block as b;
verus! {
pub(super) open spec fn across(v:Seq<usize>) -> int {(v[12]+8*v[15]-1)/(8*v[15])}
pub(super) open spec fn per_mcu(v:Seq<usize>) -> int {if v[14]==1 {1} else {v[15]*v[16]+2}}
pub(super) open spec fn total(v:Seq<usize>) -> int {across(v)*((v[13]+8*v[16]-1)/(8*v[16]))*per_mcu(v)}
pub(super) fn block_count(v:[usize;32]) -> (r:Result<usize,Error>)
    requires frame_ok(v@),ensures r.is_ok()==(total(v@)<=usize::MAX),r.is_ok() ==> r.unwrap()==total(v@),
{
    let hs=v[15]*8;let vs=v[16]*8;let x=(v[12]+hs-1)/hs;let y=(v[13]+vs-1)/vs;
    let blocks=if v[14]==1 {1} else {v[15]*v[16]+2};
    assert(x as int*y as int<=x as int*y as int*blocks as int) by(nonlinear_arith) requires blocks>=1;
    mul(mul(x,y)?,blocks)
}
pub(super) open spec fn component(v:Seq<usize>,k:int) -> int {let t=k%per_mcu(v);if t<v[15]*v[16] {0} else {t-v[15]*v[16]+1}}
pub(super) open spec fn reset(v:Seq<usize>,k:int) -> bool {
    k>0 && k%per_mcu(v)==0 && v[29]>0 && (k/per_mcu(v))%(v[29] as int)==0
}
pub(super) open spec fn padding(bits:Bits) -> bool {(bits.byte as int)%crate::deflate_bits::pow(bits.left as int)==crate::deflate_bits::pow(bits.left as int)-1}
#[verifier::opaque]
pub(super) open spec fn restart(s:Seq<u8>,v:Seq<usize>,bits:Bits,k:int) -> Option<Bits> {
    let per=per_mcu(v);let mcu=k/per;
    if k>0 && k%per==0 && v[29]>0 && mcu%(v[29] as int)==0 {
        if !padding(bits) || bits.at as int+2>bits.end || s[bits.at as int]!=255 || s[bits.at as int+1]!=208+(mcu/(v[29] as int)-1)%8 {None}
        else {Some(Bits {at:(bits.at+2) as usize,left:0,byte:0,..bits})}
    } else {Some(bits)}
}
fn read_restart(s:&[u8],v:[usize;32],bits:Bits,k:usize) -> (r:Result<Bits,Error>)
    requires frame_ok(v@),bits.valid(s@),
    ensures r.is_ok()==restart(s@,v@,bits,k as int).is_some(),r.is_ok() ==> r.unwrap()==restart(s@,v@,bits,k as int).unwrap() && r.unwrap().valid(s@),
{
    reveal(restart);
    let per=if v[14]==1 {1} else {v[15]*v[16]+2};let mcu=k/per;
    if k>0 && k%per==0 && v[29]>0 && mcu%v[29]==0 {
        assert(mcu>0) by(nonlinear_arith) requires k>0,per>0,k%per==0,mcu==k/per;
        assert(mcu/v@[29]>0) by(nonlinear_arith) requires mcu>0,v@[29]>0,mcu%v@[29]==0;
        let modulus=crate::jpeg_bits::power(bits.left);
        if (bits.byte as usize)%modulus!=modulus-1 || bits.end-bits.at<2 || s[bits.at]!=255 || s[bits.at+1] as usize!=208+(mcu/v[29]-1)%8 {return Err(Error::Invalid);}
        Ok(Bits {at:bits.at+2,left:0,byte:0,..bits})
    } else {Ok(bits)}
}
pub(super) struct Model {pub bits:Bits,pub pred:Seq<i64>,pub blocks:Seq<Seq<i64>>}
pub(super) open spec fn initial(bits:Bits) -> Model {Model {bits,pred:seq![0i64,0,0],blocks:Seq::empty()}}
#[verifier::opaque]
pub(super) open spec fn step(s:Seq<u8>,v:Seq<usize>,m:Model) -> Option<Model> {
    let k=m.blocks.len() as int;let c=component(v,k);let q=restart(s,v,m.bits,k);
    let pred=if reset(v,k) {seq![0i64,0,0]} else {m.pred};
    if q.is_none() {None} else {
        let out=b::finish(s,v[v[20+c] as int] as int,Huff {base:v[4+v[23+c]]},Huff {base:v[8+v[26+c]]},b::initial(q.unwrap(),pred[c]),65);
        if out.is_none() {None} else {let o=out.unwrap();Some(Model {bits:o.bits,pred:pred.update(c,o.pred),blocks:m.blocks.push(o.coef)})}
    }
}
pub(super) open spec fn terminated(s:Seq<u8>,bits:Bits,legacy:bool) -> bool {
    (legacy || padding(bits)) &&
    ((legacy && bits.at==bits.end) || (bits.at as int+2==bits.end && s[bits.at as int]==255 && s[bits.at as int+1]==217))
}
#[verifier::opaque]
pub(super) open spec fn finish(s:Seq<u8>,v:Seq<usize>,m:Model,n:nat,legacy:bool) -> Option<Model> decreases n {
    if n==0 {if terminated(s,m.bits,legacy) {Some(m)} else {None}}
    else {let a=step(s,v,m);if a.is_none() {None} else {finish(s,v,a.unwrap(),(n-1) as nat,legacy)}}
}
/// Lookahead tables for each component's DC and AC codes. Short scans keep
/// all-zero tables, which select the bit-serial decoder.
pub(super) struct Tables {pub dc0:[u16;256],pub ac0:[u16;256],pub dc1:[u16;256],pub ac1:[u16;256],pub dc2:[u16;256],pub ac2:[u16;256]}
impl Tables {
    pub(super) open spec fn ready(&self,s:Seq<u8>,v:Seq<usize>) -> bool {
        &&& crate::jpeg_huffman::fast_ok(s,Huff {base:v[4+v[23]]},self.dc0@) && crate::jpeg_huffman::fast_ok(s,Huff {base:v[8+v[26]]},self.ac0@)
        &&& v[14]==3 ==> crate::jpeg_huffman::fast_ok(s,Huff {base:v[4+v[24]]},self.dc1@) && crate::jpeg_huffman::fast_ok(s,Huff {base:v[8+v[27]]},self.ac1@)
            && crate::jpeg_huffman::fast_ok(s,Huff {base:v[4+v[25]]},self.dc2@) && crate::jpeg_huffman::fast_ok(s,Huff {base:v[8+v[28]]},self.ac2@)
    }
    pub(super) fn new(s:&[u8],v:[usize;32],count:usize) -> (r:Self)
        requires scan_ok(s@,v@),
        ensures r.ready(s@,v@),
    {
        let z=[0u16;256];
        let mut t=Tables {dc0:z,ac0:z,dc1:z,ac1:z,dc2:z,ac2:z};
        if count<64 {return t;}
        t.dc0=fast(s,Huff {base:v[4+v[23]]});t.ac0=fast(s,Huff {base:v[8+v[26]]});
        if v[14]==3 {
            t.dc1=if v[4+v[24]]==v[4+v[23]] {t.dc0} else {fast(s,Huff {base:v[4+v[24]]})};
            t.ac1=if v[8+v[27]]==v[8+v[26]] {t.ac0} else {fast(s,Huff {base:v[8+v[27]]})};
            t.dc2=if v[4+v[25]]==v[4+v[24]] {t.dc1} else {fast(s,Huff {base:v[4+v[25]]})};
            t.ac2=if v[8+v[28]]==v[8+v[27]] {t.ac1} else {fast(s,Huff {base:v[8+v[28]]})};
        }
        t
    }
}
pub(super) struct Cursor {pub bits:Bits,pub pred:[i64;3],pub produced:usize,pub history:Ghost<Seq<Seq<i64>>>}
impl Cursor {
    pub(super) open spec fn model(&self) -> Model {Model {bits:self.bits,pred:self.pred@,blocks:self.history@}}
    pub(super) open spec fn valid(&self,s:Seq<u8>) -> bool {
        self.bits.valid(s) && self.produced==self.history@.len() &&
        (forall|c:int| 0<=c<3 ==> -2048<=#[trigger] self.pred@[c]<=2047)
    }
    pub(super) fn new(bits:Bits) -> (r:Self)
        ensures r.model()==initial(bits),r.produced==0,r.bits==bits,
            forall|c:int| 0<=c<3 ==> r.pred@[c]==0,
    {
        let out=Cursor {bits,pred:[0i64;3],produced:0,history:Ghost(Seq::empty())};
        assert(out.pred@ =~= seq![0i64,0,0]);out
    }
    pub(super) fn next(&mut self,s:&[u8],v:[usize;32],t:&Tables) -> (r:Result<[i64;64],Error>)
        requires old(self).valid(s@),scan_ok(s@,v@),old(self).produced<usize::MAX,t.ready(s@,v@),
        ensures r.is_ok()==step(s@,v@,old(self).model()).is_some(),
            r.is_ok() ==> final(self).valid(s@) && final(self).model()==step(s@,v@,old(self).model()).unwrap(),
            r.is_ok() ==> final(self).produced==old(self).produced+1 && final(self).history@==old(self).history@.push(r.unwrap()@),
            r.is_ok() ==> forall|i:int| 0<=i<64 ==> -524288<=#[trigger] r.unwrap()@[i]<=524288,
    {
        reveal(step);
        let per=if v[14]==1 {1} else {v[15]*v[16]+2};let pos=self.produced%per;
        let c=if pos<v[15]*v[16] {0} else {pos-v[15]*v[16]+1};
        assert(c<v@[14]);
        let bits=read_restart(s,v,self.bits,self.produced)?;
        let dc=Huff {base:v[4+v[23+c]]};let ac=Huff {base:v[8+v[26+c]]};let qt=v[v[20+c]];
        assert(crate::jpeg_header::quant_ok(s@,qt as int,s@.len() as int));
        let mut pred=self.pred;
        if self.produced>0 && self.produced%per==0 && v[29]>0 && (self.produced/per)%v[29]==0 {pred=[0i64;3];}
        assert(pred@==if reset(v@,self.produced as int) {seq![0i64,0,0]} else {self.pred@});
        let (dt,at)=if c==0 {(&t.dc0,&t.ac0)} else if c==1 {(&t.dc1,&t.ac1)} else {(&t.dc2,&t.ac2)};
        let out=b::block(s,bits,qt,dc,ac,pred[c],dt,at)?;
        self.pred=pred;
        self.bits=out.bits;self.pred[c]=out.pred;self.produced=self.produced+1;
        proof {self.history=Ghost(self.history@.push(out.coef@));}
        Ok(out.coef)
    }
    pub(super) fn end(&self,s:&[u8],legacy:bool) -> (r:bool)
        requires self.valid(s@),ensures r==terminated(s@,self.bits,legacy),
    {
        let b=self.bits;let m=crate::jpeg_bits::power(b.left);
        (legacy || (b.byte as usize)%m==m-1) &&
        ((legacy && b.at==b.end) || (b.end-b.at==2 && s[b.at]==255 && s[b.at+1]==217))
    }
}
pub(super) fn validate(s:&[u8],v:[usize;32],bits:Bits,count:usize,legacy:bool) -> (r:Result<(),Error>)
    requires scan_ok(s@,v@),bits.valid(s@),
    ensures r.is_ok()==finish(s@,v@,initial(bits),count as nat,legacy).is_some(),
{
    let t=Tables::new(s,v,count);
    let mut c=Cursor::new(bits);let mut i=0usize;
    let ghost result=finish(s@,v@,initial(bits),count as nat,legacy);
    while i<count
        invariant c.valid(s@),i==c.produced,i<=count,scan_ok(s@,v@),t.ready(s@,v@),
            result==finish(s@,v@,initial(bits),count as nat,legacy),
            result==finish(s@,v@,c.model(),(count-i) as nat,legacy),
        decreases count-i,
    {
        reveal_with_fuel(finish,1);c.next(s,v,&t)?;i=i+1;
    }
    reveal_with_fuel(finish,1);
    if c.end(s,legacy) {Ok(())} else {Err(Error::Invalid)}
}
pub(super) proof fn finish_prefix(s:Seq<u8>,v:Seq<usize>,m:Model,n:nat,legacy:bool)
    requires finish(s,v,m,n,legacy).is_some(),
    ensures finish(s,v,m,n,legacy).unwrap().blocks.len()==m.blocks.len()+n,
        forall|k:int| 0<=k<m.blocks.len() ==> finish(s,v,m,n,legacy).unwrap().blocks[k]==m.blocks[k],
    decreases n,
{
    reveal_with_fuel(finish,1);
    if n>0 {
        reveal(step);
        let a=step(s,v,m).unwrap();finish_prefix(s,v,a,(n-1) as nat,legacy);
        assert forall|k:int| 0<=k<m.blocks.len() implies finish(s,v,m,n,legacy).unwrap().blocks[k]==m.blocks[k] by {
            assert(a.blocks[k]==m.blocks[k]);
            assert(finish(s,v,a,(n-1) as nat,legacy).unwrap().blocks[k]==a.blocks[k]);
        }
    }
}
pub(super) fn decode_blocks(s:&[u8],v:[usize;32],bits:Bits,count:usize,legacy:bool,output:&mut [u8]) -> (r:Result<(),Error>)
    requires scan_ok(s@,v@),bits.valid(s@),count as int*64<=old(output).len(),
        finish(s@,v@,initial(bits),count as nat,legacy).is_some(),
    ensures r.is_ok(),final(output).len()==old(output).len(),
        forall|k:int| 0<=k<count as int*64 ==> final(output)@[k]==crate::jpeg_transform::sample(finish(s@,v@,initial(bits),count as nat,legacy).unwrap().blocks[k/64],k%64),
        forall|k:int| count as int*64<=k<old(output).len() ==> final(output)@[k]==old(output)@[k],
{
    let t=Tables::new(s,v,count);
    let mut c=Cursor::new(bits);let mut i=0usize;
    let ghost result=finish(s@,v@,initial(bits),count as nat,legacy);
    let outlen=output.len();
    while i<count
        invariant c.valid(s@),i==c.produced,i<=count,scan_ok(s@,v@),t.ready(s@,v@),count as int*64<=output.len(),output.len()==old(output).len(),
            result==finish(s@,v@,initial(bits),count as nat,legacy),result.is_some(),
            result==finish(s@,v@,c.model(),(count-i) as nat,legacy),
            forall|k:int| 0<=k<i as int*64 ==> output@[k]==crate::jpeg_transform::sample(result.unwrap().blocks[k/64],k%64),
            forall|k:int| i as int*64<=k<output.len() ==> output@[k]==old(output)@[k],
        decreases count-i,
    {
        reveal_with_fuel(finish,1);
        let coef=c.next(s,v,&t)?;
        assert(result==finish(s@,v@,c.model(),(count-i-1) as nat,legacy));
        proof {finish_prefix(s@,v@,c.model(),(count-i-1) as nat,legacy);}
        assert(c.history@[i as int]==coef@);
        assert(result.unwrap().blocks[i as int]==c.history@[i as int]);
        assert(result.unwrap().blocks[i as int]==coef@);
        let values=crate::jpeg_transform::idct(coef);let mut j=0usize;
        while j<64
            invariant j<=64,i<count,count as int*64<=output.len(),output.len()==old(output).len(),
                forall|k:int| 0<=k<i as int*64 ==> output@[k]==crate::jpeg_transform::sample(result.unwrap().blocks[k/64],k%64),
                forall|k:int| 0<=k<64 ==> values@[k]==crate::jpeg_transform::sample(result.unwrap().blocks[i as int],k),
                forall|k:int| 0<=k<j ==> output@[i as int*64+k]==values@[k],
                forall|k:int| i as int*64+j<=k<output.len() ==> output@[k]==old(output)@[k],
            decreases 64-j,
        {output[i*64+j]=values[j];j=j+1;}
        assert forall|k:int| 0<=k<(i as int+1)*64 implies output@[k]==crate::jpeg_transform::sample(result.unwrap().blocks[k/64],k%64) by {
            if k>=i as int*64 {assert(k/64==i && k%64==k-i as int*64) by(nonlinear_arith) requires i as int*64<=k<(i as int+1)*64;}
        }
        i=i+1;
    }
    Ok(())
}
}
