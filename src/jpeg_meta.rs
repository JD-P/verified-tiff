// JPEG-only compatibility directory reader: unordered entries are permitted;
// all-zero tombstones are ignored; duplicate recognized fields are rejected.
use super::*;
verus! {
pub(super) open spec fn slot(id:int) -> int {
    if id==256 {0} else if id==257 {1} else if id==258 {2} else if id==259 {3} else if id==262 {4}
    else if id==266 {5} else if id==273 {6} else if id==274 {7} else if id==277 {8} else if id==278 {9}
    else if id==279 {10} else if id==284 {11} else if id==317 {12} else if id==322 {13} else if id==323 {14}
    else if id==324 {15} else if id==325 {16} else if id==338 {17} else if id==339 {18} else if id==347 {19}
    else if id==512 {20} else if id==513 {21} else if id==514 {22} else if id==515 {23} else if id==519 {24}
    else if id==520 {25} else if id==521 {26} else if id==529 {27} else if id==530 {28} else if id==531 {29}
    else if id==532 {30} else {32}
}
fn index(id:u16) -> (r:usize) ensures r as int==slot(id as int),r<=32, {
    if id==256 {0} else if id==257 {1} else if id==258 {2} else if id==259 {3} else if id==262 {4}
    else if id==266 {5} else if id==273 {6} else if id==274 {7} else if id==277 {8} else if id==278 {9}
    else if id==279 {10} else if id==284 {11} else if id==317 {12} else if id==322 {13} else if id==323 {14}
    else if id==324 {15} else if id==325 {16} else if id==338 {17} else if id==339 {18} else if id==347 {19}
    else if id==512 {20} else if id==513 {21} else if id==514 {22} else if id==515 {23} else if id==519 {24}
    else if id==520 {25} else if id==521 {26} else if id==529 {27} else if id==530 {28} else if id==531 {29}
    else if id==532 {30} else {32}
}
pub(super) open spec fn blank() -> Tag {Tag {kind:0,count:0,data:0}}
pub(super) open spec fn valid(s:Seq<u8>,v:Seq<Tag>) -> bool {
    v.len()==32 && forall|i:int| 0<=i<32 ==> v[i].kind==0 ||
        (field_size(v[i].kind)>0 && range_ok(s.len() as int,v[i].data as int,v[i].count as int*field_size(v[i].kind)))
}
#[verifier::opaque]
pub(super) open spec fn entry(s:Seq<u8>,p:int,le:bool,v:Seq<Tag>) -> Option<Seq<Tag>> {
    let id=word16(s,p,le);let k=slot(id);
    if id==0 {if word32(s,p,le)==0 && word32(s,p+4,le)==0 && word32(s,p+8,le)==0 {Some(v)} else {None}}
    else if !tag_valid(s,p,le) || (k<32 && v[k].kind!=0) {None}
    else if k<32 {Some(v.update(k,tag_model(s,p,le)))} else {Some(v)}
}
fn read_entry(s:&[u8],p:usize,le:bool,v:[Tag;32]) -> (r:Result<[Tag;32],Error>)
    requires range_ok(s.len() as int,p as int,12),valid(s@,v@),
    ensures r.is_ok()==entry(s@,p as int,le,v@).is_some(),
        r.is_ok() ==> r.unwrap()@==entry(s@,p as int,le,v@).unwrap() && valid(s@,r.unwrap()@),
{
    reveal(entry);
    let id=read16(s,p,le)?;
    if id==0 {
        if read32(s,p,le)?!=0 || read32(s,p+4,le)?!=0 || read32(s,p+8,le)?!=0 {return Err(Error::Invalid);}
        return Ok(v);
    }
    let tag=tag_at(s,p,le)?;let k=index(id);let mut out=v;
    if k<32 {if v[k].kind!=0 {return Err(Error::Invalid);}out[k]=tag;}
    Ok(out)
}
#[verifier::opaque]
pub(super) open spec fn records(s:Seq<u8>,p:int,le:bool,n:nat) -> Option<Seq<Tag>> decreases n {
    if n==0 {Some(Seq::new(32,|i:int| blank()))} else {
        let a=records(s,p,le,(n-1) as nat);if a.is_none() {None} else {entry(s,p+12*(n-1),le,a.unwrap())}
    }
}
proof fn failed_prefix(s:Seq<u8>,p:int,le:bool,i:nat,n:nat)
    requires i<=n,records(s,p,le,i).is_none(),ensures records(s,p,le,n).is_none(),decreases n-i,
{
    reveal_with_fuel(records,1);if n>i {failed_prefix(s,p,le,i,(n-1) as nat);}
}
pub(super) open spec fn directory_ok(s:Seq<u8>,off:int,le:bool,limit:int) -> bool {
    let n=word16(s,off,le);
    range_ok(s.len() as int,off,2) && 0<=off+2<=usize::MAX && n<=limit &&
    range_ok(s.len() as int,off+2,12*n+4) && records(s,off+2,le,n as nat).is_some()
}
#[derive(Clone,Copy)]
pub(super) struct Meta {pub tags:[Tag;32],pub count:usize,pub next:usize,pub off:usize,pub le:bool}
impl Meta {
    pub(super) open spec fn matches(self,s:Seq<u8>) -> bool {
        self.count==word16(s,self.off as int,self.le) && self.next==next_ifd(s,self.off as int,self.le) &&
        records(s,self.off as int+2,self.le,self.count as nat)==Some(self.tags@)
    }
}
pub(super) fn directory(s:&[u8],off:usize,le:bool,limit:usize,budget:&mut Budget) -> (r:Result<Meta,Error>)
    ensures final(budget).remaining<=old(budget).remaining,
        old(budget).remaining as int-final(budget).remaining<=limit as int+1,
        r.is_ok() ==> r.unwrap().matches(s@) && valid(s@,r.unwrap().tags@) && r.unwrap().off==off && r.unwrap().le==le,
        r.is_ok() ==> r.unwrap().count<=limit,
        directory_ok(s@,off as int,le,limit as int) && old(budget).remaining>=word16(s@,off as int,le)+1 ==> r.is_ok(),
{
    let n=read16(s,off,le)? as usize;if n>limit {return Err(Error::Limit);}
    let p=add(off,2)?;let size=add(mul(n,12)?,4)?;
    if !checked_range(s.len(),p,size) {return Err(Error::Truncated);}
    spend(budget,n+1)?;
    let mut tags=[Tag {kind:0,count:0,data:0};32];let mut i=0usize;
    assert(tags@ =~= Seq::new(32,|j:int| blank()));reveal_with_fuel(records,1);
    while i<n
        invariant i<=n,n<=65535,n<=limit,p as int+12*n+4<=s.len(),budget.remaining<=old(budget).remaining,old(budget).remaining as int-budget.remaining<=limit as int+1,
            records(s@,p as int,le,i as nat)==Some(tags@),valid(s@,tags@),
            n==word16(s@,off as int,le),p as int==off as int+2,
        decreases n-i,
    {
        let a=read_entry(s,p+12*i,le,tags);
        if a.is_err() {
            proof {reveal_with_fuel(records,1);failed_prefix(s@,p as int,le,(i+1) as nat,n as nat);}
            return Err(Error::Invalid);
        }
        tags=a.unwrap();i=i+1;reveal_with_fuel(records,1);
    }
    let next=read32(s,p+12*n,le)? as usize;
    Ok(Meta {tags,count:n,next,off,le})
}
pub(super) open spec fn number(s:Seq<u8>,t:Tag,d:usize,le:bool) -> Option<usize> {
    if t.kind==0 {Some(d)} else if t.count!=1 || !value_valid(s,t,0) {None} else {Some(tag_value(s,t,0,le) as usize)}
}
pub(super) fn scalar(s:&[u8],t:Tag,d:usize,le:bool) -> (r:Result<usize,Error>)
    ensures r.is_ok()==number(s@,t,d,le).is_some(),r.is_ok() ==> r.unwrap()==number(s@,t,d,le).unwrap(),
{
    if t.kind==0 {return Ok(d);}if t.count!=1 {return Err(Error::Invalid);}
    value(s,t,0,le)
}
}
