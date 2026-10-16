// Mapping an image coordinate into MCU-ordered 8x8 component blocks.
use super::*;
use crate::jpeg_scan::*;
use crate::jpeg_header::*;
verus! {
pub(super) open spec fn index(v:Seq<usize>,x:int,y:int,c:int) -> int {
    let h=v[15] as int;let t=v[16] as int;let mx=x/(8*h);let my=y/(8*t);let rx=x%(8*h);let ry=y%(8*t);
    let b=(my*across(v)+mx)*per_mcu(v);
    if c==0 {(b+(ry/8)*h+rx/8)*64+(ry%8)*8+rx%8}
    else {(b+h*t+c-1)*64+(ry/t)*8+rx/h}
}
pub(super) fn source(v:&[usize;32],x:usize,y:usize,c:usize) -> (r:usize)
    requires frame_ok(v@),x<v@[12],y<v@[13],c<v@[14],total(v@)*64<=usize::MAX,
    ensures r as int==index(v@,x as int,y as int,c as int),(r as int)<total(v@)*64,
{
    let h=v[15];let t=v[16];let mw=h*8;let mh=t*8;
    let a=(v[12]+mw-1)/mw;let down=(v[13]+mh-1)/mh;
    let mx=x/mw;let my=y/mh;let rx=x%mw;let ry=y%mh;
    let per=if v[14]==1 {1usize} else {h*t+2};
    assert(mx<a) by(nonlinear_arith) requires x<v@[12],mw>0,mx==x/mw,a==(v@[12]+mw-1)/(mw as int);
    assert(my<down) by(nonlinear_arith) requires y<v@[13],mh>0,my==y/mh,down==(v@[13]+mh-1)/(mh as int);
    assert((my as int*a+mx)*per+per<=a as int*down*per) by(nonlinear_arith)
        requires mx<a,my<down,per>0;
    assert(a as int*down*per==total(v@));
    assert(total(v@)>=0 && total(v@)<=usize::MAX) by(nonlinear_arith) requires total(v@)==a as int*down*per,total(v@)*64<=usize::MAX;
    assert(my as int*a+mx<=total(v@)) by(nonlinear_arith) requires per>=1,(my as int*a+mx)*per+per<=total(v@);
    let base=(my*a+mx)*per;
    if c==0 {
        let bx=rx/8;let by=ry/8;
        assert(bx<h && by<t) by(nonlinear_arith) requires rx<mw,ry<mh,mw==h*8,mh==t*8,bx==rx/8,by==ry/8;
        assert(by as int*h+bx<h*t) by(nonlinear_arith) requires bx<h,by<t,h>0;
        assert(h*t<=per);
        let b=base+(ry/8)*h+rx/8;
        assert(b as int*64+(ry%8)*8+rx%8<total(v@)*64) by(nonlinear_arith)
            requires b<total(v@),ry%8<8,rx%8<8;
        b*64+(ry%8)*8+rx%8
    } else {
        assert(c<=2 && per==h*t+2);
        assert(ry/t<8 && rx/h<8) by(nonlinear_arith) requires ry<t*8,rx<h*8,t>0,h>0;
        let b=base+h*t+c-1;
        assert(b as int*64+(ry/t)*8+rx/h<total(v@)*64) by(nonlinear_arith)
            requires b<total(v@),ry/t<8,rx/h<8;
        b*64+(ry/t)*8+rx/h
    }
}
// Within an aligned eight-pixel run, component samples are contiguous or
// repeated by the horizontal chroma factor. Compute the expensive base once.
pub(super) proof fn run_index(v:Seq<usize>,x:int,y:int,c:int,i:int)
    requires frame_ok(v),0<=x<v[12],0<=y<v[13],0<=c<v[14],x%8==0,
        0<=i<8,x+i<v[12],
    ensures index(v,x+i,y,c)==index(v,x,y,c)+(if c==0 {i} else {((x%(8*(v[15] as int)))%(v[15] as int)+i)/(v[15] as int)}),
{
    let h=v[15] as int;let mw=8*h;
    quotient_remainder(x,mw);quotient_remainder(x,8);
    let q=x/mw;let rx=x%mw;
    assert(rx==8*(x/8-h*q)) by(nonlinear_arith)
        requires x==mw*q+rx,mw==8*h,x==8*(x/8);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(rx,8,x/8-h*q,0);
    quotient_remainder(rx,8);
    assert(rx+i<mw) by(nonlinear_arith)
        requires rx==8*(rx/8),rx<mw,mw==8*h,0<=i<8;
    assert(x+i==q*mw+(rx+i)) by(nonlinear_arith) requires x==mw*q+rx;
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(x+i,mw,q,rx+i);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(rx+i,8,rx/8,i);
    if c>0 {
        quotient_remainder(rx,h);quotient_remainder(rx%h+i,h);
        let q=(rx%h+i)/h;let r=(rx%h+i)%h;
        assert(rx+i==h*(rx/h+q)+r) by(nonlinear_arith)
            requires rx==h*(rx/h)+rx%h,rx%h+i==h*q+r;
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(rx+i,h,rx/h+q,r);
    }
}

}
