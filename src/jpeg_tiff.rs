use super::*;
use crate::jpeg_meta as jm;
use crate::jpeg_format as jf;
use crate::jpeg_container as jc;
use crate::jpeg_segment as seg;
use crate::jpeg_scan as js;
use crate::jpeg as j;
verus! {
pub(super) open spec fn metadata_model(s:Seq<u8>,off:int,le:bool) -> jm::Meta {
    choose|m:jm::Meta| m.off==off && m.le==le && m.matches(s)
}
proof fn metadata_unique(s:Seq<u8>,m:jm::Meta)
    requires m.matches(s),ensures m==metadata_model(s,m.off as int,m.le),
{
    assert(exists|a:jm::Meta| a.off==m.off && a.le==m.le && a.matches(s));
    let a=metadata_model(s,m.off as int,m.le);assert(a.tags@==m.tags@);assert(a.tags==m.tags);
}
pub(super) open spec fn geometry(f:jf::Format) -> Geometry {
    Geometry {width:f.w,height:f.h,channels:f.ch,alpha:0,block_width:f.bw,block_height:f.bh,across:f.across,
        offsets:f.offsets,counts:f.counts,le:f.le,white:false,bilevel:false,lsb:false,packbits:false,lzw:false,tiled:f.tiled}
}
fn make_geometry(f:jf::Format) -> (r:Geometry) ensures r==geometry(f), {
    Geometry {width:f.w,height:f.h,channels:f.ch,alpha:0,block_width:f.bw,block_height:f.bh,across:f.across,
        offsets:f.offsets,counts:f.counts,le:f.le,white:false,bilevel:false,lsb:false,packbits:false,lzw:false,tiled:f.tiled}
}
pub(super) open spec fn header_fee(s:Seq<u8>,f:jf::Format,k:int) -> int {2*tag_value(s,f.counts,k,f.le)+1}
pub(super) open spec fn segment_fee(s:Seq<u8>,f:jf::Format,base:Seq<usize>,k:int) -> int {
    let p=seg::plan(s,f,base,k).unwrap();header_fee(s,f,k)+j::work(p.v)+j::pixel_count(p.v)
}
pub(super) open spec fn segment_scratch(s:Seq<u8>,f:jf::Format,base:Seq<usize>,k:int) -> int {
    let p=seg::plan(s,f,base,k).unwrap();64*js::total(p.v)+j::pixel_count(p.v)
}
#[verifier::opaque]
pub(super) open spec fn segment_supported(s:Seq<u8>,f:jf::Format,base:Seq<usize>,k:int,l:Limits) -> bool {
    let p=seg::plan(s,f,base,k);
    p.is_some() && j::eligible(s,p.unwrap().v,p.unwrap().bits,f.compression==6,l) &&
    0<=segment_scratch(s,f,base,k)<=l.max_output && 0<=segment_fee(s,f,base,k)<=usize::MAX
}
#[verifier::opaque]
pub(super) open spec fn work_prefix(s:Seq<u8>,f:jf::Format,base:Seq<usize>,n:nat) -> int decreases n {
    if n==0 {0} else {work_prefix(s,f,base,(n-1) as nat)+segment_fee(s,f,base,n as int-1)}
}
#[verifier::opaque]
pub(super) open spec fn scratch_prefix(s:Seq<u8>,f:jf::Format,base:Seq<usize>,n:nat) -> int decreases n {
    if n==0 {0} else {let a=scratch_prefix(s,f,base,(n-1) as nat);let b=segment_scratch(s,f,base,n as int-1);if a>b {a} else {b}}
}
proof fn prefixes(s:Seq<u8>,f:jf::Format,base:Seq<usize>,a:nat,n:nat,l:Limits)
    requires a<=n,forall|k:int| 0<=k<n ==> segment_supported(s,f,base,k,l),
    ensures 0<=work_prefix(s,f,base,a)<=work_prefix(s,f,base,n),0<=scratch_prefix(s,f,base,n)<=l.max_output,
        forall|k:int| 0<=k<n ==> segment_scratch(s,f,base,k)<=scratch_prefix(s,f,base,n),
    decreases n,
{
    reveal_with_fuel(work_prefix,1);reveal_with_fuel(scratch_prefix,1);reveal(segment_supported);
    if n>0 {
        assert(segment_supported(s,f,base,n as int-1,l));
        prefixes(s,f,base,if a<n {a} else {(a-1) as nat},(n-1) as nat,l);
        assert(work_prefix(s,f,base,n)==work_prefix(s,f,base,(n-1) as nat)+segment_fee(s,f,base,n as int-1));
        assert(0<=segment_fee(s,f,base,n as int-1));
        assert(0<=segment_scratch(s,f,base,n as int-1)<=l.max_output);
    }
}
#[verifier::opaque]
pub closed spec fn supported_jpeg_profile(s:Seq<u8>,page:usize,l:Limits) -> bool {
    let le=s[0]==73;let off=nth_ifd(s,word32(s,4,le),le,page as nat);let m=metadata_model(s,off,le);let f=jf::model(s,m);let b=jc::base_model(s,f);
    &&& header_valid(s) && s.len()<=l.max_input && page<l.max_directories
    &&& forall|p:nat| p<=page ==> nth_ifd(s,word32(s,4,le),le,p)!=0 && jm::directory_ok(s,nth_ifd(s,word32(s,4,le),le,p),le,l.max_tags as int)
    &&& jf::fields(s,m,l) && b.is_some()
    &&& forall|k:int| 0<=k<f.segments ==> segment_supported(s,f,b.unwrap(),k,l)
    &&& 0<=work_prefix(s,f,b.unwrap(),f.segments as nat)<=usize::MAX
}
pub(super) proof fn profile_from_parts(s:Seq<u8>,m:jm::Meta,l:Limits)
    requires header_valid(s),s.len()<=l.max_input,0<l.max_directories,m.le==(s[0]==73),m.off==word32(s,4,m.le),m.off!=0,
        m.matches(s),jm::directory_ok(s,m.off as int,m.le,l.max_tags as int),jf::fields(s,m,l),jc::base_model(s,jf::model(s,m)).is_some(),
        forall|k:int| 0<=k<jf::model(s,m).segments ==> segment_supported(s,jf::model(s,m),jc::base_model(s,jf::model(s,m)).unwrap(),k,l),
        0<=work_prefix(s,jf::model(s,m),jc::base_model(s,jf::model(s,m)).unwrap(),jf::model(s,m).segments as nat)<=usize::MAX,
    ensures supported_jpeg_profile(s,0,l),
{
    metadata_unique(s,m);reveal(supported_jpeg_profile);reveal_with_fuel(nth_ifd,1);
}
proof fn profile_directory(s:Seq<u8>,page:usize,l:Limits,p:nat)
    requires supported_jpeg_profile(s,page,l),p<=page,
    ensures nth_ifd(s,word32(s,4,s[0]==73),s[0]==73,p)!=0,
        jm::directory_ok(s,nth_ifd(s,word32(s,4,s[0]==73),s[0]==73,p),s[0]==73,l.max_tags as int),
{reveal(supported_jpeg_profile);}
pub open spec fn jpeg_inspection_work_bound(page:usize,l:Limits) -> int {
    (page as int+1)*(l.max_tags as int+1)+4*(l.max_input as int)+128+
    l.max_segments as int*(2*(l.max_input as int)+1+3138*(l.max_output as int))
}
pub fn jpeg_inspection_work_limit(page:usize,l:&Limits) -> (r:Result<usize,Error>)
    ensures r.is_ok()==(jpeg_inspection_work_bound(page,*l)<=usize::MAX),r.is_ok() ==> r.unwrap()==jpeg_inspection_work_bound(page,*l),
{
    let ghost p=page as int;let ghost t=l.max_tags as int;let ghost ms=l.max_segments as int;let ghost mi=l.max_input as int;let ghost mo=l.max_output as int;
    assert(p+1<=(p+1)*(t+1) && t+1<=(p+1)*(t+1)) by(nonlinear_arith) requires p>=0,t>=0;
    assert(ms*(2*mi+1+3138*mo)==2*(ms*mi)+ms+3138*(ms*mo)) by(nonlinear_arith);
    assert(0<=ms*mi<=2*(ms*mi) && 0<=ms*mo<=3138*(ms*mo)) by(nonlinear_arith) requires ms>=0,mi>=0,mo>=0;
    let d=mul(add(page,1)?,add(l.max_tags,1)?)?;
    // Expand products so a zero segment budget does not introduce false overflow.
    let a=mul(mul(l.max_segments,l.max_input)?,2)?;let b=mul(mul(l.max_segments,l.max_output)?,3138)?;
    add(add(add(add(add(d,mul(l.max_input,4)?)?,128)?,a)?,l.max_segments)?,b)
}
pub struct JpegImage<'a> {
    input:&'a [u8],meta:jm::Meta,format:jf::Format,base:[usize;32],page:usize,limits:Limits,scratch:usize,work:usize,
}
impl<'a> JpegImage<'a> {
    pub closed spec fn metadata_matches_source(&self) -> bool {
        header_valid(self.input@) && self.meta.le==(self.input@[0]==73) && self.meta.matches(self.input@) &&
        self.meta.off==nth_ifd(self.input@,word32(self.input@,4,self.meta.le),self.meta.le,self.page as nat) &&
        self.format==jf::model(self.input@,self.meta) && jf::fields(self.input@,self.meta,self.limits) &&
        jc::base_model(self.input@,self.format)==Some(self.base@)
    }
    pub closed spec fn well_formed(&self) -> bool {
        self.metadata_matches_source() && self.format.valid() && seg::base_ready(self.input@,self.format,self.base@) &&
        (forall|k:int| 0<=k<self.format.segments ==> segment_supported(self.input@,self.format,self.base@,k,self.limits)) &&
        self.scratch==scratch_prefix(self.input@,self.format,self.base@,self.format.segments as nat) && self.scratch<=self.limits.max_output &&
        self.work==work_prefix(self.input@,self.format,self.base@,self.format.segments as nat) && self.format.output<=self.limits.max_output
    }
    pub closed spec fn byte_len(&self) -> nat {self.format.output as nat}
    pub closed spec fn scratch_size(&self) -> nat {self.scratch as nat}
    pub closed spec fn work_len(&self) -> nat {self.work as nat}
    pub closed spec fn decoded_byte(&self,k:int) -> u8 {
        let g=geometry(self.format);let p=seg::plan(self.input@,self.format,self.base@,g.segment(k)).unwrap();
        segment_pixel(self.input@,self.format,p,g.local(k))
    }
    pub fn width(&self) -> usize {self.format.w}
    pub fn height(&self) -> usize {self.format.h}
    pub fn channels(&self) -> usize {self.format.ch}
    pub fn bits_per_sample(&self) -> usize {8}
    pub fn output_bits_per_sample(&self) -> usize {8}
    pub fn planar_configuration(&self) -> usize {1}
    pub fn compression(&self) -> usize {self.format.compression}
    pub fn page_index(&self) -> usize {self.page}
    pub fn directory_entry_count(&self) -> usize {self.meta.count}
    pub fn has_alpha(&self) -> bool {false}
    pub fn alpha_mode(&self) -> AlphaMode {AlphaMode::None}
    pub fn output_len(&self) -> (r:usize) ensures r==self.byte_len(), {self.format.output}
    pub fn scratch_len(&self) -> (r:usize) ensures r==self.scratch_size(), {self.scratch}
    pub fn decode_work_limit(&self) -> (r:usize) ensures r==self.work_len(), {self.work}
}
pub(super) open spec fn segment_sample(s:Seq<u8>,f:jf::Format,p:seg::Model,x:int,y:int,c:int) -> u8 {
    let blocks=js::finish(s,p.v,js::initial(p.bits),js::total(p.v) as nat,f.compression==6).unwrap().blocks;
    let k=crate::jpeg_layout::index(p.v,x,y,c);crate::jpeg_transform::sample(blocks[k/64],k%64)
}
pub(super) open spec fn segment_pixel(s:Seq<u8>,f:jf::Format,p:seg::Model,k:int) -> u8 {
    let ch=p.v[14] as int;let w=p.v[12] as int;let c=k%ch;let x=(k/ch)%w;let y=(k/ch)/w;
    if f.photo==6 {crate::jpeg_color::color(segment_sample(s,f,p,x,y,0),segment_sample(s,f,p,x,y,1),segment_sample(s,f,p,x,y,2),c,f.limited)} else {segment_sample(s,f,p,x,y,c)}
}
#[verifier::opaque]
spec fn per_segment_bound(l:Limits) -> int {2*(l.max_input as int)+1+3138*(l.max_output as int)}
fn inspect_segment(s:&[u8],f:jf::Format,base:[usize;32],k:usize,l:&Limits,budget:&mut Budget) -> (r:Result<(usize,usize),Error>)
    requires f.valid(),k<f.segments,seg::base_ready(s@,f,base@),s.len()<=l.max_input,
    ensures final(budget).remaining<=old(budget).remaining,
        old(budget).remaining as int-final(budget).remaining<=per_segment_bound(*l),
        segment_supported(s@,f,base@,k as int,*l) && old(budget).remaining as int>=per_segment_bound(*l) ==> r.is_ok(),
        r.is_ok() ==> segment_supported(s@,f,base@,k as int,*l),
        r.is_ok() ==> r.unwrap().0 as int==segment_fee(s@,f,base@,k as int) && r.unwrap().1 as int==segment_scratch(s@,f,base@,k as int),
        r.is_ok() ==> r.unwrap().1<=l.max_output,
{
    let ghost eligible=segment_supported(s@,f,base@,k as int,*l) && budget.remaining as int>=per_segment_bound(*l);
    reveal(per_segment_bound);reveal(segment_supported);reveal(seg::plan);reveal(seg::header_model);reveal(j::eligible);
    let count=value(s,f.counts,k,f.le)?;
    if count>s.len() {return Err(Error::Truncated);}
    let fee=add(mul(count,2)?,1)?;spend(budget,fee)?;
    let p=seg::read(s,f,base,k)?;
    assert(eligible ==> j::work(p.v@)<=3137*(l.max_output as int)) by(nonlinear_arith)
        requires eligible ==> 0<=js::total(p.v@) && 64*js::total(p.v@)<=l.max_output && j::pixel_count(p.v@)<=l.max_output,
            j::work(p.v@)==3136*js::total(p.v@)+j::pixel_count(p.v@);
    let image=j::prepare(s,p.v,p.bits,f.compression==6,f.photo==6,f.limited,l,budget)?;
    let need=add(image.scratch,image.output)?;if need>l.max_output {return Err(Error::Limit);}
    let cost=add(add(fee,image.work)?,image.output)?;
    assert(need as int==segment_scratch(s@,f,base@,k as int));assert(cost as int==segment_fee(s@,f,base@,k as int));
    Ok((cost,need))
}
pub fn inspect_jpeg_tiff<'a>(s:&'a [u8],page:usize,l:&Limits,budget:&mut Budget) -> (r:Result<JpegImage<'a>,Error>)
    ensures supported_jpeg_profile(s@,page,*l) && old(budget).remaining as int>=jpeg_inspection_work_bound(page,*l) ==> r.is_ok(),
        final(budget).remaining<=old(budget).remaining,r.is_ok() ==> r.unwrap().well_formed() && r.unwrap().metadata_matches_source(),
        r.is_ok() ==> r.unwrap().byte_len()<=l.max_output && r.unwrap().scratch_size()<=l.max_output,
{
    let ghost initial_budget=budget.remaining;
    let ghost eligible=supported_jpeg_profile(s@,page,*l) && budget.remaining as int>=jpeg_inspection_work_bound(page,*l);
    reveal(supported_jpeg_profile);reveal(per_segment_bound);
    let ghost dcost=(page as int+1)*(l.max_tags as int+1);
    let ghost fixed=4*(l.max_input as int)+128;let ghost unit=per_segment_bound(*l);
    assert((page as int+1)*(l.max_tags as int+1)>=0) by(nonlinear_arith) requires page>=0,l.max_tags>=0;
    assert(0<=dcost && 0<=fixed && 0<=unit);
    assert(l.max_tags as int+1<=(page as int+1)*(l.max_tags as int+1)) by(nonlinear_arith) requires page>=0,l.max_tags>=0;
    if s.len()>l.max_input || page>=l.max_directories {return Err(Error::Limit);}
    let h=crate::header(s)?;
    proof {if eligible {profile_directory(s@,page,*l,0);reveal_with_fuel(nth_ifd,1);}}
    if h.first_ifd==0 {return Err(Error::PageNotFound);}
    assert(eligible ==> jm::directory_ok(s@,h.first_ifd as int,h.little_endian,l.max_tags as int));
    let mut m=jm::directory(s,h.first_ifd,h.little_endian,l.max_tags,budget)?;let mut p=0usize;
    while p<page
        invariant p<=page,budget.remaining<=old(budget).remaining,initial_budget==old(budget).remaining,
            old(budget).remaining as int-budget.remaining<=(p as int+1)*(l.max_tags as int+1),
            eligible== (supported_jpeg_profile(s@,page,*l) && old(budget).remaining as int>=jpeg_inspection_work_bound(page,*l)),
            header_valid(s@),h.little_endian==(s@[0]==73),h.first_ifd==word32(s@,4,h.little_endian),
            m.le==h.little_endian,m.matches(s@),jm::valid(s@,m.tags@),
            m.off==nth_ifd(s@,h.first_ifd as int,h.little_endian,p as nat),
        decreases page-p,
    {
        proof {if eligible {profile_directory(s@,page,*l,(p+1) as nat);}}
        assert(eligible ==> nth_ifd(s@,h.first_ifd as int,h.little_endian,(p+1) as nat)!=0 &&
            jm::directory_ok(s@,nth_ifd(s@,h.first_ifd as int,h.little_endian,(p+1) as nat),h.little_endian,l.max_tags as int));
        assert(nth_ifd(s@,h.first_ifd as int,h.little_endian,(p+1) as nat)==m.next) by {reveal_with_fuel(nth_ifd,1);}
        assert(eligible ==> jm::directory_ok(s@,m.next as int,h.little_endian,l.max_tags as int));
        assert(eligible ==> budget.remaining>=l.max_tags as int+1) by(nonlinear_arith)
            requires eligible ==> initial_budget as int>=(page as int+1)*(l.max_tags as int+1),
                initial_budget as int-budget.remaining<=(p as int+1)*(l.max_tags as int+1),p<page;
        let off=m.next;if off==0 {return Err(Error::PageNotFound);}
        let ghost prior_budget=budget.remaining;
        m=jm::directory(s,off,h.little_endian,l.max_tags,budget)?;
        assert(initial_budget as int-budget.remaining<=(p as int+2)*(l.max_tags as int+1)) by(nonlinear_arith)
            requires initial_budget as int-prior_budget<=(p as int+1)*(l.max_tags as int+1),prior_budget as int-budget.remaining<=l.max_tags as int+1;
        p=p+1;reveal_with_fuel(nth_ifd,1);
    }
    proof {metadata_unique(s@,m);}
    assert(eligible ==> budget.remaining as int>=fixed+(l.max_segments as int)*unit) by(nonlinear_arith)
        requires eligible ==> initial_budget as int>=dcost+fixed+(l.max_segments as int)*unit,
            initial_budget as int-budget.remaining<=dcost;
    spend(budget,add(mul(s.len(),4)?,128)?)?;
    let f=jf::read(s,m,l)?;let base=jc::base(s,f)?;reveal(jf::fields);
    assert(eligible ==> budget.remaining as int>=(f.segments as int)*unit) by(nonlinear_arith)
        requires eligible ==> budget.remaining as int>=(l.max_segments as int)*unit,
            f.segments<=l.max_segments,unit>=0;
    assert(eligible ==> forall|q:int| 0<=q<f.segments ==> segment_supported(s@,f,base@,q,*l));
    assert(eligible ==> work_prefix(s@,f,base@,f.segments as nat)<=usize::MAX);
    let mut k=0usize;let mut work=0usize;let mut scratch=0usize;
    reveal_with_fuel(work_prefix,1);reveal_with_fuel(scratch_prefix,1);
    while k<f.segments
        invariant s.len()<=l.max_input,k<=f.segments,
            eligible== (supported_jpeg_profile(s@,page,*l) && old(budget).remaining as int>=jpeg_inspection_work_bound(page,*l)),
            eligible ==> budget.remaining as int>=(f.segments as int-k)*unit,
            unit==per_segment_bound(*l),unit>=0,initial_budget==old(budget).remaining,
            eligible ==> (forall|q:int| 0<=q<f.segments ==> segment_supported(s@,f,base@,q,*l)),
            eligible ==> work_prefix(s@,f,base@,f.segments as nat)<=usize::MAX,f.valid(),seg::base_ready(s@,f,base@),budget.remaining<=old(budget).remaining,
            header_valid(s@),m.le==(s@[0]==73),m.matches(s@),m.off==nth_ifd(s@,word32(s@,4,m.le),m.le,page as nat),
            f==jf::model(s@,m),jf::fields(s@,m,*l),f.output<=l.max_output,jc::base_model(s@,f)==Some(base@),
            work as int==work_prefix(s@,f,base@,k as nat),scratch as int==scratch_prefix(s@,f,base@,k as nat),scratch<=l.max_output,
            forall|j:int| 0<=j<k ==> segment_supported(s@,f,base@,j,*l),
        decreases f.segments-k,
    {
        assert(eligible ==> segment_supported(s@,f,base@,k as int,*l));
        assert(eligible ==> budget.remaining as int>=unit) by(nonlinear_arith) requires k<f.segments,unit>=0,eligible ==> budget.remaining as int>=(f.segments as int-k)*unit;
        let ghost before=budget.remaining;
        let (cost,need)=inspect_segment(s,f,base,k,l,budget)?;
        assert(eligible ==> budget.remaining as int>=(f.segments as int-k-1)*unit) by(nonlinear_arith)
            requires eligible ==> before as int>=(f.segments as int-k)*unit,before as int-budget.remaining<=unit;
        proof {
            reveal_with_fuel(work_prefix,1);
            if eligible {prefixes(s@,f,base@,(k+1) as nat,f.segments as nat,*l);}
        }
        assert(eligible ==> work as int+cost<=usize::MAX);
        work=add(work,cost)?;if need>scratch {scratch=need;}
        reveal(segment_supported);reveal(j::eligible);
        assert(need as int==segment_scratch(s@,f,base@,k as int));
        assert(cost as int==segment_fee(s@,f,base@,k as int));
        assert(segment_supported(s@,f,base@,k as int,*l));
        k=k+1;reveal_with_fuel(work_prefix,1);reveal_with_fuel(scratch_prefix,1);
    }
    Ok(JpegImage {input:s,meta:m,format:f,base,page,limits:*l,scratch,work})
}
proof fn covers(f:jf::Format,k:int)
    requires f.valid(),0<=k<f.output,
    ensures 0<=geometry(f).segment(k)<f.segments,
        0<=geometry(f).local(k)<f.bw as int*seg::rows(f,geometry(f).segment(k))*f.ch,
{
    let g=geometry(f);let w=f.w as int;let h=f.h as int;let ch=f.ch as int;let bw=f.bw as int;let bh=f.bh as int;let a=f.across as int;
    let p=k/ch;let c=k%ch;quotient_remainder(k,ch);
    assert(0<=p<w*h) by(nonlinear_arith) requires ch>0,0<=c<ch,k==ch*p+c,0<=k<w*h*ch;
    let x=p%w;let y=p/w;quotient_remainder(p,w);
    assert(0<=y<h) by(nonlinear_arith) requires w>0,0<=x<w,p==w*y+x,0<=p<w*h;
    let bx=x/bw;let by=y/bh;let lx=x%bw;let ly=y%bh;quotient_remainder(x,bw);quotient_remainder(y,bh);
    vstd::arithmetic::div_mod::lemma_div_is_ordered(x,w-1,bw);vstd::arithmetic::div_mod::lemma_div_is_ordered(y,h-1,bh);
    let down=(h-1)/bh+1;let j=by*a+bx;
    assert(0<=j<a*down) by(nonlinear_arith) requires 0<a,0<=bx<a,0<=by<down,j==by*a+bx;
    if !f.tiled {
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(w-1,w,0,w-1);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(x,w,0,x);
        assert(bw==w && a==1 && bx==0);
        assert(j==by) by(nonlinear_arith) requires a==1,bx==0,j==by*a+bx;
    }
    let rows=seg::rows(f,j);
    assert(0<=ly<rows) by(nonlinear_arith) requires 0<=ly<bh,0<=y<h,y==bh*by+ly,
        f.tiled ==> rows==bh,!f.tiled ==> j==by,!f.tiled ==> rows==(if h-j*bh<bh {h-j*bh} else {bh});
    assert(0<=(ly*bw+lx)*ch+c<bw*rows*ch) by(nonlinear_arith) requires bw>0,ch>0,0<=ly<rows,0<=lx<bw,0<=c<ch;
}
impl<'a> JpegImage<'a> {
    pub fn decode_with_scratch(&self,output:&mut [u8],scratch:&mut [u8],budget:&mut Budget) -> (r:Result<(),Error>)
        requires self.well_formed(),
        ensures final(output).len()==old(output).len(),final(scratch).len()==old(scratch).len(),final(budget).remaining<=old(budget).remaining,
            r.is_ok()==(old(output).len()>=self.byte_len() && old(scratch).len()>=self.scratch_size() && old(budget).remaining>=self.work_len()),
            r.is_ok() ==> final(budget).remaining as int+self.work_len()==old(budget).remaining,
            r.is_ok() ==> forall|q:int| 0<=q<self.byte_len() ==> final(output)@[q]==self.decoded_byte(q),
            forall|q:int| self.byte_len()<=q<old(output).len() ==> final(output)@[q]==old(output)@[q],
            forall|q:int| self.scratch_size()<=q<old(scratch).len() ==> final(scratch)@[q]==old(scratch)@[q],
    {
        if output.len()<self.format.output || scratch.len()<self.scratch {return Err(Error::OutputTooSmall);}
        spend(budget,self.work)?;
        let f=self.format;let g=make_geometry(f);let mut k=0usize;

        proof {prefixes(self.input@,f,self.base@,f.segments as nat,f.segments as nat,self.limits);}
        while k<f.segments
            invariant self.well_formed(),f==self.format,g==geometry(f),k<=f.segments,
                budget.remaining as int+self.work==old(budget).remaining,
                output.len()==old(output).len(),output.len()>=f.output,scratch.len()==old(scratch).len(),scratch.len()>=self.scratch,
                forall|q:int| 0<=q<f.output && g.segment(q)<k ==> output@[q]==self.decoded_byte(q),
                forall|q:int| 0<=q<f.output && g.segment(q)>=k ==> output@[q]==old(output)@[q],
                forall|q:int| f.output<=q<output.len() ==> output@[q]==old(output)@[q],
                forall|q:int| self.scratch<=q<scratch.len() ==> scratch@[q]==old(scratch)@[q],
                forall|q:int| 0<=q<f.segments ==> segment_scratch(self.input@,f,self.base@,q)<=self.scratch,
            decreases f.segments-k,
        {
            assert(segment_supported(self.input@,f,self.base@,k as int,self.limits));reveal(segment_supported);
            let plan=seg::read(self.input,f,self.base,k)?;
            let image=j::materialize(self.input,plan.v,plan.bits,f.compression==6,f.photo==6,f.limited,&self.limits)?;
            let raw=image.scratch;let pixels=image.output;let need=add(raw,pixels)?;
            assert(need<=self.scratch);
            let mut local_budget=Budget {remaining:image.work};
            let ghost before_scratch=scratch@;let ghost before_output=output@;
            {
                let (blockbuf,rest)=scratch.split_at_mut(raw);let (tile,tail)=rest.split_at_mut(pixels);
                assert(tail@==before_scratch.subrange(need as int,before_scratch.len() as int));
                proof {image.expose();}
                image.decode_with_scratch(tile,blockbuf,&mut local_budget)?;

                assert forall|q:int| 0<=q<pixels implies tile@[q]==segment_pixel(self.input@,f,plan.model(),q) by {
                    assert(tile@[q]==image.decoded_byte(q));
                    assert(image.decoded_byte(q)==j::pixel(self.input@,plan.v@,plan.bits,f.compression==6,f.photo==6,f.limited,q));
                    assert(j::pixel(self.input@,plan.v@,plan.bits,f.compression==6,f.photo==6,f.limited,q)==segment_pixel(self.input@,f,plan.model(),q));
                };
                scatter_segment(f,k,tile,output)?;
                assert forall|q:int| 0<=q<f.output implies output@[q]==(if g.segment(q)==k {self.decoded_byte(q)} else {before_output[q]}) by {
                    covers(f,q);
                }
            }
            k=k+1;
        }
        assert forall|q:int| 0<=q<f.output implies output@[q]==self.decoded_byte(q) by {covers(f,q);}
        Ok(())
    }
}
fn scatter_segment(f:jf::Format,k:usize,pixels:&[u8],output:&mut [u8]) -> (r:Result<(),Error>)
    requires f.valid(),k<f.segments,pixels.len()==f.bw as int*seg::rows(f,k as int)*f.ch,old(output).len()>=f.output,
    ensures r.is_ok(),final(output).len()==old(output).len(),
        forall|q:int| 0<=q<f.output ==> final(output)@[q]==(if geometry(f).segment(q)==k {pixels@[geometry(f).local(q)]} else {old(output)@[q]}),
        forall|q:int| f.output<=q<old(output).len() ==> final(output)@[q]==old(output)@[q],
{
    let g=make_geometry(f);
    if !f.tiled {
        // A strip is one contiguous span of interleaved output bytes.
        let rows=seg::height(f,k);
        assert(f.bw==f.w);
        assert((f.w as int-1)/(f.w as int)==0) by(nonlinear_arith) requires f.w>0;
        assert(f.across==1);
        assert(f.segments==(f.h as int-1)/(f.bh as int)+1) by(nonlinear_arith)
            requires f.across==1,f.segments==f.across as int*((f.h as int-1)/(f.bh as int)+1);
        let ghost (w,h,bh,ch,kk)=(f.w as int,f.h as int,f.bh as int,f.ch as int,k as int);
        assert(kk*bh+rows<=h) by(nonlinear_arith)
            requires bh>0,kk<(h-1)/bh+1,rows==(if h-kk*bh<bh {h-kk*bh} else {bh});
        assert(kk*bh>=0) by(nonlinear_arith) requires kk>=0,bh>0;
        assert((kk*bh+rows)*w<=h*w) by(nonlinear_arith) requires kk*bh+rows<=h,w>0;
        assert((kk*bh+rows)*w*ch<=h*w*ch) by(nonlinear_arith) requires (kk*bh+rows)*w<=h*w,ch>0;
        assert(pixels.len()==w*rows*ch) by(nonlinear_arith) requires pixels.len()==f.bw as int*rows*f.ch,f.bw==w,f.ch==ch;
        assert((kk*bh+rows)*w*ch==kk*bh*w*ch+pixels.len()) by(nonlinear_arith) requires pixels.len()==w*rows*ch;
        assert(kk*bh*w<=h*w && kk*bh*w>=0) by(nonlinear_arith) requires 0<=kk*bh<=h,w>0;
        assert(kk*bh*w*ch>=0 && h*w*ch==f.output) by(nonlinear_arith) requires kk*bh*w>=0,ch>0,f.output==w*h*ch;
        assert(kk*bh*w<=f.output && kk*bh*w*ch<=f.output) by(nonlinear_arith)
            requires kk*bh*w<=h*w,kk*bh*w>=0,ch>=1,h*w*ch==f.output,(kk*bh+rows)*w*ch==kk*bh*w*ch+pixels.len(),(kk*bh+rows)*w*ch<=h*w*ch;
        let base=k*f.bh*f.w*f.ch;
        assert(base as int==kk*bh*w*ch);
        copy_byte_run(pixels,0,output,base,pixels.len(),false);
        assert forall|q:int| 0<=q<f.output implies output@[q]==(if g.segment(q)==k {pixels@[g.local(q)]} else {old(output)@[q]}) by {
            strip_span(g,kk,rows as int,q);
            if g.segment(q)==k {
                let i=q-base;assert(0<=i<pixels.len());
                assert(output@[base as int+i]==pixels@[0+i]);
            }
        }
        return Ok(());
    }
    assert(f.bw as int*f.ch<=196605) by(nonlinear_arith) requires f.bw<=65535,f.ch<=3;
    let stride=f.bw*f.ch;let mut i=0usize;
    while i<pixels.len()
        invariant i<=pixels.len(),f.valid(),k<f.segments,g==geometry(f),stride as int==g.row_bytes(),
            pixels.len()==f.bw as int*seg::rows(f,k as int)*f.ch,output.len()==old(output).len(),output.len()>=f.output,
            forall|q:int| 0<=q<f.output ==> output@[q]==(if g.segment(q)==k && g.local(q)<i {pixels@[g.local(q)]} else {old(output)@[q]}),
            forall|q:int| f.output<=q<output.len() ==> output@[q]==old(output)@[q],
        decreases pixels.len()-i,
    {
        let ghost before=output@;let ghost prior=i;
        scatter_byte(g,k,i,stride,pixels[i],output);i=i+1;
        assert forall|q:int| 0<=q<f.output implies output@[q]==(if g.segment(q)==k && g.local(q)<i {pixels@[g.local(q)]} else {old(output)@[q]}) by {
            covers(f,q);
            assert(before[q]==(if g.segment(q)==k && g.local(q)<prior {pixels@[g.local(q)]} else {old(output)@[q]}));
        }
    }
    assert forall|q:int| 0<=q<f.output implies output@[q]==(if g.segment(q)==k {pixels@[g.local(q)]} else {old(output)@[q]}) by {covers(f,q);}
    Ok(())
}
}
