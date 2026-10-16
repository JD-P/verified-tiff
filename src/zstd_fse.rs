//! Normalized FSE distributions and decoding tables from the Zstandard format.
use super::*;
use crate::zstd_bits::{self as zb,Forward};
verus! {
pub(super) open spec fn floor(n:int) -> int {
    if n>=512 {9} else if n>=256 {8} else if n>=128 {7} else if n>=64 {6}
    else if n>=32 {5} else if n>=16 {4} else if n>=8 {3} else if n>=4 {2}
    else if n>=2 {1} else {0}
}
pub(super) fn log(n:usize) -> (r:usize)
    requires 0<n<=1023,
    ensures r==floor(n as int),r<=9,zb::pow(r as int)<=n<zb::pow(r as int+1),
{
    if n>=512 {9} else if n>=256 {8} else if n>=128 {7} else if n>=64 {6}
    else if n>=32 {5} else if n>=16 {4} else if n>=8 {3} else if n>=4 {2}
    else if n>=2 {1} else {0}
}
// Small probabilities occupy one fewer bit; -1 is the low-probability code.
pub(super) closed spec fn probability(s:Seq<u8>,p:int,end:int,remaining:int) -> Option<(int,int)> {
    let n=floor(remaining+1)+1;
    let half=zb::pow(n-1);let cut=2*half-remaining-2;
    if p+n-1>end*8 {None} else {
        let lo=zb::value(s,p,(n-1) as nat);
        if lo<cut {Some((p+n-1,lo-1))}
        else if p+n>end*8 {None} else {
            let whole=zb::value(s,p,n as nat);
            let v=if whole>=half {whole-cut}else{whole};
            Some((p+n,v-1))
        }
    }
}
fn read_probability(s:&[u8],bits:&mut Forward,remaining:usize) -> (r:Result<i16,Error>)
    requires old(bits).valid(s@),0<remaining<=512,
    ensures final(bits).valid(s@),final(bits).end==old(bits).end,
        r.is_ok()==probability(s@,old(bits).p as int,old(bits).end as int,remaining as int).is_some(),
        r.is_ok() ==> (final(bits).p as int,r.unwrap() as int)==probability(s@,old(bits).p as int,old(bits).end as int,remaining as int).unwrap(),
        r.is_ok() ==> -1<=r.unwrap()<=1023,
{
    reveal(probability);
    let n=log(remaining+1)+1;let half=zb::power(n-1) as usize;
    let cut=2*half-remaining-2;
    let at=bits.p;
    let lo=zb::read(s,at,bits.end,n-1)? as usize;
    if lo<cut {bits.p=at+n-1;return Ok((lo as i16)-1);}
    let v=bits.read(s,n)? as usize;
    let raw=if v>=half {v-cut}else{v};
    Ok((raw as i16)-1)
}
struct NModel {p:int,index:int,left:int,zero:bool,active:int,counts:Seq<i16>}
spec fn shape(m:NModel,end:int,maxsym:usize) -> bool {
    &&& 0<=m.p<=end*8
    &&& 0<=m.index<=maxsym<=64
    &&& 0<=m.left<=512 && 0<=m.active<=64
    &&& m.counts.len()==64
}
#[verifier::opaque]
spec fn next(s:Seq<u8>,end:int,maxsym:usize,m:NModel) -> Option<NModel> {
    if m.zero {
        if m.p+2>end*8 {None} else {
            let v=zb::value(s,m.p,2);
            if m.index+v>maxsym {None} else {
                Some(NModel {p:m.p+2,index:m.index+v,left:m.left,zero:v==3,active:m.active,counts:m.counts})
            }
        }
    } else if m.index>=maxsym || m.active>=64 {None} else {
        let r=probability(s,m.p,end,m.left);
        if r.is_none() {None} else {
            let (p,n)=r.unwrap();let cost=if n<0 {-n}else{n};
            if cost>m.left {None} else {
                Some(NModel {p,index:m.index+1,left:m.left-cost,zero:n==0,
                    active:m.active+if n!=0 {1int}else{0int},counts:m.counts.update(m.index,n as i16)})
            }
        }
    }
}
#[verifier::opaque]
spec fn finish(s:Seq<u8>,end:int,maxsym:usize,m:NModel,fuel:nat) -> Option<NModel> decreases fuel {
    if m.left==0 {if m.active>=2 {Some(m)}else{None}}
    else if fuel==0 {None} else {
        let a=next(s,end,maxsym,m);
        if a.is_none(){None}else{finish(s,end,maxsym,a.unwrap(),(fuel-1) as nat)}
    }
}
struct NState {bits:Forward,index:usize,left:usize,zero:bool,active:usize,counts:[i16;64]}
impl NState {
    spec fn model(self) -> NModel {NModel {p:self.bits.p as int,index:self.index as int,left:self.left as int,zero:self.zero,active:self.active as int,counts:self.counts@}}
    spec fn valid(self,s:Seq<u8>,maxsym:usize) -> bool {self.bits.valid(s) && shape(self.model(),self.bits.end as int,maxsym)}
    fn step(&mut self,s:&[u8],maxsym:usize) -> (r:Result<(),Error>)
        requires old(self).valid(s@,maxsym),old(self).left>0,
        ensures final(self).bits.end==old(self).bits.end,
            r.is_ok()==next(s@,old(self).bits.end as int,maxsym,old(self).model()).is_some(),
            r.is_ok() ==> final(self).bits.p>=old(self).bits.p,
            r.is_ok() ==> final(self).valid(s@,maxsym) && final(self).model()==next(s@,old(self).bits.end as int,maxsym,old(self).model()).unwrap(),
    {
        reveal(next);
        if self.zero {
            let v=self.bits.read(s,2)? as usize;
            if self.index+v>maxsym {return Err(Error::Invalid);}
            self.index=self.index+v;self.zero=v==3;
        } else {
            if self.index>=maxsym || self.active>=64 {return Err(Error::Invalid);}
            let n=read_probability(s,&mut self.bits,self.left)?;
            let cost=if n<0 {(-n) as usize}else{n as usize};
            if cost>self.left {return Err(Error::Invalid);}
            self.counts[self.index]=n;self.index=self.index+1;
            self.left=self.left-cost;self.zero=n==0;
            if n!=0 {self.active=self.active+1;}
        }
        Ok(())
    }
}
pub(super) struct Distribution {pub counts:[i16;64],pub log:usize,pub at:usize}
pub(super) struct DistributionModel {pub counts:Seq<i16>,pub log:int,pub at:int}
pub(super) closed spec fn distribution(s:Seq<u8>,start:int,end:int,maxlog:usize,maxsym:usize) -> Option<DistributionModel> {
    if start>=end {None} else {
        let log=s[start] as int%16+5;
        if log>maxlog {None} else {
            let m=NModel {p:start*8+4,index:0,left:zb::pow(log),zero:false,active:0,counts:Seq::new(64,|i:int|0i16)};
            let r=finish(s,end,maxsym,m,128);
            if r.is_none() {None} else {
                let out=r.unwrap();Some(DistributionModel {counts:out.counts,log,at:(out.p+7)/8})
            }
        }
    }
}
pub(super) fn parse(s:&[u8],start:usize,end:usize,maxlog:usize,maxsym:usize) -> (r:Result<Distribution,Error>)
    requires start<=end<=s.len(),end<=usize::MAX/8,5<=maxlog<=9,1<=maxsym<=64,
    ensures r.is_ok()==distribution(s@,start as int,end as int,maxlog,maxsym).is_some(),
        r.is_ok() ==> r.unwrap().counts@==distribution(s@,start as int,end as int,maxlog,maxsym).unwrap().counts,
        r.is_ok() ==> r.unwrap().log==distribution(s@,start as int,end as int,maxlog,maxsym).unwrap().log,
        r.is_ok() ==> r.unwrap().at==distribution(s@,start as int,end as int,maxlog,maxsym).unwrap().at,
        r.is_ok() ==> 5<=r.unwrap().log<=maxlog && start<r.unwrap().at<=end,
{
    reveal(distribution);
    if start==end{return Err(Error::Truncated);}
    let log=(s[start]%16) as usize+5;if log>maxlog{return Err(Error::Unsupported);}
    let mut m=NState {bits:Forward {p:start*8+4,end},index:0,left:zb::power(log) as usize,zero:false,active:0,counts:[0i16;64]};
    assert(m.counts@ =~= Seq::new(64,|i:int|0i16));
    let ghost goal=finish(s@,end as int,maxsym,m.model(),128);
    let mut fuel=128usize;
    loop
        invariant m.valid(s@,maxsym),m.bits.end==end,start*8+4<=m.bits.p,
            goal==finish(s@,end as int,maxsym,m.model(),fuel as nat),fuel<=128,
            goal.is_some()==distribution(s@,start as int,end as int,maxlog,maxsym).is_some(),
            goal.is_some() ==> distribution(s@,start as int,end as int,maxlog,maxsym).unwrap().counts==goal.unwrap().counts,
            goal.is_some() ==> distribution(s@,start as int,end as int,maxlog,maxsym).unwrap().at==(goal.unwrap().p+7)/8,
            goal.is_some() ==> distribution(s@,start as int,end as int,maxlog,maxsym).unwrap().log==log,
            5<=log<=maxlog<=9,start<end,end<=usize::MAX/8,
        decreases fuel,
    {
        reveal_with_fuel(finish,1);
        if m.left==0 {
            if m.active<2{return Err(Error::Invalid);}
            return Ok(Distribution {counts:m.counts,log,at:(m.bits.p+7)/8});
        }
        if fuel==0{return Err(Error::Invalid);}
        m.step(s,maxsym)?;fuel=fuel-1;
    }
}
}
