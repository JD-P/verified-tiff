// Exact fixed-point YCbCr conversion with nearest chroma expansion.
use super::*;
verus! {
pub(super) open spec fn numerator(y:u8,cb:u8,cr:u8,c:int,limited:bool) -> int {
    let base=if limited {(y as int-16)*28560000000} else {y as int*1000000};
    let b=cb as int-128;let r=cr as int-128;
    if limited {
        if c==0 {base+r*38993826000} else if c==1 {base-b*9571454568-r*19862264568} else {base+b*49284636000}
    } else {
        if c==0 {base+r*1402000} else if c==1 {base-b*344136-r*714136} else {base+b*1772000}
    }
}
pub(super) open spec fn color(y:u8,cb:u8,cr:u8,c:int,limited:bool) -> u8 {
    let d=if limited {24528000000int} else {1000000int};let n=numerator(y,cb,cr,c,limited)+d/2;
    if n<=0 {0u8} else if n>=255*d {255u8} else {(n/d) as u8}
}
pub(super) fn convert(y:u8,cb:u8,cr:u8,c:usize,limited:bool) -> (r:u8)
    requires c<3,ensures r==color(y,cb,cr,c as int,limited),
{
    let base=if limited {(y as i64-16)*28560000000} else {y as i64*1000000};
    let b=cb as i64-128;let r=cr as i64-128;
    let raw=if limited {
        if c==0 {base+r*38993826000} else if c==1 {base-b*9571454568-r*19862264568} else {base+b*49284636000}
    } else {
        if c==0 {base+r*1402000} else if c==1 {base-b*344136-r*714136} else {base+b*1772000}
    };
    let d=if limited {24528000000i64} else {1000000i64};let n=raw+d/2;
    if n<=0 {0} else if n>=255*d {255} else {(n/d) as u8}
}
}
