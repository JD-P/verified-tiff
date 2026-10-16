//! Keyframe header partition and one, two, four or eight token partitions.
use super::*;
use crate::vp8_bool as ar;
use crate::vp8_header as hd;
verus! {
#[derive(Clone,Copy)]
pub(super) struct Parsed {pub header:[i32;1096],pub reader:ar::Reader,pub parts:[ar::Reader;8],pub count:usize}
pub(super) struct Model {pub header:Seq<i32>,pub reader:ar::Reader,pub parts:Seq<ar::Reader>,pub count:usize}
impl Parsed {
    pub(super) open spec fn model(self) -> Model {Model {header:self.header@,reader:self.reader,parts:self.parts@,count:self.count}}
    pub(super) open spec fn valid(self,s:Seq<u8>) -> bool {self.reader.valid(s) && 1<=self.count<=8 &&
        (forall|i:int|0<=i<8 ==> #[trigger] self.parts@[i].valid(s)) && (forall|i:int|0<=i<1096 ==> -255<=#[trigger] self.header@[i]<=255)}
}
pub(super) open spec fn u24(s:Seq<u8>,p:int) -> int {s[p] as int+256*s[p+1] as int+65536*s[p+2] as int}
fn read24(s:&[u8],p:usize) -> (r:usize)
    requires p as int+3<=s.len(),ensures r==u24(s@,p as int),r<=16777215,
{s[p] as usize+256*s[p+1] as usize+65536*s[p+2] as usize}
pub(super) open spec fn cut(s:Seq<u8>,at:usize,end:usize) -> Option<usize> {
    if end-at<10{None}else{let n=u24(s,at as int)/32;if n>end-at-10{None}else{Some((at+10+n) as usize)}}
}
fn header_end(s:&[u8],at:usize,end:usize) -> (r:Result<usize,Error>)
    requires at<=end<=s.len(),s.len()<=usize::MAX/8,
    ensures r.is_ok()==cut(s@,at,end).is_some(),r.is_ok() ==> r.unwrap()==cut(s@,at,end).unwrap() && at+10<=r.unwrap()<=end,
{
    if end-at<10{return Err(Error::Truncated);}let n=read24(s,at)/32;
    if n>end-at-10{return Err(Error::Truncated);}Ok(at+10+n)
}
pub(super) open spec fn reader(at:int,end:usize) -> ar::Reader {ar::Reader {pos:(8*at) as usize,end,range:255,value:0,ready:false}}
pub(super) open spec fn count(h:Seq<i32>) -> usize {if h[30]==0{1usize}else if h[30]==1{2usize}else if h[30]==2{4usize}else{8usize}}
#[verifier::opaque]
pub(super) open spec fn partitions(s:Seq<u8>,at:usize,end:usize,total:usize,i:usize,p:usize,parts:Seq<ar::Reader>,fuel:nat) -> Option<Seq<ar::Reader>> decreases fuel {
    if i==total{Some(parts)}else if fuel==0{None}else{
        let n=if i+1==total{end-p}else{u24(s,at+3*i)};
        if n>end-p || (i+1==total && n==0){None}else{
            partitions(s,at,end,total,(i+1) as usize,(p+n) as usize,parts.update(i as int,reader(p as int,(p+n) as usize)),(fuel-1) as nat)
        }
    }
}
pub(super) open spec fn parsed(s:Seq<u8>,at:usize,end:usize) -> Option<Model> {
    let c=cut(s,at,end);if c.is_none(){None}else{
        let r=reader(at+10,c.unwrap());let h=hd::finish(s,r,Seq::new(1096,|i:int|0i32),0,1096);
        if h.is_none() || h.unwrap().1[30]<0 || h.unwrap().1[30]>3{None}else{
            let n=count(h.unwrap().1);let prefix=3*(n-1);
            if prefix>end-c.unwrap(){None}else{
                let parts=partitions(s,c.unwrap(),end,n,0,(c.unwrap()+prefix) as usize,Seq::new(8,|i:int|reader(end as int,end)),n as nat);
                if parts.is_none(){None}else{Some(Model {header:h.unwrap().1,reader:h.unwrap().0,parts:parts.unwrap(),count:n})}
            }
        }
    }
}
pub(super) fn parse(s:&[u8],at:usize,end:usize) -> (res:Result<Parsed,Error>)
    requires at<=end<=s.len(),s.len()<=usize::MAX/8,
    ensures res.is_ok()==parsed(s@,at,end).is_some(),res.is_ok() ==> res.unwrap().model()==parsed(s@,at,end).unwrap() && res.unwrap().valid(s@),
{
    let cut=header_end(s,at,end)?;let r=ar::start(at+10,cut);let mut h=[0i32;1096];
    assert(h@ =~= Seq::new(1096,|i:int|0i32));let r=hd::parse(s,r,&mut h)?;
    if h[30]<0 || h[30]>3{return Err(Error::Invalid);}
    let n=if h[30]==0{1usize}else if h[30]==1{2}else if h[30]==2{4}else{8};let prefix=3*(n-1);
    if prefix>end-cut{return Err(Error::Truncated);}
    let empty=ar::start(end,end);let mut parts=[empty;8];let mut p=cut+prefix;let mut i=0usize;
    assert(parts@ =~= Seq::new(8,|i:int|reader(end as int,end)));
    let ghost goal=parsed(s@,at,end);let ghost expected=partitions(s@,cut,end,n,0,p,parts@,n as nat);
    loop
        invariant at<=end<=s.len(),s.len()<=usize::MAX/8,cut<=end,1<=n<=8,i<=n,p<=end,prefix==3*(n-1),cut+prefix<=end,
            r.valid(s@),forall|j:int|0<=j<1096 ==> -255<=#[trigger] h@[j]<=255,
            forall|j:int|0<=j<8 ==> #[trigger] parts@[j].valid(s@),
            goal==parsed(s@,at,end),goal==(if expected.is_none(){None}else{Some(Model {header:h@,reader:r,parts:expected.unwrap(),count:n})}),
            expected==partitions(s@,cut,end,n,i,p,parts@,(n-i) as nat),
        decreases n-i,
    {
        reveal_with_fuel(partitions,1);if i==n{return Ok(Parsed {header:h,reader:r,parts,count:n});}
        let size=if i+1==n{end-p}else{read24(s,cut+3*i)};
        if size>end-p || (i+1==n && size==0){return Err(Error::Truncated);}
        parts[i]=ar::start(p,p+size);p=p+size;i=i+1;
    }
}
}
