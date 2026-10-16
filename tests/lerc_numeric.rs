use verified_tiff::Error;
use vstd::prelude::*;
#[path = "../src/lerc_numeric.rs"]
mod lerc_numeric;

#[test]
fn normalization_boundaries_match_hardware() {
    // Every possible highest-bit position, on both sides of normalization
    // boundaries. Hardware integer conversion is independent of the bit model.
    for bit in 0..128 {
        let power=1u128<<bit;
        for value in [0,power-1,power,power.saturating_add(1),power.saturating_add(1u128<<(bit/2)),u128::MAX] {
            for sign in [false,true] {
                let expected=if sign{-(value as f64)}else{value as f64};
                assert_eq!(lerc_numeric::pack(sign,value,0),expected.to_bits(),"normalize {value:x} sign {sign}");
            }
        }
    }
    // Float64 exponent boundaries and their adjacent representable values
    // exercise normal/subnormal float32 rounding, underflow and overflow.
    for exponent in 0..2047u64 {
        let power=exponent<<52;
        for bits in [power.saturating_sub(1),power,power+1] {
            for sign in [0,1u64<<63] {
                let bits=bits|sign;
                assert_eq!(lerc_numeric::narrow(bits),(f64::from_bits(bits) as f32).to_bits(),"narrow {bits:x}");
            }
        }
    }
}

#[test]
fn ieee_reconstruction_matches_hardware() {
    let mut state=0x372d7816adc82341u64;
    let edges=[0,1,0xfffffffffffff,0x10000000000000,0x10000000000001,
        0x3ff0000000000000,0x7fefffffffffffff,0x8000000000000000,
        0x8000000000000001,0xffefffffffffffff];
    for i in 0..100000 {
        state=state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let a=if i<edges.len(){edges[i]}else{state};
        state=state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let b=state;
        let x=f64::from_bits(a);let y=f64::from_bits(b);
        if x.is_finite() {
            assert_eq!(lerc_numeric::narrow(a),(x as f32).to_bits(),"narrow {a:x}");
            let (s,m,e)=lerc_numeric::parts(a);
            assert_eq!(lerc_numeric::pack(s,m,e),a,"round trip {a:x}");
            if (x*2.).is_finite() {
                let q=(b>>32) as u32;
                assert_eq!(lerc_numeric::scale(a,q),(q as f64*(2.*x)).to_bits(),"scale {a:x} {q}");
            }
            if y.is_finite() {
                assert_eq!(lerc_numeric::plus(a,b),(x+y).to_bits(),"sum {a:x} {b:x}");
            }
        }
        let f=(b as u32) as f32;
        let raw=f.to_bits();
        assert_eq!(lerc_numeric::number(raw as u64,6),(f as f64).to_bits());
        for k in 0..6 {
            let raw=match k {0|1=>b%256,2|3=>b%65536,_=>b%4294967296};
            let reference=match k {0=>(raw as i8) as f64,1=>(raw as u8) as f64,
                2=>(raw as i16) as f64,3=>(raw as u16) as f64,
                4=>(raw as i32) as f64,_=>(raw as u32) as f64};
            let value=lerc_numeric::number(raw,k);
            assert_eq!(value,reference.to_bits());
            assert_eq!(lerc_numeric::to_integer(value,k),Ok(raw));
        }
    }
    for &a in &edges {for &b in &edges {
        let got=lerc_numeric::plus(a,b);let expected=(f64::from_bits(a)+f64::from_bits(b)).to_bits();
        assert_eq!(got,expected,"edge sum {a:x} {b:x}");
    }}
}

#[test]
fn normal_narrowing_ties_carries_and_fallback_boundaries() {
    for exponent in [896u64,897,898,1022,1023,1149,1150,1151] {
        for fraction in [0u64,1,0x1fffff,0x3fffff,0x7ffffe,0x7fffff] {
            for remainder in [0u64,0x0fffffff,0x10000000,0x10000001,0x1fffffff] {
                for sign in [0u64,1u64<<63] {
                    let bits=sign|(exponent<<52)|(fraction<<29)|remainder;
                    assert_eq!(lerc_numeric::narrow(bits),(f64::from_bits(bits) as f32).to_bits(),"tie/carry {bits:x}");
                }
            }
        }
    }
}

