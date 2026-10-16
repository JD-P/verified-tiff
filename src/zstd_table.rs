//! FSE table construction: low-probability tail, modular spreading and state
//! transition ranges. The model is immutable and independent of fixed arrays.
use super::*;
use crate::zstd_bits as zb;
use crate::zstd_fse as zf;
verus! {
pub(super) open spec fn magnitude(n:i16) -> int {if n<0 {-(n as int)}else{n as int}}
pub(super) open spec fn mass(c:Seq<i16>,n:nat) -> int decreases n {
    if n==0 {0}else{mass(c,(n-1) as nat)+magnitude(c[n as int-1])}
}
pub(super) open spec fn distribution_ok(c:Seq<i16>,log:usize) -> bool {
    c.len()==64 && 5<=log<=9 && (forall|i:int|0<=i<64 ==> -1<=#[trigger] c[i]<=512) && mass(c,64)==zb::pow(log as int)
}
struct SModel {high:int,table:Seq<u8>}
#[verifier::opaque]
spec fn lows(c:Seq<i16>,m:SModel,i:nat) -> Option<SModel> decreases 64-i {
    if i>=64 {Some(m)} else if c[i as int] == -1 {
        if m.high==0 {None} else {lows(c,SModel {high:m.high-1,table:m.table.update(m.high-1,i as u8)},i+1)}
    } else {lows(c,m,i+1)}
}
fn low_symbols(c:&[i16;64],size:usize) -> (r:Result<(usize,[u8;512]),Error>)
    requires 32<=size<=512,
    ensures r.is_ok()==lows(c@,SModel {high:size as int,table:Seq::new(512,|i:int|255u8)},0).is_some(),
        r.is_ok() ==> r.unwrap().0==lows(c@,SModel {high:size as int,table:Seq::new(512,|i:int|255u8)},0).unwrap().high,
        r.is_ok() ==> r.unwrap().1@==lows(c@,SModel {high:size as int,table:Seq::new(512,|i:int|255u8)},0).unwrap().table,
        r.is_ok() ==> r.unwrap().0<=size,
{
    let mut table=[255u8;512];let mut high=size;let mut i=0usize;
    assert(table@ =~= Seq::new(512,|i:int|255u8));
    let ghost goal=lows(c@,SModel {high:size as int,table:table@},0);
    while i<64
        invariant i<=64,high<=size<=512,size>=32,
            goal==lows(c@,SModel {high:high as int,table:table@},i as nat),
            goal==lows(c@,SModel {high:size as int,table:Seq::new(512,|i:int|255u8)},0),
        decreases 64-i,
    {
        reveal_with_fuel(lows,1);
        if c[i]==-1 {
            if high==0 {return Err(Error::Invalid);}
            high=high-1;table[high]=i as u8;
        }
        i=i+1;
    }
    reveal_with_fuel(lows,1);Ok((high,table))
}
// Advancing one symbol at a time keeps both model and implementation bounded
// even for hostile distributions containing long runs of zero probabilities.
struct Spread {pos:int,symbol:int,used:int,table:Seq<u8>}
#[verifier::opaque]
spec fn spread(c:Seq<i16>,size:int,high:int,m:Spread,ticks:nat) -> Option<Spread> decreases ticks {
    if ticks==0 {Some(m)} else if m.pos>=high {
        spread(c,size,high,Spread {pos:(m.pos+size/2+size/8+3)%size,symbol:m.symbol,used:m.used,table:m.table},(ticks-1) as nat)
    } else {
        let symbol=next_positive(c,m.symbol as nat,m.used);
        if symbol>=64 {None} else {
            let used=if symbol==m.symbol {m.used}else{0int};
            let t=m.table.update(m.pos,symbol as u8);
            spread(c,size,high,Spread {pos:(m.pos+size/2+size/8+3)%size,symbol,used:used+1,table:t},(ticks-1) as nat)
        }
    }
}
spec fn next_positive(c:Seq<i16>,i:nat,used:int) -> int decreases 64-i {
    if i>=64 {64} else if c[i as int]>used {i as int}else{next_positive(c,i+1,0)}
}
fn positive(c:&[i16;64],start:usize,used:usize) -> (r:usize)
    requires start<=64,used<=512,
    ensures start<=r<=64,r==next_positive(c@,start as nat,used as int),
        r<64 ==> c@[r as int]>(if r==start {used as int}else{0int}),
{
    let mut i=start;let mut n=used;
    while i<64
        invariant start<=i<=64,n<=512,n==if i==start{used}else{0},
            next_positive(c@,start as nat,used as int)==next_positive(c@,i as nat,n as int),
        decreases 64-i,
    {
        reveal_with_fuel(next_positive,1);
        if c[i] as i32>n as i32{return i;}
        i=i+1;n=0;
    }
    reveal_with_fuel(next_positive,1);i
}
fn spread_symbols(c:&[i16;64],size:usize,high:usize,initial:[u8;512]) -> (r:Result<[u8;512],Error>)
    requires 32<=size<=512,high<=size,forall|i:int|0<=i<64 ==> c@[i]<=512,
    ensures r.is_ok()==spread(c@,size as int,high as int,Spread {pos:0,symbol:0,used:0,table:initial@},size as nat).is_some(),
        r.is_ok() ==> r.unwrap()@==spread(c@,size as int,high as int,Spread {pos:0,symbol:0,used:0,table:initial@},size as nat).unwrap().table,
{
    let mut table=initial;let mut symbol=0usize;let mut used=0usize;let mut pos=0usize;let mut ticks=size;
    let ghost goal=spread(c@,size as int,high as int,Spread {pos:0,symbol:0,used:0,table:initial@},size as nat);
    while ticks>0
        invariant ticks<=size,32<=size<=512,high<=size,pos<size,symbol<=64,used<=512,
            forall|i:int|0<=i<64 ==> c@[i]<=512,
            goal==spread(c@,size as int,high as int,Spread {pos:pos as int,symbol:symbol as int,used:used as int,table:table@},ticks as nat),
            goal==spread(c@,size as int,high as int,Spread {pos:0,symbol:0,used:0,table:initial@},size as nat),
        decreases ticks,
    {
        reveal_with_fuel(spread,1);
        if pos<high {
            let sym=positive(c,symbol,used);if sym>=64{return Err(Error::Invalid);}
            if sym!=symbol{used=0;}
            symbol=sym;table[pos]=sym as u8;used=used+1;
        }
        pos=(pos+size/2+size/8+3)%size;ticks=ticks-1;
    }
    reveal_with_fuel(spread,1);Ok(table)
}
pub(super) closed spec fn symbols(c:Seq<i16>,log:usize) -> Option<Seq<u8>> {
    if !distribution_ok(c,log) {None} else {
        let size=zb::pow(log as int);let low=lows(c,SModel {high:size,table:Seq::new(512,|i:int|255u8)},0);
        if low.is_none(){None}else{
            let l=low.unwrap();let a=spread(c,size,l.high,Spread {pos:0,symbol:0,used:0,table:l.table},size as nat);
            if a.is_none(){None}else{Some(a.unwrap().table)}
        }
    }
}
pub(super) open spec fn rank(s:Seq<u8>,symbol:u8,n:nat) -> int decreases n {
    if n==0 {0}else{rank(s,symbol,(n-1) as nat)+if s[n as int-1]==symbol{1int}else{0int}}
}
proof fn rank_bound(s:Seq<u8>,symbol:u8,n:nat)
    requires n<=s.len(),
    ensures 0<=rank(s,symbol,n)<=n,
    decreases n,
{if n>0{rank_bound(s,symbol,(n-1) as nat);}}
#[derive(Clone,Copy)]
pub(super) struct Cell {pub symbol:u8,pub bits:u8,pub base:u16}
pub(super) open spec fn parameters_ok(symbol:u8,count:int,log:usize) -> bool {
    &&& symbol<64 && 0<count<=1023 && zf::floor(count)<=log
    &&& zb::pow(log as int)<=count*zb::pow(log as int-zf::floor(count))
    &&& count*zb::pow(log as int-zf::floor(count))+zb::pow(log as int-zf::floor(count))<=2*zb::pow(log as int)
}
pub(super) open spec fn cell_value(symbol:u8,count:int,log:usize) -> Cell {
    let bits=log as int-zf::floor(count);
    Cell {symbol,bits:bits as u8,base:(count*zb::pow(bits)-zb::pow(log as int)) as u16}
}
pub(super) open spec fn cell_model(c:Seq<i16>,s:Seq<u8>,log:usize,i:int) -> Cell {
    cell_value(s[i],magnitude(c[s[i] as int])+rank(s,s[i],i as nat),log)
}
pub(super) open spec fn cell_valid(v:Cell,log:usize) -> bool {
    v.symbol<64 && v.bits<=log && v.base as int+zb::pow(v.bits as int)<=zb::pow(log as int)
}
spec fn entry_ok(c:Seq<i16>,s:Seq<u8>,log:usize,i:int) -> bool {
    s[i]<64 && parameters_ok(s[i],magnitude(c[s[i] as int])+rank(s,s[i],i as nat),log)
}
pub(super) closed spec fn table_model(c:Seq<i16>,log:usize) -> Option<Seq<Cell>> {
    let ss=symbols(c,log);
    if ss.is_none(){None}else{
        let s=ss.unwrap();let size=zb::pow(log as int);
        if forall|i:int|0<=i<size ==> entry_ok(c,s,log,i) {
            Some(Seq::new(512,|i:int|if i<size {cell_model(c,s,log,i)}else{Cell {symbol:0,bits:0,base:0}}))
        }else{None}
    }
}
fn make_cell(symbol:u8,count:usize,log:usize) -> (r:Result<Cell,Error>)
    requires log<=9,count<=1024,
    ensures r.is_ok()==parameters_ok(symbol,count as int,log),
        r.is_ok() ==> r.unwrap()==cell_value(symbol,count as int,log) && cell_valid(r.unwrap(),log),
{
    if symbol>=64 || count==0 || count>1023{return Err(Error::Invalid);}
    let floor=zf::log(count);if floor>log{return Err(Error::Invalid);}
    let bits=log-floor;let range=zb::power(bits);let size=zb::power(log);
    assert(range<=512 && size<=512);
    assert((count as int)*range<=523776) by(nonlinear_arith) requires count<=1023,range<=512;
    let full=(count as u64)*range;
    if full<size || full+range>2*size{return Err(Error::Invalid);}
    Ok(Cell {symbol,bits:bits as u8,base:(full-size) as u16})
}
pub(super) struct TableModel {pub cells:Seq<Cell>,pub log:usize,pub ready:bool}
#[derive(Clone,Copy)]
pub(super) struct Table {pub cells:[Cell;512],pub log:usize,pub ready:bool}
impl Table {
    pub(super) open spec fn model(self) -> TableModel {TableModel {cells:self.cells@,log:self.log,ready:self.ready}}
    pub(super) open spec fn valid(self) -> bool {
        self.log<=9 && (forall|i:int|0<=i<zb::pow(self.log as int) ==> cell_valid(self.cells@[i],self.log))
    }
    pub(super) fn empty() -> (r:Self)
        ensures r.valid(),!r.ready,r.log==0,
            forall|i:int|0<=i<512 ==> r.cells@[i]==(Cell {symbol:0,bits:0,base:0}),
    { Self {cells:[Cell {symbol:0,bits:0,base:0};512],log:0,ready:false} }
    pub(super) fn repeat(symbol:u8) -> (r:Result<Self,Error>)
        ensures r.is_ok()==(symbol<64),r.is_ok() ==> r.unwrap().valid() && r.unwrap().ready && r.unwrap().log==0,
            r.is_ok() ==> forall|i:int|0<=i<512 ==> r.unwrap().cells@[i]==(Cell {symbol,bits:0,base:0}),
    {
        if symbol>=64{return Err(Error::Invalid);}
        Ok(Self {cells:[Cell {symbol,bits:0,base:0};512],log:0,ready:true})
    }
    pub(super) fn build(c:&[i16;64],log:usize) -> (r:Result<Self,Error>)
        ensures r.is_ok()==table_model(c@,log).is_some(),
            r.is_ok() ==> r.unwrap().valid() && r.unwrap().ready && r.unwrap().log==log,
            r.is_ok() ==> r.unwrap().cells@==table_model(c@,log).unwrap(),
    {
        reveal(table_model);reveal(symbols);
        if log<5 || log>9{return Err(Error::Unsupported);}
        let size=zb::power(log) as usize;
        let mut total=0usize;let mut next=[0usize;64];let mut i=0usize;
        while i<64
            invariant i<=64,5<=log<=9,size==zb::pow(log as int),32<=size<=512,total<=i*512,
                total==mass(c@,i as nat),
                forall|k:int|0<=k<i ==> -1<=#[trigger] c@[k]<=512 && next@[k]==magnitude(c@[k]),
            decreases 64-i,
        {
            let n=c[i];if n< -1 || n>512{return Err(Error::Invalid);}
            let v=if n<0{1usize}else{n as usize};next[i]=v;total=total+v;i=i+1;
            reveal_with_fuel(mass,1);
        }
        if total!=size{return Err(Error::Invalid);}
        let (high,initial)=low_symbols(c,size)?;
        let syms=spread_symbols(c,size,high,initial)?;
        let mut cells=[Cell {symbol:0,bits:0,base:0};512];
        let ghost expected=table_model(c@,log);
        assert(symbols(c@,log)==Some(syms@));
        i=0;
        assert forall|k:int|0<=k<64 implies next@[k]==magnitude(c@[k])+rank(syms@,k as u8,0) by {reveal_with_fuel(rank,1);}
        while i<size
            invariant i<=size,5<=log<=9,size==zb::pow(log as int),32<=size<=512,
                symbols(c@,log)==Some(syms@),expected==table_model(c@,log),
                forall|k:int|0<=k<64 ==> -1<=#[trigger] c@[k]<=512 && next@[k]==magnitude(c@[k])+rank(syms@,k as u8,i as nat),
                forall|k:int|0<=k<i ==> #[trigger] entry_ok(c@,syms@,log,k),
                forall|k:int|0<=k<i ==> cells@[k]==cell_model(c@,syms@,log,k),
                forall|k:int|0<=k<i ==> cell_valid(cells@[k],log),
                forall|k:int|i<=k<512 ==> cells@[k]==(Cell {symbol:0,bits:0,base:0}),
            decreases size-i,
        {
            let sym=syms[i];
            if sym>=64 {assert(!entry_ok(c@,syms@,log,i as int));return Err(Error::Invalid);}
            proof {rank_bound(syms@,sym,i as nat);}
            let count=next[sym as usize];
            let cell=match make_cell(sym,count,log) {Ok(v)=>v,Err(e)=>{assert(!entry_ok(c@,syms@,log,i as int));reveal(table_model);return Err(e);}};
            assert(entry_ok(c@,syms@,log,i as int));
            assert(cell==cell_model(c@,syms@,log,i as int));
            cells[i]=cell;next[sym as usize]=count+1;i=i+1;
            assert forall|k:int|0<=k<64 implies next@[k]==magnitude(c@[k])+rank(syms@,k as u8,i as nat) by {reveal_with_fuel(rank,1);}
        }
        assert(forall|k:int|0<=k<size ==> entry_ok(c@,syms@,log,k));
        reveal(table_model);assert(table_model(c@,log).is_some());
        assert(cells@ =~= table_model(c@,log).unwrap());
        Ok(Self {cells,log,ready:true})
    }

}
}
