use super::*;
use crate::fax_meta as jm;
verus! {
pub(super) open spec fn is_scalar(i:int) -> bool {i==0 || i==1 || i==3 || i==4 || i==5 || i==7 || i==8 || i==9 || i==11 || i==12 || i==13 || i==14 || i==19 || i==20}
pub(super) open spec fn default(i:int) -> usize {if i==9 {4294967295usize} else if i==3 || i==5 || i==7 || i==8 || i==11 || i==12 {1usize} else {0usize}}
fn scalar_index(i:usize) -> (r:bool) ensures r==is_scalar(i as int), {i==0 || i==1 || i==3 || i==4 || i==5 || i==7 || i==8 || i==9 || i==11 || i==12 || i==13 || i==14 || i==19 || i==20}
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
#[derive(Clone,Copy)]
pub(super) struct Format {
    pub w:usize,pub h:usize,pub compression:usize,pub photo:usize,pub bw:usize,pub bh:usize,
    pub tiled:bool,pub across:usize,pub segments:usize,pub output:usize,pub lsb:bool,pub mixed:bool,pub aligned:bool,pub le:bool,
    pub offsets:Tag,pub counts:Tag,
}
impl Format {
    pub(super) open spec fn valid(self) -> bool {
        self.w>0 && self.h>0 && (self.compression==3 || self.compression==4) && self.photo<=1 &&
        (!self.tiled ==> self.bw==self.w) && 1<=self.bw<=65536 && self.bh>=1 && self.across==(self.w as int-1)/(self.bw as int)+1 &&
        self.segments==self.across as int*((self.h as int-1)/(self.bh as int)+1) && self.output==self.w as int*self.h
    }
}
pub(super) open spec fn model(s:Seq<u8>,m:jm::Meta) -> Format {
    let v=numbers(s,m);let tiled=m.tags@[15].kind!=0;
    let w=v[0];let h=v[1];let bw=if tiled {v[13]} else {w};let bh=if tiled {v[14]} else if v[9]<h {v[9]} else {h};
    let a=(w as int-1)/(bw as int)+1;let n=a*((h as int-1)/(bh as int)+1);
    Format {w,h,compression:v[3],photo:v[4],bw,bh,tiled,across:a as usize,segments:n as usize,output:(w as int*h) as usize,
        lsb:v[5]==2,mixed:v[19]%2==1,aligned:v[19]>=4,le:m.le,
        offsets:if tiled {m.tags@[15]} else {m.tags@[6]},counts:if tiled {m.tags@[16]} else {m.tags@[10]}}
}
pub(super) open spec fn one_sample(s:Seq<u8>,t:Tag,le:bool) -> bool {
    t.kind==0 || (t.kind==3 && t.count==1 && value_valid(s,t,0) && tag_value(s,t,0,le)==1)
}
fn sample(s:&[u8],t:Tag,le:bool) -> (r:Result<(),Error>) ensures r.is_ok()==one_sample(s@,t,le), {
    if t.kind==0 {return Ok(());}if t.kind!=3 || t.count!=1 {return Err(Error::Unsupported);}
    if value(s,t,0,le)?!=1 {return Err(Error::Unsupported);}Ok(())
}
#[verifier::opaque]
pub(super) open spec fn fields(s:Seq<u8>,m:jm::Meta,l:Limits) -> bool {
    let v=numbers(s,m);let f=model(s,m);
    &&& numbers_ok(s,m) && f.valid() && f.w<=l.max_dimension && f.h<=l.max_dimension && f.output<=l.max_output && f.segments<=l.max_segments
    &&& f.w as int*f.h<=usize::MAX && f.across as int*((f.h as int-1)/(f.bh as int)+1)<=usize::MAX
    &&& (v[5]==1 || v[5]==2) && v[7]==1 && v[8]==1 && v[11]==1 && v[12]==1 && m.tags@[17].kind==0
    &&& one_sample(s,m.tags@[2],m.le) && one_sample(s,m.tags@[18],m.le)
    &&& v[20]==0 && (v[19]==0 || v[19]==1 || v[19]==4 || v[19]==5) && (f.compression==4 ==> v[19]==0)
    &&& (f.tiled ==> m.tags@[6].kind==0 && m.tags@[10].kind==0 && f.bw%16==0 && f.bh%16==0)
    &&& (!f.tiled ==> m.tags@[13].kind==0 && m.tags@[14].kind==0 && m.tags@[16].kind==0)
    &&& (f.offsets.kind==3 || f.offsets.kind==4) && (f.counts.kind==3 || f.counts.kind==4) && f.offsets.count==f.segments && f.counts.count==f.segments
}
pub(super) fn read(s:&[u8],m:jm::Meta,l:&Limits) -> (r:Result<Format,Error>)
    ensures r.is_ok()==fields(s@,m,*l),r.is_ok() ==> r.unwrap()==model(s@,m) && r.unwrap().valid(),
{
    reveal(fields);let v=read_numbers(s,m)?;let w=v[0];let h=v[1];let compression=v[3];let photo=v[4];
    if w==0 || h==0 || w>l.max_dimension || h>l.max_dimension || v[8]!=1 || (compression!=3 && compression!=4) || photo>1 {return Err(Error::Unsupported);}
    if (v[5]!=1 && v[5]!=2) || v[7]!=1 || v[11]!=1 || v[12]!=1 || m.tags[17].kind!=0 || v[20]!=0 ||
        (v[19]!=0 && v[19]!=1 && v[19]!=4 && v[19]!=5) || (compression==4 && v[19]!=0) {return Err(Error::Unsupported);}
    sample(s,m.tags[2],m.le)?;sample(s,m.tags[18],m.le)?;
    let tiled=m.tags[15].kind!=0;let bw=if tiled {v[13]} else {w};let bh=if tiled {v[14]} else if v[9]<h {v[9]} else {h};
    if bw==0 || bw>65536 || bh==0 {return Err(Error::Unsupported);}
    if tiled {if m.tags[6].kind!=0 || m.tags[10].kind!=0 || bw%16!=0 || bh%16!=0 {return Err(Error::Invalid);}}
    else if m.tags[13].kind!=0 || m.tags[14].kind!=0 || m.tags[16].kind!=0 {return Err(Error::Invalid);}
    let across=(w-1)/bw+1;let down=(h-1)/bh+1;let segments=mul(across,down)?;let output=mul(w,h)?;
    if output>l.max_output || segments>l.max_segments {return Err(Error::Limit);}
    let offsets=if tiled {m.tags[15]} else {m.tags[6]};let counts=if tiled {m.tags[16]} else {m.tags[10]};
    if (offsets.kind!=3 && offsets.kind!=4) || (counts.kind!=3 && counts.kind!=4) || offsets.count!=segments || counts.count!=segments {return Err(Error::Invalid);}
    Ok(Format {w,h,compression,photo,bw,bh,tiled,across,segments,output,lsb:v[5]==2,mixed:v[19]%2==1,aligned:v[19]>=4,le:m.le,offsets,counts})
}
}
