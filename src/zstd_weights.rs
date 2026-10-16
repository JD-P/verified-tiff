//! The two interleaved FSE states used for a Zstandard Huffman description.
//! Overflow terminates the last pair exactly as specified, never by accepting
//! a truncated initial state or silently extending the literal alphabet.
use super::*;
use crate::zstd_bits::{self as zb,Reverse};
use crate::zstd_table::{Table,Cell};
verus! {
#[verifier::opaque]
spec fn finish(s:Seq<u8>,t:Seq<Cell>,low:int,p:int,a:int,b:int,out:Seq<u8>,fuel:nat) -> Option<Seq<u8>> decreases fuel {
    if fuel==0 || out.len()>253 {None} else {
        let c=t[a];let emitted=out.push(c.symbol);
        if c.bits>p-low {Some(emitted.push(t[b].symbol))}
        else {
            let v=zb::value(s,p-c.bits,c.bits as nat);
            finish(s,t,low,p-c.bits,b,c.base as int+v,emitted,(fuel-1) as nat)
        }
    }
}
pub(super) closed spec fn model(s:Seq<u8>,start:int,end:int,t:Seq<Cell>,log:usize) -> Option<Seq<u8>> {
    if start>=end || s[end-1]==0 {None} else {
        let p=(end-1)*8+zb::top(s[end-1]);let low=start*8;
        if p-low<2*log {None} else {
            let a=zb::value(s,p-log,log as nat);let b=zb::value(s,p-2*log,log as nat);
            finish(s,t,low,p-2*log,a,b,Seq::empty(),255)
        }
    }
}
pub(super) fn decode(s:&[u8],start:usize,end:usize,table:&Table) -> (r:Result<([u8;256],usize),Error>)
    requires start<=end<=s.len(),end<=usize::MAX/8,table.valid(),table.ready,5<=table.log<=6,
    ensures r.is_ok()==model(s@,start as int,end as int,table.cells@,table.log).is_some(),
        r.is_ok() ==> 2<=r.unwrap().1<=255,
        r.is_ok() ==> r.unwrap().1==model(s@,start as int,end as int,table.cells@,table.log).unwrap().len(),
        r.is_ok() ==> r.unwrap().0@.take(r.unwrap().1 as int)==model(s@,start as int,end as int,table.cells@,table.log).unwrap(),
        r.is_ok() ==> forall|i:int|r.unwrap().1<=i<256 ==> r.unwrap().0@[i]==0,
{
    reveal(model);
    let mut bits=Reverse::new(s,start,end)?;
    let mut a=bits.read(s,table.log)? as usize;let mut b=bits.read(s,table.log)? as usize;
    let mut out=[0u8;256];let mut n=0usize;let mut fuel=255usize;
    assert(out@.take(0) =~= Seq::<u8>::empty());
    let ghost goal=model(s@,start as int,end as int,table.cells@,table.log);
    while fuel>0
        invariant bits.valid(s@),bits.low==start*8,bits.end==end,table.valid(),5<=table.log<=6,
            a<zb::pow(table.log as int),b<zb::pow(table.log as int),n<=254,fuel+n==255,
            goal==model(s@,start as int,end as int,table.cells@,table.log),
            goal==finish(s@,table.cells@,bits.low as int,bits.p as int,a as int,b as int,out@.take(n as int),fuel as nat),
            forall|i:int|n<=i<256 ==> out@[i]==0,
        decreases fuel,
    {
        reveal_with_fuel(finish,1);
        if n>253{return Err(Error::Invalid);}
        let c=table.cells[a];let other=table.cells[b];
        let ghost before=out@.take(n as int);
        out[n]=c.symbol;n=n+1;
        assert(out@.take(n as int) =~= before.push(c.symbol));
        if c.bits as usize>bits.p-bits.low {
            out[n]=other.symbol;n=n+1;
            assert(out@.take(n as int) =~= before.push(c.symbol).push(other.symbol));
            return Ok((out,n));
        }
        let v=bits.read(s,c.bits as usize)? as usize;
        let next=c.base as usize+v;a=b;b=next;fuel=fuel-1;
    }
    reveal_with_fuel(finish,1);Err(Error::Invalid)
}
}
