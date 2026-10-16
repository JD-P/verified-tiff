//! Static WebP RIFF framing, including VP8X and bounded ancillary chunks.
//! Animation is deliberately not a TIFF strip/tile representation.
use super::*;
verus! {
#[derive(Clone,Copy)]
pub(super) struct Frame {pub kind:usize,pub at:usize,pub end:usize,pub alpha:usize,pub alpha_end:usize,pub extended:bool}
pub(super) open spec fn blank() -> Frame {Frame {kind:0,at:0,end:0,alpha:0,alpha_end:0,extended:false}}
pub(super) open spec fn u24(s:Seq<u8>,p:int) -> int {s[p] as int+256*s[p+1] as int+65536*s[p+2] as int}
fn read24(s:&[u8],p:usize) -> (r:usize)
    requires p as int+3<=s.len(),ensures r==u24(s@,p as int),r<=16777215,
{s[p] as usize+256*s[p+1] as usize+65536*s[p+2] as usize}
#[derive(Clone,Copy)]
pub(super) struct Chunk {pub tag:u32,pub at:usize,pub end:usize,pub next:usize}
pub(super) closed spec fn chunk(s:Seq<u8>,p:usize) -> Option<Chunk> {
    if p as int+8>s.len(){None}else{
        let n=word32(s,p as int+4,true);let end=p as int+8+n;let next=end+n%2;
        if next>s.len() || (n%2==1 && s[end]!=0){None}
        else{Some(Chunk {tag:word32(s,p as int,true) as u32,at:(p+8) as usize,end:end as usize,next:next as usize})}
    }
}
fn read_chunk(s:&[u8],p:usize) -> (r:Result<Chunk,Error>)
    requires p<=s.len(),
    ensures r.is_ok()==chunk(s@,p).is_some(),r.is_ok() ==> r.unwrap()==chunk(s@,p).unwrap(),
        r.is_ok() ==> p<r.unwrap().at<=r.unwrap().end<=r.unwrap().next<=s.len(),
{
    reveal(chunk);if s.len()-p<8{return Err(Error::Truncated);}
    let tag=read32(s,p,true)?;let n=read32(s,p+4,true)? as usize;let at=p+8;
    if n>s.len()-at || n%2>s.len()-at-n{return Err(Error::Truncated);}
    let end=at+n;let next=end+n%2;
    if n%2==1 && s[end]!=0{return Err(Error::Invalid);}
    Ok(Chunk {tag,at,end,next})
}
pub(super) closed spec fn step(s:Seq<u8>,w:usize,h:usize,c:Chunk,f:Frame) -> Option<Frame> {
    let n=c.end as int-c.at;let p=c.at as int;
    if c.tag==1480085590{ // VP8X
        if f.extended || f.kind!=0 || n!=10 || s[p]%4!=0 || s[p]>=64 || s[p+1]!=0 || s[p+2]!=0 || s[p+3]!=0 ||
            u24(s,p+4)+1!=w || u24(s,p+7)+1!=h{None}else{Some(Frame {extended:true,..f})}
    }else if c.tag==1213221953{ // ALPH
        if f.alpha!=0 || f.kind!=0 || n<1{None}else{Some(Frame {alpha:c.at,alpha_end:c.end,..f})}
    }else if c.tag==1278758998{ // VP8L
        if f.kind!=0 || f.alpha!=0 || n<5 || s[p]!=47{None}else{
            let v=word32(s,p+1,true);
            if v/536870912!=0 || v%16384+1!=w || (v/16384)%16384+1!=h{None}
            else{Some(Frame {kind:1,at:(c.at+5) as usize,end:c.end,..f})}
        }
    }else if c.tag==540561494{ // VP8 (key frame only)
        if f.kind!=0 || n<10 || w>16383 || h>16383 || s[p]%2!=0 || (s[p]/2)%8>3 || (s[p]/16)%2!=1 || s[p+3]!=157 || s[p+4]!=1 || s[p+5]!=42 ||
            word16(s,p+6,true)!=w || word16(s,p+8,true)!=h{None}
        else{Some(Frame {kind:2,at:c.at,end:c.end,..f})}
    }else if c.tag==1296649793 || c.tag==1179471425{None} // ANIM / ANMF
    else{Some(f)}
}
fn consume(s:&[u8],w:usize,h:usize,c:Chunk,f:Frame) -> (r:Result<Frame,Error>)
    requires c.at<=c.end<=s.len(),
    ensures r.is_ok()==step(s@,w,h,c,f).is_some(),r.is_ok() ==> r.unwrap()==step(s@,w,h,c,f).unwrap(),
{
    reveal(step);let n=c.end-c.at;let p=c.at;
    if c.tag==1480085590{
        if f.extended || f.kind!=0 || n!=10 || s[p]%4!=0 || s[p]>=64 || s[p+1]!=0 || s[p+2]!=0 || s[p+3]!=0 ||
            read24(s,p+4)+1!=w || read24(s,p+7)+1!=h{return Err(Error::Invalid);}
        Ok(Frame {extended:true,..f})
    }else if c.tag==1213221953{
        if f.alpha!=0 || f.kind!=0 || n<1{return Err(Error::Invalid);}
        Ok(Frame {alpha:c.at,alpha_end:c.end,..f})
    }else if c.tag==1278758998{
        if f.kind!=0 || f.alpha!=0 || n<5 || s[p]!=47{return Err(Error::Invalid);}
        let v=read32(s,p+1,true)? as usize;
        if v/536870912!=0 || v%16384+1!=w || (v/16384)%16384+1!=h{return Err(Error::Invalid);}
        Ok(Frame {kind:1,at:c.at+5,end:c.end,..f})
    }else if c.tag==540561494{
        if f.kind!=0 || n<10 || w>16383 || h>16383 || s[p]%2!=0 || (s[p]/2)%8>3 || (s[p]/16)%2!=1 || s[p+3]!=157 || s[p+4]!=1 || s[p+5]!=42 ||
            read16(s,p+6,true)? as usize!=w || read16(s,p+8,true)? as usize!=h{return Err(Error::Invalid);}
        Ok(Frame {kind:2,at:c.at,end:c.end,..f})
    }else if c.tag==1296649793 || c.tag==1179471425{Err(Error::Unsupported)}else{Ok(f)}
}
#[verifier::opaque]
pub(super) closed spec fn finish(s:Seq<u8>,w:usize,h:usize,p:usize,f:Frame,fuel:nat) -> Option<Frame> decreases fuel {
    if p==s.len(){if f.kind==0{None}else{Some(f)}}else if fuel==0{None}else{
        let c=chunk(s,p);
        if c.is_none(){None}else{
            let next=step(s,w,h,c.unwrap(),f);
            if next.is_none(){None}else{finish(s,w,h,c.unwrap().next,next.unwrap(),(fuel-1) as nat)}
        }
    }
}
pub(super) closed spec fn frame(s:Seq<u8>,w:usize,h:usize) -> Option<Frame> {
    if s.len()<12 || s.len()>usize::MAX/8 || w==0 || w>16384 || h==0 || h>16384 || word32(s,0,true)!=1179011410 ||
        word32(s,4,true)!=s.len()-8 || word32(s,8,true)!=1346520407{None}
    else{finish(s,w,h,12,blank(),s.len())}
}
pub(super) fn inspect(s:&[u8],w:usize,h:usize) -> (r:Result<Frame,Error>)
    ensures r.is_ok()==frame(s@,w,h).is_some(),r.is_ok() ==> r.unwrap()==frame(s@,w,h).unwrap(),
        r.is_ok() ==> 1<=w<=16384 && 1<=h<=16384 && s.len()<=usize::MAX/8,
        r.is_ok() ==> 1<=r.unwrap().kind<=2 && r.unwrap().at<=r.unwrap().end<=s.len() && r.unwrap().alpha<=r.unwrap().alpha_end<=s.len(),
{
    reveal(frame);
    if s.len()<12 || s.len()>usize::MAX/8 || w==0 || w>16384 || h==0 || h>16384 || read32(s,0,true)?!=1179011410 ||
        read32(s,4,true)? as usize!=s.len()-8 || read32(s,8,true)?!=1346520407{return Err(Error::Invalid);}
    let mut p=12usize;let mut f=Frame {kind:0,at:0,end:0,alpha:0,alpha_end:0,extended:false};let mut fuel=s.len();
    let ghost goal=frame(s@,w,h);
    loop
        invariant p<=s.len(),f.kind<=2,f.at<=f.end<=s.len(),f.alpha<=f.alpha_end<=s.len(),
            goal==frame(s@,w,h),goal==finish(s@,w,h,p,f,fuel as nat),
        decreases fuel,
    {
        reveal_with_fuel(finish,1);
        if p==s.len(){if f.kind==0{return Err(Error::Invalid);}return Ok(f);}
        if fuel==0{return Err(Error::Invalid);}
        let c=read_chunk(s,p)?;let next=consume(s,w,h,c,f)?;
        reveal(step);f=next;p=c.next;fuel=fuel-1;
    }
}
}
