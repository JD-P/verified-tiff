// Whole CCITT segment model and allocation-free two-row refinement.
use super::*;
use crate::fax_bits::Bits;
use crate::fax_row as fr;
use crate::fax_frame as ff;
use crate::fax_frame::Config;
verus! {
pub(super) open spec fn row_ok(ends:Seq<u32>,w:usize) -> bool {
    0<ends.len()<=65538 && ends.last()==w &&
    (forall|i:int| 0<=i<ends.len() ==> #[trigger] ends[i]<=w) &&
    (forall|i:int,j:int| 0<=i<=j<ends.len() ==> #[trigger] ends[i]<=#[trigger] ends[j])
}
#[verifier::opaque]
pub(super) open spec fn pixel(ends:Seq<u32>,x:int,n:nat) -> u8 decreases n {
    if n==0 {0u8} else {
        let lo=if n==1 {0int} else {ends[n as int-2] as int};
        if x>=lo && x<ends[n as int-1] {if (n-1)%2==0 {0u8} else {255u8}}
        else {pixel(ends,x,(n-1) as nat)}
    }
}
proof fn pixel_interval(ends:Seq<u32>,x:int,j:nat,n:nat)
    requires j<n<=ends.len(),0<=x<ends[j as int],
        j==0 || ends[j as int-1]<=x,
        forall|a:int,b:int| 0<=a<=b<ends.len() ==> #[trigger] ends[a]<=#[trigger] ends[b],
    ensures pixel(ends,x,n)==if j%2==0 {0u8} else {255u8},
    decreases n-j,
{
    reveal_with_fuel(pixel,1);
    if n>j+1 {assert(ends[j as int]<=ends[n as int-2]);pixel_interval(ends,x,j,(n-1) as nat);}
}
pub(super) fn render(ends:&[u32;65538],n:usize,w:usize,base:usize,output:&mut [u8]) -> (r:Result<(),Error>)
    requires n<=65538,row_ok(ends@.subrange(0,n as int),w),base as int+w<=old(output).len(),
    ensures r.is_ok(),final(output).len()==old(output).len(),
        forall|k:int| 0<=k<w ==> final(output)@[base as int+k]==pixel(ends@.subrange(0,n as int),k,n as nat),
        forall|k:int| 0<=k<old(output).len() && !(base<=k<base as int+w) ==> final(output)@[k]==old(output)@[k],
{
    let mut j=0usize;let mut x=0usize;let ghost rows=ends@.subrange(0,n as int);
    // Restating the proved bounds lets the compiler drop per-pixel checks.
    if output.len()<w || base>output.len()-w {return Ok(());}
    while j<n
        invariant j<=n,n<=65538,x<=w,base as int+w<=output.len(),output.len()==old(output).len(),
            rows==ends@.subrange(0,n as int),row_ok(rows,w),x==if j==0 {0int} else {ends@[j as int-1] as int},
            forall|k:int| 0<=k<x ==> output@[base as int+k]==pixel(rows,k,n as nat),
            forall|k:int| 0<=k<output.len() && !(base<=k<base as int+x) ==> output@[k]==old(output)@[k],
        decreases n-j,
    {
        let end=ends[j] as usize;let v=if j%2==0 {0u8} else {255u8};
        assert(rows[j as int]<=w);if j>0 {assert(rows[j as int-1]<=rows[j as int]);}assert(x<=end<=w);
        let end=if end>w {w} else {end};
        while x<end
            invariant j<n,n<=65538,x<=end<=w,base as int+w<=output.len(),output.len()==old(output).len(),
                rows==ends@.subrange(0,n as int),row_ok(rows,w),end==ends@[j as int],v==if j%2==0 {0u8} else {255u8},
                j==0 || ends@[j as int-1]<=x,
                forall|k:int| 0<=k<x ==> output@[base as int+k]==pixel(rows,k,n as nat),
                forall|k:int| 0<=k<output.len() && !(base<=k<base as int+x) ==> output@[k]==old(output)@[k],
            decreases end-x,
        {
            proof {pixel_interval(rows,x as int,j as nat,n as nat);}
            output[base+x]=v;x=x+1;
        }
        j=j+1;
    }
    assert(x==w);Ok(())
}
pub(super) struct Model {pub bits:Bits,pub reference:Seq<u32>,pub history:Seq<Seq<u32>>}
pub(super) open spec fn initial(b:Bits,w:usize) -> Model {Model {bits:b,reference:seq![w as u32],history:Seq::empty()}}
#[verifier::opaque]
pub(super) open spec fn step(s:Seq<u8>,c:Config,w:usize,m:Model) -> Option<Model> {
    let a=ff::line(s,m.bits,c);if a.is_none() {None} else {let (q,td)=a.unwrap();
        // A T.4 mixed-mode segment starts with an independently coded 1D line.
        if c.mixed && !c.group4 && m.history.len()==0 && td {None} else {
            let r=fr::finish(s,m.reference,w,td,fr::begin(q),(q.end-q.pos+1) as nat);
            if r.is_none() {None} else {let out=r.unwrap();Some(Model {bits:out.bits,reference:out.ends,history:m.history.push(out.ends)})}
        }
    }
}
#[verifier::opaque]
pub(super) open spec fn finish(s:Seq<u8>,c:Config,w:usize,m:Model,n:nat) -> Option<Model> decreases n {
    if n==0 {if ff::terminated(s,m.bits,c) {Some(m)} else {None}}
    else {let a=step(s,c,w,m);if a.is_none() {None} else {finish(s,c,w,a.unwrap(),(n-1) as nat)}}
}
pub(super) proof fn prefix(s:Seq<u8>,c:Config,w:usize,m:Model,n:nat)
    requires finish(s,c,w,m,n).is_some(),
    ensures finish(s,c,w,m,n).unwrap().history.len()==m.history.len()+n,
        forall|j:int| 0<=j<m.history.len() ==> finish(s,c,w,m,n).unwrap().history[j]==m.history[j],
    decreases n,
{
    reveal_with_fuel(finish,1);
    if n>0 {reveal(step);let a=step(s,c,w,m).unwrap();prefix(s,c,w,a,(n-1) as nat);
        assert forall|j:int| 0<=j<m.history.len() implies finish(s,c,w,m,n).unwrap().history[j]==m.history[j] by {
            assert(a.history[j]==m.history[j]);assert(finish(s,c,w,a,(n-1) as nat).unwrap().history[j]==a.history[j]);
        }
    }
}
pub(super) open spec fn output_byte(s:Seq<u8>,c:Config,w:usize,b:Bits,h:usize,k:int) -> u8 {
    let ends=finish(s,c,w,initial(b,w),h as nat).unwrap().history[k/(w as int)];pixel(ends,k%(w as int),ends.len())
}
pub(super) fn process(s:&[u8],c:Config,w:usize,h:usize,b:Bits,output:&mut [u8],write:bool) -> (r:Result<(),Error>)
    requires b.valid(s@),b.end<usize::MAX,0<w<=65536,h>0,w as int*h<=usize::MAX,
        write ==> w as int*h<=old(output).len(),
    ensures r.is_ok()==finish(s@,c,w,initial(b,w),h as nat).is_some(),
        final(output).len()==old(output).len(),
        r.is_ok() && write ==> forall|k:int| 0<=k<w as int*h ==> final(output)@[k]==output_byte(s@,c,w,b,h,k),
        forall|k:int| 0<=k<old(output).len() && (!write || k>=w as int*h) ==> final(output)@[k]==old(output)@[k],
{
    let mut a=[0u32;65538];let mut z=[0u32;65538];a[0]=w as u32;
    let mut use_a=true;let mut n=1usize;let mut bits=b;let mut row=0usize;
    let ghost result=finish(s@,c,w,initial(b,w),h as nat);
    let ghost mut model=initial(b,w);
    assert(a@.subrange(0,1) =~= seq![w as u32]);
    while row<h
        invariant row<=h,w as int*row<=w as int*h,0<w<=65536,h>0,w as int*h<=usize::MAX,bits.valid(s@),bits.end==b.end,b.end<usize::MAX,
            n<=65538,model.bits==bits,model.history.len()==row,
            model.reference==if use_a {a@.subrange(0,n as int)} else {z@.subrange(0,n as int)},
            result==finish(s@,c,w,initial(b,w),h as nat),result==finish(s@,c,w,model,(h-row) as nat),
            output.len()==old(output).len(),write ==> w as int*h<=output.len(),
            result.is_some() && write ==> forall|k:int| 0<=k<w as int*row ==> output@[k]==output_byte(s@,c,w,b,h,k),
            forall|k:int| 0<=k<output.len() && (!write || k>=w as int*row) ==> output@[k]==old(output)@[k],
        decreases h-row,
    {
        reveal_with_fuel(finish,1);reveal(step);
        let (q,td)=ff::begin_line(s,bits,c)?;
        if c.mixed && !c.group4 && row==0 && td {return Err(Error::Invalid);}
        let fuel=q.end-q.pos+1;
        let st=if use_a {fr::row(s,&a,n,w,td,q,&mut z,fuel)?} else {fr::row(s,&z,n,w,td,q,&mut a,fuel)?};
        let ghost before_output=output@;
        let ghost next=step(s@,c,w,model).unwrap();
        assert(result==finish(s@,c,w,next,(h-row-1) as nat));
        proof {if result.is_some() {prefix(s@,c,w,next,(h-row-1) as nat);}}
        let ghost ends=if use_a {z@.subrange(0,st.n as int)} else {a@.subrange(0,st.n as int)};
        assert(next.reference==ends && next.history==model.history.push(ends));assert(row_ok(ends,w));
        assert(next.history.len()==row+1 && next.history[row as int]==ends);
        proof { if result.is_some() {
            assert(finish(s@,c,w,next,(h-row-1) as nat).unwrap().history[row as int]==next.history[row as int]);
            assert(result.unwrap().history[row as int]==ends);
        }}
        if write {
            assert(w as int*row<=w as int*h) by(nonlinear_arith) requires w>0,row<h;
            let base=w*row;
            assert(w as int*(row+1)==base as int+w) by(nonlinear_arith) requires base as int==w as int*row;
            assert(base as int+w<=w as int*h) by(nonlinear_arith) requires base as int==w as int*row,w>0,row<h;
            if use_a {render(&z,st.n,w,base,output)?;} else {render(&a,st.n,w,base,output)?;}
            assert forall|k:int| 0<=k<w as int*(row+1) && result.is_some() implies output@[k]==output_byte(s@,c,w,b,h,k) by {
                if k>=base {
                    assert(k==row as int*w+(k-base)) by(nonlinear_arith) requires base as int==w as int*row;
                    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(k,w as int,row as int,k-base);
                    assert(result.unwrap().history[row as int]==ends);
                    assert(output@[k]==pixel(ends,k-base,ends.len()));
                }
            }
        }
        assert forall|k:int| 0<=k<output.len() && (!write || k>=w as int*(row+1)) implies output@[k]==old(output)@[k] by {
            assert(w as int*(row+1)==w as int*row+w) by(nonlinear_arith);
            assert(before_output[k]==old(output)@[k]);
        }
        proof {model=next;}
        bits=st.bits;n=st.n;use_a=!use_a;row=row+1;
        assert(w as int*row<=w as int*h) by(nonlinear_arith) requires w>0,row<=h;
    }
    reveal_with_fuel(finish,1);
    if ff::end(s,bits,c) {Ok(())} else {Err(Error::Invalid)}
}
}
