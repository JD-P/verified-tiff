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
#[derive(Clone,Copy)]
pub(super) struct Huff {pub lens:[u8;2328],pub counts:[usize;16]}
impl Huff {
    pub(super) open spec fn model(self) -> HModel {HModel {lens:self.lens@,counts:self.counts@}}
    pub(super) open spec fn valid(self) -> bool {
        forall|i:int| 0<=i<16 ==> self.counts@[i]==number(self.lens@,i,2328)
    }
    pub(super) fn build(lens:[u8;2328],strict:bool) -> (r:Result<Self,Error>)
        ensures r.is_ok() == tree_ok(lens@,strict),
            r.is_ok() ==> r.unwrap().valid() && r.unwrap().lens==lens && r.unwrap().model()==huff(lens@),
    {
        let ghost eligible=tree_ok(lens@,strict);
        let mut counts=[0usize;16];let mut i=0usize;
        while i<2328
            invariant i<=2328,eligible==tree_ok(lens@,strict),
                forall|k:int| 0<=k<i ==> lens@[k]<=15,
                forall|j:int| 0<=j<16 ==> counts@[j]==number(lens@,j,i as nat),
            decreases 2328-i,
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
        assert(Self {lens,counts}.model()==huff(lens@)) by {
            assert(counts@==huff(lens@).counts);
        }
        Ok(Self {lens,counts})
    }
    pub(super) fn find(self,len:usize,rank:usize) -> (r:Result<usize,Error>)
        requires self.valid(),1<=len<=15,
        ensures r.is_ok()==(ranked(self.lens@,len as int,rank as int,0)>=0),
            r.is_ok() ==> r.unwrap()==ranked(self.lens@,len as int,rank as int,0),
            r.is_ok() ==> r.unwrap()<2328,
    {
        let mut i=0usize;let mut r=rank;
        while i<2328
            invariant i<=2328,r<=rank,1<=len<=15,
                ranked(self.lens@,len as int,rank as int,0)==ranked(self.lens@,len as int,r as int,i as nat),
            decreases 2328-i,
        {
            reveal_with_fuel(ranked,1);
            if self.lens[i] as usize==len {if r==0 {return Ok(i);} r=r-1;}
            i=i+1;
        }
        reveal_with_fuel(ranked,1);
        Err(Error::Invalid)
    }
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
    fn read_canonical(self,s:&[u8],bits:&mut Bits) -> (r:Result<usize,Error>)
        requires self.valid(),old(bits).valid(s@),
        ensures final(bits).valid(s@),final(bits).end==old(bits).end,
            final(bits).pos()>=old(bits).pos(),
            r.is_ok()==symbol(s@,old(bits).pos(),old(bits).end as int,self.model(),1,0,0).is_some(),
            r.is_ok() ==> (r.unwrap() as int,final(bits).pos())==symbol(s@,old(bits).pos(),old(bits).end as int,self.model(),1,0,0).unwrap(),
            r.is_ok() ==> r.unwrap()<2328 && final(bits).pos()>old(bits).pos(),
    {
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
}
#[verifier::opaque]
pub(super) open spec fn decode(s:Seq<u8>,p:int,end:int,h:HModel) -> Option<(int,int)> {
    if h.counts[0]==2327 && h.counts[1]==1 {
        let k=ranked(h.lens,1,0,0);if k<0 {None} else {Some((k,p))}
    } else {symbol(s,p,end,h,1,0,0)}
}
impl Huff {
    pub(super) fn read(self,s:&[u8],bits:&mut Bits) -> (r:Result<usize,Error>)
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
