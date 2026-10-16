//! Ordered VP8 key-frame header grammar, with explicit defaults for absent fields.
use super::*;
use crate::vp8_bool as ar;
use crate::vp8_fields::{self as fields,Field};
use crate::vp8_tables::*;
verus! {
pub(super) open spec fn descriptor(i:usize,d:Seq<i32>) -> Field {
let base=Field {kind:0,n:0,prob:128,default:0};
    if i==0 || i==1 || i==2 || i==17 || i==20 || i==37 || i==1094{Field {kind:1,..base}}
    else if i==3 || i==4{Field {kind:if d[2]!=0{1}else{0},..base}}
    else if i==5{Field {kind:if d[4]!=0{1}else{0},default:1,..base}}
    else if 6<=i && i<14{Field {kind:if d[4]!=0{4}else{0},n:if i<10{7}else{6},..base}}
    else if 14<=i && i<17{Field {kind:if d[3]!=0{3}else{0},n:8,default:255,..base}}
    else if i==18{Field {kind:2,n:6,..base}}
    else if i==19{Field {kind:2,n:3,..base}}
    else if i==21{Field {kind:if d[20]!=0{1}else{0},..base}}
    else if 22<=i && i<30{Field {kind:if d[21]!=0{4}else{0},n:6,..base}}
    else if i==30{Field {kind:2,n:2,..base}}
    else if i==31{Field {kind:2,n:7,..base}}
    else if 32<=i && i<37{Field {kind:4,n:4,..base}}
    else if 38<=i && i<1094{Field {kind:3,n:8,prob:COEFF_UPDATE[(i as int)-38],default:COEFF_DEFAULT[(i as int)-38] as i32}}
    else if i==1095{Field {kind:if d[1094]!=0{2}else{0},n:8,..base}}
    else{base}
}
fn plan(i:usize,d:&[i32;1096]) -> (f:Field)
    requires i<1096,
    ensures f==descriptor(i,d@),f.valid(),
{
let base=Field {kind:0,n:0,prob:128,default:0};
    if i==0 || i==1 || i==2 || i==17 || i==20 || i==37 || i==1094{Field {kind:1,..base}}
    else if i==3 || i==4{Field {kind:if d[2]!=0{1}else{0},..base}}
    else if i==5{Field {kind:if d[4]!=0{1}else{0},default:1,..base}}
    else if 6<=i && i<14{Field {kind:if d[4]!=0{4}else{0},n:if i<10{7}else{6},..base}}
    else if 14<=i && i<17{Field {kind:if d[3]!=0{3}else{0},n:8,default:255,..base}}
    else if i==18{Field {kind:2,n:6,..base}}
    else if i==19{Field {kind:2,n:3,..base}}
    else if i==21{Field {kind:if d[20]!=0{1}else{0},..base}}
    else if 22<=i && i<30{Field {kind:if d[21]!=0{4}else{0},n:6,..base}}
    else if i==30{Field {kind:2,n:2,..base}}
    else if i==31{Field {kind:2,n:7,..base}}
    else if 32<=i && i<37{Field {kind:4,n:4,..base}}
    else if 38<=i && i<1094{Field {kind:3,n:8,prob:COEFF_UPDATE[i-38],default:COEFF_DEFAULT[i-38] as i32}}
    else if i==1095{Field {kind:if d[1094]!=0{2}else{0},n:8,..base}}
    else{base}
}
#[verifier::opaque]
pub(super) open spec fn finish(s:Seq<u8>,r:ar::Reader,d:Seq<i32>,i:usize,fuel:nat) -> Option<(ar::Reader,Seq<i32>)> decreases fuel {
    if i==1096{Some((r,d))}else if fuel==0{None}else{
        let v=fields::field(s,r,descriptor(i,d));
        if v.is_none(){None}else{finish(s,v.unwrap().1,d.update(i as int,v.unwrap().0),(i+1) as usize,(fuel-1) as nat)}
    }
}
pub(super) fn parse(s:&[u8],r:ar::Reader,d:&mut[i32;1096]) -> (out:Result<ar::Reader,Error>)
    requires r.valid(s@),
    ensures out.is_ok()==finish(s@,r,old(d)@,0,1096).is_some(),
        out.is_ok() ==> out.unwrap()==finish(s@,r,old(d)@,0,1096).unwrap().0 && final(d)@==finish(s@,r,old(d)@,0,1096).unwrap().1 && out.unwrap().valid(s@),
        out.is_ok() ==> forall|i:int|0<=i<1096 ==> -255<=#[trigger] final(d)@[i]<=255,
{
    let ghost goal=finish(s@,r,d@,0,1096);let mut t=r;let mut i=0usize;
    loop
        invariant goal==finish(s@,r,old(d)@,0,1096),t.valid(s@),i<=1096,goal==finish(s@,t,d@,i,(1096-i) as nat),
            forall|j:int|0<=j<i ==> -255<=#[trigger] d@[j]<=255,
        decreases 1096-i,
    {
        reveal_with_fuel(finish,1);if i==1096{return Ok(t);}
        let f=plan(i,d);let (v,after)=fields::parse(s,t,f)?;d[i]=v;t=after;i=i+1;
    }
}
}
