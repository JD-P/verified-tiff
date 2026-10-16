// Bounded Group 3 row markers and optional Group 3/4 termination markers.
use super::*;
use crate::fax_bits as fb;
use crate::fax_bits::Bits;
verus! {
#[verifier::opaque]
pub(super) open spec fn eol(s:Seq<u8>,b:Bits,zeros:usize,fuel:nat) -> Option<Bits> decreases fuel {
    if fuel==0 {None} else {let a=fb::one(s,b);if a.is_none() {None} else {
        let (v,q)=a.unwrap();if v==1 {if zeros>=11 {Some(q)} else {None}}
        else {eol(s,q,(zeros+1) as usize,(fuel-1) as nat)}
    }}
}
pub(super) fn read_eol(s:&[u8],b:Bits) -> (r:Result<Bits,Error>)
    requires b.valid(s@),ensures r.is_ok()==eol(s@,b,0,19).is_some(),
        r.is_ok() ==> r.unwrap()==eol(s@,b,0,19).unwrap() && r.unwrap().valid(s@) && r.unwrap().end==b.end && r.unwrap().lsb==b.lsb && b.pos+12<=r.unwrap().pos<=b.pos+19,
{
    let mut q=b;let mut zeros=0usize;let ghost result=eol(s@,b,0,19);
    while zeros<19
        invariant zeros<=19,q.valid(s@),q.end==b.end,q.lsb==b.lsb,q.pos==b.pos+zeros,result==eol(s@,b,0,19),result==eol(s@,q,zeros,(19-zeros) as nat),
        decreases 19-zeros,
    {
        reveal_with_fuel(eol,1);let (v,p)=fb::bit(s,q)?;
        if v==1 {if zeros>=11 {return Ok(p);} else {return Err(Error::Invalid);}}
        q=p;zeros=zeros+1;
    }
    reveal_with_fuel(eol,1);Err(Error::Invalid)
}
#[verifier::opaque]
pub(super) open spec fn zero_bits(s:Seq<u8>,b:Bits,n:nat) -> bool decreases n {
    if n==0 {true} else {let a=fb::one(s,b);a.is_some() && a.unwrap().0==0 && zero_bits(s,a.unwrap().1,(n-1) as nat)}
}
pub(super) fn zeros(s:&[u8],b:Bits,n:usize) -> (r:bool)
    requires b.valid(s@),ensures r==zero_bits(s@,b,n as nat),
{
    let mut q=b;let mut left=n;
    let ghost result=zero_bits(s@,b,n as nat);
    while left>0
        invariant left<=n,q.valid(s@),result==zero_bits(s@,b,n as nat),result==zero_bits(s@,q,left as nat),
        decreases left,
    {
        reveal_with_fuel(zero_bits,1);
        let a=fb::bit(s,q);if a.is_err() {return false;}let (v,p)=a.unwrap();if v!=0 {return false;}q=p;left=left-1;
    }
    reveal_with_fuel(zero_bits,1);true
}
#[derive(Clone,Copy)]
pub(super) struct Config {pub group4:bool,pub mixed:bool,pub aligned:bool,pub missing:bool}
pub(super) open spec fn line(s:Seq<u8>,b:Bits,c:Config) -> Option<(Bits,bool)> {
    if c.group4 {Some((b,true))} else if c.missing {
        let n=if c.aligned {(8-b.pos as int%8)%8}else{0int};
        if n>b.end-b.pos || !zero_bits(s,b,n as nat){None}else{Some((Bits{pos:(b.pos+n) as usize,..b},false))}
    } else {
        let a=eol(s,b,0,19);if a.is_none() || (c.aligned && a.unwrap().pos%8!=0) {None}
        else if !c.mixed {Some((a.unwrap(),false))} else {
            let t=fb::one(s,a.unwrap());if t.is_none() {None} else {Some((t.unwrap().1,t.unwrap().0==0))}
        }
    }
}
pub(super) fn begin_line(s:&[u8],b:Bits,c:Config) -> (r:Result<(Bits,bool),Error>)
    requires b.valid(s@),ensures r.is_ok()==line(s@,b,c).is_some(),
        r.is_ok() ==> r.unwrap()==line(s@,b,c).unwrap() && r.unwrap().0.valid(s@) && r.unwrap().0.end==b.end && r.unwrap().0.lsb==b.lsb,
{
    if c.group4 {return Ok((b,true));}if c.missing {
        let n=if c.aligned{(8-b.pos%8)%8}else{0};
        if n>b.end-b.pos || !zeros(s,b,n){return Err(Error::Invalid);}
        return Ok((Bits{pos:b.pos+n,..b},false));
    }
    let q=read_eol(s,b)?;if c.aligned && q.pos%8!=0 {return Err(Error::Invalid);}
    if !c.mixed {Ok((q,false))} else {let (v,p)=fb::bit(s,q)?;Ok((p,v==0))}
}
pub(super) open spec fn padding(s:Seq<u8>,b:Bits) -> bool {b.end-b.pos<=7 && zero_bits(s,b,(b.end-b.pos) as nat)}
pub(super) fn pad(s:&[u8],b:Bits) -> (r:bool) requires b.valid(s@),ensures r==padding(s@,b), {
    b.end-b.pos<=7 && zeros(s,b,b.end-b.pos)
}
#[verifier::opaque]
pub(super) open spec fn markers(s:Seq<u8>,b:Bits,c:Config,n:nat) -> bool decreases n {
    if n==0 {padding(s,b)} else {
        let a=eol(s,b,0,19);if a.is_none() {false} else {
            let q=a.unwrap();
            if (c.group4 && q.pos!=b.pos+12) || (!c.group4 && c.aligned && q.pos%8!=0) {false}
            else if c.mixed && !c.group4 {let t=fb::one(s,q);t.is_some() && t.unwrap().0==1 && markers(s,t.unwrap().1,c,(n-1) as nat)}
            else {markers(s,q,c,(n-1) as nat)}
        }
    }
}
pub(super) open spec fn terminated(s:Seq<u8>,b:Bits,c:Config) -> bool {
    padding(s,b) || (!c.missing && markers(s,b,c,if c.group4 {2nat} else {6nat}))
}
pub(super) fn end(s:&[u8],b:Bits,c:Config) -> (r:bool)
    requires b.valid(s@),ensures r==terminated(s@,b,c),
{
    if pad(s,b) {return true;}if c.missing {return false;}
    let mut left=if c.group4 {2usize} else {6usize};let mut q=b;
    let ghost result=markers(s@,b,c,left as nat);
    while left>0
        invariant q.valid(s@),left<=6,result==markers(s@,b,c,if c.group4 {2nat} else {6nat}),result==markers(s@,q,c,left as nat),
            !padding(s@,b),!c.missing,
        decreases left,
    {
        reveal_with_fuel(markers,1);
        let prior=q;let a=read_eol(s,q);if a.is_err() {return false;}q=a.unwrap();
        if (c.group4 && q.pos-prior.pos!=12) || (!c.group4 && c.aligned && q.pos%8!=0) {return false;}
        if c.mixed && !c.group4 {let a=fb::bit(s,q);if a.is_err() {return false;}let (v,p)=a.unwrap();if v!=1 {return false;}q=p;}
        left=left-1;
    }
    reveal_with_fuel(markers,1);pad(s,q)
}
}
