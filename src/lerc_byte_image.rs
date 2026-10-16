//! Lerc2 byte Huffman images, including masked per-depth delta prediction.
use super::*;
use crate::lerc_header::Header;
use crate::lerc_mask as mask;
use crate::lerc_huffman as huff;
use crate::lerc_block::ResultModel;
verus! {
pub(super) open spec fn pixel(h:Header,delta:bool,k:int) -> int {if delta{k%(h.width as int*h.height)}else{k/(h.depth as int)}}
pub(super) open spec fn target(h:Header,delta:bool,k:int) -> int {if delta{pixel(h,delta,k)*h.depth+k/(h.width as int*h.height)}else{k}}
pub(super) open spec fn prediction(h:Header,delta:bool,k:int,mask:Seq<u8>,out:Seq<u64>,previous:u64) -> u64 {
    if !delta{0u64}else{
        let p=pixel(h,delta,k);let prev=if p==0{0u64}else{previous};
        if p%(h.width as int)>0 && mask::valid(mask,p-1){prev}
        else if p>=h.width && mask::valid(mask,p-h.width){out[target(h,delta,k)-h.width as int*h.depth]}
        else{prev}
    }
}
#[verifier::opaque]
pub(super) closed spec fn finish(s:Seq<u8>,h:Header,mask:Seq<u8>,base:usize,t:huff::TableModel,delta:bool,k:usize,p:usize,prev:u64,out:Seq<u64>,fuel:nat) -> Option<ResultModel> decreases fuel {
    if k as int==h.width as int*h.height*h.depth{
        let at=base as int+((p as int+31)/32+1)*4;
        if at>s.len(){None}else{Some(ResultModel {at:at as usize,data:out})}
    }else if fuel==0{None}else{
        let px=pixel(h,delta,k as int);let dest=target(h,delta,k as int);
        let previous=if delta && px==0{0u64}else{prev};
        if !mask::valid(mask,px){finish(s,h,mask,base,t,delta,(k+1) as usize,p,previous,out,(fuel-1) as nat)}else{
            let sym=huff::symbol(s,base,p,t);
            if sym.is_none(){None}else{
                let (v,next)=sym.unwrap();let predicted=prediction(h,delta,k as int,mask,out,prev);
                let value=((v+256-if h.kind==0{128int}else{0int}+predicted)%256) as u64;
                finish(s,h,mask,base,t,delta,(k+1) as usize,next,value,out.update(dest,value),(fuel-1) as nat)
            }
        }
    }
}
pub(super) closed spec fn decoded(s:Seq<u8>,h:Header,mask:Seq<u8>,start:usize,delta:bool,out:Seq<u64>) -> Option<ResultModel> {
    let table=huff::table(s,start);
    if table.is_none(){None}else{
        let t=table.unwrap();finish(s,h,mask,t.at,t.table,delta,0,0,0,out,(h.width as int*h.height*h.depth) as nat)
    }
}
pub(super) fn decode(s:&[u8],h:Header,valid:&[u8],start:usize,delta:bool,out:&mut[u64]) -> (r:Result<usize,Error>)
    requires h.invariant(),h.kind<2,start<=s.len(),s.len()<=usize::MAX/8,
        (h.width as int*h.height+7)/8<=valid.len(),h.width as int*h.height*h.depth<=old(out).len(),
        forall|i:int|0<=i<h.width as int*h.height*h.depth ==> old(out)@[i]<=255,
    ensures final(out).len()==old(out).len(),
        r.is_ok()==decoded(s@,h,valid@,start,delta,old(out)@).is_some(),
        r.is_ok() ==> r.unwrap()==decoded(s@,h,valid@,start,delta,old(out)@).unwrap().at && final(out)@==decoded(s@,h,valid@,start,delta,old(out)@).unwrap().data,
        r.is_ok() ==> start<r.unwrap()<=s.len(),
        forall|i:int|0<=i<h.width as int*h.height*h.depth ==> final(out)@[i]<=255,
        forall|j:int|h.width as int*h.height*h.depth<=j<old(out).len() ==> final(out)@[j]==old(out)@[j],
{
    reveal(decoded);
    let (base,table)=huff::read_table(s,start)?;
    assert(h.width*h.height*h.depth<=usize::MAX) by(nonlinear_arith)
        requires h.width*h.height<=usize::MAX/512,h.depth<=64;
    let pixels=h.width*h.height;let count=pixels*h.depth;
    assert(pixels>0) by(nonlinear_arith) requires pixels==h.width*h.height,h.width>0,h.height>0;
    let mut k=0usize;let mut p=0usize;let mut prev=0u64;
    let ghost goal=decoded(s@,h,valid@,start,delta,out@);
    while k<count
        invariant h.invariant(),h.kind<2,s.len()<=usize::MAX/8,start<base<=s.len(),table.valid(),
            huff::table(s@,start)==Some(huff::Parsed {at:base,table:table.model()}),
            pixels==h.width as int*h.height,count==pixels as int*h.depth,0<pixels,k<=count,p<=s.len()*8,prev<=255,
            (pixels+7)/8<=valid.len(),count<=out.len(),out.len()==old(out).len(),
            forall|i:int|0<=i<count ==> out@[i]<=255,
            forall|j:int|count<=j<out.len() ==> out@[j]==old(out)@[j],
            goal==decoded(s@,h,valid@,start,delta,old(out)@),
            goal==finish(s@,h,valid@,base,table.model(),delta,k,p,prev,out@,(count-k) as nat),
        decreases count-k,
    {
        reveal_with_fuel(finish,1);
        assert(k/h.depth<pixels && k/pixels<h.depth) by(nonlinear_arith) requires k<pixels*h.depth,pixels>0,h.depth>0;
        let px=if delta{k%pixels}else{k/h.depth};
        assert(px*h.depth+k/pixels<count) by(nonlinear_arith) requires px<pixels,k/pixels<h.depth,count==pixels*h.depth;
        let dest=if delta{px*h.depth+k/pixels}else{k};
        if delta && px==0{prev=0;}
        if mask::is_valid(valid,px){
            let (symbol,next)=huff::read_symbol(s,base,p,&table)?;
            let predicted=if !delta{0u64}else if px%h.width>0 && mask::is_valid(valid,px-1){prev}
                else if px>=h.width && mask::is_valid(valid,px-h.width){
                    assert(h.width*h.depth<=dest) by(nonlinear_arith) requires px>=h.width,dest==px*h.depth+k/pixels,h.depth>0,k/pixels>=0;
                    out[dest-h.width*h.depth]
                }else{prev};
            let v=(symbol as u64+256-if h.kind==0{128u64}else{0u64}+predicted)%256;
            out[dest]=v;prev=v;p=next;
        }
        k=k+1;
    }
    reveal_with_fuel(finish,1);
    let delta=(p/32+if p%32==0{1usize}else{2usize})*4;
    if delta>s.len()-base{return Err(Error::Truncated);}
    Ok(base+delta)
}
}
