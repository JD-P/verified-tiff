//! Zstandard sequence execution against a byte-string history model.
use super::*;
use crate::zstd_bits::{self as zb,Reverse};
use crate::zstd_table::{Table,TableModel};
use crate::zstd_sequence_tables as zs;
use crate::zstd_sequence_token as zt;
use crate::zstd_output as zo;
verus! {
pub(super) struct RunModel {pub data:Seq<u8>,pub previous:[usize;3]}
#[verifier::opaque]
pub(super) open spec fn run(s:Seq<u8>,t:Seq<TableModel>,low:int,p:int,state:[usize;3],left:nat,previous:[usize;3],literals:Seq<u8>,lp:int,out:Seq<u8>,cap:usize,window:usize) -> Option<RunModel> decreases left {
    if left==0 {
        if p!=low || out.len()+literals.len()-lp>cap {None}else{
            Some(RunModel {data:out+literals.subrange(lp,literals.len() as int),previous})
        }
    }else{
        let token=zt::token(s,low,p,t,state,left==1);
        if token.is_none(){None}else{
            let v=token.unwrap();let pp=zt::offset_model(previous,v.offset,v.literals==0);
            if pp.is_none(){None}else{
                let next=pp.unwrap();let distance=next[0];
                if lp+v.literals>literals.len() || out.len()+v.literals+v.matched>cap || distance>window || distance>out.len()+v.literals {None}else{
                    let prefix=out+literals.subrange(lp,lp+v.literals);
                    let data=zo::matched(prefix,distance,v.matched);
                    run(s,t,low,v.p as int,v.states,(left-1) as nat,next,literals,lp+v.literals,data,cap,window)
                }
            }
        }
    }
}
fn execute(s:&[u8],bits:&mut Reverse,tables:&[Table;3],initial:[usize;3],sequences:usize,previous:&mut[usize;3],literals:&[u8],literal_count:usize,out:&mut[u8],at:usize,cap:usize,window:usize) -> (r:Result<usize,Error>)
    requires old(bits).valid(s@),zs::valid(*tables),zt::states_ok(zs::models(*tables),initial),
        old(previous)[0]>0,old(previous)[1]>0,old(previous)[2]>0,literal_count<=literals.len(),
        at<=cap<=old(out).len(),sequences<=98047,
    ensures final(bits).valid(s@),final(bits).low==old(bits).low,final(bits).end==old(bits).end,
        final(previous)[0]>0,final(previous)[1]>0,final(previous)[2]>0,final(out).len()==old(out).len(),
        r.is_ok()==run(s@,zs::models(*tables),old(bits).low as int,old(bits).p as int,initial,sequences as nat,*old(previous),literals@.take(literal_count as int),0,old(out)@.take(at as int),cap,window).is_some(),
        r.is_ok() ==> r.unwrap()<=cap && final(out)@.take(r.unwrap() as int)==run(s@,zs::models(*tables),old(bits).low as int,old(bits).p as int,initial,sequences as nat,*old(previous),literals@.take(literal_count as int),0,old(out)@.take(at as int),cap,window).unwrap().data,
        r.is_ok() ==> *final(previous)==run(s@,zs::models(*tables),old(bits).low as int,old(bits).p as int,initial,sequences as nat,*old(previous),literals@.take(literal_count as int),0,old(out)@.take(at as int),cap,window).unwrap().previous,
        forall|k:int|cap<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
{
    let mut state=initial;let mut left=sequences;let mut pos=at;let mut lp=0usize;
    let ghost goal=run(s@,zs::models(*tables),bits.low as int,bits.p as int,state,sequences as nat,*previous,literals@.take(literal_count as int),0,out@.take(at as int),cap,window);
    loop
        invariant bits.valid(s@),bits.low==old(bits).low,bits.end==old(bits).end,
            zs::valid(*tables),zt::states_ok(zs::models(*tables),state),
            previous@[0]>0,previous@[1]>0,previous@[2]>0,
            left<=sequences<=98047,lp<=literal_count<=literals.len(),pos<=cap<=out.len(),out.len()==old(out).len(),
            goal==run(s@,zs::models(*tables),old(bits).low as int,old(bits).p as int,initial,sequences as nat,*old(previous),literals@.take(literal_count as int),0,old(out)@.take(at as int),cap,window),
            goal==run(s@,zs::models(*tables),bits.low as int,bits.p as int,state,left as nat,*previous,literals@.take(literal_count as int),lp as int,out@.take(pos as int),cap,window),
            forall|k:int|cap<=k<out.len() ==> out@[k]==old(out)@[k],
        decreases left,
    {
        reveal_with_fuel(run,1);
        if left==0 {
            if bits.p!=bits.low || literal_count-lp>cap-pos{return Err(Error::Invalid);}
            assert(literals@.subrange(lp as int,literal_count as int) =~= literals@.take(literal_count as int).subrange(lp as int,literal_count as int));
            zo::append_literals(literals,lp,literal_count-lp,out,pos);
            pos=pos+(literal_count-lp);
            return Ok(pos);
        }
        let v=zt::read(s,bits,tables,state,left==1)?;
        let next=zt::resolve(*previous,v.offset,v.literals==0)?;
        let distance=next[0];
        if v.literals>literal_count-lp || v.literals>cap-pos || v.matched>cap-pos-v.literals || distance>window || distance>pos+v.literals{return Err(Error::Invalid);}
        assert(literals@.subrange(lp as int,lp as int+v.literals) =~= literals@.take(literal_count as int).subrange(lp as int,lp as int+v.literals));
        zo::append_literals(literals,lp,v.literals,out,pos);pos=pos+v.literals;lp=lp+v.literals;
        zo::append_match(out,pos,distance,v.matched);pos=pos+v.matched;
        *previous=next;state=v.states;left=left-1;
    }
}
pub(super) struct SeqModel {pub data:Seq<u8>,pub previous:[usize;3],pub tables:Seq<TableModel>}
pub(super) closed spec fn model(s:Seq<u8>,start:int,end:int,t:Seq<TableModel>,previous:[usize;3],literals:Seq<u8>,out:Seq<u8>,cap:usize,window:usize) -> Option<SeqModel> {
    let head=zs::header(s,start,end,t);
    if head.is_none(){None}else{
        let h=head.unwrap();
        if h.count==0 {
            if out.len()+literals.len()>cap{None}else{Some(SeqModel {data:out+literals,previous,tables:h.tables})}
        }else if h.at>=end || s[end-1]==0{None}else{
            let low=h.at as int*8;let p=(end-1)*8+zb::top(s[end-1]);
            let ll=h.tables[0].log;let of=h.tables[1].log;let ml=h.tables[2].log;
            if p-low<ll+of+ml{None}else{
                let a=zb::value(s,p-ll,ll as nat) as usize;let b=zb::value(s,p-ll-of,of as nat) as usize;
                let c=zb::value(s,p-ll-of-ml,ml as nat) as usize;
                let r=run(s,h.tables,low,p-ll-of-ml,[a,b,c],h.count as nat,previous,literals,0,out,cap,window);
                if r.is_none(){None}else{Some(SeqModel {data:r.unwrap().data,previous:r.unwrap().previous,tables:h.tables})}
            }
        }
    }
}
pub(super) fn decode(s:&[u8],start:usize,end:usize,tables:&mut[Table;3],previous:&mut[usize;3],literals:&[u8],literal_count:usize,out:&mut[u8],at:usize,cap:usize,window:usize) -> (r:Result<usize,Error>)
    requires start<=end<=s.len(),end<=usize::MAX/8,zs::valid(*old(tables)),
        old(previous)[0]>0,old(previous)[1]>0,old(previous)[2]>0,literal_count<=literals.len(),at<=cap<=old(out).len(),
    ensures zs::valid(*final(tables)),final(out).len()==old(out).len(),final(previous)[0]>0,final(previous)[1]>0,final(previous)[2]>0,
        r.is_ok()==model(s@,start as int,end as int,zs::models(*old(tables)),*old(previous),literals@.take(literal_count as int),old(out)@.take(at as int),cap,window).is_some(),
        r.is_ok() ==> r.unwrap()<=cap,
        r.is_ok() ==> final(out)@.take(r.unwrap() as int)==model(s@,start as int,end as int,zs::models(*old(tables)),*old(previous),literals@.take(literal_count as int),old(out)@.take(at as int),cap,window).unwrap().data,
        r.is_ok() ==> *final(previous)==model(s@,start as int,end as int,zs::models(*old(tables)),*old(previous),literals@.take(literal_count as int),old(out)@.take(at as int),cap,window).unwrap().previous,
        r.is_ok() ==> zs::models(*final(tables))==model(s@,start as int,end as int,zs::models(*old(tables)),*old(previous),literals@.take(literal_count as int),old(out)@.take(at as int),cap,window).unwrap().tables,
        forall|k:int|cap<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
{
    reveal(model);
    let (after,count)=zs::read(s,start,end,tables)?;
    if count==0 {
        if literal_count>cap-at{return Err(Error::Invalid);}
        zo::append_literals(literals,0,literal_count,out,at);
        return Ok(at+literal_count);
    }
    let mut bits=Reverse::new(s,after,end)?;
    let a=bits.read(s,tables[0].log)? as usize;let b=bits.read(s,tables[1].log)? as usize;let c=bits.read(s,tables[2].log)? as usize;
    execute(s,&mut bits,tables,[a,b,c],count,previous,literals,literal_count,out,at,cap,window)
}
}