#[test]
fn word_addition_alignment_cancellation_and_rounding() {
    // Sweep exponent separations through both retained-word and original-model
    // shift boundaries, with independent hardware addition as the oracle.
    let fractions=[0u64,1,0x7ffffffffffff,0xfffffffffffff];
    let gaps=[0u64,1,2,7,8,9,10,51,52,53,54,55,56,63,64,65,115,116,117,127,128,129,2046];
    for exp in 0..2047u64 {
        for gap in gaps {if gap>exp{continue;}
            for fa in fractions {for fb in fractions {
                for sa in [0u64,1u64<<63] {for sb in [0u64,1u64<<63] {
                    let a=sa|(exp<<52)|fa;let b=sb|((exp-gap)<<52)|fb;
                    let expected=(f64::from_bits(a)+f64::from_bits(b)).to_bits();
                    assert_eq!(lerc_numeric::plus(a,b),expected,"aligned {a:x} {b:x}");
                    assert_eq!(lerc_numeric::plus(b,a),expected,"reversed {b:x} {a:x}");
                }}
            }}
        }
    }
    // Cancellation on either side of the narrow path's guard, including exact
    // equality, sign changes, and tiny differences that require the fallback.
    for exp in [1u64,2,52,53,54,1022,1023,2045,2046] {
        for delta in [0u64,1,2,(1<<45)-1,1<<45,(1<<46)-1,1<<46,(1<<46)+1,(1<<47)-1] {
            for start in [0u64,1,(1<<51)-1] {
                let a=(exp<<52)|start;let b=(exp<<52)|(start+delta);
                for (x,y) in [(a,b|(1<<63)),(b,a|(1<<63)),(a|(1<<63),b)] {
                    assert_eq!(lerc_numeric::plus(x,y),(f64::from_bits(x)+f64::from_bits(y)).to_bits(),"cancel {x:x} {y:x}");
                }
            }
        }
    }
}

#[test]
fn scaled_products_cross_word_and_ieee_boundaries() {
    let mut quantized=vec![0u32,u32::MAX];
    for bit in 0..32 {let p=1u32<<bit;quantized.extend([p-1,p,p.saturating_add(1)]);}
    for exp in 0..2046u64 {
        for fraction in [0u64,1,0x7ffffffffffff,0xfffffffffffff] {
            for sign in [0u64,1u64<<63] {
                let bits=sign|(exp<<52)|fraction;let doubled=2.*f64::from_bits(bits);
                assert!(doubled.is_finite());
                for &q in &quantized {
                    assert_eq!(lerc_numeric::scale(bits,q),(f64::from(q)*doubled).to_bits(),"product {bits:x} {q}");
                }
            }
        }
    }
}

// The old verified software implementations are independent executable oracles
// for the newly trusted hardware contracts. Comparing hardware with itself is
// not a meaningful test of the model-to-machine boundary.
#[test]
fn native_primitives_match_software_reference() {
    let mut state=0xaca523497921ff3du64;
    let edges=[0,1,0xfffffffffffff,0x10000000000000,0x10000000000001,
        0x3fefffffffffffff,0x3ff0000000000000,0x7fdfffffffffffff,
        0x7fe0000000000000,0x7fefffffffffffff,0x7ff0000000000000,
        0x7ff0000000000001,0x7ff8000000000001,0x7fffffffffffffff];
    let check=|a:u64,b:u64,q:u32| {
        assert_eq!(lerc_numeric::plus(a,b),lerc_numeric::plus_software(a,b),"sum {a:x} {b:x}");
        assert_eq!(lerc_numeric::scale(a,q),lerc_numeric::scale_software(a,q),"scale {a:x} {q}");
        assert_eq!(lerc_numeric::narrow(a),lerc_numeric::narrow_software(a),"narrow {a:x}");
    };
    for a in edges {for b in edges {for signa in [0,1u64<<63] {for signb in [0,1u64<<63] {
        for q in [0,1,2,3,65535,65536,0x7fffffff,0x80000000,u32::MAX] {check(a|signa,b|signb,q);}
    }}}}
    for exp in 0..2048u64 {
        for frac in [0,1,0x7ffffffffffff,0xfffffffffffff] {
            for sign in [0,1u64<<63] {
                let a=(exp<<52)|frac|sign;
                for gap in [0,1,2,8,9,51,52,53,54,55,63,64,65,115,116,117,127,128,2047] {
                    let b=(exp.saturating_sub(gap)<<52)|frac;
                    check(a,b,0xffffffff);
                    check(a,b^(1u64<<63),0x80000001);
                }
            }
        }
    }
    for _ in 0..200000 {
        state=state.wrapping_mul(6364136223846793005).wrapping_add(1);let a=state;
        state=state.wrapping_mul(6364136223846793005).wrapping_add(1);let b=state;
        check(a,b,(b>>32) as u32);
        for k in 0..8 {
            let raw=match k {0|1=>b%256,2|3=>b%65536,4|5|6=>b%4294967296,_=>b};
            assert_eq!(lerc_numeric::number(raw,k),lerc_numeric::number_software(raw,k),"number {raw:x} {k}");
        }
    }
    // Every binary32 exponent, exceptional payloads and signed zero.
    for exp in 0..256u64 {for frac in [0,1,0x3fffff,0x7fffff] {for sign in [0,1u64<<31] {
        let raw=(exp<<23)|frac|sign;
        assert_eq!(lerc_numeric::number(raw,6),lerc_numeric::number_software(raw,6));
    }}}
}
