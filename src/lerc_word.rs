//! Bounded word extraction, proved against the original per-bit LERC model.
use super::*;
use crate::zstd_bits as zb;
verus! {
#[verifier::inline]
pub(super) open spec fn word(a:u8,b:u8,c:u8,d:u8,e:u8) -> u64 {
    a as u64 | (b as u64)<<8 | (c as u64)<<16 | (d as u64)<<24 | (e as u64)<<32
}
#[verifier::inline]
pub(super) open spec fn extract(w:u64,skip:u64,n:u64) -> u64 {(w>>skip)&(((1u64<<n)-1) as u64)}
proof fn powers(n:u64)
    requires n<=32,
    ensures (1u64<<n) as int==zb::pow(n as int),
{
    assert((1u64<<n)==if n==0 {1u64} else if n==1 {2u64} else if n==2 {4u64}
        else if n==3 {8u64} else if n==4 {16u64} else if n==5 {32u64}
        else if n==6 {64u64} else if n==7 {128u64} else if n==8 {256u64}
        else if n==9 {512u64} else if n==10 {1024u64} else if n==11 {2048u64}
        else if n==12 {4096u64} else if n==13 {8192u64} else if n==14 {16384u64}
        else if n==15 {32768u64} else if n==16 {65536u64} else if n==17 {131072u64}
        else if n==18 {262144u64} else if n==19 {524288u64} else if n==20 {1048576u64}
        else if n==21 {2097152u64} else if n==22 {4194304u64} else if n==23 {8388608u64}
        else if n==24 {16777216u64} else if n==25 {33554432u64} else if n==26 {67108864u64}
        else if n==27 {134217728u64} else if n==28 {268435456u64} else if n==29 {536870912u64}
        else if n==30 {1073741824u64} else if n==31 {2147483648u64} else {4294967296u64}) by(bit_vector) requires n<=32;
}
proof fn byte_bit(a:u8,b:u8,c:u8,d:u8,e:u8,j:u64)
    requires j<40,
    ensures ((word(a,b,c,d,e)>>j)&1) as int==
        ((if j<8 {a as int}else if j<16 {b as int}else if j<24 {c as int}else if j<32 {d as int}else{e as int})/power2(j as int%8))%2,
{
    let bit=j%8;powers(bit);assert(zb::pow(bit as int)==power2(bit as int));
    assert(((word(a,b,c,d,e)>>j)&1)==
        ((if j<8 {a as u64}else if j<16 {b as u64}else if j<24 {c as u64}else if j<32 {d as u64}else{e as u64})/(1u64<<bit))%2)
        by(bit_vector) requires j<40,bit==j%8;
}
proof fn model(s:Seq<u8>,at:int,skip:u64,n:u64)
    requires 0<=at,at+5<=s.len(),skip<=40,skip+n<=40,n<=32,
    ensures extract(word(s[at],s[at+1],s[at+2],s[at+3],s[at+4]),skip,n) as int==zb::value(s,at*8+skip,n as nat),
    decreases n,
{
    reveal_with_fuel(zb::value,1);
    let w=word(s[at],s[at+1],s[at+2],s[at+3],s[at+4]);
    if n==0 {assert(extract(w,skip,n)==0) by(bit_vector) requires n==0;}
    else {
        model(s,at,skip,(n-1) as u64);powers((n-1) as u64);
        assert(extract(w,skip,n)==extract(w,skip,(n-1) as u64)+(1u64<<((n-1) as u64))*((w>>((skip+n-1) as u64))&1))
            by(bit_vector) requires skip<=40,skip+n<=40,0<n<=32;
        assert(extract(w,skip,(n-1) as u64)<4294967296 && (1u64<<((n-1) as u64))<=2147483648 && ((w>>((skip+n-1) as u64))&1)<=1)
            by(bit_vector) requires skip<=40,skip+n<=40,0<n<=32;
        let j=(skip+n-1) as u64;byte_bit(s[at],s[at+1],s[at+2],s[at+3],s[at+4],j);
        quotient_remainder(j as int,8);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(at*8+j as int,8,at+j as int/8,j as int%8);
        assert(s[(at*8+j as int)/8]==if j<8{s[at]}else if j<16{s[at+1]}else if j<24{s[at+2]}else if j<32{s[at+3]}else{s[at+4]});
    }
}
#[inline(always)]
pub(super) fn read(s:&[u8],p:usize,end:usize,n:usize) -> (r:Result<u32,Error>)
    requires end<=s.len(),end<=usize::MAX/8,p<=end*8,n<=32,
    ensures r.is_ok()==(p as int+n<=end as int*8),
        r.is_ok() ==> r.unwrap() as int==zb::value(s@,p as int,n as nat),
        r.is_ok() ==> r.unwrap()<zb::pow(n as int),
{
    if n>end*8-p{return Err(Error::Truncated);}
    if end<5{return zb::read(s,p,end,n);}
    let at=if end-p/8>=5{p/8}else{end-5};
    let w=s[at] as u64 | (s[at+1] as u64)<<8 | (s[at+2] as u64)<<16 | (s[at+3] as u64)<<24 | (s[at+4] as u64)<<32;
    let skip=(p-at*8) as u64;let len=n as u64;
    proof {powers(len);quotient_remainder(p as int,8);assert(skip+len<=40);}
    let v=(w>>skip)&((1u64<<len)-1);
    proof {
        model(s@,at as int,skip,len);quotient_remainder(p as int,8);
        assert(v<(1u64<<len)) by(bit_vector) requires v==extract(w,skip,len),len<=32;
    }
    Ok(v as u32)
}
}
