// Fixed-point separable 8x8 inverse DCT. Constants are round(2^14 C(u) cos(...)).
// The contract specifies this integer transform; it is not a proof of real cosine.
use super::*;
verus! {
pub(super) open spec fn basis(k:int) -> int {seq![11585int,16069,15137,13623,11585,9102,6270,3196,11585,13623,6270,-3196,-11585,-16069,-15137,-9102,11585,9102,-6270,-16069,-11585,3196,15137,13623,11585,3196,-15137,-9102,11585,13623,-6270,-16069,11585,-3196,-15137,9102,11585,-13623,-6270,16069,11585,-9102,-6270,16069,-11585,-3196,15137,-13623,11585,-13623,6270,3196,-11585,16069,-15137,9102,11585,-16069,15137,-13623,11585,-9102,6270,-3196][k]}
pub(super) open spec fn row(c:Seq<i64>,v:int,x:int,n:nat) -> int decreases n {
    if n==0 {0} else {row(c,v,x,(n-1) as nat)+c[v*8+n-1] as int*basis(x*8+n-1)}
}
pub(super) open spec fn col(c:Seq<i64>,y:int,x:int,n:nat) -> int decreases n {
    if n==0 {0} else {col(c,y,x,(n-1) as nat)+row(c,n as int-1,x,8)*basis(y*8+n-1)}
}
pub(super) open spec fn sample(c:Seq<i64>,k:int) -> u8 {
    let v=col(c,k/8,k%8,8)+137975824384;
    if v<=0 {0u8} else if v>=273804165120 {255u8} else {(v/1073741824) as u8}
}
spec fn line(c:Seq<i64>,x:int,n:nat) -> int decreases n {
    if n==0 {0} else {line(c,x,(n-1) as nat)+c[n-1] as int*basis(x*8+n-1)}
}
// Exact factorization of our existing integer matrix, including its rounding.
// Mirror symmetry shares even/odd terms; constant blocks bypass the AC work.
#[inline(always)]
#[verifier::spinoff_prover]
fn transform(c:[i64;8],limit:i64) -> (out:[i64;8])
    requires 0<=limit<=68719476736,forall|i:int| 0<=i<8 ==> -limit<=#[trigger] c@[i]<=limit,
    ensures forall|i:int| 0<=i<8 ==> out@[i] as int==line(c@,i,8)
        && -(limit as int)*131072<=out@[i]<=(limit as int)*131072,
{
    assert(-limit<=c[0]<=limit);assert(-limit<=c[1]<=limit);
    assert(-limit<=c[2]<=limit);assert(-limit<=c[3]<=limit);
    assert(-limit<=c[4]<=limit);assert(-limit<=c[5]<=limit);
    assert(-limit<=c[6]<=limit);assert(-limit<=c[7]<=limit);
    if c[1]==0 && c[2]==0 && c[3]==0 && c[4]==0 && c[5]==0 && c[6]==0 && c[7]==0 {
        let dc=c[0]*11585;
        let out=[dc;8];
        assert forall|i:int| 0<=i<8 implies out@[i] as int==line(c@,i,8)
            && -(limit as int)*131072<=out@[i]<=(limit as int)*131072 by {
            reveal_with_fuel(line,9);
            if i==0 {} else if i==1 {} else if i==2 {} else if i==3 {} else if i==4 {} else if i==5 {} else if i==6 {} else {}
        }
        return out;
    }
    let a=(c[0]+c[4])*11585;let b=(c[0]-c[4])*11585;
    let e=c[2]*15137+c[6]*6270;let f=c[2]*6270-c[6]*15137;
    let e0=a+e;let e1=b+f;let e2=b-f;let e3=a-e;
    let o0=c[1]*16069+c[3]*13623+c[5]*9102+c[7]*3196;
    let o1=c[1]*13623-c[3]*3196-c[5]*16069-c[7]*9102;
    let o2=c[1]*9102-c[3]*16069+c[5]*3196+c[7]*13623;
    let o3=c[1]*3196-c[3]*9102+c[5]*13623-c[7]*16069;
    let out=[e0+o0,e1+o1,e2+o2,e3+o3,e3-o3,e2-o2,e1-o1,e0-o0];
    assert forall|i:int| 0<=i<8 implies out@[i] as int==line(c@,i,8)
        && -(limit as int)*131072<=out@[i]<=(limit as int)*131072 by {
        reveal_with_fuel(line,9);
        if i==0 {} else if i==1 {} else if i==2 {} else if i==3 {} else if i==4 {} else if i==5 {} else if i==6 {} else {}
    }
    out
}
proof fn line_row(c:Seq<i64>,v:int,x:int,n:nat)
    requires c.len()==64,0<=v<8,0<=x<8,n<=8,
    ensures line(Seq::new(8,|i:int| c[v*8+i]),x,n)==row(c,v,x,n),
    decreases n,
{if n>0 {line_row(c,v,x,(n-1) as nat);}}
proof fn line_col(c:Seq<i64>,t:Seq<i64>,x:int,y:int,n:nat)
    requires c.len()==64,t.len()==64,0<=x<8,0<=y<8,n<=8,
        forall|j:int| 0<=j<64 ==> t[j] as int==row(c,j/8,j%8,8),
    ensures line(Seq::new(8,|i:int| t[i*8+x]),y,n)==col(c,y,x,n),
    decreases n,
{
    if n>0 {
        line_col(c,t,x,y,(n-1) as nat);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse((n as int-1)*8+x,8,n as int-1,x);
    }
}
pub(super) fn idct(c:[i64;64]) -> (out:[u8;64])
    requires forall|k:int| 0<=k<64 ==> -524288<=#[trigger] c@[k]<=524288,
    ensures forall|k:int| 0<=k<64 ==> out@[k]==sample(c@,k),
{
    let mut t=[0i64;64];let mut v=0usize;
    while v<8
        invariant v<=8,forall|j:int| 0<=j<64 ==> -524288<=#[trigger] c@[j]<=524288,
            forall|j:int| 0<=j<v as int*8 ==> t@[j] as int==row(c@,j/8,j%8,8) && -68719476736<=t@[j]<=68719476736,
        decreases 8-v,
    {
        let i=v*8;
        let a=[c[i],c[i+1],c[i+2],c[i+3],c[i+4],c[i+5],c[i+6],c[i+7]];
        assert(a@ =~= Seq::new(8,|j:int| c@[v as int*8+j]));
        let r=transform(a,524288);let mut x=0usize;
        while x<8
            invariant v<8,i==v*8,x<=8,a@==Seq::new(8,|j:int| c@[v as int*8+j]),
                forall|j:int| 0<=j<64 ==> -524288<=#[trigger] c@[j]<=524288,
                forall|j:int| 0<=j<8 ==> r@[j] as int==line(a@,j,8) && -68719476736<=r@[j]<=68719476736,
                forall|j:int| 0<=j<v as int*8+x ==> t@[j] as int==row(c@,j/8,j%8,8) && -68719476736<=t@[j]<=68719476736,
            decreases 8-x,
        {
            proof {
                line_row(c@,v as int,x as int,8);
                vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(i as int+x,8,v as int,x as int);
            }
            t[i+x]=r[x];x=x+1;
        }
        v=v+1;
    }
    let mut out=[0u8;64];let mut x=0usize;
    while x<8
        invariant x<=8,
            forall|j:int| 0<=j<64 ==> t@[j] as int==row(c@,j/8,j%8,8) && -68719476736<=t@[j]<=68719476736,
            forall|j:int| 0<=j<64 && j%8<x ==> out@[j]==sample(c@,j),
        decreases 8-x,
    {
        let a=[t[x],t[8+x],t[16+x],t[24+x],t[32+x],t[40+x],t[48+x],t[56+x]];
        assert(a@ =~= Seq::new(8,|j:int| t@[j*8+x]));
        let r=transform(a,68719476736);let mut y=0usize;
        while y<8
            invariant x<8,y<=8,a@==Seq::new(8,|j:int| t@[j*8+x]),
                forall|j:int| 0<=j<64 ==> t@[j] as int==row(c@,j/8,j%8,8) && -68719476736<=t@[j]<=68719476736,
                forall|j:int| 0<=j<8 ==> r@[j] as int==line(a@,j,8) && -9007199254740992<=r@[j]<=9007199254740992,
                forall|j:int| 0<=j<64 && (j%8<x || (j%8==x && j/8<y)) ==> out@[j]==sample(c@,j),
            decreases 8-y,
        {
            let k=y*8+x;
            proof {line_col(c@,t@,x as int,y as int,8);
                vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(k as int,8,y as int,x as int);}
            let v=r[y];let shifted=v+137975824384;
            out[k]=if shifted<=0 {0} else if shifted>=273804165120 {255} else {(shifted/1073741824) as u8};
            y=y+1;
        }
        x=x+1;
    }
    out
}
}
