//! VP8 coefficient token, category, zigzag and signed dequantization model.
use super::*;
use crate::vp8_bool as ar;
use crate::vp8_tree as tr;
verus! {
pub(super) const TREE:[i16;22]=[0,1,-1,2,-2,3,4,6,-3,5,-4,-5,7,8,-6,-7,9,10,-8,-9,-10,-11];
pub(super) const BANDS:[usize;16]=[0,1,2,3,6,4,5,6,6,6,6,6,6,6,6,7];
pub(super) const ZIGZAG:[usize;16]=[0,1,4,8,5,2,3,6,9,12,13,10,7,11,14,15];
pub(super) const CATS:[[u8;11];6]=[
    [159,0,0,0,0,0,0,0,0,0,0],[165,145,0,0,0,0,0,0,0,0,0],
    [173,148,140,0,0,0,0,0,0,0,0],[176,155,140,135,0,0,0,0,0,0,0],
    [180,157,141,134,130,0,0,0,0,0,0],[254,254,243,230,196,177,153,140,133,130,129]];
pub(super) const LENGTHS:[usize;6]=[1,2,3,4,5,11];
pub(super) const BASE:[u32;6]=[5,7,11,19,35,67];
pub(super) open spec fn magnitude(s:Seq<u8>,r:ar::Reader,token:u8) -> Option<(u32,ar::Reader)> {
    if token<2 || token>11{None}else if token<6{Some(((token-1) as u32,r))}else{
        let c=token as int-6;let p=CATS[c]@.take(LENGTHS[c] as int);
        let v=tr::suffix(s,r,p,0,0,p.len());
        if v.is_none(){None}else{Some(((BASE[c]+v.unwrap().0) as u32,v.unwrap().1))}
    }
}
fn number(s:&[u8],r:ar::Reader,token:u8) -> (out:Result<(u32,ar::Reader),Error>)
    requires r.valid(s@),
    ensures out.is_ok()==magnitude(s@,r,token).is_some(),
        out.is_ok() ==> out.unwrap()==magnitude(s@,r,token).unwrap() && out.unwrap().1.valid(s@) && 1<=out.unwrap().0<=2114,
{
    if token<2 || token>11{return Err(Error::Invalid);}
    if token<6{return Ok(((token-1) as u32,r));}
    let c=(token-6) as usize;let n=LENGTHS[c];let base=BASE[c];let (p,_)=CATS[c].split_at(n);
    assert(1<=n<=11 && 5<=base<=67);
    let (v,t)=tr::extra(s,r,p)?;assert(v<=2047);
    Ok((base+v,t))
}
pub(super) open spec fn wrapped(v:int) -> int {let u=(v+4294967296)%65536;if u>=32768{u-65536}else{u}}
pub(super) fn wrap(v:i64) -> (r:i32)
    requires -1000000000<=v<=1000000000,
    ensures r==wrapped(v as int),-32768<=r<=32767,
{
    let u=((v+4294967296) as u64)%65536;
    if u>=32768{u as i32-65536}else{u as i32}
}
pub(super) open spec fn signed(s:Seq<u8>,r:ar::Reader,token:u8,q:u16) -> Option<(i32,ar::Reader)> {
    let v=magnitude(s,r,token);if v.is_none(){None}else{
        let sign=ar::symbol(s,v.unwrap().1,128);
        if sign.is_none(){None}else{
            let x=v.unwrap().0 as int*q;
            Some((wrapped(if sign.unwrap().0{-x}else{x}) as i32,sign.unwrap().1))
        }
    }
}
fn dequant(s:&[u8],r:ar::Reader,token:u8,q:u16) -> (out:Result<(i32,ar::Reader),Error>)
    requires r.valid(s@),
    ensures out.is_ok()==signed(s@,r,token,q).is_some(),
        out.is_ok() ==> out.unwrap()==signed(s@,r,token,q).unwrap() && out.unwrap().1.valid(s@) && -32768<=out.unwrap().0<=32767,
{
    let (v,t)=number(s,r,token)?;let (negative,t)=ar::bit(s,t,128)?;
    assert(v as int*q<=138541990) by(nonlinear_arith) requires v<=2114,q<=65535;
    let x=v as i64*q as i64;let x=if negative{-x}else{x};Ok((wrap(x),t))
}
pub(super) open spec fn probabilities(h:Seq<i32>,kind:usize,n:usize,ctx:usize) -> Seq<u8> {
    Seq::new(11,|j:int|h[38+((kind*8+BANDS[n as int])*3+ctx)*11+j] as u8)
}
fn probs(h:&[i32;1096],kind:usize,n:usize,ctx:usize) -> (p:[u8;11])
    requires kind<4,n<16,ctx<3,
    ensures p@==probabilities(h@,kind,n,ctx),
{
    let b=BANDS[n];assert(b<8);let at=38+((kind*8+b)*3+ctx)*11;
    let mut p=[0u8;11];let mut j=0usize;
    while j<11
        invariant j<=11,at==38+((kind*8+b)*3+ctx)*11,b==BANDS[n as int],kind<4,b<8,ctx<3,
            forall|k:int|0<=k<j ==> #[trigger] p@[k]==h@[at+k] as u8,
        decreases 11-j,
    {p[j]=h[at+j] as u8;j=j+1;}
    assert(p@ =~= probabilities(h@,kind,n,ctx));p
}
#[derive(Clone,Copy)]
pub(super) struct State {pub reader:ar::Reader,pub n:usize,pub ctx:usize,pub zero:bool,pub done:bool}
pub(super) open spec fn advance(s:Seq<u8>,h:Seq<i32>,kind:usize,q0:u16,q1:u16,st:State,out:Seq<i32>) -> Option<(State,Seq<i32>)> {
    let p=probabilities(h,kind,st.n,st.ctx);
    let t=tr::walk(s,st.reader,p,TREE@,if st.zero{1usize}else{0usize},32);
    if t.is_none(){None}else{
        let token=t.unwrap().0;let reader=t.unwrap().1;
        if token==0{Some((State {reader,done:true,..st},out))}
        else if token==1{Some((State {reader,n:(st.n+1) as usize,ctx:0,zero:true,..st},out))}
        else{
            let v=signed(s,reader,token,if st.n==0{q0}else{q1});
            if v.is_none(){None}else{Some((State {reader:v.unwrap().1,n:(st.n+1) as usize,ctx:if token==2{1usize}else{2usize},zero:false,..st},out.update(ZIGZAG[st.n as int] as int,v.unwrap().0)))}
        }
    }
}
#[verifier::opaque]
pub(super) open spec fn finish(s:Seq<u8>,h:Seq<i32>,kind:usize,q0:u16,q1:u16,st:State,out:Seq<i32>,fuel:nat) -> Option<(State,Seq<i32>)> decreases fuel {
    if st.n==16 || st.done{Some((st,out))}else if fuel==0{None}else{
        let a=advance(s,h,kind,q0,q1,st,out);
        if a.is_none(){None}else{finish(s,h,kind,q0,q1,a.unwrap().0,a.unwrap().1,(fuel-1) as nat)}
    }
}
pub(super) fn decode(s:&[u8],r:ar::Reader,h:&[i32;1096],kind:usize,ctx:usize,first:usize,q0:u16,q1:u16,out:&mut[i32;16]) -> (res:Result<(usize,ar::Reader),Error>)
    requires r.valid(s@),kind<4,ctx<3,first<=1,forall|i:int|0<=i<16 ==> -32768<=#[trigger] old(out)@[i]<=32767,
    ensures res.is_ok()==finish(s@,h@,kind,q0,q1,State {reader:r,n:first,ctx,zero:false,done:false},old(out)@,17).is_some(),
        res.is_ok() ==> {
            let d=finish(s@,h@,kind,q0,q1,State {reader:r,n:first,ctx,zero:false,done:false},old(out)@,17).unwrap();
            res.unwrap()==(d.0.n,d.0.reader) && final(out)@==d.1 && res.unwrap().1.valid(s@) && first<=res.unwrap().0<=16
        },
        forall|i:int|0<=i<16 ==> -32768<=#[trigger] final(out)@[i]<=32767,
{
    let initial=State {reader:r,n:first,ctx,zero:false,done:false};let mut st=initial;let mut fuel=17usize;
    let ghost goal=finish(s@,h@,kind,q0,q1,initial,out@,17);
    loop
        invariant st.reader.valid(s@),kind<4,st.ctx<3,first<=st.n<=16,fuel<=17,first<=1,
            goal==finish(s@,h@,kind,q0,q1,initial,old(out)@,17),goal==finish(s@,h@,kind,q0,q1,st,out@,fuel as nat),
            initial==(State {reader:r,n:first,ctx,zero:false,done:false}),
            forall|i:int|0<=i<16 ==> -32768<=#[trigger] out@[i]<=32767,
        decreases fuel,
    {
        reveal_with_fuel(finish,1);
        if st.n==16 || st.done{return Ok((st.n,st.reader));}
        if fuel==0{return Err(Error::Invalid);}
        let p=probs(h,kind,st.n,st.ctx);
        let (token,reader)=tr::decode(s,st.reader,&p,&TREE,if st.zero{1}else{0})?;
        let ghost before=out@;let prev=st;
        if token==0{st=State {reader,done:true,..st};}
        else if token==1{st=State {reader,n:st.n+1,ctx:0,zero:true,..st};}
        else{
            let q=if st.n==0{q0}else{q1};let (v,reader)=dequant(s,reader,token,q)?;
            let z=ZIGZAG[st.n];assert(z<16);out[z]=v;
            st=State {reader,n:st.n+1,ctx:if token==2{1}else{2},zero:false,..st};
        }
        assert(advance(s@,h@,kind,q0,q1,prev,before)==Some((st,out@)));fuel=fuel-1;
    }
}
}
