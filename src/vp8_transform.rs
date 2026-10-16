//! VP8's integer inverse DCT and Walsh transforms, including signed rounding.
//! Numerical constants follow libwebp; see third-party/libwebp.
use super::*;
verus! {
pub(super) open spec fn floor16(v:int) -> int {(v+1099511627776)/65536-16777216}
pub(super) open spec fn floor8(v:int) -> int {(v+1073741824)/8-134217728}
#[inline(always)]
fn div16(v:i64) -> (r:i64)
    requires -100000000000<=v<=100000000000,
    ensures r==floor16(v as int),r*65536<=v<(r+1)*65536,
{(v+1099511627776)/65536-16777216}
#[inline(always)]
pub(super) fn div8(v:i64) -> (r:i64)
    requires -10000000<=v<=10000000,
    ensures r==floor8(v as int),r*8<=v<(r+1)*8,-1250000<=r<=1250000,
{(v+1073741824)/8-134217728}
pub(super) open spec fn pass(a:int,b:int,c:int,d:int,k:usize,bias:int,walsh:bool) -> int {
    if walsh {
        if k==0{a+b+c+d+bias}else if k==1{a+b-c-d+bias}else if k==2{a-b-c+d+bias}else{a-b+c-d+bias}
    }else{
        let u=floor16(35468*b)-floor16(20091*d)-d;
        let v=floor16(20091*b)+b+floor16(35468*d);
        if k==0{a+c+bias+v}else if k==1{a-c+bias+u}else if k==2{a-c+bias-u}else{a+c+bias-v}
    }
}
#[inline(always)]
fn line(a:i64,b:i64,c:i64,d:i64,k:usize,bias:i64,walsh:bool,limit:i64) -> (r:i64)
    requires k<4,0<=bias<=4,0<=limit<=1000000,-limit<=a<=limit,-limit<=b<=limit,-limit<=c<=limit,-limit<=d<=limit,
    ensures r==pass(a as int,b as int,c as int,d as int,k,bias as int,walsh),-6*limit-8<=r<=6*limit+8,
{
    if walsh {
        if k==0{a+b+c+d+bias}else if k==1{a+b-c-d+bias}else if k==2{a-b-c+d+bias}else{a-b+c-d+bias}
    }else{
        let m1b=div16(20091*b);let m2b=div16(35468*b);let m1d=div16(20091*d);let m2d=div16(35468*d);
        let u=m2b-m1d-d;let v=m1b+b+m2d;
        if k==0{a+c+bias+v}else if k==1{a-c+bias+u}else if k==2{a-c+bias-u}else{a+c+bias-v}
    }
}
pub(super) open spec fn vertical(c:Seq<i32>,x:int,y:int,walsh:bool) -> int {
    pass(c[x] as int,c[4+x] as int,c[8+x] as int,c[12+x] as int,y as usize,0,walsh)
}
pub(super) open spec fn sample(c:Seq<i32>,k:int,walsh:bool) -> i32 {
    let y=k/4;let v=pass(vertical(c,0,y,walsh),vertical(c,1,y,walsh),vertical(c,2,y,walsh),vertical(c,3,y,walsh),(k%4) as usize,if walsh{3int}else{4int},walsh);
    if walsh{crate::vp8_coeff::wrapped(floor8(v)) as i32}else{floor8(v) as i32}
}
// Four independent lanes share each butterfly intermediate. The only assumed
// operations are the fixed-size integer primitives, not this transform.
fn pass4(a:[i32;4],b:[i32;4],c:[i32;4],d:[i32;4],bias:i32,walsh:bool,limit:i32) -> (r:[[i32;4];4])
    requires 0<=limit<=200000,0<=bias<=4,
        verified_tiff_simd::bounded(a,limit as int),verified_tiff_simd::bounded(b,limit as int),
        verified_tiff_simd::bounded(c,limit as int),verified_tiff_simd::bounded(d,limit as int),
    ensures forall|k:int,j:int|0<=k<4 && 0<=j<4 ==>
        #[trigger] r@[k]@[j]==pass(a@[j] as int,b@[j] as int,c@[j] as int,d@[j] as int,k as usize,bias as int,walsh)
        && -6*limit-8<=r@[k]@[j]<=6*limit+8,
{
    use verified_tiff_simd::{add,sub,mul_shift16};
    let z=[bias;4];
    let r=if walsh {
        let ab=add(a,b);let cd=add(c,d);let amb=sub(a,b);let cmd=sub(c,d);
        [add(add(ab,cd),z),add(sub(ab,cd),z),add(sub(amb,cmd),z),add(add(amb,cmd),z)]
    }else{
        let m1b=mul_shift16(b,20091);let m2b=mul_shift16(b,35468);
        let m1d=mul_shift16(d,20091);let m2d=mul_shift16(d,35468);
        let u=sub(sub(m2b,m1d),d);let v=add(add(m1b,b),m2d);
        let ac=add(add(a,c),z);let amc=add(sub(a,c),z);
        [add(ac,v),add(amc,u),sub(amc,u),sub(ac,v)]
    };
    assert forall|k:int,j:int|0<=k<4 && 0<=j<4 implies
        #[trigger] r@[k]@[j]==pass(a@[j] as int,b@[j] as int,c@[j] as int,d@[j] as int,k as usize,bias as int,walsh)
        && -6*limit-8<=r@[k]@[j]<=6*limit+8 by {
        assert(a@[j]>=-limit && b@[j]>=-limit && c@[j]>=-limit && d@[j]>=-limit);
    }
    r
}
fn transpose(t:[[i32;4];4]) -> (r:[[i32;4];4])
    ensures forall|i:int,j:int|0<=i<4 && 0<=j<4 ==> #[trigger] r@[i]@[j]==t@[j]@[i],
{
    [[t[0][0],t[1][0],t[2][0],t[3][0]],[t[0][1],t[1][1],t[2][1],t[3][1]],
     [t[0][2],t[1][2],t[2][2],t[3][2]],[t[0][3],t[1][3],t[2][3],t[3][3]]]
}
fn rounded_rows(h:[[i32;4];4],walsh:bool) -> (out:[i32;16])
    requires forall|i:int,j:int|0<=i<4 && 0<=j<4 ==> -1200008<=#[trigger] h@[i]@[j]<=1200008,
    ensures forall|k:int|0<=k<16 ==> #[trigger] out@[k]==
        (if walsh{crate::vp8_coeff::wrapped(floor8(h@[k%4]@[k/4] as int)) as i32}else{floor8(h@[k%4]@[k/4] as int) as i32})
        && -1250000<=out@[k]<=1250000 && (walsh ==> -32768<=out@[k]<=32767),
{
    let r=[verified_tiff_simd::shift3(h[0]),verified_tiff_simd::shift3(h[1]),verified_tiff_simd::shift3(h[2]),verified_tiff_simd::shift3(h[3])];
    let mut out=[0i32;16];let mut k=0usize;
    while k<16
        invariant k<=16,
            forall|i:int,j:int|0<=i<4 && 0<=j<4 ==> -1200008<=#[trigger] h@[i]@[j]<=1200008 && r@[i]@[j]==h@[i]@[j] as int/8,
            forall|i:int|0<=i<k ==> #[trigger] out@[i]==
                (if walsh{crate::vp8_coeff::wrapped(floor8(h@[i%4]@[i/4] as int)) as i32}else{floor8(h@[i%4]@[i/4] as int) as i32})
                && -1250000<=out@[i]<=1250000 && (walsh ==> -32768<=out@[i]<=32767),
        decreases 16-k,
    {
        assert(h@[(k%4) as int]@[(k/4) as int]>=-1200008);
        assert(r@[(k%4) as int]@[(k/4) as int]==h@[(k%4) as int]@[(k/4) as int] as int/8);
        let v=r[k%4][k/4];
        out[k]=if walsh{crate::vp8_coeff::wrap(v as i64)}else{v};k=k+1;
    }
    out
}
#[verifier::spinoff_prover]
fn vector_transform(c:&[i32;16],walsh:bool) -> (out:[i32;16])
    requires forall|k:int|0<=k<16 ==> -32768<=#[trigger] c@[k]<=32767,
    ensures forall|k:int|0<=k<16 ==> #[trigger] out@[k]==sample(c@,k,walsh) && -1250000<=out@[k]<=1250000 && (walsh ==> -32768<=out@[k]<=32767),
{
    let t=pass4([c[0],c[1],c[2],c[3]],[c[4],c[5],c[6],c[7]],
                [c[8],c[9],c[10],c[11]],[c[12],c[13],c[14],c[15]],0,walsh,32768);
    assert forall|y:int,x:int|0<=y<4 && 0<=x<4 implies
        #[trigger] t@[y]@[x]==vertical(c@,x,y,walsh) by {}
    let cols=transpose(t);
    let h=pass4(cols[0],cols[1],cols[2],cols[3],if walsh{3}else{4},walsh,200000);
    let out=rounded_rows(h,walsh);
    assert forall|k:int|0<=k<16 implies #[trigger] out@[k]==sample(c@,k,walsh) by {
        let x=k%4;let y=k/4;
        assert(t@[y]@[0]==vertical(c@,0,y,walsh));
        assert(t@[y]@[1]==vertical(c@,1,y,walsh));
        assert(t@[y]@[2]==vertical(c@,2,y,walsh));
        assert(t@[y]@[3]==vertical(c@,3,y,walsh));
        assert(cols@[0]@[y]==t@[y]@[0]);assert(cols@[1]@[y]==t@[y]@[1]);
        assert(cols@[2]@[y]==t@[y]@[2]);assert(cols@[3]@[y]==t@[y]@[3]);
    }
    out
}
#[verifier::spinoff_prover]
pub(super) fn transform(c:&[i32;16],walsh:bool) -> (out:[i32;16])
    requires forall|k:int|0<=k<16 ==> -32768<=#[trigger] c@[k]<=32767,
    ensures forall|k:int|0<=k<16 ==> #[trigger] out@[k]==sample(c@,k,walsh) && -1250000<=out@[k]<=1250000 && (walsh ==> -32768<=out@[k]<=32767),
{
    let mut zero=true;let mut j=1usize;
    while j<16
        invariant 1<=j<=16,zero==(forall|k:int|1<=k<j ==> c@[k]==0),
        decreases 16-j,
    {zero=zero&&c[j]==0;j=j+1;}
    if zero{
        let v=div8(c[0] as i64+if walsh{3}else{4});
        let v=if walsh{crate::vp8_coeff::wrap(v)}else{v as i32};
        let out=[v;16];
        assert forall|k:int|0<=k<16 implies out@[k]==sample(c@,k,walsh) && -1250000<=out@[k]<=1250000 && (walsh ==> -32768<=out@[k]<=32767) by {
            assert(c@[1]==0&&c@[2]==0&&c@[3]==0);
            assert(c@[4]==0&&c@[5]==0&&c@[6]==0&&c@[7]==0);
            assert(c@[8]==0&&c@[9]==0&&c@[10]==0&&c@[11]==0);
            assert(c@[12]==0&&c@[13]==0&&c@[14]==0&&c@[15]==0);
        }
        return out;
    }
    if verified_tiff_simd::available(){return vector_transform(c,walsh);}
    let mut t=[0i64;16];let mut i=0usize;
    while i<16
        invariant i<=16,forall|k:int|0<=k<16 ==> -32768<=#[trigger] c@[k]<=32767,
            forall|k:int|0<=k<i ==> #[trigger] t@[k]==vertical(c@,k%4,k/4,walsh) && -200000<=t@[k]<=200000,
        decreases 16-i,
    {
        let x=i%4;let y=i/4;
        t[i]=line(c[x] as i64,c[4+x] as i64,c[8+x] as i64,c[12+x] as i64,y,0,walsh,32768);i=i+1;
    }
    let mut out=[0i32;16];let mut i=0usize;
    while i<16
        invariant i<=16,forall|k:int|0<=k<16 ==> #[trigger] t@[k]==vertical(c@,k%4,k/4,walsh) && -200000<=t@[k]<=200000,
            forall|k:int|0<=k<i ==> #[trigger] out@[k]==sample(c@,k,walsh) && -1250000<=out@[k]<=1250000 && (walsh ==> -32768<=out@[k]<=32767),
        decreases 16-i,
    {
        let y=i/4;let x=i%4;let j=4*y;
        assert((j+1)%4==1 && (j+2)%4==2 && (j+3)%4==3);
        let v=line(t[j],t[j+1],t[j+2],t[j+3],x,if walsh{3}else{4},walsh,200000);
        let v=div8(v);out[i]=if walsh{crate::vp8_coeff::wrap(v)}else{v as i32};i=i+1;
    }
    out
}
}
