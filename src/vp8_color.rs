//! Centered bilinear chroma upsampling and fixed-point BT.601 RGB conversion.
//! Integer rounding matches libwebp 1.6.0, including image-edge replication.
use super::*;
verus! {
pub(super) open spec fn clipped(v:int) -> u8 {if v<0{0u8}else if v>=16384{255u8}else{(v/64) as u8}}
#[inline(always)]
fn clip(v:i32) -> (r:u8) ensures r==clipped(v as int),
{if v<0{0}else if v>=16384{255}else{(v/64) as u8}}
pub(super) open spec fn color(y:u8,u:u8,v:u8) -> [u8;4] {
    let yy=(y as int*19077)/256;
    [clipped(yy+(v as int*26149)/256-14234),clipped(yy-(u as int*6419)/256-(v as int*13320)/256+8708),clipped(yy+(u as int*33050)/256-17685),255u8]
}
#[inline(always)]
fn rgb(y:u8,u:u8,v:u8) -> (r:[u8;4]) ensures r==color(y,u,v),
{
    let yy=y as i32*19077/256;
    [clip(yy+v as i32*26149/256-14234),clip(yy-u as i32*6419/256-v as i32*13320/256+8708),clip(yy+u as i32*33050/256-17685),255]
}
#[inline(always)]
fn at(p:&[u8],base:usize,stride:usize,height:usize,x:usize,y:usize) -> (r:u8)
    requires stride>0,x<stride,y<height,base as int+stride as int*height<=p.len(),
    ensures r==p@[base+y*stride+x],
{
    assert(base+y*stride+x<p.len()) by(nonlinear_arith) requires stride>0,x<stride,y<height,base+stride*height<=p.len();
    assert(y*stride<=base+y*stride+x) by(nonlinear_arith) requires stride>0,y>=0,base>=0,x>=0;
    p[base+y*stride+x]
}
pub(super) open spec fn chroma(p:Seq<u8>,base:usize,stride:usize,w:usize,h:usize,x:usize,y:usize) -> u8 {
    let x0=if x==0{0int}else{(x-1)/2};let x1=if (x+1)/2>(w-1)/2{(w-1)/2}else{(x+1)/2};
    let y0=if y==0{0int}else{(y-1)/2};let y1=if (y+1)/2>(h-1)/2{(h-1)/2}else{(y+1)/2};
    let wx=if x%2==0{1int}else{3int};let wy=if y%2==0{1int}else{3int};
    let a=p[base+y0*stride+x0] as int;let b=p[base+y0*stride+x1] as int;let c=p[base+y1*stride+x0] as int;let d=p[base+y1*stride+x1] as int;
    ((wx*wy*a+(4-wx)*wy*b+wx*(4-wy)*c+(4-wx)*(4-wy)*d+8)/16) as u8
}
#[inline(always)]
fn interpolate(p:&[u8],base:usize,stride:usize,height:usize,w:usize,h:usize,x:usize,y:usize) -> (r:u8)
    requires 1<=w<=16383,1<=h<=16383,x<w,y<h,(w+1)/2<=stride,(h+1)/2<=height,base as int+stride as int*height<=p.len(),
    ensures r==chroma(p@,base,stride,w,h,x,y),
{
    let x0=if x==0{0}else{(x-1)/2};let x1=if (x+1)/2>(w-1)/2{(w-1)/2}else{(x+1)/2};
    let y0=if y==0{0}else{(y-1)/2};let y1=if (y+1)/2>(h-1)/2{(h-1)/2}else{(y+1)/2};
    let wx=if x%2==0{1u32}else{3u32};let wy=if y%2==0{1u32}else{3u32};
    let a=at(p,base,stride,height,x0,y0) as u32;let b=at(p,base,stride,height,x1,y0) as u32;
    let c=at(p,base,stride,height,x0,y1) as u32;let d=at(p,base,stride,height,x1,y1) as u32;
    assert(wx*wy*a+(4-wx)*wy*b+wx*(4-wy)*c+(4-wx)*(4-wy)*d<=4080) by(nonlinear_arith) requires 1<=wx<=3,1<=wy<=3,a<=255,b<=255,c<=255,d<=255;
    ((wx*wy*a+(4-wx)*wy*b+wx*(4-wy)*c+(4-wx)*(4-wy)*d+8)/16) as u8
}
pub(super) open spec fn sample(p:Seq<u8>,pw:usize,ph:usize,w:usize,h:usize,k:usize) -> [u8;4] {
    let x=k%w;let y=k/w;let ylen=pw*ph;let clen=(pw/2)*(ph/2);
    color(p[y*pw+x],chroma(p,ylen as usize,(pw/2) as usize,w,h,x,y),chroma(p,(ylen+clen) as usize,(pw/2) as usize,w,h,x,y))
}
#[inline(always)]
pub(super) fn pixel(p:&[u8],pw:usize,ph:usize,w:usize,h:usize,k:usize) -> (r:[u8;4])
    requires 1<=w<=16383,1<=h<=16383,w<=pw<=16384,h<=ph<=16384,pw%2==0,ph%2==0,k<w as int*h,
        pw as int*ph+2*(pw/2)*(ph/2)<=p.len(),
    ensures r==sample(p@,pw,ph,w,h,k),
{
    let x=k%w;let y=k/w;
    proof{vstd::arithmetic::div_mod::lemma_fundamental_div_mod(k as int,w as int);}
    assert(y<h) by(nonlinear_arith) requires k==w*y+x,0<=x<w,k<w*h,w>0;
    assert(pw*ph<=268435456) by(nonlinear_arith) requires pw<=16384,ph<=16384;
    assert((pw/2)*(ph/2)<=67108864) by(nonlinear_arith) requires pw<=16384,ph<=16384;
    let ylen=pw*ph;let clen=(pw/2)*(ph/2);
    assert(ylen+2*clen<=p.len()) by(nonlinear_arith) requires ylen==pw*ph,clen==(pw/2)*(ph/2),pw*ph+2*(pw/2)*(ph/2)<=p.len();
    let yy=at(p,0,pw,ph,x,y);
    let u=interpolate(p,ylen,pw/2,ph/2,w,h,x,y);let v=interpolate(p,ylen+clen,pw/2,ph/2,w,h,x,y);rgb(yy,u,v)
}
fn rgb4(y:[u8;4],u:[u8;4],v:[u8;4]) -> (r:[[u8;4];4])
    ensures forall|j:int|0<=j<4 ==> #[trigger] r@[j]==color(y@[j],u@[j],v@[j]),
{
    use verified_tiff_simd::{mul_shift8,add,sub,pack_shift6};
    let yy=mul_shift8(y,19077);let vr=mul_shift8(v,26149);
    let ug=mul_shift8(u,6419);let vg=mul_shift8(v,13320);let ub=mul_shift8(u,33050);
    let red=pack_shift6(sub(add(yy,vr),[14234;4]));
    let green=pack_shift6(add(sub(sub(yy,ug),vg),[8708;4]));
    let blue=pack_shift6(sub(add(yy,ub),[17685;4]));
    [[red[0],green[0],blue[0],255],[red[1],green[1],blue[1],255],
     [red[2],green[2],blue[2],255],[red[3],green[3],blue[3],255]]
}
// Four adjacent pixels share row geometry and execute RGB arithmetic in lanes.
// The caller proves that the group stays in a row; edge interpolation is the
// same verified scalar operation used by pixel().
pub(super) fn pixels4(p:&[u8],pw:usize,ph:usize,w:usize,h:usize,k:usize) -> (r:[[u8;4];4])
    requires 1<=w<=16383,1<=h<=16383,w<=pw<=16384,h<=ph<=16384,pw%2==0,ph%2==0,
        k<w as int*h,k%w+4<=w,pw as int*ph+2*(pw/2)*(ph/2)<=p.len(),
    ensures forall|j:int|0<=j<4 ==> #[trigger] r@[j]==sample(p@,pw,ph,w,h,(k+j) as usize),
{
    let x=k%w;let y=k/w;
    proof{vstd::arithmetic::div_mod::lemma_fundamental_div_mod(k as int,w as int);}
    assert(y<h) by(nonlinear_arith) requires k==w*y+x,0<=x<w,k<w*h,w>0;
    assert(pw*ph<=268435456) by(nonlinear_arith) requires pw<=16384,ph<=16384;
    assert((pw/2)*(ph/2)<=67108864) by(nonlinear_arith) requires pw<=16384,ph<=16384;
    let ylen=pw*ph;let clen=(pw/2)*(ph/2);
    assert(ylen+2*clen<=p.len()) by(nonlinear_arith) requires ylen==pw*ph,clen==(pw/2)*(ph/2),pw*ph+2*(pw/2)*(ph/2)<=p.len();
    let mut yy=[0u8;4];let mut u=[0u8;4];let mut v=[0u8;4];let mut j=0usize;
    while j<4
        invariant j<=4,1<=w<=16383,1<=h<=16383,w<=pw<=16384,h<=ph<=16384,pw%2==0,ph%2==0,
            k<w*h,k==w*y+x,x==k%w,y==k/w,y<h,x+4<=w,ylen==pw*ph,clen==(pw/2)*(ph/2),ylen+2*clen<=p.len(),
            forall|n:int|0<=n<j ==> #[trigger] yy@[n]==p@[y*pw+x+n],
            forall|n:int|0<=n<j ==> #[trigger] u@[n]==chroma(p@,ylen,(pw/2) as usize,w,h,(x+n) as usize,y),
            forall|n:int|0<=n<j ==> #[trigger] v@[n]==chroma(p@,(ylen+clen) as usize,(pw/2) as usize,w,h,(x+n) as usize,y),
        decreases 4-j,
    {
        assert(y*pw+x+j<p.len()) by(nonlinear_arith) requires y<h,h<=ph,x+4<=w,w<=pw,j<4,pw*ph<=p.len();
        yy[j]=at(p,0,pw,ph,x+j,y);
        assert(yy@[j as int]==p@[y*pw+x+j]);
        u[j]=interpolate(p,ylen,pw/2,ph/2,w,h,x+j,y);
        v[j]=interpolate(p,ylen+clen,pw/2,ph/2,w,h,x+j,y);j=j+1;
    }
    let r=rgb4(yy,u,v);
    assert forall|j:int|0<=j<4 implies #[trigger] r@[j]==sample(p@,pw,ph,w,h,(k+j) as usize) by {
        assert(k as int+j<w as int*h) by(nonlinear_arith) requires k==w*y+x,y<h,x+4<=w,0<=j<4,w>0;
        assert(w as int*h<=268435456) by(nonlinear_arith) requires w<=16383,h<=16383;
        assert(y as int*pw+x+j<p.len()) by(nonlinear_arith) requires y<h,h<=ph,x+4<=w,w<=pw,0<=j<4,pw*ph<=p.len();
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(k as int+j,w as int,y as int,x as int+j);
        assert(((k+j) as usize)/w==y && ((k+j) as usize)%w==x+j);
        assert(r@[j]==color(yy@[j],u@[j],v@[j]));
        assert(yy@[j]==p@[y*pw+x+j]);
        assert(u@[j]==chroma(p@,ylen,(pw/2) as usize,w,h,(x+j) as usize,y));
        assert(v@[j]==chroma(p@,(ylen+clen) as usize,(pw/2) as usize,w,h,(x+j) as usize,y));
    }
    r
}

}
