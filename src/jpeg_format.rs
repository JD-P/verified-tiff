// Supported TIFF/JPEG fields. Legacy compatibility is explicit and bounded.
use super::*;
use crate::jpeg_meta as jm;
verus! {
pub(super) open spec fn is_scalar(i:int) -> bool {i==0 || i==1 || i==3 || i==4 || i==5 || i==7 || i==8 || i==9 || i==11 || i==12 || i==13 || i==14 || i==20 || i==21 || i==22 || i==23 || i==29}
pub(super) open spec fn default(i:int) -> usize {if i==9 {4294967295usize} else if i==3 || i==5 || i==7 || i==8 || i==11 || i==12 || i==20 || i==29 {1usize} else {0usize}}
fn scalar_index(i:usize) -> (r:bool) ensures r==is_scalar(i as int), {i==0 || i==1 || i==3 || i==4 || i==5 || i==7 || i==8 || i==9 || i==11 || i==12 || i==13 || i==14 || i==20 || i==21 || i==22 || i==23 || i==29}
fn default_value(i:usize) -> (r:usize) ensures r==default(i as int), {if i==9 {4294967295} else if i==3 || i==5 || i==7 || i==8 || i==11 || i==12 || i==20 || i==29 {1} else {0}}
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
    requires n<=3,ensures r.is_ok()==uniform(s@,t,n as int,want as int,le,optional),
{
    if optional && t.kind==0 {return Ok(());}
    if (t.kind!=3 && t.kind!=4) || t.count!=n {return Err(Error::Unsupported);}
    let mut i=0usize;
    while i<n
        invariant i<=n,n<=3,t.count==n,t.kind==3 || t.kind==4,
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
pub(super) open spec fn rational_is(s:Seq<u8>,t:Tag,i:int,n:int,d:int,le:bool) -> bool {
    if t.kind==4 {value_valid(s,t,i) && tag_value(s,t,i,le)*d==n}
    else {t.kind==5 && 0<=i<t.count && range_ok(s.len() as int,t.data as int+8*i,8) &&
        word32(s,t.data as int+8*i+4,le)>0 && word32(s,t.data as int+8*i,le)*d==word32(s,t.data as int+8*i+4,le)*n}
}
fn product(a:u64,b:usize) -> (r:u64)
    requires a<=4294967295,b<=1000,ensures r as int==a as int*b as int,
{assert(a as int*b as int<=4294967295000) by(nonlinear_arith) requires a<=4294967295,b<=1000;a*(b as u64)}
fn equals(s:&[u8],t:Tag,i:usize,n:usize,d:usize,le:bool) -> (r:bool)
    requires i<6,n<=1000,1<=d<=1000,
    ensures r==rational_is(s@,t,i as int,n as int,d as int,le),
{
    let _len=s.len();
    if t.kind==4 {match value(s,t,i,le) {Ok(v)=>{assert(v<=4294967295);product(v as u64,d)==n as u64},Err(_)=>false}}
    else {
        if t.kind!=5 || i>=t.count {return false;}
        let p=match add(t.data,8*i) {Ok(p)=>p,Err(_)=>return false};
        if !checked_range(s.len(),p,8) {return false;}
        let a=match read32(s,p,le) {Ok(v)=>v as u64,Err(_)=>return false};
        let b=match read32(s,p+4,le) {Ok(v)=>v as u64,Err(_)=>return false};
        b>0 && product(a,d)==product(b,n)
    }
}
pub(super) open spec fn coefficients(s:Seq<u8>,t:Tag,le:bool) -> bool {
    t.kind==0 || (t.count==3 && rational_is(s,t,0,299,1000,le) && rational_is(s,t,1,587,1000,le) && rational_is(s,t,2,114,1000,le))
}
pub(super) open spec fn reference(s:Seq<u8>,t:Tag,le:bool,limited:bool) -> bool {
    if t.kind==0 {!limited} else {t.count==6 && rational_is(s,t,0,if limited {16int} else {0int},1,le) &&
        rational_is(s,t,1,if limited {235int} else {255int},1,le) && rational_is(s,t,2,128,1,le) && rational_is(s,t,3,if limited {240int} else {255int},1,le) &&
        rational_is(s,t,4,128,1,le) && rational_is(s,t,5,if limited {240int} else {255int},1,le)}
}
fn ref_match(s:&[u8],t:Tag,le:bool,limited:bool) -> (r:bool) ensures r==reference(s@,t,le,limited), {
    if t.kind==0 {!limited} else {t.count==6 && equals(s,t,0,if limited {16} else {0},1,le) && equals(s,t,1,if limited {235} else {255},1,le) &&
        equals(s,t,2,128,1,le) && equals(s,t,3,if limited {240} else {255},1,le) && equals(s,t,4,128,1,le) && equals(s,t,5,if limited {240} else {255},1,le)}
}
pub(super) open spec fn sub_model(s:Seq<u8>,m:jm::Meta,photo:int) -> Option<(usize,usize)> {
    let t=m.tags@[28];
    if photo!=6 {Some((1usize,1usize))} else if t.kind==0 {Some((2usize,2usize))}
    else if t.count==2 && value_valid(s,t,0) && value_valid(s,t,1) {Some((tag_value(s,t,0,m.le) as usize,tag_value(s,t,1,m.le) as usize))} else {None}
}
fn subsampling(s:&[u8],m:jm::Meta,photo:usize) -> (r:Result<(usize,usize),Error>)
    ensures r.is_ok()==sub_model(s@,m,photo as int).is_some(),r.is_ok() ==> r.unwrap()==sub_model(s@,m,photo as int).unwrap(),
{
    if photo!=6 {return Ok((1,1));}let t=m.tags[28];if t.kind==0 {return Ok((2,2));}
    if t.count!=2 {return Err(Error::Invalid);}Ok((value(s,t,0,m.le)?,value(s,t,1,m.le)?))
}
#[derive(Clone,Copy)]
pub(super) struct Format {
    pub w:usize,pub h:usize,pub ch:usize,pub compression:usize,pub photo:usize,pub bw:usize,pub bh:usize,
    pub tiled:bool,pub across:usize,pub segments:usize,pub output:usize,pub hs:usize,pub vs:usize,pub limited:bool,pub le:bool,
    pub offsets:Tag,pub counts:Tag,pub shared:Tag,pub q:Tag,pub dc:Tag,pub ac:Tag,pub interchange:usize,pub length:usize,pub restart:usize,
}
impl Format {
    pub(super) open spec fn valid(self) -> bool {
        self.w>0 && self.h>0 && (self.ch==1 || self.ch==3) && (self.compression==6 || self.compression==7) &&
        (self.photo==1 || self.photo==2 || self.photo==6) && (self.photo==1)==(self.ch==1) &&
        (!self.tiled ==> self.bw==self.w) && 1<=self.bw<=65535 && 1<=self.bh<=65535 && self.across==(self.w as int-1)/(self.bw as int)+1 &&
        self.segments==self.across as int*((self.h as int-1)/(self.bh as int)+1) && self.output==self.w as int*self.h*self.ch &&
        (self.hs==1 || self.hs==2 || self.hs==4) && (self.vs==1 || self.vs==2 || self.vs==4) && self.hs*self.vs<=4 && self.restart<=65535 &&
        (self.ch==1 ==> self.hs==1 && self.vs==1)
    }
}
pub(super) open spec fn model(s:Seq<u8>,m:jm::Meta) -> Format {
    let v=numbers(s,m);let tiled=m.tags@[15].kind!=0;
    let w=v[0];let h=v[1];let bw=if tiled {v[13]} else {w};let bh=if tiled {v[14]} else if v[9]<h {v[9]} else {h};
    let a=(w as int-1)/(bw as int)+1;let n=a*((h as int-1)/(bh as int)+1);let sub=sub_model(s,m,v[4] as int).unwrap();
    Format {w,h,ch:v[8],compression:v[3],photo:v[4],bw,bh,tiled,across:a as usize,segments:n as usize,output:(w as int*h*v[8]) as usize,
        hs:sub.0,vs:sub.1,limited:!reference(s,m.tags@[30],m.le,false),le:m.le,
        offsets:if tiled {m.tags@[15]} else {m.tags@[6]},counts:if tiled {m.tags@[16]} else {m.tags@[10]},shared:m.tags@[19],
        q:m.tags@[24],dc:m.tags@[25],ac:m.tags@[26],interchange:v[21],length:v[22],restart:v[23]}
}
#[verifier::opaque]
pub(super) open spec fn fields(s:Seq<u8>,m:jm::Meta,l:Limits) -> bool {
    let v=numbers(s,m);let f=model(s,m);
    &&& numbers_ok(s,m) && sub_model(s,m,v[4] as int).is_some()
    &&& f.valid() && f.w<=l.max_dimension && f.h<=l.max_dimension && f.output<=l.max_output && f.segments<=l.max_segments
    &&& f.w as int*f.h*f.ch<=usize::MAX && f.across as int*((f.h as int-1)/(f.bh as int)+1)<=usize::MAX
    &&& v[5]==1 && v[7]==1 && v[11]==1 && v[12]==1 && v[20]==1 && v[29]==1 && m.tags@[17].kind==0
    &&& uniform(s,m.tags@[2],f.ch as int,8,m.le,false) && uniform(s,m.tags@[18],f.ch as int,1,m.le,true)
    &&& coefficients(s,m.tags@[27],m.le) && (reference(s,m.tags@[30],m.le,false) || reference(s,m.tags@[30],m.le,true))
    &&& (f.tiled ==> m.tags@[6].kind==0 && m.tags@[10].kind==0 && f.bw%16==0 && f.bh%16==0)
    &&& (!f.tiled ==> m.tags@[13].kind==0 && m.tags@[14].kind==0 && m.tags@[16].kind==0)
    &&& (f.offsets.kind==3 || f.offsets.kind==4) && (f.counts.kind==3 || f.counts.kind==4) && f.offsets.count==f.segments && f.counts.count==f.segments
}
pub(super) fn read(s:&[u8],m:jm::Meta,l:&Limits) -> (r:Result<Format,Error>)
    ensures r.is_ok()==fields(s@,m,*l),r.is_ok() ==> r.unwrap()==model(s@,m) && r.unwrap().valid(),
{
    reveal(fields);
    let v=read_numbers(s,m)?;
    let w=v[0];let h=v[1];let ch=v[8];let compression=v[3];let photo=v[4];
    if w==0 || h==0 || w>l.max_dimension || h>l.max_dimension || (ch!=1 && ch!=3) || (compression!=6 && compression!=7) ||
        (photo!=1 && photo!=2 && photo!=6) || (photo==1)!=(ch==1) {return Err(Error::Unsupported);}
    if v[5]!=1 || v[7]!=1 || v[11]!=1 || v[12]!=1 || v[20]!=1 || v[29]!=1 || m.tags[17].kind!=0 || v[23]>65535 {return Err(Error::Unsupported);}
    all_values(s,m.tags[2],ch,8,m.le,false)?;all_values(s,m.tags[18],ch,1,m.le,true)?;
    let ct=m.tags[27];
    if ct.kind!=0 && (ct.count!=3 || !equals(s,ct,0,299,1000,m.le) || !equals(s,ct,1,587,1000,m.le) || !equals(s,ct,2,114,1000,m.le)) {return Err(Error::Unsupported);}
    let limited=!ref_match(s,m.tags[30],m.le,false);
    if limited && !ref_match(s,m.tags[30],m.le,true) {return Err(Error::Unsupported);}
    let (hs,vs)=subsampling(s,m,photo)?;
    if hs==0 || hs==3 || hs>4 || vs==0 || vs==3 || vs>4 {return Err(Error::Unsupported);}
    assert(hs as int*vs<=16) by(nonlinear_arith) requires hs<=4,vs<=4;
    if hs*vs>4 {return Err(Error::Unsupported);}
    let tiled=m.tags[15].kind!=0;
    let bw=if tiled {v[13]} else {w};let bh=if tiled {v[14]} else if v[9]<h {v[9]} else {h};
    if bw==0 || bw>65535 || bh==0 || bh>65535 {return Err(Error::Unsupported);}
    if tiled {if m.tags[6].kind!=0 || m.tags[10].kind!=0 || bw%16!=0 || bh%16!=0 {return Err(Error::Invalid);}}
    else if m.tags[13].kind!=0 || m.tags[14].kind!=0 || m.tags[16].kind!=0 {return Err(Error::Invalid);}
    let across=(w-1)/bw+1;let down=(h-1)/bh+1;let segments=mul(across,down)?;
    assert(w as int*h<=w as int*h*ch) by(nonlinear_arith) requires ch>=1;
    let output=mul(mul(w,h)?,ch)?;
    if output>l.max_output || segments>l.max_segments {return Err(Error::Limit);}
    let offsets=if tiled {m.tags[15]} else {m.tags[6]};let counts=if tiled {m.tags[16]} else {m.tags[10]};
    if (offsets.kind!=3 && offsets.kind!=4) || (counts.kind!=3 && counts.kind!=4) || offsets.count!=segments || counts.count!=segments {return Err(Error::Invalid);}
    Ok(Format {w,h,ch,compression,photo,bw,bh,tiled,across,segments,output,hs,vs,limited,le:m.le,offsets,counts,
        shared:m.tags[19],q:m.tags[24],dc:m.tags[25],ac:m.tags[26],interchange:v[21],length:v[22],restart:v[23]})
}
}
