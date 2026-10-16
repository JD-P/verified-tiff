use super::*;
use crate::deflate;
verus! {

pub(super) open spec fn capacity(g:Geometry) -> int {
    g.row_bytes()*(if !g.tiled && g.block_height>g.height {g.height as int} else {g.block_height as int})
}
pub(super) open spec fn quota(count:int,cap:int) -> int {8*count+2*cap+64}
pub(super) open spec fn decoded(s:Seq<u8>,g:Geometry,j:int) -> Option<Seq<u8>> {
    let start=tag_value(s,g.offsets,j,g.le);let count=tag_value(s,g.counts,j,g.le);
    deflate::result(s,start,count,capacity(g),quota(count,capacity(g)) as nat)
}
pub(super) open spec fn raw(s:Seq<u8>,g:Geometry,k:int) -> u8 {
    decoded(s,g,g.segment(k)).unwrap()[g.local(k)]
}
pub(super) open spec fn segment_ok(s:Seq<u8>,g:Geometry,j:int) -> bool {
    &&& value_valid(s,g.offsets,j) && value_valid(s,g.counts,j)
    &&& range_ok(s.len() as int,tag_value(s,g.offsets,j,g.le),tag_value(s,g.counts,j,g.le))
    &&& 0<=quota(tag_value(s,g.counts,j,g.le),capacity(g))<usize::MAX
    &&& decoded(s,g,j).is_some()
    &&& storage_bytes(g,g.tiled,j)<=decoded(s,g,j).unwrap().len()<=capacity(g)
}
pub(super) open spec fn ready(s:Seq<u8>,g:Geometry,len:int) -> bool {
    &&& g.positive() && !g.bilevel && !g.lsb && !g.white && g.alpha==0 && !g.packbits && !g.lzw
    &&& len==g.width as int*g.height*g.channels && 0<len<=usize::MAX
    &&& g.across as int==(g.width as int-1)/(g.block_width as int)+1
    &&& (!g.tiled ==> g.block_width==g.width)
    &&& 0<g.row_bytes()<=capacity(g)<=usize::MAX
    &&& segment_count(g)<=usize::MAX && g.counts.count==segment_count(g) && g.offsets.count==segment_count(g)
    &&& forall|j:int| 0<=j<segment_count(g) ==> segment_ok(s,g,j)
    &&& forall|k:int| 0<=k<len ==> 0<=#[trigger] g.segment(k)<segment_count(g) && 0<=g.local(k)<storage_bytes(g,g.tiled,g.segment(k))
}
proof fn covers(g:Geometry,k:int)
    requires g.positive(),!g.bilevel,
        g.across as int==(g.width as int-1)/(g.block_width as int)+1,
        !g.tiled ==> g.block_width==g.width,
        0<=k<g.width as int*g.height*g.channels,
    ensures 0<=g.segment(k)<segment_count(g),0<=g.local(k)<storage_bytes(g,g.tiled,g.segment(k)),
{
    let w=g.width as int;let h=g.height as int;let ch=g.channels as int;
    let bw=g.block_width as int;let bh=g.block_height as int;let a=g.across as int;
    let p=k/ch;let c=k%ch;quotient_remainder(k,ch);
    assert(0<=p<w*h) by(nonlinear_arith) requires ch>0,0<=c<ch,k==ch*p+c,0<=k<w*h*ch;
    let x=p%w;let y=p/w;quotient_remainder(p,w);
    assert(0<=y<h) by(nonlinear_arith) requires w>0,0<=x<w,p==w*y+x,0<=p<w*h;
    let bx=x/bw;let by=y/bh;let lx=x%bw;let ly=y%bh;
    quotient_remainder(x,bw);quotient_remainder(y,bh);
    vstd::arithmetic::div_mod::lemma_div_is_ordered(x,w-1,bw);
    vstd::arithmetic::div_mod::lemma_div_is_ordered(y,h-1,bh);
    let down=(h-1)/bh+1;let j=by*a+bx;
    assert(0<=j<a*down) by(nonlinear_arith) requires a>0,0<=bx<a,0<=by<down,j==by*a+bx;
    if !g.tiled {
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(w-1,w,0,w-1);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(x,w,0,x);
        assert(j==by) by(nonlinear_arith) requires a==1,bx==0,j==by*a+bx;
    }
    let rows=stored_rows(g,g.tiled,j);
    assert(0<=ly<rows) by(nonlinear_arith)
        requires 0<=ly<bh,0<=y<h,y==bh*by+ly,g.tiled ==> rows==bh,
            !g.tiled ==> j==by,!g.tiled ==> rows==(if h-j*bh<bh {h-j*bh} else {bh});
    assert(0<=(ly*bw+lx)*ch+c<bw*rows*ch) by(nonlinear_arith)
        requires bw>0,ch>0,0<=ly<rows,0<=lx<bw,0<=c<ch;
}
fn cap_size(g:Geometry) -> (r:Result<usize,Error>)
    requires g.positive(),!g.bilevel,
    ensures r.is_ok()==(capacity(g)<=usize::MAX),r.is_ok() ==> r.unwrap()==capacity(g),
        r.is_ok() ==> 0<g.row_bytes()<=r.unwrap(),
{
    let rows=if !g.tiled && g.block_height>g.height {g.height} else {g.block_height};
    assert(g.block_width as int*g.channels<=capacity(g)) by(nonlinear_arith) requires g.block_width>0,g.channels>0,rows>0,capacity(g)==g.block_width as int*g.channels*rows;
    let stride=mul(g.block_width,g.channels)?;
    let cap=mul(stride,rows)?;
    assert(0<stride<=cap) by(nonlinear_arith) requires g.block_width>0,g.channels>0,rows>0,stride as int==g.block_width as int*g.channels,cap as int==stride as int*rows;
    Ok(cap)
}
fn work_size(count:usize,cap:usize) -> (r:Result<usize,Error>)
    ensures r.is_ok()==(quota(count as int,cap as int)<=usize::MAX),r.is_ok() ==> r.unwrap()==quota(count as int,cap as int),
{add(add(mul(count,8)?,mul(cap,2)?)?,64)}

#[verifier::opaque]
pub(super) open spec fn work_prefix(s:Seq<u8>,g:Geometry,n:nat) -> int
    decreases n,
{
    if n==0 {0} else {work_prefix(s,g,(n-1) as nat)+quota(tag_value(s,g.counts,n as int-1,g.le),capacity(g))+1}
}
pub(super) proof fn work_step(s:Seq<u8>,g:Geometry,n:nat)
    ensures work_prefix(s,g,n+1)==work_prefix(s,g,n)+quota(tag_value(s,g.counts,n as int,g.le),capacity(g))+1,
{reveal_with_fuel(work_prefix,1);}
pub(super) proof fn work_monotone(s:Seq<u8>,g:Geometry,n:nat,m:nat)
    requires n<=m,forall|j:int| 0<=j<m ==> segment_ok(s,g,j),
    ensures 0<=work_prefix(s,g,n)<=work_prefix(s,g,m),
    decreases m,
{
    reveal_with_fuel(work_prefix,1);
    if m>0 {
        if n<m {work_monotone(s,g,n,(m-1) as nat);}
        else {work_monotone(s,g,(m-1) as nat,(m-1) as nat);}
        assert(segment_ok(s,g,m as int-1));
    }
}
#[verifier::opaque]
pub(super) open spec fn profile(s:Seq<u8>,g:Geometry,max_output:usize) -> bool {
    &&& 0<g.row_bytes()<=capacity(g)<=max_output
    &&& forall|j:int| 0<=j<segment_count(g) ==> segment_ok(s,g,j)
    &&& 0<=work_prefix(s,g,segment_count(g) as nat)<=usize::MAX
}

proof fn strip_position(g:Geometry,k:int)
    requires g.positive(),!g.bilevel,!g.tiled,g.block_width==g.width,
        g.across as int==(g.width as int-1)/(g.block_width as int)+1,
        0<=k<g.width as int*g.height*g.channels,
    ensures g.across==1,
        k==g.segment(k)*(g.block_height as int)*g.row_bytes()+g.local(k),
        0<=g.local(k)<g.block_height as int*g.row_bytes(),
{
    let row=g.width as int*g.channels;let y=k/row;let i=k%row;
    assert(row>0) by(nonlinear_arith) requires g.width>0,g.channels>0,row==g.width as int*g.channels;
    quotient_remainder(k,row);
    assert(k==y*row+i) by(nonlinear_arith) requires k==row*y+i;
    assert(y<g.height) by(nonlinear_arith)
        requires k<g.width as int*g.height*g.channels,row==g.width as int*g.channels,k==row*y+i,i>=0,row>0;
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(g.width as int-1,g.width as int,0,g.width as int-1);
    assert(k==y*g.width*g.channels+i) by(nonlinear_arith) requires k==row*y+i,row==g.width as int*g.channels;
    strip_row_mapping(g,y,i);quotient_remainder(y,g.block_height as int);
    assert(g.local(k)==(y%(g.block_height as int))*row+i) by(nonlinear_arith)
        requires g.local(k)==(y%(g.block_height as int))*g.width*g.channels+i,row==g.width as int*g.channels;
    assert(k==g.segment(k)*(g.block_height as int)*row+g.local(k)) by(nonlinear_arith)
        requires k==y*row+i,g.segment(k)==y/(g.block_height as int),g.local(k)==(y%(g.block_height as int))*row+i,
            y==g.block_height as int*(y/(g.block_height as int))+y%(g.block_height as int);
    assert(0<=g.local(k)<g.block_height as int*row) by(nonlinear_arith)
        requires g.local(k)==(y%(g.block_height as int))*row+i,0<=y%(g.block_height as int)<g.block_height,0<=i<row,row>0;
}

pub(super) fn validate(s:&[u8],g:Geometry,len:usize,max_output:usize,budget:&mut Budget) -> (r:Result<usize,Error>)
    requires g.positive(),!g.bilevel,!g.lsb,!g.white,g.alpha==0,!g.packbits,!g.lzw,
        len as int==g.width as int*g.height*g.channels,0<len,
        g.across as int==(g.width as int-1)/(g.block_width as int)+1,
        !g.tiled ==> g.block_width==g.width,
        g.offsets.count==segment_count(g),g.counts.count==segment_count(g),segment_count(g)<=usize::MAX,
    ensures final(budget).remaining<=old(budget).remaining,r.is_ok() ==> ready(s@,g,len as int),
        old(budget).remaining as int-final(budget).remaining<=segment_count(g)*(quota(s.len() as int,max_output as int)+1),
        r.is_ok() ==> r.unwrap() as int==work_prefix(s@,g,segment_count(g) as nat),
        r.is_ok() ==> r.unwrap() as int==old(budget).remaining as int-final(budget).remaining,
        profile(s@,g,max_output) && old(budget).remaining as int>=segment_count(g)*(quota(s.len() as int,max_output as int)+1) ==> r.is_ok(),
{
    let mut output=[0u8;0];
    validate_into(s,g,len,max_output,budget,&mut output,false)
}

pub(super) fn validate_into(s:&[u8],g:Geometry,len:usize,max_output:usize,budget:&mut Budget,output:&mut [u8],write:bool) -> (r:Result<usize,Error>)
    requires write ==> !g.tiled && len<=old(output).len(),
        g.positive(),!g.bilevel,!g.lsb,!g.white,g.alpha==0,!g.packbits,!g.lzw,
        len as int==g.width as int*g.height*g.channels,0<len,
        g.across as int==(g.width as int-1)/(g.block_width as int)+1,
        !g.tiled ==> g.block_width==g.width,
        g.offsets.count==segment_count(g),g.counts.count==segment_count(g),segment_count(g)<=usize::MAX,
    ensures final(output).len()==old(output).len(),
        forall|k:int| 0<=k<old(output).len() && (!write || k>=len) ==> final(output)@[k]==old(output)@[k],
        r.is_ok() && write ==> forall|k:int| 0<=k<len ==> final(output)@[k]==raw(s@,g,k),
        final(budget).remaining<=old(budget).remaining,r.is_ok() ==> ready(s@,g,len as int),
        old(budget).remaining as int-final(budget).remaining<=segment_count(g)*(quota(s.len() as int,max_output as int)+1),
        r.is_ok() ==> r.unwrap() as int==work_prefix(s@,g,segment_count(g) as nat),
        r.is_ok() ==> r.unwrap() as int==old(budget).remaining as int-final(budget).remaining,
        profile(s@,g,max_output) && old(budget).remaining as int>=segment_count(g)*(quota(s.len() as int,max_output as int)+1) ==> r.is_ok(),
{
    let ghost eligible=profile(s@,g,max_output) && budget.remaining as int>=segment_count(g)*(quota(s.len() as int,max_output as int)+1);
    let ghost initial=budget.remaining;
    let ghost unit=quota(s.len() as int,max_output as int)+1;
    reveal(profile);
    let cap=cap_size(g)?;if cap>max_output {return Err(Error::Limit);}
    let stride=mul(g.block_width,g.channels)?;
    let mut j=0usize;let mut work=0usize;
    reveal_with_fuel(work_prefix,1);
    while j<g.counts.count
        invariant initial==old(budget).remaining,eligible==(profile(s@,g,max_output) && old(budget).remaining as int>=segment_count(g)*unit),
            unit==quota(s.len() as int,max_output as int)+1,unit>0,cap<=max_output,
            old(budget).remaining as int-budget.remaining<=j as int*unit,
            work as int==work_prefix(s@,g,j as nat),work as int==old(budget).remaining as int-budget.remaining,
            eligible ==> profile(s@,g,max_output),
            j<=g.counts.count,g.positive(),!g.bilevel,!g.lsb,!g.white,g.alpha==0,!g.packbits,!g.lzw,
            len as int==g.width as int*g.height*g.channels,0<len,
            g.across as int==(g.width as int-1)/(g.block_width as int)+1,
            !g.tiled ==> g.block_width==g.width,
            g.offsets.count==segment_count(g),g.counts.count==segment_count(g),segment_count(g)<=usize::MAX,
            cap as int==capacity(g),0<g.row_bytes()<=cap,stride as int==g.row_bytes(),
            budget.remaining<=old(budget).remaining,
            forall|q:int| 0<=q<j ==> segment_ok(s@,g,q),
            output.len()==old(output).len(),write ==> !g.tiled && len<=output.len(),
            forall|k:int| 0<=k<output.len() && (!write || k>=len) ==> output@[k]==old(output)@[k],
            write ==> forall|k:int| 0<=k<len && g.segment(k)<j ==> output@[k]==raw(s@,g,k),
        decreases g.counts.count-j,
    {
        reveal(profile);
        assert(eligible ==> segment_ok(s@,g,j as int));
        proof {if eligible {
            traversal_budget(old(budget).remaining as int,budget.remaining as int,j as int,g.counts.count as int-1,unit-1,0);
        }}
        let ghost before=budget.remaining;
        assert((j as int+1)*unit<=segment_count(g)*unit) by(nonlinear_arith)
            requires j<g.counts.count,g.counts.count==segment_count(g),unit>0;
        assert(initial as int-before+unit<=segment_count(g)*unit) by(nonlinear_arith)
            requires initial as int-before<=j as int*unit,j<g.counts.count,g.counts.count==segment_count(g),unit>0;
        spend(budget,1)?;
        let start=value(s,g.offsets,j,g.le)?;let count=value(s,g.counts,j,g.le)?;
        if !checked_range(s.len(),start,count) {return Err(Error::Truncated);}
        let fuel=work_size(count,cap)?;
        let charged=add(fuel,1)?;
        spend(budget,fuel)?;

        proof {if eligible && !g.tiled {
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(g.width as int-1,g.width as int,0,g.width as int-1);
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod(g.height as int-1,g.block_height as int);
            vstd::arithmetic::div_mod::lemma_mod_pos_bound(g.height as int-1,g.block_height as int);
            assert(segment_count(g)==(g.height as int-1)/(g.block_height as int)+1) by(nonlinear_arith)
                requires g.across==1,segment_count(g)==g.across as int*((g.height as int-1)/(g.block_height as int)+1);
            assert(j as int*g.block_height<g.height) by(nonlinear_arith)
                requires g.across==1,j<segment_count(g),g.block_height>0,
                    (g.height as int-1)==g.block_height as int*((g.height as int-1)/(g.block_height as int))+(g.height as int-1)%(g.block_height as int),
                    0<=(g.height as int-1)%(g.block_height as int)<g.block_height,
                    segment_count(g)==(g.height as int-1)/(g.block_height as int)+1;
        }}
        let rows=if g.tiled {g.block_height} else {
            let first=mul(j,g.block_height)?;
            if first>=g.height {return Err(Error::Invalid);}
            if g.height-first<g.block_height {g.height-first} else {g.block_height}
        };
        assert(stride as int*rows==storage_bytes(g,g.tiled,j as int)) by(nonlinear_arith)
            requires stride as int==g.block_width as int*g.channels,rows as int==stored_rows(g,g.tiled,j as int),!g.bilevel;
        let needed=mul(stride,rows)?;
        assert(needed as int==storage_bytes(g,g.tiled,j as int)) by(nonlinear_arith) requires needed as int==stride as int*rows,stride as int==g.block_width as int*g.channels,rows as int==stored_rows(g,g.tiled,j as int),!g.bilevel;
        let ghost previous=output@;
        let actual=if write {
            let first=mul(j,g.block_height)?;
            if first>=g.height {return Err(Error::Invalid);}
            proof {
                vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(g.width as int-1,g.width as int,0,g.width as int-1);
                assert(first as int*stride<=len) by(nonlinear_arith)
                    requires first<g.height,stride as int==g.width as int*g.channels,len as int==g.width as int*g.height*g.channels;
                assert(needed as int==rows as int*stride) by(nonlinear_arith) requires needed as int==stride as int*rows;
            }
            let begin=mul(first,stride)?;
            proof {
                assert(g.across==1);
                assert(begin as int+needed<=len) by(nonlinear_arith)
                    requires begin as int==first as int*stride,needed as int==rows as int*stride,
                        rows<=g.height-first,len as int==g.width as int*g.height*g.channels,stride as int==g.width as int*g.channels;
            }
            let (_,tail)=output.split_at_mut(begin);let (part,_)=tail.split_at_mut(needed);
            let actual=deflate::process(s,start,count,cap,fuel,part,needed)?;
            assert forall|k:int| 0<=k<len && g.segment(k)<j implies output@[k]==previous[k] by {
                strip_position(g,k);
                assert(k<begin) by(nonlinear_arith)
                    requires k==g.segment(k)*(g.block_height as int)*stride+g.local(k),
                        0<=g.local(k)<g.block_height as int*stride,g.segment(k)<j,
                        begin as int==first as int*stride,first as int==j as int*g.block_height,stride>0,g.block_height>0;
            }
            assert forall|k:int| 0<=k<len && g.segment(k)==j && g.local(k)<actual implies output@[k]==raw(s@,g,k) by {
                strip_position(g,k);
                assert(begin as int==j as int*g.block_height*stride) by(nonlinear_arith)
                    requires begin as int==first as int*stride,first as int==j as int*g.block_height;
                covers(g,k);
                assert(output@[k]==decoded(s@,g,j as int).unwrap()[g.local(k)]);
            }
            actual
        } else {deflate::validate(s,start,count,cap,fuel)?};
        if actual<needed {return Err(Error::Invalid);}
        assert(segment_ok(s@,g,j as int));
        proof {
            if write {
                assert forall|k:int| 0<=k<len && g.segment(k)<j as int+1 implies output@[k]==raw(s@,g,k) by {
                    if g.segment(k)==j {covers(g,k);assert(g.local(k)<needed);}
                    else {assert(output@[k]==previous[k]);}
                }
            }
            if eligible {work_monotone(s@,g,(j+1) as nat,segment_count(g) as nat);}
            work_step(s@,g,j as nat);
        }
        work=add(work,charged)?;
        assert((j as int+1)*unit==j as int*unit+unit) by(nonlinear_arith);
        j=j+1;
    }
    assert forall|k:int| 0<=k<len implies 0<=#[trigger] g.segment(k)<segment_count(g) && 0<=g.local(k)<storage_bytes(g,g.tiled,g.segment(k)) by {covers(g,k);}
    Ok(work)
}

fn decode_segment(s:&[u8],g:Geometry,len:usize,j:usize,cap:usize,stride:usize,output:&mut [u8]) -> (r:Result<(),Error>)
    requires ready(s@,g,len as int),j<segment_count(g),cap as int==capacity(g),stride as int==g.row_bytes(),len<=old(output).len(),
    ensures r.is_ok(),final(output).len()==old(output).len(),
        forall|k:int| 0<=k<len ==> final(output)@[k]==if g.segment(k)==j {raw(s@,g,k)} else {old(output)@[k]},
        forall|k:int| len<=k<old(output).len() ==> final(output)@[k]==old(output)@[k],
{
    assert(segment_ok(s@,g,j as int));
    let start=value(s,g.offsets,j,g.le)?;let count=value(s,g.counts,j,g.le)?;
    let fuel=work_size(count,cap)?;
    if !g.tiled {
        proof {
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(g.width as int-1,g.width as int,0,g.width as int-1);
            quotient_remainder(g.height as int-1,g.block_height as int);
            assert(0<=j as int*g.block_height<g.height) by(nonlinear_arith)
                requires g.across==1,j<segment_count(g),g.block_height>0,
                    segment_count(g)==g.across as int*((g.height as int-1)/(g.block_height as int)+1),
                    g.height as int-1==g.block_height as int*((g.height as int-1)/(g.block_height as int))+(g.height as int-1)%(g.block_height as int),
                    0<=(g.height as int-1)%(g.block_height as int)<g.block_height;
        }
        let first=j*g.block_height;
        let rows=if g.height-first<g.block_height {g.height-first} else {g.block_height};
        proof {
            assert(0<=first as int*stride<=first as int*stride+rows as int*stride<=len) by(nonlinear_arith)
                requires first<g.height,0<rows<=g.height-first,stride as int==g.width as int*g.channels,
                    len as int==g.width as int*g.height*g.channels;
        }
        let begin=first*stride;let needed=rows*stride;
        assert(needed as int==storage_bytes(g,false,j as int)) by(nonlinear_arith)
            requires stride as int==g.block_width as int*g.channels,rows as int==stored_rows(g,false,j as int),needed as int==rows as int*stride,!g.bilevel;
        {
            let (_,tail)=output.split_at_mut(begin);let (part,_)=tail.split_at_mut(needed);
            let actual=deflate::process(s,start,count,cap,fuel,part,needed)?;
            assert(actual>=needed);
            assert forall|i:int| 0<=i<needed implies part@[i]==decoded(s@,g,j as int).unwrap()[i] by {};
        }
        assert forall|k:int| 0<=k<len implies output@[k]==if g.segment(k)==j {raw(s@,g,k)} else {old(output)@[k]} by {
            strip_position(g,k);covers(g,k);
            assert(begin as int==j as int*g.block_height*stride) by(nonlinear_arith)
                requires begin as int==first as int*stride,first as int==j as int*g.block_height;
            if g.segment(k)==j {
                assert(k==begin as int+g.local(k)) by(nonlinear_arith)
                    requires k==g.segment(k)*(g.block_height as int)*stride+g.local(k),g.segment(k)==j,begin as int==j as int*g.block_height*stride;
                assert(output@[k]==decoded(s@,g,j as int).unwrap()[g.local(k)]);
            } else {
                assert(k<begin || k>=begin as int+needed) by(nonlinear_arith)
                    requires k==g.segment(k)*(g.block_height as int)*stride+g.local(k),
                        g.segment(k)!=j,0<=g.local(k)<g.block_height as int*stride,
                        begin as int==j as int*g.block_height*stride,needed as int==rows as int*stride,
                        0<rows<=g.block_height,stride>0,g.block_height>0;
            }
        }
        return Ok(());
    }
    reveal(deflate::result);
    let mut c=deflate::Cursor::new(s,start,count,cap)?;
    let mut left=fuel;
    while !c.done()
        invariant ready(s@,g,len as int),j<segment_count(g),cap as int==capacity(g),stride as int==g.row_bytes(),
            output.len()==old(output).len(),len<=output.len(),
            start as int==tag_value(s@,g.offsets,j as int,g.le),count as int==tag_value(s@,g.counts,j as int,g.le),
            c.valid(s@,cap as int),c.end()==start as int+count-4,left<=fuel,
            segment_ok(s@,g,j as int),
            decoded(s@,g,j as int)==deflate::finish(s@,c.end(),cap as int,c.model(),left as nat),
            forall|k:int| 0<=k<len ==> output@[k]==if g.segment(k)==j && g.local(k)<c.produced {raw(s@,g,k)} else {old(output)@[k]},
            forall|k:int| len<=k<old(output).len() ==> output@[k]==old(output)@[k],
        decreases left,
    {
        reveal_with_fuel(deflate::finish,1);
        assert(left>0);
        let i=c.produced;
        let next=c.step(s,cap)?;left=left-1;
        if let Some(byte)=next {
            proof {deflate::finish_prefix(s@,c.end(),cap as int,c.model(),left as nat);}
            assert(byte==decoded(s@,g,j as int).unwrap()[i as int]);
            scatter_byte(g,j,i,stride,byte,output);
        }
    }
    reveal_with_fuel(deflate::finish,1);
    assert(c.produced==decoded(s@,g,j as int).unwrap().len());
    Ok(())
}
pub(super) fn decode(s:&[u8],g:Geometry,len:usize,output:&mut [u8]) -> (r:Result<(),Error>)
    requires ready(s@,g,len as int),len<=old(output).len(),
    ensures r.is_ok(),final(output).len()==old(output).len(),
        forall|k:int| 0<=k<len ==> final(output)@[k]==raw(s@,g,k),
        forall|k:int| len<=k<old(output).len() ==> final(output)@[k]==old(output)@[k],
{
    let cap=cap_size(g)?;let stride=mul(g.block_width,g.channels)?;
    let mut j=0usize;
    while j<g.counts.count
        invariant ready(s@,g,len as int),j<=g.counts.count,cap as int==capacity(g),stride as int==g.row_bytes(),
            output.len()==old(output).len(),len<=output.len(),
            forall|k:int| 0<=k<len && g.segment(k)<j ==> output@[k]==raw(s@,g,k),
            forall|k:int| len<=k<old(output).len() ==> output@[k]==old(output)@[k],
        decreases g.counts.count-j,
    {decode_segment(s,g,len,j,cap,stride,output)?;j=j+1;}
    Ok(())
}
}
