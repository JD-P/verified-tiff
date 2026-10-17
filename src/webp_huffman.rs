// Canonical VP8L prefix codes, including the zero-bit single-symbol case.
use super::*;
use crate::deflate_bits::*;
verus! {
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
    if n==0 {1} else {2*space(l,(n-1) as nat)-number(l,n as int,2328)}
}
pub(super) open spec fn tree_ok(l:Seq<u8>,strict:bool) -> bool {
    &&& l.len()==2328
    &&& forall|i:int| 0<=i<2328 ==> l[i]<=15
    &&& forall|n:nat| 1<=n<=15 ==> #[trigger] space(l,n)>=0
    &&& space(l,15)==0 || (!strict && ((number(l,0,2328)==2327 && number(l,1,2328)==1)))
}
pub(super) struct HModel {pub lens:Seq<u8>,pub counts:Seq<usize>}
pub(super) open spec fn huff(l:Seq<u8>) -> HModel {
    HModel {lens:l,counts:Seq::new(16,|i:int| number(l,i,2328) as usize)}
}
// Count code lengths below a bucket. Used only to place each length's symbols.
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
// Lengths past the alphabet are zero, so only the alphabet needs scanning.
proof fn number_tail(l:Seq<u8>,v:int,n:nat,m:nat)
    requires n<=m<=l.len(),forall|k:int| n<=k<l.len() ==> l[k]==0,
    ensures number(l,v,m)==number(l,v,n)+if v==0 {m-n} else {0},
    decreases m,
{ if m>n {number_tail(l,v,n,(m-1) as nat);} }
// Lengths are ghost: decoding needs only the counts and ordered symbols.
#[derive(Clone,Copy)]
pub(super) struct Huff {
    pub lens:Ghost<Seq<u8>>,pub counts:[usize;16],
    pub offsets:[usize;16],pub symbols:[u16;2328],
}
impl Huff {
    pub(super) open spec fn model(self) -> HModel {HModel {lens:self.lens@,counts:self.counts@}}
    pub(super) open spec fn valid(self) -> bool {
        &&& self.lens@.len()==2328
        &&& forall|i:int| 0<=i<16 ==> self.counts@[i]==number(self.lens@,i,2328)
        &&& forall|i:int| 1<=i<16 ==> self.offsets@[i] as int+self.counts@[i]<=2328
        &&& forall|i:int,r:int| 1<=i<16 && 0<=r<self.counts@[i] ==>
            #[trigger] self.symbols@[self.offsets@[i] as int+r] == ranked(self.lens@,i,r,0)
            && self.symbols@[self.offsets@[i] as int+r]<2328
    }
    // `alphabet` bounds the entries that may be nonzero; the model still
    // ranges over all 2328 lengths.
    pub(super) fn build(lens:[u8;2328],strict:bool,alphabet:usize) -> (r:Result<Self,Error>)
        requires alphabet<=2328,forall|k:int| alphabet<=k<2328 ==> lens@[k]==0,
        ensures r.is_ok() == tree_ok(lens@,strict),
            r.is_ok() ==> r.unwrap().valid() && r.unwrap().lens@==lens@ && r.unwrap().model()==huff(lens@),
    {
        let ghost eligible=tree_ok(lens@,strict);
        let mut counts=[0usize;16];let mut i=0usize;
        while i<alphabet
            invariant i<=alphabet<=2328,eligible==tree_ok(lens@,strict),
                forall|k:int| alphabet<=k<2328 ==> lens@[k]==0,
                forall|k:int| 0<=k<i ==> lens@[k]<=15,
                forall|j:int| 0<=j<16 ==> counts@[j]==number(lens@,j,i as nat),
            decreases alphabet-i,
        {
            let v=lens[i] as usize;if v>15 {return Err(Error::Invalid);}
            proof {number_bound(lens@,v as int,i as nat);}
            counts[v]=counts[v]+1;
            let ghost before=i;i=i+1;
            assert forall|j:int| 0<=j<16 implies counts@[j]==number(lens@,j,i as nat) by {reveal_with_fuel(number,1);};
        }
        proof {number_bound(lens@,0,alphabet as nat);}
        counts[0]=counts[0]+(2328-alphabet);
        assert forall|j:int| 0<=j<16 implies counts@[j]==number(lens@,j,2328) by {
            number_tail(lens@,j,alphabet as nat,2328);
        };
        let mut left=1usize;let mut n=1usize;
        while n<=15
            invariant 1<=n<=16,left<=pow(n as int-1),left as int==space(lens@,(n-1) as nat),
                eligible==tree_ok(lens@,strict),
                forall|k:int| 0<=k<2328 ==> lens@[k]<=15,
                forall|j:nat| 1<=j<n ==> #[trigger] space(lens@,j)>=0,
                forall|j:int| 0<=j<16 ==> counts@[j]==number(lens@,j,2328),
            decreases 16-n,
        {
            proof {pow_bound(n as int);}
            assert(eligible ==> space(lens@,n as nat)>=0);
            assert(space(lens@,n as nat)==2*(left as int)-counts@[n as int]) by {reveal_with_fuel(space,1);}
            left=left*2;
            if counts[n]>left {return Err(Error::Invalid);}
            left=left-counts[n];n=n+1;
        }
        // VP8L permits a single leaf, but no empty or incomplete multi-leaf tree.
        if left!=0 && (strict || !(counts[0]==2327 && counts[1]==1)) {return Err(Error::Invalid);}
        let (offsets,symbols)=ordered(lens,counts,alphabet);
        let out=Self {lens:Ghost(lens@),counts,offsets,symbols};
        assert(out.model()==huff(lens@)) by {assert(counts@==huff(lens@).counts);}
        Ok(out)
    }
    pub(super) fn find(&self,len:usize,rank:usize) -> (r:Result<usize,Error>)
        requires self.valid(),1<=len<=15,
        ensures r.is_ok()==(ranked(self.lens@,len as int,rank as int,0)>=0),
            r.is_ok() ==> r.unwrap()==ranked(self.lens@,len as int,rank as int,0),
            r.is_ok() ==> r.unwrap()<2328,
    {
        if rank>=self.counts[len] {
            proof {ranked_absent(self.lens@,len as int,rank as int,0);}
            return Err(Error::Invalid);
        }
        Ok(self.symbols[self.offsets[len]+rank] as usize)
    }
}

