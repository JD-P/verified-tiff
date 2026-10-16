//! Exact native sample words and modular horizontal prediction for 8/16/32/64 bits.
//! Signed and floating samples are transported as bits, never numerically narrowed.
use super::*;
verus! {
use crate::float64 as f64model;
pub open spec fn width_ok(bytes:usize) -> bool {bytes==1 || bytes==2 || bytes==4 || bytes==8}
pub open spec fn modulus(bytes:usize) -> int {
    if bytes==1 {256} else if bytes==2 {65536} else if bytes==4 {4294967296} else {18446744073709551616}
}
pub closed spec fn word(s:Seq<u8>,at:int,bytes:usize,le:bool) -> int {
    if bytes==1 {s[at] as int} else if bytes==2 {word16(s,at,le)}
    else if bytes==4 {word32(s,at,le)}
    else if le {word32(s,at,true)+4294967296*word32(s,at+4,true)}
    else {4294967296*word32(s,at,false)+word32(s,at+4,false)}
}
/// Expose the one-byte case for concrete container/profile witnesses.
pub proof fn byte_word(s:Seq<u8>,at:int,le:bool)
    requires 0<=at<s.len(),
    ensures word(s,at,1,le)==s[at] as int,
{reveal(word);}
#[inline(always)]
pub fn read_word(s:&[u8],at:usize,bytes:usize,le:bool) -> (r:Result<u64,Error>)
    ensures r.is_ok()==(width_ok(bytes) && range_ok(s.len() as int,at as int,bytes as int)),
        r.is_ok() ==> r.unwrap()==word(s@,at as int,bytes,le) && r.unwrap()<modulus(bytes),
{
    reveal(word);
    if !width_ok_exec(bytes) {return Err(Error::Unsupported);}
    if !checked_range(s.len(),at,bytes) {return Err(Error::Truncated);}
    if bytes==1 {Ok(s[at] as u64)}
    else if bytes==2 {Ok(read16(s,at,le)? as u64)}
    else if bytes==4 {Ok(read32(s,at,le)? as u64)}
    else {
        let a=read32(s,at,le)? as u64;let b=read32(s,at+4,le)? as u64;
        Ok(if le {a+4294967296*b} else {4294967296*a+b})
    }
}
#[inline(always)]
fn width_ok_exec(bytes:usize) -> (r:bool) ensures r==width_ok(bytes) {bytes==1 || bytes==2 || bytes==4 || bytes==8}
#[inline(always)]
fn divisor(j:usize) -> (r:u64) requires j<8,ensures r==f64model::scale(j as int),r>0,
{if j==0 {1} else if j==1 {256} else if j==2 {65536} else if j==3 {16777216} else if j==4 {4294967296} else if j==5 {1099511627776} else if j==6 {281474976710656} else {72057594037927936}}
#[inline(always)]
fn store(out:&mut [u8],at:usize,bytes:usize,le:bool,v:u64)
    requires width_ok(bytes),at as int+bytes<=old(out).len(),
    ensures final(out).len()==old(out).len(),
        forall|j:int| 0<=j<bytes ==> #[trigger] final(out)@[at as int+j]==f64model::digit(v as int,if le {j} else {bytes as int-1-j}),
        forall|k:int| 0<=k<old(out).len() && !(at<=k<at as int+bytes) ==> final(out)@[k]==old(out)@[k],
{
    let mut j=0usize;
    while j<bytes
        invariant width_ok(bytes),j<=bytes,at as int+bytes<=out.len(),out.len()==old(out).len(),
            forall|k:int| 0<=k<j ==> #[trigger] out@[at as int+k]==f64model::digit(v as int,if le {k} else {bytes as int-1-k}),
            forall|k:int| 0<=k<out.len() && !(at<=k<at as int+bytes) ==> out@[k]==old(out)@[k],
        decreases bytes-j,
    {let p=if le {j} else {bytes-1-j};out[at+j]=((v/divisor(p))%256) as u8;j=j+1;}
}
#[verifier::opaque]
pub closed spec fn sample(s:Seq<u8>,row:usize,ch:usize,bytes:usize,le:bool,predict:bool,n:int) -> int
    decreases n,
{
    let first=!predict || row==0 || ch==0 || n<ch || n%(row as int)<ch;
    (word(s,n*bytes,bytes,le)+(if first {0int} else {sample(s,row,ch,bytes,le,predict,n-ch)}))%modulus(bytes)
}
pub closed spec fn converted(s:Seq<u8>,row:usize,ch:usize,bytes:usize,le:bool,predict:bool,out_le:bool,k:int) -> u8 {
    f64model::digit(sample(s,row,ch,bytes,le,predict,k/(bytes as int)),if out_le {k%(bytes as int)} else {bytes as int-1-k%(bytes as int)})
}
pub open spec fn valid(s:Seq<u8>,row_bytes:usize,ch:usize,bytes:usize) -> bool {
    width_ok(bytes) && 0<ch<=64 && row_bytes>0 && (row_bytes as int)%(ch as int*bytes)==0 && (s.len() as int)%(row_bytes as int)==0
}
/// Input/output are separate caller buffers. All source bits and row padding
/// survive; prediction resets once per row and independently for each channel.
#[inline(always)]
pub fn convert(raw:&[u8],row_bytes:usize,ch:usize,bytes:usize,le:bool,predict:bool,out_le:bool,out:&mut [u8]) -> (r:Result<(),Error>)
    ensures r.is_ok()==(valid(raw@,row_bytes,ch,bytes) && old(out).len()>=raw.len()),
        final(out).len()==old(out).len(),
        r.is_ok() ==> forall|k:int| 0<=k<raw.len() ==> final(out)@[k]==converted(raw@,row_bytes/bytes,ch,bytes,le,predict,out_le,k),
        forall|k:int| raw.len()<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
{
    reveal(converted);
    if !width_ok_exec(bytes) || ch==0 || ch>64 || row_bytes==0 {return Err(Error::Unsupported);}
    let step=ch*bytes;
    if row_bytes%step!=0 || raw.len()%row_bytes!=0 {return Err(Error::Invalid);}
    if out.len()<raw.len() {return Err(Error::OutputTooSmall);}
    let row=row_bytes/bytes;
    proof {
        quotient_remainder(row_bytes as int, step as int);
        let q=(row_bytes as int)/(step as int);
        assert(row_bytes as int==bytes*(q*ch)) by(nonlinear_arith)
            requires row_bytes as int==step*q,step==ch*bytes;
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(row_bytes as int,bytes as int,q*ch,0);
        quotient_remainder(raw.len() as int,row_bytes as int);
        let q2=(raw.len() as int)/(row_bytes as int);
        assert(raw.len() as int==bytes*(q2*(row_bytes as int/(bytes as int)))) by(nonlinear_arith)
            requires raw.len() as int==row_bytes*q2,row_bytes as int==bytes*(row_bytes as int/(bytes as int));
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(raw.len() as int,bytes as int,q2*(row_bytes as int/(bytes as int)),0);
        assert(row>=ch) by(nonlinear_arith) requires row_bytes>0,row_bytes%step==0,step==ch*bytes,bytes>0,ch>0,row==row_bytes/bytes;
    }
    let modulo=if bytes==1 {256u128} else if bytes==2 {65536u128} else if bytes==4 {4294967296u128} else {18446744073709551616u128};
    let mut previous=[0u64;64];let mut n=0usize;
    while n<raw.len()/bytes
        invariant valid(raw@,row_bytes,ch,bytes),row==row_bytes/bytes,row>=ch,raw.len()%bytes==0,
            modulo==modulus(bytes),0<modulo<=18446744073709551616,
            out.len()==old(out).len(),out.len()>=raw.len(),n<=raw.len()/bytes,
            forall|k:int| 0<=k<n as int*bytes ==> out@[k]==converted(raw@,row,ch,bytes,le,predict,out_le,k),
            forall|k:int| raw.len()<=k<out.len() ==> out@[k]==old(out)@[k],
            forall|j:int| 0<=j<n && n as int-ch<=j ==> previous@[j%(ch as int)]==sample(raw@,row,ch,bytes,le,predict,j),
        decreases raw.len()/bytes-n,
    {
        let c=n%ch;let first=!predict || n%row<ch || n<ch;
        assert(n as int*bytes+bytes<=raw.len()) by(nonlinear_arith) requires n<raw.len()/bytes,bytes>0;
        let a=read_word(raw,n*bytes,bytes,le)?;
        let b=if first {0u64} else {previous[c]};
        proof {
            if !first {
                vstd::arithmetic::div_mod::lemma_mod_sub_multiples_vanish(n as int,ch as int);
                assert(previous@[c as int]==sample(raw@,row,ch,bytes,le,predict,n as int-ch));
            }
            reveal_with_fuel(sample,1);
        }
        let value=((a as u128+b as u128)%modulo) as u64;
        assert(value as int==sample(raw@,row,ch,bytes,le,predict,n as int));
        store(out,n*bytes,bytes,out_le,value);
        assert forall|k:int| 0<=k<(n as int+1)*bytes implies out@[k]==converted(raw@,row,ch,bytes,le,predict,out_le,k) by {
            if k>=n as int*bytes {
                vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(k,bytes as int,n as int,k-n as int*bytes);
            }
        }
        previous[c]=value;
        assert forall|j:int| 0<=j<n as int+1 && n as int+1-ch<=j implies previous@[j%(ch as int)]==sample(raw@,row,ch,bytes,le,predict,j) by {
            if j<n {
                quotient_remainder(j,ch as int);quotient_remainder(n as int,ch as int);
                assert(j%(ch as int)!=c) by(nonlinear_arith)
                    requires j<n,n as int-ch<j,ch>0,j==ch*(j/(ch as int))+j%(ch as int),n==ch*((n as int)/(ch as int))+c;
            }
        }
        n=n+1;
    }
    Ok(())
}
pub open spec fn reversed(v:u8) -> u8 {
    (((v as int)/128)%2+2*((v as int)/64%2)+4*((v as int)/32%2)+8*((v as int)/16%2)+16*((v as int)/8%2)+32*((v as int)/4%2)+64*((v as int)/2%2)+128*(v as int%2)) as u8
}
pub fn reverse_bits(s:&mut [u8])
    ensures final(s).len()==old(s).len(),forall|k:int|0<=k<old(s).len() ==> final(s)@[k]==reversed(old(s)@[k]),
{
    let mut i=0usize;
    while i<s.len()
        invariant i<=s.len(),s.len()==old(s).len(),
            forall|k:int|0<=k<i ==> s@[k]==reversed(old(s)@[k]),
            forall|k:int|i<=k<s.len() ==> s@[k]==old(s)@[k],
        decreases s.len()-i,
    {
        let v=s[i] as u16;
        let r=(v/128)%2+2*(v/64%2)+4*(v/32%2)+8*(v/16%2)+16*(v/8%2)+32*(v/4%2)+64*(v/2%2)+128*(v%2);
        s[i]=r as u8;i=i+1;
    }
}

/// Byte permutation for native component transport, including 24-bit floats.
/// Complex values call this with the component width, not the pair width.
pub fn swap_components(raw:&[u8],bytes:usize,out:&mut[u8]) -> (r:Result<(),Error>)
    ensures r.is_ok()==(0<bytes<=8 && (raw.len() as int)%(bytes as int)==0 && old(out).len()>=raw.len()),
        final(out).len()==old(out).len(),
        r.is_ok() ==> forall|k:int|0<=k<raw.len() ==> final(out)@[k]==raw@[k-k%(bytes as int)+bytes-1-k%(bytes as int)],
        forall|k:int|raw.len()<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
{
    if bytes==0||bytes>8{return Err(Error::Unsupported);}
    if raw.len()%bytes!=0{return Err(Error::Invalid);}
    if out.len()<raw.len(){return Err(Error::OutputTooSmall);}
    let mut k=0usize;
    while k<raw.len()
        invariant 0<bytes<=8,raw.len()%bytes==0,k<=raw.len(),out.len()==old(out).len(),out.len()>=raw.len(),
            forall|j:int|0<=j<k ==> out@[j]==raw@[j-j%(bytes as int)+bytes-1-j%(bytes as int)],
            forall|j:int|raw.len()<=j<out.len() ==> out@[j]==old(out)@[j],
        decreases raw.len()-k,
    {
        proof{quotient_remainder(k as int,bytes as int);quotient_remainder(raw.len() as int,bytes as int);}
        let at=k-k%bytes;
        proof {
            let q=k as int/(bytes as int);let total=raw.len() as int/(bytes as int);
            assert(q<total) by(nonlinear_arith)
                requires k<raw.len(),bytes>0,k as int==bytes*q+k as int%(bytes as int),
                    raw.len() as int==bytes*total,0<=k as int%(bytes as int)<bytes;
            assert(at as int+bytes<=raw.len()) by(nonlinear_arith)
                requires q<total,bytes>0,at as int==bytes*q,raw.len() as int==bytes*total;
        }
        let ghost prior=out@;
        out[k]=raw[at+bytes-1-k%bytes];
        assert forall|j:int|0<=j<k+1 implies out@[j]==raw@[j-j%(bytes as int)+bytes-1-j%(bytes as int)] by {
            if j==k{assert(at as int==k as int-k as int%(bytes as int));}else{assert(out@[j]==prior[j]);}
        }
        k=k+1;
    }
    Ok(())
}

}
