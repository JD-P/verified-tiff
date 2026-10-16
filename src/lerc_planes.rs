//! Validity preparation and per-depth native ranges.
use super::*;
use crate::lerc_header::Header;
use crate::lerc_numeric as num;
use crate::lerc_samples as sample;
use crate::lerc_mask as mask;
verus! {
pub(super) open spec fn mask_size(h:Header) -> int {(h.width as int*h.height+7)/8}
#[verifier::opaque]
pub(super) open spec fn count(s:Seq<u8>,n:nat) -> int decreases n {
    if n==0{0}else{count(s,(n-1) as nat)+if mask::valid(s,n as int-1){1int}else{0int}}
}
#[verifier::opaque]
pub(super) open spec fn validity(s:Seq<u8>,h:Header,old:Seq<u8>,reuse:bool) -> Option<Seq<u8>> {
    let size=mask_size(h) as usize;
    let v=if h.valid==0{Some(Seq::new(size as nat,|i:int|0u8))}
        else if h.valid==h.width as int*h.height{Some(Seq::new(size as nat,|i:int|255u8))}
        else if h.mask==0{if reuse{Some(old.take(size as int))}else{None}}
        else{mask::result(s,70,h.data,Seq::empty(),size,h.data as nat)};
    if v.is_none(){None}else{
        if count(v.unwrap(),(h.width as int*h.height) as nat)!=h.valid{None}else{v}
    }
}
proof fn count_uniform(s:Seq<u8>,n:nat,b:bool)
    requires forall|p:int|0<=p<n ==> mask::valid(s,p)==b,
    ensures count(s,n)==if b{n as int}else{0int},
    decreases n,
{
    reveal_with_fuel(count,1);
    if n>0{count_uniform(s,(n-1) as nat,b);}
}
pub(super) fn prepare(s:&[u8],h:Header,reuse:bool,out:&mut[u8]) -> (r:Result<(),Error>)
    requires h.invariant(),h.data<=s.len(),mask_size(h)<=old(out).len(),
    ensures final(out).len()==old(out).len(),r.is_ok()==validity(s@,h,old(out)@,reuse).is_some(),
        r.is_ok() ==> final(out)@.take(mask_size(h))==validity(s@,h,old(out)@,reuse).unwrap(),
        forall|i:int|mask_size(h)<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
{
    reveal(validity);let pixels=h.width*h.height;let size=(pixels+7)/8;
    if h.valid==0 || h.valid==pixels{
        let b=h.valid!=0;let byte=if b{255u8}else{0u8};
        crate::zstd_output::append_repeat(out,0,size,byte);
        let ghost bits=out@.take(size as int);
        assert(bits =~= Seq::new(size as nat,|i:int|byte));
        assert forall|p:int|0<=p<pixels implies mask::valid(bits,p)==b by {
            assert(0<=p/8<size);assert(bits[p/8]==byte);
        }
        proof {count_uniform(bits,pixels as nat,b);}
        return Ok(());
    }
    else if h.mask==0{if !reuse{return Err(Error::Invalid);}}
    else{mask::decode(s,70,h.data,size,out)?;}
    let ghost bits=out@.take(size as int);let mut i=0usize;let mut n=0usize;
    reveal_with_fuel(count,1);
    while i<pixels
        invariant i<=pixels,n<=i,pixels==h.width as int*h.height,size==(pixels+7)/8,size<=out.len(),
            bits==out@.take(size as int),n==count(bits,i as nat),
        decreases pixels-i,
    {
        let b=mask::is_valid(out,i);
        assert(mask::valid(out@,i as int)==mask::valid(bits,i as int));
        if b{n=n+1;}i=i+1;reveal_with_fuel(count,1);
    }
    if n!=h.valid{return Err(Error::Invalid);}Ok(())
}
pub(super) open spec fn equal(a:u64,b:u64) -> bool {a==b || (a%0x8000000000000000u64==0 && b%0x8000000000000000u64==0)}
pub(super) fn same(a:u64,b:u64) -> (r:bool) ensures r==equal(a,b),
{a==b || (a%0x8000000000000000u64==0 && b%0x8000000000000000u64==0)}
pub(super) struct Ranges {pub at:usize,pub constant:bool,pub min:[u64;64],pub max:[u64;64]}
pub(super) struct RangeModel {pub at:usize,pub constant:bool,pub min:Seq<u64>,pub max:Seq<u64>}
impl Ranges {pub(super) open spec fn model(self) -> RangeModel {RangeModel {at:self.at,constant:self.constant,min:self.min@,max:self.max@}}}
pub(super) closed spec fn ranges(s:Seq<u8>,h:Header) -> Option<RangeModel> {
    if equal(h.min,h.max){Some(RangeModel {at:h.data,constant:true,min:Seq::new(64,|i:int|h.min),max:Seq::new(64,|i:int|h.max)})}
    else{
        let size=num::type_size(h.kind);let at=h.data as int+2*h.depth*size;
        let minima=Seq::new(64,|i:int|if i<h.depth{sample::number(s,(h.data as int+i*size) as usize,h.kind).unwrap()}else{h.min});
        let maxima=Seq::new(64,|i:int|if i<h.depth{sample::number(s,(h.data as int+(h.depth+i)*size) as usize,h.kind).unwrap()}else{h.max});
        if at>s.len() || !(forall|i:int|0<=i<h.depth ==> num::finite(minima[i]) && num::finite(maxima[i]) && !num::less(maxima[i],minima[i])){None}
        else{Some(RangeModel {at:at as usize,constant:forall|i:int|0<=i<h.depth ==> equal(minima[i],maxima[i]),min:minima,max:maxima})}
    }
}
pub(super) fn read_ranges(s:&[u8],h:Header) -> (r:Result<Ranges,Error>)
    requires h.invariant(),h.data<=s.len(),s.len()<=usize::MAX/8,
    ensures r.is_ok()==ranges(s@,h).is_some(),r.is_ok() ==> r.unwrap().model()==ranges(s@,h).unwrap(),r.is_ok() ==> h.data<=r.unwrap().at<=s.len(),
{
    reveal(ranges);reveal(sample::number);
    let mut lo=[h.min;64];let mut hi=[h.max;64];
    assert(lo@ =~= Seq::new(64,|i:int|h.min));assert(hi@ =~= Seq::new(64,|i:int|h.max));
    if same(h.min,h.max){return Ok(Ranges {at:h.data,constant:true,min:lo,max:hi});}
    let size=num::size(h.kind);let bytes=2*h.depth*size;
    if bytes>s.len()-h.data{return Err(Error::Truncated);}
    let mut k=0usize;let mut constant=true;
    let ghost minima=Seq::new(64,|i:int|if i<h.depth{sample::number(s@,(h.data as int+i*size) as usize,h.kind).unwrap()}else{h.min});
    let ghost maxima=Seq::new(64,|i:int|if i<h.depth{sample::number(s@,(h.data as int+(h.depth+i)*size) as usize,h.kind).unwrap()}else{h.max});
    while k<h.depth
        invariant h.invariant(),h.data<=s.len(),s.len()<=usize::MAX/8,size==num::type_size(h.kind),size==1 || size==2 || size==4 || size==8,
            k<=h.depth,bytes==2*h.depth*size,h.data as int+bytes<=s.len(),!equal(h.min,h.max),
            minima==Seq::new(64,|i:int|if i<h.depth{sample::number(s@,(h.data as int+i*size) as usize,h.kind).unwrap()}else{h.min}),
            maxima==Seq::new(64,|i:int|if i<h.depth{sample::number(s@,(h.data as int+(h.depth+i)*size) as usize,h.kind).unwrap()}else{h.max}),
            forall|i:int|0<=i<k ==> lo@[i]==minima[i] && hi@[i]==maxima[i] && num::finite(minima[i]) && num::finite(maxima[i]) && !num::less(maxima[i],minima[i]),
            forall|i:int|k<=i<64 ==> lo@[i]==h.min && hi@[i]==h.max,
            constant==(forall|i:int|0<=i<k ==> equal(minima[i],maxima[i])),
        decreases h.depth-k,
    {
        reveal(sample::number);
        assert(h.data+(h.depth+k)*size+size<=s.len()) by(nonlinear_arith) requires k<h.depth,h.data+2*h.depth*size<=s.len(),size>0;
        assert(h.data+k*size+size<=s.len()) by(nonlinear_arith) requires k<h.depth,h.data+2*h.depth*size<=s.len(),size>0;
        let a=sample::read(s,h.data+k*size,h.kind)?;let b=sample::read(s,h.data+(h.depth+k)*size,h.kind)?;
        assert(a==minima[k as int] && b==maxima[k as int]);
        if (a/0x10000000000000u64)%2048==2047 || (b/0x10000000000000u64)%2048==2047 || num::minimum(b,a)!=a{
            assert(!(forall|i:int|0<=i<h.depth ==> num::finite(minima[i]) && num::finite(maxima[i]) && !num::less(maxima[i],minima[i])));
            return Err(Error::Invalid);
        }
        lo[k]=a;hi[k]=b;if !same(a,b){
            assert(!(forall|i:int|0<=i<k+1 ==> equal(minima[i],maxima[i])));constant=false;
        }k=k+1;
    }
    assert(lo@ =~= minima);assert(hi@ =~= maxima);
    Ok(Ranges {at:h.data+bytes,constant,min:lo,max:hi})
}
pub(super) closed spec fn constants(values:Seq<u64>,depth:usize,kind:usize) -> Option<Seq<u64>> {
    if !(forall|i:int|0<=i<depth ==> sample::storage(values[i],kind).is_some()){None}
    else{Some(Seq::new(64,|i:int|if i<depth{sample::storage(values[i],kind).unwrap()}else{0u64}))}
}
pub(super) fn convert(values:&[u64;64],depth:usize,kind:usize) -> (r:Result<[u64;64],Error>)
    requires depth<=64,
    ensures r.is_ok()==constants(values@,depth,kind).is_some(),r.is_ok() ==> r.unwrap()@==constants(values@,depth,kind).unwrap(),
{
    reveal(constants);let mut out=[0u64;64];let mut i=0usize;
    while i<depth
        invariant i<=depth<=64,
            forall|j:int|0<=j<i ==> sample::storage(values@[j],kind)==Some(out@[j]),
            forall|j:int|i<=j<64 ==> out@[j]==0,
        decreases depth-i,
    {
        let v=sample::stored(values[i],kind)?;
        assert(sample::storage(values@[i as int],kind)==Some(v));
        out[i]=v;
        assert forall|j:int|0<=j<i+1 implies sample::storage(values@[j],kind)==Some(out@[j]) by {if j==i{}}
        i=i+1;
    }
    assert forall|j:int|0<=j<depth implies sample::storage(values@[j],kind).is_some() by {}
    assert(out@ =~= constants(values@,depth,kind).unwrap());Ok(out)
}
}
