//! Exact VP8 deblocking kernel with explicit signed rounding and clipping.
//! Numerical algorithm follows libwebp 1.6.0; see third-party/libwebp.
use super::*;
use crate::vp8_predict as pr;
verus! {
pub(super) open spec fn absolute(v:int) -> int {if v<0{-v}else{v}}
#[inline(always)]
fn abs(v:i32) -> (r:i32)
    requires -255<=v<=255,ensures r==absolute(v as int),0<=r<=255,
{if v<0{-v}else{v}}
pub(super) open spec fn bounded(v:int,lo:int,hi:int) -> int {if v<lo{lo}else if v>hi{hi}else{v}}
#[inline(always)]
fn bound(v:i32,lo:i32,hi:i32) -> (r:i32)
    requires lo<=hi,ensures r==bounded(v as int,lo as int,hi as int),lo<=r<=hi,
{if v<lo{lo}else if v>hi{hi}else{v}}
pub(super) open spec fn divided(v:int,d:int) -> int {(v+1048576)/d-1048576int/d}
#[inline(always)]
fn div(v:i32,d:i32) -> (r:i32)
    requires -10000<=v<=10000,d==2 || d==8 || d==128,
    ensures r==divided(v as int,d as int),-5000<=r<=5000,
{(v+1048576)/d-1048576/d}
pub(super) open spec fn sample(e:Seq<u8>,simple:bool,edge:bool,limit:u8,inner:u8,threshold:u8,k:usize) -> u8 {
let p3=e[0] as int;let p2=e[1] as int;let p1=e[2] as int;let p0=e[3] as int;
    let q0=e[4] as int;let q1=e[5] as int;let q2=e[6] as int;let q3=e[7] as int;
    let ok=4*absolute(p0-q0)+absolute(p1-q1)<=2*limit as int+1;
    let smooth=absolute(p3-p2)<=inner as int && absolute(p2-p1)<=inner as int && absolute(p1-p0)<=inner as int &&
        absolute(q3-q2)<=inner as int && absolute(q2-q1)<=inner as int && absolute(q1-q0)<=inner as int;
    if !ok || (!simple && !smooth){e[k as int]}else{
        let hev=absolute(p1-p0)>threshold as int || absolute(q1-q0)>threshold as int;
        if simple || hev {
            let a=3*(q0-p0)+bounded(p1-q1,-128,127);
            let a1=bounded(divided(a+4,8),-16,15);let a2=bounded(divided(a+3,8),-16,15);
            if k==3{pr::clipped(p0+a2)}else if k==4{pr::clipped(q0-a1)}else{e[k as int]}
        }else if edge{
            let a=bounded(3*(q0-p0)+bounded(p1-q1,-128,127),-128,127);
            let a1=divided(27*a+63,128);let a2=divided(18*a+63,128);let a3=divided(9*a+63,128);
            if k==1{pr::clipped(p2+a3)}else if k==2{pr::clipped(p1+a2)}else if k==3{pr::clipped(p0+a1)}
            else if k==4{pr::clipped(q0-a1)}else if k==5{pr::clipped(q1-a2)}else if k==6{pr::clipped(q2-a3)}else{e[k as int]}
        }else{
            let a=3*(q0-p0);let a1=bounded(divided(a+4,8),-16,15);let a2=bounded(divided(a+3,8),-16,15);let a3=divided(a1+1,2);
            if k==2{pr::clipped(p1+a3)}else if k==3{pr::clipped(p0+a2)}else if k==4{pr::clipped(q0-a1)}else if k==5{pr::clipped(q1-a3)}else{e[k as int]}
        }
    }
}
#[inline(always)]
pub(super) fn pixel(e:&[u8;8],simple:bool,edge:bool,limit:u8,inner:u8,threshold:u8,k:usize) -> (r:u8)
    requires k<8,
    ensures r==sample(e@,simple,edge,limit,inner,threshold,k),
{
let p3=e[0] as i32;let p2=e[1] as i32;let p1=e[2] as i32;let p0=e[3] as i32;
    let q0=e[4] as i32;let q1=e[5] as i32;let q2=e[6] as i32;let q3=e[7] as i32;
    let ok=4*abs(p0-q0)+abs(p1-q1)<=2*limit as i32+1;
    let smooth=abs(p3-p2)<=inner as i32 && abs(p2-p1)<=inner as i32 && abs(p1-p0)<=inner as i32 &&
        abs(q3-q2)<=inner as i32 && abs(q2-q1)<=inner as i32 && abs(q1-q0)<=inner as i32;
    if !ok || (!simple && !smooth){e[k]}else{
        let hev=abs(p1-p0)>threshold as i32 || abs(q1-q0)>threshold as i32;
        if simple || hev {
            let a=3*(q0-p0)+bound(p1-q1,-128,127);
            let a1=bound(div(a+4,8),-16,15);let a2=bound(div(a+3,8),-16,15);
            if k==3{pr::clip(p0+a2)}else if k==4{pr::clip(q0-a1)}else{e[k]}
        }else if edge{
            let a=bound(3*(q0-p0)+bound(p1-q1,-128,127),-128,127);
            let a1=div(27*a+63,128);let a2=div(18*a+63,128);let a3=div(9*a+63,128);
            if k==1{pr::clip(p2+a3)}else if k==2{pr::clip(p1+a2)}else if k==3{pr::clip(p0+a1)}
            else if k==4{pr::clip(q0-a1)}else if k==5{pr::clip(q1-a2)}else if k==6{pr::clip(q2-a3)}else{e[k]}
        }else{
            let a=3*(q0-p0);let a1=bound(div(a+4,8),-16,15);let a2=bound(div(a+3,8),-16,15);let a3=div(a1+1,2);
            if k==2{pr::clip(p1+a3)}else if k==3{pr::clip(p0+a2)}else if k==4{pr::clip(q0-a1)}else if k==5{pr::clip(q1-a3)}else{e[k]}
        }
    }
}
#[inline(always)]
pub(super) fn filter(e:&[u8;8],simple:bool,edge:bool,limit:u8,inner:u8,threshold:u8) -> (out:[u8;8])
    ensures forall|k:int|0<=k<8 ==> #[trigger] out@[k]==sample(e@,simple,edge,limit,inner,threshold,k as usize),
{
    let mut out=*e;
    let p3=e[0] as i32;let p2=e[1] as i32;let p1=e[2] as i32;let p0=e[3] as i32;
    let q0=e[4] as i32;let q1=e[5] as i32;let q2=e[6] as i32;let q3=e[7] as i32;
    let ok=4*abs(p0-q0)+abs(p1-q1)<=2*limit as i32+1;
    let smooth=abs(p3-p2)<=inner as i32 && abs(p2-p1)<=inner as i32 && abs(p1-p0)<=inner as i32 &&
        abs(q3-q2)<=inner as i32 && abs(q2-q1)<=inner as i32 && abs(q1-q0)<=inner as i32;
    if ok && (simple || smooth){
        let hev=abs(p1-p0)>threshold as i32 || abs(q1-q0)>threshold as i32;
        if simple || hev {
            let a=3*(q0-p0)+bound(p1-q1,-128,127);
            let a1=bound(div(a+4,8),-16,15);let a2=bound(div(a+3,8),-16,15);
            out[3]=pr::clip(p0+a2);out[4]=pr::clip(q0-a1);
        }else if edge{
            let a=bound(3*(q0-p0)+bound(p1-q1,-128,127),-128,127);
            let a1=div(27*a+63,128);let a2=div(18*a+63,128);let a3=div(9*a+63,128);
            out[1]=pr::clip(p2+a3);out[2]=pr::clip(p1+a2);out[3]=pr::clip(p0+a1);
            out[4]=pr::clip(q0-a1);out[5]=pr::clip(q1-a2);out[6]=pr::clip(q2-a3);
        }else{
            let a=3*(q0-p0);let a1=bound(div(a+4,8),-16,15);let a2=bound(div(a+3,8),-16,15);let a3=div(a1+1,2);
            out[2]=pr::clip(p1+a3);out[3]=pr::clip(p0+a2);out[4]=pr::clip(q0-a1);out[5]=pr::clip(q1-a3);
        }
    }
    assert forall|k:int|0<=k<8 implies out@[k]==sample(e@,simple,edge,limit,inner,threshold,k as usize) by {}

    out
}
}
