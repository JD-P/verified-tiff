//! Canonical Zstandard literal trees. Weights, implicit last symbol and
//! decreasing-code-length assignment are distinct from Deflate's convention.
use super::*;
use crate::zstd_bits::{self as zb,Reverse};
use crate::zstd_fse as zf;
verus! {
pub(super) open spec fn high(n:int) -> int {if n>=1024{10}else{zf::floor(n)}}
fn high_bit(n:usize) -> (r:usize)
    requires 0<n<=2047,
    ensures r==high(n as int),r<=10,zb::pow(r as int)<=n<zb::pow(r as int+1),
{if n>=1024{10}else{zf::log(n)}}
pub(super) open spec fn count(s:Seq<u8>,w:int,n:nat) -> int decreases n {
    if n==0 {0}else{count(s,w,(n-1) as nat)+if s[n as int-1]==w{1int}else{0int}}
}
proof fn count_bound(s:Seq<u8>,w:int,n:nat)
    requires n<=s.len(),
    ensures 0<=count(s,w,n)<=n,
    decreases n,
{if n>0{count_bound(s,w,(n-1) as nat);}}
pub(super) open spec fn weight_sum(s:Seq<u8>,n:nat) -> int decreases n {
    if n==0{0}else{weight_sum(s,(n-1) as nat)+if s[n as int-1]==0{0int}else{zb::pow(s[n as int-1] as int-1)}}
}
pub(super) closed spec fn completed(s:Seq<u8>,n:usize) -> Option<(Seq<u8>,usize)> {
    if n==0 || n>255 || (exists|i:int|0<=i<n && s[i]>11) {None}else{
        let sum=weight_sum(s,n as nat);
        if sum<=0 || sum>=2048 {None}else{
            let log=high(sum)+1;let rest=zb::pow(log)-sum;let last=high(rest)+1;
            let full=s.update(n as int,last as u8);let ones=count(s,1,n as nat)+if last==1{1int}else{0int};
            if zb::pow(last-1)!=rest || ones<2 || ones%2!=0 {None}else{Some((full,log as usize))}
        }
    }
}
pub(super) fn complete(s:[u8;256],n:usize) -> (r:Result<([u8;256],usize),Error>)
    requires forall|i:int|n<=i<256 ==> s@[i]==0,
    ensures r.is_ok()==completed(s@,n).is_some(),
        r.is_ok() ==> r.unwrap().0@==completed(s@,n).unwrap().0 && r.unwrap().1==completed(s@,n).unwrap().1,
        r.is_ok() ==> 1<=r.unwrap().1<=11,
        r.is_ok() ==> forall|i:int|0<=i<256 ==> r.unwrap().0@[i]<=11,
{
    reveal(completed);
    if n==0 || n>255{return Err(Error::Invalid);}
    let mut sum=0usize;let mut ones=0usize;let mut i=0usize;
    while i<n
        invariant i<=n<=255,sum<=i*1024,sum==weight_sum(s@,i as nat),ones==count(s@,1,i as nat),ones<=i,
            forall|j:int|0<=j<i ==> s@[j]<=11,
            forall|j:int|n<=j<256 ==> s@[j]==0,
        decreases n-i,
    {
        let w=s[i] as usize;if w>11{return Err(Error::Invalid);}
        if w>0{sum=sum+zb::power(w-1) as usize;}
        if w==1{ones=ones+1;}
        i=i+1;reveal_with_fuel(weight_sum,1);reveal_with_fuel(count,1);
    }
    if sum==0 || sum>=2048{return Err(Error::Invalid);}
    let log=high_bit(sum)+1;let rest=zb::power(log) as usize-sum;let last=high_bit(rest)+1;
    if zb::power(last-1) as usize!=rest{return Err(Error::Invalid);}
    if last==1{ones=ones+1;}
    if ones<2 || ones%2!=0{return Err(Error::Invalid);}
    let mut full=s;full[n]=last as u8;Ok((full,log))
}
pub(super) open spec fn below(s:Seq<u8>,w:int,n:nat) -> int decreases n {
    if n==0 {0}else{below(s,w,(n-1) as nat)+if 0<s[n as int-1]<w{1int}else{0int}}
}
proof fn tally(s:Seq<u8>,w:int,n:nat)
    requires n<=s.len(),1<=w<=11,
    ensures 0<=count(s,w,n)<=n,0<=below(s,w,n)<=n,
        below(s,1,n)==0,below(s,w+1,n)==below(s,w,n)+count(s,w,n),
        below(s,w,n)+count(s,w,n)<=n,
    decreases n,
{if n>0{tally(s,w,(n-1) as nat);}}
proof fn below_monotone(s:Seq<u8>,a:int,b:int,n:nat)
    requires n<=s.len(),1<=a<=b<=12,
    ensures below(s,a,n)<=below(s,b,n),
    decreases n,
{if n>0{below_monotone(s,a,b,(n-1) as nat);}}
pub(super) open spec fn span(s:Seq<u8>,w:nat) -> int decreases w {
    if w<=1{0}else{span(s,(w-1) as nat)+count(s,w as int-1,256)*zb::pow(w as int-2)}
}
pub(super) open spec fn code_base(s:Seq<u8>,w:usize) -> int {span(s,w as nat)/zb::pow(w as int-1)}
pub(super) closed spec fn tree_ok(s:Seq<u8>,log:usize) -> bool {
    &&& s.len()==256 && 1<=log<=11
    &&& forall|i:int|0<=i<256 ==> s[i]<=log
    &&& span(s,12)==zb::pow(log as int)
    &&& forall|w:nat|1<=w<=log ==> #[trigger] span(s,w)%zb::pow(w as int-1)==0
    &&& count(s,1,256)>=2 && count(s,1,256)%2==0
}
pub(super) open spec fn ranked(s:Seq<u8>,w:int,r:int,start:nat) -> int decreases 256-start {
    if start>=256{-1}else if s[start as int]==w {
        if r==0{start as int}else{ranked(s,w,r-1,start+1)}
    }else{ranked(s,w,r,start+1)}
}
proof fn count_monotone(s:Seq<u8>,w:int,a:nat,b:nat)
    requires a<=b<=s.len(),
    ensures 0<=count(s,w,a)<=count(s,w,b),
    decreases b,
{if b>0{count_monotone(s,w,if a<b{a}else{(a-1) as nat},(b-1) as nat);}}
proof fn ranked_at(s:Seq<u8>,w:int,i:nat,start:nat)
    requires s.len()==256,start<=i<256,s[i as int]==w,
    ensures ranked(s,w,count(s,w,i)-count(s,w,start),start)==i,
    decreases i-start,
{
    if start<i{ranked_at(s,w,i,start+1);count_monotone(s,w,start+1,i);}
    reveal_with_fuel(ranked,1);reveal_with_fuel(count,1);
}
fn sort(s:&[u8;256],counts:&[usize;12]) -> (r:([u16;12],[u8;256]))
    requires forall|w:int|1<=w<=11 ==> counts@[w]==count(s@,w,256),
    ensures forall|w:int|1<=w<=11 ==> r.0@[w]==below(s@,w,256) && r.0@[w] as int+counts@[w]<=256,
        forall|w:int,k:int|1<=w<=11 && 0<=k<counts@[w] ==> r.1@[r.0@[w] as int+k]==ranked(s@,w,k,0),
{
    let mut starts=[0u16;12];let mut ordered=[0u8;256];let mut w=1usize;
    while w<=11
        invariant 1<=w<=12,
            forall|j:int|1<=j<=11 ==> counts@[j]==count(s@,j,256),
            forall|j:int|1<=j<w ==> starts@[j]==below(s@,j,256),
            forall|j:int,k:int|1<=j<w && 0<=k<counts@[j] ==> ordered@[starts@[j] as int+k]==ranked(s@,j,k,0),
        decreases 12-w,
    {
        let mut start=0usize;let mut i=0usize;
        while i<256
            invariant i<=256,1<=w<=11,start==below(s@,w as int,i as nat),start<=i,
            decreases 256-i,
        {if s[i]>0 && (s[i] as usize)<w {start=start+1;}i=i+1;reveal_with_fuel(below,1);}
        starts[w]=start as u16;
        let mut found=0usize;i=0;
        proof{tally(s@,w as int,256);}
        while i<256
            invariant i<=256,1<=w<=11,start==below(s@,w as int,256),found==count(s@,w as int,i as nat),found<=i,
                starts@[w as int]==start,start+counts@[w as int]<=256,
                forall|j:int|1<=j<=11 ==> counts@[j]==count(s@,j,256),
                forall|j:int|1<=j<w ==> starts@[j]==below(s@,j,256),
                forall|j:int,k:int|1<=j<w && 0<=k<counts@[j] ==> ordered@[starts@[j] as int+k]==ranked(s@,j,k,0),
                forall|k:int|0<=k<found ==> ordered@[start+k]==ranked(s@,w as int,k,0),
            decreases 256-i,
        {
            if s[i] as usize==w {
                proof{count_monotone(s@,w as int,(i+1) as nat,256);ranked_at(s@,w as int,i as nat,0);reveal_with_fuel(count,1);}
                assert forall|j:int|1<=j<w implies starts@[j] as int+counts@[j]<=start by {tally(s@,j,256);below_monotone(s@,j+1,w as int,256);}
                assert(found<counts@[w as int]);
                ordered[start+found]=i as u8;found=found+1;
            }
            i=i+1;reveal_with_fuel(count,1);
        }
        w=w+1;
    }
    assert forall|j:int|1<=j<=11 implies starts@[j] as int+counts@[j]<=256 by {tally(s@,j,256);}
    (starts,ordered)
}
#[verifier::opaque]
pub(super) open spec fn token(weights:Seq<u8>,log:usize,s:Seq<u8>,low:int,p:int,n:nat,code:int) -> Option<(u8,int)> decreases 12-n {
    if log>11 || n==0 || n>log || p<=low {None}else{
        let next=2*code+zb::value(s,p-1,1);let w=log as int+1-n;
        let base=code_base(weights,w as usize);let amount=count(weights,w,256);
        if base<=next<base+amount {Some((ranked(weights,w,next-base,0) as u8,p-1))}
        else{token(weights,log,s,low,p-1,n+1,next)}
    }
}
#[derive(Clone,Copy)]
pub(super) struct Huff {
    pub weights:[u8;256],pub counts:[usize;12],pub bases:[usize;12],pub starts:[u16;12],pub ordered:[u8;256],pub log:usize,pub ready:bool,
}
impl Huff {
    pub(super) open spec fn valid(self) -> bool {
        &&& 1<=self.log<=11
        &&& self.ready ==> forall|w:int|1<=w<=11 ==> self.counts@[w]==count(self.weights@,w,256) && self.bases@[w]==code_base(self.weights@,w as usize)
        &&& self.ready ==> forall|w:int|1<=w<=11 ==> self.starts@[w]==below(self.weights@,w,256) && self.starts@[w] as int+self.counts@[w]<=256
        &&& self.ready ==> forall|w:int,k:int|1<=w<=11 && 0<=k<self.counts@[w] ==> self.ordered@[self.starts@[w] as int+k]==ranked(self.weights@,w,k,0)
    }
    pub(super) fn build(weights:[u8;256],log:usize) -> (r:Result<Self,Error>)
        ensures r.is_ok()==tree_ok(weights@,log),
            r.is_ok() ==> r.unwrap().valid() && r.unwrap().ready && r.unwrap().log==log && r.unwrap().weights==weights,
    {
        reveal(tree_ok);
        if log==0 || log>11{return Err(Error::Unsupported);}
        let mut counts=[0usize;12];let mut i=0usize;
        while i<256
            invariant i<=256,1<=log<=11,
                forall|k:int|0<=k<i ==> weights@[k]<=log,
                forall|w:int|0<=w<=11 ==> counts@[w]==count(weights@,w,i as nat),
            decreases 256-i,
        {
            let w=weights[i] as usize;if w>log{return Err(Error::Invalid);}
            proof{count_bound(weights@,w as int,i as nat);}
            assert(counts@[w as int]<=i);
            counts[w]=counts[w]+1;i=i+1;
            assert forall|j:int|0<=j<=11 implies counts@[j]==count(weights@,j,i as nat) by {reveal_with_fuel(count,1);}
        }
        if counts[1]<2 || counts[1]%2!=0{return Err(Error::Invalid);}
        let mut bases=[0usize;12];let mut total=0usize;let mut w=1usize;
        while w<=11
            invariant 1<=w<=12,1<=log<=11,total==span(weights@,w as nat),total<=262144*(w-1),
                forall|j:int|0<=j<256 ==> weights@[j]<=log,
                forall|j:int|0<=j<=11 ==> counts@[j]==count(weights@,j,256),
                counts@[1]>=2,counts@[1]%2==0,
                forall|j:int|1<=j<w ==> bases@[j]==code_base(weights@,j as usize) && (j<=log ==> span(weights@,j as nat)%zb::pow(j-1)==0),
            decreases 12-w,
        {
            let scale=zb::power(w-1) as usize;
            if w<=log && total%scale!=0{return Err(Error::Invalid);}
            bases[w]=total/scale;
            proof{tally(weights@,w as int,256);}
            assert(counts@[w as int]<=256 && scale<=1024);
            assert(counts@[w as int] as int*scale<=262144) by(nonlinear_arith) requires counts@[w as int]<=256,scale<=1024;
            total=total+counts[w]*scale;
            w=w+1;reveal_with_fuel(span,1);
        }
        if total!=zb::power(log) as usize{return Err(Error::Invalid);}
        assert forall|j:nat|1<=j<=log implies #[trigger] span(weights@,j)%zb::pow(j as int-1)==0 by {
            let k=j as int;assert(bases@[k]==code_base(weights@,k as usize) && (k<=log ==> span(weights@,k as nat)%zb::pow(k-1)==0));
        }
        assert(tree_ok(weights@,log));
        let (starts,ordered)=sort(&weights,&counts);
        Ok(Self {weights,counts,bases,starts,ordered,log,ready:true})
    }
    pub(super) fn empty() -> (r:Self)
        ensures r.valid(),!r.ready,r.log==1,forall|i:int|0<=i<256 ==> r.weights@[i]==0,
    {Self {weights:[0u8;256],counts:[0usize;12],bases:[0usize;12],starts:[0u16;12],ordered:[0u8;256],log:1,ready:false}}
    pub(super) fn read(&self,s:&[u8],bits:&mut Reverse) -> (r:Result<u8,Error>)
        requires self.valid(),self.ready,old(bits).valid(s@),
        ensures final(bits).valid(s@),final(bits).low==old(bits).low,final(bits).end==old(bits).end,
            r.is_ok()==token(self.weights@,self.log,s@,old(bits).low as int,old(bits).p as int,1,0).is_some(),
            r.is_ok() ==> (r.unwrap(),final(bits).p as int)==token(self.weights@,self.log,s@,old(bits).low as int,old(bits).p as int,1,0).unwrap(),
    {
        let mut n=1usize;let mut code=0usize;
        let ghost goal=token(self.weights@,self.log,s@,bits.low as int,bits.p as int,1,0);
        while n<=self.log
            invariant self.valid(),self.ready,1<=n<=self.log+1,self.log<=11,
                bits.valid(s@),bits.low==old(bits).low,bits.end==old(bits).end,
                code<zb::pow(n as int-1),
                goal==token(self.weights@,self.log,s@,old(bits).low as int,old(bits).p as int,1,0),
                goal==token(self.weights@,self.log,s@,bits.low as int,bits.p as int,n as nat,code as int),
            decreases self.log+1-n,
        {
            reveal_with_fuel(token,1);
            let bit=bits.read(s,1)? as usize;
            proof{zb::power_step(n as int-1);}
            assert(code<=1023);
            code=2*code+bit;let w=self.log+1-n;
            let base=self.bases[w];let count=self.counts[w];
            if code>=base && code-base<count {
                let at=(self.starts[w] as usize)+(code-base);
                return Ok(self.ordered[at]);
            }
            n=n+1;
        }
        reveal_with_fuel(token,1);Err(Error::Invalid)
    }

}
}
