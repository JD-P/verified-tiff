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
// Writes each sample's bytes in turn; byte r of sample j in a row comes from
// significance plane r (reversed for little-endian output) at offset j.
#[inline(always)]
fn unshuffle(raw:&[u8],row:usize,ch:usize,bytes:usize,samples:usize,out_le:bool,scratch:&[u8],out:&mut [u8])
    requires valid(raw@,row,ch,bytes),row%bytes==0,samples==row/bytes,samples>0,
        old(out).len()>=raw.len(),scratch.len()>=raw.len(),
        forall|j:int| 0<=j<raw.len() ==> scratch@[j]==ns::converted(raw@,row,ch,1,true,true,true,j),
    ensures final(out).len()==old(out).len(),
        forall|k:int| 0<=k<raw.len() ==> final(out)@[k]==decoded(raw@,row,ch,bytes,out_le,k),
        forall|k:int| raw.len()<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
{
    proof{
        quotient_remainder(row as int,bytes as int);
        assert(row as int==samples*bytes) by(nonlinear_arith) requires row as int==bytes*(row as int/(bytes as int)),samples==row/bytes;
        quotient_remainder(raw.len() as int,row as int);
        if raw.len()>0 {
            assert(row<=raw.len()) by(nonlinear_arith) requires raw.len() as int==row*(raw.len() as int/(row as int)),raw.len()>0,row>0;
        }
    }
    let mut k=0usize;let mut base=0usize;let mut j=0usize;
    while k<raw.len()
        invariant valid(raw@,row,ch,bytes),row%bytes==0,samples==row/bytes,samples>0,row as int==samples*bytes,
            k<=raw.len(),base%row==0,j<samples,k==base+j*bytes,k<raw.len() ==> base+row<=raw.len(),
            out.len()==old(out).len(),out.len()>=raw.len(),scratch.len()>=raw.len(),
            forall|i:int| 0<=i<raw.len() ==> scratch@[i]==ns::converted(raw@,row,ch,1,true,true,true,i),
            forall|i:int| 0<=i<k ==> out@[i]==decoded(raw@,row,ch,bytes,out_le,i),
            forall|i:int| raw.len()<=i<out.len() ==> out@[i]==old(out)@[i],
        decreases raw.len()-k,
    {
        assert(k+bytes<=base+row) by(nonlinear_arith) requires k==base+j*bytes,j<samples,row as int==samples*bytes;
        proof{quotient_remainder(base as int,row as int);}
        let mut r=0usize;
        while r<bytes
            invariant valid(raw@,row,ch,bytes),samples==row/bytes,samples>0,row as int==samples*bytes,
                base%row==0,base as int==row*(base as int/(row as int)),j<samples,k==base+j*bytes,k+bytes<=base+row,base+row<=raw.len(),r<=bytes,
                out.len()==old(out).len(),out.len()>=raw.len(),scratch.len()>=raw.len(),
                forall|i:int| 0<=i<raw.len() ==> scratch@[i]==ns::converted(raw@,row,ch,1,true,true,true,i),
                forall|i:int| 0<=i<k+r ==> out@[i]==decoded(raw@,row,ch,bytes,out_le,i),
                forall|i:int| raw.len()<=i<out.len() ==> out@[i]==old(out)@[i],
            decreases bytes-r,
        {
            let plane=if out_le{bytes-1-r}else{r};
            proof{
                let local=j as int*bytes+r;
                assert(0<=local<row) by(nonlinear_arith) requires local==j*bytes+r,j<samples,r<bytes,row as int==samples*bytes;
                vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(k as int+r,row as int,base as int/(row as int),local);
                vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(local,bytes as int,j as int,r as int);
                assert(plane*samples+j<row) by(nonlinear_arith) requires plane<bytes,j<samples,row as int==samples*bytes;
            }
            reveal(decoded);out[k+r]=scratch[base+plane*samples+j];r=r+1;
        }
        let ghost prior=base;
        k=k+bytes;
        if j+1==samples{
            base=base+row;j=0;
            proof{
                assert(k==base) by(nonlinear_arith) requires k==prior+(samples-1)*bytes+bytes,base==prior+row,row as int==samples*bytes;
                quotient_remainder(raw.len() as int,row as int);
                let (a,b)=(prior as int/(row as int)+1,raw.len() as int/(row as int));
                assert(base as int==a*row) by(nonlinear_arith) requires prior as int==row*(prior as int/(row as int)),base==prior+row,a==prior as int/(row as int)+1;
                vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(base as int,row as int,a,0);
                if k<raw.len() {
                    assert(base+row<=raw.len()) by(nonlinear_arith)
                        requires base as int==row*a,raw.len() as int==row*b,base<raw.len(),row>0;
                }
            }
        }else{
            j=j+1;
            assert(k==base+j*bytes) by(nonlinear_arith) requires k==base+(j-1)*bytes+bytes;
        }
    }
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
    let samples=row/bytes;
    // Constant widths let each arm unroll the per-sample byte loop.
    if bytes==4 {unshuffle(raw,row,ch,4,samples,out_le,scratch,out);}
    else if bytes==2 {unshuffle(raw,row,ch,2,samples,out_le,scratch,out);}
    else if bytes==8 {unshuffle(raw,row,ch,8,samples,out_le,scratch,out);}
    else {unshuffle(raw,row,ch,3,samples,out_le,scratch,out);}
    Ok(())
}
}
