//! Integer presentation and safe indexing from IEEE-754 binary32 bit patterns.
//! Floating-point matrix/gamma evaluation remains outside this module.
use super::*;
verus! {
pub open spec fn f32_value(bits:u32)->u64 {
    let e=(bits>>23)&255;let m=((bits&0x7fffff)|0x800000) as u64;
    if bits>>31!=0 || e<127 || e==255 && bits&0x7fffff!=0 {0}
    else if e>=159 {4294967295}
    else if e>=150 {m<<((e-150) as u64)} else {m>>((150-e) as u64)}
}
/// Saturating nonnegative truncation, including NaN/Inf/negative values.
/// The cap is applied before any host indexing or C integer conversion.
pub fn f32_index(bits:u32,cap:u32)->(r:u32)
    ensures r<=cap,r as u64==if f32_value(bits)>cap {cap as u64}else{f32_value(bits)},
{
    let e=(bits>>23)&255;let m=((bits&0x7fffff)|0x800000) as u64;
    let v=if bits>>31!=0||e<127||e==255&&bits&0x7fffff!=0 {0u64}
        else if e>=159{4294967295u64}
        else if e>=150{m<<((e-150) as u64)}else{m>>((150-e) as u64)};
    if v>cap as u64{cap}else{v as u32}
}
pub fn finite(bits:u32)->(r:bool)
    ensures r==(((bits>>23)&255)!=255),
{((bits>>23)&255)!=255}
pub fn cmyk(c:u16,m:u16,y:u16,k:u16,wide:bool)->(r:[u8;4])
    requires wide || c<=255&&m<=255&&y<=255&&k<=255,
    ensures r[0] as int==(255-scale(k,wide))*(255-scale(c,wide))/255,
        r[1] as int==(255-scale(k,wide))*(255-scale(m,wide))/255,
        r[2] as int==(255-scale(k,wide))*(255-scale(y,wide))/255,r[3]==255,
{
    let kk=255-normalize(k,wide) as u32;
    let cc=255-normalize(c,wide) as u32;let mm=255-normalize(m,wide) as u32;let yy=255-normalize(y,wide) as u32;
    proof{assert(kk*cc<=65025 && kk*mm<=65025 && kk*yy<=65025) by(nonlinear_arith) requires kk<=255,cc<=255,mm<=255,yy<=255;}
    [(kk*cc/255) as u8,(kk*mm/255) as u8,(kk*yy/255) as u8,255]
}
pub open spec fn scale(v:u16,wide:bool)->int {if wide {(v as int+128)/257}else{v as int}}
fn normalize(v:u16,wide:bool)->(r:u8)
    requires wide||v<=255, ensures r as int==scale(v,wide),
{if wide{((v as u32+128)/257) as u8}else{v as u8}}
pub fn clamp_byte(v:i64)->(r:u8)
    ensures r as int==if v<0{0}else if v>255{255}else{v as int},
{if v<0{0}else if v>255{255}else{v as u8}}
/// Wide intermediates prevent overflow even if caller-owned YCbCr tables have
/// been changed after initialization. Pointer validity is still the C contract.
pub fn ycbcr_tables(y:i32,rr:i32,bb:i32,cg:i32,rg:i32)->(r:[u8;3])
    ensures r[0] as int==if y as int+rr<0{0}else if y as int+rr>255{255}else{y as int+rr},
        r[2] as int==if y as int+bb<0{0}else if y as int+bb>255{255}else{y as int+bb},
{
    let sum=cg as i64+rg as i64;
    assert(-65536<=sum>>16<=65535) by(bit_vector) requires -4294967296<=sum<=4294967294;
    [clamp_byte(y as i64+rr as i64),clamp_byte(y as i64+(sum>>16)),clamp_byte(y as i64+bb as i64)]
}
}
