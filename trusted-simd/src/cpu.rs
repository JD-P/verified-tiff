//! Audited implementation of fixed-size integer operations.
//! SSE2 calls are guarded; all loads/stores are value transmutations of exactly
//! 16 bytes, or register moves of exactly four bytes. No memory overreads.
#[inline(always)]
pub fn available() -> bool {
    if cfg!(feature="force-scalar") {return false;}
    #[cfg(target_arch="x86_64")] {return true;}
    #[cfg(target_arch="x86")] {return std::is_x86_feature_detected!("sse2");}
    #[cfg(not(any(target_arch="x86",target_arch="x86_64")))] {false}
}
#[cfg(any(target_arch="x86",target_arch="x86_64"))]
mod sse2 {
    #[cfg(target_arch="x86")] use std::arch::x86::*;
    #[cfg(target_arch="x86_64")] use std::arch::x86_64::*;
    #[inline(always)] unsafe fn load(a:[i32;4])->__m128i {std::mem::transmute(a)}
    #[inline(always)] unsafe fn save(a:__m128i)->[i32;4] {std::mem::transmute(a)}
    #[inline(always)] unsafe fn bytes(a:[u8;4])->__m128i {_mm_cvtsi32_si128(i32::from_le_bytes(a))}
    #[inline(always)] unsafe fn low(a:__m128i)->[u8;4] {_mm_cvtsi128_si32(a).to_le_bytes()}
    #[inline]
    #[target_feature(enable="sse2")]
    pub unsafe fn add(a:[i32;4],b:[i32;4])->[i32;4] {save(_mm_add_epi32(load(a),load(b)))}
    #[inline]
    #[target_feature(enable="sse2")]
    pub unsafe fn sub(a:[i32;4],b:[i32;4])->[i32;4] {save(_mm_sub_epi32(load(a),load(b)))}
    #[inline]
    #[target_feature(enable="sse2")]
    pub unsafe fn shift3(a:[i32;4])->[i32;4] {save(_mm_srai_epi32::<3>(load(a)))}
    #[inline]
    #[target_feature(enable="sse2")]
    pub unsafe fn mul_shift16(a:[i32;4],c:u32)->[i32;4] {
        // SSE2 multiplies unsigned even u32 lanes to u64. Correct signed
        // negatives by subtracting c*2^32; logical >>16 then retains the exact
        // low i32 floor quotient. Odd lanes use a shifted copy of the input.
        let x=load(a);let k=_mm_set1_epi32(c as i32);
        let sign=_mm_srai_epi32::<31>(x);
        let corr=_mm_and_si128(sign,k);
        let even=_mm_sub_epi64(_mm_mul_epu32(x,k),_mm_slli_epi64::<32>(corr));
        let odd=_mm_sub_epi64(_mm_mul_epu32(_mm_srli_si128::<4>(x),k),_mm_slli_epi64::<32>(_mm_srli_si128::<4>(corr)));
        let even=_mm_shuffle_epi32::<0xd8>(_mm_srli_epi64::<16>(even));
        let odd=_mm_shuffle_epi32::<0xd8>(_mm_srli_epi64::<16>(odd));
        save(_mm_unpacklo_epi32(even,odd))
    }
    #[inline]
    #[target_feature(enable="sse2")]
    pub unsafe fn mul_shift8(a:[u8;4],c:u16)->[i32;4] {
        let z=_mm_setzero_si128();
        let x=_mm_slli_epi16::<8>(_mm_unpacklo_epi8(bytes(a),z));
        let product=_mm_mulhi_epu16(x,_mm_set1_epi16(c as i16));
        save(_mm_unpacklo_epi16(product,z))
    }
    #[inline]
    #[target_feature(enable="sse2")]
    pub unsafe fn pack_shift6(a:[i32;4])->[u8;4] {
        let z=_mm_setzero_si128();
        let x=_mm_srai_epi32::<6>(load(a));
        low(_mm_packus_epi16(_mm_packs_epi32(x,z),z))
    }
    #[inline]
    #[target_feature(enable="sse2")]
    pub unsafe fn add_bytes(a:[u8;4],b:[u8;4])->[u8;4] {low(_mm_add_epi8(bytes(a),bytes(b)))}
    #[inline]
    #[target_feature(enable="sse2")]
    pub unsafe fn clamp_add_sub(a:[u8;4],b:[u8;4],c:[u8;4])->[u8;4] {
        let z=_mm_setzero_si128();
        let x=_mm_unpacklo_epi8(bytes(a),z);let y=_mm_unpacklo_epi8(bytes(b),z);let t=_mm_unpacklo_epi8(bytes(c),z);
        low(_mm_packus_epi16(_mm_sub_epi16(_mm_add_epi16(x,y),t),z))
    }
    #[inline]
    #[target_feature(enable="sse2")]
    pub unsafe fn signed_product_shift5(a:[u8;4],b:[u8;4])->[i32;4] {
        let z=_mm_setzero_si128();let x=bytes(a);let y=bytes(b);
        let xx=_mm_unpacklo_epi8(x,_mm_cmpgt_epi8(z,x));
        let yy=_mm_unpacklo_epi8(y,_mm_cmpgt_epi8(z,y));
        let product=_mm_srai_epi16::<5>(_mm_mullo_epi16(xx,yy));
        save(_mm_unpacklo_epi16(product,_mm_srai_epi16::<15>(product)))
    }
    #[inline]
    #[target_feature(enable="sse2")]
    pub unsafe fn wrap_bytes(a:[i32;4])->[u8;4] {
        let z=_mm_setzero_si128();let low_byte=_mm_and_si128(load(a),_mm_set1_epi32(255));
        low(_mm_packus_epi16(_mm_packs_epi32(low_byte,z),z))
    }
    #[inline]
    #[target_feature(enable="sse2")]
    pub unsafe fn add_bytes16(a:[u8;16],b:[u8;16])->[u8;16] {
        std::mem::transmute(_mm_add_epi8(std::mem::transmute(a),std::mem::transmute(b)))
    }

}
macro_rules! dispatch {
    ($op:ident, $($arg:ident),+) => {
        #[cfg(any(target_arch="x86",target_arch="x86_64"))]
        if available() {return unsafe{sse2::$op($($arg),+)};}
    }
}
#[inline(always)] pub fn add(a:[i32;4],b:[i32;4])->[i32;4] {dispatch!(add,a,b);std::array::from_fn(|i|a[i]+b[i])}
#[inline(always)] pub fn sub(a:[i32;4],b:[i32;4])->[i32;4] {dispatch!(sub,a,b);std::array::from_fn(|i|a[i]-b[i])}
#[inline(always)] pub fn mul_shift16(a:[i32;4],c:u32)->[i32;4] {dispatch!(mul_shift16,a,c);std::array::from_fn(|i|((a[i] as i64*c as i64)>>16) as i32)}
#[inline(always)] pub fn shift3(a:[i32;4])->[i32;4] {dispatch!(shift3,a);a.map(|v|v>>3)}
#[inline(always)] pub fn mul_shift8(a:[u8;4],c:u16)->[i32;4] {dispatch!(mul_shift8,a,c);a.map(|v|v as i32*c as i32/256)}
#[inline(always)] pub fn pack_shift6(a:[i32;4])->[u8;4] {dispatch!(pack_shift6,a);a.map(|v|(v>>6).clamp(0,255) as u8)}

#[inline(always)] pub fn add_bytes(a:[u8;4],b:[u8;4])->[u8;4] {dispatch!(add_bytes,a,b);std::array::from_fn(|i|a[i].wrapping_add(b[i]))}
#[inline(always)] pub fn clamp_add_sub(a:[u8;4],b:[u8;4],c:[u8;4])->[u8;4] {dispatch!(clamp_add_sub,a,b,c);std::array::from_fn(|i|(a[i] as i32+b[i] as i32-c[i] as i32).clamp(0,255) as u8)}
#[inline(always)] pub fn signed_product_shift5(a:[u8;4],b:[u8;4])->[i32;4] {dispatch!(signed_product_shift5,a,b);std::array::from_fn(|i|(a[i] as i8 as i32*b[i] as i8 as i32)>>5)}
#[inline(always)] pub fn wrap_bytes(a:[i32;4])->[u8;4] {dispatch!(wrap_bytes,a);a.map(|v|v as u8)}
#[inline(always)] pub fn add_bytes16(a:[u8;16],b:[u8;16])->[u8;16] {dispatch!(add_bytes16,a,b);std::array::from_fn(|i|a[i].wrapping_add(b[i]))}
