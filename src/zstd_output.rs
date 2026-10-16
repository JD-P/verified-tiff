//! Byte-string LZ history semantics and framed output operations.
use super::*;
verus! {
pub(super) open spec fn matched(prefix:Seq<u8>,distance:usize,count:usize) -> Seq<u8> {
    prefix+Seq::new(count as nat,|i:int|prefix[prefix.len() as int-distance+i%(distance as int)])
}
pub(super) fn append_match(out:&mut[u8],at:usize,distance:usize,count:usize)
    requires 0<distance<=at<=old(out).len(),count<=old(out).len()-at,
    ensures final(out).len()==old(out).len(),
        final(out)@.take(at as int+count)==matched(old(out)@.take(at as int),distance,count),
        forall|k:int|at as int+count<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
{
    let mut i=0usize;
    while i<count
        invariant i<=count,0<distance<=at<=out.len(),count<=out.len()-at,out.len()==old(out).len(),
            forall|k:int|0<=k<at ==> out@[k]==old(out)@[k],
            forall|k:int|at<=k<at as int+i ==> out@[k]==old(out)@[at as int-distance+(k-at)%(distance as int)],
            forall|k:int|at as int+count<=k<out.len() ==> out@[k]==old(out)@[k],
        decreases count-i,
    {
        let source=at+i-distance;
        if i<distance{proof{vstd::arithmetic::div_mod::lemma_small_mod(i as nat,distance as nat);}assert(source<at && i%distance==i);}
        else{
            proof{vstd::arithmetic::div_mod::lemma_mod_sub_multiples_vanish(i as int,distance as int);}
            assert(out@[source as int]==old(out)@[at as int-distance+(i as int)%(distance as int)]);
        }
        out[at+i]=out[source];i=i+1;
    }
    assert(out@.take(at as int+count) =~= matched(old(out)@.take(at as int),distance,count)) by {
        assert forall|k:int|0<=k<at as int+count implies out@[k]==matched(old(out)@.take(at as int),distance,count)[k] by {
            if k<at {}else{quotient_remainder(k-at,distance as int);}
        }
    }
}
pub(super) fn append_literals(input:&[u8],start:usize,count:usize,out:&mut[u8],at:usize)
    requires start<=input.len(),count<=input.len()-start,at<=old(out).len(),count<=old(out).len()-at,
    ensures final(out).len()==old(out).len(),
        final(out)@.take(at as int+count)==old(out)@.take(at as int)+input@.subrange(start as int,start as int+count),
        forall|k:int|at as int+count<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
{
    copy_byte_run(input,start,out,at,count,false);
    let ghost expected=old(out)@.take(at as int)+input@.subrange(start as int,start as int+count);
    assert forall|k:int|0<=k<at as int+count implies out@[k]==expected[k] by {
        if k<at {}else{assert(out@[at as int+(k-at)]==input@[start as int+(k-at)]);}
    }
    assert(out@.take(at as int+count) =~= expected);
}

pub(super) fn append_repeat(out:&mut[u8],at:usize,count:usize,value:u8)
    requires at<=old(out).len(),count<=old(out).len()-at,
    ensures final(out).len()==old(out).len(),
        final(out)@.take(at as int+count)==old(out)@.take(at as int)+Seq::new(count as nat,|i:int|value),
        forall|k:int|at as int+count<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
{
    let mut i=0usize;
    while i<count
        invariant i<=count,at<=out.len(),count<=out.len()-at,out.len()==old(out).len(),
            forall|k:int|0<=k<at ==> out@[k]==old(out)@[k],
            forall|k:int|at<=k<at as int+i ==> out@[k]==value,
            forall|k:int|at as int+count<=k<out.len() ==> out@[k]==old(out)@[k],
        decreases count-i,
    {out[at+i]=value;i=i+1;}
    assert(out@.take(at as int+count) =~= old(out)@.take(at as int)+Seq::new(count as nat,|i:int|value));
}
}
