//! TRUSTED: native IEEE-754 primitives, not a verified implementation.
//! See docs/native-float.md for the platform/environment assumptions and audit.
//! No input buffers, allocation, codec logic, or unsafe Rust lives here.
#![forbid(unsafe_code)]
use vstd::prelude::*;
pub mod model;
use model::*;
verus! {
#[verifier::external_body]
#[inline(always)]
pub fn add(a:u64,b:u64) -> (r:u64)
    requires finite(a),finite(b),
    ensures r==sum(a,b),
{
    (f64::from_bits(a)+f64::from_bits(b)).to_bits()
}
#[verifier::external_body]
#[inline(always)]
pub fn mul_twice_u32(a:u64,q:u32) -> (r:u64)
    requires finite(a),
    ensures r==scaled(a,q),
{
    // 2*q fits 33 bits and converts exactly. One rounded multiply also
    // preserves the model when an intermediate 2*a would overflow.
    (f64::from_bits(a)*((q as u64*2) as f64)).to_bits()
}
#[verifier::external_body]
#[inline(always)]
pub fn narrow(a:u64) -> (r:u32)
    requires finite(a),
    ensures r==narrowed(a),
{
    (f64::from_bits(a) as f32).to_bits()
}
#[verifier::external_body]
#[inline(always)]
pub fn widen(a:u32) -> (r:u64)
    requires (a/8388608)%256!=255,
    ensures r==value(a as u64,6),
{
    (f32::from_bits(a) as f64).to_bits()
}
#[verifier::external_body]
#[inline(always)]
pub fn from_magnitude(a:u32,negative:bool) -> (r:u64)
    ensures r==packed(negative,a as u128,0),
{
    let value=a as f64;
    (if negative{-value}else{value}).to_bits()
}
}
