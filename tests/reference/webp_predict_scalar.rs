//! Exact integer VP8L prediction and cross-color reconstruction. Pixel order
//! is RGBA; all component additions wrap modulo 256 as specified by VP8L.
use super::*;
use crate::webp_pixels::Pixel;
verus! {
pub(super) open spec fn average(a:u8,b:u8) -> u8 {((a as int+b as int)/2) as u8}
#[inline(always)]
fn avg(a:u8,b:u8) -> (r:u8) ensures r==average(a,b),{((a as u16+b as u16)/2) as u8}
pub(super) open spec fn clamp(a:int) -> u8 {if a<0{0u8}else if a>255{255u8}else{a as u8}}
#[inline(always)]
fn clip(a:i32) -> (r:u8) ensures r==clamp(a as int),{if a<0{0}else if a>255{255}else{a as u8}}
pub(super) open spec fn difference(a:u8,b:u8) -> int {if a>=b{a as int-b as int}else{b as int-a as int}}
#[inline(always)]
fn diff(a:u8,b:u8) -> (r:usize) ensures r==difference(a,b),r<=255,
{if a>=b{(a-b) as usize}else{(b-a) as usize}}
pub(super) open spec fn distance(a:Pixel,b:Pixel) -> int {difference(a[0],b[0])+difference(a[1],b[1])+difference(a[2],b[2])+difference(a[3],b[3])}
#[inline(always)]
fn dist(a:Pixel,b:Pixel) -> (r:usize) ensures r==distance(a,b),r<=1020,
{diff(a[0],b[0])+diff(a[1],b[1])+diff(a[2],b[2])+diff(a[3],b[3])}
pub(super) open spec fn component(mode:usize,l:u8,t:u8,tl:u8,tr:u8,choose_left:bool,alpha:bool) -> u8 {
    if mode==0{if alpha{255u8}else{0u8}}
    else if mode==1{l}else if mode==2{t}else if mode==3{tr}else if mode==4{tl}
    else if mode==5{average(average(l,tr),t)}else if mode==6{average(l,tl)}else if mode==7{average(l,t)}
    else if mode==8{average(tl,t)}else if mode==9{average(t,tr)}else if mode==10{average(average(l,tl),average(t,tr))}
    else if mode==11{if choose_left{l}else{t}}
    else if mode==12{clamp(l as int+t as int-tl as int)}
    else{let a=average(l,t) as int;let d=a-tl as int;clamp(a+if d<0{-((-d)/2)}else{d/2})}
}
#[inline(always)]
fn predict_one(mode:usize,l:u8,t:u8,tl:u8,tr:u8,choose_left:bool,alpha:bool) -> (r:u8)
    requires mode<=13,ensures r==component(mode,l,t,tl,tr,choose_left,alpha),
{
    if mode==0{if alpha{255}else{0}}
    else if mode==1{l}else if mode==2{t}else if mode==3{tr}else if mode==4{tl}
    else if mode==5{avg(avg(l,tr),t)}else if mode==6{avg(l,tl)}else if mode==7{avg(l,t)}
    else if mode==8{avg(tl,t)}else if mode==9{avg(t,tr)}else if mode==10{avg(avg(l,tl),avg(t,tr))}
    else if mode==11{if choose_left{l}else{t}}
    else if mode==12{clip(l as i32+t as i32-tl as i32)}
    else{let a=avg(l,t) as i32;let d=a-tl as i32;clip(a+if d<0{-((-d)/2)}else{d/2})}
}
pub(super) open spec fn prediction(mode:usize,l:Pixel,t:Pixel,tl:Pixel,tr:Pixel) -> Pixel {
    let left=distance(t,tl)<distance(l,tl);
    [component(mode,l[0],t[0],tl[0],tr[0],left,false),component(mode,l[1],t[1],tl[1],tr[1],left,false),
     component(mode,l[2],t[2],tl[2],tr[2],left,false),component(mode,l[3],t[3],tl[3],tr[3],left,true)]
}
#[inline(always)]
pub(super) fn predict(mode:usize,l:Pixel,t:Pixel,tl:Pixel,tr:Pixel) -> (r:Pixel)
    requires mode<=13,ensures r==prediction(mode,l,t,tl,tr),
{
    if mode==0{return [0,0,0,255];}
    if mode==1{return l;}if mode==2{return t;}if mode==3{return tr;}if mode==4{return tl;}
    if mode==12{return [clip(l[0] as i32+t[0] as i32-tl[0] as i32),clip(l[1] as i32+t[1] as i32-tl[1] as i32),clip(l[2] as i32+t[2] as i32-tl[2] as i32),clip(l[3] as i32+t[3] as i32-tl[3] as i32)];}
    let left=dist(t,tl)<dist(l,tl);
    [predict_one(mode,l[0],t[0],tl[0],tr[0],left,false),predict_one(mode,l[1],t[1],tl[1],tr[1],left,false),
     predict_one(mode,l[2],t[2],tl[2],tr[2],left,false),predict_one(mode,l[3],t[3],tl[3],tr[3],left,true)]
}
pub(super) open spec fn sum(a:Pixel,b:Pixel) -> Pixel {
    [((a[0] as int+b[0] as int)%256) as u8,((a[1] as int+b[1] as int)%256) as u8,
     ((a[2] as int+b[2] as int)%256) as u8,((a[3] as int+b[3] as int)%256) as u8]
}
#[inline(always)]
pub(super) fn add_pixel(a:Pixel,b:Pixel) -> (r:Pixel) ensures r==sum(a,b),
{
    [((a[0] as u16+b[0] as u16)%256) as u8,((a[1] as u16+b[1] as u16)%256) as u8,
     ((a[2] as u16+b[2] as u16)%256) as u8,((a[3] as u16+b[3] as u16)%256) as u8]
}
pub(super) open spec fn signed(a:u8) -> int {if a<128{a as int}else{a as int-256}}
pub(super) open spec fn delta(a:u8,b:u8) -> int {signed(a)*signed(b)/32}
#[inline(always)]
fn color_delta(a:u8,b:u8) -> (r:i32) ensures r==delta(a,b),-512<=r<=512,
{
    let x=if a<128{a as i32}else{a as i32-256};let y=if b<128{b as i32}else{b as i32-256};
    assert(-16384<=x*y<=16384) by(nonlinear_arith) requires -128<=x<=127,-128<=y<=127;
    let p=x*y;
    assert((p as int+16384)/32-512==p as int/32) by(nonlinear_arith);
    (p+16384)/32-512
}
pub(super) open spec fn green(p:Pixel) -> Pixel { [((p[0] as int+p[1] as int)%256) as u8,p[1],((p[2] as int+p[1] as int)%256) as u8,p[3]] }
#[inline(always)]
pub(super) fn add_green(p:Pixel) -> (r:Pixel) ensures r==green(p),
{[((p[0] as u16+p[1] as u16)%256) as u8,p[1],((p[2] as u16+p[1] as u16)%256) as u8,p[3]]}
pub(super) open spec fn color(p:Pixel,m:Pixel) -> Pixel {
    let red=((p[0] as int+delta(m[2],p[1]))%256) as u8;
    let blue=((p[2] as int+delta(m[1],p[1])+delta(m[0],red))%256) as u8;
    [red,p[1],blue,p[3]]
}
#[inline(always)]
pub(super) fn inverse_color(p:Pixel,m:Pixel) -> (r:Pixel) ensures r==color(p,m),
{
    let d=color_delta(m[2],p[1]);let red=((p[0] as i32+d+1024)%256) as u8;
    let a=color_delta(m[1],p[1]);let b=color_delta(m[0],red);let blue=((p[2] as i32+a+b+2048)%256) as u8;
    assert((p[0] as int+d+1024)%256==(p[0] as int+d)%256) by(nonlinear_arith);
    assert((p[2] as int+a+b+2048)%256==(p[2] as int+a+b)%256) by(nonlinear_arith);
    [red,p[1],blue,p[3]]
}
}
