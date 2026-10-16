// Bounded RFC 1950/1951 interpreter. The model uses unbounded byte strings;
// the executable refines them with a fixed 32 KiB circular history.
use super::*;
use crate::deflate_bits::*;
use crate::deflate_header::*;
verus! {

pub(super) open spec fn lb(c:int) -> int {
    seq![3int,4,5,6,7,8,9,10,11,13,15,17,19,23,27,31,35,43,51,59,67,83,99,115,131,163,195,227,258][c-257]
}
pub(super) open spec fn lext(c:int) -> int {if c<=264 || c==285 {0} else {(c-261)/4}}
pub(super) open spec fn db(c:int) -> int {
    seq![1int,2,3,4,5,7,9,13,17,25,33,49,65,97,129,193,257,385,513,769,1025,1537,2049,3073,4097,6145,8193,12289,16385,24577][c]
}
pub(super) open spec fn dext(c:int) -> int {if c<4 {0} else {c/2-1}}
fn length_info(c:usize) -> (r:(usize,usize))
    requires 257<=c<=285,
    ensures r.0==lb(c as int),r.1==lext(c as int),3<=r.0<=258,r.1<=5,r.0+pow(r.1 as int)-1<=258,
{
    let base=&[3usize,4,5,6,7,8,9,10,11,13,15,17,19,23,27,31,35,43,51,59,67,83,99,115,131,163,195,227,258];
    (base[c-257],if c<=264 || c==285 {0} else {(c-261)/4})
}
fn distance_info(c:usize) -> (r:(usize,usize))
    requires c<30,
    ensures r.0==db(c as int),r.1==dext(c as int),1<=r.0<=24577,r.1<=13,r.0+pow(r.1 as int)-1<=32768,
{
    let base=&[1usize,2,3,4,5,7,9,13,17,25,33,49,65,97,129,193,257,385,513,769,1025,1537,2049,3073,4097,6145,8193,12289,16385,24577];
    (base[c],if c<4 {0} else {c/2-1})
}
pub(super) open spec fn adler1(b:Seq<u8>) -> int decreases b.len() {
    if b.len()==0 {1} else {(adler1(b.drop_last())+b.last() as int)%65521}
}
pub(super) open spec fn adler2(b:Seq<u8>) -> int decreases b.len() {
    if b.len()==0 {0} else {(adler2(b.drop_last())+adler1(b))%65521}
}
pub(super) struct Model {
    pub pos:int,pub mode:u8,pub left:usize,pub distance:usize,pub last:bool,
    pub lit:HModel,pub dist:HModel,pub bytes:Seq<u8>,pub window:usize,
}
pub(super) open spec fn append(v:Model,b:u8) -> Model {Model {bytes:v.bytes.push(b),..v}}
pub(super) open spec fn advance(v:Model) -> Model {Model {mode:if v.last {4u8} else {0u8},..v}}
#[verifier::opaque]
pub(super) open spec fn transition(s:Seq<u8>,end:int,cap:int,v:Model) -> Option<Model> {
    if v.mode==0 {
        if v.pos+3>end*8 {None} else {
            let last=field(s,v.pos,1)!=0;let kind=field(s,v.pos+1,2);let q=v.pos+3;
            if kind==0 {
                let p=(q+7)/8*8;let n=field(s,p,16);let inv=field(s,p+16,16);
                if p+32>end*8 || n+inv!=65535 {None}
                else {Some(Model {pos:p+32,mode:1,left:n as usize,last,..v})}
            } else if kind==1 {
                if !tree_ok(fixed_lens(false),false) || !tree_ok(fixed_lens(true),false) {None}
                else {Some(Model {pos:q,mode:2,last,lit:huff(fixed_lens(false)),dist:huff(fixed_lens(true)),..v})}
            } else if kind==2 {
                let d=dynamic_model(s,q,end);
                if d.is_none() {None} else {let (p,lit,dist)=d.unwrap();Some(Model {pos:p,mode:2,last,lit,dist,..v})}
            } else {None}
        }
    } else if v.mode==1 {
        if v.left==0 {Some(advance(v))}
        else if v.pos+8>end*8 || v.bytes.len()>=cap {None}
        else {Some(append(Model {pos:v.pos+8,left:(v.left-1) as usize,..v},field(s,v.pos,8) as u8))}
    } else if v.mode==2 {
        let sym=symbol(s,v.pos,end,v.lit,1,0,0);
        if sym.is_none() {None} else {
            let (c,p)=sym.unwrap();
            if c<256 {if v.bytes.len()>=cap {None} else {Some(append(Model {pos:p,..v},c as u8))}}
            else if c==256 {Some(advance(Model {pos:p,..v}))}
            else if c>285 {None}
            else {
                let n=lb(c)+field(s,p,lext(c) as nat);let q=p+lext(c);
                let ds=symbol(s,q,end,v.dist,1,0,0);
                if q>end*8 || ds.is_none() {None} else {
                    let (d,r)=ds.unwrap();
                    if d>=30 {None} else {
                        let distance=db(d)+field(s,r,dext(d) as nat);
                        if r+dext(d)>end*8 || distance>v.bytes.len() || distance>v.window || v.bytes.len()+n>cap {None}
                        else {Some(Model {pos:r+dext(d),mode:3,left:n as usize,distance:distance as usize,..v})}
                    }
                }
            }
        }
    } else if v.mode==3 {
        if v.left==0 {Some(Model {mode:2,..v})}
        else if v.bytes.len()>=cap || v.distance==0 || v.distance>v.bytes.len() || v.distance>v.window {None}
        else {Some(append(Model {left:(v.left-1) as usize,..v},v.bytes[v.bytes.len() as int-v.distance]))}
    } else if v.mode==4 {
        if (v.pos+7)/8!=end || word32(s,end,false)!=adler2(v.bytes)*65536+adler1(v.bytes) {None}
        else {Some(Model {mode:5,..v})}
    } else {None}
}

pub(super) struct Cursor {
    pub bits:Bits,pub mode:u8,pub left:usize,pub distance:usize,pub last:bool,pub lit:Huff,pub dist:Huff,
    pub ring:[u8;32768],pub produced:usize,pub window:usize,pub a:usize,pub b:usize,pub pending:usize,
    pub history:Ghost<Seq<u8>>,
}
impl Cursor {
    pub(super) open spec fn model(&self) -> Model {
        Model {pos:self.bits.pos(),mode:self.mode,left:self.left,distance:self.distance,last:self.last,
            lit:self.lit.model(),dist:self.dist.model(),bytes:self.history@,window:self.window}
    }
    pub(super) open spec fn end(&self) -> int {self.bits.end as int}
    pub(super) open spec fn valid(&self,s:Seq<u8>,cap:int) -> bool {
        &&& self.bits.valid(s) && self.bits.end as int+4<=s.len()
        &&& self.mode<=5 && self.lit.valid() && self.dist.valid()
        &&& 1<=self.window<=32768 && self.left<=65535
        &&& self.history@.len()==self.produced && self.produced<=cap
        &&& self.a%65521==adler1(self.history@) && self.b%65521==adler2(self.history@)
            && self.pending<=256 && self.a<=65520+self.pending*255 && self.b<=65520+self.pending*130800
        &&& forall|k:int| 0<=k<self.produced && self.produced as int-32768<=k ==> self.ring@[k%32768]==self.history@[k]
    }
    #[inline(always)]
    pub(super) fn emit(&mut self,byte:u8,cap:usize) -> (r:Result<(),Error>)
        requires old(self).produced==old(self).history@.len(),old(self).produced<=cap,
            old(self).a%65521==adler1(old(self).history@),old(self).b%65521==adler2(old(self).history@),
            old(self).pending<=256,old(self).a<=65520+old(self).pending*255,old(self).b<=65520+old(self).pending*130800,
            forall|k:int| 0<=k<old(self).produced && old(self).produced as int-32768<=k ==> old(self).ring@[k%32768]==old(self).history@[k],
        ensures r.is_ok()==(old(self).produced<cap),
            r.is_ok() ==> final(self).produced==old(self).produced+1 && final(self).produced<=cap,
            r.is_ok() ==> final(self).model()==append(old(self).model(),byte),
            r.is_ok() ==> final(self).a%65521==adler1(final(self).history@) && final(self).b%65521==adler2(final(self).history@)
                && final(self).pending<=256 && final(self).a<=65520+final(self).pending*255 && final(self).b<=65520+final(self).pending*130800,
            r.is_ok() ==> forall|k:int| 0<=k<final(self).produced && final(self).produced as int-32768<=k ==> final(self).ring@[k%32768]==final(self).history@[k],
            final(self).bits==old(self).bits,final(self).lit==old(self).lit,final(self).dist==old(self).dist,
            r.is_ok() ==> final(self).history@.len()==final(self).produced,
    {
        if self.produced==cap {return Err(Error::Invalid);}
        let at=self.produced%32768;
        self.ring[at]=byte;
        // Accumulate bounded chunks; reduction is needed only every 256 bytes.
        if self.pending==256 {
            self.a=self.a%65521;self.b=self.b%65521;self.pending=0;
        }
        let ghost prior_a=self.a;let ghost prior_b=self.b;
        self.a=self.a+byte as usize;self.b=self.b+self.a;self.pending=self.pending+1;
        proof {
            assert(self.a%65521==(prior_a%65521+byte as int)%65521);
            assert(self.b%65521==(prior_b%65521+self.a%65521)%65521);
        }
        proof {self.history=Ghost(self.history@.push(byte));}
        self.produced=self.produced+1;
        proof {
            assert(self.history@==old(self).history@.push(byte));
            assert(self.history@.drop_last() =~= old(self).history@) by {
                assert forall|k:int| 0<=k<old(self).history@.len() implies self.history@.drop_last()[k]==old(self).history@[k] by {};
            }
            assert(self.history@.last()==byte);
            reveal_with_fuel(adler1,2);reveal_with_fuel(adler2,2);
        }
        assert forall|k:int| 0<=k<self.produced && self.produced as int-32768<=k implies self.ring@[k%32768]==self.history@[k] by {
            if k==old(self).produced {assert(k%32768==at);} else {
                assert(old(self).ring@[k%32768]==old(self).history@[k]);
                if k%32768==at {
                    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(k,32768);
                    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(old(self).produced as int,32768);
                    let n=old(self).produced as int;
                    assert(false) by(nonlinear_arith)
                        requires k%32768==n%32768,n-32768<k<n,
                            k==k/32768*32768+k%32768,n==n/32768*32768+n%32768;
                }
            }
        }
        Ok(())
    }
    pub(super) fn done(&self) -> (r:bool) ensures r==(self.model().mode==5) {self.mode==5}
    #[inline(always)]
    pub(super) fn match_byte(&mut self,s:&[u8],cap:usize) -> (byte:u8)
        requires old(self).valid(s@,cap as int),old(self).mode==3,old(self).left>0,
            0<old(self).distance<=old(self).produced,old(self).distance<=old(self).window,old(self).produced<cap,
        ensures final(self).valid(s@,cap as int),final(self).end()==old(self).end(),
            Some(final(self).model())==transition(s@,old(self).end(),cap as int,old(self).model()),
            final(self).produced==old(self).produced+1,final(self).history@==old(self).history@.push(byte),
            final(self).mode==3,final(self).left+1==old(self).left,final(self).distance==old(self).distance,
            final(self).bits==old(self).bits,final(self).window==old(self).window,
    {
        reveal(transition);
        let byte=self.ring[(self.produced-self.distance)%32768];
        assert(byte==self.history@[self.produced as int-self.distance]);
        self.left=self.left-1;
        let r=self.emit(byte,cap);assert(r.is_ok());byte
    }
    #[inline(always)]
    pub(super) fn step(&mut self,s:&[u8],cap:usize) -> (r:Result<Option<u8>,Error>)
        requires old(self).valid(s@,cap as int),old(self).mode<5,
        ensures r.is_ok()==transition(s@,old(self).end(),cap as int,old(self).model()).is_some(),
            r.is_ok() ==> final(self).valid(s@,cap as int),
            r.is_ok() ==> final(self).end()==old(self).end(),
            r.is_ok() ==> final(self).model()==transition(s@,old(self).end(),cap as int,old(self).model()).unwrap(),
            r.is_ok() ==> match r.unwrap() {
                Some(byte)=>final(self).produced==old(self).produced+1 && final(self).history@==old(self).history@.push(byte),
                None=>final(self).produced==old(self).produced && final(self).history@==old(self).history@,
            },
    {
        reveal(transition);
        if self.mode==0 {
            let last=self.bits.read(s,1)?!=0;let kind=self.bits.read(s,2)?;
            if kind==0 {
                self.bits.align();
                let n=self.bits.read(s,16)?;let inv=self.bits.read(s,16)?;
                if n+inv!=65535 {return Err(Error::Invalid);}
                self.left=n;self.mode=1;
            } else if kind==1 {
                self.lit=fixed(false)?;self.dist=fixed(true)?;self.mode=2;
            } else if kind==2 {
                let (lit,dist)=dynamic(s,&mut self.bits)?;
                self.lit=lit;self.dist=dist;self.mode=2;
            } else {return Err(Error::Invalid);}
            self.last=last;Ok(None)
        } else if self.mode==1 {
            if self.left==0 {self.mode=if self.last {4} else {0};Ok(None)}
            else {let v=self.bits.read(s,8)? as u8;self.left=self.left-1;self.emit(v,cap)?;Ok(Some(v))}
        } else if self.mode==2 {
            let c=self.lit.read(s,&mut self.bits)?;
            if c<256 {self.emit(c as u8,cap)?;Ok(Some(c as u8))}
            else if c==256 {self.mode=if self.last {4} else {0};Ok(None)}
            else {
                if c>285 {return Err(Error::Invalid);}
                let (base,nb)=length_info(c);
                let n=base+self.bits.read(s,nb)?;
                let d=self.dist.read(s,&mut self.bits)?;
                if d>=30 {return Err(Error::Invalid);}
                let (base,nb)=distance_info(d);
                let distance=base+self.bits.read(s,nb)?;
                if distance>self.produced || distance>self.window || n>cap-self.produced {return Err(Error::Invalid);}
                self.distance=distance;self.left=n;self.mode=3;Ok(None)
            }
        } else if self.mode==3 {
            if self.left==0 {self.mode=2;Ok(None)} else {
                if self.distance==0 || self.distance>self.produced || self.distance>self.window {return Err(Error::Invalid);}
                let at=(self.produced-self.distance)%32768;let byte=self.ring[at];
                assert(byte==self.history@[self.produced as int-self.distance]);
                self.left=self.left-1;self.emit(byte,cap)?;Ok(Some(byte))
            }
        } else {
            if (self.bits.at!=self.bits.end && self.bits.end-self.bits.at!=1)
                || (self.bits.at!=self.bits.end && self.bits.bit==0) {return Err(Error::Invalid);}
            let checksum=read32(s,self.bits.end,false)?;
            if checksum as usize!=(self.b%65521)*65536+self.a%65521 {return Err(Error::Invalid);}
            self.mode=5;Ok(None)
        }
    }
}

pub(super) open spec fn initial(s:Seq<u8>,start:int,count:int) -> Option<Model> {
    if !range_ok(s.len() as int,start,count) || count<6 {None} else {
        let cmf=s[start] as int;let flg=s[start+1] as int;
        if cmf%16!=8 || cmf/16>7 || (flg/32)%2!=0 || (cmf*256+flg)%31!=0 {None}
        else {Some(Model {pos:(start+2)*8,mode:0,left:0,distance:0,last:false,
            lit:huff(Seq::new(288,|i:int|0u8)),dist:huff(Seq::new(288,|i:int|0u8)),
            bytes:Seq::empty(),window:pow(cmf/16+8) as usize})}
    }
}
impl Cursor {
    pub(super) fn new(s:&[u8],start:usize,count:usize,cap:usize) -> (r:Result<Self,Error>)
        ensures r.is_ok()==initial(s@,start as int,count as int).is_some(),
            r.is_ok() ==> r.unwrap().valid(s@,cap as int),
            r.is_ok() ==> r.unwrap().model()==initial(s@,start as int,count as int).unwrap(),
            r.is_ok() ==> r.unwrap().end()==start as int+count-4 && r.unwrap().produced==0,
    {
        if !checked_range(s.len(),start,count) {return Err(Error::Truncated);}
        if count<6 {return Err(Error::Invalid);}
        let cmf=s[start] as usize;let flg=s[start+1] as usize;
        if cmf%16!=8 || cmf/16>7 || (flg/32)%2!=0 || (cmf*256+flg)%31!=0 {return Err(Error::Invalid);}
        let sizes=[256usize,512,1024,2048,4096,8192,16384,32768];
        let window=sizes[cmf/16];
        let lens=[0u8;288];
        assert(lens@==Seq::new(288,|i:int|0u8));
        let mut counts=[0usize;16];counts[0]=288;
        assert forall|i:int| 0<=i<16 implies counts@[i]==number(lens@,i,288) by {
            zero_number(i,288);
        }
        let blank=Huff {lens,counts,offsets:[0usize;16],symbols:[0u16;288]};
        assert(counts@==huff(lens@).counts);
        assert(blank.model()==huff(Seq::new(288,|i:int|0u8)));
        let history=Ghost(Seq::empty());
        Ok(Self {bits:Bits {at:start+2,bit:0,end:start+count-4},mode:0,left:0,distance:0,last:false,
            lit:blank,dist:blank,ring:[0u8;32768],produced:0,window,a:1,b:0,pending:0,history})
    }
}
proof fn zero_number(i:int,n:nat)
    requires n<=288,
    ensures number(Seq::new(288,|j:int|0u8),i,n)==if i==0 {n as int} else {0int},
    decreases n,
{ if n>0 {zero_number(i,(n-1) as nat);} }

#[verifier::opaque]
pub(super) open spec fn finish(s:Seq<u8>,end:int,cap:int,v:Model,fuel:nat) -> Option<Seq<u8>>
    decreases fuel,
{
    if v.mode==5 {Some(v.bytes)} else if fuel==0 {None} else {
        let next=transition(s,end,cap,v);
        if next.is_none() {None} else {finish(s,end,cap,next.unwrap(),(fuel-1) as nat)}
    }
}
#[verifier::opaque]
pub(super) open spec fn result(s:Seq<u8>,start:int,count:int,cap:int,fuel:nat) -> Option<Seq<u8>> {
    let init=initial(s,start,count);
    if init.is_none() {None} else {finish(s,start+count-4,cap,init.unwrap(),fuel)}
}
// Every transition either keeps history or appends exactly one specified byte.
proof fn transition_prefix(s:Seq<u8>,end:int,cap:int,v:Model)
    ensures transition(s,end,cap,v).is_some() ==> {
        let b=transition(s,end,cap,v).unwrap().bytes;
        b.len()>=v.bytes.len() && b.subrange(0,v.bytes.len() as int)==v.bytes
    },
{reveal(transition);}
pub(super) proof fn finish_prefix(s:Seq<u8>,end:int,cap:int,v:Model,fuel:nat)
    requires finish(s,end,cap,v,fuel).is_some(),
    ensures finish(s,end,cap,v,fuel).unwrap().len()>=v.bytes.len(),
        finish(s,end,cap,v,fuel).unwrap().subrange(0,v.bytes.len() as int)==v.bytes,
    decreases fuel,
{
    reveal_with_fuel(finish,1);
    if v.mode!=5 {
        let next=transition(s,end,cap,v).unwrap();
        transition_prefix(s,end,cap,v);
        finish_prefix(s,end,cap,next,(fuel-1) as nat);
        let b=finish(s,end,cap,next,(fuel-1) as nat).unwrap();
        assert(b.subrange(0,v.bytes.len() as int)==v.bytes) by {
            assert forall|i:int| 0<=i<v.bytes.len() implies b.subrange(0,v.bytes.len() as int)[i]==v.bytes[i] by {
                assert(b[i]==next.bytes[i]);
            }
        }
    }
}

pub(super) fn validate(s:&[u8],start:usize,count:usize,cap:usize,fuel:usize) -> (r:Result<usize,Error>)
    ensures r.is_ok()==result(s@,start as int,count as int,cap as int,fuel as nat).is_some(),
        r.is_ok() ==> r.unwrap()==result(s@,start as int,count as int,cap as int,fuel as nat).unwrap().len(),
        r.is_ok() ==> r.unwrap()<=cap,
{
    let mut output=[0u8;0];
    process(s,start,count,cap,fuel,&mut output,0)
}

// Validate and emit in the same traversal. The original validator uses an
// empty sink; TIFF's combined path supplies the visible strip buffer.
pub(super) fn process(s:&[u8],start:usize,count:usize,cap:usize,fuel:usize,output:&mut [u8],write:usize) -> (r:Result<usize,Error>)
    requires write<=old(output).len(),
    ensures r.is_ok()==result(s@,start as int,count as int,cap as int,fuel as nat).is_some(),
        r.is_ok() ==> r.unwrap()==result(s@,start as int,count as int,cap as int,fuel as nat).unwrap().len(),
        r.is_ok() ==> r.unwrap()<=cap,
        final(output).len()==old(output).len(),
        r.is_ok() ==> forall|k:int| 0<=k<write && k<r.unwrap() ==> final(output)@[k]==result(s@,start as int,count as int,cap as int,fuel as nat).unwrap()[k],
        forall|k:int| write<=k<old(output).len() ==> final(output)@[k]==old(output)@[k],
{
    reveal(result);
    let mut c=Cursor::new(s,start,count,cap)?;
    let mut left=fuel;
    while !c.done() && left>0
        invariant c.valid(s@,cap as int),left<=fuel,c.end()==start as int+count-4,
            result(s@,start as int,count as int,cap as int,fuel as nat)==finish(s@,c.end(),cap as int,c.model(),left as nat),
            output.len()==old(output).len(),write<=output.len(),
            forall|k:int| 0<=k<write && k<c.produced ==> output@[k]==c.history@[k],
            forall|k:int| write<=k<output.len() ==> output@[k]==old(output)@[k],
        decreases left,
    {
        reveal_with_fuel(finish,1);
        let pos=c.produced;let ghost previous=c.history@;
        let next=c.step(s,cap)?;left=left-1;
        if let Some(byte)=next {
            if pos<write {output[pos]=byte;}
            assert forall|k:int| 0<=k<write && k<c.produced implies output@[k]==c.history@[k] by {
                if k==pos {assert(c.history@[k]==byte);} else {assert(c.history@[k]==previous[k]);}
            }
        }
    }
    reveal_with_fuel(finish,1);
    if !c.done() {return Err(Error::Limit);}
    Ok(c.produced)
}

// An inhabited codec model: a complete zlib stream containing one stored byte.
proof fn stored_stream_witness()
    ensures result(seq![120u8,1,1,1,0,254,255,65,0,66,0,66],0,12,1,162)==Some(seq![65u8]),
{
    let s=seq![120u8,1,1,1,0,254,255,65,0,66,0,66];
    reveal(field);
    assert(field(s,16,1)==1) by(compute);
    assert(field(s,17,2)==0) by(compute);
    assert(field(s,24,16)==1) by(compute);
    assert(field(s,40,16)==65534) by(compute);
    assert(field(s,56,8)==65) by(compute);
    assert(adler1(seq![65u8])==66) by(compute);
    assert(adler2(seq![65u8])==66) by(compute);
    assert(initial(s,0,12).is_some());
    let v0=initial(s,0,12).unwrap();
    let v1=Model {pos:56,mode:1,left:1,last:true,..v0};
    let v2=Model {pos:64,left:0,bytes:seq![65u8],..v1};
    let v3=Model {mode:4,..v2};
    let v4=Model {mode:5,..v3};
    reveal(transition);
    assert(transition(s,8,1,v0)==Some(v1));
    assert(transition(s,8,1,v1)==Some(v2)) by {assert(Seq::<u8>::empty().push(65) =~= seq![65u8]);}
    assert(transition(s,8,1,v2)==Some(v3));
    assert(transition(s,8,1,v3)==Some(v4));
    reveal_with_fuel(finish,1);
    assert(finish(s,8,1,v4,158)==Some(seq![65u8]));
    assert(finish(s,8,1,v3,159)==Some(seq![65u8]));
    assert(finish(s,8,1,v2,160)==Some(seq![65u8]));
    assert(finish(s,8,1,v1,161)==Some(seq![65u8]));
    assert(finish(s,8,1,v0,162)==Some(seq![65u8]));
    reveal(result);
}

}
