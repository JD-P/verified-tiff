//! Source-window conservation. I/O supplies bytes at the returned offset;
//! compacting an unread suffix preserves those bytes exactly.
use super::*;
verus! {
pub open spec fn valid(cap:usize,len:usize,at:usize,loaded:usize,total:usize)->bool {
    at<=len<=cap && len<=loaded<=total
}
pub fn refill_plan(cap:usize,len:usize,at:usize,loaded:usize,total:usize)->(r:Result<(usize,usize,usize),Error>)
    ensures r.is_ok()==valid(cap,len,at,loaded,total),
        r.is_ok() ==> r.unwrap().0==len-at,
        r.is_ok() ==> r.unwrap().1==if cap as int-(len-at)<total as int-loaded {cap as int-(len-at)} else {total as int-loaded},
        r.is_ok() ==> r.unwrap().2==loaded as int+r.unwrap().1 && r.unwrap().2<=total,
        r.is_ok() ==> r.unwrap().0 as int+r.unwrap().1<=cap,
        // Absolute position of the unread byte is unchanged by refill.
        r.is_ok() ==> r.unwrap().2 as int-(r.unwrap().0+r.unwrap().1)==loaded as int-len+at,
{
    if at>len||len>cap||len>loaded||loaded>total{return Err(Error::Invalid);}
    let rest=len-at;let room=cap-rest;let left=total-loaded;
    let n=if room<left{room}else{left};Ok((rest,n,loaded+n))
}
pub fn compact(data:&mut[u8],at:usize,len:usize)->(r:Result<usize,Error>)
    ensures r.is_ok()==(at<=len<=old(data).len()),final(data).len()==old(data).len(),
        r.is_err() ==> final(data)@==old(data)@,
        r.is_ok() ==> r.unwrap()==len-at,
        r.is_ok() ==> forall|i:int|0<=i<r.unwrap() ==> #[trigger] final(data)@[i]==old(data)@[at+i],
        r.is_ok() ==> forall|i:int|r.unwrap()<=i<old(data).len() ==> #[trigger] final(data)@[i]==old(data)@[i],
{
    if at>len||len>data.len(){return Err(Error::Invalid);}
    data.copy_within(at..len,0);Ok(len-at)
}
pub fn remaining(total:usize,loaded:usize,len:usize,at:usize,bit:usize)->(r:Result<u128,Error>)
    ensures r.is_ok()==(at<=len<=loaded<=total && bit<8 && (at<len||loaded<total||bit==0)),
        r.is_ok() ==> r.unwrap()==(total as int-loaded+len-at)*8-bit,
{
    if at>len||len>loaded||loaded>total||bit>=8||at==len&&loaded==total&&bit!=0{return Err(Error::Invalid);}
    let bytes=total-loaded+(len-at);Ok(bytes as u128*8-bit as u128)
}
}
