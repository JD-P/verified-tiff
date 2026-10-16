//! Lerc2's explicitly transmitted Huffman codes. Bit words are little-endian
//! u32 values consumed most-significant bit first; they are not VP8L codes.
use super::*;
use crate::lerc_bits as packed;
verus! {
pub(super) closed spec fn window(s:Seq<u8>,base:usize,p:usize) -> Option<u32> {
    let at=base as int+(p as int/32)*4;let shift=(p%32) as u32;let count=if shift==0{4int}else{8int};
    if !range_ok(s.len() as int,at,count){None}else{
        let a=word32(s,at,true) as u32;
        if shift==0{Some(a)}else{Some((a<<shift)|((word32(s,at+4,true) as u32)>>(32-shift)))}
    }
}
pub(super) fn peek(s:&[u8],base:usize,p:usize) -> (r:Result<u32,Error>)
    requires s.len()<=usize::MAX/8,base<=s.len(),p<=s.len()*8,
    ensures r.is_ok()==window(s@,base,p).is_some(),r.is_ok() ==> r.unwrap()==window(s@,base,p).unwrap(),
        r.is_ok() ==> p as int+32<=s.len() as int*8,
{
    reveal(window);let delta=(p/32)*4;
    if delta>s.len()-base{return Err(Error::Truncated);}
    let at=base+delta;let shift=(p%32) as u32;let count=if shift==0{4usize}else{8usize};
    if count>s.len()-at{return Err(Error::Truncated);}
    let a=read32(s,at,true)?;
    if shift==0{Ok(a)}else{Ok((a<<shift)|(read32(s,at+4,true)?>>(32-shift)))}
}
pub(super) open spec fn prefix(word:u32,len:u32) -> u32 {if len==0{0u32}else{word>>(32-len)}}
fn take(word:u32,len:u32) -> (r:u32) requires len<=32,ensures r==prefix(word,len),
{if len==0{0}else{word>>(32-len)}}
#[derive(Clone,Copy)]
pub(super) struct Table {pub lengths:[u32;256],pub codes:[u32;256]}
pub(super) struct TableModel {pub lengths:Seq<u32>,pub codes:Seq<u32>}
impl Table {
    pub(super) open spec fn model(self) -> TableModel {TableModel {lengths:self.lengths@,codes:self.codes@}}
    pub(super) open spec fn valid(self) -> bool {forall|i:int|0<=i<256 ==> self.lengths@[i]<=32}
}
pub(super) struct Description {pub base:usize,pub first:usize,pub size:usize,pub lengths:Seq<u32>}
pub(super) closed spec fn description(s:Seq<u8>,start:usize) -> Option<Description> {
    if !range_ok(s.len() as int,start as int,16){None}else{
        let version=word32(s,start as int,true);let size=word32(s,start as int+4,true);
        let first=word32(s,start as int+8,true);let end=word32(s,start as int+12,true);
        let a=packed::decoded(s,(start+16) as usize);
        if version<2 || size==0 || size>256 || first>=end || end-first>size || first>=size || end>2*size || a.is_none(){None}else{
            let (at,lengths)=a.unwrap();
            if lengths.len()!=end-first{None}else{Some(Description {base:at,first:first as usize,size:size as usize,lengths})}
        }
    }
}
pub(super) struct Parsed {pub at:usize,pub table:TableModel}
#[verifier::opaque]
pub(super) closed spec fn codes(s:Seq<u8>,d:Description,k:usize,p:usize,t:TableModel,fuel:nat) -> Option<Parsed> decreases fuel {
    if k==d.lengths.len(){
        let at=d.base as int+((p as int+31)/32)*4;
        if at>s.len(){None}else{Some(Parsed {at:at as usize,table:t})}
    }else if fuel==0 || d.lengths[k as int]>32{None}else{
        let len=d.lengths[k as int];let win=window(s,d.base,p);
        if len!=0 && win.is_none(){None}else{
            let code=if len==0{0u32}else{prefix(win.unwrap(),len)};let i=(d.first+k)%(d.size as int);
            codes(s,d,(k+1) as usize,(p+len) as usize,TableModel {lengths:t.lengths.update(i,len),codes:t.codes.update(i,code)},(fuel-1) as nat)
        }
    }
}
pub(super) closed spec fn table(s:Seq<u8>,start:usize) -> Option<Parsed> {
    let d=description(s,start);
    if d.is_none(){None}else{codes(s,d.unwrap(),0,0,TableModel {lengths:Seq::new(256,|i:int|0u32),codes:Seq::new(256,|i:int|0u32)},256)}
}
pub(super) fn read_table(s:&[u8],start:usize) -> (r:Result<(usize,Table),Error>)
    requires s.len()<=usize::MAX/8,start<=s.len(),
    ensures r.is_ok()==table(s@,start).is_some(),
        r.is_ok() ==> r.unwrap().0==table(s@,start).unwrap().at && r.unwrap().1.model()==table(s@,start).unwrap().table,
        r.is_ok() ==> start<r.unwrap().0<=s.len() && r.unwrap().1.valid(),
{
    reveal(table);reveal(description);
    if s.len()-start<16{return Err(Error::Truncated);}
    let version=read32(s,start,true)?;let size=read32(s,start+4,true)? as usize;
    let first=read32(s,start+8,true)? as usize;let end=read32(s,start+12,true)? as usize;
    if version<2 || size==0 || size>256 || first>=end || end-first>size || first>=size || end>2*size{return Err(Error::Invalid);}
    let values=packed::decode(s,start+16)?;
    if values.n!=end-first{return Err(Error::Invalid);}
    let ghost d=description(s@,start).unwrap();
    let mut t=Table {lengths:[0u32;256],codes:[0u32;256]};let mut k=0usize;let mut p=0usize;
    assert(t.model().lengths =~= Seq::new(256,|i:int|0u32));assert(t.model().codes =~= Seq::new(256,|i:int|0u32));
    let ghost goal=table(s@,start);
    while k<values.n
        invariant s.len()<=usize::MAX/8,start+16<=values.at<=s.len(),values.n<=256,first<size<=256,0<size,
            k<=values.n,p<=k*32,p<=s.len()*8,t.valid(),
            d==description(s@,start).unwrap(),d.base==values.at,d.first==first,d.size==size,d.lengths==values.data@.take(values.n as int),
            goal==table(s@,start),goal==codes(s@,d,k,p,t.model(),(256-k) as nat),
        decreases values.n-k,
    {
        reveal_with_fuel(codes,1);let len=values.data[k];
        if len>32{return Err(Error::Invalid);}
        let code=if len==0{0u32}else{take(peek(s,values.at,p)?,len)};
        if len as usize>s.len()*8-p{return Err(Error::Truncated);}
        let i=(first+k)%size;t.lengths[i]=len;t.codes[i]=code;
        p=p+len as usize;k=k+1;
    }
    reveal_with_fuel(codes,1);
    let delta=((p+31)/32)*4;
    if delta>s.len()-values.at{return Err(Error::Truncated);}
    Ok((values.at+delta,t))
}
#[verifier::opaque]
pub(super) open spec fn find(t:TableModel,w:u32,i:nat) -> Option<(usize,usize)> decreases 256-i {
    if i>=256{None}else{
        let n=t.lengths[i as int];
        if n!=0 && prefix(w,n)==t.codes[i as int]{Some((i as usize,n as usize))}else{find(t,w,i+1)}
    }
}
pub(super) closed spec fn symbol(s:Seq<u8>,base:usize,p:usize,t:TableModel) -> Option<(usize,usize)> {
    let w=window(s,base,p);
    if w.is_none(){None}else{
        let found=find(t,w.unwrap(),0);
        if found.is_none(){None}else{Some((found.unwrap().0,(p+found.unwrap().1) as usize))}
    }
}
pub(super) fn read_symbol(s:&[u8],base:usize,p:usize,t:&Table) -> (r:Result<(usize,usize),Error>)
    requires t.valid(),s.len()<=usize::MAX/8,base<=s.len(),p<=s.len()*8,
    ensures r.is_ok()==symbol(s@,base,p,t.model()).is_some(),r.is_ok() ==> r.unwrap()==symbol(s@,base,p,t.model()).unwrap(),
        r.is_ok() ==> r.unwrap().0<256 && p<r.unwrap().1<=s.len()*8,
{
    reveal(symbol);let word=peek(s,base,p)?;let mut i=0usize;
    while i<256
        invariant i<=256,t.valid(),window(s@,base,p)==Some(word),p as int+32<=s.len() as int*8,s.len()<=usize::MAX/8,find(t.model(),word,0)==find(t.model(),word,i as nat),
        decreases 256-i,
    {
        reveal_with_fuel(find,1);let len=t.lengths[i];
        if len!=0 && take(word,len)==t.codes[i]{
            reveal(window);return Ok((i,p+len as usize));
        }
        i=i+1;
    }
    reveal_with_fuel(find,1);Err(Error::Invalid)
}
}
