// Baseline sequential JPEG block syntax, zig-zag order and DC prediction.
use super::*;
use crate::jpeg_bits::{Bits,read};
use crate::jpeg_bits::*;
use crate::jpeg_huffman::*;
use crate::deflate_bits as db;
verus! {
pub(super) open spec fn zig(k:int) -> int {
    seq![0int,1,8,16,9,2,3,10,17,24,32,25,18,11,4,5,12,19,26,33,40,48,41,34,27,20,13,6,7,14,21,28,35,42,49,56,57,50,43,36,29,22,15,23,30,37,44,51,58,59,52,45,38,31,39,46,53,60,61,54,47,55,62,63][k]
}
#[inline(always)]
fn zigzag(k:usize) -> (r:usize) requires k<64,ensures r==zig(k as int),r<64, {
    let a=&[0usize,1,8,16,9,2,3,10,17,24,32,25,18,11,4,5,12,19,26,33,40,48,41,34,27,20,13,6,7,14,21,28,35,42,49,56,57,50,43,36,29,22,15,23,30,37,44,51,58,59,52,45,38,31,39,46,53,60,61,54,47,55,62,63];a[k]
}
pub(super) open spec fn extend(v:int,n:int) -> int {if n==0 {0} else if v<db::pow(n-1) {v+1-db::pow(n)} else {v}}
#[inline(always)]
fn amplitude(s:&[u8],b:Bits,n:usize) -> (r:Result<(i64,Bits),Error>)
    requires b.valid(s@),n<=11,
    ensures r.is_ok()==field(s@,b,n as nat).is_some(),
        r.is_ok() ==> r.unwrap().0 as int==extend(field(s@,b,n as nat).unwrap().0 as int,n as int),
        r.is_ok() ==> r.unwrap().1==field(s@,b,n as nat).unwrap().1 && r.unwrap().1.valid(s@),
        r.is_ok() ==> -2047<=r.unwrap().0<=2047,
{
    let (v,q)=read_fast(s,b,n)?;
    if n==0 {return Ok((0,q));}
    let mid=crate::jpeg_bits::power(n-1);let full=crate::jpeg_bits::power(n);
    Ok((if v<mid {v as i64+1-full as i64} else {v as i64},q))
}
pub(super) struct Model {pub bits:Bits,pub coef:Seq<i64>,pub pred:i64,pub k:usize,pub done:bool}
pub(super) open spec fn initial(b:Bits,pred:i64) -> Model {Model {bits:b,coef:Seq::new(64,|i:int| 0i64),pred,k:0,done:false}}
#[verifier::opaque]
pub(super) open spec fn step(s:Seq<u8>,qt:int,dc:Huff,ac:Huff,m:Model) -> Option<Model> {
    let sym=symbol(s,m.bits,if m.k==0 {dc.base as int} else {ac.base as int},1,0,0,0);
    if sym.is_none() || m.done || m.k>=64 {None} else {
        let (code,b)=sym.unwrap();
        if m.k==0 {
            let a=field(s,b,code as nat);
            if code>11 || a.is_none() {None} else {
                let (v,q)=a.unwrap();let p=m.pred as int+extend(v as int,code as int);
                if p < -2048 || p>2047 {None} else {
                    Some(Model {bits:q,coef:m.coef.update(0,(p*s[qt] as int) as i64),pred:p as i64,k:1,done:false})
                }
            }
        } else if code==0 {Some(Model {bits:b,done:true,..m})}
        else if code==240 {
            if m.k+16>64 {None} else {Some(Model {bits:b,k:(m.k+16) as usize,done:m.k+16==64,..m})}
        } else {
            let run=code as int/16;let n=code as int%16;let k=m.k+run;let a=field(s,b,n as nat);
            if n==0 || n>10 || k>=64 || a.is_none() {None} else {
                let (v,q)=a.unwrap();let value=extend(v as int,n)*s[qt+k] as int;
                Some(Model {bits:q,coef:m.coef.update(zig(k),value as i64),k:(k+1) as usize,done:k+1==64,..m})
            }
        }
    }
}
#[verifier::opaque]
pub(super) open spec fn finish(s:Seq<u8>,qt:int,dc:Huff,ac:Huff,m:Model,fuel:nat) -> Option<Model> decreases fuel {
    if m.done {Some(m)} else if fuel==0 {None} else {
        let a=step(s,qt,dc,ac,m);if a.is_none() {None} else {finish(s,qt,dc,ac,a.unwrap(),(fuel-1) as nat)}
    }
}
pub(super) struct Block {pub bits:Bits,pub coef:[i64;64],pub pred:i64,pub k:usize,pub done:bool}
impl Block {
    pub(super) open spec fn model(&self) -> Model {Model {bits:self.bits,coef:self.coef@,pred:self.pred,k:self.k,done:self.done}}
    pub(super) open spec fn valid(&self,s:Seq<u8>) -> bool {
        self.bits.valid(s) && self.k<=64 && -2048<=self.pred<=2047 &&
        (forall|i:int| 0<=i<64 ==> -524288<=#[trigger] self.coef@[i]<=524288)
    }
    #[inline(always)]
    fn advance(&mut self,s:&[u8],qt:usize,dc:Huff,ac:Huff,dt:&[u16;256],at:&[u16;256]) -> (r:Result<(),Error>)
        requires old(self).valid(s@),range_ok(s.len() as int,qt as int,64),dc.valid(s@),ac.valid(s@),fast_ok(s@,dc,dt@),fast_ok(s@,ac,at@),
        ensures r.is_ok()==step(s@,qt as int,dc,ac,old(self).model()).is_some(),
            r.is_ok() ==> final(self).model()==step(s@,qt as int,dc,ac,old(self).model()).unwrap(),
            r.is_ok() ==> final(self).valid(s@),
    {
        reveal(step);
        if self.done || self.k>=64 {return Err(Error::Invalid);}
        let (h,t)=if self.k==0 {(dc,dt)} else {(ac,at)};let (code,b)=decode_fast(s,self.bits,h,t)?;
        if self.k==0 {
            if code>11 {return Err(Error::Invalid);}
            let (v,q)=amplitude(s,b,code as usize)?;
            let p=self.pred+v;
            if p < -2048 || p>2047 {return Err(Error::Invalid);}
            let quant=s[qt] as i64;
            assert(-524288<=p as int*quant as int<=524288) by(nonlinear_arith) requires -2048<=p<=2047,0<=quant<=255;
            self.coef[0]=p*quant;self.pred=p;self.k=1;self.bits=q;
        } else if code==0 {self.bits=b;self.done=true;}
        else if code==240 {
            if self.k>48 {return Err(Error::Invalid);}
            self.k=self.k+16;self.done=self.k==64;self.bits=b;
        } else {
            let run=(code/16) as usize;let n=(code%16) as usize;let k=self.k+run;
            if n==0 || n>10 || k>=64 {return Err(Error::Invalid);}
            let (v,q)=amplitude(s,b,n)?;let quant=s[qt+k] as i64;
            assert(-524288<=v as int*quant as int<=524288) by(nonlinear_arith) requires -2047<=v<=2047,0<=quant<=255;
            let at=zigzag(k);self.coef[at]=v*quant;self.k=k+1;self.done=self.k==64;self.bits=q;
        }
        Ok(())
    }
}
pub(super) fn block(s:&[u8],bits:Bits,qt:usize,dc:Huff,ac:Huff,pred:i64,dt:&[u16;256],at:&[u16;256]) -> (r:Result<Block,Error>)
    requires bits.valid(s@),range_ok(s.len() as int,qt as int,64),dc.valid(s@),ac.valid(s@),-2048<=pred<=2047,fast_ok(s@,dc,dt@),fast_ok(s@,ac,at@),
    ensures r.is_ok()==finish(s@,qt as int,dc,ac,initial(bits,pred),65).is_some(),
        r.is_ok() ==> r.unwrap().valid(s@) && r.unwrap().done && r.unwrap().model()==finish(s@,qt as int,dc,ac,initial(bits,pred),65).unwrap(),
{
    let mut b=Block {bits,coef:[0i64;64],pred,k:0,done:false};let mut left=65usize;
    let ghost result=finish(s@,qt as int,dc,ac,initial(bits,pred),65);
    assert(b.coef@ =~= Seq::new(64,|i:int| 0i64));
    assert(b.model()==initial(bits,pred));
    while !b.done
        invariant b.valid(s@),left<=65,range_ok(s.len() as int,qt as int,64),dc.valid(s@),ac.valid(s@),fast_ok(s@,dc,dt@),fast_ok(s@,ac,at@),
            result==finish(s@,qt as int,dc,ac,initial(bits,pred),65),
            result==finish(s@,qt as int,dc,ac,b.model(),left as nat),
        decreases left,
    {
        reveal_with_fuel(finish,1);
        if left==0 {return Err(Error::Invalid);}
        b.advance(s,qt,dc,ac,dt,at)?;left=left-1;
    }
    reveal_with_fuel(finish,1);Ok(b)
}
}
