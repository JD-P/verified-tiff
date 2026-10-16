//! Allocation and range policies executed by the compatibility adapter.
use super::*;
use core::mem::MaybeUninit;
verus! {
pub open spec fn admission(used:usize,n:usize,single:usize,total:usize)->bool {
    used as int+n<=usize::MAX && (single==0||n<=single) && (total==0||used as int+n<=total)
}
pub fn claim(used:usize,n:usize,single:usize,total:usize)->(r:Result<usize,Error>)
    ensures r.is_ok()==admission(used,n,single,total),r.is_ok() ==> r.unwrap()==used as int+n,
{
    if single!=0&&n>single{return Err(Error::Limit);}
    let next=match add(used,n){Ok(v)=>v,Err(e)=>return Err(e)};
    if total!=0&&next>total{return Err(Error::Limit);}Ok(next)
}
pub fn release(used:usize,n:usize)->(r:Result<usize,Error>)
    ensures r.is_ok()==(n<=used),r.is_ok() ==> r.unwrap() as int+n==used,
{if n>used {Err(Error::Invalid)}else{Ok(used-n)}}
/// Requested allocation bytes; no allocator rounding or process-RSS claim.
pub fn allocation(count:usize,width:usize)->(r:Result<usize,Error>)
    ensures r.is_ok()==(count as int*width<=isize::MAX),r.is_ok() ==> r.unwrap()==count as int*width,
{
    let n=match mul(count,width){Ok(n)=>n,Err(e)=>return Err(e)};
    if n>isize::MAX as usize{return Err(Error::Limit);}Ok(n)
}
pub fn source_range(len:u64,off:u64,n:usize)->(r:bool)
    ensures r==(off as int+n<=len),
{off<=len && n as u64<=len-off}
pub fn source_offset(base:u64,rel:usize,len:usize,file:u64)->(r:Result<u64,Error>)
    ensures r.is_ok()==(base as int+rel+len<=file),
        r.is_ok() ==> r.unwrap()==base as int+rel && r.unwrap() as int+len<=file,
{
    let at=match base.checked_add(rel as u64){Some(v)=>v,None=>return Err(Error::Limit)};
    if !source_range(file,at,len){return Err(Error::Truncated);}Ok(at)
}
/// Expand a requested byte interval to complete predictor rows, without the
/// overflowing end+row-1 shortcut. The last row may be partial only if total is.
pub fn row_range(total:usize,row:usize,start:usize,len:usize)->(r:Result<(usize,usize),Error>)
    ensures r.is_ok() ==> row>0 && start as int+len<=total && r.unwrap().0<=start
        && start as int+len<=r.unwrap().1 && r.unwrap().1<=total,
        r.is_ok() ==> r.unwrap().0==row*(start as int/(row as int)),
        r.is_ok() ==> r.unwrap().1==if (start as int+len)%(row as int)==0 {start as int+len} else {row*((start as int+len)/(row as int))+row},
        row>0 && total%row==0 && start as int+len<=total ==> r.is_ok(),
{
    if row==0||!checked_range(total,start,len){return Err(Error::Invalid);}
    let end=start+len;
    proof {
        vstd::arithmetic::div_mod::lemma_mod_decreases(start as nat,row as nat);
        vstd::arithmetic::div_mod::lemma_mod_decreases(end as nat,row as nat);
        quotient_remainder(start as int,row as int);quotient_remainder(end as int,row as int);
        if total%row==0 && end%row!=0 {
            quotient_remainder(total as int,row as int);
            let q=end as int/(row as int);let tq=total as int/(row as int);let rem=end as int%(row as int);
            assert(q<tq) by(nonlinear_arith) requires end as int==row*q+rem,total as int==row*tq,end<=total,rem>0,row>0;
            assert(end as int-rem+row<=total) by(nonlinear_arith) requires q<tq,total as int==row*tq,end as int==row*q+rem,row>0;
        }
    }
    let first=start-start%row;
    let last=if end%row==0{end}else{match add(end-end%row,row){Ok(v)=>v,Err(e)=>return Err(e)}};
    if last>total {return Err(Error::Invalid);}
    Ok((first,last))
}
/// Keep a finite traversal quota even when all offsets are distinct.
pub fn directory_step(steps:usize,max:usize)->(r:Result<usize,Error>)
    ensures r.is_ok()==(steps<max),r.is_ok() ==> r.unwrap()==steps as int+1 && r.unwrap()<=max,
{if steps>=max{Err(Error::Limit)}else{Ok(steps+1)}}
/// Failure-atomic output commit: the only written interval is exactly source.
#[inline(always)]
pub fn commit(source:&[u8],out:&mut[u8],at:usize)->(r:Result<(),Error>)
    ensures r.is_ok()==(at as int+source.len()<=old(out).len()),
        final(out).len()==old(out).len(),r.is_err() ==> final(out)@==old(out)@,
        r.is_ok() ==> forall|i:int|0<=i<source.len() ==> #[trigger] final(out)@[at+i]==source@[i],
        forall|i:int|0<=i<old(out).len() && !(at<=i<at as int+source.len()) ==> final(out)@[i]==old(out)@[i],
{
    if !checked_range(out.len(),at,source.len()){return Err(Error::OutputTooSmall);}
    copy_byte_run(source,0,out,at,source.len(),false);Ok(())
}

/// C output buffers need not be initialized. This loop initializes exactly the
/// successful output prefix without reading its previous contents.
#[inline(always)]
pub fn commit_uninit(source:&[u8],out:&mut[MaybeUninit<u8>])->(r:Result<(),Error>)
    ensures r.is_ok()==(source.len()<=old(out).len()),final(out).len()==old(out).len(),
        r.is_err() ==> final(out)@==old(out)@,
        r.is_ok() ==> forall|i:int|0<=i<source.len() ==> #[trigger] vstd::std_specs::maybe_uninit::MaybeUninitAdditionalSpecFns::as_option(final(out)@[i])==Some(source@[i]),
        forall|i:int|source.len()<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
{
    if source.len()>out.len(){return Err(Error::OutputTooSmall);}
    let mut i=0;
    while i<source.len()
        invariant i<=source.len()<=out.len(),out.len()==old(out).len(),
            forall|k:int|0<=k<i ==> #[trigger] vstd::std_specs::maybe_uninit::MaybeUninitAdditionalSpecFns::as_option(out@[k])==Some(source@[k]),
            forall|k:int|i<=k<out.len() ==> out@[k]==old(out)@[k],
        decreases source.len()-i,
    {out[i]=MaybeUninit::new(source[i]);i+=1;}
    Ok(())
}

}
