//! LERC numeric dispatch and verified software reference/fallback.
//! Hardware primitives have explicit contracts in the trusted-float dependency.
use super::Error;
use vstd::prelude::*;
use verified_tiff_float as hw;
pub(super) use hw::model::*;
verus! {




#[inline(always)]
pub(super) fn parts(v:u64) -> (r:(bool,u128,i32))
    ensures r==(sign(v),mantissa(v),exponent(v)),r.1<0x20000000000000u128,-1074<=r.2<=972,
{
    let e=(v/0x10000000000000u64)%2048;
    (v>=0x8000000000000000u64,(v%0x10000000000000u64) as u128+if e==0{0u128}else{0x10000000000000u128},if e==0{-1074i32}else{e as i32-1075})
}

#[inline(always)]
pub(super) fn round(v:u128,n:u32) -> (r:u128)
    ensures r==rounded(v,n),n>0 ==> r<=0x80000000000000000000000000000000u128,
{
    if n==0{return v;}
    if n>128{return 0;}
    if n==128{return if v>0x80000000000000000000000000000000u128{1}else{0};}
    if n>=64 {
        let high=(v>>64u32) as u64;let low=v as u64;
        let shift=n-64;
        let q=high>>shift;
        let up=if shift==0 {low>0x8000000000000000u64 || (low==0x8000000000000000u64 && q%2==1)}else{
            assert((1u64<<shift)>0) by(bit_vector) requires 0<shift<64;
            let half=1u64<<(shift-1);let rem=high&((1u64<<shift)-1);
            rem>half || (rem==half && (low!=0 || q%2==1))
        };
        let r=q as u128+if up{1u128}else{0u128};
        assert(r==rounded(v,n)) by(bit_vector)
            requires 64<=n<128,high==(v>>64u32) as u64,low==v as u64,shift==n-64,q==high>>shift,
                up==if shift==0 {low>0x8000000000000000u64 || (low==0x8000000000000000u64 && q%2==1)}else{
                    (high&(((1u64<<shift)-1) as u64))>(1u64<<((shift-1) as u32)) || ((high&(((1u64<<shift)-1) as u64))==(1u64<<((shift-1) as u32)) && (low!=0 || q%2==1))},
                r==q as u128+if up{1u128}else{0u128};
        return r;
    }
    let q=v>>n;let half=1u128<<(n-1);
    assert((1u128<<n)>0 && q<0x80000000000000000000000000000000u128) by(bit_vector)
        requires 0<n<128,q==v>>n;
    let rem=v&(((1u128<<n)-1) as u128);
    q+if rem>half || (rem==half && q%2==1){1}else{0}
}

#[inline(always)]
fn shift_sticky(v:u128,n:u32) -> (r:u128)
    ensures r==sticky(v,n),r<=v,
{
    if n==0{return v;}
    if n>=128{return if v==0{0}else{1};}
    assert((1u128<<n)>0) by(bit_vector) requires 0<n<128;
    let r=(v>>n)|if v&(((1u128<<n)-1) as u128)==0{0u128}else{1u128};
    assert(r<=v) by(bit_vector) requires 0<n<128,r==(v>>n)|if v&(((1u128<<n)-1) as u128)==0{0u128}else{1u128};
    r
}

// Erased induction connects a chunked shift to the unchanged bit-by-bit model.
proof fn top_shift(v:u128,n:u32,fuel:nat)
    requires n<128,n<fuel<=128,v>>n>0,
    ensures top(v,fuel)==n+top(v>>n,(fuel-n) as nat),
    decreases n,
{
    if n>0 {
        assert(v>=2 && ((v/2)>>((n-1) as u32))==v>>n) by(bit_vector)
            requires 0<n<128,v>>n>0;
        top_shift(v/2,(n-1) as u32,(fuel-1) as nat);
        reveal_with_fuel(top,1);
    } else {assert(v>>0u32==v) by(bit_vector);}
}
#[inline(always)]
fn highest64(v:u64) -> (r:u32)
    requires v>0,
    ensures r<64,(v as u128)>>r==1u128,r==top(v as u128,128),
{
    let zeros=v.leading_zeros();
    proof {vstd::std_specs::bits::axiom_u64_leading_zeros(v);}
    let n=63-zeros;
    assert((v as u128)>>n==1u128) by(bit_vector)
        requires zeros<64,n==63-zeros,
            (v>>((63-zeros) as u64))&1u64!=0u64,
            v>>((64-zeros) as u64)==0;
    proof {top_shift(v as u128,n,128);reveal_with_fuel(top,1);}
    n
}
#[inline(always)]
fn highest(v:u128) -> (r:u32)
    ensures r==top(v,128),r<=127,v==0 ==> r==0,v>0 ==> v>>r==1u128,
{
    if v==0 {reveal_with_fuel(top,1);return 0;}
    let high=(v>>64u32) as u64;
    let n=if high>0 {
        let low=highest64(high);
        let n=64+low;
        assert(v>>n==1u128) by(bit_vector)
            requires low<64,(high as u128)>>low==1u128,high==(v>>64u32) as u64,n==64+low;
        n
    } else {
        let low=v as u64;
        assert(low>0) by(bit_vector) requires v>0,((v>>64u32) as u64)==0,low==v as u64;
        let n=highest64(low);
        assert(v>>n==1u128) by(bit_vector)
            requires n<64,(low as u128)>>n==1u128,((v>>64u32) as u64)==0,low==v as u64;
        n
    };
    proof {
        top_shift(v,n,128);
        assert(top(v>>n,(128-n) as nat)==0) by {reveal_with_fuel(top,1);}
    }
    n
}

// The normal rounder knows its leading-bit position before rounding. Rounding
// can only retain that position or carry once; a second leading-zero count is
// unnecessary. The carry is absorbed by adding the significand to the field.
#[inline(always)]
fn round_significand(v:u128,n:u32) -> (r:u64)
    requires n<128,0x10000000000000u128<=v>>n<0x20000000000000u128,
    ensures r as u128==rounded(v,n),0x10000000000000u64<=r<=0x20000000000000u64,
{
    let q=(v>>n) as u64;
    if n==0{assert(v>>0u32==v) by(bit_vector);return q;}
    if n<64 {
        assert((1u64<<n)>0) by(bit_vector) requires 0<n<64;
        let half=1u64<<(n-1);let rem=(v as u64)&((1u64<<n)-1);
        let r=q+if rem>half || (rem==half && q%2==1){1u64}else{0u64};
        assert(r as u128==rounded(v,n)) by(bit_vector)
            requires 0<n<64,0x10000000000000u128<=v>>n<0x20000000000000u128,
                q==(v>>n) as u64,half==1u64<<((n-1) as u32),rem==(v as u64)&(((1u64<<n)-1) as u64),
                r==q+if rem>half || (rem==half && q%2==1){1u64}else{0u64};
        return r;
    }
    let wide=round(v,n);
    assert(0x10000000000000u128<=wide<=0x20000000000000u128) by(bit_vector)
        requires 64<=n<128,0x10000000000000u128<=v>>n<0x20000000000000u128,wide==rounded(v,n);
    wide as u64
}
#[inline(always)]
fn pack_normal(negative:bool,v:u128,e:i32,hi:u32) -> (r:u64)
    requires 52<=hi<128,v>>hi==1u128,-4096<=e<=4096,
        -1022<=e+hi<=1023,top(v,128)==hi,
    ensures r==packed(negative,v,e),
{
    assert(0x10000000000000u128<=v>>((hi-52) as u32)<0x20000000000000u128) by(bit_vector)
        requires 52<=hi<128,v>>hi==1u128;
    let q=round_significand(v,hi-52);
    let exp=e+hi as i32;
    let sg=if negative{0x8000000000000000u64}else{0u64};
    proof {
        assert(0x10000000000000u64<=q<=0x20000000000000u64 &&
            q as u128==rounded(v,(hi-52) as u32)) by(bit_vector)
            requires 52<=hi<128,v>>hi==1u128,q==rounded(v,(hi-52) as u32) as u64;
        let h=if q==0x20000000000000u64{53u32}else{52u32};
        assert((q as u128)>>h==1u128) by(bit_vector)
            requires 0x10000000000000u64<=q<=0x20000000000000u64,
                h==if q==0x20000000000000u64{53u32}else{52u32};
        top_shift(q as u128,h,128);reveal_with_fuel(top,1);
    }
    assert((((exp+1022) as u64)<<52)<=0x7fd0000000000000u64) by(bit_vector)
        requires -1022<=exp<=1023;
    let out=sg|(((exp+1022) as u64)<<52)+q;
    proof {
        let h=if q==0x20000000000000u64{53u32}else{52u32};
        let ex=(exp+h as i32-52) as i32;
        let bits=if h==53{q>>1u32}else{q};
        assert(bits as u128==if h>52{(q as u128)>>((h-52) as u32)}else{(q as u128)<<((52-h) as u32)}) by(bit_vector)
            requires h==if q==0x20000000000000u64{53u32}else{52u32},bits==if h==53{q>>1u32}else{q};
        let expected=if ex>1023{sg|0x7ff0000000000000u64}else{sg|(((ex+1023) as u64)<<52)|(bits%0x10000000000000u64)};
        assert(packed(negative,v,e)==expected);
        assert(out==expected) by(bit_vector)
            requires -1022<=exp<=1023,0x10000000000000u64<=q<=0x20000000000000u64,
                h==if q==0x20000000000000u64{53u32}else{52u32},ex==(exp+h as i32-52) as i32,
                bits==if h==53{q>>1u32}else{q},
                out==sg|(((((exp+1022) as u64)<<52)+q) as u64),
                expected==if ex>1023{sg|0x7ff0000000000000u64}else{sg|(((ex+1023) as u64)<<52)|(bits%0x10000000000000u64)};
    }
    out
}
#[inline(always)]
pub(super) fn pack(negative:bool,v:u128,e:i32) -> (r:u64)
    requires -4096<=e<=4096,
    ensures r==packed(negative,v,e),
{
    let hi=highest(v);
    if hi>=52 && -1022<=e+hi as i32 && e+hi as i32<=1023 {
        return pack_normal(negative,v,e,hi);
    }
    let shift=if hi>52{hi-52}else{0u32};
    let small=if -1074-e>shift as i32{(-1074-e) as u32}else{shift};
    let q=round(v,small);let power=e+small as i32;
    let hi2=highest(q);let exp=power+hi2 as i32;
    let sg=if negative{0x8000000000000000u64}else{0u64};
    if q==0{return sg;}
    if exp>1023{return sg|0x7ff0000000000000u64;}
    if exp< -1022{
        return sg|((q<<((power+1074) as u32)) as u64);
    }
    let bits=if hi2>52{q>>(hi2-52)}else{q<<(52-hi2)};
    sg|(((exp+1023) as u64)<<52)|(bits as u64%0x10000000000000u64)
}

/// Rounded q * (2 * maximum error). Header validation excludes overflow of
/// the intermediate doubled error, matching Lerc2's binary64 evaluation.
#[inline(never)]
pub(super) fn scale_software(a:u64,q:u32) -> (r:u64)
    ensures r==scaled(a,q),
{
    let (sg,m,e)=parts(a);
    assert(m as int*q<=0x1fffffffffffffu128 as int*4294967295) by(nonlinear_arith)
        requires m<0x20000000000000u128,q<=4294967295;
    pack(sg,m*(q as u128),e+1)
}

// As in word-based IEEE implementations, keep eight low alignment bits and
// summarize the discarded suffix by its nonzeroness. The proofs below relate
// this directly to the original 128-bit LERC model, including subtraction.
#[inline(always)]
fn jam64(v:u64,n:u32) -> (r:u64)
    ensures r as u128==sticky(v as u128,n),r<=v,
{
    if n==0{return v;}
    if n>=64 {
        let r=if v==0{0u64}else{1u64};
        assert(r as u128==sticky(v as u128,n)) by(bit_vector)
            requires n>=64,r==if v==0{0u64}else{1u64};
        return r;
    }
    assert((1u64<<n)>0) by(bit_vector) requires 0<n<64;
    let r=(v>>n)|if v&((1u64<<n)-1)==0{0u64}else{1u64};
    assert(r as u128==sticky(v as u128,n) && r<=v) by(bit_vector)
        requires 0<n<64,r==(v>>n)|if v&(((1u64<<n)-1) as u64)==0{0u64}else{1u64};
    r
}
proof fn align_word(m:u128,d:u32)
    requires m<0x20000000000000u128,
    ensures sticky(((m as u64)<<8u32) as u128,d)==sticky(sticky(m<<64u32,d),56u32),
{
    assert(sticky(((m as u64)<<8u32) as u128,d)==sticky(sticky(m<<64u32,d),56u32)) by(bit_vector)
        requires m<0x20000000000000u128;
}
proof fn combine_words(a:u128,b:u128,same:bool)
    requires a<0x200000000000000000000000000000u128,
        b<0x200000000000000000000000000000u128,
        a&0xffffffffffffffffu128==0,
    ensures
        (a>>56u32)>=sticky(b,56u32) <==> a>=b,
        (a>>56u32)==sticky(b,56u32) <==> a==b,
        sticky((if same{a+b}else if a>=b{a-b}else{b-a}) as u128,56u32)==
            if same{(a>>56u32)+sticky(b,56u32)}
            else if a>=b{(a>>56u32)-sticky(b,56u32)}else{sticky(b,56u32)-(a>>56u32)},
{
    assert((a>>56u32)<0x2000000000000000u128 && sticky(b,56u32)<0x2000000000000000u128) by(bit_vector)
        requires a<0x200000000000000000000000000000u128,b<0x200000000000000000000000000000u128;
    assert((a>>56u32)>=sticky(b,56u32) <==> a>=b) by(bit_vector)
        requires a&0xffffffffffffffffu128==0;
    assert((a>>56u32)==sticky(b,56u32) <==> a==b) by(bit_vector)
        requires a&0xffffffffffffffffu128==0;
    assert(sticky((if same{a+b}else if a>=b{a-b}else{b-a}) as u128,56u32)==
            if same{((a>>56u32)+sticky(b,56u32)) as u128}
            else if a>=b{((a>>56u32)-sticky(b,56u32)) as u128}else{(sticky(b,56u32)-(a>>56u32)) as u128}) by(bit_vector)
        requires a<0x200000000000000000000000000000u128,
            b<0x200000000000000000000000000000u128,a&0xffffffffffffffffu128==0;
}
// Rounding after jamming: the jam bit must stay strictly below the guard bit.
// Here 56 discarded bits and at least 54 leading bits in the retained word
// give the separation required even after cancellation (FloatLib theorem 4).
proof fn packed_jam(negative:bool,v:u128,w:u64,e:i32,n:u32)
    requires w as u128==sticky(v,56u32),54<=n<64,(w as u128)>>n==1u128,
        -1074<=e<=972,-1022<=e+n-8,
    ensures packed(negative,v,(e-64) as i32)==packed(negative,w as u128,(e-8) as i32),
{
    assert(v>>((n+56) as u32)==1u128) by(bit_vector)
        requires w as u128==sticky(v,56u32),54<=n<64,(w as u128)>>n==1u128;
    top_shift(v,(n+56) as u32,128);
    top_shift(w as u128,n,128);
    reveal_with_fuel(top,1);
    assert(rounded(v,(n+4) as u32)==rounded(w as u128,(n-52) as u32)) by(bit_vector)
        requires w as u128==sticky(v,56u32),54<=n<64;
}
#[inline(never)]
pub(super) fn plus_software(a:u64,b:u64) -> (r:u64)
    ensures r==sum(a,b),
{
    let (sa,ma,ea)=parts(a);let (sb,mb,eb)=parts(b);
    let e=if ea>eb{ea}else{eb};
    let da=(e-ea) as u32;let db=(e-eb) as u32;
    let ax=(ma as u64)<<8u32;let bx=(mb as u64)<<8u32;
    assert(ax<0x2000000000000000u64 && bx<0x2000000000000000u64) by(bit_vector)
        requires ma<0x20000000000000u128,mb<0x20000000000000u128,
            ax==(ma as u64)<<8u32,bx==(mb as u64)<<8u32;
    let x=jam64(ax,da);let y=jam64(bx,db);
    let m=if sa==sb{x+y}else if x>=y{x-y}else{y-x};
    if m>=0x40000000000000u64 {
        let n=highest64(m);
        assert(n>=54) by(bit_vector) requires m>=0x40000000000000u64,n<64,(m as u128)>>n==1u128;
        if -1022<=e-8+n as i32 && e-8+n as i32<=1023 {
            let sg=if sa==sb || x>=y{sa}else{sb};
            proof {
                let wx=sticky(ma<<64u32,da);let wy=sticky(mb<<64u32,db);
                let wm=(if sa==sb{wx+wy}else if wx>=wy{wx-wy}else{wy-wx}) as u128;
                align_word(ma,da);align_word(mb,db);
                assert(wx<0x200000000000000000000000000000u128 && wy<0x200000000000000000000000000000u128) by(bit_vector)
                    requires ma<0x20000000000000u128,mb<0x20000000000000u128,
                        wx==sticky(ma<<64u32,da),wy==sticky(mb<<64u32,db);
                if ea>=eb {
                    assert(wx==ma<<64u32 && (wx>>56u32)==ax as u128 && wx&0xffffffffffffffffu128==0) by(bit_vector)
                        requires da==0,wx==sticky(ma<<64u32,da),ma<0x20000000000000u128,ax==(ma as u64)<<8u32;
                    combine_words(wx,wy,sa==sb);
                } else {
                    assert(wy==mb<<64u32 && (wy>>56u32)==bx as u128 && wy&0xffffffffffffffffu128==0) by(bit_vector)
                        requires db==0,wy==sticky(mb<<64u32,db),mb<0x20000000000000u128,bx==(mb as u64)<<8u32;
                    combine_words(wy,wx,sa==sb);
                }
                assert(m as u128==sticky(wm,56u32));
                assert(wm>0) by(bit_vector) requires m>0,m as u128==sticky(wm,56u32);
                assert(sum(a,b)==packed(sg,wm,(e-64) as i32));
                packed_jam(sg,wm,m,e,n);
            }
            return pack_normal(sg,m as u128,e-8,n);
        }
    }
    let aa=ma<<64;let bb=mb<<64;
    assert(aa<0x200000000000000000000000000000u128 && bb<0x200000000000000000000000000000u128) by(bit_vector)
        requires ma<0x20000000000000u128,mb<0x20000000000000u128,aa==ma<<64u32,bb==mb<<64u32;
    let x=shift_sticky(aa,(e-ea) as u32);let y=shift_sticky(bb,(e-eb) as u32);
    let m=if sa==sb{x+y}else if x>=y{x-y}else{y-x};
    let sg=if m==0 && sa!=sb{false}else if sa==sb || x>=y{sa}else{sb};
    pack(sg,m,e-64)
}

#[inline(always)]
pub(super) fn minimum(a:u64,b:u64) -> (r:u64)
    ensures r==if less(a,b){a}else{b},
{
    let cmp=if a%0x8000000000000000u64==0 && b%0x8000000000000000u64==0{false}
        else if (a>=0x8000000000000000u64)!=(b>=0x8000000000000000u64){a>=0x8000000000000000u64}
        else if a>=0x8000000000000000u64{a>b}else{a<b};
    if cmp{a}else{b}
}

pub(super) fn size(kind:usize) -> (r:usize) ensures r==type_size(kind),r==1 || r==2 || r==4 || r==8,
{if kind<2{1}else if kind<4{2}else if kind<7{4}else{8}}

#[inline(never)]
pub(super) fn number_software(v:u64,kind:usize) -> (r:u64)
    requires kind<8,kind<7 ==> v<if kind<2{256int}else if kind<4{65536int}else{4294967296int},
    ensures r==value(v,kind),
{
    if kind==7{return v;}
    if kind==6{
        let e=(v/8388608)%256;let m=v%8388608+if e==0{0u64}else{8388608u64};
        if e==255{return (if v>=2147483648{0x8000000000000000u64}else{0u64})|0x7ff0000000000000u64|(v%8388608)<<29;}
        return pack(v>=2147483648,m as u128,if e==0{-149i32}else{e as i32-150});
    }
    let modulus=if kind<2{256u64}else if kind<4{65536u64}else{4294967296u64};
    let neg=kind%2==0 && v>=modulus/2;
    pack(neg,if neg{(modulus-v) as u128}else{v as u128},0)
}

#[inline(never)]
pub(super) fn narrow_software(v:u64) -> (r:u32) ensures r==narrowed(v),
{
    let exp=((v>>52u32)&2047) as u32;
    if 897<=exp && exp<=1150{
        let m=(v&0xfffffffffffffu64)|0x10000000000000u64;
        let base=(m>>29u32) as u32;let rem=m&0x1fffffffu64;
        assert(8388608<=base<16777216) by(bit_vector)
            requires m==(v&0xfffffffffffffu64)|0x10000000000000u64,base==(m>>29u32) as u32;
        let q=base+if rem>0x10000000u64 || (rem==0x10000000u64 && base%2==1){1u32}else{0u32};
        let sg=if v>=0x8000000000000000u64{0x80000000u32}else{0u32};
        proof{
            assert(exp==(v/0x10000000000000u64)%2048 && mantissa(v)==m as u128) by(bit_vector)
                requires exp==((v>>52u32)&2047) as u32,897<=exp<=1150,m==(v&0xfffffffffffffu64)|0x10000000000000u64;
            assert((m as u128)>>52u32==1u128) by(bit_vector) requires m==(v&0xfffffffffffffu64)|0x10000000000000u64;
            top_shift(m as u128,52,128);reveal_with_fuel(top,1);
            assert(q as u128==rounded(m as u128,29u32) && 8388608<=q<=16777216) by(bit_vector)
                requires m==(v&0xfffffffffffffu64)|0x10000000000000u64,base==(m>>29u32) as u32,rem==m&0x1fffffffu64,
                    q==base+if rem>0x10000000u64 || (rem==0x10000000u64 && base%2==1){1u32}else{0u32};
            let hi=if q==16777216{24u32}else{23u32};
            assert((q as u128)>>hi==1u128) by(bit_vector) requires 8388608<=q<=16777216,hi==if q==16777216{24u32}else{23u32};
            top_shift(q as u128,hi,128);reveal_with_fuel(top,1);
        }
        let out=sg|((exp-897)*8388608+q);
        let hi=if q==16777216{24u32}else{23u32};
        let ex=exp as i32-1046+hi as i32;
        let bits=if hi>23{q>>1u32}else{q};
        let expected=if ex>127{sg|0x7f800000u32}else{sg|(((ex+127) as u32)<<23u32)|(bits%8388608)};
        assert(top(m as u128,128)==52);
        assert(top(q as u128,128)==hi);
        assert(exponent(v)==exp as int-1075);
        assert(bits as u128==if hi>23{(q as u128)>>((hi-23) as u32)}else{(q as u128)<<((23-hi) as u32)}) by(bit_vector)
            requires hi==if q==16777216{24u32}else{23u32},bits==if hi>23{q>>1u32}else{q};
        assert(narrowed(v)==expected);
        assert(out==expected) by(bit_vector)
            requires 897<=exp<=1150,8388608<=q<=16777216,
                hi==if q==16777216{24u32}else{23u32},ex==exp as i32-1046+hi as i32,
                bits==if hi>23{q>>1u32}else{q},
                out==sg|(((exp-897)*8388608+q) as u32),
                expected==if ex>127{sg|0x7f800000u32}else{sg|(((ex+127) as u32)<<23u32)|(bits%8388608)};
        return out;
    }
    let (sgn,m,e)=parts(v);let sg=if sgn{0x80000000u32}else{0u32};
    if (v/0x10000000000000u64)%2048==2047{return sg|0x7f800000u32|((v%0x10000000000000u64)>>29) as u32;}
    let hi=highest(m);let shift=if hi>23{hi-23}else{0u32};let n=if -149-e>shift as i32{(-149-e) as u32}else{shift};
    let q=round(m,n);let hi2=highest(q);let ex=e+n as i32+hi2 as i32;
    if q==0{sg}else if ex>127{sg|0x7f800000u32}else if ex< -126{sg|((q<<((e+n as i32+149) as u32)) as u32)}
    else{sg|(((ex+127) as u32)<<23)|((if hi2>23{q>>(hi2-23)}else{q<<(23-hi2)}) as u32%8388608)}
}

pub(super) fn to_integer(v:u64,kind:usize) -> (r:Result<u64,Error>)
    requires kind<6,
    ensures r.is_ok()==integer(v,kind).is_some(),r.is_ok() ==> r.unwrap()==integer(v,kind).unwrap(),
{
    let (sg,m,e)=parts(v);
    if (v/0x10000000000000u64)%2048==2047 || e>32{return Err(Error::Invalid);}
    let q=if e>=0{m<<(e as u32)}else if e<= -128{0u128}else{m>>((-e) as u32)};
    let modulus=if kind<2{256u64}else if kind<4{65536u64}else{4294967296u64};
    let bound=if kind%2==0{modulus/2+if sg{1u64}else{0u64}}else{modulus};
    if q>=bound as u128 || (sg && kind%2==1 && q!=0){return Err(Error::Invalid);}
    Ok(if sg && q!=0{modulus-q as u64}else{q as u64})
}

/// Finite operands use the CPU; unusual non-finite bit-model cases retain
/// their historical software behavior, including NaN payload conventions.
#[inline(always)]
pub(super) fn scale(a:u64,q:u32) -> (r:u64)
    ensures r==scaled(a,q),
{
    if (a/0x10000000000000u64)%2048!=2047 {hw::mul_twice_u32(a,q)}
    else {scale_software(a,q)}
}
#[inline(always)]
pub(super) fn plus(a:u64,b:u64) -> (r:u64)
    ensures r==sum(a,b),
{
    if (a/0x10000000000000u64)%2048!=2047 && (b/0x10000000000000u64)%2048!=2047 {hw::add(a,b)}
    else {plus_software(a,b)}
}
#[inline(always)]
pub(super) fn narrow(v:u64) -> (r:u32)
    ensures r==narrowed(v),
{
    if (v/0x10000000000000u64)%2048!=2047 {hw::narrow(v)}
    else {narrow_software(v)}
}
#[inline(always)]
pub(super) fn number(v:u64,kind:usize) -> (r:u64)
    requires kind<8,kind<7 ==> v<if kind<2{256int}else if kind<4{65536int}else{4294967296int},
    ensures r==value(v,kind),
{
    if kind==7{return v;}
    if kind==6 {
        if (v/8388608)%256!=255 {return hw::widen(v as u32);}
        return number_software(v,kind);
    }
    let modulus=if kind<2{256u64}else if kind<4{65536u64}else{4294967296u64};
    let neg=kind%2==0 && v>=modulus/2;
    let mag=if neg{modulus-v}else{v};
    hw::from_magnitude(mag as u32,neg)
}
}
