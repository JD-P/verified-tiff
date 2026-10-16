//! Lane composition for VP8L spans; codec semantics remain checked here.
use super::*;
use crate::webp_pixels::Pixel;
use crate::webp_predict as pred;
verus! {
#[inline(always)]
pub(super) fn predict12(residual:Pixel,left:Pixel,top:Pixel,top_left:Pixel) -> (r:Pixel)
    ensures r==pred::sum(residual,pred::prediction(12,left,top,top_left,[0u8,0,0,0])),
{
    let p=verified_tiff_simd::clamp_add_sub(left,top,top_left);
    verified_tiff_simd::add_bytes(residual,p)
}
#[inline(always)]
fn widen(a:[u8;4]) -> (r:[i32;4])
    ensures forall|j:int|0<=j<4 ==> #[trigger] r@[j]==a@[j] as int,
{[a[0] as i32,a[1] as i32,a[2] as i32,a[3] as i32]}
#[inline(always)]
pub(super) fn color4(p:[Pixel;4],m:Pixel) -> (r:[Pixel;4])
    ensures forall|j:int|0<=j<4 ==> #[trigger] r@[j]==pred::color(p@[j],m),
{
    use verified_tiff_simd::{add,signed_product_shift5,wrap_bytes};
    let g=[p[0][1],p[1][1],p[2][1],p[3][1]];
    let red=[p[0][0],p[1][0],p[2][0],p[3][0]];let blue=[p[0][2],p[1][2],p[2][2],p[3][2]];
    let dr=signed_product_shift5([m[2];4],g);
    let red=wrap_bytes(add(widen(red),dr));
    let db=signed_product_shift5([m[1];4],g);let rb=signed_product_shift5([m[0];4],red);
    let blue=wrap_bytes(add(add(widen(blue),db),rb));
    [[red[0],p[0][1],blue[0],p[0][3]],[red[1],p[1][1],blue[1],p[1][3]],
     [red[2],p[2][1],blue[2],p[2][3]],[red[3],p[3][1],blue[3],p[3][3]]]
}
#[inline(always)]
pub(super) fn green4(p:[Pixel;4]) -> (r:[Pixel;4])
    ensures forall|j:int|0<=j<4 ==> #[trigger] r@[j]==pred::green(p@[j]),
{
    let a=[p[0][0],p[0][1],p[0][2],p[0][3],p[1][0],p[1][1],p[1][2],p[1][3],
           p[2][0],p[2][1],p[2][2],p[2][3],p[3][0],p[3][1],p[3][2],p[3][3]];
    let b=[p[0][1],0,p[0][1],0,p[1][1],0,p[1][1],0,p[2][1],0,p[2][1],0,p[3][1],0,p[3][1],0];
    let v=verified_tiff_simd::add_bytes16(a,b);
    [[v[0],v[1],v[2],v[3]],[v[4],v[5],v[6],v[7]],[v[8],v[9],v[10],v[11]],[v[12],v[13],v[14],v[15]]]
}
}
