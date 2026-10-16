//! Dictionary-free Zstandard strip/tile decoding, with explicit caller scratch,
//! complete frame/checksum validation and bounded byte-string output semantics.
use super::*;
use crate::zstd_frame as zf;
use crate::zstd_table::{Table,TableModel,Cell};
use crate::zstd_huffman::Huff;
use crate::zstd_literals as zl;
use crate::zstd_sequences as zs;
use crate::zstd_sequence_tables as zt;
use crate::zstd_output as zo;
use crate::zstd_checksum as zc;
verus! {
struct ContextModel {weights:Seq<u8>,log:usize,ready:bool,tables:Seq<TableModel>,previous:[usize;3]}
struct Context {huff:Huff,tables:[Table;3],previous:[usize;3]}
impl Context {
    spec fn model(self) -> ContextModel {ContextModel {weights:self.huff.weights@,log:self.huff.log,ready:self.huff.ready,tables:zt::models(self.tables),previous:self.previous}}
    spec fn valid(self) -> bool {self.huff.valid() && zt::valid(self.tables) && self.previous[0]>0 && self.previous[1]>0 && self.previous[2]>0}
}
spec fn blank() -> ContextModel {
    let cells=Seq::new(512,|i:int|Cell {symbol:0,bits:0,base:0});
    ContextModel {weights:Seq::new(256,|i:int|0u8),log:1,ready:false,
        tables:Seq::new(3,|i:int|TableModel {cells,log:0,ready:false}),previous:[1,4,8]}
}
fn empty() -> (r:Context) ensures r.valid(),r.model()==blank(),
{
    let huff=Huff::empty();let t=Table::empty();
    let r=Context {huff,tables:[t,t,t],previous:[1,4,8]};
    assert(r.model().weights =~= blank().weights);
    assert(t.cells@ =~= blank().tables[0].cells);
    assert(r.model().tables =~= blank().tables);
    r
}
pub open spec fn scratch_size(cap:usize) -> usize {if cap<131072{cap}else{131072}}
pub fn scratch_len(cap:usize) -> (r:usize) ensures r==scratch_size(cap),
{if cap<131072{cap}else{131072}}
spec fn capacity(cap:usize,at:int,window:usize) -> int {
    let block=if window<131072{window as int}else{131072int};if cap as int-at<block{cap as int-at}else{block}
}
struct BlockModel {data:Seq<u8>,context:ContextModel}
#[verifier::opaque]
spec fn block_result(s:Seq<u8>,b:zf::Block,window:usize,c:ContextModel,out:Seq<u8>,cap:usize) -> Option<BlockModel> {
    if b.kind<2 {
        if out.len()+b.size>cap {None}else{
            let data=if b.kind==0{s.subrange(b.at as int,b.next as int)}else{Seq::new(b.size as nat,|i:int|s[b.at as int])};
            Some(BlockModel {data:out+data,context:c})
        }
    }else{
        let space=capacity(cap,out.len() as int,window) as usize;
        let ll=zl::model(s,b.at as int,b.next as int,c.weights,c.log,c.ready,space);
        if ll.is_none(){None}else{
            let l=ll.unwrap();let q=zs::model(s,l.at as int,b.next as int,c.tables,c.previous,l.data,out,(out.len()+space) as usize,window);
            if q.is_none(){None}else{
                let v=q.unwrap();Some(BlockModel {data:v.data,context:ContextModel {weights:l.weights,log:l.log,ready:l.ready,tables:v.tables,previous:v.previous}})
            }
        }
    }
}
fn decode_block(s:&[u8],b:zf::Block,window:usize,context:&mut Context,out:&mut[u8],at:usize,cap:usize,scratch:&mut[u8]) -> (r:Result<usize,Error>)
    requires old(context).valid(),b.at<=b.next<=s.len(),s.len()<=usize::MAX/8,b.kind<3,b.size<=131072,
        b.kind==1 ==> b.next==b.at+1,b.kind!=1 ==> b.next==b.at+b.size,
        at<=cap<=old(out).len(),old(scratch).len()>=scratch_size(cap),
    ensures final(context).valid(),final(out).len()==old(out).len(),final(scratch).len()==old(scratch).len(),
        r.is_ok()==block_result(s@,b,window,old(context).model(),old(out)@.take(at as int),cap).is_some(),
        r.is_ok() ==> r.unwrap()<=cap,
        r.is_ok() ==> final(out)@.take(r.unwrap() as int)==block_result(s@,b,window,old(context).model(),old(out)@.take(at as int),cap).unwrap().data,
        r.is_ok() ==> final(context).model()==block_result(s@,b,window,old(context).model(),old(out)@.take(at as int),cap).unwrap().context,
        forall|k:int|cap<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
        forall|k:int|scratch_size(cap)<=k<old(scratch).len() ==> final(scratch)@[k]==old(scratch)@[k],
{
    reveal_with_fuel(block_result,1);
    if b.kind<2 {
        if b.size>cap-at{return Err(Error::OutputTooSmall);}
        if b.kind==0{zo::append_literals(s,b.at,b.size,out,at);}
        else{zo::append_repeat(out,at,b.size,s[b.at]);}
        Ok(at+b.size)
    }else{
        let maximum=if window<131072{window}else{131072usize};
        let space=if cap-at<maximum{cap-at}else{maximum};
        let (after,count)=zl::decode(s,b.at,b.next,&mut context.huff,scratch,space)?;
        zs::decode(s,after,b.next,&mut context.tables,&mut context.previous,scratch,count,out,at,at+space,window)
    }
}
spec fn terminal(s:Seq<u8>,p:zf::Plan,out:Seq<u8>) -> bool {
    (!p.header.known || out.len()==p.header.size) &&
        (!p.header.checksum || word32(s,p.footer as int,true)==(zc::checksum(out,out.len() as usize) as int%4294967296))
}
#[verifier::opaque]
spec fn finish(s:Seq<u8>,p:zf::Plan,at:int,c:ContextModel,out:Seq<u8>,cap:usize,fuel:nat) -> Option<Seq<u8>> decreases fuel {
    if fuel==0{None}else{
        let bb=zf::block(s,at,p.header.window);
        if bb.is_none(){None}else{
            let b=bb.unwrap();let rr=block_result(s,b,p.header.window,c,out,cap);
            if rr.is_none(){None}else{
                let r=rr.unwrap();
                if b.last {
                    if fuel!=1 || b.next!=p.footer || !terminal(s,p,r.data){None}else{Some(r.data)}
                }else{finish(s,p,b.next as int,r.context,r.data,cap,(fuel-1) as nat)}
            }
        }
    }
}
pub closed spec fn decoded(s:Seq<u8>,cap:usize) -> Option<Seq<u8>> {
    if s.len()>usize::MAX/8{None}else{
        let pp=zf::plan(s);
        if pp.is_none(){None}else{
            let p=pp.unwrap();
            if p.header.known && p.header.size>cap{None}else{finish(s,p,p.header.at as int,blank(),Seq::empty(),cap,p.blocks as nat)}
        }
    }
}
fn process(s:&[u8],p:zf::Plan,cap:usize,out:&mut[u8],scratch:&mut[u8]) -> (r:Result<usize,Error>)
    requires s.len()<=usize::MAX/8,zf::plan(s@).is_some(),p==zf::plan(s@).unwrap(),
        p.header.at<=p.footer<=s.len(),p.footer as int+if p.header.checksum{4int}else{0int}==s.len(),
        cap<=old(out).len(),scratch_size(cap)<=old(scratch).len(),
    ensures final(out).len()==old(out).len(),final(scratch).len()==old(scratch).len(),
        r.is_ok()==decoded(s@,cap).is_some(),
        r.is_ok() ==> r.unwrap()<=cap && final(out)@.take(r.unwrap() as int)==decoded(s@,cap).unwrap(),
        forall|k:int|cap<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
        forall|k:int|scratch_size(cap)<=k<old(scratch).len() ==> final(scratch)@[k]==old(scratch)@[k],
{
    reveal(decoded);
    if p.header.known && p.header.size>cap as u64{return Err(Error::OutputTooSmall);}
    let mut context=empty();let mut at=p.header.at;let mut pos=0usize;let mut fuel=p.blocks;
    let ghost goal=decoded(s@,cap);
    assert(out@.take(0) =~= Seq::<u8>::empty());
    loop
        invariant context.valid(),at<=s.len(),pos<=cap<=out.len(),scratch_size(cap)<=scratch.len(),s.len()<=usize::MAX/8,
            out.len()==old(out).len(),scratch.len()==old(scratch).len(),fuel<=p.blocks,
            p.header.at<=p.footer<=s.len(),p.footer as int+if p.header.checksum{4int}else{0int}==s.len(),
            goal==decoded(s@,cap),goal==finish(s@,p,at as int,context.model(),out@.take(pos as int),cap,fuel as nat),
            forall|k:int|cap<=k<out.len() ==> out@[k]==old(out)@[k],
            forall|k:int|scratch_size(cap)<=k<scratch.len() ==> scratch@[k]==old(scratch)@[k],
        decreases fuel,
    {
        reveal_with_fuel(finish,1);
        if fuel==0{return Err(Error::Invalid);}
        let b=zf::read_block(s,at,p.header.window)?;
        pos=decode_block(s,b,p.header.window,&mut context,out,pos,cap,scratch)?;
        if b.last {
            if fuel!=1 || b.next!=p.footer{return Err(Error::Invalid);}
            if p.header.known && pos as u64!=p.header.size{return Err(Error::Invalid);}
            if p.header.checksum {
                let (head,_)=out.split_at(pos);
                let sum=zc::compute(head,pos);
                let wanted=read32(s,p.footer,true)?;
                if (sum%4294967296u64) as u32!=wanted{return Err(Error::Invalid);}
            }
            return Ok(pos);
        }
        at=b.next;fuel=fuel-1;
    }
}
pub open spec fn fee(count:int,cap:int,compressed:int) -> int {8*count+64*cap+50000*compressed+128}
fn cost(count:usize,cap:usize,compressed:usize) -> (r:Result<usize,Error>)
    ensures r.is_ok()==(fee(count as int,cap as int,compressed as int)<=usize::MAX),
        r.is_ok() ==> r.unwrap()==fee(count as int,cap as int,compressed as int),
{add(add(add(mul(count,8)?,mul(cap,64)?)?,mul(compressed,50000)?)?,128)}
pub closed spec fn work(s:Seq<u8>,cap:usize) -> int {
    if s.len()>usize::MAX/8 || zf::plan(s).is_none(){-1}else{fee(s.len() as int,cap as int,zf::plan(s).unwrap().compressed as int)}
}
/// Cost of a structurally valid frame; entropy/checksum validation is performed
/// by decode. This inspection does not allocate or decompress pixel data.
pub fn work_limit(s:&[u8],cap:usize) -> (r:Result<usize,Error>)
    ensures r.is_ok()==(0<=work(s@,cap)<=usize::MAX),r.is_ok() ==> r.unwrap()==work(s@,cap),
{
    reveal(work);
    if s.len()>usize::MAX/8{return Err(Error::Limit);}
    let p=zf::inspect(s)?;cost(s.len(),cap,p.compressed)
}
/// Writes the complete frame (at least needed, at most cap). A C caller uses a
/// private output buffer to retain failure atomicity. Only the output prefix
/// below cap and scratch prefix below scratch_len(cap) can change on failure.
pub fn decode(s:&[u8],needed:usize,cap:usize,out:&mut[u8],scratch:&mut[u8],budget:&mut Budget) -> (r:Result<usize,Error>)
    ensures final(out).len()==old(out).len(),final(scratch).len()==old(scratch).len(),final(budget).remaining<=old(budget).remaining,
        r.is_ok()==(needed<=cap && old(out).len()>=cap && old(scratch).len()>=scratch_size(cap)
            && decoded(s@,cap).is_some() && decoded(s@,cap).unwrap().len()>=needed
            && 0<=work(s@,cap)<=old(budget).remaining),
        r.is_ok() ==> needed<=r.unwrap()<=cap && final(out)@.take(r.unwrap() as int)==decoded(s@,cap).unwrap(),
        r.is_ok() ==> final(budget).remaining as int+work(s@,cap)==old(budget).remaining,
        forall|k:int|cap<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
        forall|k:int|scratch_size(cap)<=k<old(scratch).len() ==> final(scratch)@[k]==old(scratch)@[k],
{
    reveal(work);reveal(decoded);
    if needed>cap{return Err(Error::Invalid);}
    if out.len()<cap || scratch.len()<scratch_len(cap){return Err(Error::OutputTooSmall);}
    if s.len()>usize::MAX/8{return Err(Error::Limit);}
    let prefix=s.len()*8;spend(budget,prefix)?;
    let p=zf::inspect(s)?;let fee=cost(s.len(),cap,p.compressed)?;
    spend(budget,fee-prefix)?;
    let n=process(s,p,cap,out,scratch)?;
    if n<needed{return Err(Error::Truncated);}
    Ok(n)
}

proof fn raw_witness()
    ensures decoded(zf::witness_bytes(),2)==Some(seq![65u8,66u8]),
{
    zf::raw_witness();reveal(decoded);reveal_with_fuel(finish,1);reveal_with_fuel(block_result,1);
    assert(Seq::<u8>::empty()+zf::witness_bytes().subrange(9,11) =~= seq![65u8,66u8]);
}
}
