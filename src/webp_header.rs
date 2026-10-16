use super::*;
use crate::deflate_bits::{self as db,Bits};
use crate::webp_huffman::*;
verus! {

pub(super) open spec fn order(i:int) -> int {
    seq![17int,18,0,1,2,3,4,5,16,6,7,8,9,10,11,12,13,14,15][i]
}
fn code_order(i:usize) -> (r:usize)
    requires i<19,ensures r==order(i as int),r<19,
{let a=[17usize,18,0,1,2,3,4,5,16,6,7,8,9,10,11,12,13,14,15];a[i]}
#[verifier::opaque]
pub(super) open spec fn code_lengths(s:Seq<u8>,p:int,end:int,n:nat,i:nat,l:Seq<u8>) -> Option<(int,Seq<u8>)>
    decreases n-i,
{
    if i>=n {Some((p,l))} else if p+3>end*8 {None}
    else {code_lengths(s,p+3,end,n,i+1,l.update(order(i as int),db::field(s,p,3) as u8))}
}
fn read_code_lengths(s:&[u8],bits:&mut Bits,n:usize) -> (r:Result<Huff,Error>)
    requires old(bits).valid(s@),4<=n<=19,
    ensures final(bits).valid(s@),final(bits).end==old(bits).end,final(bits).pos()>=old(bits).pos(),
        r.is_ok()==(code_lengths(s@,old(bits).pos(),old(bits).end as int,n as nat,0,Seq::new(2328,|i:int|0u8)).is_some()
            && tree_ok(code_lengths(s@,old(bits).pos(),old(bits).end as int,n as nat,0,Seq::new(2328,|i:int|0u8)).unwrap().1,false)),
        r.is_ok() ==> r.unwrap().valid(),
        r.is_ok() ==> final(bits).pos()==code_lengths(s@,old(bits).pos(),old(bits).end as int,n as nat,0,Seq::new(2328,|i:int|0u8)).unwrap().0,
        r.is_ok() ==> r.unwrap().model()==huff(code_lengths(s@,old(bits).pos(),old(bits).end as int,n as nat,0,Seq::new(2328,|i:int|0u8)).unwrap().1),
{
    let mut lens=[0u8;2328];let mut i=0usize;
    assert(lens@==Seq::new(2328,|i:int|0u8));
    let ghost model=code_lengths(s@,bits.pos(),bits.end as int,n as nat,0,lens@);
    while i<n
        invariant bits.valid(s@),bits.end==old(bits).end,bits.pos()>=old(bits).pos(),4<=n<=19,i<=n,
            model==code_lengths(s@,old(bits).pos(),old(bits).end as int,n as nat,0,Seq::new(2328,|i:int|0u8)),
            model==code_lengths(s@,bits.pos(),bits.end as int,n as nat,i as nat,lens@),
        decreases n-i,
    {
        reveal_with_fuel(code_lengths,1);
        let v=bits.read(s,3)?;
        lens[code_order(i)]=v as u8;i=i+1;
    }
    reveal_with_fuel(code_lengths,1);
    Huff::build(lens,false)
}


// Each repetition consumes one of max_symbol code-length tokens, even when
// it fills many alphabet entries. Zero literals do not reset the last nonzero length.
#[verifier::opaque]
pub(super) open spec fn lengths(s:Seq<u8>,p:int,end:int,h:HModel,n:int,left:nat,prev:u8,l:Seq<u8>) -> Option<(int,Seq<u8>)>
    decreases left,
{
    if l.len()>=n || left==0 {Some((p,l+Seq::new((2328-l.len()) as nat,|i:int|0u8)))} else {
        let sym=decode(s,p,end,h);
        if sym.is_none() {None} else {
            let (c,q)=sym.unwrap();
            if c<=15 {lengths(s,q,end,h,n,(left-1) as nat,if c!=0 {c as u8} else {prev},l.push(c as u8))}
            else if c>18 {None} else {
                let nb=if c==16 {2nat} else if c==17 {3nat} else {7nat};
                let reps=db::field(s,q,nb)+if c==18 {11int} else {3int};
                let value=if c==16 {prev} else {0u8};
                if q+nb>end*8 || reps<1 || l.len()+reps>n {None}
                else {lengths(s,q+nb,end,h,n,(left-1) as nat,prev,l+Seq::new(reps as nat,|i:int|value))}
            }
        }
    }
}
fn read_lengths(s:&[u8],bits:&mut Bits,h:Huff,n:usize,left:usize) -> (r:Result<[u8;2328],Error>)
    requires old(bits).valid(s@),h.valid(),1<=n<=2328,left<=n,
    ensures final(bits).valid(s@),final(bits).end==old(bits).end,final(bits).pos()>=old(bits).pos(),
        r.is_ok()==lengths(s@,old(bits).pos(),old(bits).end as int,h.model(),n as int,left as nat,8,Seq::empty()).is_some(),
        r.is_ok() ==> (final(bits).pos(),r.unwrap()@)==lengths(s@,old(bits).pos(),old(bits).end as int,h.model(),n as int,left as nat,8,Seq::empty()).unwrap(),
{
    let mut buf=[0u8;2328];let mut i=0usize;let mut remain=left;let mut prev=8u8;
    assert(buf@.subrange(0,0)==Seq::<u8>::empty());
    let ghost expected=lengths(s@,bits.pos(),bits.end as int,h.model(),n as int,left as nat,8,Seq::empty());
    while i<n && remain>0
        invariant i<=n<=2328,remain<=left<=n,prev<=15,bits.valid(s@),bits.end==old(bits).end,bits.pos()>=old(bits).pos(),h.valid(),
            forall|k:int| 0<=k<2328 ==> buf@[k]<=15,
            forall|k:int| i<=k<2328 ==> buf@[k]==0,
            expected==lengths(s@,old(bits).pos(),old(bits).end as int,h.model(),n as int,left as nat,8,Seq::empty()),
            expected==lengths(s@,bits.pos(),bits.end as int,h.model(),n as int,remain as nat,prev,buf@.subrange(0,i as int)),
        decreases remain,
    {
        reveal_with_fuel(lengths,1);let ghost before=buf@.subrange(0,i as int);
        let c=h.read(s,bits)?;
        let (value,reps)=if c<=15 {if c!=0 {prev=c as u8;}(c as u8,1usize)} else {
            if c>18 {return Err(Error::Invalid);}
            let value=if c==16 {prev} else {0u8};
            let v=bits.read(s,if c==16 {2} else if c==17 {3} else {7})?;
            let reps=v+if c==18 {11} else {3};
            if reps>n-i {return Err(Error::Invalid);}(value,reps)
        };
        remain=remain-1;
        let ghost target=before+Seq::new(reps as nat,|k:int|value);
        if c<=15 {assert(target==before.push(c as u8));}
        assert(expected==lengths(s@,bits.pos(),bits.end as int,h.model(),n as int,remain as nat,prev,target));
        let mut j=0usize;
        while j<reps
            invariant i<n<=2328,1<=reps<=n-i,j<=reps,value<=15,
                before.len()==i,target==before+Seq::new(reps as nat,|k:int|value),buf@.subrange(0,i as int)==before,
                forall|k:int| 0<=k<j ==> #[trigger] buf@[i as int+k]==value,
                forall|k:int| 0<=k<2328 ==> buf@[k]<=15,
                forall|k:int| i+j<=k<2328 ==> buf@[k]==0,
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
    assert(buf@ =~= buf@.subrange(0,i as int)+Seq::new((2328-i) as nat,|k:int|0u8));
    Ok(buf)
}
#[verifier::opaque]
pub(super) open spec fn table(s:Seq<u8>,p:int,end:int,n:int) -> Option<(int,HModel)> {
    if p+1>end*8 {None} else if db::field(s,p,1)==1 {
        let two=db::field(s,p+1,1)==1;let nb=if db::field(s,p+2,1)==0 {1nat} else {8nat};
        let a=db::field(s,p+3,nb);let q=p+3+nb;let b=db::field(s,q,8);let r=q+if two {8int} else {0int};
        let lens=Seq::new(2328,|i:int|if i==a || (two && i==b) {1u8} else {0u8});
        if p+3>end*8 || r>end*8 || a>=n || (two && b>=n) || !tree_ok(lens,false) {None} else {Some((r,huff(lens)))}
    } else {
        let nc=db::field(s,p+1,4)+4;let cl=code_lengths(s,p+5,end,nc as nat,0,Seq::new(2328,|i:int|0u8));
        if p+5>end*8 || cl.is_none() || !tree_ok(cl.unwrap().1,false) {None} else {
            let (q,ls)=cl.unwrap();let flag=db::field(s,q,1);let nb=2+2*db::field(s,q+1,3);
            let left=if flag==0 {n} else {2+db::field(s,q+4,nb as nat)};let r=if flag==0 {q+1} else {q+4+nb};
            if q+1>end*8 || (flag!=0 && q+4>end*8) || r>end*8 || left>n {None} else {
                let lens=lengths(s,r,end,huff(ls),n,left as nat,8,Seq::empty());
                if lens.is_none() || !tree_ok(lens.unwrap().1,false) {None}
                else {Some((lens.unwrap().0,huff(lens.unwrap().1)))}
            }
        }
    }
}
pub(super) fn read_table(s:&[u8],bits:&mut Bits,n:usize) -> (r:Result<Huff,Error>)
    requires old(bits).valid(s@),1<=n<=2328,
    ensures final(bits).valid(s@),final(bits).end==old(bits).end,final(bits).pos()>=old(bits).pos(),
        r.is_ok()==table(s@,old(bits).pos(),old(bits).end as int,n as int).is_some(),
        r.is_ok() ==> r.unwrap().valid(),
        r.is_ok() ==> (final(bits).pos(),r.unwrap().model())==table(s@,old(bits).pos(),old(bits).end as int,n as int).unwrap(),
{
    reveal(table);
    if bits.read(s,1)?!=0 {
        let two=bits.read(s,1)?!=0;let nb=if bits.read(s,1)?==0 {1usize} else {8usize};
        let a=bits.read(s,nb)?;let b=if two {bits.read(s,8)?} else {0usize};
        if a>=n || (two && b>=n) {return Err(Error::Invalid);}
        let mut lens=[0u8;2328];lens[a]=1;if two {lens[b]=1;}
        assert(lens@ =~= Seq::new(2328,|i:int|if i==a || (two && i==b) {1u8} else {0u8}));
        let ghost p=old(bits).pos();
        let ghost sub=db::field(s@,p+1,1)==1;
        let ghost nbm=if db::field(s@,p+2,1)==0 {1nat} else {8nat};
        let ghost am=db::field(s@,p+3,nbm);let ghost bm=db::field(s@,p+3+nbm,8);
        assert(two==sub && a==am && (two ==> b==bm));
        assert(lens@ =~= Seq::new(2328,|i:int|if i==am || (sub && i==bm) {1u8} else {0u8}));
        assert(table(s@,old(bits).pos(),old(bits).end as int,n as int).is_some()==tree_ok(lens@,false));
        assert(tree_ok(lens@,false) ==> table(s@,old(bits).pos(),old(bits).end as int,n as int).unwrap()==(bits.pos(),huff(lens@)));
        Huff::build(lens,false)
    } else {
        let nc=bits.read(s,4)?+4;let cl=read_code_lengths(s,bits,nc)?;
        let left=if bits.read(s,1)?==0 {n} else {let nb=2+2*bits.read(s,3)?;bits.read(s,nb)?+2};
        if left>n {return Err(Error::Invalid);}
        let lens=read_lengths(s,bits,cl,n,left)?;
        assert(table(s@,old(bits).pos(),old(bits).end as int,n as int).is_some()==tree_ok(lens@,false));
        assert(tree_ok(lens@,false) ==> table(s@,old(bits).pos(),old(bits).end as int,n as int).unwrap()==(bits.pos(),huff(lens@)));
        Huff::build(lens,false)
    }
}
}
