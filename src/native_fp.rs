//! TIFF floating Predictor 3: byte accumulation followed by significance-plane
//! unshuffle. IEEE values, including NaN payloads, remain uninterpreted bits.
use super::*;
use crate::native_samples as ns;
verus! {
pub open spec fn valid(s:Seq<u8>,row:usize,ch:usize,bytes:usize) -> bool {
    (bytes==2 || bytes==3 || bytes==4 || bytes==8) && 0<ch<=64 && row>0 && row as int%(ch as int*bytes)==0 && (s.len() as int)%(row as int)==0
}
pub open spec fn source_index(row:usize,bytes:usize,out_le:bool,k:int) -> int {
    let local=k%(row as int);
    k/(row as int)*row + (if out_le {bytes as int-1-local%(bytes as int)} else {local%(bytes as int)})*(row as int/(bytes as int))+local/(bytes as int)
}
pub closed spec fn decoded(s:Seq<u8>,row:usize,ch:usize,bytes:usize,out_le:bool,k:int) -> u8 {
    ns::converted(s,row,ch,1,true,true,true,source_index(row,bytes,out_le,k))
}
proof fn layout(row:int,ch:int,bytes:int,len:int)
    requires row>0,0<ch<=64,bytes==2 || bytes==3 || bytes==4 || bytes==8,row%(ch*bytes)==0,len%row==0,
    ensures row%bytes==0,row%ch==0,row/bytes>0,
{
    quotient_remainder(row,ch*bytes);
    let q=row/(ch*bytes);
    assert(row==bytes*(q*ch)) by(nonlinear_arith) requires row==(ch*bytes)*q;
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(row,bytes,q*ch,0);
    assert(row==ch*(q*bytes)) by(nonlinear_arith) requires row==(ch*bytes)*q;
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(row,ch,q*bytes,0);
    assert(row/bytes>0) by(nonlinear_arith) requires row>0,row%bytes==0,bytes>0;
}
proof fn index_bound(row:usize,bytes:usize,out_le:bool,k:int,len:int)
    requires 0<row,row as int%(bytes as int)==0,bytes==2 || bytes==3 || bytes==4 || bytes==8,len%(row as int)==0,0<=k<len,
    ensures 0<=source_index(row,bytes,out_le,k)<len,
{
    let local=k%(row as int);let samples=row as int/(bytes as int);
    quotient_remainder(k,row as int);quotient_remainder(local,bytes as int);quotient_remainder(row as int,bytes as int);quotient_remainder(len,row as int);
    assert(0<=local/(bytes as int)<samples) by(nonlinear_arith) requires 0<=local<row,bytes>0,row as int==bytes*samples,local==bytes*(local/(bytes as int))+local%(bytes as int),0<=local%(bytes as int)<bytes;
    let plane=if out_le {bytes as int-1-local%(bytes as int)}else{local%(bytes as int)};
    assert(0<=plane*samples+local/(bytes as int)<row) by(nonlinear_arith)
        requires 0<=plane<bytes,samples>0,0<=local/(bytes as int)<samples,row as int==bytes*samples;
    let q=k/(row as int);let total=len/(row as int);let rem=k%(row as int);
    assert(len==row*total);
    assert(q<total) by(nonlinear_arith) requires k==row*q+rem,len==row*total,0<=rem<row,0<=k<len,row>0;
    assert(q*row+row<=len) by(nonlinear_arith) requires q<total,row>0,len==row*total;

}
#[inline(always)]
pub fn decode(raw:&[u8],row:usize,ch:usize,bytes:usize,out_le:bool,scratch:&mut [u8],out:&mut [u8]) -> (r:Result<(),Error>)
    ensures r.is_ok()==(valid(raw@,row,ch,bytes) && old(scratch).len()>=raw.len() && old(out).len()>=raw.len()),
        final(out).len()==old(out).len(),final(scratch).len()==old(scratch).len(),
        r.is_ok() ==> forall|k:int| 0<=k<raw.len() ==> final(out)@[k]==decoded(raw@,row,ch,bytes,out_le,k),
        forall|k:int| raw.len()<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
        forall|k:int| raw.len()<=k<old(scratch).len() ==> final(scratch)@[k]==old(scratch)@[k],
{
    if (bytes!=2 && bytes!=3 && bytes!=4 && bytes!=8)||ch==0||ch>64||row==0{return Err(Error::Unsupported);}
    if row%(ch*bytes)!=0||raw.len()%row!=0{return Err(Error::Invalid);}
    if out.len()<raw.len()||scratch.len()<raw.len(){return Err(Error::OutputTooSmall);}
    proof{layout(row as int,ch as int,bytes as int,raw.len() as int);}
    ns::convert(raw,row,ch,1,true,true,true,scratch)?;
    let samples=row/bytes;let mut k=0usize;let mut base=0usize;let mut local=0usize;
    while k<raw.len()
        invariant valid(raw@,row,ch,bytes),row%bytes==0,samples==row/bytes,samples>0,k<=raw.len(),
            base<=k,base%row==0,local<row,k==base+local,
            scratch.len()==old(scratch).len(),out.len()==old(out).len(),out.len()>=raw.len(),scratch.len()>=raw.len(),
            forall|j:int| 0<=j<raw.len() ==> scratch@[j]==ns::converted(raw@,row,ch,1,true,true,true,j),
            forall|j:int| 0<=j<k ==> out@[j]==decoded(raw@,row,ch,bytes,out_le,j),
            forall|j:int| raw.len()<=j<out.len() ==> out@[j]==old(out)@[j],
            forall|j:int| raw.len()<=j<scratch.len() ==> scratch@[j]==old(scratch)@[j],
        decreases raw.len()-k,
    {
        proof{quotient_remainder(base as int,row as int);vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(k as int,row as int,base as int/(row as int),local as int);}
        let plane=if out_le{bytes-1-local%bytes}else{local%bytes};
        proof{index_bound(row,bytes,out_le,k as int,raw.len() as int);}
        let at=base+plane*samples+local/bytes;
        reveal(decoded);out[k]=scratch[at];k=k+1;
        if local==row-1{
            let prior=base;assert(base+row==k);base=k;local=0;
            proof{
                quotient_remainder(prior as int,row as int);
                assert(base==row*(prior as int/(row as int)+1)) by(nonlinear_arith) requires base==prior+row,prior==row*(prior as int/(row as int));
                vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(base as int,row as int,prior as int/(row as int)+1,0);
            }
        }else{local=local+1;}
    }
    Ok(())
}
}
