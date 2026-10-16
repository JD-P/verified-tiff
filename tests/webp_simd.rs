use vstd::prelude::*;
mod webp_pixels {pub type Pixel=[u8;4];}
#[path="reference/webp_predict_scalar.rs"] mod webp_predict;
#[path="../src/webp_simd.rs"] mod kernels;
#[test]
fn lossless_lanes_match_previous_scalar() {
    let mut seed=0x274b1e998324c25du64;
    let mut pixel=||{std::array::from_fn(|_|{seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;seed as u8})};
    for i in 0..200000 {
        let p=[pixel(),pixel(),pixel(),pixel()];
        let m=if i<256{[i as u8;4]}else{pixel()};
        let colors=kernels::color4(p,m);let green=kernels::green4(p);
        for j in 0..4 {
            assert_eq!(colors[j],webp_predict::inverse_color(p[j],m));
            assert_eq!(green[j],webp_predict::add_green(p[j]));
        }
        assert_eq!(kernels::predict12(p[0],p[1],p[2],p[3]),
            webp_predict::add_pixel(p[0],webp_predict::predict(12,p[1],p[2],p[3],[0;4])));
    }
}
