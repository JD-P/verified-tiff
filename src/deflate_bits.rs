// RFC 1951 bit fields and canonical Huffman codes. No runtime dependencies.
use super::*;
verus! {

pub(super) open spec fn pow(n:int) -> int {
    if n<=0 {1} else if n==1 {2} else if n==2 {4} else if n==3 {8}
    else if n==4 {16} else if n==5 {32} else if n==6 {64} else if n==7 {128}
    else if n==8 {256} else if n==9 {512} else if n==10 {1024}
    else if n==11 {2048} else if n==12 {4096} else if n==13 {8192}
    else if n==14 {16384} else if n==15 {32768} else {65536}
}
pub(super) proof fn pow_bound(n:int)
    requires 0<=n<=16,
    ensures 1<=pow(n)<=65536, n<16 ==> 2*pow(n)==pow(n+1), n>0 ==> pow(n)==2*pow(n-1),
{}

#[verifier::opaque]
pub(super) open spec fn field(s:Seq<u8>,p:int,n:nat) -> int decreases n {
    if n==0 {0} else {field(s,p,(n-1) as nat)+pow(n as int-1)*((s[(p+n-1)/8] as int / power2((p+n-1)%8))%2)}
}
// An aligned eight-bit field is exactly the stored byte.
proof fn aligned_byte(s:Seq<u8>,at:int)
    requires 0<=at<s.len(),
    ensures field(s,at*8,8)==s[at] as int,
{
    let v=s[at] as u64;
    assert(v==v%2+2*((v/2)%2)+4*((v/4)%2)+8*((v/8)%2)
        +16*((v/16)%2)+32*((v/32)%2)+64*((v/64)%2)+128*((v/128)%2)) by(bit_vector)
        requires v<=255;
    reveal_with_fuel(field,9);
}
#[derive(Clone,Copy)]
pub(super) struct Bits {pub at:usize,pub bit:usize,pub end:usize}
impl Bits {
    pub(super) open spec fn pos(self) -> int {self.at as int*8+self.bit}
    pub(super) open spec fn valid(self,s:Seq<u8>) -> bool {
        self.at<=self.end<=s.len() && self.bit<8 && (self.at==self.end ==> self.bit==0)
    }
    #[inline(always)]
    pub(super) fn read(&mut self,s:&[u8],n:usize) -> (r:Result<usize,Error>)
        requires old(self).valid(s@),n<=16,
        ensures final(self).valid(s@),final(self).end==old(self).end,
            final(self).pos()>=old(self).pos(),
            r.is_ok()==(old(self).pos()+n<=old(self).end as int*8),
            r.is_ok() ==> final(self).pos()==old(self).pos()+n,
            r.is_ok() ==> r.unwrap() as int==field(s@,old(self).pos(),n as nat),
            r.is_ok() ==> r.unwrap()<pow(n as int),
    {
        if n==8 && self.bit==0 {
            if self.at==self.end {return Err(Error::Truncated);}
            proof {aligned_byte(s@,self.at as int);}
            let byte=s[self.at];self.at=self.at+1;
            return Ok(byte as usize);
        }
        if n>1 && self.end-self.at>=3 {
            let v=crate::bit_window::read(s,self.at,self.bit,n,false);
            let next=self.bit+n;
            proof {quotient_remainder(next as int,8);}
            self.at=self.at+next/8;self.bit=next%8;
            return Ok(v);
        }
        let mut v=0usize;let mut weight=1usize;let mut i=0usize;
        reveal_with_fuel(field,1);
        while i<n
            invariant self.valid(s@),self.end==old(self).end,n<=16,i<=n,
                self.pos()==old(self).pos()+i,
                v as int==field(s@,old(self).pos(),i as nat),
                weight as int==pow(i as int),v<weight,weight<=65536,
            decreases n-i,
        {
            if self.at==self.end {return Err(Error::Truncated);}
            let b=(s[self.at]/bit_divisor(self.bit))%2;
            proof {pow_bound(i as int);}
            assert(b<=1);
            assert(v as int+weight as int*b<=65535) by(nonlinear_arith) requires v<weight,weight<=32768,b<=1;
            v=v+weight*(b as usize);weight=weight*2;
            if self.bit==7 {self.at=self.at+1;self.bit=0;} else {self.bit=self.bit+1;}
            i=i+1;
            reveal_with_fuel(field,1);
        }
        Ok(v)
    }
    pub(super) fn align(&mut self)
        requires old(self).at<=old(self).end,old(self).bit<8,old(self).at==old(self).end ==> old(self).bit==0,
        ensures final(self).end==old(self).end,final(self).bit==0,final(self).at<=final(self).end,
            final(self).at as int==(old(self).pos()+7)/8,
    {
        if self.bit!=0 {self.at=self.at+1;self.bit=0;}
    }
}

pub(super) open spec fn number(l:Seq<u8>,v:int,n:nat) -> int decreases n {
    if n==0 {0} else {number(l,v,(n-1) as nat)+if l[n as int-1]==v {1int} else {0int}}
}
proof fn number_bound(l:Seq<u8>,v:int,n:nat)
    requires n<=l.len(),
    ensures 0<=number(l,v,n)<=n,
    decreases n,
{ if n>0 {number_bound(l,v,(n-1) as nat);} }
// First symbol of length v having rank r in numerical symbol order.
pub(super) open spec fn ranked(l:Seq<u8>,v:int,r:int,i:nat) -> int decreases l.len()-i {
    if i>=l.len() {-1} else if l[i as int]==v {
        if r==0 {i as int} else {ranked(l,v,r-1,i+1)}
    } else {ranked(l,v,r,i+1)}
}
pub(super) open spec fn space(l:Seq<u8>,n:nat) -> int decreases n {
    if n==0 {1} else {2*space(l,(n-1) as nat)-number(l,n as int,288)}
}
pub(super) open spec fn tree_ok(l:Seq<u8>,strict:bool) -> bool {
    &&& l.len()==288
    &&& forall|i:int| 0<=i<288 ==> l[i]<=15
    &&& forall|n:nat| 1<=n<=15 ==> #[trigger] space(l,n)>=0
    &&& space(l,15)==0 || (!strict && (number(l,0,288)==288 || (number(l,0,288)==287 && number(l,1,288)==1)))
}
pub(super) struct HModel {pub lens:Seq<u8>,pub counts:Seq<usize>}
pub(super) open spec fn huff(l:Seq<u8>) -> HModel {
    HModel {lens:l,counts:Seq::new(16,|i:int| number(l,i,288) as usize)}
}
// Count code lengths below a bucket. Used only to prove cache capacity.
spec fn below(l:Seq<u8>,v:int,n:nat) -> int decreases n {
    if n==0 {0} else {below(l,v,(n-1) as nat)+if l[n as int-1]<v {1int} else {0int}}
}
proof fn below_step(l:Seq<u8>,v:int,n:nat)
    requires n<=l.len(),v>=0,
    ensures 0<=below(l,v,n)<=n,below(l,0,n)==0,
        below(l,v+1,n)==below(l,v,n)+number(l,v,n),
        below(l,v,n)+number(l,v,n)<=n,
    decreases n,
{ if n>0 {below_step(l,v,(n-1) as nat);} }
proof fn number_monotone(l:Seq<u8>,v:int,a:nat,b:nat)
    requires a<=b<=l.len(),
    ensures 0<=number(l,v,a)<=number(l,v,b),
    decreases b,
{ if b>0 {number_monotone(l,v,if a<b {a} else {(a-1) as nat},(b-1) as nat);} }
// The i-th symbol is the symbol with its count of earlier peers as rank.
proof fn ranked_at(l:Seq<u8>,v:int,i:nat,start:nat)
    requires start<=i<l.len(),l[i as int]==v,
    ensures ranked(l,v,number(l,v,i)-number(l,v,start),start)==i,
    decreases i-start,
{
    if start<i {ranked_at(l,v,i,start+1);number_monotone(l,v,start+1,i);}
    reveal_with_fuel(ranked,1);reveal_with_fuel(number,1);
}
proof fn ranked_absent(l:Seq<u8>,v:int,r:int,start:nat)
    requires start<=l.len(),r>=number(l,v,l.len())-number(l,v,start),
    ensures ranked(l,v,r,start)==-1,
    decreases l.len()-start,
{
    if start<l.len() {
        number_monotone(l,v,start+1,l.len());
        ranked_absent(l,v,r-if l[start as int]==v {1int} else {0int},start+1);
    }
    reveal_with_fuel(ranked,1);reveal_with_fuel(number,1);
}
#[derive(Clone,Copy)]
pub(super) struct Huff {
    pub lens:[u8;288],pub counts:[usize;16],
    pub offsets:[usize;16],pub symbols:[u16;288],
}
impl Huff {
    pub(super) open spec fn model(self) -> HModel {HModel {lens:self.lens@,counts:self.counts@}}
    pub(super) open spec fn valid(self) -> bool {
        &&& forall|i:int| 0<=i<16 ==> self.counts@[i]==number(self.lens@,i,288)
        &&& forall|i:int| 1<=i<16 ==> self.offsets@[i] as int+self.counts@[i]<=288
        &&& forall|i:int,r:int| 1<=i<16 && 0<=r<self.counts@[i] ==>
            #[trigger] self.symbols@[self.offsets@[i] as int+r] == ranked(self.lens@,i,r,0)
            && self.symbols@[self.offsets@[i] as int+r]<288
    }
    pub(super) fn build(lens:[u8;288],strict:bool) -> (r:Result<Self,Error>)
        ensures r.is_ok() == tree_ok(lens@,strict),
            r.is_ok() ==> r.unwrap().valid() && r.unwrap().lens==lens && r.unwrap().model()==huff(lens@),
    {
        let ghost eligible=tree_ok(lens@,strict);
        let mut counts=[0usize;16];let mut i=0usize;
        while i<288
            invariant i<=288,eligible==tree_ok(lens@,strict),
                forall|k:int| 0<=k<i ==> lens@[k]<=15,
                forall|j:int| 0<=j<16 ==> counts@[j]==number(lens@,j,i as nat),
            decreases 288-i,
        {
            let v=lens[i] as usize;if v>15 {return Err(Error::Invalid);}
            proof {number_bound(lens@,v as int,i as nat);}
            counts[v]=counts[v]+1;
            let ghost before=i;i=i+1;
            assert forall|j:int| 0<=j<16 implies counts@[j]==number(lens@,j,i as nat) by {reveal_with_fuel(number,1);};
        }
        let mut left=1usize;let mut n=1usize;
        while n<=15
            invariant 1<=n<=16,left<=pow(n as int-1),left as int==space(lens@,(n-1) as nat),
                eligible==tree_ok(lens@,strict),
                forall|k:int| 0<=k<288 ==> lens@[k]<=15,
                forall|j:nat| 1<=j<n ==> #[trigger] space(lens@,j)>=0,
                forall|j:int| 0<=j<16 ==> counts@[j]==number(lens@,j,288),
            decreases 16-n,
        {
            proof {pow_bound(n as int);}
            assert(eligible ==> space(lens@,n as nat)>=0);
            assert(space(lens@,n as nat)==2*(left as int)-counts@[n as int]) by {reveal_with_fuel(space,1);}
            left=left*2;
            if counts[n]>left {return Err(Error::Invalid);}
            left=left-counts[n];n=n+1;
        }
        // Empty distance alphabets are represented, but cannot decode a symbol.
        // A one-symbol length-one alphabet is the permitted incomplete tree.
        if left!=0 && (strict || (counts[0]!=288 && !(counts[0]==287 && counts[1]==1))) {return Err(Error::Invalid);}
        let (offsets,symbols)=ordered(lens,counts);
        let out=Self {lens,counts,offsets,symbols};
        assert(out.model()==huff(lens@)) by {assert(counts@==huff(lens@).counts);}
        Ok(out)
    }
    pub(super) fn find(&self,len:usize,rank:usize) -> (r:Result<usize,Error>)
        requires self.valid(),1<=len<=15,
        ensures r.is_ok()==(ranked(self.lens@,len as int,rank as int,0)>=0),
            r.is_ok() ==> r.unwrap()==ranked(self.lens@,len as int,rank as int,0),
            r.is_ok() ==> r.unwrap()<288,
    {
        if rank>=self.counts[len] {
            proof {ranked_absent(self.lens@,len as int,rank as int,0);}
            return Err(Error::Invalid);
        }
        Ok(self.symbols[self.offsets[len]+rank] as usize)
    }
}

// Stable counting order, built once per Huffman table. The pure `ranked`
// specification remains the original independent linear definition.
fn ordered(lens:[u8;288],counts:[usize;16]) -> (out:([usize;16],[u16;288]))
    requires forall|i:int| 0<=i<16 ==> counts@[i]==number(lens@,i,288),
    ensures forall|i:int| 1<=i<16 ==> out.0@[i] as int+counts@[i]<=288,
        forall|i:int,r:int| 1<=i<16 && 0<=r<counts@[i] ==>
            #[trigger] out.1@[out.0@[i] as int+r]==ranked(lens@,i,r,0) && out.1@[out.0@[i] as int+r]<288,
{
    let mut offsets=[0usize;16];let mut symbols=[0u16;288];
    let mut len=0usize;let mut base=0usize;
    proof {below_step(lens@,0,288);}
    while len<16
        invariant len<=16,base<=288,base as int==below(lens@,len as int,288),
            forall|i:int| 0<=i<16 ==> counts@[i]==number(lens@,i,288),
            forall|i:int| 0<=i<len ==> offsets@[i] as int+counts@[i]<=base,
            forall|i:int,r:int| 0<=i<len && 0<=r<counts@[i] ==>
                #[trigger] symbols@[offsets@[i] as int+r]==ranked(lens@,i,r,0) && symbols@[offsets@[i] as int+r]<288,
        decreases 16-len,
    {
        proof {below_step(lens@,len as int,288);}
        offsets[len]=base;
        let mut i=0usize;let mut rank=0usize;
        while i<288
            invariant i<=288,len<16,base<=288,base as int+counts@[len as int]<=288,
                rank as int==number(lens@,len as int,i as nat),rank<=counts@[len as int],offsets@[len as int]==base,
                forall|j:int| 0<=j<16 ==> counts@[j]==number(lens@,j,288),
                forall|j:int| 0<=j<len ==> offsets@[j] as int+counts@[j]<=base,
                forall|j:int,r:int| 0<=j<len && 0<=r<counts@[j] ==>
                    #[trigger] symbols@[offsets@[j] as int+r]==ranked(lens@,j,r,0) && symbols@[offsets@[j] as int+r]<288,
                forall|r:int| 0<=r<rank ==> #[trigger] symbols@[base as int+r]==ranked(lens@,len as int,r,0) && symbols@[base as int+r]<288,
            decreases 288-i,
        {
            if lens[i] as usize==len {
                proof {number_monotone(lens@,len as int,(i+1) as nat,288);ranked_at(lens@,len as int,i as nat,0);}
                symbols[base+rank]=i as u16;rank=rank+1;
            }
            i=i+1;
            proof {number_monotone(lens@,len as int,i as nat,288);}
        }
        base=base+rank;len=len+1;
    }
    (offsets,symbols)
}

#[verifier::opaque]
pub(super) open spec fn symbol(s:Seq<u8>,p:int,end:int,h:HModel,n:nat,code:int,first:int) -> Option<(int,int)>
    decreases 16-n,
{
    if n==0 || n>15 || p>=end*8 {None} else {
        let c=2*code+field(s,p,1);let count=h.counts[n as int] as int;
        if first<=c<first+count {
            let k=ranked(h.lens,n as int,c-first,0);
            if k<0 {None} else {Some((k,p+1))}
        } else {symbol(s,p+1,end,h,n+1,c,2*(first+count))}
    }
}
impl Huff {
    #[inline(always)]
    pub(super) fn read(&self,s:&[u8],bits:&mut Bits) -> (r:Result<usize,Error>)
        requires self.valid(),old(bits).valid(s@),
        ensures final(bits).valid(s@),final(bits).end==old(bits).end,
            final(bits).pos()>=old(bits).pos(),
            r.is_ok()==symbol(s@,old(bits).pos(),old(bits).end as int,self.model(),1,0,0).is_some(),
            r.is_ok() ==> (r.unwrap() as int,final(bits).pos())==symbol(s@,old(bits).pos(),old(bits).end as int,self.model(),1,0,0).unwrap(),
            r.is_ok() ==> r.unwrap()<288 && final(bits).pos()>old(bits).pos(),
    {
        if bits.end-bits.at>=3 {return self.read_window(s,bits);}
        let mut n=1usize;let mut code=0usize;let mut first=0usize;
        while n<=15
            invariant 1<=n<=16,self.valid(),bits.valid(s@),bits.end==old(bits).end,
                bits.pos()==old(bits).pos()+n-1,
                code<pow(n as int-1),first<=576*(pow(n as int-1)-1),
                symbol(s@,old(bits).pos(),old(bits).end as int,self.model(),1,0,0)
                    ==symbol(s@,bits.pos(),bits.end as int,self.model(),n as nat,code as int,first as int),
            decreases 16-n,
        {
            reveal_with_fuel(symbol,1);
            let b=bits.read(s,1)?;
            proof {pow_bound(n as int);number_bound(self.lens@,n as int,288);}
            code=code*2+b;
            let count=self.counts[n];
            if code>=first && code-first<count {
                let found=self.find(n,code-first)?;
                return Ok(found);
            }
            first=(first+count)*2;n=n+1;
        }
        reveal_with_fuel(symbol,1);
        Err(Error::Invalid)
    }
    #[inline(always)]
    fn read_window(&self,s:&[u8],bits:&mut Bits) -> (r:Result<usize,Error>)
        requires self.valid(),old(bits).valid(s@),old(bits).end-old(bits).at>=3,
        ensures final(bits).valid(s@),final(bits).end==old(bits).end,final(bits).pos()>=old(bits).pos(),
            r.is_ok()==symbol(s@,old(bits).pos(),old(bits).end as int,self.model(),1,0,0).is_some(),
            r.is_ok() ==> (r.unwrap() as int,final(bits).pos())==symbol(s@,old(bits).pos(),old(bits).end as int,self.model(),1,0,0).unwrap(),
            r.is_ok() ==> r.unwrap()<288 && final(bits).pos()>old(bits).pos(),
    {
        let at=bits.at;let skip=bits.bit;
        let look=crate::bit_window::read(s,at,skip,16,false);
        proof {crate::bit_window::model(s@,at as int,skip as u32,16,false);}
        let mut n=1usize;let mut code=0usize;let mut first=0usize;
        while n<=15
            invariant 1<=n<=16,self.valid(),bits.valid(s@),*bits==*old(bits),at==bits.at,skip==bits.bit,bits.end-at>=3,
                look<65536,look as int==field(s@,bits.pos(),16),
                look as u32==crate::bit_window::extract(crate::bit_window::word(s@[at as int],s@[at as int+1],s@[at as int+2],false),skip as u32,16,false),
                code<pow(n as int-1),first<=576*(pow(n as int-1)-1),
                symbol(s@,old(bits).pos(),old(bits).end as int,self.model(),1,0,0)
                    ==symbol(s@,bits.pos()+n-1,bits.end as int,self.model(),n as nat,code as int,first as int),
            decreases 16-n,
        {
            reveal_with_fuel(symbol,1);
            let bit=(look>>(n-1))&1;
            proof {
                crate::bit_window::digit(s@,at as int,skip as u32,16,(n-1) as u32);
                assert(bit as u32==((look as u32>>((n-1) as u32))&1)) by(bit_vector)
                    requires look<65536,1<=n<=15,bit==((look>>((n-1) as usize))&1);
                assert(bit<=1) by(bit_vector) requires bit==((look>>((n-1) as usize))&1);
                pow_bound(n as int);number_bound(self.lens@,n as int,288);
            }
            code=code*2+bit;let count=self.counts[n];
            if code>=first && code-first<count {
                let found=self.find(n,code-first)?;
                let next=skip+n;
                proof {quotient_remainder(next as int,8);}
                bits.at=at+next/8;bits.bit=next%8;
                return Ok(found);
            }
            first=(first+count)*2;n=n+1;
        }
        reveal_with_fuel(symbol,1);Err(Error::Invalid)
    }

}
}
