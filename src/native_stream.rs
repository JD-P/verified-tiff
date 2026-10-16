//! Bounded resumable codec primitives. These refine the existing dictionary,
//! packet and Deflate transition models. The adapter supplies consecutive input
//! windows; input-window I/O and whole-stream orchestration are not proved here.
use super::*;
use crate::deflate::{Cursor,Model};
use crate::deflate_bits::Bits;
verus! {

// A batch is exactly a bounded sequence of the existing RFC1951 transitions.
// Nonfinal input windows yield before a header could run out of lookahead.
pub(super) open spec fn batch_stop(s:Seq<u8>,v:Model,space:nat,last:bool)->bool {
    v.mode==5 || space==0 || (!last && (v.mode==4 || v.pos/8+1024>s.len()))
}
#[verifier::opaque]
pub(super) open spec fn batch(s:Seq<u8>,end:int,cap:int,v:Model,space:nat,fuel:nat,last:bool)->Option<(Model,nat)>
    decreases fuel,
{
    if batch_stop(s,v,space,last){Some((v,fuel))}else if fuel==0{None}else{
        let next=crate::deflate::transition(s,end,cap,v);
        if next.is_none(){None}else{
            let n=next.unwrap();
            batch(s,end,cap,n,(space-(n.bytes.len()-v.bytes.len())) as nat,(fuel-1) as nat,last)
        }
    }
}

pub struct Inflater {cursor:Cursor,live:bool,cap:usize}
impl Inflater {
    pub closed spec fn valid(&self)->bool {
        self.live ==> self.cursor.valid(Seq::new((self.cursor.end()+4) as nat,|i:int|0u8),self.cap as int)
    }
    pub closed spec fn model(&self)->Option<Model>{if self.live{Some(self.cursor.model())}else{None}}
    pub closed spec fn initial(s:Seq<u8>)->Option<Model>{crate::deflate::initial(s,0,s.len() as int)}
    pub closed spec fn accepts(&self,s:Seq<u8>,at:usize,bit:usize,end:usize,work:usize)->bool {
        self.model().is_some() && self.model().unwrap().mode<5 &&
        at<=end && end as int+4<=s.len() && bit<8 && (at==end ==> bit==0) && work>0 &&
        crate::deflate::transition(s,end as int,self.cap as int,Model{pos:at as int*8+bit,..self.model().unwrap()}).is_some()
    }
    pub closed spec fn advanced(&self,prior:Self,s:Seq<u8>,at:usize,bit:usize,end:usize,r:(usize,usize,u8,u8))->bool {
        &&& self.cap==prior.cap
        &&& self.model()==crate::deflate::transition(s,end as int,prior.cap as int,Model{pos:at as int*8+bit,..prior.model().unwrap()})
        &&& r.0<=end && r.1<8 && self.model().unwrap().pos==r.0 as int*8+r.1
        &&& (r.2==1 ==> self.model().unwrap().bytes==prior.model().unwrap().bytes.push(r.3))
        &&& (r.2!=1 ==> self.model().unwrap().bytes==prior.model().unwrap().bytes)
        &&& (r.2==2)==(self.model().unwrap().mode==5)
    }
    pub fn new(s:&[u8],cap:usize)->(r:Result<Self,Error>)
        ensures r.is_ok()==Self::initial(s@).is_some(),
            r.is_ok() ==> r.unwrap().valid(),r.is_ok() ==> r.unwrap().model()==Self::initial(s@),
    {
        reveal(Inflater::initial);
        let cursor=Cursor::new(s,0,s.len(),cap)?;
        Ok(Self{cursor,live:true,cap})
    }
    /// Rebind the bounded input window and perform exactly one RFC1951
    /// transition. Returned tuple is (byte cursor, bit cursor, event, value),
    /// where event 0 is control, 1 emits a byte and 2 validates the checksum.
    pub fn step(&mut self,s:&[u8],at:usize,bit:usize,end:usize,budget:&mut Budget)->(r:Result<(usize,usize,u8,u8),Error>)
        requires old(self).valid(),
        ensures final(self).valid(),final(budget).remaining<=old(budget).remaining,
            r.is_ok()==old(self).accepts(s@,at,bit,end,old(budget).remaining),
            r.is_ok() ==> final(self).advanced(*old(self),s@,at,bit,end,r.unwrap()),
            r.is_ok() ==> final(budget).remaining+1==old(budget).remaining,
    {
        reveal(Inflater::accepts);reveal(Inflater::advanced);
        if !self.live{return Err(Error::Invalid);}
        self.live=false;
        let c=&mut self.cursor;
        if c.mode>=5||at>end||end>s.len()||s.len()-end<4||bit>=8||(at==end&&bit!=0){return Err(Error::Invalid);}
        spend(budget,1)?;
        c.bits=Bits{at,bit,end};
        assert(c.valid(s@,self.cap as int));
        let byte=c.step(s,self.cap)?;
        proof{if c.mode==5{assert(byte.is_none()) by{reveal(crate::deflate::transition);}}}
        let event=if c.mode==5{2u8}else if byte.is_some(){1u8}else{0u8};
        let value=if let Some(v)=byte{v}else{0};
        let result=(c.bits.at,c.bits.bit,event,value);
        self.live=true;Ok(result)
    }
    pub closed spec fn state_mode(&self)->u8{if self.model().is_some(){self.model().unwrap().mode}else{255}}
    pub fn mode(&self)->(r:u8) requires self.valid(),
        ensures r==self.state_mode(),
    {reveal(Inflater::state_mode);if self.live{self.cursor.mode}else{255}}

    pub closed spec fn batch_result(&self,s:Seq<u8>,at:usize,bit:usize,end:usize,space:usize,work:usize,last:bool)->Option<(Model,nat)> {
        if self.model().is_none() || at>end || end as int+4>s.len() || bit>=8 || (at==end && bit!=0){None}
        else{batch(s,end as int,self.cap as int,Model{pos:at as int*8+bit,..self.model().unwrap()},space as nat,(work/2) as nat,last)}
    }
    pub closed spec fn batch_output(&self,prior:Self,s:Seq<u8>,at:usize,bit:usize,end:usize,write:usize,work:usize,left:usize,last:bool,r:(usize,usize,usize,bool),out:Seq<u8>)->bool {
        let result=prior.batch_result(s,at,bit,end,write,work,last).unwrap();
        &&& self.cap==prior.cap
        &&& self.model()==Some(result.0)
        &&& left==2*result.1+work%2
        &&& r.0<=end && r.1<8
        &&& result.0.pos==r.0 as int*8+r.1
        &&& r.2<=write
        &&& result.0.bytes.len()==prior.model().unwrap().bytes.len()+r.2
        &&& r.3==(result.0.mode==5)
        &&& forall|k:int|0<=k<r.2 ==> out[k]==result.0.bytes[prior.model().unwrap().bytes.len()+k]
    }
    /// Rebind once, then decode a bounded run. Each original transition costs
    /// two units, retaining the adapter's former step-plus-loop work charge.
    /// The output prefix is private scratch until the adapter validates the
    /// whole segment. Returned values: cursor, bit, bytes written, checksum done.
    pub fn pull(&mut self,s:&[u8],at:usize,bit:usize,end:usize,last:bool,output:&mut[u8],write:usize,budget:&mut Budget)->(r:Result<(usize,usize,usize,bool),Error>)
        requires old(self).valid(),
        ensures final(self).valid(),
            final(output).len()==old(output).len(),final(budget).remaining<=old(budget).remaining,
            r.is_ok()==(write<=old(output).len() && old(self).batch_result(s@,at,bit,end,write,old(budget).remaining,last).is_some()),
            r.is_ok() ==> final(self).batch_output(*old(self),s@,at,bit,end,write,old(budget).remaining,final(budget).remaining,last,r.unwrap(),final(output)@),
            forall|k:int|write<=k<old(output).len() ==> final(output)@[k]==old(output)@[k],
    {
        reveal(Inflater::batch_result);reveal(Inflater::batch_output);
        if !self.live{return Err(Error::Invalid);}
        self.live=false;
        if write>output.len(){return Err(Error::OutputTooSmall);}
        if at>end||end>s.len()||s.len()-end<4||bit>=8||(at==end&&bit!=0){return Err(Error::Invalid);}
        let c=&mut self.cursor;c.bits=Bits{at,bit,end};
        assert(c.valid(s@,self.cap as int));
        let base=c.produced;let mut written=0usize;let mut left=budget.remaining/2;
        let ghost wanted=batch(s@,end as int,self.cap as int,c.model(),write as nat,left as nat,last);
        while !c.done() && written<write && (last || (c.mode!=4 && s.len()-c.bits.at>=1024)) && left>0
            invariant !self.live,self.cap==old(self).cap,old(self).model().is_some(),
                c.valid(s@,self.cap as int),c.end()==end,
                base==old(self).model().unwrap().bytes.len(),c.produced==base+written,
                written<=write,write<=output.len(),output.len()==old(output).len(),
                budget.remaining==2*left+old(budget).remaining%2,budget.remaining<=old(budget).remaining,
                wanted==old(self).batch_result(s@,at,bit,end,write,old(budget).remaining,last),
                wanted==batch(s@,end as int,self.cap as int,c.model(),(write-written) as nat,left as nat,last),
                forall|k:int|0<=k<written ==> output@[k]==c.history@[base+k],
                forall|k:int|write<=k<output.len() ==> output@[k]==old(output)@[k],
            decreases left,
        {
            if c.mode==3 && c.left>0 && c.distance>0 && c.distance<=c.produced && c.distance<=c.window && c.left<=self.cap-c.produced {
                let n=c.left.min(write-written).min(left);let mut i=0usize;
                let at_before=c.bits.at;let count_before=c.left;let prior_left=left;
                while i<n
                    invariant i<=n,n>0,n<=count_before,n<=left+i,n<=write-written+i,left+i==prior_left,
                        c.mode==3,c.left+i==count_before,c.bits.at==at_before,
                        0<c.distance<=c.produced,c.distance<=c.window,c.left<=self.cap-c.produced,
                        last || (c.mode!=4 && s.len()-c.bits.at>=1024),
                        c.valid(s@,self.cap as int),c.end()==end,
                        c.produced==base+written,written<=write,write<=output.len(),output.len()==old(output).len(),
                        budget.remaining==2*left+old(budget).remaining%2,budget.remaining<=old(budget).remaining,
                        wanted==batch(s@,end as int,self.cap as int,c.model(),(write-written) as nat,left as nat,last),
                        forall|k:int|0<=k<written ==> output@[k]==c.history@[base+k],
                        forall|k:int|write<=k<output.len() ==> output@[k]==old(output)@[k],
                    decreases n-i,
                {
                    reveal_with_fuel(batch,1);spend(budget,2)?;
                    let ghost previous=c.history@;
                    let byte=c.match_byte(s,self.cap);
                    output[written]=byte;written=written+1;left=left-1;i=i+1;
                    assert forall|k:int|0<=k<written implies output@[k]==c.history@[base+k] by {
                        if k==written-1 {assert(c.history@[base+k]==byte);}
                        else {assert(c.history@[base+k]==previous[base+k]);}
                    }
                }
                assert(left<prior_left);
                continue;
            }
            reveal_with_fuel(batch,1);
            spend(budget,2)?;
            let ghost previous=c.history@;
            let next=c.step(s,self.cap)?;left=left-1;
            if let Some(byte)=next {
                output[written]=byte;written=written+1;
                assert forall|k:int|0<=k<written implies output@[k]==c.history@[base+k] by {
                    if k==written-1 {assert(c.history@[base+k]==byte);}
                    else {assert(c.history@[base+k]==previous[base+k]);}
                }
            }
        }
        reveal_with_fuel(batch,1);
        if !c.done() && written<write && (last || (c.mode!=4 && s.len()-c.bits.at>=1024)) {return Err(Error::Limit);}
        let result=(c.bits.at,c.bits.bit,written,c.mode==5);
        self.live=true;Ok(result)
    }
}

pub closed spec fn initial_dict()->Seq<Seq<u8>>{lz_initial()}
pub closed spec fn code_width(d:Seq<Seq<u8>>,legacy:bool)->int{lz_width(d.len() as int,legacy)}
pub closed spec fn code_valid(d:Seq<Seq<u8>>,prev:int,c:int,legacy:bool)->bool{lz_data_ok(d,prev,c,legacy)}
pub closed spec fn phrase(d:Seq<Seq<u8>>,prev:int,c:int)->Seq<u8>{lz_phrase(d,prev,c)}
pub closed spec fn after(d:Seq<Seq<u8>>,prev:int,c:int)->Seq<Seq<u8>>{lz_after(d,prev,c)}
pub closed spec fn code_bits(s:Seq<u8>,pos:int,width:nat,legacy:bool)->int{lz_wire_bits(s,pos,width,legacy)}
pub struct Lzw {table:LzTable,legacy:bool,first:bool}
impl Lzw {
    pub closed spec fn valid(&self)->bool {self.table.valid()}
    pub closed spec fn dict(&self)->Seq<Seq<u8>> {self.table._dict@}
    pub closed spec fn previous(&self)->int{self.table.prev as int}
    pub closed spec fn first(&self)->bool{self.first}
    pub closed spec fn legacy(&self)->bool{self.legacy}
    pub fn new(legacy:bool)->(r:Self)
        ensures r.valid(),r.dict()==initial_dict(),r.previous()==256,r.first(),r.legacy()==legacy,
    {reveal(initial_dict);Self{table:LzTable::new(),legacy,first:true}}
    #[inline(always)]
    pub fn is_legacy(&self)->(r:bool) ensures r==self.legacy(), {self.legacy}
    #[inline(always)]
    pub fn width(&self)->(r:usize) requires self.valid(),
        ensures r==code_width(self.dict(),self.legacy()),9<=r<=12,
    {reveal(code_width);self.table.width(self.legacy)}
    #[inline(always)]
    pub fn code(&mut self,c:usize,out:&mut[u8;4096])->(r:Result<usize,Error>)
        requires old(self).valid(),
        ensures final(self).valid(),final(self).legacy()==old(self).legacy(),
            r.is_ok()==(c<4096 && (!old(self).first()||c==256) &&
                (c==256||c==257||code_valid(old(self).dict(),old(self).previous(),c as int,old(self).legacy()))),
            r.is_ok() ==> !final(self).first(),
            r.is_ok() ==> (c==256 ==> r.unwrap()==0 && final(self).dict()==initial_dict() && final(self).previous()==256),
            r.is_ok() ==> (c==257 ==> r.unwrap()==0 && final(self).dict()==old(self).dict() && final(self).previous()==old(self).previous()),
            r.is_ok() && c!=256 && c!=257 ==> {
                &&& r.unwrap()==phrase(old(self).dict(),old(self).previous(),c as int).len()
                &&& 1<=r.unwrap()<=4096
                &&& final(self).dict()==after(old(self).dict(),old(self).previous(),c as int)
                &&& final(self).previous()==c
                &&& forall|k:int|0<=k<r.unwrap() ==> final(out)@[k]==phrase(old(self).dict(),old(self).previous(),c as int)[k]
            },
    {
        reveal(code_valid);reveal(phrase);reveal(after);reveal(initial_dict);
        if c>=4096||(self.first&&c!=256){return Err(Error::Invalid);}
        self.first=false;
        if c==256{self.table.clear();Ok(0)}
        else if c==257{Ok(0)}
        else{self.table.data(c,self.legacy)?;Ok(self.table.expand(c,out))}
    }
}
#[inline(always)]
pub fn lzw_code(s:&[u8],at:usize,bit:usize,width:usize,legacy:bool)->(r:Result<(usize,usize,usize),Error>)
    ensures r.is_ok()==(at<=s.len()&&bit<8&&(at==s.len() ==> bit==0)&&1<=width<=12&&at as int*8+bit+width<=s.len() as int*8),
        r.is_ok() ==> r.unwrap().0<4096 && r.unwrap().1<=s.len() && r.unwrap().2<8,
        r.is_ok() ==> r.unwrap().1 as int*8+r.unwrap().2==at as int*8+bit+width,
        r.is_ok() ==> r.unwrap().0==code_bits(s@,at as int*8+bit,width as nat,legacy),
{
    reveal(code_bits);
    if at>s.len()||bit>=8||(at==s.len()&&bit!=0)||width==0||width>12{return Err(Error::Invalid);}
    let mut bits=LzBits{at,bit,end:s.len(),legacy};
    let c=bits.read(s,width)?;Ok((c,bits.at,bits.bit))
}
/// One complete PackBits packet; no payload copies or input ownership.
pub closed spec fn packet_contract(s:Seq<u8>,at:usize,stride:usize,produced:usize,needed:usize,success:bool,r:(usize,usize,usize,bool))->bool {
    &&& (success==(stride>0&&produced<=needed&&pb_packet_ok(s,at as int,s.len() as int)&&
        pb_len(s,at as int)<=needed-produced&&pb_len(s,at as int)<=stride-(produced as int)%(stride as int)))
    &&& (success ==> r.0>at && r.0<=s.len() && r.2<=128 && produced as int+r.2<=needed)
    &&& (success ==> r.2==pb_len(s,at as int))
    &&& (success ==> r.1==at+1 && (r.3 ==> r.2==0 || r.1<s.len()))
    &&& (success ==> (!r.3 ==> r.1 as int+r.2<=s.len()))
    &&& (success ==> r.3==(s[at as int]>128))
}
pub fn packet(s:&[u8],at:usize,stride:usize,produced:usize,needed:usize)->(r:Result<(usize,usize,usize,bool),Error>)
    ensures packet_contract(s@,at,stride,produced,needed,r.is_ok(),if r.is_ok(){r.unwrap()}else{(0usize,0usize,0usize,false)}),
{
    reveal(packet_contract);
    if stride==0||produced>needed{return Err(Error::Invalid);}
    let p=pb_packet(s,at,s.len())?;
    if p.len>needed-produced||p.len>stride-produced%stride{return Err(Error::Invalid);}
    Ok((p.next,p.data,p.len,p.repeat))
}
}
