use verified_tiff::Error;
use vstd::prelude::*;
fn bit_divisor(n:usize)->u8 {1<<n}
#[path="../src/zstd_bits.rs"] mod zstd_bits;
#[path="../src/lerc_word.rs"] mod lerc_word;

#[test]
fn every_width_alignment_and_short_ending_matches_independent_bits(){
    let mut state=0x12345678abcdef91u64;
    for _ in 0..64 {
        let mut data=[0u8;16];
        for b in &mut data {state=state.wrapping_mul(6364136223846793005).wrapping_add(1);*b=(state>>32) as u8;}
        for end in 0..=16 {for p in 0..=end*8 {for n in 0..=32 {
            let got=lerc_word::read(&data,p,end,n);
            if p+n>end*8 {assert_eq!(got,Err(Error::Truncated));}
            else {
                let mut wanted=0u32;
                for i in 0..n {wanted|=(((data[(p+i)/8]>>((p+i)%8))&1) as u32)<<i;}
                assert_eq!(got,Ok(wanted),"end={end} p={p} n={n}");
            }
        }}}
    }
}
