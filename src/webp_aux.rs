//! Nonrecursive VP8L auxiliary images: transform data and entropy meta-images.
use super::*;
use crate::deflate_bits::{self as db,Bits};
use crate::webp_container::{self as wc,Config};
use crate::webp_pixels::{self as pixels,Pixel};
use crate::webp as wp;
verus! {
#[verifier::opaque]
pub(super) open spec fn cache(s:Seq<u8>,p:int,end:int) -> Option<(int,usize)> {
    if p>=end*8{None}else if db::field(s,p,1)==0{Some((p+1,0usize))}else{
        let n=db::field(s,p+1,4);
        if p+5>end*8 || n==0 || n>11{None}else{Some((p+5,n as usize))}
    }
}
pub(super) fn read_cache(s:&[u8],bits:&mut Bits) -> (r:Result<usize,Error>)
    requires old(bits).valid(s@),
    ensures final(bits).valid(s@),final(bits).end==old(bits).end,final(bits).pos()>=old(bits).pos(),
        r.is_ok()==cache(s@,old(bits).pos(),old(bits).end as int).is_some(),
        r.is_ok() ==> (final(bits).pos(),r.unwrap())==cache(s@,old(bits).pos(),old(bits).end as int).unwrap() && r.unwrap()<=11,
{
    reveal(cache);
    if bits.read(s,1)?==0{return Ok(0);}
    let n=bits.read(s,4)?;if n==0 || n>11{return Err(Error::Invalid);}Ok(n)
}
#[verifier::opaque]
pub(super) open spec fn decoded(s:Seq<u8>,bits:Bits,w:usize,h:usize) -> Option<wp::State> {
    let cc=cache(s,bits.pos(),bits.end as int);
    if cc.is_none(){None}else{
        let (q,cb)=cc.unwrap();let gg=wc::group(s,q,bits.end as int,cb as int);
        if gg.is_none(){None}else{
            let (p,g)=gg.unwrap();let b=Bits {at:(p/8) as usize,bit:(p%8) as usize,end:bits.end};
            let c=Config {bits:b,w,h,subtract:false,cache:cb};
            pixels::finish(s,c,seq![g],Seq::empty(),0,w as int*h,wp::initial(p),(w as int*h) as nat)
        }
    }
}
pub(super) fn decode(s:&[u8],bits:Bits,w:usize,h:usize,out:&mut[Pixel]) -> (r:Result<Bits,Error>)
    requires bits.valid(s@),1<=w<=16384,1<=h<=16384,w as int*h<=old(out).len(),
    ensures final(out).len()==old(out).len(),r.is_ok()==decoded(s@,bits,w,h).is_some(),
        r.is_ok() ==> r.unwrap().valid(s@) && r.unwrap().end==bits.end && r.unwrap().pos()>=bits.pos(),
        r.is_ok() ==> r.unwrap().pos()==decoded(s@,bits,w,h).unwrap().pos && final(out)@.take((w as int*h) as int)==decoded(s@,bits,w,h).unwrap().pixels,
        forall|i:int|w as int*h<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
{
    reveal(decoded);let mut next=bits;let cb=read_cache(s,&mut next)?;let group=wc::read_group(s,&mut next,cb)?;
    assert(next.at as int==next.pos()/8 && next.bit as int==next.pos()%8) by(nonlinear_arith)
        requires next.bit<8,next.pos()==next.at as int*8+next.bit;
    let c=Config {bits:next,w,h,subtract:false,cache:cb};let groups=[group];let meta:[Pixel;0]=[];
    assert(pixels::groups(groups@) =~= seq![group.model()]);assert(meta@ =~= Seq::<Pixel>::empty());
    pixels::decode(s,c,&groups,&meta,0,out)
}
pub(super) open spec fn expanded(s:Seq<Pixel>,n:nat) -> Pixel decreases n {
    if n==0{s[0]}else{crate::webp_predict::sum(s[n as int],expanded(s,(n-1) as nat))}
}
pub(super) fn expand_palette(out:&mut[Pixel],n:usize)
    requires 1<=n<=256,n<=old(out).len(),
    ensures final(out).len()==old(out).len(),
        forall|i:int|0<=i<n ==> final(out)@[i]==expanded(old(out)@,i as nat),
        forall|i:int|n<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
{
    let mut i=1usize;
    while i<n
        invariant 1<=i<=n<=256,n<=out.len(),out.len()==old(out).len(),
            forall|j:int|0<=j<i ==> out@[j]==expanded(old(out)@,j as nat),
            forall|j:int|i<=j<out.len() ==> out@[j]==old(out)@[j],
        decreases n-i,
    {out[i]=crate::webp_predict::add_pixel(out[i],out[i-1]);i=i+1;}
}
pub(super) open spec fn image(s:Seq<u8>,bits:Bits,w:usize,h:usize,palette:bool) -> Option<(int,Seq<Pixel>)> {
    let d=decoded(s,bits,w,h);
    if d.is_none(){None}else{
        let v=d.unwrap();Some((v.pos,if palette{Seq::new(v.pixels.len(),|i:int|expanded(v.pixels,i as nat))}else{v.pixels}))
    }
}
pub(super) fn append(s:&[u8],bits:Bits,w:usize,h:usize,palette:bool,start:usize,out:&mut[Pixel]) -> (r:Result<Bits,Error>)
    requires bits.valid(s@),1<=w<=16384,1<=h<=16384,palette ==> h==1 && w<=256,
        start as int+w as int*h<=old(out).len(),
    ensures final(out).len()==old(out).len(),r.is_ok()==image(s@,bits,w,h,palette).is_some(),
        r.is_ok() ==> r.unwrap().valid(s@) && r.unwrap().end==bits.end && r.unwrap().pos()>=bits.pos(),
        r.is_ok() ==> r.unwrap().pos()==image(s@,bits,w,h,palette).unwrap().0 && final(out)@.subrange(start as int,start as int+w as int*h)==image(s@,bits,w,h,palette).unwrap().1,
        forall|i:int|0<=i<old(out).len() && !(start<=i<start as int+w as int*h) ==> final(out)@[i]==old(out)@[i],
{
    assert(w*h<=268435456) by(nonlinear_arith) requires w<=16384,h<=16384;
    let count=w*h;let (_,tail)=out.split_at_mut(start);
    let next=decode(s,bits,w,h,tail)?;
    if palette{
        let ghost raw=tail@;
        expand_palette(tail,count);
        assert forall|i:int|0<=i<count implies #[trigger] expanded(raw,i as nat)==expanded(raw.take(count as int),i as nat) by {
            expansion_prefix(raw,count,i as nat);
        }
    }
    Ok(next)
}
proof fn expansion_prefix(s:Seq<Pixel>,count:usize,n:nat)
    requires n<count<=s.len(),
    ensures expanded(s,n)==expanded(s.take(count as int),n),
    decreases n,
{if n>0{expansion_prefix(s,count,(n-1) as nat);}}
}
