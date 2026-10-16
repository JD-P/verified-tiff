use verified_tiff_simd::*;
#[test]
fn color_multiply_exhaustive_and_pack_edges() {
    for a in 0..=255u16 {for c in 0..=65535u16 {
        let x=[a as u8,255-a as u8,(a/2) as u8,(a^137) as u8];
        assert_eq!(mul_shift8(x,c),x.map(|v|v as i32*c as i32/256));
    }}
    for x in [i32::MIN,-100000,-1,0,1,63,64,16319,16320,16383,16384,100000,i32::MAX] {
        let a=[x,x.saturating_add(1),x.saturating_sub(1),x/2];
        assert_eq!(pack_shift6(a),a.map(|v|v.div_euclid(64).clamp(0,255) as u8));
    }
}
#[test]
fn wide_lanes_boundaries_and_random() {
    let edge=[-1000000,-200000,-65537,-65536,-32768,-1,0,1,32767,65535,200000,1000000];
    for a in edge {for c in [0,1,20091,32768,35468,65535] {
        let x=[a,-a,a/2,-a/3];
        assert_eq!(mul_shift16(x,c),x.map(|v|(v as i64*c as i64).div_euclid(65536) as i32));
    }}
    let mut rng=0xf1251aebd3416749u64;
    let mut next=||{rng^=rng<<13;rng^=rng>>7;rng^=rng<<17;rng};
    for _ in 0..200000 {
        let a=std::array::from_fn(|_|(next()%2000001) as i32-1000000);
        let b=std::array::from_fn(|_|(next()%8000001) as i32-4000000);
        let c=(next()%65536) as u32;
        assert_eq!(add(a,b),std::array::from_fn(|i|a[i]+b[i]));
        assert_eq!(sub(a,b),std::array::from_fn(|i|a[i]-b[i]));
        assert_eq!(mul_shift16(a,c),a.map(|v|(v as i64*c as i64).div_euclid(65536) as i32));
        assert_eq!(shift3(b),b.map(|v|v.div_euclid(8)));
    }
}
#[test]
fn dispatch_configuration() {
    if cfg!(feature="force-scalar") {assert!(!available());}
    else if cfg!(target_arch="x86_64") {assert!(available());}
}
#[test]
fn lossless_integer_operations() {
    for a in 0..=255u16 {for b in 0..=255u16 {
        let x=[a as u8,b as u8,255-a as u8,255-b as u8];let y=[b as u8,a as u8,b as u8,a as u8];
        assert_eq!(add_bytes(x,y),std::array::from_fn(|i|x[i].wrapping_add(y[i])));
        assert_eq!(signed_product_shift5(x,y),std::array::from_fn(|i|(x[i] as i8 as i32*y[i] as i8 as i32).div_euclid(32)));
        for c in [0,1,127,128,254,255] {
            assert_eq!(clamp_add_sub(x,y,[c;4]),std::array::from_fn(|i|(x[i] as i32+y[i] as i32-c as i32).clamp(0,255) as u8));
        }
        let xx=std::array::from_fn(|i|x[i%4]);let yy=std::array::from_fn(|i|y[(i/4+i)%4]);
        assert_eq!(add_bytes16(xx,yy),std::array::from_fn(|i|xx[i].wrapping_add(yy[i])));
    }}
    for x in [i32::MIN,-65536,-513,-512,-257,-256,-255,-1,0,255,256,65535,i32::MAX] {
        let a=[x,x.saturating_add(1),x.saturating_sub(1),x/2];
        assert_eq!(wrap_bytes(a),a.map(|v|v.rem_euclid(256) as u8));
    }
}
