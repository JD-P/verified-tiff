//! Zstandard literal sections and exact reverse-stream termination.
use super::*;
use crate::zstd_bits::{self as zb,Reverse};
use crate::zstd_huffman::{self as zh,Huff};
use crate::zstd_description as zd;
verus! {
#[verifier::opaque]
spec fn stream_finish(s:Seq<u8>,weights:Seq<u8>,log:usize,low:int,p:int,left:nat,out:Seq<u8>) -> Option<Seq<u8>> decreases left {
    if left==0{if p==low{Some(out)}else{None}}else{
        let r=zh::token(weights,log,s,low,p,1,0);
        if r.is_none(){None}else{let (v,next)=r.unwrap();stream_finish(s,weights,log,low,next,(left-1) as nat,out.push(v))}
    }
}
pub(super) closed spec fn stream(s:Seq<u8>,start:int,end:int,weights:Seq<u8>,log:usize,count:usize) -> Option<Seq<u8>> {
    if start>=end || s[end-1]==0 {None}else{
        stream_finish(s,weights,log,start*8,(end-1)*8+zb::top(s[end-1]),count as nat,Seq::empty())
    }
}
fn decode_stream(s:&[u8],start:usize,end:usize,h:&Huff,out:&mut[u8],off:usize,count:usize) -> (r:Result<(),Error>)
    requires start<=end<=s.len(),end<=usize::MAX/8,h.valid(),h.ready,off<=old(out).len(),count<=old(out).len()-off,
    ensures r.is_ok()==stream(s@,start as int,end as int,h.weights@,h.log,count).is_some(),
        final(out).len()==old(out).len(),
        r.is_ok() ==> final(out)@.subrange(off as int,off as int+count)==stream(s@,start as int,end as int,h.weights@,h.log,count).unwrap(),
        forall|k:int|0<=k<old(out).len() && !(off<=k<off as int+count) ==> final(out)@[k]==old(out)@[k],
{
    reveal(stream);
    let mut bits=Reverse::new(s,start,end)?;let mut i=0usize;
    let ghost goal=stream(s@,start as int,end as int,h.weights@,h.log,count);
    assert(out@.subrange(off as int,off as int) =~= Seq::<u8>::empty());
    while i<count
        invariant i<=count,off<=out.len(),count<=out.len()-off,out.len()==old(out).len(),h.valid(),h.ready,
            bits.valid(s@),bits.low==start*8,bits.end==end,
            goal==stream(s@,start as int,end as int,h.weights@,h.log,count),
            goal==stream_finish(s@,h.weights@,h.log,bits.low as int,bits.p as int,(count-i) as nat,out@.subrange(off as int,off as int+i)),
            forall|k:int|0<=k<out.len() && !(off<=k<off as int+count) ==> out@[k]==old(out)@[k],
        decreases count-i,
    {
        reveal_with_fuel(stream_finish,1);
        let v=h.read(s,&mut bits)?;
        let ghost before=out@.subrange(off as int,off as int+i);
        out[off+i]=v;i=i+1;
        assert(out@.subrange(off as int,off as int+i) =~= before.push(v));
    }
    reveal_with_fuel(stream_finish,1);
    if bits.p!=bits.low{return Err(Error::Invalid);}
    Ok(())
}
fn fill(out:&mut[u8],count:usize,value:u8)
    requires count<=old(out).len(),
    ensures final(out).len()==old(out).len(),
        forall|i:int|0<=i<count ==> final(out)@[i]==value,
        forall|i:int|count<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
{
    let mut i=0usize;
    while i<count
        invariant i<=count<=out.len(),out.len()==old(out).len(),
            forall|k:int|0<=k<i ==> out@[k]==value,
            forall|k:int|count<=k<out.len() ==> out@[k]==old(out)@[k],
        decreases count-i,
    {out[i]=value;i=i+1;}
}
#[derive(Clone,Copy)]
pub(super) struct Header {pub kind:usize,pub four:bool,pub count:usize,pub compressed:usize,pub at:usize}
pub(super) closed spec fn header(s:Seq<u8>,start:int,end:int) -> Option<Header> {
    if start>=end{None}else{
        let kind=s[start] as int%4;let fmt=(s[start] as int/4)%4;
        let raw=kind<2;let bytes=if raw{if fmt==0||fmt==2{1int}else if fmt==1{2int}else{3int}}else{if fmt<2{3int}else if fmt==2{4int}else{5int}};
        let shift=if raw&&(fmt==0||fmt==2){3int}else{4int};
        let bits=if raw{bytes*8-shift}else if fmt<2{10int}else if fmt==2{14int}else{18int};
        if start+bytes>end {None}else{
            let count=zb::value(s,start*8+shift,bits as nat);
            let compressed=if raw{if kind==0{count}else{1int}}else{zb::value(s,start*8+4+bits,bits as nat)};
            if count>131072 || compressed==0 && !raw || start+bytes+compressed>end {None}else{
                Some(Header {kind:kind as usize,four:!raw&&fmt!=0,count:count as usize,compressed:compressed as usize,at:(start+bytes) as usize})
            }
        }
    }
}
fn read_header(s:&[u8],start:usize,end:usize) -> (r:Result<Header,Error>)
    requires start<=end<=s.len(),end<=usize::MAX/8,
    ensures r.is_ok()==header(s@,start as int,end as int).is_some(),
        r.is_ok() ==> r.unwrap()==header(s@,start as int,end as int).unwrap(),
        r.is_ok() ==> r.unwrap().kind<4 && r.unwrap().count<=131072 && start<r.unwrap().at<=end && r.unwrap().compressed<=end-r.unwrap().at,
        r.is_ok() ==> (r.unwrap().kind==0 ==> r.unwrap().compressed==r.unwrap().count),
        r.is_ok() ==> (r.unwrap().kind==1 ==> r.unwrap().compressed==1),
        r.is_ok() ==> (r.unwrap().kind<2 ==> !r.unwrap().four),
{
    reveal(header);
    if start==end{return Err(Error::Truncated);}
    let kind=(s[start]%4) as usize;let fmt=((s[start]/4)%4) as usize;
    let raw=kind<2;let bytes=if raw{if fmt==0||fmt==2{1usize}else if fmt==1{2}else{3}}else{if fmt<2{3}else if fmt==2{4}else{5}};
    let shift=if raw&&(fmt==0||fmt==2){3usize}else{4};
    let bits=if raw{bytes*8-shift}else if fmt<2{10}else if fmt==2{14}else{18};
    if bytes>end-start{return Err(Error::Truncated);}
    let count=zb::read(s,start*8+shift,end,bits)? as usize;
    let compressed=if raw{if kind==0{count}else{1usize}}else{zb::read(s,start*8+4+bits,end,bits)? as usize};
    let at=start+bytes;
    if count>131072 || (compressed==0 && !raw) || compressed>end-at{return Err(Error::Invalid);}
    Ok(Header {kind,four:!raw&&fmt!=0,count,compressed,at})
}

#[derive(Clone,Copy)]
struct Cuts {points:[usize;5],step:usize}
spec fn cuts(s:Seq<u8>,start:int,end:int,count:usize) -> Option<Cuts> {
    if end-start<6 || count<6 {None}else{
        let a=word16(s,start,true);let b=word16(s,start+2,true);let c=word16(s,start+4,true);
        let first=start+6;
        if first+a+b+c>end {None}else{
            Some(Cuts {points:[first as usize,(first+a) as usize,(first+a+b) as usize,(first+a+b+c) as usize,end as usize],step:((count+3)/4) as usize})
        }
    }
}
fn read_cuts(s:&[u8],start:usize,end:usize,count:usize) -> (r:Result<Cuts,Error>)
    requires start<=end<=s.len(),end<=usize::MAX/8,count<=131072,
    ensures r.is_ok()==cuts(s@,start as int,end as int,count).is_some(),
        r.is_ok() ==> r.unwrap()==cuts(s@,start as int,end as int,count).unwrap(),
        r.is_ok() ==> 3*r.unwrap().step<=count && r.unwrap().step>0,
        r.is_ok() ==> forall|i:int|0<=i<4 ==> start<=#[trigger] r.unwrap().points@[i]<=r.unwrap().points@[i+1]<=end,
{
    reveal(cuts);
    if end-start<6 || count<6{return Err(Error::Invalid);}
    let a=read16(s,start,true)? as usize;let b=read16(s,start+2,true)? as usize;let c=read16(s,start+4,true)? as usize;
    let first=start+6;
    if a+b+c>end-first{return Err(Error::Truncated);}
    Ok(Cuts {points:[first,first+a,first+a+b,first+a+b+c,end],step:((count+3)/4) as usize})
}
pub(super) closed spec fn coded(s:Seq<u8>,start:int,end:int,w:Seq<u8>,log:usize,count:usize,four:bool) -> Option<Seq<u8>> {
    if !four {stream(s,start,end,w,log,count)}else{
        let cc=cuts(s,start,end,count);
        if cc.is_none(){None}else{
            let c=cc.unwrap();let a=stream(s,c.points[0] as int,c.points[1] as int,w,log,c.step);
            let b=stream(s,c.points[1] as int,c.points[2] as int,w,log,c.step);
            let d=stream(s,c.points[2] as int,c.points[3] as int,w,log,c.step);
            let e=stream(s,c.points[3] as int,c.points[4] as int,w,log,(count-3*c.step) as usize);
            if a.is_none()||b.is_none()||d.is_none()||e.is_none(){None}else{
                Some(Seq::new(count as nat,|i:int|if i<c.step {a.unwrap()[i]}else if i<2*c.step{b.unwrap()[i-c.step]}else if i<3*c.step{d.unwrap()[i-2*c.step]}else{e.unwrap()[i-3*c.step]}))
            }
        }
    }
}
fn decode_coded(s:&[u8],start:usize,end:usize,h:&Huff,out:&mut[u8],count:usize,four:bool) -> (r:Result<(),Error>)
    requires start<=end<=s.len(),end<=usize::MAX/8,h.valid(),h.ready,count<=old(out).len(),count<=131072,
    ensures r.is_ok()==coded(s@,start as int,end as int,h.weights@,h.log,count,four).is_some(),
        final(out).len()==old(out).len(),
        r.is_ok() ==> final(out)@.take(count as int)==coded(s@,start as int,end as int,h.weights@,h.log,count,four).unwrap(),
        forall|k:int|count<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
{
    reveal(coded);
    if !four{return decode_stream(s,start,end,h,out,0,count);}
    let c=read_cuts(s,start,end,count)?;let step=c.step;
    decode_stream(s,c.points[0],c.points[1],h,out,0,step)?;
    decode_stream(s,c.points[1],c.points[2],h,out,step,step)?;
    decode_stream(s,c.points[2],c.points[3],h,out,2*step,step)?;
    decode_stream(s,c.points[3],c.points[4],h,out,3*step,count-3*step)?;
    assert(out@.take(count as int) =~= coded(s@,start as int,end as int,h.weights@,h.log,count,true).unwrap()) by {
        assert forall|i:int|0<=i<count implies out@[i]==coded(s@,start as int,end as int,h.weights@,h.log,count,true).unwrap()[i] by {
            if i<step {}else if i<2*step{}else if i<3*step{}else{}
        }
    }
    Ok(())
}
pub(super) struct LiteralModel {pub data:Seq<u8>,pub at:usize,pub weights:Seq<u8>,pub log:usize,pub ready:bool}
pub(super) closed spec fn model(s:Seq<u8>,start:int,end:int,w:Seq<u8>,log:usize,ready:bool,cap:usize) -> Option<LiteralModel> {
    let hh=header(s,start,end);
    if hh.is_none(){None}else{
        let h=hh.unwrap();let next=h.at+h.compressed;
        if h.count>cap{None}else if h.kind<2 {
            let data=Seq::new(h.count as nat,|i:int|if h.kind==0{s[h.at as int+i]}else{s[h.at as int]});
            Some(LiteralModel {data,at:next as usize,weights:w,log,ready})
        }else{
            let desc=if h.kind==2 {zd::description(s,h.at as int,next as int)}
                else if ready{Some(zd::Description {weights:w,log,at:h.at as int})}else{None};
            if desc.is_none(){None}else{
                let tree=desc.unwrap();let data=coded(s,tree.at,next as int,tree.weights,tree.log,h.count,h.four);
                if data.is_none(){None}else{Some(LiteralModel {data:data.unwrap(),at:next as usize,weights:tree.weights,log:tree.log,ready:true})}
            }
        }
    }
}
pub(super) fn decode(s:&[u8],start:usize,end:usize,h:&mut Huff,out:&mut[u8],cap:usize) -> (r:Result<(usize,usize),Error>)
    requires start<=end<=s.len(),end<=usize::MAX/8,old(h).valid(),cap<=old(out).len(),
    ensures final(h).valid(),final(out).len()==old(out).len(),
        r.is_ok()==model(s@,start as int,end as int,old(h).weights@,old(h).log,old(h).ready,cap).is_some(),
        r.is_ok() ==> r.unwrap().0==model(s@,start as int,end as int,old(h).weights@,old(h).log,old(h).ready,cap).unwrap().at,
        r.is_ok() ==> r.unwrap().1==model(s@,start as int,end as int,old(h).weights@,old(h).log,old(h).ready,cap).unwrap().data.len(),
        r.is_ok() ==> final(out)@.take(r.unwrap().1 as int)==model(s@,start as int,end as int,old(h).weights@,old(h).log,old(h).ready,cap).unwrap().data,
        r.is_ok() ==> final(h).weights@==model(s@,start as int,end as int,old(h).weights@,old(h).log,old(h).ready,cap).unwrap().weights,
        r.is_ok() ==> final(h).log==model(s@,start as int,end as int,old(h).weights@,old(h).log,old(h).ready,cap).unwrap().log && final(h).ready==model(s@,start as int,end as int,old(h).weights@,old(h).log,old(h).ready,cap).unwrap().ready,
        r.is_ok() ==> start<r.unwrap().0<=end && r.unwrap().1<=cap && r.unwrap().1<=131072,
        forall|k:int|cap<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
{
    reveal(model);
    let header=read_header(s,start,end)?;let count=header.count;let next=header.at+header.compressed;
    if count>cap{return Err(Error::OutputTooSmall);}
    if header.kind<2 {
        if header.kind==0{copy_byte_run(s,header.at,out,0,count,false);}
        else {
            fill(out,count,s[header.at]);
        }
        let ghost wanted=model(s@,start as int,end as int,old(h).weights@,old(h).log,old(h).ready,cap).unwrap().data;
        assert forall|i:int|0<=i<count implies out@[i]==wanted[i] by {
            if header.kind==0{assert(out@[0int+i]==s@[header.at as int+i]);}
        }
        assert(out@.take(count as int) =~= wanted);
    }else{
        let mut at=header.at;
        if header.kind==2 {let (tree,after)=zd::read(s,at,next)?;*h=tree;at=after;}
        else if !h.ready{return Err(Error::Invalid);}
        decode_coded(s,at,next,h,out,count,header.four)?;
    }
    Ok((next,count))
}
}
