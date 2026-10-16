use super::*;
use crate::fax_meta as jm;
verus! {
pub(super) open spec fn is_scalar(i:int) -> bool {i==0 || i==1 || i==3 || i==4 || i==5 || i==7 || i==8 || i==9 || i==11 || i==12 || i==13 || i==14 || i==17}
pub(super) open spec fn default(i:int) -> usize {if i==9 {4294967295usize} else if i==3 || i==5 || i==7 || i==8 || i==11 || i==12 {1usize} else {0usize}}
fn scalar_index(i:usize) -> (r:bool) ensures r==is_scalar(i as int), {i==0 || i==1 || i==3 || i==4 || i==5 || i==7 || i==8 || i==9 || i==11 || i==12 || i==13 || i==14 || i==17}
fn default_value(i:usize) -> (r:usize) ensures r==default(i as int), {if i==9 {4294967295} else if i==3 || i==5 || i==7 || i==8 || i==11 || i==12 {1} else {0}}
pub(super) open spec fn numbers_ok(s:Seq<u8>,m:jm::Meta) -> bool {forall|i:int| 0<=i<32 && is_scalar(i) ==> jm::number(s,m.tags@[i],default(i),m.le).is_some()}
pub(super) open spec fn numbers(s:Seq<u8>,m:jm::Meta) -> Seq<usize> {Seq::new(32,|i:int| if is_scalar(i) {jm::number(s,m.tags@[i],default(i),m.le).unwrap()} else {0usize})}
fn read_numbers(s:&[u8],m:jm::Meta) -> (r:Result<[usize;32],Error>)
    ensures r.is_ok()==numbers_ok(s@,m),r.is_ok() ==> r.unwrap()@==numbers(s@,m),
{
    let mut a=[0usize;32];let mut i=0usize;
    while i<32
        invariant i<=32,
            forall|j:int| 0<=j<i && is_scalar(j) ==> jm::number(s@,m.tags@[j],default(j),m.le).is_some(),
            forall|j:int| 0<=j<i ==> a@[j]==numbers(s@,m)[j],
            forall|j:int| i<=j<32 ==> a@[j]==0,
        decreases 32-i,
    {
        if scalar_index(i) {a[i]=jm::scalar(s,m.tags[i],default_value(i),m.le)?;}
        i=i+1;
    }
    assert(a@ =~= numbers(s@,m));Ok(a)
}
pub(super) open spec fn uniform(s:Seq<u8>,t:Tag,n:int,want:int,le:bool,optional:bool) -> bool {
    (optional && t.kind==0) || ((t.kind==3 || t.kind==4) && t.count==n &&
        forall|i:int| 0<=i<n ==> value_valid(s,t,i) && tag_value(s,t,i,le)==want)
}
fn all_values(s:&[u8],t:Tag,n:usize,want:usize,le:bool,optional:bool) -> (r:Result<(),Error>)
    requires n<=4,ensures r.is_ok()==uniform(s@,t,n as int,want as int,le,optional),
{
    if optional && t.kind==0 {return Ok(());}
    if (t.kind!=3 && t.kind!=4) || t.count!=n {return Err(Error::Unsupported);}
    let mut i=0usize;
    while i<n
        invariant i<=n,n<=4,t.count==n,t.kind==3 || t.kind==4,
            forall|j:int| 0<=j<i ==> value_valid(s@,t,j) && tag_value(s@,t,j,le)==want,
        decreases n-i,
    {let v=value(s,t,i,le)?;if v!=want {return Err(Error::Unsupported);}
        assert(value_valid(s@,t,i as int) && tag_value(s@,t,i as int,le)==want);
        assert forall|j:int| 0<=j<i+1 implies value_valid(s@,t,j) && tag_value(s@,t,j,le)==want by {
            if j==i {} else {assert(j<i);}
        }
        i=i+1;}
    Ok(())
}
#[derive(Clone,Copy)]
pub(super) struct Format {
    pub w:usize,pub h:usize,pub ch:usize,pub alpha:usize,pub bw:usize,pub bh:usize,
    pub tiled:bool,pub across:usize,pub segments:usize,pub output:usize,pub le:bool,pub offsets:Tag,pub counts:Tag,
}
impl Format {
    pub(super) open spec fn valid(self) -> bool {
        self.w>0 && self.h>0 && (self.ch==3 || self.ch==4) &&
        ((self.ch==3 && self.alpha==0) || (self.ch==4 && (self.alpha==1 || self.alpha==2))) &&
        (!self.tiled ==> self.bw==self.w) && 1<=self.bw<=16384 && 1<=self.bh<=16384 && self.across==(self.w as int-1)/(self.bw as int)+1 &&
        self.segments==self.across as int*((self.h as int-1)/(self.bh as int)+1) && self.output==self.w as int*self.h*self.ch
    }
}
pub(super) open spec fn model(s:Seq<u8>,m:jm::Meta) -> Format {
    let v=numbers(s,m);let tiled=m.tags@[15].kind!=0;
    let w=v[0];let h=v[1];let bw=if tiled {v[13]} else {w};let bh=if tiled {v[14]} else if v[9]<h {v[9]} else {h};
    let a=(w as int-1)/(bw as int)+1;let n=a*((h as int-1)/(bh as int)+1);
    Format {w,h,ch:v[8],alpha:v[17],bw,bh,tiled,across:a as usize,segments:n as usize,output:(w as int*h*v[8]) as usize,le:m.le,
        offsets:if tiled {m.tags@[15]} else {m.tags@[6]},counts:if tiled {m.tags@[16]} else {m.tags@[10]}}
}
#[verifier::opaque]
pub(super) open spec fn fields(s:Seq<u8>,m:jm::Meta,l:Limits) -> bool {
    let v=numbers(s,m);let f=model(s,m);
    &&& numbers_ok(s,m) && f.valid() && f.w<=l.max_dimension && f.h<=l.max_dimension && f.output<=l.max_output && f.segments<=l.max_segments
    &&& f.w as int*f.h*f.ch<=usize::MAX && f.across as int*((f.h as int-1)/(f.bh as int)+1)<=usize::MAX
    &&& v[3]==50001 && v[4]==2 && v[5]==1 && v[7]==1 && v[11]==1 && v[12]==1
    &&& (f.ch==3 ==> m.tags@[17].kind==0) && (f.ch==4 ==> m.tags@[17].kind==3 && m.tags@[17].count==1)
    &&& uniform(s,m.tags@[2],f.ch as int,8,m.le,false) && uniform(s,m.tags@[18],f.ch as int,1,m.le,true)
    &&& (f.tiled ==> m.tags@[6].kind==0 && m.tags@[10].kind==0 && f.bw%16==0 && f.bh%16==0)
    &&& (!f.tiled ==> m.tags@[13].kind==0 && m.tags@[14].kind==0 && m.tags@[16].kind==0)
    &&& (f.offsets.kind==3 || f.offsets.kind==4) && (f.counts.kind==3 || f.counts.kind==4) && f.offsets.count==f.segments && f.counts.count==f.segments
}
pub(super) fn read(s:&[u8],m:jm::Meta,l:&Limits) -> (r:Result<Format,Error>)
    ensures r.is_ok()==fields(s@,m,*l),r.is_ok() ==> r.unwrap()==model(s@,m) && r.unwrap().valid(),
{
    reveal(fields);let v=read_numbers(s,m)?;let w=v[0];let h=v[1];let ch=v[8];let alpha=v[17];
    if w==0 || h==0 || w>l.max_dimension || h>l.max_dimension || (ch!=3 && ch!=4) || v[3]!=50001 || v[4]!=2 {return Err(Error::Unsupported);}
    if v[5]!=1 || v[7]!=1 || v[11]!=1 || v[12]!=1 {return Err(Error::Unsupported);}
    if ch==3 {if alpha!=0 || m.tags[17].kind!=0 {return Err(Error::Unsupported);}}
    else if (alpha!=1 && alpha!=2) || m.tags[17].kind!=3 || m.tags[17].count!=1 {return Err(Error::Unsupported);}
    all_values(s,m.tags[2],ch,8,m.le,false)?;all_values(s,m.tags[18],ch,1,m.le,true)?;
    let tiled=m.tags[15].kind!=0;let bw=if tiled {v[13]} else {w};let bh=if tiled {v[14]} else if v[9]<h {v[9]} else {h};
    if bw==0 || bw>16384 || bh==0 || bh>16384 {return Err(Error::Unsupported);}
    if tiled {if m.tags[6].kind!=0 || m.tags[10].kind!=0 || bw%16!=0 || bh%16!=0 {return Err(Error::Invalid);}}
    else if m.tags[13].kind!=0 || m.tags[14].kind!=0 || m.tags[16].kind!=0 {return Err(Error::Invalid);}
    let across=(w-1)/bw+1;let down=(h-1)/bh+1;let segments=mul(across,down)?;
    assert(w as int*h<=w as int*h*ch) by(nonlinear_arith) requires ch>=1;
    let output=mul(mul(w,h)?,ch)?;
    if output>l.max_output || segments>l.max_segments {return Err(Error::Limit);}
    let offsets=if tiled {m.tags[15]} else {m.tags[6]};let counts=if tiled {m.tags[16]} else {m.tags[10]};
    if (offsets.kind!=3 && offsets.kind!=4) || (counts.kind!=3 && counts.kind!=4) || offsets.count!=segments || counts.count!=segments {return Err(Error::Invalid);}
    Ok(Format {w,h,ch,alpha,bw,bh,tiled,across,segments,output,le:m.le,offsets,counts})
}
}
