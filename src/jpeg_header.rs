// Bounded marker/table parsing. Tables are borrowed offsets, never input-sized allocations.
use super::*;
use crate::jpeg_huffman as h;
verus! {
// Slots 0..4 quantization; 4..12 DC/AC Huffman; 12..17 frame geometry;
// 17..20 component IDs; 20..23 quant selectors; 23..29 scan selectors;
// 29 restart interval, 30 frame present, 31 terminal marker (SOS=1, EOI=2).
pub(super) open spec fn quant_ok(s:Seq<u8>,p:int,end:int) -> bool {
    0<=p && p+64<=end<=s.len() && forall|i:int| 0<=i<64 ==> #[trigger] s[p+i]>0
}
pub(super) fn quant(s:&[u8],p:usize,end:usize) -> (r:Result<(),Error>)
    requires end<=s.len(),ensures r.is_ok()==quant_ok(s@,p as int,end as int),
{
    if !checked_range(end,p,64) {return Err(Error::Truncated);}
    let mut i=0usize;
    while i<64
        invariant i<=64,p as int+64<=end<=s.len(),forall|j:int| 0<=j<i ==> #[trigger] s@[p+j]>0,
        decreases 64-i,
    {if s[p+i]==0 {return Err(Error::Invalid);}i=i+1;}
    Ok(())
}
pub(super) open spec fn tables_ok(s:Seq<u8>,v:Seq<usize>) -> bool {
    v.len()==32 &&
    (forall|i:int| 0<=i<4 && v[i]!=0 ==> #[trigger] quant_ok(s,v[i] as int,s.len() as int)) &&
    (forall|i:int| 4<=i<12 && v[i]!=0 ==> #[trigger] h::table_ok(s,v[i] as int,s.len() as int))
}
pub(super) open spec fn frame_ok(v:Seq<usize>) -> bool {
    &&& v.len()==32
    &&& 1<=v[12]<=65535 && 1<=v[13]<=65535 && (v[14]==1 || v[14]==3)
    &&& 1<=v[15]<=4 && 1<=v[16]<=4 && v[15]*v[16]<=4
    &&& v[14]==1 ==> v[15]==1 && v[16]==1
    &&& v[17]<=255 && v[20]<4 && v[23]<4 && v[26]<4 && v[18]<=255 && v[21]<4 && v[24]<4 && v[27]<4 && v[19]<=255 && v[22]<4 && v[25]<4 && v[28]<4
    &&& v[14]==3 ==> v[17]!=v[18] && v[17]!=v[19] && v[18]!=v[19]
    &&& v[30]==1
}
pub(super) open spec fn scan_ok(s:Seq<u8>,v:Seq<usize>) -> bool {
    tables_ok(s,v) && frame_ok(v) && v[v[20] as int]!=0 && v[4+v[23]]!=0 && v[8+v[26]]!=0 &&
    (v[14]==3 ==> v[v[21] as int]!=0 && v[4+v[24]]!=0 && v[8+v[27]]!=0 && v[v[22] as int]!=0 && v[4+v[25]]!=0 && v[8+v[28]]!=0)
}
#[verifier::opaque]
pub(super) open spec fn frame(s:Seq<u8>,p:int,e:int,v:Seq<usize>) -> Option<Seq<usize>> {
    if p+6>e || s[p]!=8 || !(s[p+5]==1 || s[p+5]==3) || e!=p+6+3*s[p+5] {None} else {
        let ch=s[p+5] as usize;
        let w=word16(s,p+3,false) as usize;let ht=word16(s,p+1,false) as usize;
        let hs=if ch==1 {1usize} else {(s[p+7]/16) as usize};let vs=if ch==1 {1usize} else {(s[p+7]%16) as usize};
        let u=v.update(12,w).update(13,ht).update(14,ch).update(15,hs).update(16,vs).update(30,1);
        let a=u.update(17,s[p+6] as usize).update(20,s[p+8] as usize);
        let b=if ch==3 {a.update(18,s[p+9] as usize).update(21,s[p+11] as usize).update(19,s[p+12] as usize).update(22,s[p+14] as usize)} else {a};
        if s[p+7]/16==0 || s[p+7]/16>4 || s[p+7]%16==0 || s[p+7]%16>4 || !frame_ok(b) || (ch==3 && (s[p+10]!=17 || s[p+13]!=17)) {None} else {Some(b)}
    }
}
fn read_frame(s:&[u8],p:usize,e:usize,v:[usize;32]) -> (r:Result<[usize;32],Error>)
    requires p<=e<=s.len(),v@[23]<4,v@[24]<4,v@[25]<4,v@[26]<4,v@[27]<4,v@[28]<4,
        v@[18]<=255,v@[19]<=255,v@[21]<4,v@[22]<4,
    ensures r.is_ok()==frame(s@,p as int,e as int,v@).is_some(),
        r.is_ok() ==> r.unwrap()@==frame(s@,p as int,e as int,v@).unwrap() && frame_ok(r.unwrap()@),
{
    reveal(frame);
    if e-p<6 || s[p]!=8 || (s[p+5]!=1 && s[p+5]!=3) {return Err(Error::Unsupported);}
    let ch=s[p+5] as usize;if e-p!=6+3*ch {return Err(Error::Invalid);}
    let mut u=v;
    u[12]=read16(s,p+3,false)? as usize;u[13]=read16(s,p+1,false)? as usize;
    u[14]=ch;u[15]=if ch==1 {1} else {(s[p+7]/16) as usize};u[16]=if ch==1 {1} else {(s[p+7]%16) as usize};u[30]=1;
    u[17]=s[p+6] as usize;u[20]=s[p+8] as usize;
    if ch==3 {u[18]=s[p+9] as usize;u[21]=s[p+11] as usize;u[19]=s[p+12] as usize;u[22]=s[p+14] as usize;}
    assert(u@[15]<=15 && u@[16]<=15);
    assert(u@[15]*u@[16]<=225) by(nonlinear_arith) requires u@[15]<=15,u@[16]<=15;
    if s[p+7]/16==0 || s[p+7]/16>4 || s[p+7]%16==0 || s[p+7]%16>4 || u[12]==0 || u[13]==0 || u[15]==0 || u[15]>4 || u[16]==0 || u[16]>4 || u[15]*u[16]>4 ||
        (ch==1 && (u[15]!=1 || u[16]!=1)) || u[20]>3 || u[21]>3 || u[22]>3 ||
        (ch==3 && (u[17]==u[18] || u[17]==u[19] || u[18]==u[19] || s[p+10]!=17 || s[p+13]!=17)) {return Err(Error::Unsupported);}
    Ok(u)
}
#[verifier::opaque]
pub(super) open spec fn scan(s:Seq<u8>,p:int,e:int,v:Seq<usize>) -> Option<Seq<usize>> {
    if !frame_ok(v) || p+1>e || s[p]!=v[14] || e!=p+4+2*v[14] || s[e-3]!=0 || s[e-2]!=63 || s[e-1]!=0 {None}
    else if s[p+1]!=v[17] || (v[14]==3 && (s[p+3]!=v[18] || s[p+5]!=v[19])) {None}
    else {
        let a=v.update(23,(s[p+2]/16) as usize).update(26,(s[p+2]%16) as usize).update(31,1);
        let b=if v[14]==3 {a.update(24,(s[p+4]/16) as usize).update(27,(s[p+4]%16) as usize).update(25,(s[p+6]/16) as usize).update(28,(s[p+6]%16) as usize)} else {a};
        if scan_ok(s,b) {Some(b)} else {None}
    }
}
fn read_scan(s:&[u8],p:usize,e:usize,v:[usize;32]) -> (r:Result<[usize;32],Error>)
    requires p<=e<=s.len(),tables_ok(s@,v@),frame_ok(v@),
    ensures r.is_ok()==scan(s@,p as int,e as int,v@).is_some(),
        r.is_ok() ==> r.unwrap()@==scan(s@,p as int,e as int,v@).unwrap() && scan_ok(s@,r.unwrap()@),
{
    reveal(scan);
    if e-p!=4+2*v[14] || s[p] as usize!=v[14] || s[e-3]!=0 || s[e-2]!=63 || s[e-1]!=0 {return Err(Error::Unsupported);}
    if s[p+1] as usize!=v[17] || (v[14]==3 && (s[p+3] as usize!=v[18] || s[p+5] as usize!=v[19])) {return Err(Error::Invalid);}
    let mut u=v;u[23]=(s[p+2]/16) as usize;u[26]=(s[p+2]%16) as usize;u[31]=1;
    if v[14]==3 {u[24]=(s[p+4]/16) as usize;u[27]=(s[p+4]%16) as usize;u[25]=(s[p+6]/16) as usize;u[28]=(s[p+6]%16) as usize;}
    if u[23]>3 || u[24]>3 || u[25]>3 || u[26]>3 || u[27]>3 || u[28]>3 {return Err(Error::Unsupported);}
    if u[u[20]]==0 || u[4+u[23]]==0 || u[8+u[26]]==0 ||
        (u[14]==3 && (u[u[21]]==0 || u[4+u[24]]==0 || u[8+u[27]]==0 ||
                     u[u[22]]==0 || u[4+u[25]]==0 || u[8+u[28]]==0)) {return Err(Error::Invalid);}
    Ok(u)
}
pub(super) open spec fn shape(v:Seq<usize>) -> bool {
    v.len()==32 && v[29]<=65535 && v[30]<=1 && v[31]<=2 &&
    (v[17]<=255 && v[20]<4 && v[23]<4 && v[26]<4 && v[18]<=255 && v[21]<4 && v[24]<4 && v[27]<4 && v[19]<=255 && v[22]<4 && v[25]<4 && v[28]<4) &&
    (v[30]==1 ==> frame_ok(v))
}
pub(super) struct Model {pub v:Seq<usize>,pub at:usize,pub end:usize,pub sub:usize,pub kind:usize}
#[derive(Clone,Copy)]
pub(super) struct Header {pub v:[usize;32],pub at:usize,pub end:usize,pub sub:usize,pub kind:usize}
impl Header {
    pub(super) open spec fn model(self) -> Model {Model {v:self.v@,at:self.at,end:self.end,sub:self.sub,kind:self.kind}}
    pub(super) open spec fn valid(self,s:Seq<u8>) -> bool {
        self.at<=self.end<=s.len() && self.kind<=2 && (self.kind!=0 ==> self.at<=self.sub<=self.end) &&
        shape(self.v@) && tables_ok(s,self.v@) && (self.v@[31]==1 ==> scan_ok(s,self.v@))
    }
}
#[verifier::opaque]
pub(super) open spec fn step(s:Seq<u8>,m:Model) -> Option<Model> {
    let p=m.at as int;let end=m.end as int;let v=m.v;
    if m.kind==1 {
        if p==m.sub {Some(Model {kind:0,..m})}
        else if p+65>m.sub || s[p]>3 || !quant_ok(s,p+1,m.sub as int) {None}
        else {Some(Model {at:(p+65) as usize,v:v.update(s[p] as int,(p+1) as usize),..m})}
    } else if m.kind==2 {
        if p==m.sub {Some(Model {kind:0,..m})}
        else if p>=m.sub || s[p]/16>1 || s[p]%16>3 || !h::table_ok(s,p+1,m.sub as int) {None}
        else {Some(Model {at:(p+17+h::count(s,p+1,16)) as usize,v:v.update(4+4*(s[p] as int/16)+s[p] as int%16,(p+1) as usize),..m})}
    } else if p+2>end || s[p]!=255 {None}
    else if s[p+1]==255 {Some(Model {at:(p+1) as usize,..m})}
    else if s[p+1]==217 {
        if p+2!=end {None} else {Some(Model {at:m.end,v:v.update(31,2),..m})}
    } else if p+4>end || word16(s,p+2,false)<2 || p+2+word16(s,p+2,false)>end {None}
    else {
        let code=s[p+1];let a=p+4;let e=p+2+word16(s,p+2,false);
        if code==219 || code==196 {
            if a==e {None} else {Some(Model {at:a as usize,sub:e as usize,kind:if code==219 {1usize} else {2usize},..m})}
        } else if code==192 || code==193 {
            let f=frame(s,a,e,v);
            if v[30]!=0 || f.is_none() {None} else {Some(Model {at:e as usize,v:f.unwrap(),..m})}
        } else if code==218 {
            let f=scan(s,a,e,v);
            if f.is_none() {None} else {Some(Model {at:e as usize,v:f.unwrap(),..m})}
        } else if code==221 {
            if e!=a+2 {None} else {Some(Model {at:e as usize,v:v.update(29,word16(s,a,false) as usize),..m})}
        } else if (224<=code<=239) || code==254 {Some(Model {at:e as usize,..m})}
        else {None}
    }
}
impl Header {
    pub(super) fn advance(self,s:&[u8]) -> (r:Result<Self,Error>)
        requires self.valid(s@),
        ensures r.is_ok()==step(s@,self.model()).is_some(),
            r.is_ok() ==> r.unwrap().valid(s@) && r.unwrap().model()==step(s@,self.model()).unwrap(),
            r.is_ok() ==> r.unwrap().end==self.end,
    {
        reveal(step);
        let mut u=self;let p=self.at;
        if self.kind==1 {
            if p==self.sub {u.kind=0;return Ok(u);}
            if self.sub-p<65 || s[p]>3 {return Err(Error::Invalid);}
            quant(s,p+1,self.sub)?;
            u.v[s[p] as usize]=p+1;u.at=p+65;
        } else if self.kind==2 {
            if p==self.sub {u.kind=0;return Ok(u);}
            if s[p]/16>1 || s[p]%16>3 {return Err(Error::Invalid);}
            let (_,end)=h::table(s,p+1,self.sub)?;
            let id=4+4*((s[p]/16) as usize)+(s[p]%16) as usize;
            u.v[id]=p+1;u.at=end;
        } else {
            if self.end-p<2 || s[p]!=255 {return Err(Error::Invalid);}
            let code=s[p+1];
            if code==255 {u.at=p+1;return Ok(u);}
            if code==217 {
                if self.end-p!=2 {return Err(Error::Invalid);}
                u.at=self.end;u.v[31]=2;return Ok(u);
            }
            if self.end-p<4 {return Err(Error::Truncated);}
            let len=read16(s,p+2,false)? as usize;
            if len<2 || len>self.end-(p+2) {return Err(Error::Truncated);}
            let a=p+4;let e=p+2+len;
            if code==219 || code==196 {
                if a==e {return Err(Error::Invalid);}
                u.at=a;u.sub=e;u.kind=if code==219 {1} else {2};
            } else if code==192 || code==193 {
                if self.v[30]!=0 {return Err(Error::Invalid);}
                u.v=read_frame(s,a,e,self.v)?;u.at=e;
            } else if code==218 {
                if self.v[30]!=1 {reveal(scan);return Err(Error::Invalid);}
                u.v=read_scan(s,a,e,self.v)?;u.at=e;
            } else if code==221 {
                if e-a!=2 {return Err(Error::Invalid);}
                u.v[29]=read16(s,a,false)? as usize;u.at=e;
            } else if (224<=code && code<=239) || code==254 {u.at=e;}
            else {return Err(Error::Unsupported);}
        }
        reveal(frame);reveal(scan);
        assert(u.valid(s@));
        assert(u.model()==step(s@,self.model()).unwrap());
        Ok(u)
    }
}
#[verifier::opaque]
pub(super) open spec fn finish(s:Seq<u8>,m:Model,n:nat) -> Option<Model> decreases n {
    if m.v[31]!=0 || (m.at==m.end && m.kind==0) {Some(m)} else if n==0 {None} else {
        let t=step(s,m);if t.is_none() {None} else {finish(s,t.unwrap(),(n-1) as nat)}
    }
}
pub(super) fn parse(s:&[u8],start:Header,fuel:usize) -> (r:Result<Header,Error>)
    requires start.valid(s@),
    ensures r.is_ok()==finish(s@,start.model(),fuel as nat).is_some(),
        r.is_ok() ==> r.unwrap().valid(s@) && r.unwrap().model()==finish(s@,start.model(),fuel as nat).unwrap(),
        r.is_ok() ==> r.unwrap().end==start.end,
{
    let mut h=start;let mut n=fuel;let ghost result=finish(s@,start.model(),fuel as nat);
    while h.v[31]==0 && (h.at!=h.end || h.kind!=0)
        invariant h.valid(s@),n<=fuel,h.end==start.end,
            result==finish(s@,start.model(),fuel as nat),result==finish(s@,h.model(),n as nat),
        decreases n,
    {
        reveal_with_fuel(finish,1);
        if n==0 {return Err(Error::Limit);}
        h=h.advance(s)?;n=n-1;
    }
    reveal_with_fuel(finish,1);Ok(h)
}
}
