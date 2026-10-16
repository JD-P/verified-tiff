//! TRUSTED integer lane operations. Bodies are not verified by Verus.
//! Fixed values only: no caller pointers, buffer ranges, or codec decisions.
//! See docs/webp-simd.md and audit.json.
use vstd::prelude::*;
mod cpu;
verus! {
pub open spec fn bounded(a:[i32;4],limit:int) -> bool {
    forall|j:int|0<=j<4 ==> -limit<=#[trigger] a@[j]<=limit
}
pub open spec fn clipped(v:int) -> u8 {if v<0{0}else if v>255{255}else{v as u8}}
#[verifier::external_body]
#[inline(always)]
pub fn available() -> bool { cpu::available() }
#[verifier::external_body]
#[inline(always)]
pub fn add(a:[i32;4],b:[i32;4]) -> (r:[i32;4])
    requires bounded(a,4000000),bounded(b,4000000),
    ensures forall|j:int|0<=j<4 ==> #[trigger] r@[j]==a@[j] as int+b@[j] as int,
{cpu::add(a,b)}
#[verifier::external_body]
#[inline(always)]
pub fn sub(a:[i32;4],b:[i32;4]) -> (r:[i32;4])
    requires bounded(a,4000000),bounded(b,4000000),
    ensures forall|j:int|0<=j<4 ==> #[trigger] r@[j]==a@[j] as int-b@[j] as int,
{cpu::sub(a,b)}
#[verifier::external_body]
#[inline(always)]
pub fn mul_shift16(a:[i32;4],c:u32) -> (r:[i32;4])
    requires bounded(a,1000000),c<=65535,
    ensures forall|j:int|0<=j<4 ==> #[trigger] r@[j]==(a@[j] as int*c as int)/65536,
{cpu::mul_shift16(a,c)}
#[verifier::external_body]
#[inline(always)]
pub fn shift3(a:[i32;4]) -> (r:[i32;4])
    ensures forall|j:int|0<=j<4 ==> #[trigger] r@[j]==(a@[j] as int)/8,
{cpu::shift3(a)}
#[verifier::external_body]
#[inline(always)]
pub fn mul_shift8(a:[u8;4],c:u16) -> (r:[i32;4])
    ensures forall|j:int|0<=j<4 ==> #[trigger] r@[j]==(a@[j] as int*c as int)/256,
{cpu::mul_shift8(a,c)}
#[verifier::external_body]
#[inline(always)]
pub fn pack_shift6(a:[i32;4]) -> (r:[u8;4])
    ensures forall|j:int|0<=j<4 ==> #[trigger] r@[j]==clipped(a@[j] as int/64),
{cpu::pack_shift6(a)}
pub open spec fn signed_byte(x:u8) -> int {if x<128{x as int}else{x as int-256}}
#[verifier::external_body]
#[inline(always)]
pub fn add_bytes(a:[u8;4],b:[u8;4]) -> (r:[u8;4])
    ensures forall|j:int|0<=j<4 ==> #[trigger] r@[j]==((a@[j] as int+b@[j] as int)%256) as u8,
{cpu::add_bytes(a,b)}
#[verifier::external_body]
#[inline(always)]
pub fn clamp_add_sub(a:[u8;4],b:[u8;4],c:[u8;4]) -> (r:[u8;4])
    ensures forall|j:int|0<=j<4 ==> #[trigger] r@[j]==clipped(a@[j] as int+b@[j] as int-c@[j] as int),
{cpu::clamp_add_sub(a,b,c)}
#[verifier::external_body]
#[inline(always)]
pub fn signed_product_shift5(a:[u8;4],b:[u8;4]) -> (r:[i32;4])
    ensures forall|j:int|0<=j<4 ==> #[trigger] r@[j]==signed_byte(a@[j])*signed_byte(b@[j])/32 && -512<=r@[j]<=512,
{cpu::signed_product_shift5(a,b)}
#[verifier::external_body]
#[inline(always)]
pub fn wrap_bytes(a:[i32;4]) -> (r:[u8;4])
    ensures forall|j:int|0<=j<4 ==> #[trigger] r@[j]==((a@[j] as int)%256) as u8,
{cpu::wrap_bytes(a)}
#[verifier::external_body]
#[inline(always)]
pub fn add_bytes16(a:[u8;16],b:[u8;16]) -> (r:[u8;16])
    ensures forall|j:int|0<=j<16 ==> #[trigger] r@[j]==((a@[j] as int+b@[j] as int)%256) as u8,
{cpu::add_bytes16(a,b)}

}
