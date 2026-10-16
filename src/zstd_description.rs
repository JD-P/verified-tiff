//! Direct/FSE-compressed Huffman weight descriptions, including implicit tail.
use super::*;
use crate::zstd_fse as zf;
use crate::zstd_table as zt;
use crate::zstd_weights as zw;
use crate::zstd_huffman::{self as zh,Huff};
verus! {
pub(super) struct WModel {pub values:Seq<u8>,pub at:int}
pub(super) open spec fn padded(s:Seq<u8>) -> Seq<u8> {Seq::new(256,|i:int|if i<s.len(){s[i]}else{0u8})}
pub(super) closed spec fn weights(s:Seq<u8>,start:int,end:int) -> Option<WModel> {
    if start>=end{None}else{
        let h=s[start] as int;
        if h>=128 {
            let n=h-127;let used=(n+1)/2;
            if start+1+used>end {None}else{
                Some(WModel {values:Seq::new(n as nat,|i:int|if i%2==0 {s[start+1+i/2]/16}else{s[start+1+i/2]%16}),at:start+1+used})
            }
        } else if start+1+h>end {None}else{
            let at=start+1+h;let d=zf::distribution(s,start+1,at,6,12);
            if d.is_none(){None}else{
                let q=d.unwrap();let t=zt::table_model(q.counts,q.log as usize);
                if t.is_none(){None}else{
                    let w=zw::model(s,q.at,at,t.unwrap(),q.log as usize);
                    if w.is_none(){None}else{Some(WModel {values:w.unwrap(),at})}
                }
            }
        }
    }
}
pub(super) struct Weights {pub array:[u8;256],pub count:usize,pub at:usize}
fn load(s:&[u8],start:usize,end:usize) -> (r:Result<Weights,Error>)
    requires start<=end<=s.len(),end<=usize::MAX/8,
    ensures r.is_ok()==weights(s@,start as int,end as int).is_some(),
        r.is_ok() ==> r.unwrap().array@==padded(weights(s@,start as int,end as int).unwrap().values),
        r.is_ok() ==> r.unwrap().count==weights(s@,start as int,end as int).unwrap().values.len(),
        r.is_ok() ==> r.unwrap().at==weights(s@,start as int,end as int).unwrap().at,
        r.is_ok() ==> 1<=r.unwrap().count<=255 && start<r.unwrap().at<=end,
{
    reveal(weights);
    if start==end{return Err(Error::Truncated);}
    let h=s[start] as usize;
    if h>=128 {
        let count=h-127;let used=(count+1)/2;
        if used>end-start-1{return Err(Error::Truncated);}
        let at=start+1+used;let mut array=[0u8;256];let mut i=0usize;
        while i<count
            invariant i<=count,1<=count<=128,count==h-127,h==s@[start as int],h>=128,
                start<end,end<=s.len(),at==start+1+(count+1)/2,at<=end,
                forall|k:int|0<=k<i ==> array@[k]==if k%2==0{s@[start as int+1+k/2]/16}else{s@[start as int+1+k/2]%16},
                forall|k:int|i<=k<256 ==> array@[k]==0,
            decreases count-i,
        {
            let byte=s[start+1+i/2];array[i]=if i%2==0{byte/16}else{byte%16};i=i+1;
        }
        assert(array@ =~= padded(weights(s@,start as int,end as int).unwrap().values));
        Ok(Weights {array,count,at})
    } else {
        if h>end-start-1{return Err(Error::Truncated);}
        let at=start+1+h;
        let d=zf::parse(s,start+1,at,6,12)?;
        let table=zt::Table::build(&d.counts,d.log)?;
        let (array,count)=zw::decode(s,d.at,at,&table)?;
        assert(array@ =~= padded(weights(s@,start as int,end as int).unwrap().values));
        Ok(Weights {array,count,at})
    }
}
pub(super) struct Description {pub weights:Seq<u8>,pub log:usize,pub at:int}
pub(super) closed spec fn description(s:Seq<u8>,start:int,end:int) -> Option<Description> {
    let w=weights(s,start,end);
    if w.is_none(){None}else{
        let v=w.unwrap();let c=zh::completed(padded(v.values),v.values.len() as usize);
        if c.is_none(){None}else{
            let (ws,log)=c.unwrap();
            if !zh::tree_ok(ws,log){None}else{Some(Description {weights:ws,log,at:v.at})}
        }
    }
}
pub(super) fn read(s:&[u8],start:usize,end:usize) -> (r:Result<(Huff,usize),Error>)
    requires start<=end<=s.len(),end<=usize::MAX/8,
    ensures r.is_ok()==description(s@,start as int,end as int).is_some(),
        r.is_ok() ==> r.unwrap().0.valid() && r.unwrap().0.ready,
        r.is_ok() ==> r.unwrap().0.weights@==description(s@,start as int,end as int).unwrap().weights,
        r.is_ok() ==> r.unwrap().0.log==description(s@,start as int,end as int).unwrap().log,
        r.is_ok() ==> r.unwrap().1==description(s@,start as int,end as int).unwrap().at && start<r.unwrap().1<=end,
{
    reveal(description);
    let w=load(s,start,end)?;
    let (weights,log)=zh::complete(w.array,w.count)?;
    let h=Huff::build(weights,log)?;
    Ok((h,w.at))
}
}
