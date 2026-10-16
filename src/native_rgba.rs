//! Allocation-free row presentation for native unsigned 8/16-bit samples.
//! The byte model specifies libtiff's packed ABGR word, including alpha rounding.
use super::*;
verus! {
pub open spec fn value(s:Seq<u8>,i:int,wide:bool,le:bool) -> int {
    if wide {word16(s,2*i,le)} else {s[i] as int}
}
pub open spec fn color(v:int,wide:bool,rgb:bool,invert:bool) -> int {
    let x=if invert {(if wide {65535int} else {255int})-v} else {v};
    if !wide {x} else if rgb {(x+128)/257} else {x/256}
}
pub open spec fn component(s:Seq<u8>,i:int,ch:usize,wide:bool,rgb:bool,invert:bool,alpha:usize,premul:bool,le:bool,c:int) -> u8 {
    let a=if alpha<ch {color(value(s,i*ch+alpha,wide,le),wide,rgb,false)} else {255int};
    let v=color(value(s,i*ch+(if rgb {c} else {0}),wide,le),wide,rgb,invert);
    (if c==3 {a} else if premul {(v*a+127)/255} else {v}) as u8
}
pub open spec fn valid(len:int,n:usize,ch:usize,wide:bool,rgb:bool,invert:bool,alpha:usize,premul:bool,outlen:int) -> bool {
    0<ch<=64 && (!rgb || ch>=3) && (!invert || !rgb) && alpha<=ch
    && (alpha==ch || alpha>=(if rgb {3int} else {1int}))
    && (!premul || rgb && alpha<ch)
    && n as int*ch*(if wide {2int} else {1int})<=len && n as int*4<=outlen
}
#[inline(always)]
fn load(s:&[u8],i:usize,wide:bool,le:bool) -> (v:u32)
    requires (i as int+1)*(if wide {2int} else {1int})<=s.len(),
    ensures v==value(s@,i as int,wide,le),v<=if wide {65535int} else {255int},
{
    if wide {
        let a=s[2*i] as u32;let b=s[2*i+1] as u32;
        if le {a+256*b} else {256*a+b}
    }else{s[i] as u32}
}
#[inline(always)]
fn scale(v:u32,wide:bool,rgb:bool,invert:bool) -> (b:u8)
    requires v<=if wide {65535int} else {255int},
    ensures b as int==color(v as int,wide,rgb,invert),
{
    let x=if invert {(if wide {65535u32} else {255u32})-v} else {v};
    (if !wide {x} else if rgb {(x+128)/257} else {x/256}) as u8
}
#[inline(always)]
fn associate(v:u8,a:u8,premul:bool) -> (b:u8)
    ensures b as int==if premul {(v as int*a as int+127)/255} else {v as int},
{
    assert(0<=v as int*a as int<=65025) by(nonlinear_arith);
    if premul {((v as u32*a as u32+127)/255) as u8} else {v}
}
/// A bounded row, optionally mirrored. Invalid arguments leave all output intact;
/// success changes exactly 4*n bytes. Source and output lengths bound the work.
/// Inline parameters allow the adapter to specialize common layouts once per row.
#[inline(always)]
pub fn row(raw:&[u8],n:usize,ch:usize,wide:bool,rgb:bool,invert:bool,alpha:usize,premul:bool,le:bool,reverse:bool,out:&mut[u8]) -> (r:Result<(),Error>)
    ensures r.is_ok()==valid(raw.len() as int,n,ch,wide,rgb,invert,alpha,premul,old(out).len() as int),
        final(out).len()==old(out).len(),
        r.is_err() ==> final(out)@==old(out)@,
        r.is_ok() ==> forall|i:int,c:int|0<=i<n && 0<=c<4 ==> #[trigger] final(out)@[4*i+c]==component(raw@,if reverse {n as int-1-i} else {i},ch,wide,rgb,invert,alpha,premul,le,if le {c} else {3-c}),
        forall|k:int|n as int*4<=k<old(out).len() ==> final(out)@[k]==old(out)@[k],
{
    if ch==0||ch>64||rgb&&ch<3||invert&&rgb||alpha>ch||alpha<ch&&alpha<(if rgb {3} else {1})||premul&&(!rgb||alpha==ch){return Err(Error::Unsupported);}
    let stride=ch*(if wide {2usize} else {1usize});
    proof {
        quotient_remainder(raw.len() as int,stride as int);quotient_remainder(out.len() as int,4);
        assert((n<=raw.len()/stride)==(n as int*ch*(if wide {2int} else {1int})<=raw.len())) by(nonlinear_arith)
            requires stride==ch*(if wide {2int} else {1int}),stride>0,
                raw.len() as int==stride*(raw.len() as int/(stride as int))+raw.len() as int%(stride as int),
                0<=raw.len() as int%(stride as int)<stride;
    }
    if n>raw.len()/stride||n>out.len()/4{return Err(Error::OutputTooSmall);}
    let mut i=0usize;
    while i<n
        invariant valid(raw.len() as int,n,ch,wide,rgb,invert,alpha,premul,out.len() as int),
            i<=n,out.len()==old(out).len(),
            forall|p:int,c:int|0<=p<i && 0<=c<4 ==> #[trigger] out@[4*p+c]==component(raw@,if reverse {n as int-1-p} else {p},ch,wide,rgb,invert,alpha,premul,le,if le {c} else {3-c}),
            forall|k:int|i as int*4<=k<out.len() ==> out@[k]==old(out)@[k],
        decreases n-i,
    {
        let p=if reverse {n-1-i} else {i};
        assert((p as int+1)*ch*(if wide {2int} else {1int})<=raw.len()) by(nonlinear_arith)
            requires p<n,n as int*ch*(if wide {2int} else {1int})<=raw.len(),0<ch<=64;
        assert(p as int*ch+ch<=raw.len()) by(nonlinear_arith)
            requires (p as int+1)*ch*(if wide {2int} else {1int})<=raw.len(),0<ch;
        let at=p*ch;
        assert((at as int+ch)*(if wide {2int} else {1int})<=raw.len()) by(nonlinear_arith)
            requires at==p as int*ch,(p as int+1)*ch*(if wide {2int} else {1int})<=raw.len();
        proof {
            if wide {
                assert(2*(at as int+ch)<=raw.len());
                assert(2*(at as int+1)<=raw.len());
                if alpha<ch {assert(2*(at as int+alpha+1)<=raw.len());}
                if rgb {assert(2*(at as int+3)<=raw.len());}
            }
        }
        let a=if alpha<ch {scale(load(raw,at+alpha,wide,le),wide,rgb,false)} else {255u8};
        let r=scale(load(raw,at,wide,le),wide,rgb,invert);
        let g=if rgb {scale(load(raw,at+1,wide,le),wide,true,false)} else {r};
        let b=if rgb {scale(load(raw,at+2,wide,le),wide,true,false)} else {r};
        let r=associate(r,a,premul);let g=associate(g,a,premul);let b=associate(b,a,premul);
        let d=4*i;
        out[d]=if le {r} else {a};out[d+1]=if le {g} else {b};
        out[d+2]=if le {b} else {g};out[d+3]=if le {a} else {r};
        i=i+1;
    }
    Ok(())
}
}
