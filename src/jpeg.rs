// Reusable bounded JPEG segment API; TIFF owns photometric interpretation.
use super::*;
use crate::jpeg_header as jh;
use crate::jpeg_scan as js;
use crate::jpeg_bits::Bits;
verus! {
pub(super) open spec fn start(s:Seq<u8>,off:int,end:int,v:Seq<usize>) -> Option<jh::Model> {
    if off<0 || off+2>end || end>s.len() || s[off]!=255 || s[off+1]!=216 {None}
    else {Some(jh::Model {v:v.update(31,0),at:(off+2) as usize,end:end as usize,sub:end as usize,kind:0})}
}
pub(super) fn header(s:&[u8],off:usize,end:usize,v:[usize;32],fuel:usize) -> (r:Result<jh::Header,Error>)
    requires jh::shape(v@),jh::tables_ok(s@,v@),
    ensures r.is_ok()==(start(s@,off as int,end as int,v@).is_some() && jh::finish(s@,start(s@,off as int,end as int,v@).unwrap(),fuel as nat).is_some()),
        r.is_ok() ==> r.unwrap().valid(s@) && r.unwrap().model()==jh::finish(s@,start(s@,off as int,end as int,v@).unwrap(),fuel as nat).unwrap(),
{
    if end>s.len() || !checked_range(end,off,2) || s[off]!=255 || s[off+1]!=216 {return Err(Error::Invalid);}
    let mut a=v;a[31]=0;
    jh::parse(s,jh::Header {v:a,at:off+2,end,sub:end,kind:0},fuel)
}
pub(super) fn empty() -> (v:[usize;32])
    ensures forall|k:int| 0<=k<32 ==> v@[k]==0,
{
    [0usize;32]
}
pub struct JpegSegment<'a> {
    pub(super) input:&'a [u8],pub(super) v:[usize;32],pub(super) bits:Bits,pub(super) count:usize,
    pub(super) legacy:bool,pub(super) ycbcr:bool,pub(super) limited:bool,
    pub(super) scratch:usize,pub(super) output:usize,pub(super) work:usize,
}
impl<'a> JpegSegment<'a> {
    pub closed spec fn dimensions(&self) -> (usize,usize,usize) {(self.v@[12],self.v@[13],self.v@[14])}
    pub closed spec fn byte_len(&self) -> nat {self.output as nat}
    pub closed spec fn scratch_size(&self) -> nat {self.scratch as nat}
    pub closed spec fn work_len(&self) -> nat {self.work as nat}