// Stable counting order, built once per prefix code in two linear passes:
// prefix sums of the counts, then one scatter of every symbol. The pure
// `ranked` specification remains the original independent linear definition.
fn ordered(lens:[u8;2328],counts:[usize;16],alphabet:usize) -> (out:([usize;16],[u16;2328]))
    requires forall|i:int| 0<=i<16 ==> counts@[i]==number(lens@,i,2328),
        forall|i:int| 0<=i<2328 ==> lens@[i]<=15,
        alphabet<=2328,forall|k:int| alphabet<=k<2328 ==> lens@[k]==0,
    ensures forall|i:int| 1<=i<16 ==> out.0@[i] as int+counts@[i]<=2328,
        forall|i:int,r:int| 1<=i<16 && 0<=r<counts@[i] ==>
            #[trigger] out.1@[out.0@[i] as int+r]==ranked(lens@,i,r,0) && out.1@[out.0@[i] as int+r]<2328,
{
    let mut offsets=[0usize;16];
    let mut len=0usize;let mut base=0usize;
    proof {below_step(lens@,0,2328);}
    while len<16
        invariant len<=16,base<=2328,base as int==below(lens@,len as int,2328),
            forall|i:int| 0<=i<16 ==> counts@[i]==number(lens@,i,2328),
            forall|i:int| 0<=i<len ==> offsets@[i] as int+counts@[i]<=base,
            forall|a:int,b:int| 0<=a<b<len ==> offsets@[a] as int+counts@[a]<=offsets@[b],
        decreases 16-len,
    {
        proof {below_step(lens@,len as int,2328);}
        offsets[len]=base;base=base+counts[len];len=len+1;
    }
    let mut next=offsets;let mut symbols=[0u16;2328];let mut i=0usize;
    while i<alphabet
        invariant i<=alphabet<=2328,
            forall|j:int| 0<=j<16 ==> counts@[j]==number(lens@,j,2328),
            forall|k:int| 0<=k<2328 ==> lens@[k]<=15,
            forall|k:int| alphabet<=k<2328 ==> lens@[k]==0,
            forall|j:int| 0<=j<16 ==> offsets@[j] as int+counts@[j]<=2328,
            forall|a:int,b:int| 0<=a<b<16 ==> offsets@[a] as int+counts@[a]<=offsets@[b],
            forall|j:int| 1<=j<16 ==> #[trigger] next@[j] as int==offsets@[j]+number(lens@,j,i as nat),
            forall|j:int,r:int| 1<=j<16 && 0<=r<number(lens@,j,i as nat) ==>
                #[trigger] symbols@[offsets@[j] as int+r]==ranked(lens@,j,r,0) && symbols@[offsets@[j] as int+r]<2328,
        decreases alphabet-i,
    {
        let l=lens[i] as usize;
        if l!=0 {
            proof {
                number_monotone(lens@,l as int,(i+1) as nat,2328);
                ranked_at(lens@,l as int,i as nat,0);
            }
            let at=next[l];
            let ghost before=symbols@;let ghost old_next=next@;
            proof {
                number_monotone(lens@,l as int,0,i as nat);
                assert(number(lens@,l as int,(i+1) as nat)==number(lens@,l as int,i as nat)+1);
                assert(at as int==offsets@[l as int]+number(lens@,l as int,i as nat));
                assert(at as int+1<=offsets@[l as int]+counts@[l as int]);
            }
            symbols[at]=i as u16;next[l]=at+1;
            proof {
                assert forall|j:int| 1<=j<16 implies #[trigger] next@[j] as int==offsets@[j]+number(lens@,j,(i+1) as nat) by {
                    assert(old_next[j] as int==offsets@[j]+number(lens@,j,i as nat));
                };
                assert forall|j:int,r:int| 1<=j<16 && 0<=r<number(lens@,j,(i+1) as nat) implies
                    #[trigger] symbols@[offsets@[j] as int+r]==ranked(lens@,j,r,0) && symbols@[offsets@[j] as int+r]<2328 by {
                    if j==l as int && r==number(lens@,j,i as nat) {
                        assert(offsets@[j] as int+r==at as int);
                    } else {
                        if j!=l as int {assert(number(lens@,j,(i+1) as nat)==number(lens@,j,i as nat));}
                        assert(r<number(lens@,j,i as nat));
                        number_monotone(lens@,j,i as nat,2328);
                        if j<l as int {assert(offsets@[j] as int+counts@[j]<=offsets@[l as int]);}
                        else if j>l as int {assert(offsets@[l as int] as int+counts@[l as int]<=offsets@[j]);}
                        assert(offsets@[j] as int+r!=at as int);
                        assert(before[offsets@[j] as int+r]==ranked(lens@,j,r,0));
                    }
                };
            }
        } else {
            proof {
                assert forall|j:int| 1<=j<16 implies number(lens@,j,(i+1) as nat)==number(lens@,j,i as nat) by {};
            }
        }
        i=i+1;
    }
    proof {
        assert forall|j:int| 1<=j<16 implies number(lens@,j,alphabet as nat)==counts@[j] by {
            number_tail(lens@,j,alphabet as nat,2328);
        };
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
    fn read_canonical(&self,s:&[u8],bits:&mut Bits) -> (r:Result<usize,Error>)
        requires self.valid(),old(bits).valid(s@),
        ensures final(bits).valid(s@),final(bits).end==old(bits).end,
            final(bits).pos()>=old(bits).pos(),
            r.is_ok()==symbol(s@,old(bits).pos(),old(bits).end as int,self.model(),1,0,0).is_some(),
            r.is_ok() ==> (r.unwrap() as int,final(bits).pos())==symbol(s@,old(bits).pos(),old(bits).end as int,self.model(),1,0,0).unwrap(),
            r.is_ok() ==> r.unwrap()<2328 && final(bits).pos()>old(bits).pos(),
    {
        if bits.end-bits.at>=3 {return self.read_window(s,bits);}
        let mut n=1usize;let mut code=0usize;let mut first=0usize;
        while n<=15
            invariant 1<=n<=16,self.valid(),bits.valid(s@),bits.end==old(bits).end,
                bits.pos()==old(bits).pos()+n-1,
                code<pow(n as int-1),first<=4656*(pow(n as int-1)-1),
                symbol(s@,old(bits).pos(),old(bits).end as int,self.model(),1,0,0)
                    ==symbol(s@,bits.pos(),bits.end as int,self.model(),n as nat,code as int,first as int),
            decreases 16-n,
        {
            reveal_with_fuel(symbol,1);
            let b=bits.read(s,1)?;
            proof {pow_bound(n as int);number_bound(self.lens@,n as int,2328);}
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
    // One 16-bit LSB-first lookahead serves every canonical length <= 15.
    #[inline(always)]
    fn read_window(&self,s:&[u8],bits:&mut Bits) -> (r:Result<usize,Error>)
        requires self.valid(),old(bits).valid(s@),old(bits).end-old(bits).at>=3,
        ensures final(bits).valid(s@),final(bits).end==old(bits).end,final(bits).pos()>=old(bits).pos(),
            r.is_ok()==symbol(s@,old(bits).pos(),old(bits).end as int,self.model(),1,0,0).is_some(),
            r.is_ok() ==> (r.unwrap() as int,final(bits).pos())==symbol(s@,old(bits).pos(),old(bits).end as int,self.model(),1,0,0).unwrap(),
            r.is_ok() ==> r.unwrap()<2328 && final(bits).pos()>old(bits).pos(),
    {
        let at=bits.at;let skip=bits.bit;
        let look=crate::bit_window::read(s,at,skip,16,false);
        proof {crate::bit_window::model(s@,at as int,skip as u32,16,false);}
        let mut n=1usize;let mut code=0usize;let mut first=0usize;
        while n<=15
            invariant 1<=n<=16,self.valid(),bits.valid(s@),*bits==*old(bits),at==bits.at,skip==bits.bit,bits.end-at>=3,
                look<65536,look as int==field(s@,bits.pos(),16),
                look as u32==crate::bit_window::extract(crate::bit_window::word(s@[at as int],s@[at as int+1],s@[at as int+2],false),skip as u32,16,false),
                code<pow(n as int-1),first<=4656*(pow(n as int-1)-1),
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
                pow_bound(n as int);number_bound(self.lens@,n as int,2328);
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
#[verifier::opaque]
pub(super) open spec fn decode(s:Seq<u8>,p:int,end:int,h:HModel) -> Option<(int,int)> {
    if h.counts[0]==2327 && h.counts[1]==1 {
        let k=ranked(h.lens,1,0,0);if k<0 {None} else {Some((k,p))}
    } else {symbol(s,p,end,h,1,0,0)}
}
impl Huff {
    #[inline(always)]
    pub(super) fn read(&self,s:&[u8],bits:&mut Bits) -> (r:Result<usize,Error>)
        requires self.valid(),old(bits).valid(s@),
        ensures final(bits).valid(s@),final(bits).end==old(bits).end,final(bits).pos()>=old(bits).pos(),
            r.is_ok()==decode(s@,old(bits).pos(),old(bits).end as int,self.model()).is_some(),
            r.is_ok() ==> (r.unwrap() as int,final(bits).pos())==decode(s@,old(bits).pos(),old(bits).end as int,self.model()).unwrap(),
            r.is_ok() ==> r.unwrap()<2328,
    {
        reveal(decode);
        if self.counts[0]==2327 && self.counts[1]==1 {self.find(1,0)} else {self.read_canonical(s,bits)}
    }
}
}
