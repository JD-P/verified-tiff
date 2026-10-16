//! Lerc2 validity-mask run-length stream. Every run is bounded and the EOF
//! marker must coincide with both the compressed and expanded ends.
use super::*;
verus! {
#[derive(Clone,Copy)]
struct Run {at:usize,next:usize,count:usize,repeat:bool,last:bool}
closed spec fn run(s:Seq<u8>,at:usize,end:usize,space:usize) -> Option<Run> {
    if at as int+2>end{None}else{
        let v=word16(s,at as int,true);
        if v==32768{Some(Run {at:(at+2) as usize,next:(at+2) as usize,count:0,repeat:false,last:true})}
        else{
            let count=if v>32768{65536-v}else{v};let size=if v>32768{1int}else{count};
            if count==0 || count>space || at as int+2+size>end{None}
            else{Some(Run {at:(at+2) as usize,next:(at as int+2+size) as usize,count:count as usize,repeat:v>32768,last:false})}
        }
    }
}
fn read(s:&[u8],at:usize,end:usize,space:usize) -> (r:Result<Run,Error>)
    requires at<=end<=s.len(),
    ensures r.is_ok()==run(s@,at,end,space).is_some(),r.is_ok() ==> r.unwrap()==run(s@,at,end,space).unwrap(),
        r.is_ok() ==> at<r.unwrap().at<=r.unwrap().next<=end && r.unwrap().count<=space,
        r.is_ok() ==> !r.unwrap().last ==> r.unwrap().at<r.unwrap().next,
        r.is_ok() ==> !r.unwrap().repeat ==> r.unwrap().next==r.unwrap().at+r.unwrap().count,
{
    reveal(run);if end-at<2{return Err(Error::Truncated);}
    let v=read16(s,at,true)? as usize;
    if v==32768{return Ok(Run {at:at+2,next:at+2,count:0,repeat:false,last:true});}
    let count=if v>32768{65536-v}else{v};let size=if v>32768{1usize}else{count};
    if count==0 || count>space || size>end-at-2{return Err(Error::Invalid);}
    Ok(Run {at:at+2,next:at+2+size,count,repeat:v>32768,last:false})
}
#[verifier::opaque]
pub(super) closed spec fn result(s:Seq<u8>,at:usize,end:usize,prefix:Seq<u8>,cap:usize,fuel:nat) -> Option<Seq<u8>> decreases fuel {
    if fuel==0{None}else{
        let rr=run(s,at,end,(cap-prefix.len()) as usize);
        if rr.is_none(){None}else{
            let r=rr.unwrap();
            if r.last{if r.next==end && prefix.len()==cap{Some(prefix)}else{None}}
            else{
                let data=if r.repeat{Seq::new(r.count as nat,|i:int|s[r.at as int])}else{s.subrange(r.at as int,r.next as int)};
                result(s,r.next,end,prefix+data,cap,(fuel-1) as nat)
            }
        }
    }
}
pub(super) fn decode(s:&[u8],start:usize,end:usize,cap:usize,out:&mut[u8]) -> (r:Result<(),Error>)
    requires start<=end<=s.len(),cap<=old(out).len(),
    ensures final(out).len()==old(out).len(),
        r.is_ok()==result(s@,start,end,Seq::empty(),cap,end as nat).is_some(),
        r.is_ok() ==> final(out)@.take(cap as int)==result(s@,start,end,Seq::empty(),cap,end as nat).unwrap(),
        forall|k:int|cap<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
{
    let mut at=start;let mut pos=0usize;let mut fuel=end;
    let ghost goal=result(s@,start,end,Seq::empty(),cap,end as nat);
    assert(out@.take(0) =~= Seq::<u8>::empty());
    loop
        invariant at<=end<=s.len(),pos<=cap<=out.len(),out.len()==old(out).len(),
            goal==result(s@,start,end,Seq::empty(),cap,end as nat),
            goal==result(s@,at,end,out@.take(pos as int),cap,fuel as nat),
            forall|k:int|cap<=k<out.len() ==> out@[k]==old(out)@[k],
        decreases fuel,
    {
        reveal_with_fuel(result,1);
        if fuel==0{return Err(Error::Invalid);}
        let run=read(s,at,end,cap-pos)?;
        if run.last{
            if run.next!=end || pos!=cap{return Err(Error::Invalid);}
            return Ok(());
        }
        if run.repeat{crate::zstd_output::append_repeat(out,pos,run.count,s[run.at]);}
        else{crate::zstd_output::append_literals(s,run.at,run.count,out,pos);}
        pos=pos+run.count;at=run.next;fuel=fuel-1;
    }
}
pub(super) open spec fn valid(mask:Seq<u8>,pixel:int) -> bool {(mask[pixel/8] as int/power2(7-pixel%8))%2==1}
pub(super) fn is_valid(mask:&[u8],pixel:usize) -> (r:bool)
    requires pixel/8<mask.len(),
    ensures r==valid(mask@,pixel as int),
{(mask[pixel/8]/bit_divisor(7-pixel%8))%2==1}
// Conservative shortcut: padding bits may make this false, never incorrectly true.
pub(super) fn all(mask:&[u8],pixels:usize) -> (r:bool)
    requires pixels<=usize::MAX-7,(pixels as int+7)/8<=mask.len(),
    ensures r ==> forall|p:int|0<=p<pixels ==> valid(mask@,p),
{
    let n=(pixels+7)/8;let mut i=0usize;
    while i<n
        invariant i<=n,n==(pixels as int+7)/8,n<=mask.len(),
            forall|j:int|0<=j<i ==> mask@[j]==255,
        decreases n-i,
    {if mask[i]!=255{return false;}i=i+1;}
    assert forall|p:int|0<=p<pixels implies valid(mask@,p) by {
        assert(0<=p/8<n);assert(mask@[p/8]==255);
        assert((255int/power2(7-p%8))%2==1);
    }
    true
}

}
