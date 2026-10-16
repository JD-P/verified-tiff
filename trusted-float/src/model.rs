//! Pure bit specifications moved unchanged from lerc_numeric.rs.
use vstd::prelude::*;
verus! {
pub open spec fn sign(v:u64) -> bool {v>=0x8000000000000000u64}
pub open spec fn finite(v:u64) -> bool {(v/0x10000000000000u64)%2048!=2047}
pub open spec fn mantissa(v:u64) -> u128 {
    ((v%0x10000000000000u64) as u128+if (v/0x10000000000000u64)%2048==0{0u128}else{0x10000000000000u128}) as u128
}
pub open spec fn exponent(v:u64) -> i32 {
    let e=(v/0x10000000000000u64)%2048;
    if e==0{-1074i32}else{(e as int-1075) as i32}
}
pub open spec fn rounded(v:u128,n:u32) -> u128 {
    if n==0{v}else if n>128{0}else if n==128{if v>0x80000000000000000000000000000000u128{1u128}else{0u128}}
    else{
        let q=v>>n;let half=1u128<<((n-1) as u32);let rem=v&(((1u128<<n)-1) as u128);
        (q+if rem>half || (rem==half && q%2==1){1u128}else{0u128}) as u128
    }
}
pub open spec fn sticky(v:u128,n:u32) -> u128 {
    if n==0{v}else if n>=128{if v==0{0u128}else{1u128}}
    else{(v>>n)|if v&(((1u128<<n)-1) as u128)==0{0u128}else{1u128}}
}
pub open spec fn top(v:u128,fuel:nat) -> int decreases fuel {
    if fuel==0 || v<2{0}else{1+top(v/2,(fuel-1) as nat)}
}
pub open spec fn packed(negative:bool,v:u128,e:i32) -> u64 {
    let hi=top(v,128) as int;
    let shift=if hi>52{hi-52}else{0int};
    let small=if -1074-e>shift{-1074-e}else{shift};
    let q=rounded(v,small as u32);
    let power=e+small;
    let hi2=top(q,128) as int;
    let exp=power+hi2;
    let sg=if negative{0x8000000000000000u64}else{0u64};
    if q==0{sg}else if exp>1023{sg|0x7ff0000000000000u64}
    else if exp< -1022{sg|((q<<((power+1074) as u32)) as u64)}
    else{
        let bits=if hi2>52{q>>(hi2-52) as u32}else{q<<(52-hi2) as u32};
        sg|(((exp+1023) as u64)<<52)|(bits as u64%0x10000000000000u64)
    }
}
pub open spec fn scaled(a:u64,q:u32) -> u64 {
    packed(sign(a),(mantissa(a)*q) as u128,(exponent(a)+1) as i32)
}
pub open spec fn sum(a:u64,b:u64) -> u64 {
    let e=if exponent(a)>exponent(b){exponent(a)}else{exponent(b)};
    let x=sticky(mantissa(a)<<64,(e-exponent(a)) as u32);
    let y=sticky(mantissa(b)<<64,(e-exponent(b)) as u32);
    let m=if sign(a)==sign(b){x+y}else if x>=y{x-y}else{y-x};
    let sg=if m==0 && sign(a)!=sign(b){false}else if sign(a)==sign(b) || x>=y{sign(a)}else{sign(b)};
    packed(sg,m as u128,(e-64) as i32)
}
pub open spec fn less(a:u64,b:u64) -> bool {
    if a%0x8000000000000000u64==0 && b%0x8000000000000000u64==0{false}
    else if sign(a)!=sign(b){sign(a)}else if sign(a){a>b}else{a<b}
}
pub open spec fn type_size(kind:usize) -> usize {if kind<2{1}else if kind<4{2}else if kind<7{4}else{8}}
pub open spec fn value(v:u64,kind:usize) -> u64 {
    if kind==7{v}else if kind==6{
        let e=(v/8388608)%256;let m=v%8388608+if e==0{0int}else{8388608int};
        if e==255{((if v>=2147483648{0x8000000000000000u64}else{0u64})|0x7ff0000000000000u64|((v%8388608) as u64)<<29)}
        else{packed(v>=2147483648,m as u128,if e==0{-149i32}else{(e-150) as i32})}
    }else{
        let modulus=if kind<2{256u64}else if kind<4{65536u64}else{4294967296u64};
        let neg=kind%2==0 && v>=modulus/2;
        packed(neg,if neg{(modulus-v) as u128}else{v as u128},0)
    }
}
pub open spec fn narrowed(v:u64) -> u32 {
    let m=mantissa(v);let e=exponent(v);let hi=top(m,128);
    let shift=if hi>23{hi-23}else{0int};let n=if -149-e>shift{-149-e}else{shift};
    let q=rounded(m,n as u32);let hi2=top(q,128);let ex=e+n+hi2;
    let sg=if sign(v){0x80000000u32}else{0u32};
    if !finite(v){sg|0x7f800000u32|((v%0x10000000000000u64)>>29) as u32}
    else if q==0{sg}else if ex>127{sg|0x7f800000u32}else if ex< -126{sg|((q<<((e+n+149) as u32)) as u32)}
    else{sg|(((ex+127) as u32)<<23)|((if hi2>23{q>>(hi2-23) as u32}else{q<<(23-hi2) as u32}) as u32%8388608)}
}
pub open spec fn integer(v:u64,kind:usize) -> Option<u64> {
    let m=mantissa(v);let e=exponent(v);
    let q=if e>=0{m<<(e as u32)}else if e<= -128{0u128}else{m>>((-e) as u32)};
    let modulus=if kind<2{256u64}else if kind<4{65536u64}else{4294967296u64};
    let bound=if kind%2==0{modulus/2+if sign(v){1int}else{0int}}else{modulus as int};
    if !finite(v) || e>32 || q>=bound || (sign(v) && kind%2==1 && q!=0){None}
    else{Some(if sign(v) && q!=0{(modulus-q) as u64}else{q as u64})}
}
}
