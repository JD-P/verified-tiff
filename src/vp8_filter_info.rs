//! Segment, reference, mode and sharpness contributions to VP8 loop filtering.
use super::*;
verus! {
#[derive(Clone,Copy)]
pub struct Filter {pub limit:u8,pub inner:u8,pub threshold:u8,pub filter_inner:bool,pub simple:bool}
impl Filter {
    pub fn empty() -> (r:Self) ensures r.limit==0,r.inner==0,r.threshold==0,!r.filter_inner,!r.simple,
    {Self {limit:0,inner:0,threshold:0,filter_inner:false,simple:false}}
}
pub(super) open spec fn clamp(v:int,max:int) -> int {if v<0{0}else if v>max{max}else{v}}
fn clip(v:i32,max:i32) -> (r:i32) requires 0<=max<=255,ensures r==clamp(v as int,max as int),0<=r<=max,
{if v<0{0}else if v>max{max}else{v}}
pub(super) open spec fn strength(h:Seq<i32>,segment:usize,small:bool,empty:bool) -> Filter {
    let base=if h[2]!=0{h[10+segment] as int+if h[5]==0{h[18] as int}else{0int}}else{h[18] as int};
    let extra=if h[20]!=0{h[22] as int+if small{h[26] as int}else{0int}}else{0int};
    let level=clamp(base+extra,63);let sharp=clamp(h[19] as int,7);
    let initial=if sharp>4{level/4}else if sharp>0{level/2}else{level};
    let bounded=if sharp>0 && initial>9-sharp{9-sharp}else{initial};let inner=if bounded<1{1int}else{bounded};
    Filter {limit:if h[18]==0 || level==0{0u8}else{(2*level+inner) as u8},inner:inner as u8,
        threshold:if level>=40{2u8}else if level>=15{1u8}else{0u8},filter_inner:small || !empty,simple:h[17]!=0}
}
pub(super) fn prepare(h:&[i32;1096],segment:usize,small:bool,empty:bool) -> (r:Filter)
    requires segment<4,forall|i:int|0<=i<1096 ==> -255<=#[trigger] h@[i]<=255,
    ensures r==strength(h@,segment,small,empty),r.limit<=189,r.inner<=63,r.threshold<=2,
{
    let base=if h[2]!=0{h[10+segment]+if h[5]==0{h[18]}else{0}}else{h[18]};
    let extra=if h[20]!=0{h[22]+if small{h[26]}else{0}}else{0};
    let level=clip(base+extra,63);let sharp=clip(h[19],7);
    let initial=if sharp>4{level/4}else if sharp>0{level/2}else{level};
    let bounded=if sharp>0 && initial>9-sharp{9-sharp}else{initial};let inner=if bounded<1{1}else{bounded};
    Filter {limit:if h[18]==0 || level==0{0}else{(2*level+inner) as u8},inner:inner as u8,
        threshold:if level>=40{2}else if level>=15{1}else{0},filter_inner:small || !empty,simple:h[17]!=0}
}
}
