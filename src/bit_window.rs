// Constant-size safe bit extraction, refined against the original bit models.
use super::*;
use crate::deflate_bits as db;
verus! {
#[verifier::inline]
pub(super) open spec fn word(a:u8,b:u8,c:u8,msb:bool) -> u32 {
    if msb {(a as u32)<<16 | (b as u32)<<8 | c as u32}
    else {a as u32 | (b as u32)<<8 | (c as u32)<<16}
}
#[verifier::inline]
pub(super) open spec fn extract(w:u32,skip:u32,n:u32,msb:bool) -> u32 {
    (w >> (if msb {(24-skip-n) as u32} else {skip})) & (((1u32<<n)-1) as u32)
}
pub(super) proof fn powers(n:u32)
    requires n<=16,
    ensures (1u32<<n) as int==db::pow(n as int),n<=12 ==> db::pow(n as int)==pow2_code(n as int),
{
    assert((1u32<<n)==if n==0 {1u32} else if n==1 {2u32} else if n==2 {4u32}
        else if n==3 {8u32} else if n==4 {16u32} else if n==5 {32u32}
        else if n==6 {64u32} else if n==7 {128u32} else if n==8 {256u32}
        else if n==9 {512u32} else if n==10 {1024u32} else if n==11 {2048u32}
        else if n==12 {4096u32} else if n==13 {8192u32} else if n==14 {16384u32}
        else if n==15 {32768u32} else {65536u32}) by(bit_vector) requires n<=16;
}
proof fn byte_bit(a:u8,b:u8,c:u8,j:u32,msb:bool)
    requires j<24,
    ensures ((word(a,b,c,msb) >> (if msb {(23-j) as u32} else {j})) & 1) as int
        == ((if j<8 {a as int} else if j<16 {b as int} else {c as int})
            /power2(if msb {7-(j as int)%8} else {(j as int)%8}))%2,
{
    let bit=if msb {(7-j%8) as u32} else {j%8};
    powers(bit);
    assert(((word(a,b,c,msb) >> (if msb {(23-j) as u32} else {j})) & 1)
        == ((if j<8 {a as u32} else if j<16 {b as u32} else {c as u32})/(1u32<<bit))%2) by(bit_vector)
        requires j<24,bit==if msb {(7-j%8) as u32} else {j%8};
    assert(db::pow(bit as int)==power2(bit as int));
}
proof fn recurrence(w:u32,skip:u32,n:u32,msb:bool)
    requires skip<8,0<n<=16,
    ensures extract(w,skip,n,msb) as int ==
        if msb {2*(extract(w,skip,(n-1) as u32,msb) as int)+((w>>((24-skip-n) as u32))&1) as int}
        else {extract(w,skip,(n-1) as u32,msb) as int+db::pow(n as int-1)*(((w>>((skip+n-1) as u32))&1) as int)},
        (extract(w,skip,n,msb) as int)<db::pow(n as int),
{
    powers(n);powers((n-1) as u32);
    assert(extract(w,skip,n,msb)<(1u32<<n)) by(bit_vector) requires skip<8,0<n<=16;
    if msb {
        assert(extract(w,skip,n,true)==2*extract(w,skip,(n-1) as u32,true)+((w>>((24-skip-n) as u32))&1)) by(bit_vector)
            requires skip<8,0<n<=16;
        assert(extract(w,skip,(n-1) as u32,true)<65536 && ((w>>((24-skip-n) as u32))&1)<=1) by(bit_vector)
            requires skip<8,0<n<=16;
    } else {
        assert(extract(w,skip,n,false)==extract(w,skip,(n-1) as u32,false)+(1u32<<((n-1) as u32))*((w>>((skip+n-1) as u32))&1)) by(bit_vector)
            requires skip<8,0<n<=16;
        assert(extract(w,skip,(n-1) as u32,false)<65536 && ((w>>((skip+n-1) as u32))&1)<=1 && (1u32<<((n-1) as u32))<=32768) by(bit_vector)
            requires skip<8,0<n<=16;
    }
}
pub(super) proof fn model(s:Seq<u8>,at:int,skip:u32,n:u32,msb:bool)
    requires 0<=at,at+3<=s.len(),skip<8,n<=16,
    ensures extract(word(s[at],s[at+1],s[at+2],msb),skip,n,msb) as int
        ==if msb {lz_bits(s,at*8+skip,n as nat)} else {db::field(s,at*8+skip,n as nat)},
        n<=12 ==> lz_lsb_bits(s,at*8+skip,n as nat)==db::field(s,at*8+skip,n as nat),
    decreases n,
{
    reveal_with_fuel(lz_bits,1);reveal_with_fuel(lz_lsb_bits,1);reveal_with_fuel(db::field,1);
    let w=word(s[at],s[at+1],s[at+2],msb);
    if n==0 {
        assert(extract(w,skip,n,msb)==0) by(bit_vector) requires n==0,skip<8;
    } else {
        model(s,at,skip,(n-1) as u32,msb);recurrence(w,skip,n,msb);
        let j=(skip+n-1) as u32;
        byte_bit(s[at],s[at+1],s[at+2],j,msb);powers((n-1) as u32);
        quotient_remainder(j as int,8);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(at*8+j as int,8,at+j as int/8,j as int%8);
        assert(s[(at*8+j as int)/8]==if j<8 {s[at]} else if j<16 {s[at+1]} else {s[at+2]});
    }
}
pub(super) proof fn digit(s:Seq<u8>,at:int,skip:u32,n:u32,k:u32)
    requires 0<=at,at+3<=s.len(),skip<8,k<n<=16,
    ensures ((extract(word(s[at],s[at+1],s[at+2],false),skip,n,false)>>k)&1) as int
        ==db::field(s,at*8+skip+k,1),
{
    let w=word(s[at],s[at+1],s[at+2],false);let j=(skip+k) as u32;
    byte_bit(s[at],s[at+1],s[at+2],j,false);
    assert(((extract(w,skip,n,false)>>k)&1)==((w>>((skip+k) as u32))&1)) by(bit_vector)
        requires skip<8,k<n<=16;
    quotient_remainder(j as int,8);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(at*8+j as int,8,at+j as int/8,j as int%8);
    assert(s[(at*8+j as int)/8]==if j<8 {s[at]} else if j<16 {s[at+1]} else {s[at+2]});
    reveal_with_fuel(db::field,2);
}
#[inline(always)]
pub(super) fn read(s:&[u8],at:usize,skip:usize,n:usize,msb:bool) -> (r:usize)
    requires at as int+3<=s.len(),skip<8,n<=16,
    ensures r as int==if msb {lz_bits(s@,at as int*8+skip,n as nat)} else {db::field(s@,at as int*8+skip,n as nat)},
        r<db::pow(n as int),n<=12 && !msb ==> r as int==lz_lsb_bits(s@,at as int*8+skip,n as nat),
{
    let a=s[at] as u32;let b=s[at+1] as u32;let c=s[at+2] as u32;
    let w=if msb {a<<16 | b<<8 | c} else {a | b<<8 | c<<16};
    let k=skip as u32;let len=n as u32;
    proof {powers(len);assert(db::pow(len as int)>=1);}
    let r=(w>>(if msb {24-k-len} else {k}))&((1u32<<len)-1);
    proof {
        model(s@,at as int,k,len,msb);powers(len);
        assert(r<(1u32<<len)) by(bit_vector)
            requires r==extract(w,k,len,msb),k<8,len<=16;
    }
    r as usize
}
}