    pub closed spec fn well_formed(&self) -> bool {
        &&& jh::scan_ok(self.input@,self.v@) && self.bits.valid(self.input@)
        &&& self.count==js::total(self.v@) && self.scratch==self.count as int*64
        &&& self.output==self.v@[12] as int*self.v@[13]*self.v@[14]
        &&& self.work==self.count as int*3136+self.output
        &&& js::finish(self.input@,self.v@,js::initial(self.bits),self.count as nat,self.legacy).is_some()
    }
    pub(super) open spec fn coefficient_blocks(&self) -> Seq<Seq<i64>> {js::finish(self.input@,self.v@,js::initial(self.bits),self.count as nat,self.legacy).unwrap().blocks}
    pub(super) open spec fn sample(&self,x:int,y:int,c:int) -> u8 {
        let k=crate::jpeg_layout::index(self.v@,x,y,c);
        crate::jpeg_transform::sample(self.coefficient_blocks()[k/64],k%64)
    }
    pub closed spec fn decoded_byte(&self,k:int) -> u8 {pixel(self.input@,self.v@,self.bits,self.legacy,self.ycbcr,self.limited,k)}
    pub(super) proof fn expose(&self)
        ensures self.byte_len()==self.output,self.scratch_size()==self.scratch,self.work_len()==self.work,
            forall|k:int| self.decoded_byte(k)==pixel(self.input@,self.v@,self.bits,self.legacy,self.ycbcr,self.limited,k),
    {}
    pub fn width(&self) -> (r:usize) ensures r==self.dimensions().0, {self.v[12]}
    pub fn height(&self) -> (r:usize) ensures r==self.dimensions().1, {self.v[13]}
    pub fn channels(&self) -> (r:usize) ensures r==self.dimensions().2, {self.v[14]}
    pub fn output_len(&self) -> (r:usize) ensures r==self.byte_len(), {self.output}
    pub fn scratch_len(&self) -> (r:usize) ensures r==self.scratch_size(), {self.scratch}
    pub fn decode_work_limit(&self) -> (r:usize) ensures r==self.work_len(), {self.work}
    pub fn decode_with_scratch(&self,output:&mut [u8],scratch:&mut [u8],budget:&mut Budget) -> (r:Result<(),Error>)
        requires self.well_formed(),
        ensures final(output).len()==old(output).len(),final(scratch).len()==old(scratch).len(),
            final(budget).remaining<=old(budget).remaining,
            r.is_ok()==(old(output).len()>=self.byte_len() && old(scratch).len()>=self.scratch_size() && old(budget).remaining>=self.work_len()),
            r.is_ok() ==> final(budget).remaining as int+self.work_len()==old(budget).remaining,
            r.is_ok() ==> forall|k:int| 0<=k<self.byte_len() ==> final(output)@[k]==self.decoded_byte(k),
            forall|k:int| self.byte_len()<=k<old(output).len() ==> final(output)@[k]==old(output)@[k],
            forall|k:int| self.scratch_size()<=k<old(scratch).len() ==> final(scratch)@[k]==old(scratch)@[k],
    {
        if output.len()<self.output || scratch.len()<self.scratch {return Err(Error::OutputTooSmall);}
        spend(budget,self.work)?;
        js::decode_blocks(self.input,self.v,self.bits,self.count,self.legacy,scratch)?;
        self.render(scratch,output);
        Ok(())
    }
    fn render(&self,scratch:&[u8],output:&mut [u8])
        requires self.well_formed(),scratch.len()>=self.scratch,old(output).len()>=self.output,
            forall|j:int| 0<=j<self.scratch ==> scratch@[j]==crate::jpeg_transform::sample(self.coefficient_blocks()[j/64],j%64),
        ensures final(output).len()==old(output).len(),
            forall|j:int| 0<=j<self.output ==> final(output)@[j]==self.decoded_byte(j),
            forall|j:int| self.output<=j<old(output).len() ==> final(output)@[j]==old(output)@[j],
    {
        let w=self.v[12];let h=self.v[13];let ch=self.v[14];let hs=self.v[15];
        assert(w as int*ch<=self.output) by(nonlinear_arith)
            requires self.output as int==w as int*h*ch,h>0,w>0,ch>0;
        let row=w*ch;let mut y=0usize;
        assert(self.output as int==h as int*row) by(nonlinear_arith)
            requires self.output as int==w as int*h*ch,row as int==w as int*ch;
        while y<h
            invariant self.well_formed(),w==self.v@[12],h==self.v@[13],ch==self.v@[14],hs==self.v@[15],
                row as int==w as int*ch,y<=h,self.output as int==h as int*row,
                scratch.len()>=self.scratch,output.len()==old(output).len(),output.len()>=self.output,
                forall|j:int| 0<=j<self.scratch ==> scratch@[j]==crate::jpeg_transform::sample(self.coefficient_blocks()[j/64],j%64),
                forall|j:int| 0<=j<y as int*row ==> output@[j]==self.decoded_byte(j),
                forall|j:int| self.output<=j<output.len() ==> output@[j]==old(output)@[j],
            decreases h-y,
        {
            let mut x=0usize;
            while x<w
                invariant self.well_formed(),w==self.v@[12],h==self.v@[13],ch==self.v@[14],hs==self.v@[15],
                    row as int==w as int*ch,y<h,self.output as int==h as int*row,
                    x<=w,x<w ==> x%8==0,
                    scratch.len()>=self.scratch,output.len()==old(output).len(),output.len()>=self.output,
                    forall|j:int| 0<=j<self.scratch ==> scratch@[j]==crate::jpeg_transform::sample(self.coefficient_blocks()[j/64],j%64),
                    forall|j:int| 0<=j<y as int*row+x as int*ch ==> output@[j]==self.decoded_byte(j),
                    forall|j:int| self.output<=j<output.len() ==> output@[j]==old(output)@[j],
                decreases w-x,
            {
                let n=if w-x<8 {w-x} else {8};let count=n*ch;
                let a=crate::jpeg_layout::source(&self.v,x,y,0);
                let ae=crate::jpeg_layout::source(&self.v,x+n-1,y,0);
                let b=if ch==3 {crate::jpeg_layout::source(&self.v,x,y,1)} else {a};
                let be=if ch==3 {crate::jpeg_layout::source(&self.v,x+n-1,y,1)} else {ae};
                let d=if ch==3 {crate::jpeg_layout::source(&self.v,x,y,2)} else {a};
                let de=if ch==3 {crate::jpeg_layout::source(&self.v,x+n-1,y,2)} else {ae};
                let phase=(x%(hs*8))%hs;
                proof {
                    crate::jpeg_layout::run_index(self.v@,x as int,y as int,0,n as int-1);
                    if ch==3 {
                        crate::jpeg_layout::run_index(self.v@,x as int,y as int,1,n as int-1);
                        crate::jpeg_layout::run_index(self.v@,x as int,y as int,2,n as int-1);
                    }
                    assert(y as int*row+x as int*ch+count<=self.output) by(nonlinear_arith)
                        requires y<h,x+n<=w,row as int==w as int*ch,count as int==n as int*ch,self.output as int==h as int*row,ch>0;
                }
                // One pixel per step: all channels of a pixel share its geometry.
                let dst=y*row+x*ch;let mut col=0usize;
                while col<n
                    invariant self.well_formed(),w==self.v@[12],h==self.v@[13],ch==self.v@[14],hs==self.v@[15],
                        row as int==w as int*ch,y<h,x<w,x%8==0,0<n<=8,x+n<=w,
                        dst as int==y as int*row+x as int*ch,count as int==n as int*ch,dst as int+count<=self.output,col<=n,
                        a==crate::jpeg_layout::index(self.v@,x as int,y as int,0),a as int+n<=self.scratch,
                        ch==3 ==> b==crate::jpeg_layout::index(self.v@,x as int,y as int,1) && d==crate::jpeg_layout::index(self.v@,x as int,y as int,2)
                            && b as int+(phase as int+n-1)/(hs as int)<self.scratch && d as int+(phase as int+n-1)/(hs as int)<self.scratch,
                        phase as int==(x as int%(hs as int*8))%(hs as int),phase<hs,
                        scratch.len()>=self.scratch,output.len()==old(output).len(),output.len()>=self.output,
                        forall|j:int| 0<=j<self.scratch ==> scratch@[j]==crate::jpeg_transform::sample(self.coefficient_blocks()[j/64],j%64),
                        forall|j:int| 0<=j<dst as int+col as int*ch ==> output@[j]==self.decoded_byte(j),
                        forall|j:int| self.output<=j<output.len() ==> output@[j]==old(output)@[j],
                    decreases n-col,
                {
                    assert(col as int*ch+ch<=count) by(nonlinear_arith) requires col<n,count as int==n as int*ch;
                    let at=dst+col*ch;
                    proof {
                        let pp=y as int*w+x+col;
                        assert(at as int==pp*ch) by(nonlinear_arith)
                            requires at as int==dst as int+col as int*ch,dst as int==y as int*row+x as int*ch,row as int==w as int*ch,pp==y as int*w+x+col;
                        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(at as int,ch as int,pp,0);
                        if ch==3 {
                            vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(at as int+1,3,pp,1);
                            vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(at as int+2,3,pp,2);
                        }
                        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(pp,w as int,y as int,x as int+col);
                        crate::jpeg_layout::run_index(self.v@,x as int,y as int,0,col as int);
                        if ch==3 {
                            crate::jpeg_layout::run_index(self.v@,x as int,y as int,1,col as int);
                            crate::jpeg_layout::run_index(self.v@,x as int,y as int,2,col as int);
                            vstd::arithmetic::div_mod::lemma_div_is_ordered(phase as int+col,phase as int+n-1,hs as int);
                        }
                    }
                    if ch==3 {
                        let off=if hs==1 {phase+col} else if hs==2 {(phase+col)/2} else if hs==4 {(phase+col)/4} else {(phase+col)/hs};
                        let (y0,c1,c2)=(scratch[a+col],scratch[b+off],scratch[d+off]);
                        let (v0,v1,v2)=if self.ycbcr {
                            (crate::jpeg_color::convert(y0,c1,c2,0,self.limited),crate::jpeg_color::convert(y0,c1,c2,1,self.limited),crate::jpeg_color::convert(y0,c1,c2,2,self.limited))
                        } else {(y0,c1,c2)};
                        assert(v0==self.decoded_byte(at as int) && v1==self.decoded_byte(at as int+1) && v2==self.decoded_byte(at as int+2));
                        output[at]=v0;output[at+1]=v1;output[at+2]=v2;
                    } else {
                        let v0=scratch[a+col];
                        assert(v0==self.decoded_byte(at as int));
                        output[at]=v0;
                    }
                    col=col+1;
                }
                x=x+n;
                assert(x<w ==> x%8==0);
            }
            assert(y as int*row+w as int*ch==(y as int+1)*row) by(nonlinear_arith)
                requires row as int==w as int*ch;
            y=y+1;
        }
    }

}
pub(super) open spec fn sample(s:Seq<u8>,v:Seq<usize>,bits:Bits,legacy:bool,x:int,y:int,c:int) -> u8 {
    let blocks=js::finish(s,v,js::initial(bits),js::total(v) as nat,legacy).unwrap().blocks;
    let k=crate::jpeg_layout::index(v,x,y,c);crate::jpeg_transform::sample(blocks[k/64],k%64)
}
pub(super) open spec fn pixel(s:Seq<u8>,v:Seq<usize>,bits:Bits,legacy:bool,ycbcr:bool,limited:bool,k:int) -> u8 {
    let ch=v[14] as int;let w=v[12] as int;let c=k%ch;let x=(k/ch)%w;let y=(k/ch)/w;
    if ycbcr && v[14]==3 {crate::jpeg_color::color(sample(s,v,bits,legacy,x,y,0),sample(s,v,bits,legacy,x,y,1),sample(s,v,bits,legacy,x,y,2),c,limited)} else {sample(s,v,bits,legacy,x,y,c)}
}
pub(super) open spec fn pixel_count(v:Seq<usize>) -> int {v[12] as int*v[13]*v[14]}
pub(super) open spec fn work(v:Seq<usize>) -> int {js::total(v)*3136+pixel_count(v)}
#[verifier::opaque]
pub(super) open spec fn eligible(s:Seq<u8>,v:Seq<usize>,bits:Bits,legacy:bool,l:Limits) -> bool {
    v[12]<=l.max_dimension && v[13]<=l.max_dimension && js::total(v)*64<=l.max_output &&
    pixel_count(v)<=l.max_output && work(v)<=usize::MAX &&
    js::finish(s,v,js::initial(bits),js::total(v) as nat,legacy).is_some()
}
pub(super) fn prepare<'a>(s:&'a [u8],v:[usize;32],bits:Bits,legacy:bool,ycbcr:bool,limited:bool,limits:&Limits,budget:&mut Budget) -> (r:Result<JpegSegment<'a>,Error>)
    requires jh::scan_ok(s@,v@),bits.valid(s@),
    ensures final(budget).remaining<=old(budget).remaining,
        r.is_ok()==(eligible(s@,v@,bits,legacy,*limits) && old(budget).remaining as int>=work(v@)),
        r.is_ok() ==> final(budget).remaining as int+work(v@)==old(budget).remaining,
        old(budget).remaining as int-final(budget).remaining<=work(v@),
        old(budget).remaining as int-final(budget).remaining<=3137*(limits.max_output as int),
        r.is_ok() ==> r.unwrap().well_formed() && r.unwrap().input@==s@ && r.unwrap().v==v && r.unwrap().bits==bits && r.unwrap().legacy==legacy && r.unwrap().ycbcr==ycbcr && r.unwrap().limited==limited,
        r.is_ok() ==> r.unwrap().output<=limits.max_output && r.unwrap().scratch<=limits.max_output,
        r.is_ok() ==> r.unwrap().output==pixel_count(v@) && r.unwrap().scratch==64*js::total(v@) && r.unwrap().work==work(v@),
{
    reveal(eligible);
    assert(js::total(v@)>=0);
    assert(js::total(v@)<=js::total(v@)*64) by(nonlinear_arith) requires js::total(v@)>=0;
    assert(v@[12] as int*v@[13]<=pixel_count(v@)) by(nonlinear_arith) requires v@[14]>=1;
    if v[12]>limits.max_dimension || v[13]>limits.max_dimension {return Err(Error::Limit);}
    let count=js::block_count(v)?;let scratch=mul(count,64)?;
    let output=mul(mul(v[12],v[13])?,v[14])?;
    if scratch>limits.max_output || output>limits.max_output {return Err(Error::Limit);}
    let work=add(mul(count,3136)?,output)?;
    assert(count as int<=limits.max_output) by(nonlinear_arith) requires scratch as int==count as int*64,scratch<=limits.max_output;
    assert(work as int<=3137*(limits.max_output as int));
    spend(budget,work)?;
    js::validate(s,v,bits,count,legacy)?;
    Ok(JpegSegment {input:s,v,bits,count,legacy,ycbcr,limited,scratch,output,work})
}
pub(super) fn materialize<'a>(s:&'a [u8],v:[usize;32],bits:Bits,legacy:bool,ycbcr:bool,limited:bool,limits:&Limits) -> (r:Result<JpegSegment<'a>,Error>)
    requires jh::scan_ok(s@,v@),bits.valid(s@),eligible(s@,v@,bits,legacy,*limits),
    ensures r.is_ok(),r.unwrap().well_formed(),r.unwrap().input@==s@,r.unwrap().v==v,r.unwrap().bits==bits,r.unwrap().legacy==legacy,r.unwrap().ycbcr==ycbcr,r.unwrap().limited==limited,
        r.unwrap().output==pixel_count(v@),r.unwrap().scratch==64*js::total(v@),r.unwrap().work==work(v@),
{
    reveal(eligible);
    assert(js::total(v@)>=0);
    assert(js::total(v@)<=js::total(v@)*64) by(nonlinear_arith) requires js::total(v@)>=0;
    assert(v@[12] as int*v@[13]<=pixel_count(v@)) by(nonlinear_arith) requires v@[14]>=1;
    let count=js::block_count(v)?;let scratch=mul(count,64)?;let output=mul(mul(v[12],v[13])?,v[14])?;let work=add(mul(count,3136)?,output)?;
    Ok(JpegSegment {input:s,v,bits,count,legacy,ycbcr,limited,scratch,output,work})
}
/// Decode a standalone baseline JPEG with an explicit color-space choice.
/// TIFF callers supply shared tables and legacy framing through the container API.
pub fn inspect_jpeg<'a>(s:&'a [u8],ycbcr:bool,limits:&Limits,budget:&mut Budget) -> (r:Result<JpegSegment<'a>,Error>)
    ensures final(budget).remaining<=old(budget).remaining,r.is_ok() ==> r.unwrap().well_formed(),
{
    if s.len()>limits.max_input {return Err(Error::Limit);}
    let fuel=add(mul(s.len(),2)?,1)?;spend(budget,fuel)?;
    let v=empty();assert(jh::tables_ok(s@,v@) && jh::shape(v@));
    let h=header(s,0,s.len(),v,fuel)?;
    if h.v[31]!=1 {return Err(Error::Invalid);}
    // A terminal SOS is established only by the scan parser.
    assert(jh::scan_ok(s@,h.v@));
    let bits=Bits {at:h.at,end:h.end,byte:0,left:0};
    prepare(s,h.v,bits,false,ycbcr,false,limits,budget)
}
}
