//! Sequence-table modes and normative predefined FSE distributions.
use super::*;
use crate::zstd_bits as zb;
use crate::zstd_fse as zf;
use crate::zstd_table::{self as zt,Table,TableModel,Cell};
verus! {
pub(super) open spec fn predefined(kind:usize) -> Seq<i16> {
    if kind==0{seq![4i16,3i16,2i16,2i16,2i16,2i16,2i16,2i16,2i16,2i16,2i16,2i16,2i16,1i16,1i16,1i16,2i16,2i16,2i16,2i16,2i16,2i16,2i16,2i16,2i16,3i16,2i16,1i16,1i16,1i16,1i16,1i16,-1i16,-1i16,-1i16,-1i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16]}else if kind==1{seq![1i16,1i16,1i16,1i16,1i16,1i16,2i16,2i16,2i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,-1i16,-1i16,-1i16,-1i16,-1i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16]}else{seq![1i16,4i16,3i16,2i16,2i16,2i16,2i16,2i16,2i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,-1i16,-1i16,-1i16,-1i16,-1i16,-1i16,-1i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16]}
}
fn defaults(kind:usize) -> (r:[i16;64])
    requires kind<3,
    ensures r@==predefined(kind),
{if kind==0{[4i16,3i16,2i16,2i16,2i16,2i16,2i16,2i16,2i16,2i16,2i16,2i16,2i16,1i16,1i16,1i16,2i16,2i16,2i16,2i16,2i16,2i16,2i16,2i16,2i16,3i16,2i16,1i16,1i16,1i16,1i16,1i16,-1i16,-1i16,-1i16,-1i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16]}else if kind==1{[1i16,1i16,1i16,1i16,1i16,1i16,2i16,2i16,2i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,-1i16,-1i16,-1i16,-1i16,-1i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16]}else{[1i16,4i16,3i16,2i16,2i16,2i16,2i16,2i16,2i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,1i16,-1i16,-1i16,-1i16,-1i16,-1i16,-1i16,-1i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16,0i16]}}
pub(super) struct ReadModel {pub table:TableModel,pub at:usize}
pub(super) closed spec fn table(s:Seq<u8>,start:int,end:int,mode:usize,kind:usize,old:TableModel) -> Option<ReadModel> {
    let maxsym=if kind==0{36usize}else if kind==1{32usize}else{53usize};
    if mode==0 {
        let log=if kind==1{5usize}else{6usize};let t=zt::table_model(predefined(kind),log);
        if t.is_none(){None}else{Some(ReadModel {table:TableModel {cells:t.unwrap(),log,ready:true},at:start as usize})}
    }else if mode==1 {
        if start>=end || s[start]>=maxsym {None}else{
            Some(ReadModel {table:TableModel {cells:Seq::new(512,|i:int|Cell {symbol:s[start],bits:0,base:0}),log:0,ready:true},at:(start+1) as usize})
        }
    }else if mode==2 {
        let d=zf::distribution(s,start,end,if kind==1{8usize}else{9usize},maxsym);
        if d.is_none(){None}else{
            let q=d.unwrap();let t=zt::table_model(q.counts,q.log as usize);
            if t.is_none(){None}else{Some(ReadModel {table:TableModel {cells:t.unwrap(),log:q.log as usize,ready:true},at:q.at as usize})}
        }
    }else if mode==3 && old.ready {Some(ReadModel {table:old,at:start as usize})}else{None}
}
fn read_table(s:&[u8],start:usize,end:usize,mode:usize,kind:usize,old:Table) -> (r:Result<(Table,usize),Error>)
    requires start<=end<=s.len(),end<=usize::MAX/8,kind<3,mode<4,old.valid(),
    ensures r.is_ok()==table(s@,start as int,end as int,mode,kind,old.model()).is_some(),
        r.is_ok() ==> r.unwrap().0.valid() && r.unwrap().0.ready,
        r.is_ok() ==> r.unwrap().0.model()==table(s@,start as int,end as int,mode,kind,old.model()).unwrap().table,
        r.is_ok() ==> r.unwrap().1==table(s@,start as int,end as int,mode,kind,old.model()).unwrap().at && start<=r.unwrap().1<=end,
{
    reveal(table);
    let maxsym=if kind==0{36usize}else if kind==1{32}else{53};
    if mode==0 {
        let log=if kind==1{5usize}else{6};let c=defaults(kind);let t=Table::build(&c,log)?;Ok((t,start))
    }else if mode==1 {
        if start==end{return Err(Error::Truncated);}
        let symbol=s[start];if symbol as usize>=maxsym{return Err(Error::Invalid);}
        let t=Table::repeat(symbol)?;
        assert(t.cells@ =~= table(s@,start as int,end as int,mode,kind,old.model()).unwrap().table.cells);
        Ok((t,start+1))
    }else if mode==2 {
        let d=zf::parse(s,start,end,if kind==1{8usize}else{9},maxsym)?;
        let t=Table::build(&d.counts,d.log)?;Ok((t,d.at))
    }else{
        if !old.ready{return Err(Error::Invalid);}
        Ok((old,start))
    }
}
pub(super) closed spec fn count_header(s:Seq<u8>,start:int,end:int) -> Option<(usize,usize)> {
    if start>=end{None}else{
        let b=s[start] as int;
        if b<128{Some((b as usize,(start+1) as usize))}
        else if b<255 {if start+2>end{None}else{Some((((b-128)*256+s[start+1]) as usize,(start+2) as usize))}}
        else {if start+3>end{None}else{Some(((word16(s,start+1,true)+32512) as usize,(start+3) as usize))}}
    }
}
fn read_count(s:&[u8],start:usize,end:usize) -> (r:Result<(usize,usize),Error>)
    requires start<=end<=s.len(),end<=usize::MAX/8,
    ensures r.is_ok()==count_header(s@,start as int,end as int).is_some(),
        r.is_ok() ==> r.unwrap()==count_header(s@,start as int,end as int).unwrap(),
        r.is_ok() ==> r.unwrap().0<=98047 && start<r.unwrap().1<=end,
{
    reveal(count_header);
    if start==end{return Err(Error::Truncated);}
    let b=s[start] as usize;
    if b<128{Ok((b,start+1))}
    else if b<255{
        if end-start<2{return Err(Error::Truncated);}
        Ok(((b-128)*256+s[start+1] as usize,start+2))
    }else{
        if end-start<3{return Err(Error::Truncated);}
        Ok((read16(s,start+1,true)? as usize+32512,start+3))
    }
}
pub(super) open spec fn models(t:[Table;3]) -> Seq<TableModel> {seq![t[0].model(),t[1].model(),t[2].model()]}
pub(super) open spec fn valid(t:[Table;3]) -> bool {t[0].valid() && t[1].valid() && t[2].valid()}
pub(super) struct HeaderModel {pub tables:Seq<TableModel>,pub at:usize,pub count:usize}
pub(super) closed spec fn header(s:Seq<u8>,start:int,end:int,old:Seq<TableModel>) -> Option<HeaderModel> {
    let cc=count_header(s,start,end);
    if cc.is_none(){None}else{
        let (count,at)=cc.unwrap();
        if count==0 {if at!=end{None}else{Some(HeaderModel {tables:old,at,count})}}
        else if at>=end || s[at as int]%4!=0 {None}else{
            let m=s[at as int] as int;let a=table(s,at as int+1,end,(m/64) as usize,0,old[0]);
            if a.is_none(){None}else{
                let aa=a.unwrap();let b=table(s,aa.at as int,end,((m/16)%4) as usize,1,old[1]);
                if b.is_none(){None}else{
                    let bb=b.unwrap();let c=table(s,bb.at as int,end,((m/4)%4) as usize,2,old[2]);
                    if c.is_none(){None}else{let c=c.unwrap();Some(HeaderModel {tables:seq![aa.table,bb.table,c.table],at:c.at,count})}
                }
            }
        }
    }
}
pub(super) fn read(s:&[u8],start:usize,end:usize,tables:&mut[Table;3]) -> (r:Result<(usize,usize),Error>)
    requires start<=end<=s.len(),end<=usize::MAX/8,valid(*old(tables)),
    ensures valid(*final(tables)),
        r.is_ok()==header(s@,start as int,end as int,models(*old(tables))).is_some(),
        r.is_ok() ==> models(*final(tables))==header(s@,start as int,end as int,models(*old(tables))).unwrap().tables,
        r.is_ok() ==> r.unwrap().0==header(s@,start as int,end as int,models(*old(tables))).unwrap().at,
        r.is_ok() ==> r.unwrap().1==header(s@,start as int,end as int,models(*old(tables))).unwrap().count,
        r.is_ok() ==> start<r.unwrap().0<=end && r.unwrap().1<=98047,
        r.is_ok() && r.unwrap().1>0 ==> final(tables)@[0].ready && final(tables)@[1].ready && final(tables)@[2].ready,
{
    reveal(header);
    let (count,at)=read_count(s,start,end)?;
    if count==0{if at!=end{return Err(Error::Invalid);}return Ok((at,0));}
    if at==end{return Err(Error::Truncated);}
    let m=s[at] as usize;if m%4!=0{return Err(Error::Invalid);}
    let (a,after)=read_table(s,at+1,end,m/64,0,tables[0])?;tables[0]=a;
    let (b,after)=read_table(s,after,end,(m/16)%4,1,tables[1])?;tables[1]=b;
    let (c,after)=read_table(s,after,end,(m/4)%4,2,tables[2])?;tables[2]=c;
    assert(models(*tables) =~= header(s@,start as int,end as int,models(*old(tables))).unwrap().tables);
    Ok((after,count))
}
}
