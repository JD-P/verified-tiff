//! VP8 binary arithmetic decoder. Range normalization is delayed until the
//! next symbol, so the final decision does not request unused padding bits.
use super::*;
verus! {
#[derive(Clone,Copy)]
pub(super) struct Reader {pub pos:usize,pub end:usize,pub range:u32,pub value:u32,pub ready:bool}
impl Reader {
    pub(super) open spec fn valid(self,s:Seq<u8>) -> bool {
        self.end<=s.len() && self.end<=usize::MAX/8 && self.pos<=self.end*8 &&
        1<=self.range<=255 && self.value<self.range &&
        (!self.ready ==> self.range==255 && self.value==0 && self.pos%8==0)
    }
}
pub(super) fn start(at:usize,end:usize) -> (r:Reader)
    requires at<=end<=usize::MAX/8,
    ensures r==(Reader {pos:(at*8) as usize,end,range:255,value:0,ready:false}),
{Reader {pos:at*8,end,range:255,value:0,ready:false}}
pub(super) open spec fn load(s:Seq<u8>,r:Reader) -> Option<Reader> {
    if r.ready{Some(r)}else if r.pos/8>=r.end || s[(r.pos as int)/8]>=255{None}else{
        Some(Reader {pos:(r.pos+8) as usize,value:s[(r.pos as int)/8] as u32,ready:true,..r})
    }
}
pub(super) open spec fn next(s:Seq<u8>,r:Reader) -> Option<Reader> {
    if r.pos>=r.end*8{None}else{
        Some(Reader {pos:(r.pos+1) as usize,range:(r.range*2) as u32,
            value:(r.value*2+(s[(r.pos as int)/8] as int/power2(7-r.pos%8))%2) as u32,..r})
    }
}
#[verifier::opaque]
pub(super) open spec fn normalize(s:Seq<u8>,r:Reader,fuel:nat) -> Option<Reader> decreases fuel {
    if r.range>=128{Some(r)}else if fuel==0{None}else{
        let n=next(s,r);if n.is_none(){None}else{normalize(s,n.unwrap(),(fuel-1) as nat)}
    }
}
pub(super) open spec fn decision(r:Reader,p:u8) -> (bool,Reader) {
    let split=1+((r.range-1)*p)/256;
    if r.value>=split {(true,Reader {value:(r.value-split) as u32,range:(r.range-split) as u32,..r})}
    else{(false,Reader {range:split as u32,..r})}
}
pub(super) open spec fn symbol(s:Seq<u8>,r:Reader,p:u8) -> Option<(bool,Reader)> {
    let l=load(s,r);if l.is_none(){None}else{
        let n=normalize(s,l.unwrap(),7);if n.is_none(){None}else{Some(decision(n.unwrap(),p))}
    }
}
pub(super) fn bit(s:&[u8],r:Reader,p:u8) -> (res:Result<(bool,Reader),Error>)
    requires r.valid(s@),
    ensures res.is_ok()==symbol(s@,r,p).is_some(),
        res.is_ok() ==> res.unwrap()==symbol(s@,r,p).unwrap() && res.unwrap().1.valid(s@) && res.unwrap().1.ready,
{
    let mut t=r;
    if !t.ready {
        if t.pos/8>=t.end || s[t.pos/8]>=255{return Err(Error::Truncated);}
        t.value=s[t.pos/8] as u32;t.pos=t.pos+8;t.ready=true;
    }
    let ghost loaded=t;let mut fuel=7usize;
    while t.range<128
        invariant t.valid(s@),t.ready,fuel<=7,
            load(s@,r).is_some(),normalize(s@,loaded,7)==normalize(s@,t,fuel as nat),loaded==load(s@,r).unwrap(),
        decreases fuel,
    {
        reveal_with_fuel(normalize,1);
        if fuel==0{return Err(Error::Invalid);}
        if t.pos>=t.end*8{return Err(Error::Truncated);}
        let d=bit_divisor(7-t.pos%8);
        let b=(s[t.pos/8]/d)%2;
        let old_t=t;
        t=Reader {pos:t.pos+1,range:t.range*2,value:t.value*2+b as u32,..t};
        assert(t==next(s@,old_t).unwrap());fuel=fuel-1;
    }
    reveal_with_fuel(normalize,1);
    assert((t.range-1)*p<=64770) by(nonlinear_arith) requires 1<=t.range<=255,p<=255;
    let split=1+((t.range-1)*p as u32)/256;
    assert(1<=split<t.range) by(nonlinear_arith) requires 128<=t.range<=255,p<=255,split==1+((t.range-1)*p)/256;
    let take=t.value>=split;
    if take {Ok((true,Reader {value:t.value-split,range:t.range-split,..t}))}
    else{Ok((false,Reader {range:split,..t}))}
}
#[verifier::opaque]
pub(super) open spec fn bits(s:Seq<u8>,r:Reader,n:nat,v:u32) -> Option<(u32,Reader)> decreases n {
    if n==0{Some((v,r))}else{
        let b=symbol(s,r,128);if b.is_none(){None}else{
            bits(s,b.unwrap().1,(n-1) as nat,(v*2+if b.unwrap().0{1int}else{0int}) as u32)
        }
    }
}
pub(super) fn read(s:&[u8],r:Reader,n:usize) -> (res:Result<(u32,Reader),Error>)
    requires r.valid(s@),n<=8,
    ensures res.is_ok()==bits(s@,r,n as nat,0).is_some(),
        res.is_ok() ==> res.unwrap()==bits(s@,r,n as nat,0).unwrap() && res.unwrap().1.valid(s@) && res.unwrap().0<crate::zstd_bits::pow(n as int),
{
    let mut t=r;let mut v=0u32;let mut i=0usize;
    loop
        invariant t.valid(s@),i<=n<=8,v<crate::zstd_bits::pow(i as int),
            bits(s@,r,n as nat,0)==bits(s@,t,(n-i) as nat,v),
        decreases n-i,
    {
        reveal_with_fuel(bits,1);if i==n{return Ok((v,t));}
        let (b,after)=bit(s,t,128)?;t=after;
        proof {crate::zstd_bits::power_step(i as int);}
        v=v*2+if b{1}else{0};i=i+1;
    }
}
}
