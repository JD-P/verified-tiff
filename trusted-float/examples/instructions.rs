//! Disassembly witness for the actual trusted primitive bodies.
use verified_tiff_float as fp;
#[no_mangle]
pub extern "C" fn native_add(a:u64,b:u64)->u64 {fp::add(a,b)}
#[no_mangle]
pub extern "C" fn native_scale(a:u64,q:u32)->u64 {fp::mul_twice_u32(a,q)}
#[no_mangle]
pub extern "C" fn native_narrow(a:u64)->u32 {fp::narrow(a)}
#[no_mangle]
pub extern "C" fn native_widen(a:u32)->u64 {fp::widen(a)}
#[no_mangle]
pub extern "C" fn native_integer(a:u32,negative:bool)->u64 {fp::from_magnitude(a,negative)}
fn main() {
    let a=std::hint::black_box(1f64.to_bits());
    std::hint::black_box(native_add(a,a));
    std::hint::black_box(native_scale(a,17));
    std::hint::black_box(native_narrow(a));
    std::hint::black_box(native_widen(1f32.to_bits()));
    std::hint::black_box(native_integer(17,true));
}
