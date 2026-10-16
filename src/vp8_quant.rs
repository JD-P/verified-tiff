//! Segment quantizers and per-plane coefficient scales.
use super::*;
use crate::vp8_tables::{DC_QUANT,AC_QUANT};
verus! {
pub(super) open spec fn clipped(v:int,max:int) -> int {if v<0{0}else if v>max{max}else{v}}
fn clip(v:i32,max:usize) -> (r:usize)
    requires max<=127,
    ensures r==clipped(v as int,max as int),r<=max,
{if v<0{0}else if v as usize>max{max}else{v as usize}}
pub(super) open spec fn qbase(h:Seq<i32>,seg:usize) -> int {
    if h[2]==0{h[31] as int}else{h[6+seg] as int+if h[5]==0{h[31] as int}else{0int}}
}
pub(super) open spec fn factor(h:Seq<i32>,seg:usize,plane:usize,ac:bool) -> u16 {
    let q=qbase(h,seg);
    if plane==0{if ac{AC_QUANT[clipped(q,127)]}else{DC_QUANT[clipped(q+h[32],127)]}}
    else if plane==1{if ac{let v=AC_QUANT[clipped(q+h[34],127)] as int*101581/65536;(if v<8{8int}else{v}) as u16}
        else{(2*DC_QUANT[clipped(q+h[33],127)]) as u16}}
    else{if ac{AC_QUANT[clipped(q+h[36],127)]}else{DC_QUANT[clipped(q+h[35],117)]}}
}
pub(super) fn scale(h:&[i32;1096],seg:usize,plane:usize,ac:bool) -> (r:u16)
    requires seg<4,plane<3,forall|i:int|0<=i<1096 ==> -255<=#[trigger] h@[i]<=255,
    ensures r==factor(h@,seg,plane,ac),
{
    let q=if h[2]==0{h[31]}else{h[6+seg]+if h[5]==0{h[31]}else{0}};
    if plane==0{if ac{AC_QUANT[clip(q,127)]}else{DC_QUANT[clip(q+h[32],127)]}}
    else if plane==1{if ac{let v=AC_QUANT[clip(q+h[34],127)] as u64*101581/65536;(if v<8{8}else{v}) as u16}
        else{(2*DC_QUANT[clip(q+h[33],127)] as u32) as u16}}
    else{if ac{AC_QUANT[clip(q+h[36],127)]}else{DC_QUANT[clip(q+h[35],117)]}}
}
}
