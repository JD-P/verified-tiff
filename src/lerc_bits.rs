//! Lerc2 v3+ bit stuffing, including lookup tables and byte-aligned endings.
use super::*;
use crate::zstd_bits as zb;
use crate::native_samples as ns;
verus! {
#[derive(Clone,Copy)]
pub(super) struct Plan {pub n:usize,pub bits:usize,pub entries:usize,pub lut:usize,pub at:usize,pub next:usize,pub width:usize}
pub(super) open spec fn ceil_bytes(n:int,b:int) -> int {(n*b+7)/8}
pub(super) open spec fn index_width(n:usize) -> usize {
    if n<=2{1}else if n<=4{2}else if n<=8{3}else if n<=16{4}else if n<=32{5}else if n<=64{6}else if n<=128{7}else{8}
}
pub(super) closed spec fn plan(s:Seq<u8>,start:usize) -> Option<Plan> {
    if start>=s.len() || s.len()>usize::MAX/8{None}else{
        let h=s[start as int] as int;let b=if h/64==0{4usize}else{(3-h/64) as usize};
        if b==0 || start as int+1+b>s.len(){None}else{
            let n=ns::word(s,start as int+1,b,true);let bits=h%32;let p=start as int+1+b;
            if n>256{None}else if h%64<32{
                let end=p+ceil_bytes(n,bits);
                if end>s.len(){None}else{Some(Plan {n:n as usize,bits:bits as usize,entries:0,lut:p as usize,at:p as usize,next:end as usize,width:bits as usize})}
            }else if bits==0 || p>=s.len() || s[p]<2{None}else{
                let entries=s[p] as usize;let lut=p+1;let at=lut+ceil_bytes(entries as int-1,bits);let width=index_width(entries);
                let end=at+ceil_bytes(n,width as int);
                if end>s.len(){None}else{Some(Plan {n:n as usize,bits:bits as usize,entries,lut:lut as usize,at:at as usize,next:end as usize,width})}
            }
        }
    }
}
pub(super) fn inspect(s:&[u8],start:usize) -> (r:Result<Plan,Error>)
    ensures r.is_ok()==plan(s@,start).is_some(),r.is_ok() ==> r.unwrap()==plan(s@,start).unwrap(),
        r.is_ok() ==> r.unwrap().n<=256 && r.unwrap().bits<=31 && r.unwrap().width<=31 && r.unwrap().entries<=255,
        r.is_ok() ==> r.unwrap().at<=r.unwrap().next<=s.len() && r.unwrap().lut<=r.unwrap().at,
        r.is_ok() ==> r.unwrap().at as int*8+r.unwrap().n*r.unwrap().width<=r.unwrap().next as int*8,
        r.is_ok() ==> r.unwrap().entries>0 ==> r.unwrap().lut as int*8+(r.unwrap().entries-1)*r.unwrap().bits<=r.unwrap().at as int*8,
        r.is_ok() ==> s.len()<=usize::MAX/8,
{
    reveal(plan);
    if start>=s.len() || s.len()>usize::MAX/8{return Err(Error::Truncated);}
    let h=s[start] as usize;let b=if h/64==0{4usize}else{3-h/64};
    if b==0 || b>s.len()-start-1{return Err(Error::Truncated);}
    let n=ns::read_word(s,start+1,b,true)? as usize;let bits=h%32;let p=start+1+b;
    if n>256{return Err(Error::Invalid);}
    if h%64<32{
        assert(n*bits<=7936) by(nonlinear_arith) requires n<=256,bits<=31;
        let size=(n*bits+7)/8;
        if size>s.len()-p{return Err(Error::Truncated);}
        return Ok(Plan {n,bits,entries:0,lut:p,at:p,next:p+size,width:bits});
    }
    if bits==0 || p>=s.len() || s[p]<2{return Err(Error::Invalid);}
    let entries=s[p] as usize;let lut=p+1;
    assert((entries-1)*bits<=7874) by(nonlinear_arith) requires 2<=entries<=255,bits<=31;
    let size=((entries-1)*bits+7)/8;
    if size>s.len()-lut{return Err(Error::Truncated);}
    let at=lut+size;
    let width=if entries<=2{1usize}else if entries<=4{2}else if entries<=8{3}else if entries<=16{4}else if entries<=32{5}else if entries<=64{6}else if entries<=128{7}else{8};
    assert(n*width<=2048) by(nonlinear_arith) requires n<=256,width<=8;
    let size=(n*width+7)/8;
    if size>s.len()-at{return Err(Error::Truncated);}
    Ok(Plan {n,bits,entries,lut,at,next:at+size,width})
}
pub(super) open spec fn index(s:Seq<u8>,p:Plan,k:int) -> int {zb::value(s,p.at as int*8+k*p.width,p.width as nat)}
pub(super) open spec fn item(s:Seq<u8>,p:Plan,k:int) -> u32 {
    let q=index(s,p,k);
    if p.entries==0{q as u32}else if q==0{0u32}else{zb::value(s,p.lut as int*8+(q-1)*p.bits,p.bits as nat) as u32}
}
pub(super) closed spec fn decoded(s:Seq<u8>,start:usize) -> Option<(usize,Seq<u32>)> {
    let pp=plan(s,start);
    if pp.is_none(){None}else{
        let p=pp.unwrap();
        if p.entries!=0 && !(forall|k:int|0<=k<p.n ==> #[trigger] index(s,p,k)<p.entries){None}
        else{Some((p.next,Seq::new(p.n as nat,|k:int|item(s,p,k))))}
    }
}
pub(super) struct Array {pub at:usize,pub n:usize,pub data:[u32;256]}
pub(super) fn decode_into(s:&[u8],start:usize,data:&mut[u32;256]) -> (r:Result<(usize,usize),Error>)
    ensures r.is_ok()==decoded(s@,start).is_some(),
        r.is_ok() ==> r.unwrap().0==decoded(s@,start).unwrap().0 && r.unwrap().1<=256,
        r.is_ok() ==> final(data)@.take(r.unwrap().1 as int)==decoded(s@,start).unwrap().1,
        r.is_ok() ==> start<r.unwrap().0<=s.len(),
{
    reveal(decoded);let p=inspect(s,start)?;let mut k=0usize;
    while k<p.n
        invariant plan(s@,start)==Some(p),k<=p.n<=256,p.bits<=31,p.width<=31,p.entries<=255,p.lut<=p.at<=p.next<=s.len(),s.len()<=usize::MAX/8,
            p.at as int*8+p.n*p.width<=p.next as int*8,
            p.entries>0 ==> p.lut as int*8+(p.entries-1)*p.bits<=p.at as int*8,
            forall|j:int|0<=j<k ==> data@[j]==item(s@,p,j),
            forall|j:int|0<=j<k ==> p.entries==0 || #[trigger] index(s@,p,j)<p.entries,
        decreases p.n-k,
    {
        assert(p.at as int*8+k*p.width<=p.next as int*8) by(nonlinear_arith)
            requires k<p.n,p.at as int*8+p.n*p.width<=p.next as int*8;
        let bit=p.at*8+k*p.width;
        assert(bit as int+p.width<=p.next as int*8) by(nonlinear_arith)
            requires k<p.n,p.at as int*8+p.n*p.width<=p.next as int*8,bit==p.at as int*8+k*p.width;
        let q=crate::lerc_word::read(s,bit,p.next,p.width)?;
        if p.entries>0 && q as usize>=p.entries{
            assert(index(s@,p,k as int)==q);
            assert(!(forall|j:int|0<=j<p.n ==> #[trigger] index(s@,p,j)<p.entries));
            return Err(Error::Invalid);
        }
        let value=if p.entries==0{q}else if q==0{0u32}else{
            assert(p.lut as int*8+(q-1)*p.bits<=p.at as int*8) by(nonlinear_arith)
                requires 0<q<p.entries,p.lut as int*8+(p.entries-1)*p.bits<=p.at as int*8;
            let pos=p.lut*8+(q as usize-1)*p.bits;
            assert(pos as int+p.bits<=p.at as int*8) by(nonlinear_arith)
                requires 0<q<p.entries,p.lut as int*8+(p.entries-1)*p.bits<=p.at as int*8,pos==p.lut as int*8+(q-1)*p.bits;
            crate::lerc_word::read(s,pos,p.at,p.bits)?
        };
        data[k]=value;k=k+1;
    }
    assert(data@.take(p.n as int) =~= Seq::new(p.n as nat,|j:int|item(s@,p,j)));
    reveal(plan);
    Ok((p.next,p.n))
}
pub(super) fn decode(s:&[u8],start:usize) -> (r:Result<Array,Error>)
    ensures r.is_ok()==decoded(s@,start).is_some(),
        r.is_ok() ==> r.unwrap().at==decoded(s@,start).unwrap().0 && r.unwrap().n<=256,
        r.is_ok() ==> r.unwrap().data@.take(r.unwrap().n as int)==decoded(s@,start).unwrap().1,
        r.is_ok() ==> start<r.unwrap().at<=s.len(),
{
    let mut data=[0u32;256];let (at,n)=decode_into(s,start,&mut data)?;
    Ok(Array {at,n,data})
}
}
