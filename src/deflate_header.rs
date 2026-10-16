use super::*;
use crate::deflate_bits::*;
verus! {

pub(super) open spec fn order(i:int) -> int {
    if i==0 {16} else if i==1 {17} else if i==2 {18} else if i==3 {0}
    else if i==4 {8} else if i==5 {7} else if i==6 {9} else if i==7 {6}
    else if i==8 {10} else if i==9 {5} else if i==10 {11} else if i==11 {4}
    else if i==12 {12} else if i==13 {3} else if i==14 {13} else if i==15 {2}
    else if i==16 {14} else if i==17 {1} else {15}
}
fn code_order(i:usize) -> (r:usize)
    requires i<19,
    ensures r==order(i as int),r<19,
{
    let a=[16usize,17,18,0,8,7,9,6,10,5,11,4,12,3,13,2,14,1,15];a[i]
}
#[verifier::opaque]
pub(super) open spec fn code_lengths(s:Seq<u8>,p:int,end:int,n:nat,i:nat,l:Seq<u8>) -> Option<(int,Seq<u8>)>
    decreases n-i,
{
    if i>=n {Some((p,l))} else if p+3>end*8 {None}
    else {code_lengths(s,p+3,end,n,i+1,l.update(order(i as int),field(s,p,3) as u8))}
}
fn read_code_lengths(s:&[u8],bits:&mut Bits,n:usize) -> (r:Result<Huff,Error>)
    requires old(bits).valid(s@),4<=n<=19,
    ensures final(bits).valid(s@),final(bits).end==old(bits).end,final(bits).pos()>=old(bits).pos(),
        r.is_ok()==(code_lengths(s@,old(bits).pos(),old(bits).end as int,n as nat,0,Seq::new(288,|i:int|0u8)).is_some()
            && tree_ok(code_lengths(s@,old(bits).pos(),old(bits).end as int,n as nat,0,Seq::new(288,|i:int|0u8)).unwrap().1,true)),
        r.is_ok() ==> r.unwrap().valid(),
        r.is_ok() ==> final(bits).pos()==code_lengths(s@,old(bits).pos(),old(bits).end as int,n as nat,0,Seq::new(288,|i:int|0u8)).unwrap().0,
        r.is_ok() ==> r.unwrap().model()==huff(code_lengths(s@,old(bits).pos(),old(bits).end as int,n as nat,0,Seq::new(288,|i:int|0u8)).unwrap().1),
{
    let mut lens=[0u8;288];let mut i=0usize;
    assert(lens@==Seq::new(288,|i:int|0u8));
    let ghost model=code_lengths(s@,bits.pos(),bits.end as int,n as nat,0,lens@);
    while i<n
        invariant bits.valid(s@),bits.end==old(bits).end,bits.pos()>=old(bits).pos(),4<=n<=19,i<=n,
            model==code_lengths(s@,old(bits).pos(),old(bits).end as int,n as nat,0,Seq::new(288,|i:int|0u8)),
            model==code_lengths(s@,bits.pos(),bits.end as int,n as nat,i as nat,lens@),
        decreases n-i,
    {
        reveal_with_fuel(code_lengths,1);
        let v=bits.read(s,3)?;
        lens[code_order(i)]=v as u8;i=i+1;
    }
    reveal_with_fuel(code_lengths,1);
    Huff::build(lens,true)
}

#[verifier::opaque]
pub(super) open spec fn lengths(s:Seq<u8>,p:int,end:int,h:HModel,n:int,l:Seq<u8>) -> Option<(int,Seq<u8>)>
    decreases n-l.len(),
{
    if l.len()>=n {Some((p,l))} else {
        let sym=symbol(s,p,end,h,1,0,0);
        if sym.is_none() {None} else {
            let (c,q)=sym.unwrap();
            if c<=15 {lengths(s,q,end,h,n,l.push(c as u8))}
            else if c>18 || (c==16 && l.len()==0) {None}
            else {
                let nb=if c==16 {2nat} else if c==17 {3nat} else {7nat};
                let reps=field(s,q,nb)+if c==18 {11int} else {3int};
                let value=if c==16 {l[l.len() as int-1]} else {0u8};
                if q+nb>end*8 || reps<1 || l.len()+reps>n {None}
                else {lengths(s,q+nb,end,h,n,l+Seq::new(reps as nat,|i:int|value))}
            }
        }
    }
}
fn read_lengths(s:&[u8],bits:&mut Bits,h:Huff,n:usize) -> (r:Result<[u8;320],Error>)
    requires old(bits).valid(s@),h.valid(),n<=320,
    ensures final(bits).valid(s@),final(bits).end==old(bits).end,final(bits).pos()>=old(bits).pos(),
        r.is_ok()==lengths(s@,old(bits).pos(),old(bits).end as int,h.model(),n as int,Seq::empty()).is_some(),
        r.is_ok() ==> (final(bits).pos(),r.unwrap()@.subrange(0,n as int))==lengths(s@,old(bits).pos(),old(bits).end as int,h.model(),n as int,Seq::empty()).unwrap(),
        r.is_ok() ==> forall|i:int| 0<=i<320 ==> r.unwrap()@[i]<=15,
{
    let mut buf=[0u8;320];let mut i=0usize;
    assert(buf@.subrange(0,0)==Seq::<u8>::empty());
    let ghost model=lengths(s@,bits.pos(),bits.end as int,h.model(),n as int,Seq::empty());
    while i<n
        invariant i<=n<=320,bits.valid(s@),bits.end==old(bits).end,bits.pos()>=old(bits).pos(),h.valid(),
            forall|k:int| 0<=k<320 ==> buf@[k]<=15,
            model==lengths(s@,old(bits).pos(),old(bits).end as int,h.model(),n as int,Seq::empty()),
            model==lengths(s@,bits.pos(),bits.end as int,h.model(),n as int,buf@.subrange(0,i as int)),
        decreases n-i,
    {
        reveal_with_fuel(lengths,1);
        let ghost before=buf@.subrange(0,i as int);
        let c=h.read(s,bits)?;
        let (value,reps)=if c<=15 {(c as u8,1usize)} else {
            if c>18 || (c==16 && i==0) {return Err(Error::Invalid);}
            let value=if c==16 {buf[i-1]} else {0u8};
            let v=bits.read(s,if c==16 {2} else if c==17 {3} else {7})?;
            let reps=v+if c==18 {11} else {3};
            if reps>n-i {return Err(Error::Invalid);}
            (value,reps)
        };
        let ghost target=before+Seq::new(reps as nat,|k:int|value);
        if c<=15 {assert(target==before.push(c as u8));}
        assert(model==lengths(s@,bits.pos(),bits.end as int,h.model(),n as int,target));
        let mut j=0usize;
        while j<reps
            invariant i<n<=320,1<=reps<=n-i,j<=reps,value<=15,
                before.len()==i,target==before+Seq::new(reps as nat,|k:int|value),
                buf@.subrange(0,i as int)==before,
                forall|k:int| 0<=k<j ==> #[trigger] buf@[i as int+k]==value,
                forall|k:int| 0<=k<320 ==> buf@[k]<=15,
            decreases reps-j,
        {buf[i+j]=value;j=j+1;}
        i=i+reps;
        assert(buf@.subrange(0,i as int)==target) by {
            assert forall|k:int| 0<=k<i implies buf@.subrange(0,i as int)[k]==target[k] by {
                if k<before.len() {assert(buf@[k]==before[k]);}
                else {assert(buf@[before.len() as int+(k-before.len())]==value);}
            };
        }
    }
    reveal_with_fuel(lengths,1);
    Ok(buf)
}

pub(super) open spec fn fixed_lens(distance:bool) -> Seq<u8> {
    Seq::new(288,|i:int| if distance {if i<32 {5u8} else {0u8}}
        else if i<144 {8u8} else if i<256 {9u8} else if i<280 {7u8} else {8u8})
}
pub(super) fn fixed(distance:bool) -> (r:Result<Huff,Error>)
    ensures r.is_ok()==tree_ok(fixed_lens(distance),false),
        r.is_ok() ==> r.unwrap().valid() && r.unwrap().model()==huff(fixed_lens(distance)),
{
    let mut lens=[0u8;288];let mut i=0usize;
    while i<288
        invariant i<=288,forall|k:int| 0<=k<i ==> lens@[k]==fixed_lens(distance)[k],
        decreases 288-i,
    {lens[i]=if distance {if i<32 {5} else {0}} else if i<144 {8} else if i<256 {9} else if i<280 {7} else {8};i=i+1;}
    assert(lens@==fixed_lens(distance));
    Huff::build(lens,false)
}

pub(super) open spec fn padded(l:Seq<u8>,start:int,n:int) -> Seq<u8> {
    Seq::new(288,|i:int|if i<n {l[start+i]} else {0u8})
}
#[verifier::opaque]
pub(super) open spec fn dynamic_model(s:Seq<u8>,p:int,end:int) -> Option<(int,HModel,HModel)> {
    let nl=field(s,p,5)+257;let nd=field(s,p+5,5)+1;let nc=field(s,p+10,4)+4;
    if p+14>end*8 || nl>286 {None} else {
        let cl=code_lengths(s,p+14,end,nc as nat,0,Seq::new(288,|i:int|0u8));
        if cl.is_none() || !tree_ok(cl.unwrap().1,true) {None} else {
            let (q,codes)=cl.unwrap();
            let all=lengths(s,q,end,huff(codes),nl+nd,Seq::empty());
            if all.is_none() {None} else {
                let (r,ls)=all.unwrap();let lit=padded(ls,0,nl);let dist=padded(ls,nl,nd);
                if ls[256]==0 || !tree_ok(lit,false) || !tree_ok(dist,false) {None}
                else {Some((r,huff(lit),huff(dist)))}
            }
        }
    }
}
pub(super) fn dynamic(s:&[u8],bits:&mut Bits) -> (r:Result<(Huff,Huff),Error>)
    requires old(bits).valid(s@),
    ensures final(bits).valid(s@),final(bits).end==old(bits).end,final(bits).pos()>=old(bits).pos(),
        r.is_ok()==dynamic_model(s@,old(bits).pos(),old(bits).end as int).is_some(),
        r.is_ok() ==> r.unwrap().0.valid() && r.unwrap().1.valid(),
        r.is_ok() ==> (final(bits).pos(),r.unwrap().0.model(),r.unwrap().1.model())==dynamic_model(s@,old(bits).pos(),old(bits).end as int).unwrap(),
{
    reveal(dynamic_model);
    let nl=bits.read(s,5)?+257;let nd=bits.read(s,5)?+1;let nc=bits.read(s,4)?+4;
    if nl>286 {return Err(Error::Invalid);}
    let codes=read_code_lengths(s,bits,nc)?;
    let all=read_lengths(s,bits,codes,nl+nd)?;
    if all[256]==0 {return Err(Error::Invalid);}
    let mut lit=[0u8;288];let mut dist=[0u8;288];let mut i=0usize;
    while i<288
        invariant i<=288,257<=nl<=286,1<=nd<=32,nl+nd<=320,
            forall|k:int| 0<=k<i ==> lit@[k]==padded(all@.subrange(0,(nl+nd) as int),0,nl as int)[k],
            forall|k:int| 0<=k<i ==> dist@[k]==padded(all@.subrange(0,(nl+nd) as int),nl as int,nd as int)[k],
        decreases 288-i,
    {
        lit[i]=if i<nl {all[i]} else {0};
        dist[i]=if i<nd {all[nl+i]} else {0};
        i=i+1;
    }
    assert(lit@==padded(all@.subrange(0,(nl+nd) as int),0,nl as int));
    assert(dist@==padded(all@.subrange(0,(nl+nd) as int),nl as int,nd as int));
    Ok((Huff::build(lit,false)?,Huff::build(dist,false)?))
}
}
