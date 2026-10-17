//! Exact crop placement and strip/tile selection, with no FFI preconditions.
use super::*;
verus! {
pub fn grid(width:usize,height:usize,bw:usize,bh:usize,planes:usize,cap:usize)->(r:Result<(usize,usize,usize,usize),Error>)
    ensures r.is_ok() ==> width>0 && height>0 && bw>0 && bh>0 && planes>0,
        r.is_ok() ==> r.unwrap().0==(width as int-1)/(bw as int)+1 && r.unwrap().1==(height as int-1)/(bh as int)+1,
        r.is_ok() ==> r.unwrap().2==r.unwrap().0 as int*r.unwrap().1 && r.unwrap().3==r.unwrap().2 as int*planes && r.unwrap().3<=cap,
        width>0 && height>0 && bw>0 && bh>0 && planes>0
         && ((width as int-1)/(bw as int)+1)*((height as int-1)/(bh as int)+1)*planes<=cap ==> r.is_ok(),
{
    if width==0||height==0||bw==0||bh==0||planes==0{return Err(Error::Invalid);}
    let across=(width-1)/bw+1;let down=(height-1)/bh+1;
    proof {if across as int*down*planes<=cap {
        assert(across as int*down<=cap) by(nonlinear_arith)
            requires across>0,down>0,planes>0,across as int*down*planes<=cap;
    }}
    let per=match mul(across,down){Ok(v)=>v,Err(e)=>return Err(e)};
    let count=match mul(per,planes){Ok(v)=>v,Err(e)=>return Err(e)};
    if count>cap{return Err(Error::Limit);}Ok((across,down,per,count))
}
pub fn segment(across:usize,down:usize,planes:usize,bx:usize,by:usize,plane:usize)->(r:Result<usize,Error>)
    ensures r.is_ok() ==> bx<across && by<down && plane<planes,
        r.is_ok() ==> r.unwrap()==(plane as int*down+by)*across+bx,
        r.is_ok() ==> r.unwrap()<across as int*down*planes,
        bx<across && by<down && plane<planes && across as int*down*planes<=usize::MAX ==> r.is_ok(),
{
    if bx>=across||by>=down||plane>=planes{return Err(Error::Invalid);}
    proof {
        assert(plane as int*down+by<planes as int*down) by(nonlinear_arith)
            requires plane<planes,by<down,down>0;
        assert((plane as int*down+by)*across+bx<across as int*down*planes) by(nonlinear_arith)
            requires plane as int*down+by<planes as int*down,bx<across,across>0;
        assert(plane as int*down+by<across as int*down*planes) by(nonlinear_arith)
            requires plane as int*down+by<planes as int*down,across>0,down>0,planes>0;
    }
    let p=match mul(plane,down){Ok(v)=>v,Err(e)=>return Err(e)};
    let y=match add(p,by){Ok(v)=>v,Err(e)=>return Err(e)};
    let base=match mul(y,across){Ok(v)=>v,Err(e)=>return Err(e)};
    let k=match add(base,bx){Ok(v)=>v,Err(e)=>return Err(e)};
    proof {assert(k<across as int*down*planes) by(nonlinear_arith)
        requires k==(plane as int*down+by)*across+bx,bx<across,by<down,plane<planes;}
    Ok(k)
}
pub open spec fn span_valid(w:usize,h:usize,vw:usize,vh:usize,x:usize,y:usize,n:usize)->bool {
    0<vw<=w && 0<vh<=h && x as int+n<=vw && y<vh && w as int*h*4<=usize::MAX
}
pub open spec fn span_at(w:usize,h:usize,vw:usize,vh:usize,x:usize,y:usize,n:usize,fx:bool,fy:bool)->int {
    ((h as int-vh+if fy {vh as int-1-y}else{y as int})*w + if fx {vw as int-x-n}else{x as int})*4
}
/// A horizontal span maps to a contiguous destination even when mirrored.
/// The caller reverses its pixels using the already-verified row kernel.
#[inline]
pub fn crop_span(w:usize,h:usize,vw:usize,vh:usize,x:usize,y:usize,n:usize,fx:bool,fy:bool)->(r:Result<(usize,usize),Error>)
    ensures r.is_ok()==span_valid(w,h,vw,vh,x,y,n),
        r.is_ok() ==> r.unwrap().0==span_at(w,h,vw,vh,x,y,n,fx,fy) && r.unwrap().1==n as int*4,
        r.is_ok() ==> r.unwrap().0 as int+r.unwrap().1<=w as int*h*4,
{
    if vw==0||vw>w||vh==0||vh>h||x>vw||n>vw-x||y>=vh{return Err(Error::Invalid);}
    let pixels=match mul(w,h){Ok(v)=>v,Err(e)=>return Err(e)};
    let _bytes=match mul(pixels,4){Ok(v)=>v,Err(e)=>return Err(e)};
    let tx=if fx{vw-x-n}else{x};let ty=h-vh+if fy{vh-1-y}else{y};
    proof {assert((ty as int*w+tx+n)*4<=w as int*h*4) by(nonlinear_arith)
        requires ty<h,tx as int+n<=w,w>0;}
    let at=(ty*w+tx)*4;let count=n*4;Ok((at,count))
}
/// Offset of pixel i inside a span from crop_span: the same destination a
/// one-pixel crop_span at x+i would return, so a span keeps per-pixel placement.
#[inline(always)]
pub fn span_pixel(w:usize,h:usize,vw:usize,vh:usize,x:usize,y:usize,n:usize,fx:bool,fy:bool,i:usize)->(r:usize)
    requires span_valid(w,h,vw,vh,x,y,n),i<n,
    ensures span_at(w,h,vw,vh,x,y,n,fx,fy)+r==span_at(w,h,vw,vh,(x+i) as usize,y,1,fx,fy),r as int+4<=n as int*4,
{
    proof {assert(n as int*4<=w as int*h*4) by(nonlinear_arith) requires n<=w,h>0;}
    if fx {(n-1-i)*4} else {i*4}
}
pub fn orientation(source:usize,target:usize)->(r:Result<(bool,bool),Error>)
    ensures r.is_ok()==(1<=source<=8 && 1<=target<=8),
        r.is_ok() ==> r.unwrap().0==(((source-1)%4==1||(source-1)%4==2)!=((target-1)%4==1||(target-1)%4==2)),
        r.is_ok() ==> r.unwrap().1==(((source-1)%4>=2)!=((target-1)%4>=2)),
{
    if source==0||source>8||target==0||target>8{return Err(Error::Invalid);}
    let s=(source-1)%4;let t=(target-1)%4;
    Ok(((s==1||s==2)!=(t==1||t==2),(s>=2)!=(t>=2)))
}
}
