//! All four VP8L inverse transforms on caller-owned RGBA pixels.
use super::*;
use crate::webp_pixels::{self as pix,Pixel};
use crate::webp_transforms::Transform;
use crate::webp_predict as pred;
verus! {
#[verifier::opaque]
pub(super) open spec fn coefficient(t:Transform,data:Seq<Pixel>,p:usize) -> Option<Pixel> {
    let side=pix::tile_size(t.bits) as int;
    let cell=(p as int/(t.width as int)/side)*pix::sampled(t.width,t.bits)+(p as int%(t.width as int))/side;
    let at=t.offset as int+cell;if at>=data.len(){None}else{Some(data[at])}
}
#[inline(always)]
pub(super) fn matrix(t:Transform,data:&[Pixel],p:usize,x:usize,y:usize) -> (r:Result<Pixel,Error>)
    requires t.valid(),t.kind<2,p<t.width*t.height,x==p%t.width,y==p/t.width,
    ensures r.is_ok()==coefficient(t,data@,p).is_some(),r.is_ok() ==> r.unwrap()==coefficient(t,data@,p).unwrap(),
{
    reveal(coefficient);let side=pix::size(t.bits);let across=pix::subsample(t.width,t.bits);
    let y=pix::coordinate(y,t.bits);let x=pix::coordinate(x,t.bits);
    assert(p/t.width<t.height) by(nonlinear_arith) requires p<t.width*t.height,t.width>0;
    assert(y*across+x<=268451840) by(nonlinear_arith) requires y<16384,across<=16384,x<16384;
    let cell=y*across+x;
    if t.offset>data.len() || cell>=data.len()-t.offset{return Err(Error::Invalid);}
    Ok(data[t.offset+cell])
}
pub(super) open spec fn entry(t:Transform,out:Seq<Pixel>,p:usize) -> usize {
    let per=pix::tile_size(t.bits) as int;let width=pix::sampled(t.width,t.bits) as int;
    let source=(p as int/(t.width as int))*width+(p as int%(t.width as int))/per;
    let bits=8int/per;let shift=((p as int%(t.width as int))%per)*bits;
    let modulus=if bits==8{256int}else{power2(bits)};
    ((out[source]@[1] as int/power2(shift))%modulus) as usize
}
#[inline(always)]
fn index(t:Transform,out:&[Pixel],p:usize) -> (r:usize)
    requires t.valid(),t.kind==3,p<t.width*t.height,t.width as int*t.height<=out.len(),
    ensures r==entry(t,out@,p),r<256,
{
    let per=pix::size(t.bits);let width=pix::subsample(t.width,t.bits);
    let y=p/t.width;let x=p%t.width;
    assert(y<t.height) by(nonlinear_arith) requires p<t.width*t.height,t.width>0,y==p/t.width;
    assert(y*width+x/per<t.width*t.height) by(nonlinear_arith) requires y<t.height,width<=t.width,x<t.width,per>=1,t.width>0;
    let source=y*width+x/per;let bits=8/per;let shift=(x%per)*bits;
    assert(per==1 || per==2 || per==4 || per==8);
    assert(shift<8 && (bits==1 || bits==2 || bits==4 || bits==8)) by(nonlinear_arith) requires per==1 || per==2 || per==4 || per==8,bits==8usize/per,shift==(x%per)*bits;
    let modulus=if bits==8{256usize}else{bit_divisor(bits) as usize};
    (out[source][1] as usize/(bit_divisor(shift) as usize))%modulus
}
#[verifier::opaque]
pub(super) open spec fn value(t:Transform,data:Seq<Pixel>,out:Seq<Pixel>,p:usize) -> Option<Pixel> {
    if t.kind==2{Some(pred::green(out[p as int]))}else if t.kind==3{
        let k=entry(t,out,p);
        if k>=t.colors{Some([0u8,0,0,0])}else if t.offset as int+k>=data.len(){None}else{Some(data[t.offset as int+k])}
    }else if t.kind==1{
        let m=coefficient(t,data,p);if m.is_none(){None}else{Some(pred::color(out[p as int],m.unwrap()))}
    }else{
        let prediction=if p==0{Some([0u8,0,0,255])}else if p<t.width{Some(out[p as int-1])}
            else if (p as int)%(t.width as int)==0{Some(out[p as int-t.width])}else{
                let m=coefficient(t,data,p);
                if m.is_none() || m.unwrap()@[1]%16>13{None}else{
                    Some(pred::prediction((m.unwrap()@[1]%16) as usize,out[p as int-1],out[p as int-t.width],out[p as int-t.width-1],out[p as int-t.width+1]))
                }
            };
        if prediction.is_none(){None}else{Some(pred::sum(out[p as int],prediction.unwrap()))}
    }
}
#[inline(always)]
fn pixel(t:Transform,data:&[Pixel],out:&[Pixel],p:usize,x:usize,y:usize) -> (r:Result<Pixel,Error>)
    requires t.valid(),p<t.width*t.height,t.width as int*t.height<=out.len(),x==p%t.width,y==p/t.width,
    ensures r.is_ok()==value(t,data@,out@,p).is_some(),r.is_ok() ==> r.unwrap()==value(t,data@,out@,p).unwrap(),
{
    reveal(value);
    if t.kind==2{return Ok(pred::add_green(out[p]));}
    if t.kind==3{
        let k=index(t,out,p);
        if k>=t.colors{return Ok([0u8,0,0,0]);}
        if t.offset>data.len() || k>=data.len()-t.offset{return Err(Error::Invalid);}
        return Ok(data[t.offset+k]);
    }
    if t.kind==1{return Ok(pred::inverse_color(out[p],matrix(t,data,p,x,y)?));}
    let prediction=if p==0{[0u8,0,0,255]}else if p<t.width{out[p-1]}else if x==0{out[p-t.width]}else{
        let m=matrix(t,data,p,x,y)?;let mode=(m[1]%16) as usize;
        if mode>13{return Err(Error::Invalid);}
        assert(p>t.width && t.width>=2) by(nonlinear_arith) requires p>=t.width,p%t.width!=0,t.width>=1;
        pred::predict(mode,out[p-1],out[p-t.width],out[p-t.width-1],out[p-t.width+1])
    };
    Ok(pred::add_pixel(out[p],prediction))
}
#[verifier::opaque]
pub(super) open spec fn finish(t:Transform,data:Seq<Pixel>,out:Seq<Pixel>,k:usize,fuel:nat) -> Option<Seq<Pixel>> decreases fuel {
    let count=t.width as int*t.height;
    if k==count{Some(out)}else if fuel==0{None}else{
        let p=if t.kind==3{(count-1-k) as usize}else{k};let v=value(t,data,out,p);
        if v.is_none(){None}else{finish(t,data,out.update(p as int,v.unwrap()),(k+1) as usize,(fuel-1) as nat)}
    }
}
proof fn next_position(w:int,h:int,k:int,x:int,y:int,reverse:bool)
    requires w>0,h>0,0<=x<w,0<=y<h,0<=k,k+1<w*h,
        (if reverse{w*h-1-k}else{k})==y*w+x,
    ensures reverse && x==0 ==> y>0,
        !reverse && x==w-1 ==> y+1<h,
        (if reverse{w*h-2-k}else{k+1})==
            (if reverse{if x==0{(y-1)*w+w-1}else{y*w+x-1}}
             else{if x==w-1{(y+1)*w}else{y*w+x+1}}),
{
    assert(reverse && x==0 ==> y>0) by(nonlinear_arith)
        requires w>0,h>0,y>=0,k+1<w*h,reverse ==> w*h-1-k==y*w+x;
    assert(!reverse && x==w-1 ==> y+1<h) by(nonlinear_arith)
        requires w>0,h>0,y>=0,k+1<w*h,!reverse ==> k==y*w+x;
    assert((if reverse{w*h-2-k}else{k+1})==
        (if reverse{if x==0{(y-1)*w+w-1}else{y*w+x-1}}
         else{if x==w-1{(y+1)*w}else{y*w+x+1}})) by(nonlinear_arith)
        requires (if reverse{w*h-1-k}else{k})==y*w+x;
}
pub(super) fn apply(t:Transform,data:&[Pixel],out:&mut[Pixel]) -> (r:Result<(),Error>)
    requires t.valid(),t.width as int*t.height<=old(out).len(),
    ensures final(out).len()==old(out).len(),r.is_ok()==finish(t,data@,old(out)@,0,(t.width as int*t.height) as nat).is_some(),
        r.is_ok() ==> final(out)@==finish(t,data@,old(out)@,0,(t.width as int*t.height) as nat).unwrap(),
        forall|i:int|t.width as int*t.height<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
{
    assert(t.width*t.height<=268435456) by(nonlinear_arith) requires t.width<=16384,t.height<=16384;
    let count=t.width*t.height;let mut k=0usize;let mut x=if t.kind==3{t.width-1}else{0};let mut y=if t.kind==3{t.height-1}else{0};let ghost goal=finish(t,data@,out@,0,count as nat);
    assert((if t.kind==3{count-1}else{0})==y*t.width+x) by(nonlinear_arith)
        requires count==t.width*t.height,x==(if t.kind==3{t.width-1}else{0}),y==(if t.kind==3{t.height-1}else{0});
    while k<count
        invariant t.valid(),count==t.width as int*t.height,k<=count,count<=out.len(),out.len()==old(out).len(),
            x<t.width,y<t.height,k<count ==> (if t.kind==3{count-1-k}else{k as int})==y*t.width+x,
            goal==finish(t,data@,old(out)@,0,count as nat),goal==finish(t,data@,out@,k,(count-k) as nat),
            forall|i:int|count<=i<out.len() ==> out@[i]==old(out)@[i],
        decreases count-k,
    {
        if verified_tiff_simd::available() && t.kind==2 {
            crate::webp_spans::colors(t,data,out,k,count-k,[0u8;4]);k=count;continue;
        }
        if verified_tiff_simd::available() && (t.kind==1 || (t.kind==0 && x>0 && y>0)) {
            proof{vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(k as int,t.width as int,y as int,x as int);}
            if t.kind==0{assert(k>t.width) by(nonlinear_arith) requires k==y*t.width+x,y>0,x>0,t.width>0;}
            reveal_with_fuel(finish,1);reveal(value);
            let (n,m)=crate::webp_spans::select(t,data,k,x,y)?;
            if t.kind==0 {
                if m[1]%16>13{reveal_with_fuel(finish,1);reveal(value);return Err(Error::Invalid);}
                crate::webp_spans::predict(t,data,out,k,n,x,y,m);
            }else{crate::webp_spans::colors(t,data,out,k,n,m);}
            if k+n<count {
                if x+n==t.width {
                    assert(y+1<t.height) by(nonlinear_arith) requires k==y*t.width+x,x+n==t.width,k+n<count,count==t.width*t.height,t.width>0;
                }
                assert(k+n==(if x+n==t.width{(y+1)*t.width}else{y*t.width+x+n})) by(nonlinear_arith)
                    requires k==y*t.width+x,x+n<=t.width;
            }
            k=k+n;
            if k<count{if x+n==t.width{x=0;y=y+1;}else{x=x+n;}}
            continue;
        }
        reveal_with_fuel(finish,1);let p=if t.kind==3{count-1-k}else{k};
        proof{vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(p as int,t.width as int,y as int,x as int);}
        let value=pixel(t,data,out,p,x,y)?;out[p]=value;
        proof{if k+1<count{next_position(t.width as int,t.height as int,k as int,x as int,y as int,t.kind==3);}}
        k=k+1;
        if k<count{
            if t.kind==3{
                if x==0{assert(y>0);x=t.width-1;y=y-1;}else{x=x-1;}
            }else{
                if x==t.width-1{x=0;y=y+1;}else{x=x+1;}
            }
        }
    }
    reveal_with_fuel(finish,1);Ok(())
}
#[verifier::opaque]
pub(super) open spec fn all(transforms:Seq<Transform>,data:Seq<Pixel>,out:Seq<Pixel>,n:nat) -> Option<Seq<Pixel>> decreases n {
    if n==0{Some(out)}else{
        let t=transforms[n as int-1];let r=finish(t,data,out,0,(t.width as int*t.height) as nat);
        if r.is_none(){None}else{all(transforms,data,r.unwrap(),(n-1) as nat)}
    }
}
pub(super) fn inverse(transforms:&[Transform;4],count:usize,data:&[Pixel],out:&mut[Pixel]) -> (r:Result<(),Error>)
    requires count<=4,forall|i:int|0<=i<count ==> transforms@[i].valid() && transforms@[i].width as int*transforms@[i].height<=old(out).len(),
    ensures final(out).len()==old(out).len(),r.is_ok()==all(transforms@.take(count as int),data@,old(out)@,count as nat).is_some(),
        r.is_ok() ==> final(out)@==all(transforms@.take(count as int),data@,old(out)@,count as nat).unwrap(),
{
    let mut n=count;let ghost goal=all(transforms@.take(count as int),data@,out@,count as nat);
    while n>0
        invariant n<=count<=4,out.len()==old(out).len(),
            forall|i:int|0<=i<count ==> transforms@[i].valid() && transforms@[i].width as int*transforms@[i].height<=out.len(),
            goal==all(transforms@.take(count as int),data@,old(out)@,count as nat),goal==all(transforms@.take(count as int),data@,out@,n as nat),
        decreases n,
    {reveal_with_fuel(all,1);apply(transforms[n-1],data,out)?;n=n-1;}
    reveal_with_fuel(all,1);Ok(())
}
}
