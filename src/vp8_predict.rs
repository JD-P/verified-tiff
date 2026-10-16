//! VP8 directional intra prediction. Tables transcribe libwebp 1.6.0's
//! named predictors; attribution and license are in third-party/libwebp.
use super::*;
verus! {
pub(super) const PATTERNS:[[[usize;4];16];8]=[
    [[0,1,2,3],[1,2,3,3],[2,3,4,3],[3,4,5,3],[0,1,2,3],[1,2,3,3],[2,3,4,3],[3,4,5,3],[0,1,2,3],[1,2,3,3],[2,3,4,3],[3,4,5,3],[0,1,2,3],[1,2,3,3],[2,3,4,3],[3,4,5,3]],
    [[0,9,10,3],[0,9,10,3],[0,9,10,3],[0,9,10,3],[9,10,11,3],[9,10,11,3],[9,10,11,3],[9,10,11,3],[10,11,12,3],[10,11,12,3],[10,11,12,3],[10,11,12,3],[11,12,12,3],[11,12,12,3],[11,12,12,3],[11,12,12,3]],
    [[1,0,9,3],[2,1,0,3],[3,2,1,3],[4,3,2,3],[0,9,10,3],[1,0,9,3],[2,1,0,3],[3,2,1,3],[9,10,11,3],[0,9,10,3],[1,0,9,3],[2,1,0,3],[10,11,12,3],[9,10,11,3],[0,9,10,3],[1,0,9,3]],
    [[0,1,1,2],[1,2,2,2],[2,3,3,2],[3,4,4,2],[9,0,1,3],[0,1,2,3],[1,2,3,3],[2,3,4,3],[10,9,0,3],[0,1,1,2],[1,2,2,2],[2,3,3,2],[11,10,9,3],[9,0,1,3],[0,1,2,3],[1,2,3,3]],
    [[1,2,3,3],[2,3,4,3],[3,4,5,3],[4,5,6,3],[2,3,4,3],[3,4,5,3],[4,5,6,3],[5,6,7,3],[3,4,5,3],[4,5,6,3],[5,6,7,3],[6,7,8,3],[4,5,6,3],[5,6,7,3],[6,7,8,3],[7,8,8,3]],
    [[1,2,2,2],[2,3,3,2],[3,4,4,2],[4,5,5,2],[1,2,3,3],[2,3,4,3],[3,4,5,3],[4,5,6,3],[2,3,3,2],[3,4,4,2],[4,5,5,2],[5,6,7,3],[2,3,4,3],[3,4,5,3],[4,5,6,3],[6,7,8,3]],
    [[9,0,0,2],[9,0,1,3],[0,1,2,3],[1,2,3,3],[10,9,9,2],[10,9,0,3],[9,0,0,2],[9,0,1,3],[11,10,10,2],[11,10,9,3],[10,9,9,2],[10,9,0,3],[12,11,11,2],[12,11,10,3],[11,10,10,2],[11,10,9,3]],
    [[9,10,10,2],[9,10,11,3],[10,11,11,2],[10,11,12,3],[10,11,11,2],[10,11,12,3],[11,12,12,2],[11,12,12,3],[11,12,12,2],[11,12,12,3],[12,12,12,1],[12,12,12,1],[12,12,12,1],[12,12,12,1],[12,12,12,1],[12,12,12,1]],
];
pub(super) open spec fn clipped(v:int) -> u8 {if v<0{0u8}else if v>255{255u8}else{v as u8}}
#[inline(always)]
pub(super) fn clip(v:i32) -> (r:u8)
    ensures r==clipped(v as int),
{if v<0{0}else if v>255{255}else{v as u8}}
pub(super) open spec fn small(e:Seq<u8>,mode:usize,k:usize) -> u8 {
    if mode==0{((4+e[1] as int+e[2] as int+e[3] as int+e[4] as int+e[9] as int+e[10] as int+e[11] as int+e[12] as int)/8) as u8}
    else if mode==1{clipped(e[1+k%4] as int+e[9+k/4] as int-e[0] as int)}
    else{
        let p=PATTERNS[(mode as int)-2][k as int];let a=e[p[0] as int] as int;let b=e[p[1] as int] as int;let c=e[p[2] as int] as int;
        if p[3]==1{a as u8}else if p[3]==2{((a+b+1)/2) as u8}else{((a+2*b+c+2)/4) as u8}
    }
}
pub(super) fn predict4(e:&[u8;13],mode:usize,k:usize) -> (r:u8)
    requires mode<10,k<16,
    ensures r==small(e@,mode,k),
{
    if mode==0{((4+e[1] as u32+e[2] as u32+e[3] as u32+e[4] as u32+e[9] as u32+e[10] as u32+e[11] as u32+e[12] as u32)/8) as u8}
    else if mode==1{clip(e[1+k%4] as i32+e[9+k/4] as i32-e[0] as i32)}
    else{
        let p=PATTERNS[mode-2][k];assert(p[0]<13 && p[1]<13 && p[2]<13);
        let a=e[p[0]] as u32;let b=e[p[1]] as u32;let c=e[p[2]] as u32;
        if p[3]==1{a as u8}else if p[3]==2{((a+b+1)/2) as u8}else{((a+2*b+c+2)/4) as u8}
    }
}
pub(super) open spec fn sum(e:Seq<u8>,start:int,n:nat) -> int decreases n {
    if n==0{0}else{sum(e,start,(n-1) as nat)+e[start+n-1] as int}
}
pub(super) open spec fn dc(e:Seq<u8>,n:usize,top:bool,left:bool) -> u8 {
    let total=(if top{sum(e,1,n as nat)}else{0})+(if left{sum(e,17,n as nat)}else{0});
    let count=if top && left{2*n}else{n as int};
    if !top && !left{128u8}else{((total+count/2)/count) as u8}
}
pub(super) fn average(e:&[u8;33],n:usize,top:bool,left:bool) -> (r:u8)
    requires n==8 || n==16,
    ensures r==dc(e@,n,top,left),
{
    if !top && !left{return 128;}
    let mut v=0u32;let mut i=0usize;
    while i<n
        invariant (n==8 || n==16),i<=n,v<=510*i,
            v==(if top{sum(e@,1,i as nat)}else{0})+(if left{sum(e@,17,i as nat)}else{0}),
        decreases n-i,
    {
        if top{v=v+e[1+i] as u32;}if left{v=v+e[17+i] as u32;}
        i=i+1;reveal_with_fuel(sum,1);
    }
    let count=if top && left{2*n}else{n};((v+count as u32/2)/count as u32) as u8
}
pub(super) open spec fn large(e:Seq<u8>,mode:usize,x:usize,y:usize,dc:u8) -> u8 {
    if mode==0{dc}else if mode==1{clipped(e[1+x] as int+e[17+y] as int-e[0] as int)}else if mode==2{e[1+x]}else{e[17+y]}
}
pub(super) fn predict(e:&[u8;33],mode:usize,x:usize,y:usize,dc:u8) -> (r:u8)
    requires mode<4,x<16,y<16,
    ensures r==large(e@,mode,x,y,dc),
{
    if mode==0{dc}else if mode==1{clip(e[1+x] as i32+e[17+y] as i32-e[0] as i32)}else if mode==2{e[1+x]}else{e[17+y]}
}
}
