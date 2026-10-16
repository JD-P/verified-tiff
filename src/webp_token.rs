use super::*;
use crate::deflate_bits::{self as db,Bits};
use crate::webp_huffman as wh;
use crate::webp_container::{self as wc,Group,GroupModel};
verus! {
pub(super) open spec fn base(c:int) -> int {seq![1int,2int,3int,4int,5int,7int,9int,13int,17int,25int,33int,49int,65int,97int,129int,193int,257int,385int,513int,769int,1025int,1537int,2049int,3073int,4097int,6145int,8193int,12289int,16385int,24577int,32769int,49153int,65537int,98305int,131073int,196609int,262145int,393217int,524289int,786433int][c]}
pub(super) open spec fn extra(c:int) -> int {if c<4 {0} else {(c-2)/2}}
fn info(c:usize) -> (r:(usize,usize))
    requires c<40,ensures r.0==base(c as int),r.1==extra(c as int),1<=r.0<=786433,r.1<=18,
        c<24 ==> r.0<=3073 && r.1<=10,
{
let a=[1usize,2usize,3usize,4usize,5usize,7usize,9usize,13usize,17usize,25usize,33usize,49usize,65usize,97usize,129usize,193usize,257usize,385usize,513usize,769usize,1025usize,1537usize,2049usize,3073usize,4097usize,6145usize,8193usize,12289usize,16385usize,24577usize,32769usize,49153usize,65537usize,98305usize,131073usize,196609usize,262145usize,393217usize,524289usize,786433usize];(a[c],if c<4 {0} else {(c-2)/2})
}
pub(super) open spec fn wide(s:Seq<u8>,p:int,n:int) -> int {
    if n<=16 {db::field(s,p,n as nat)} else {db::field(s,p,16)+65536*db::field(s,p+16,(n-16) as nat)}
}
fn read_wide(s:&[u8],b:&mut Bits,n:usize) -> (r:Result<usize,Error>)
    requires old(b).valid(s@),n<=18,
    ensures final(b).valid(s@),final(b).end==old(b).end,final(b).pos()>=old(b).pos(),
        r.is_ok()==(old(b).pos()+n<=old(b).end as int*8),
        r.is_ok() ==> final(b).pos()==old(b).pos()+n && r.unwrap() as int==wide(s@,old(b).pos(),n as int) && r.unwrap()<262144,
        r.is_ok() && n<=10 ==> r.unwrap()<1024,
{
    if n<=16 {b.read(s,n)} else {let lo=b.read(s,16)?;let hi=b.read(s,n-16)?;Ok(lo+65536*hi)}
}
#[verifier::opaque]
pub(super) open spec fn prefix(s:Seq<u8>,p:int,end:int,c:int) -> Option<(int,int)> {
    if p+extra(c)>end*8 {None} else {Some((p+extra(c),base(c)+wide(s,p,extra(c))))}
}
fn read_prefix(s:&[u8],b:&mut Bits,c:usize) -> (r:Result<usize,Error>)
    requires old(b).valid(s@),c<40,
    ensures final(b).valid(s@),final(b).end==old(b).end,final(b).pos()>=old(b).pos(),
        r.is_ok()==prefix(s@,old(b).pos(),old(b).end as int,c as int).is_some(),
        r.is_ok() ==> (final(b).pos(),r.unwrap() as int)==prefix(s@,old(b).pos(),old(b).end as int,c as int).unwrap(),
        r.is_ok() ==> 1<=r.unwrap()<=1048576,c<24 && r.is_ok() ==> r.unwrap()<=4096,
{
    reveal(prefix);let (a,n)=info(c);Ok(a+read_wide(s,b,n)?)
}
pub(super) open spec fn dx(i:int) -> int {seq![0int,1int,1int,-1int,0int,2int,1int,-1int,2int,-2int,2int,-2int,0int,3int,1int,-1int,3int,-3int,2int,-2int,3int,-3int,0int,4int,1int,-1int,4int,-4int,3int,-3int,2int,-2int,4int,-4int,0int,3int,-3int,4int,-4int,5int,1int,-1int,5int,-5int,2int,-2int,5int,-5int,4int,-4int,3int,-3int,5int,-5int,0int,6int,1int,-1int,6int,-6int,2int,-2int,6int,-6int,4int,-4int,5int,-5int,3int,-3int,6int,-6int,0int,7int,1int,-1int,5int,-5int,7int,-7int,4int,-4int,6int,-6int,2int,-2int,7int,-7int,3int,-3int,7int,-7int,5int,-5int,6int,-6int,8int,4int,-4int,7int,-7int,8int,8int,6int,-6int,8int,5int,-5int,7int,-7int,8int,6int,-6int,7int,-7int,8int,7int,-7int,8int,8int][i]}
pub(super) open spec fn dy(i:int) -> int {seq![1int,0int,1int,1int,2int,0int,2int,2int,1int,1int,2int,2int,3int,0int,3int,3int,1int,1int,3int,3int,2int,2int,4int,0int,4int,4int,1int,1int,3int,3int,4int,4int,2int,2int,5int,4int,4int,3int,3int,0int,5int,5int,1int,1int,5int,5int,2int,2int,4int,4int,5int,5int,3int,3int,6int,0int,6int,6int,1int,1int,6int,6int,2int,2int,5int,5int,4int,4int,6int,6int,3int,3int,7int,0int,7int,7int,5int,5int,1int,1int,6int,6int,4int,4int,7int,7int,2int,2int,7int,7int,3int,3int,6int,6int,5int,5int,0int,7int,7int,4int,4int,1int,2int,6int,6int,3int,7int,7int,5int,5int,4int,7int,7int,6int,6int,5int,7int,7int,6int,7int][i]}
pub(super) open spec fn distance(c:int,w:int) -> int {
    if c>120 {c-120} else {let d=dx(c-1)+dy(c-1)*w;if d<1 {1} else {d}}
}
fn map_distance(c:usize,w:usize) -> (r:usize)
    requires 1<=c<=1048576,1<=w<=16384,ensures r as int==distance(c as int,w as int),1<=r<=1048576,
{
    if c>120 {c-120} else {
let x=[0i32,1i32,1i32,-1i32,0i32,2i32,1i32,-1i32,2i32,-2i32,2i32,-2i32,0i32,3i32,1i32,-1i32,3i32,-3i32,2i32,-2i32,3i32,-3i32,0i32,4i32,1i32,-1i32,4i32,-4i32,3i32,-3i32,2i32,-2i32,4i32,-4i32,0i32,3i32,-3i32,4i32,-4i32,5i32,1i32,-1i32,5i32,-5i32,2i32,-2i32,5i32,-5i32,4i32,-4i32,3i32,-3i32,5i32,-5i32,0i32,6i32,1i32,-1i32,6i32,-6i32,2i32,-2i32,6i32,-6i32,4i32,-4i32,5i32,-5i32,3i32,-3i32,6i32,-6i32,0i32,7i32,1i32,-1i32,5i32,-5i32,7i32,-7i32,4i32,-4i32,6i32,-6i32,2i32,-2i32,7i32,-7i32,3i32,-3i32,7i32,-7i32,5i32,-5i32,6i32,-6i32,8i32,4i32,-4i32,7i32,-7i32,8i32,8i32,6i32,-6i32,8i32,5i32,-5i32,7i32,-7i32,8i32,6i32,-6i32,7i32,-7i32,8i32,7i32,-7i32,8i32,8i32];
let y=[1i32,0i32,1i32,1i32,2i32,0i32,2i32,2i32,1i32,1i32,2i32,2i32,3i32,0i32,3i32,3i32,1i32,1i32,3i32,3i32,2i32,2i32,4i32,0i32,4i32,4i32,1i32,1i32,3i32,3i32,4i32,4i32,2i32,2i32,5i32,4i32,4i32,3i32,3i32,0i32,5i32,5i32,1i32,1i32,5i32,5i32,2i32,2i32,4i32,4i32,5i32,5i32,3i32,3i32,6i32,0i32,6i32,6i32,1i32,1i32,6i32,6i32,2i32,2i32,5i32,5i32,4i32,4i32,6i32,6i32,3i32,3i32,7i32,0i32,7i32,7i32,5i32,5i32,1i32,1i32,6i32,6i32,4i32,4i32,7i32,7i32,2i32,2i32,7i32,7i32,3i32,3i32,6i32,6i32,5i32,5i32,0i32,7i32,7i32,4i32,4i32,1i32,2i32,6i32,6i32,3i32,7i32,7i32,5i32,5i32,4i32,7i32,7i32,6i32,6i32,5i32,7i32,7i32,6i32,7i32];
        let xi=x[c-1];let yi=y[c-1];
        assert(-7<=xi<=8 && 0<=yi<=7);
        assert(0<=yi as int*w as int<=114688) by(nonlinear_arith) requires 0<=yi<=7,1<=w<=16384;
        let d=xi+yi*(w as i32);if d<1 {1} else {d as usize}
    }
}
#[derive(Clone,Copy)]
pub(super) struct Token {pub kind:u8,pub len:usize,pub dist:usize,pub key:usize,pub rgba:[u8;4]}
pub(super) open spec fn literal(r:int,g:int,b:int,a:int) -> Token {Token {kind:0,len:1,dist:0,key:0,rgba:[r as u8,g as u8,b as u8,a as u8]}}
#[verifier::opaque]
pub(super) open spec fn token(s:Seq<u8>,p:int,end:int,h:GroupModel,w:int,cb:int,n:int,left:int) -> Option<(int,Token)> {
    let g=wh::decode(s,p,end,h.g);if g.is_none() {None} else {
    let (c,q)=g.unwrap();
    if c<256 {
        let r=wh::decode(s,q,end,h.r);if r.is_none() || r.unwrap().0>=256 {None} else {
        let b=wh::decode(s,r.unwrap().1,end,h.b);if b.is_none() || b.unwrap().0>=256 {None} else {
        let a=wh::decode(s,b.unwrap().1,end,h.a);if a.is_none() || a.unwrap().0>=256 {None} else {
        Some((a.unwrap().1,literal(r.unwrap().0,c,b.unwrap().0,a.unwrap().0)))}}}
    } else if c<280 {
        let ln=prefix(s,q,end,c-256);if ln.is_none() {None} else {
        let dc=wh::decode(s,ln.unwrap().0,end,h.d);if dc.is_none() || dc.unwrap().0>=40 {None} else {
        let dp=prefix(s,dc.unwrap().1,end,dc.unwrap().0);if dp.is_none() {None} else {
        let d=distance(dp.unwrap().1,w);let len=ln.unwrap().1;
        if d>n || len>left {None} else {Some((dp.unwrap().0,Token {kind:1,len:len as usize,dist:d as usize,key:0,rgba:[0u8,0,0,0]}))}}}}
    } else if c-280>=wc::cache_size(cb) {None}
    else {Some((q,Token {kind:2,len:1,dist:0,key:(c-280) as usize,rgba:[0u8,0,0,0]}))}
    }
}
pub(super) fn read_token(s:&[u8],bits:&mut Bits,h:&Group,w:usize,cb:usize,n:usize,left:usize) -> (r:Result<Token,Error>)
    requires old(bits).valid(s@),h.valid(),1<=w<=16384,cb<=11,left>0,
    ensures final(bits).valid(s@),final(bits).end==old(bits).end,final(bits).pos()>=old(bits).pos(),
        r.is_ok()==token(s@,old(bits).pos(),old(bits).end as int,h.model(),w as int,cb as int,n as int,left as int).is_some(),
        r.is_ok() ==> (final(bits).pos(),r.unwrap())==token(s@,old(bits).pos(),old(bits).end as int,h.model(),w as int,cb as int,n as int,left as int).unwrap(),
        r.is_ok() ==> 1<=r.unwrap().len<=left && r.unwrap().kind<=2 && (r.unwrap().kind==1 ==> 1<=r.unwrap().dist<=n),
        r.is_ok() ==> (r.unwrap().kind==2 ==> r.unwrap().key<wc::cache_size(cb as int)),
        r.is_ok() ==> (r.unwrap().kind!=1 ==> r.unwrap().len==1),
{
    reveal(token);let c=h.g.read(s,bits)?;
    if c<256 {
        let r=h.r.read(s,bits)?;if r>=256 {return Err(Error::Invalid);}
        let b=h.b.read(s,bits)?;if b>=256 {return Err(Error::Invalid);}
        let a=h.a.read(s,bits)?;if a>=256 {return Err(Error::Invalid);}
        Ok(Token {kind:0,len:1,dist:0,key:0,rgba:[r as u8,c as u8,b as u8,a as u8]})
    } else if c<280 {
        let len=read_prefix(s,bits,c-256)?;let dc=h.d.read(s,bits)?;if dc>=40 {return Err(Error::Invalid);}
        let d=map_distance(read_prefix(s,bits,dc)?,w);
        if d>n || len>left {return Err(Error::Invalid);}
        Ok(Token {kind:1,len,dist:d,key:0,rgba:[0u8,0,0,0]})
    } else {
        let size=wc::size(cb);if c-280>=size {return Err(Error::Invalid);}
        Ok(Token {kind:2,len:1,dist:0,key:c-280,rgba:[0u8,0,0,0]})
    }
}
}
