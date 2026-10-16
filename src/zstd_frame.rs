//! Dictionary-free single Zstandard frames. Header, block ranges, final marker
//! and optional checksum location are validated before entropy decoding.
use super::*;
use crate::native_samples as ns;
verus! {
#[derive(Clone,Copy)]
pub(super) struct Header {pub at:usize,pub window:usize,pub known:bool,pub size:u64,pub checksum:bool}
pub(super) open spec fn window_model(w:u8) -> int {
    let base=(1u64<<((w/8) as u32+10)) as int;base+base/8*(w as int%8)
}
fn window(w:u8) -> (r:u64)
    ensures r==window_model(w),1024<=r<=4123168604160,
{
    let shift=(w/8) as u32+10;let base=1u64<<shift;
    assert(1024<=base<=2199023255552) by(bit_vector) requires 10<=shift<=41,base==1u64<<shift;
    assert((base as int/8)*(w as int%8)<=1924145348608) by(nonlinear_arith) requires base<=2199023255552,w%8<=7;
    let add=(base/8)*((w%8) as u64);
    assert(base as int+add<=4123168604160) by(nonlinear_arith) requires base<=2199023255552,add==base/8*(w%8),w%8<=7;
    base+add
}
pub(super) closed spec fn header(s:Seq<u8>) -> Option<Header> {
    if s.len()<5 || word32(s,0,true)!=4247762216 || s[4]%16>=8 {None}else{
        let f=s[4] as int;let single=(f/32)%2==1;let flag=f/64;
        let dl=if f%4==3{4usize}else{(f%4) as usize};
        let fl=if flag==0{if single{1usize}else{0usize}}else if flag==1{2usize}else if flag==2{4usize}else{8usize};
        let start=if single{5int}else{6int};
        if start+dl+fl>s.len(){None}else{
            let dict=if dl==0{0int}else{ns::word(s,start,dl,true)};
            let content=if fl==0{0int}else{ns::word(s,start+dl,fl,true)+if fl==2{256int}else{0int}};
            let win=if single{content}else{window_model(s[5])};
            if dict!=0 || win>usize::MAX {None}else{
                Some(Header {at:(start+dl+fl) as usize,window:win as usize,known:fl!=0,size:content as u64,checksum:(f/4)%2==1})
            }
        }
    }
}
pub(super) fn read_header(s:&[u8]) -> (r:Result<Header,Error>)
    ensures r.is_ok()==header(s@).is_some(),r.is_ok() ==> r.unwrap()==header(s@).unwrap(),
        r.is_ok() ==> 5<=r.unwrap().at<=s.len(),
{
    reveal(header);
    if s.len()<5 || read32(s,0,true)?!=4247762216u32 || s[4]%16>=8{return Err(Error::Invalid);}
    let f=s[4] as usize;let single=(f/32)%2==1;let flag=f/64;
    let dl=if f%4==3{4usize}else{f%4};
    let fl=if flag==0{if single{1usize}else{0usize}}else if flag==1{2}else if flag==2{4}else{8};
    let start=if single{5usize}else{6};
    if start+dl+fl>s.len(){return Err(Error::Truncated);}
    let dict=if dl==0{0u64}else{ns::read_word(s,start,dl,true)?};
    let value=if fl==0{0u64}else{ns::read_word(s,start+dl,fl,true)?};
    let content=if fl==2{value+256}else{value};
    let win=if single{content}else{window(s[5])};
    if dict!=0{return Err(Error::Unsupported);}
    if win>usize::MAX as u64{return Err(Error::Limit);}
    Ok(Header {at:start+dl+fl,window:win as usize,known:fl!=0,size:content,checksum:(f/4)%2==1})
}
#[derive(Clone,Copy)]
pub(super) struct Block {pub at:usize,pub next:usize,pub size:usize,pub kind:usize,pub last:bool}
pub(super) closed spec fn block(s:Seq<u8>,at:int,window:usize) -> Option<Block> {
    if at+3>s.len(){None}else{
        let v=s[at] as int+256*s[at+1] as int+65536*s[at+2] as int;
        let kind=(v/2)%4;let size=v/8;let stored=if kind==1{1int}else{size};
        if kind==3 || size>131072 || size>window || at+3+stored>s.len(){None}else{
            Some(Block {at:(at+3) as usize,next:(at+3+stored) as usize,size:size as usize,kind:kind as usize,last:v%2==1})
        }
    }
}
pub(super) fn read_block(s:&[u8],at:usize,window:usize) -> (r:Result<Block,Error>)
    requires at<=s.len(),
    ensures r.is_ok()==block(s@,at as int,window).is_some(),r.is_ok() ==> r.unwrap()==block(s@,at as int,window).unwrap(),
        r.is_ok() ==> at<r.unwrap().at<=r.unwrap().next<=s.len() && r.unwrap().kind<3 && r.unwrap().size<=131072 && r.unwrap().size<=window,
        r.is_ok() ==> r.unwrap().kind==1 ==> r.unwrap().next==r.unwrap().at+1,
        r.is_ok() ==> r.unwrap().kind!=1 ==> r.unwrap().next==r.unwrap().at+r.unwrap().size,
{
    reveal(block);
    if s.len()-at<3{return Err(Error::Truncated);}
    let v=s[at] as usize+256*(s[at+1] as usize)+65536*(s[at+2] as usize);
    let kind=(v/2)%4;let size=v/8;let stored=if kind==1{1usize}else{size};
    if kind==3 || size>131072 || size>window || stored>s.len()-at-3{return Err(Error::Invalid);}
    Ok(Block {at:at+3,next:at+3+stored,size,kind,last:v%2==1})
}
#[derive(Clone,Copy)]
pub(super) struct Plan {pub header:Header,pub footer:usize,pub blocks:usize,pub compressed:usize}
#[verifier::opaque]
spec fn finish(s:Seq<u8>,h:Header,at:int,blocks:int,compressed:int,fuel:nat) -> Option<Plan> decreases fuel {
    if fuel==0{None}else{
        let b=block(s,at,h.window);
        if b.is_none(){None}else{
            let v=b.unwrap();let c=compressed+if v.kind==2{1int}else{0int};
            if v.last{
                if v.next as int+if h.checksum{4int}else{0int}!=s.len(){None}else{
                    Some(Plan {header:h,footer:v.next,blocks:(blocks+1) as usize,compressed:c as usize})
                }
            }else{finish(s,h,v.next as int,blocks+1,c,(fuel-1) as nat)}
        }
    }
}
pub(super) closed spec fn plan(s:Seq<u8>) -> Option<Plan> {
    let h=header(s);
    if h.is_none(){None}else{finish(s,h.unwrap(),h.unwrap().at as int,0,0,s.len())}
}
pub(super) fn inspect(s:&[u8]) -> (r:Result<Plan,Error>)
    ensures r.is_ok()==plan(s@).is_some(),r.is_ok() ==> r.unwrap()==plan(s@).unwrap(),
        r.is_ok() ==> 1<=r.unwrap().blocks<=s.len() && r.unwrap().compressed<=r.unwrap().blocks,
        r.is_ok() ==> r.unwrap().header.at<=r.unwrap().footer<=s.len(),
        r.is_ok() ==> r.unwrap().footer as int+if r.unwrap().header.checksum{4int}else{0int}==s.len(),
{
    reveal(plan);
    let h=read_header(s)?;let mut at=h.at;let mut blocks=0usize;let mut compressed=0usize;let mut fuel=s.len();
    let ghost goal=plan(s@);
    loop
        invariant h.at<=at<=s.len(),5<=h.at,compressed<=blocks,blocks+fuel==s.len(),
            goal==plan(s@),goal==finish(s@,h,at as int,blocks as int,compressed as int,fuel as nat),
        decreases fuel,
    {
        reveal_with_fuel(finish,1);
        if fuel==0{return Err(Error::Invalid);}
        let b=read_block(s,at,h.window)?;
        blocks=blocks+1;if b.kind==2{compressed=compressed+1;}
        at=b.next;fuel=fuel-1;
        if b.last{
            let trailer=if h.checksum{4usize}else{0};
            if s.len()-at!=trailer{return Err(Error::Invalid);}
            return Ok(Plan {header:h,footer:at,blocks,compressed});
        }
    }
}

pub(super) open spec fn witness_bytes() -> Seq<u8> {seq![40u8,181u8,47u8,253u8,32u8,2u8,17u8,0u8,0u8,65u8,66u8]}
pub(super) proof fn raw_witness()
    ensures plan(witness_bytes())==Some(Plan {header:Header {at:6,window:2,known:true,size:2,checksum:false},footer:11,blocks:1,compressed:0}),
        block(witness_bytes(),6,2)==Some(Block {at:9,next:11,size:2,kind:0,last:true}),
{
    ns::byte_word(witness_bytes(),5,true);reveal(plan);reveal(header);reveal(block);reveal_with_fuel(finish,1);
}
}
