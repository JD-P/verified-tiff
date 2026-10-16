// T.4/T.6 run endpoints. Duplicate endpoints remain significant for 2D lines.
use super::*;
use crate::fax_bits as fb;
use crate::fax_bits::Bits;
verus! {
pub(super) open spec fn mode_value(code:usize,n:usize) -> i8 {
    if n==1 && code==1 {0} else if n==3 && code==3 {1} else if n==3 && code==2 {(-1) as i8}
    else if n==6 && code==3 {2} else if n==6 && code==2 {(-2) as i8}
    else if n==7 && code==3 {3} else if n==7 && code==2 {(-3) as i8}
    else if n==4 && code==1 {8} else if n==3 && code==1 {9} else {127}
}
fn value(code:usize,n:usize) -> (r:i8) ensures r==mode_value(code,n), {
    if n==1 && code==1 {0} else if n==3 && code==3 {1} else if n==3 && code==2 {(-1) as i8}
    else if n==6 && code==3 {2} else if n==6 && code==2 {(-2) as i8}
    else if n==7 && code==3 {3} else if n==7 && code==2 {(-3) as i8}
    else if n==4 && code==1 {8} else if n==3 && code==1 {9} else {127}
}
#[verifier::opaque]
pub(super) open spec fn mode(s:Seq<u8>,b:Bits,code:usize,n:usize,fuel:nat) -> Option<(i8,Bits)> decreases fuel {
    if fuel==0 {None} else {let a=fb::one(s,b);if a.is_none() {None} else {
        let (v,q)=a.unwrap();let next=(2*code+v) as usize;let size=(n+1) as usize;let k=mode_value(next,size);
        if k!=127 {Some((k,q))} else {mode(s,q,next,size,(fuel-1) as nat)}
    }}
}
fn read_mode(s:&[u8],b:Bits) -> (r:Result<(i8,Bits),Error>)
    requires b.valid(s@),
    ensures r.is_ok()==mode(s@,b,0,0,7).is_some(),r.is_ok() ==> r.unwrap()==mode(s@,b,0,0,7).unwrap(),
        r.is_ok() ==> r.unwrap().1.valid(s@) && r.unwrap().1.end==b.end && r.unwrap().1.lsb==b.lsb && b.pos<r.unwrap().1.pos && -3<=r.unwrap().0<=9,
{
    let mut code=0usize;let mut n=0usize;let mut q=b;let ghost result=mode(s@,b,0,0,7);
    while n<7
        invariant n<=7,code<crate::deflate_bits::pow(n as int),q.valid(s@),q.end==b.end,q.lsb==b.lsb,b.pos<=q.pos,
            result==mode(s@,b,0,0,7),result==mode(s@,q,code,n,(7-n) as nat),
        decreases 7-n,
    {
        reveal_with_fuel(mode,1);let (v,p)=fb::bit(s,q)?;
        proof {crate::deflate_bits::pow_bound(n as int);}
        code=2*code+v;n=n+1;let k=value(code,n);if k!=127 {return Ok((k,p));}q=p;
    }
    reveal_with_fuel(mode,1);Err(Error::Invalid)
}
pub(super) open spec fn ref_at(reference:Seq<u32>,i:usize,w:usize) -> usize {
    if i==0 {0} else if i<=reference.len() {reference[i as int-1] as usize} else {w}
}
fn reference_at(reference:&[u32;65538],len:usize,i:usize,w:usize) -> (r:usize)
    requires len<=65538,
    ensures r==ref_at(reference@.subrange(0,len as int),i,w),
{
    if i==0 {0} else if i<=len {reference[i-1] as usize} else {w}
}
#[verifier::opaque]
pub(super) open spec fn seek(reference:Seq<u32>,i:usize,a:usize,w:usize,fuel:nat) -> Option<usize> decreases fuel {
    if fuel==0 {None} else if ref_at(reference,i,w)>a || ref_at(reference,i,w)>=w {Some(i)}
    else if i>65538 {None} else {seek(reference,(i+2) as usize,a,w,(fuel-1) as nat)}
}
fn find_reference(reference:&[u32;65538],len:usize,start:usize,a:usize,w:usize) -> (r:Result<usize,Error>)
    requires len<=65538,start<=65540,
    ensures r.is_ok()==seek(reference@.subrange(0,len as int),start,a,w,65541).is_some(),
        r.is_ok() ==> r.unwrap()==seek(reference@.subrange(0,len as int),start,a,w,65541).unwrap() && r.unwrap()<=65540,
{
    let ghost refs=reference@.subrange(0,len as int);let ghost result=seek(refs,start,a,w,65541);
    let mut i=start;let mut fuel=65541usize;
    while fuel>0
        invariant i<=65540,fuel<=65541,len<=65538,refs==reference@.subrange(0,len as int),
            result==seek(refs,start,a,w,65541),result==seek(refs,i,a,w,fuel as nat),
        decreases fuel,
    {
        reveal_with_fuel(seek,1);let b=reference_at(reference,len,i,w);
        if b>a || b>=w {return Ok(i);}if i>65538 {return Err(Error::Invalid);}
        i=i+2;fuel=fuel-1;
    }
    reveal_with_fuel(seek,1);Err(Error::Invalid)
}
pub(super) struct Model {pub bits:Bits,pub a:usize,pub bi:usize,pub ends:Seq<u32>}
#[derive(Clone,Copy)]
pub(super) struct State {pub bits:Bits,pub a:usize,pub bi:usize,pub n:usize}
impl State {
    pub(super) open spec fn model(self,ends:Seq<u32>) -> Model {Model {bits:self.bits,a:self.a,bi:self.bi,ends:ends.subrange(0,self.n as int)}}
    pub(super) open spec fn valid(self,s:Seq<u8>,ends:Seq<u32>,w:usize) -> bool {
        self.bits.valid(s) && self.a<=w<=65536 && self.bi<=65540 && self.n<=65538 && ends.len()==65538 &&
        (self.n>65536 ==> ends[self.n as int-1]==self.a) &&
        (forall|i:int| 0<=i<self.n ==> #[trigger] ends[i]<=self.a) &&
        (forall|i:int,j:int| 0<=i<=j<self.n ==> #[trigger] ends[i]<=#[trigger] ends[j])
    }
}
pub(super) open spec fn begin(b:Bits) -> Model {Model {bits:b,a:0,bi:1,ends:Seq::empty()}}
pub(super) open spec fn normalized(ends:Seq<u32>,a:usize,w:usize) -> Seq<u32> {
    if a<w && ends.len()>=2 && ends.len()%2==0 && ends.last()==ends[ends.len() as int-2] &&
        (ends.len()==2 && a==0 || ends.len()>2 && ends.last()==ends[ends.len() as int-3])
    {ends.subrange(0,ends.len() as int-2)} else {ends}
}
#[verifier::opaque]
pub(super) open spec fn step(s:Seq<u8>,reference:Seq<u32>,w:usize,td:bool,m:Model) -> Option<Model> {
    let black=m.ends.len()%2==1;
    if m.a>=w || m.ends.len()>65536 {None}
    else if !td {
        let r=fb::run(s,m.bits,black,(w-m.a) as usize,0,1026);
        if r.is_none() {None} else {let (n,q)=r.unwrap();let a=(m.a+n) as usize;
            Some(Model {bits:q,a,bi:m.bi,ends:normalized(m.ends.push(a as u32),a,w)})}
    } else {
        let r=mode(s,m.bits,0,0,7);if r.is_none() {None} else {let (kind,q)=r.unwrap();
            if kind==9 {
                let r1=fb::run(s,q,black,(w-m.a) as usize,0,1026);
                if r1.is_none() {None} else {let (n1,q1)=r1.unwrap();let a1=(m.a+n1) as usize;
                    let r2=fb::run(s,q1,!black,(w-a1) as usize,0,1026);
                    if r2.is_none() {None} else {let (n2,q2)=r2.unwrap();let a2=(a1+n2) as usize;
                        let bi=seek(reference,m.bi,a2,w,65541);
                        if bi.is_none() {None} else {Some(Model {bits:q2,a:a2,bi:bi.unwrap(),ends:m.ends.push(a1 as u32).push(a2 as u32)})}
                    }
                }
            } else {
                let b=if m.ends.len()==0 {Some(m.bi)} else {seek(reference,m.bi,m.a,w,65541)};
                if b.is_none() {None} else {let bi=b.unwrap();let v=ref_at(reference,bi,w);
                    let to=if kind==8 {ref_at(reference,(bi+1) as usize,w) as int} else {v as int+kind as int};
                    if to<m.a || to>w || (kind<0 && bi==0) || (kind>=0 && bi>65538) {None}
                    else {Some(Model {bits:q,a:to as usize,
                        bi:if kind==8 {(bi+2) as usize} else if kind<0 {(bi-1) as usize} else {(bi+1) as usize},
                        ends:if kind==8 {m.ends} else {m.ends.push(to as u32)}})}
                }
            }
        }
    }
}
pub(super) fn advance(s:&[u8],reference:&[u32;65538],rn:usize,w:usize,td:bool,st:State,ends:&mut [u32;65538]) -> (r:Result<State,Error>)
    requires st.valid(s@,old(ends)@,w),rn<=65538,
    ensures r.is_ok()==step(s@,reference@.subrange(0,rn as int),w,td,st.model(old(ends)@)).is_some(),
        r.is_ok() ==> r.unwrap().model(final(ends)@)==step(s@,reference@.subrange(0,rn as int),w,td,st.model(old(ends)@)).unwrap(),
        r.is_ok() ==> r.unwrap().valid(s@,final(ends)@,w) && st.bits.pos<r.unwrap().bits.pos && r.unwrap().bits.end==st.bits.end && r.unwrap().bits.lsb==st.bits.lsb,
{
    reveal(step);let black=st.n%2==1;
    if st.a>=w || st.n>65536 {return Err(Error::Invalid);}
    if !td {
        let (n,q)=fb::length(s,st.bits,black,w-st.a)?;let a=st.a+n;
        let ghost before=ends@.subrange(0,st.n as int);
        ends[st.n]=a as u32;let mut count=st.n+1;
        assert(ends@.subrange(0,count as int) =~= before.push(a as u32));
        if a<w && count>=2 && count%2==0 && ends[count-1]==ends[count-2] &&
            ((count==2 && a==0) || (count>2 && ends[count-1]==ends[count-3])) {count=count-2;}
        assert(ends@.subrange(0,count as int) =~= normalized(before.push(a as u32),a,w));
        return Ok(State {bits:q,a,bi:st.bi,n:count});
    }
    let (kind,q)=read_mode(s,st.bits)?;
    if kind==9 {
        let (n1,q1)=fb::length(s,q,black,w-st.a)?;let a1=st.a+n1;
        let (n2,q2)=fb::length(s,q1,!black,w-a1)?;let a2=a1+n2;
        let bi=find_reference(reference,rn,st.bi,a2,w)?;
        let ghost before=ends@.subrange(0,st.n as int);
        ends[st.n]=a1 as u32;ends[st.n+1]=a2 as u32;
        assert(ends@.subrange(0,st.n as int+2) =~= before.push(a1 as u32).push(a2 as u32));
        Ok(State {bits:q2,a:a2,bi,n:st.n+2})
    } else {
        let bi=if st.n==0 {st.bi} else {find_reference(reference,rn,st.bi,st.a,w)?};
        let v=reference_at(reference,rn,bi,w);
        let to=if kind==8 {reference_at(reference,rn,bi+1,w) as i64} else {(v as i64)+(kind as i64)};
        if to<(st.a as i64) || to>(w as i64) || (kind<0 && bi==0) || (kind>=0 && bi>65538) {return Err(Error::Invalid);}
        let next=if kind==8 {bi+2} else if kind<0 {bi-1} else {bi+1};
        let ghost before=ends@.subrange(0,st.n as int);
        let count=if kind==8 {st.n} else {ends[st.n]=to as u32;st.n+1};
        assert(ends@.subrange(0,count as int) =~= if kind==8 {before} else {before.push(to as u32)});
        Ok(State {bits:q,a:to as usize,bi:next,n:count})
    }
}
#[verifier::opaque]
pub(super) open spec fn finish(s:Seq<u8>,reference:Seq<u32>,w:usize,td:bool,m:Model,fuel:nat) -> Option<Model> decreases fuel {
    if m.a==w {
        if m.ends.len()==0 || m.ends.last()<w {Some(Model {ends:m.ends.push(w as u32),..m})} else {Some(m)}
    } else if fuel==0 {None} else {let a=step(s,reference,w,td,m);if a.is_none() {None} else {finish(s,reference,w,td,a.unwrap(),(fuel-1) as nat)}}
}
pub(super) fn row(s:&[u8],reference:&[u32;65538],rn:usize,w:usize,td:bool,b:Bits,ends:&mut [u32;65538],fuel:usize) -> (r:Result<State,Error>)
    requires b.valid(s@),0<w<=65536,rn<=65538,
    ensures r.is_ok()==finish(s@,reference@.subrange(0,rn as int),w,td,begin(b),fuel as nat).is_some(),
        r.is_ok() ==> r.unwrap().model(final(ends)@)==finish(s@,reference@.subrange(0,rn as int),w,td,begin(b),fuel as nat).unwrap(),
        r.is_ok() ==> r.unwrap().valid(s@,final(ends)@,w) && r.unwrap().a==w && r.unwrap().bits.end==b.end && r.unwrap().bits.lsb==b.lsb && 0<r.unwrap().n<=65538 && final(ends)@[r.unwrap().n as int-1]==w,
{
    let mut st=State {bits:b,a:0,bi:1,n:0};let mut left=fuel;
    let ghost refs=reference@.subrange(0,rn as int);let ghost result=finish(s@,refs,w,td,begin(b),fuel as nat);
    assert(ends@.subrange(0,0) =~= Seq::<u32>::empty());assert(st.model(ends@)==begin(b));
    while st.a<w
        invariant st.valid(s@,ends@,w),st.bits.end==b.end,st.bits.lsb==b.lsb,0<w<=65536,rn<=65538,left<=fuel,
            refs==reference@.subrange(0,rn as int),
            result==finish(s@,refs,w,td,begin(b),fuel as nat),result==finish(s@,refs,w,td,st.model(ends@),left as nat),
        decreases left,
    {
        reveal_with_fuel(finish,1);
        if left==0 {return Err(Error::Invalid);}
        st=advance(s,reference,rn,w,td,st,ends)?;left=left-1;
    }
    reveal_with_fuel(finish,1);
    if st.n==0 || ends[st.n-1]<(w as u32) {
        // A pass ending the row has a pending run and at most 65,536 endpoints.
        if st.n==65538 {assert(false);}
        let ghost before=ends@.subrange(0,st.n as int);
        ends[st.n]=w as u32;st.n=st.n+1;
        assert(ends@.subrange(0,st.n as int) =~= before.push(w as u32));
    }
    Ok(st)
}
}
