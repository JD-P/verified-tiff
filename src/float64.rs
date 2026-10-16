//! Bit-exact IEEE binary64 transport and TIFF horizontal integer prediction.
//! No floating-point arithmetic is performed, including for NaN payloads.
use super::*;
use crate::raster_layout::RasterLayout;
verus! {

pub(super) open spec fn scale(j:int) -> int {
    if j==0 {1} else if j==1 {256} else if j==2 {65536} else if j==3 {16777216}
    else if j==4 {4294967296} else if j==5 {1099511627776} else if j==6 {281474976710656} else {72057594037927936}
}
pub(super) open spec fn digit(v:int,j:int) -> u8 {((v/scale(j))%256) as u8}

pub(super) open spec fn source(l:RasterLayout,raw:Seq<u8>,n:int,j:int) -> int {
    raw[l.source_index(n*8+(if l.le {j} else {7-j}))] as int
}
pub(super) open spec fn word(l:RasterLayout,raw:Seq<u8>,n:int) -> int {
    source(l,raw,n,0)+256*source(l,raw,n,1)+65536*source(l,raw,n,2)+16777216*source(l,raw,n,3)
    +4294967296*source(l,raw,n,4)+1099511627776*source(l,raw,n,5)+281474976710656*source(l,raw,n,6)+72057594037927936*source(l,raw,n,7)
}

/// Prefix sums reset at each storage row and tile boundary, independently per channel.
#[verifier::opaque]
pub(super) open spec fn sample(l:RasterLayout,raw:Seq<u8>,bw:usize,predict:bool,n:int) -> int
    decreases n,
{
    let first=!predict || l.ch==0 || l.w==0 || bw==0 || (n/(l.ch as int))%(l.w as int)%(bw as int)==0 || n<l.ch;
    (word(l,raw,n)+(if first {0int} else {sample(l,raw,bw,predict,n-l.ch)}))%18446744073709551616
}
pub(super) open spec fn decoded(l:RasterLayout,raw:Seq<u8>,bw:usize,predict:bool,k:int) -> u8 {
    digit(sample(l,raw,bw,predict,k/8),k%8)
}

fn load_byte(l:RasterLayout,raw:&[u8],n:usize,j:usize) -> (r:u64)
    requires l.valid(),l.bytes==8,n<l.output_len/8,j<8,raw.len()>=l.scratch_len,
    ensures r==source(l,raw@.subrange(0,l.scratch_len as int),n as int,j as int),r<=255,
{
    assert(n as int*8+7<l.output_len) by(nonlinear_arith) requires n<l.output_len/8,l.output_len>=0;
    let k=n*8+if l.le {j} else {7-j};
    raw[l.index(k)] as u64
}
fn load(l:RasterLayout,raw:&[u8],n:usize) -> (r:u64)
    requires l.valid(),l.bytes==8,n<l.output_len/8,raw.len()>=l.scratch_len,
    ensures r==word(l,raw@.subrange(0,l.scratch_len as int),n as int),
{
    let a=load_byte(l,raw,n,0);let b=load_byte(l,raw,n,1);let c=load_byte(l,raw,n,2);let d=load_byte(l,raw,n,3);
    let e=load_byte(l,raw,n,4);let f=load_byte(l,raw,n,5);let g=load_byte(l,raw,n,6);let h=load_byte(l,raw,n,7);
    a+256*b+65536*c+16777216*d+4294967296*e+1099511627776*f+281474976710656*g+72057594037927936*h
}

fn store(output:&mut [u8],n:usize,v:u64)
    requires n as int*8+8<=old(output).len(),
    ensures final(output).len()==old(output).len(),
        forall|j:int| 0<=j<8 ==> final(output)@[n as int*8+j]==digit(v as int,j),
        forall|k:int| 0<=k<old(output).len() && !(n as int*8<=k<n as int*8+8) ==> final(output)@[k]==old(output)@[k],
{
    let k=n*8;
    output[k]=(v%256) as u8;
    output[k+1]=((v/256)%256) as u8;
    output[k+2]=((v/65536)%256) as u8;
    output[k+3]=((v/16777216)%256) as u8;
    output[k+4]=((v/4294967296)%256) as u8;
    output[k+5]=((v/1099511627776)%256) as u8;
    output[k+6]=((v/281474976710656)%256) as u8;
    output[k+7]=((v/72057594037927936)%256) as u8;
    assert forall|j:int| 0<=j<8 implies output@[n as int*8+j]==digit(v as int,j) by {
        if j==0 {} else if j==1 {} else if j==2 {} else if j==3 {} else if j==4 {} else if j==5 {} else if j==6 {} else {assert(j==7);}
    }
}

fn accumulate(value:u64,previous:u64) -> (r:u64)
    ensures r as int==(value as int+previous as int)%18446744073709551616,
{
    ((value as u128+previous as u128)%18446744073709551616u128) as u64
}

pub(super) fn convert(l:RasterLayout,raw:&[u8],bw:usize,predict:bool,output:&mut [u8])
    requires l.valid(),l.bytes==8,bw>0,raw.len()>=l.scratch_len,old(output).len()>=l.output_len,
    ensures final(output).len()==old(output).len(),
        forall|k:int| 0<=k<l.output_len ==> final(output)@[k]==decoded(l,raw@.subrange(0,l.scratch_len as int),bw,predict,k),
        forall|k:int| l.output_len<=k<old(output).len() ==> final(output)@[k]==old(output)@[k],
{
    let ghost bytes=raw@.subrange(0,l.scratch_len as int);
    let mut previous=[0u64;4];
    let mut n=0usize;
    assert(l.output_len%8==0) by(nonlinear_arith) requires l.output_len as int==l.w as int*l.h*l.ch*8;
    while n<l.output_len/8
        invariant l.valid(),l.bytes==8,bw>0,raw.len()>=l.scratch_len,
            bytes==raw@.subrange(0,l.scratch_len as int),
            n<=l.output_len/8,l.output_len%8==0,
            output.len()==old(output).len(),output.len()>=l.output_len,
            forall|k:int| 0<=k<n as int*8 ==> output@[k]==decoded(l,bytes,bw,predict,k),
            forall|k:int| l.output_len<=k<output.len() ==> output@[k]==old(output)@[k],
            forall|j:int| 0<=j<n && n as int-l.ch<=j ==> previous@[j%(l.ch as int)]==sample(l,bytes,bw,predict,j),
        decreases l.output_len/8-n,
    {
        let c=n%l.ch;
        let x=(n/l.ch)%l.w;
        let first=!predict || x%bw==0;
        let raw_value=load(l,raw,n);
        let prior=if first {0u64} else {previous[c]};
        proof {
            if !first {
                assert(x>0) by {if x==0 {vstd::arithmetic::div_mod::lemma_small_mod(0,bw as nat);}}
                if l.ch==1 {} else {assert(l.ch==3);}
                assert(n>=l.ch) by {
                    if n<l.ch {
                        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(n as int,l.ch as int,0,n as int);
                        vstd::arithmetic::div_mod::lemma_small_mod(0,l.w as nat);
                    }
                }
                vstd::arithmetic::div_mod::lemma_mod_sub_multiples_vanish(n as int,l.ch as int);
                assert((n as int-l.ch)%(l.ch as int)==c);
                assert(previous@[c as int]==sample(l,bytes,bw,predict,n as int-l.ch));
            }
            reveal_with_fuel(sample,1);
            assert((raw_value as int+prior as int)%18446744073709551616==sample(l,bytes,bw,predict,n as int));
        }
        let value=accumulate(raw_value,prior);
        store(output,n,value);
        assert forall|k:int| 0<=k<(n as int+1)*8 implies output@[k]==decoded(l,bytes,bw,predict,k) by {
            if k>=n as int*8 {
                vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(k,8,n as int,k-n as int*8);
                assert(output@[k]==digit(value as int,k-n as int*8));
            }
        }
        previous[c]=value;
        assert forall|j:int| 0<=j<n as int+1 && n as int+1-l.ch<=j implies previous@[j%(l.ch as int)]==sample(l,bytes,bw,predict,j) by {
            if j<n {
                assert(j%(l.ch as int)!=c) by {if l.ch==1 {} else {assert(l.ch==3);}}
            }
        }
        n=n+1;
    }
}
}
