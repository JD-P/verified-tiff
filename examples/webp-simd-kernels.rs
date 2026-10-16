//! Diagnostic transform-only timings; excludes TIFF, entropy, and rendering.
#![allow(dead_code,unused_imports,unused_variables)]
use verified_tiff::Error;
use vstd::prelude::*;
fn bit_divisor(n:usize)->u8{1u8<<n}
#[path="../src/zstd_bits.rs"] mod zstd_bits;
#[path="../src/vp8_bool.rs"] mod vp8_bool;
#[path="../src/vp8_tree.rs"] mod vp8_tree;
#[path="../src/vp8_coeff.rs"] mod vp8_coeff;
#[path="../src/vp8_transform.rs"] mod current;
#[path="../tests/reference/vp8_transform_scalar.rs"] mod previous;
use std::{hint::black_box,time::{Instant,Duration}};
fn measure(f:fn(&[i32;16],bool)->[i32;16],blocks:&[[i32;16]],walsh:bool)->f64 {
    let start=Instant::now();let mut count=0;
    while start.elapsed()<Duration::from_millis(150) {
        for c in blocks {black_box(f(black_box(c),black_box(walsh)));}
        count+=blocks.len();
    }
    start.elapsed().as_secs_f64()*1e9/count as f64
}
fn main() {
    let mut seed=0x718209af14922d65u64;
    let mut next=||{seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;seed};
    for kind in ["dct","walsh","dc"] {
        let blocks:Vec<[i32;16]>=(0..4096).map(|_|std::array::from_fn(|k|if kind=="dc" && k!=0{0}else{(next()%4096) as i32-2048})).collect();
        let walsh=kind=="walsh";
        for c in &blocks {assert_eq!(current::transform(c,walsh),previous::transform(c,walsh));}
        for round in 0..9 {
            for i in 0..2 {
                let before=(round+i)%2==0;
                let f=if before{previous::transform}else{current::transform};
                let ns=measure(black_box(f),&blocks,walsh);
                println!("{{\"kernel\":\"{}\",\"round\":{},\"engine\":\"{}\",\"ns\":{}}}",kind,round,if before{"before"}else{"after"},ns);
            }
        }
    }
}
