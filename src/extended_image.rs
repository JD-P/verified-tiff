use super::*;
use crate::raster_layout::RasterLayout;
use crate::deflate_plane as dp;
#[path="extended_witness.rs"]
mod witness;
#[path="float_witness.rs"]
mod float_witness;

verus! {

spec fn layout_matches(l:RasterLayout,f:ExtFormat,le:bool) -> bool {
    &&& l.valid() && l.le==le && l.w==f.width && l.h==f.height
    &&& l.ch==(if f.photo==3 {3usize} else {f.channels})
    &&& l.bytes==(if f.depth==64 {8usize} else if f.depth==16 {2usize} else {1usize})
    &&& l.slots==(if f.depth==1 {8usize} else if f.depth==4 {2usize} else {1usize})
    &&& l.palette==(f.photo==3) && l.white==(f.photo==0) && l.alpha==f.alpha
    &&& l.separate==(f.planar==2 && f.photo!=3)
}

spec fn window(t:Tag,p:int,n:int) -> Tag {
    Tag {kind:t.kind,count:n as usize,data:(t.data as int+p*n*(if t.kind==3 {2int} else {4int})) as usize}
}

fn tag_window(t:Tag,p:usize,n:usize) -> (r:Result<Tag,Error>)
    ensures r.is_ok() ==> r.unwrap()==window(t,p as int,n as int),
        window_valid(t,p as int,n as int) ==> r.is_ok(),
{
    reveal(window_valid);
    if t.kind!=3 && t.kind!=4 {return Err(Error::Invalid);}
    let ghost scale=if t.kind==3 {2int} else {4int};
    assert(0<=p as int*n<=p as int*n*scale) by(nonlinear_arith) requires scale>0;
    let data=add(t.data,mul(mul(p,n)?,if t.kind==3 {2} else {4})?)?;
    Ok(Tag {kind:t.kind,count:n,data})
}

spec fn geom_model(s:Seq<u8>,d:Directory,le:bool,l:RasterLayout,p:int) -> Geometry {
    let tiled=lookup(s,d,324,le).is_some();
    let original_bw=if tiled {scalar_model(s,d,322,0,le)} else {l.w as int};
    let bw=(original_bw-1)/(l.slots as int)+1;
    let bh=if tiled {scalar_model(s,d,323,0,le)} else {scalar_model(s,d,278,4294967295,le)};
    let across=(l.encoded_width as int-1)/bw+1;
    let n=across*((l.h as int-1)/bh+1);
    Geometry {width:l.encoded_width,height:l.h,channels:l.plane_channels,alpha:0,
        block_width:bw as usize,block_height:bh as usize,across:across as usize,
        offsets:window(lookup(s,d,if tiled {324} else {273},le).unwrap(),p,n),
        counts:window(lookup(s,d,if tiled {325} else {279},le).unwrap(),p,n),
        le,white:false,bilevel:false,lsb:false,
        packbits:scalar_model(s,d,259,1,le)==32773,lzw:scalar_model(s,d,259,1,le)==5,tiled}
}

pub struct ExtendedImage<'a> {
    input:&'a [u8],format:ExtFormat,layout:RasterLayout,geometry:[Geometry;4],
    directory:Directory,page:usize,limits:Limits,decode_work:usize,
}

impl<'a> ExtendedImage<'a> {
    pub closed spec fn metadata_matches_source(&self) -> bool {
        let s=self.input@;let l=self.layout;let d=self.directory;
        &&& header_valid(s) && l.le==(s[0]==73)
        &&& d.start==nth_ifd(s,word32(s,4,l.le),l.le,self.page as nat)+2
        &&& d.count==word16(s,d.start as int-2,l.le)
        &&& self.format.matches(s,d,l.le,self.limits)
        &&& layout_matches(l,self.format,l.le)
        &&& (l.palette ==> lookup(s,d,320,l.le)==Some(l.map) && l.map.kind==3 && l.map.count==3*l.colors)
        &&& forall|p:int| 0<=p<l.planes ==> self.geometry@[p]==geom_model(s,d,l.le,l,p)
    }
    pub closed spec fn well_formed(&self) -> bool {
        &&& self.metadata_matches_source()
        &&& self.layout.valid() && self.layout.plane_len>0
        &&& 1<=self.layout.planes<=4
        &&& self.layout.output_len<=self.limits.max_output && self.layout.scratch_len<=self.limits.max_output
        &&& (self.layout.palette ==> range_ok(self.input.len() as int,self.layout.map.data as int,6*(self.layout.colors as int)))
        &&& forall|p:int| 0<=p<self.layout.planes ==> if self.format.compression==8 || self.format.compression==32946 {dp::ready(self.input@,self.geometry@[p],self.layout.plane_len as int)} else {plane_ready(self.input@,self.geometry@[p],self.layout.plane_len as int)}
    }
    pub closed spec fn raw_byte(&self,k:int) -> u8 {
        if self.format.compression==8 || self.format.compression==32946 {dp::raw(self.input@,self.geometry@[k/(self.layout.plane_len as int)],k%(self.layout.plane_len as int))} else {storage_pixel(self.input@,self.geometry@[k/(self.layout.plane_len as int)],k%(self.layout.plane_len as int))}
    }
    pub closed spec fn native_bytes(&self) -> Seq<u8> {
        Seq::new(self.layout.scratch_len as nat,|k:int| self.raw_byte(k))
    }
    pub closed spec fn decoded_byte(&self,k:int) -> u8 {
        if self.layout.bytes==8 {crate::float64::decoded(self.layout,self.native_bytes(),self.geometry@[0].block_width,self.format.predictor==2,k)}
        else {self.layout.converted(self.input@,self.native_bytes(),k)}
    }
    pub closed spec fn model_width(&self) -> nat {self.layout.w as nat}
    pub closed spec fn model_height(&self) -> nat {self.layout.h as nat}
    pub closed spec fn model_channels(&self) -> nat {self.layout.ch as nat}
    pub closed spec fn source_depth(&self) -> nat {self.format.depth as nat}
    pub closed spec fn output_depth(&self) -> nat {8*self.layout.bytes as nat}
    pub closed spec fn source_planar(&self) -> nat {self.format.planar as nat}
    pub closed spec fn source_compression(&self) -> nat {self.format.compression as nat}
    pub closed spec fn byte_len(&self) -> nat {self.layout.output_len as nat}
    pub closed spec fn scratch_size(&self) -> nat {self.layout.scratch_len as nat}
    pub closed spec fn work_len(&self) -> nat {self.decode_work as nat}
    pub closed spec fn selected_page(&self) -> nat {self.page as nat}
    pub closed spec fn tag_count(&self) -> nat {self.directory.count as nat}
    pub closed spec fn source_alpha(&self) -> nat {self.layout.alpha as nat}
    pub fn width(&self) -> (r:usize) ensures r==self.model_width() {self.layout.w}
    pub fn height(&self) -> (r:usize) ensures r==self.model_height() {self.layout.h}
    pub fn channels(&self) -> (r:usize) ensures r==self.model_channels() {self.layout.ch}
    pub fn bits_per_sample(&self) -> (r:usize) ensures r==self.source_depth() {self.format.depth}
    pub fn output_bits_per_sample(&self) -> (r:usize) ensures self.well_formed() ==> r==self.output_depth() {if self.layout.bytes==8 {64} else if self.layout.bytes==2 {16} else {8}}
    pub closed spec fn floating_point(&self) -> bool {self.format.depth==64}
    pub closed spec fn source_predictor(&self) -> nat {self.format.predictor as nat}
    pub fn is_floating_point(&self) -> (r:bool) ensures r==self.floating_point() {self.format.depth==64}
    pub fn predictor(&self) -> (r:usize) ensures r==self.source_predictor() {self.format.predictor}
    pub fn planar_configuration(&self) -> (r:usize) ensures r==self.source_planar() {self.format.planar}
    pub fn compression(&self) -> (r:usize) ensures r==self.source_compression() {self.format.compression}
    pub fn output_len(&self) -> (r:usize) ensures r==self.byte_len() {self.layout.output_len}
    pub fn scratch_len(&self) -> (r:usize) ensures r==self.scratch_size() {self.layout.scratch_len}
    pub fn decode_work_limit(&self) -> (r:usize) ensures r==self.work_len() {self.decode_work}
    pub fn page_index(&self) -> (r:usize) ensures r==self.selected_page() {self.page}
    pub fn directory_entry_count(&self) -> (r:usize) ensures r==self.tag_count() {self.directory.count}
    pub fn has_alpha(&self) -> (r:bool) ensures r==(self.source_alpha()!=0) {self.layout.alpha!=0}
    pub fn alpha_mode(&self) -> (r:AlphaMode) {
        if self.layout.alpha==1 {AlphaMode::Associated} else if self.layout.alpha==2 {AlphaMode::Unassociated} else {AlphaMode::None}
    }

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
        if output.len()<self.layout.output_len || scratch.len()<self.layout.scratch_len {return Err(Error::OutputTooSmall);}
        spend(budget,self.decode_work)?;
        let l=self.layout;
        let mut p=0usize;
        while p<l.planes
            invariant self.well_formed(),l==self.layout,p<=l.planes,
                output.len()==old(output).len(),output@==old(output)@,
                scratch.len()==old(scratch).len(),scratch.len()>=l.scratch_len,
                forall|k:int| 0<=k<p as int*l.plane_len ==> scratch@[k]==self.raw_byte(k),
                forall|k:int| l.scratch_len<=k<scratch.len() ==> scratch@[k]==old(scratch)@[k],
            decreases l.planes-p,
        {
            assert(0<=p as int*l.plane_len<=(p as int+1)*l.plane_len<=l.scratch_len) by(nonlinear_arith)
                requires p<l.planes,l.plane_len>0,l.scratch_len as int==l.plane_len as int*l.planes;
            let start=p*l.plane_len;
            assert(start as int+l.plane_len<=scratch.len()) by(nonlinear_arith) requires p<l.planes,start as int==p as int*l.plane_len,l.scratch_len as int==l.plane_len as int*l.planes,l.scratch_len<=scratch.len();
            let ghost before=scratch@;
            {
                let (prefix,rest)=scratch.split_at_mut(start);
                let (plane,tail)=rest.split_at_mut(l.plane_len);
                assert(prefix@==before.subrange(0,start as int));
                assert(tail@==before.subrange(start as int+l.plane_len,before.len() as int));
                let g=self.geometry[p];
                if self.format.compression==8 || self.format.compression==32946 {dp::decode(self.input,g,l.plane_len,plane)?;}
                else if g.packbits || g.lzw {plane_decode_compressed(self.input,g,l.plane_len,plane)?;}
                else {plane_decode_plain(self.input,g,l.plane_len,plane)?;}
                assert forall|k:int| 0<=k<l.plane_len implies plane@[k]==self.raw_byte(start as int+k) by {
                    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(start as int+k,l.plane_len as int,p as int,k);
                }
            }
            assert forall|k:int| 0<=k<(p as int+1)*l.plane_len implies scratch@[k]==self.raw_byte(k) by {
                if k<start {assert(before[k]==self.raw_byte(k));}
                else {
                    assert(0<=k-start<l.plane_len) by(nonlinear_arith) requires start as int==p as int*l.plane_len,start<=k<(p as int+1)*l.plane_len;
                    assert(scratch@[k]==self.raw_byte(start as int+(k-start)));
                }
            }
            assert forall|k:int| l.scratch_len<=k<scratch.len() implies scratch@[k]==old(scratch)@[k] by {
                assert(k>=start as int+l.plane_len) by(nonlinear_arith) requires k>=l.scratch_len,start as int==p as int*l.plane_len,p<l.planes,l.scratch_len as int==l.plane_len as int*l.planes,l.plane_len>0;
                assert(scratch@[k]==before[k]);
            }
            p=p+1;
        }
        assert(p as int*l.plane_len==l.scratch_len) by(nonlinear_arith) requires p==l.planes,l.scratch_len as int==l.plane_len as int*l.planes;
        assert(scratch@.subrange(0,l.scratch_len as int)==self.native_bytes()) by {
            assert(scratch@.subrange(0,l.scratch_len as int) =~= self.native_bytes());
        }
        self.render_native(scratch,output);
        Ok(())
    }
    fn render_native(&self,scratch:&[u8],output:&mut [u8])
        requires self.well_formed(),scratch.len()>=self.scratch_size(),old(output).len()>=self.byte_len(),
            forall|k:int| 0<=k<self.scratch_size() ==> scratch@[k]==self.raw_byte(k),
        ensures final(output).len()==old(output).len(),
            forall|k:int| 0<=k<self.byte_len() ==> final(output)@[k]==self.decoded_byte(k),
            forall|k:int| self.byte_len()<=k<old(output).len() ==> final(output)@[k]==old(output)@[k],
    {
        let l=self.layout;
        assert(scratch@.subrange(0,l.scratch_len as int) =~= self.native_bytes());
        if l.bytes==8 {
            let g=self.geometry[0];
            if self.format.compression==8 || self.format.compression==32946 {assert(dp::ready(self.input@,g,l.plane_len as int));}
            else {assert(plane_ready(self.input@,g,l.plane_len as int));}
            crate::float64::convert(l,scratch,g.block_width,self.format.predictor==2,output);
            return;
        }
        if l.bytes==1 && l.slots==1 && !l.palette && !l.separate && !l.white && l.alpha!=1 {
            proof {l.identity(self.input@,self.native_bytes());}
            let native:&[u8]=scratch;
            let (source,_)=native.split_at(l.output_len);
            let (dest,_)=output.split_at_mut(l.output_len);
            dest.copy_from_slice(source);
            assert forall|k:int| 0<=k<self.byte_len() implies output@[k]==self.decoded_byte(k) by {
                assert(k<l.output_len);
                assert(output@[k]==native@[k]);
                assert(native@[k]==scratch@[k]);
                assert(self.decoded_byte(k)==l.converted(self.input@,self.native_bytes(),k));
                assert(scratch@[k]==self.raw_byte(k));
                assert(self.native_bytes()[k]==scratch@[k]);
                assert(l.converted(self.input@,self.native_bytes(),k)==scratch@[k]);
            }
            return;
        }
        if l.bytes==2 && l.slots==1 && !l.palette && !l.separate && !l.white {
            proof {l.interleaved16(self.input@,self.native_bytes());}
            let mut k=0usize;
            while k<l.output_len
                invariant self.well_formed(),l==self.layout,l.bytes==2,l.slots==1,!l.palette,!l.separate,!l.white,
                    k<=l.output_len,k%2==0,l.output_len%2==0,l.scratch_len==l.output_len,
                    scratch.len()>=l.scratch_len,output.len()==old(output).len(),l.output_len<=output.len(),
                    forall|i:int| 0<=i<l.scratch_len ==> scratch@[i]==self.raw_byte(i),
                    forall|i:int| 0<=i<l.output_len ==> self.decoded_byte(i)==self.native_bytes()[if l.le {i+1-2*(i%2)} else {i}],
                    forall|i:int| 0<=i<k ==> output@[i]==self.decoded_byte(i),
                    forall|i:int| l.output_len<=i<output.len() ==> output@[i]==old(output)@[i],
                decreases l.output_len-k,
            {
                assert(k+1<l.output_len);
                output[k]=scratch[if l.le {k+1} else {k}];
                output[k+1]=scratch[if l.le {k} else {k+1}];
                k=k+2;
            }
            return;
        }
        let mut k=0usize;
        while k<l.output_len
            invariant self.well_formed(),l==self.layout,l.bytes!=8,k<=l.output_len<=output.len(),
                output.len()==old(output).len(),
                forall|i:int| 0<=i<k ==> output@[i]==self.decoded_byte(i),
                forall|i:int| l.output_len<=i<output.len() ==> output@[i]==old(output)@[i],
                scratch.len()>=l.scratch_len,
                forall|i:int| 0<=i<l.scratch_len ==> scratch@[i]==self.raw_byte(i),
            decreases l.output_len-k,
        {
            output[k]=l.convert(self.input,scratch,k);
            assert(scratch@.subrange(0,l.scratch_len as int)==self.native_bytes()) by {
                assert forall|i:int| 0<=i<l.scratch_len implies scratch@.subrange(0,l.scratch_len as int)[i]==self.native_bytes()[i] by {};
            }
            k=k+1;
        }
    }

}


spec fn layout_model(s:Seq<u8>,d:Directory,le:bool) -> RasterLayout {
    let w=scalar_model(s,d,256,0,le);let h=scalar_model(s,d,257,0,le);
    let depth=bits_model(s,d,le);let photo=scalar_model(s,d,262,999,le);
    let palette=photo==3;let ch=if palette {3int} else {scalar_model(s,d,277,1,le)};
    let bytes=if depth==64 {8int} else if depth==16 {2int} else {1int};let slots=if depth==1 {8int} else if depth==4 {2int} else {1int};
    let separate=scalar_model(s,d,284,1,le)==2 && !palette;
    let ew=(w-1)/slots+1;let pc=if palette {1int} else if separate {bytes} else {ch*bytes};
    let planes=if separate {ch} else {1int};let plen=ew*h*pc;
    RasterLayout {w:w as usize,h:h as usize,ch:ch as usize,bytes:bytes as usize,slots:slots as usize,
        palette,separate,le,white:photo==0,alpha:scalar_model(s,d,338,0,le) as usize,
        encoded_width:ew as usize,plane_channels:pc as usize,planes:planes as usize,
        plane_len:plen as usize,scratch_len:(plen*planes) as usize,output_len:(w*h*ch*bytes) as usize,
        colors:if slots==8 {2usize} else if slots==2 {16usize} else {256usize},
        map:if palette {lookup(s,d,320,le).unwrap()} else {Tag {kind:3,count:0,data:0}}}
}
#[verifier::opaque]
spec fn raster_layout_profile(s:Seq<u8>,d:Directory,le:bool,limits:Limits) -> bool {
    let l=layout_model(s,d,le);
    &&& l.valid() && l.plane_len>0
    &&& l.output_len<=limits.max_output && l.scratch_len<=limits.max_output
    &&& (l.palette ==> lookup(s,d,320,le).is_some() && l.map.kind==3 && l.map.count==3*l.colors
        && range_ok(s.len() as int,l.map.data as int,6*(l.colors as int)))
}
#[verifier::opaque]
spec fn window_valid(t:Tag,p:int,n:int) -> bool {
    (t.kind==3 || t.kind==4) && 0<=p && 0<=n<=usize::MAX
        && 0<=t.data as int+p*n*(if t.kind==3 {2int} else {4int})<=usize::MAX
}
#[verifier::opaque]
spec fn geometry_windows(s:Seq<u8>,d:Directory,le:bool,l:RasterLayout,p:int) -> bool {
    let tiled=lookup(s,d,324,le).is_some();let n=segment_count(geom_model(s,d,le,l,0));
    window_valid(lookup(s,d,if tiled {324} else {273},le).unwrap(),p,n)
        && window_valid(lookup(s,d,if tiled {325} else {279},le).unwrap(),p,n)
}
#[verifier::opaque]
spec fn geometry_profile(s:Seq<u8>,d:Directory,le:bool,l:RasterLayout,limits:Limits) -> bool {
    let tiled=lookup(s,d,324,le).is_some();let g=geom_model(s,d,le,l,0);
    let offsets=lookup(s,d,if tiled {324} else {273},le);
    let counts=lookup(s,d,if tiled {325} else {279},le);
    &&& (if tiled {
        &&& lookup(s,d,273,le).is_none() && lookup(s,d,279,le).is_none()
        &&& scalar_valid(s,d,322,le) && scalar_valid(s,d,323,le)
        &&& 0<scalar_model(s,d,322,0,le)<=usize::MAX && scalar_model(s,d,322,0,le)%16==0
        &&& 0<scalar_model(s,d,323,0,le)<=usize::MAX && scalar_model(s,d,323,0,le)%16==0
    } else {
        &&& lookup(s,d,322,le).is_none() && lookup(s,d,323,le).is_none() && lookup(s,d,325,le).is_none()
        &&& scalar_valid(s,d,278,le) && 0<scalar_model(s,d,278,4294967295,le)<=usize::MAX
    })
    &&& g.positive() && g.across as int==(g.width as int-1)/(g.block_width as int)+1
    &&& 0<segment_count(g)*(l.planes as int)<=limits.max_segments
    &&& offsets.is_some() && counts.is_some()
    &&& offsets.unwrap().count==segment_count(g)*(l.planes as int) && counts.unwrap().count==segment_count(g)*(l.planes as int)
    &&& forall|p:int| 0<=p<l.planes ==> geometry_windows(s,d,le,l,p)
}
spec fn is_deflate(s:Seq<u8>,d:Directory,le:bool) -> bool {
    scalar_model(s,d,259,1,le)==8 || scalar_model(s,d,259,1,le)==32946
}
#[verifier::opaque]
spec fn plane_work(s:Seq<u8>,d:Directory,le:bool,l:RasterLayout,p:int) -> int {
    let g=geom_model(s,d,le,l,p);
    if is_deflate(s,d,le) {dp::work_prefix(s,g,segment_count(g) as nat)} else {codec_work_prefix(s,g,segment_count(g) as nat)}
}
#[verifier::opaque]
spec fn planes_work(s:Seq<u8>,d:Directory,le:bool,l:RasterLayout,n:nat) -> int
    decreases n,
{if n==0 {0} else {planes_work(s,d,le,l,(n-1) as nat)+plane_work(s,d,le,l,n as int-1)}}
proof fn planes_work_step(s:Seq<u8>,d:Directory,le:bool,l:RasterLayout,n:nat)
    ensures planes_work(s,d,le,l,n+1)==planes_work(s,d,le,l,n)+plane_work(s,d,le,l,n as int),
{reveal_with_fuel(planes_work,1);}
proof fn planes_work_monotone(s:Seq<u8>,d:Directory,le:bool,l:RasterLayout,n:nat,m:nat)
    requires n<=m,forall|p:int| 0<=p<m ==> 0<=#[trigger] plane_work(s,d,le,l,p),
    ensures 0<=planes_work(s,d,le,l,n)<=planes_work(s,d,le,l,m),
    decreases m,
{
    reveal_with_fuel(planes_work,1);
    if m>0 {
        if n<m {planes_work_monotone(s,d,le,l,n,(m-1) as nat);}
        else {planes_work_monotone(s,d,le,l,(m-1) as nat,(m-1) as nat);}
    }
}
#[verifier::opaque]
spec fn storage_plane_profile(s:Seq<u8>,d:Directory,le:bool,l:RasterLayout,p:int,limits:Limits) -> bool {
    let g=geom_model(s,d,le,l,p);
    if is_deflate(s,d,le) {dp::profile(s,g,limits.max_output)} else {plane_profile(s,g)}
}
#[verifier::opaque]
spec fn storage_profile(s:Seq<u8>,d:Directory,le:bool,l:RasterLayout,limits:Limits) -> bool {
    &&& forall|p:int| 0<=p<l.planes ==> storage_plane_profile(s,d,le,l,p,limits)
    &&& 0<=l.scratch_len as int+l.output_len+planes_work(s,d,le,l,l.planes as nat)<=usize::MAX
}
#[verifier::opaque]
pub closed spec fn supported_extended_profile(s:Seq<u8>,page:usize,limits:Limits) -> bool {
    let le=s[0]==73;let first=word32(s,4,le);
    let d=directory_model(s,nth_ifd(s,first,le,page as nat),le);let l=layout_model(s,d,le);
    &&& header_valid(s) && s.len()<=limits.max_input && page<limits.max_directories
    &&& forall|p:nat| p<=page ==> {let at=#[trigger] nth_ifd(s,first,le,p);0<at<=usize::MAX && directory_valid(s,at,le,limits)}
    &&& ext_fields(s,d,le,limits)
    &&& raster_layout_profile(s,d,le,limits)
    &&& geometry_profile(s,d,le,l,limits)
    &&& storage_profile(s,d,le,l,limits)
}
pub closed spec fn supported_deflate_profile(s:Seq<u8>,page:usize,limits:Limits) -> bool {
    let le=s[0]==73;
    let d=directory_model(s,nth_ifd(s,word32(s,4,le),le,page as nat),le);
    supported_extended_profile(s,page,limits) && is_deflate(s,d,le)
}
pub open spec fn deflate_decode_work_bound(page:usize,limits:Limits) -> int {
    2*extended_inspection_work_bound(page,limits)+2*(limits.max_output as int)
}
pub fn deflate_decode_work_limit(page:usize,limits:&Limits) -> (r:Result<usize,Error>)
    ensures r.is_ok()==(deflate_decode_work_bound(page,*limits)<=usize::MAX),
        r.is_ok() ==> r.unwrap()==deflate_decode_work_bound(page,*limits),
{
    let inspect=extended_inspection_work_limit(page,limits)?;
    add(mul(inspect,2)?,mul(limits.max_output,2)?)
}

pub open spec fn extended_inspection_work_bound(page:usize,limits:Limits) -> int {
    (page as int+1)*(limits.max_tags as int+1)+20*limits.max_tags+4
        +4*((limits.max_segments as int)*(8*(limits.max_input as int)+2*(limits.max_output as int)+65))
}
pub fn extended_inspection_work_limit(page:usize,limits:&Limits) -> (r:Result<usize,Error>)
    ensures r.is_ok()==(extended_inspection_work_bound(page,*limits)<=usize::MAX),
        r.is_ok() ==> r.unwrap()==extended_inspection_work_bound(page,*limits),
{
    let ghost p=page as int;let ghost t=limits.max_tags as int;
    assert(p+1<=(p+1)*(t+1) && t+1<=(p+1)*(t+1)) by(nonlinear_arith) requires p>=0,t>=0;
    let dirs=mul(add(page,1)?,add(limits.max_tags,1)?)?;
    let tags=add(mul(20,limits.max_tags)?,4)?;
    let ghost ms=limits.max_segments as int;let ghost mi=limits.max_input as int;let ghost mo=limits.max_output as int;
    assert(4*(ms*(8*mi+2*mo+65))==32*(ms*mi)+8*(ms*mo)+260*ms) by(nonlinear_arith);
    assert(0<=ms*mi<=32*(ms*mi) && 0<=ms*mo<=8*(ms*mo)) by(nonlinear_arith) requires ms>=0,mi>=0,mo>=0;
    let encoded=mul(mul(limits.max_segments,limits.max_input)?,32)?;
    let expanded=mul(mul(limits.max_segments,limits.max_output)?,8)?;
    let fixed=mul(limits.max_segments,260)?;
    add(add(dirs,tags)?,add(add(encoded,expanded)?,fixed)?)
}

fn read_raster_layout(s:&[u8],d:Directory,le:bool,f:ExtFormat,limits:&Limits,budget:&mut Budget) -> (r:Result<RasterLayout,Error>)
    requires d.readable(s@,le),f.matches(s@,d,le,*limits),
    ensures final(budget).remaining<=old(budget).remaining,
        old(budget).remaining as int-final(budget).remaining<=d.count,
        r.is_ok() ==> layout_matches(r.unwrap(),f,le) && r.unwrap()==layout_model(s@,d,le),
        r.is_ok() ==> r.unwrap().plane_len>0 && r.unwrap().output_len<=limits.max_output && r.unwrap().scratch_len<=limits.max_output,
        r.is_ok() ==> r.unwrap().palette ==> lookup(s@,d,320,le)==Some(r.unwrap().map)
            && r.unwrap().map.kind==3 && r.unwrap().map.count==3*r.unwrap().colors
            && range_ok(s.len() as int,r.unwrap().map.data as int,6*(r.unwrap().colors as int)),
        raster_layout_profile(s@,d,le,*limits) && old(budget).remaining>=d.count ==> r.is_ok(),
{
    let ghost eligible=raster_layout_profile(s@,d,le,*limits) && budget.remaining>=d.count;
    reveal(ext_fields);reveal(raster_layout_profile);
    let palette=f.photo==3;
    let ch=if palette {3} else {f.channels};
    let bytes=if f.depth==64 {8} else if f.depth==16 {2} else {1};
    let slots=if f.depth==1 {8} else if f.depth==4 {2} else {1};
    let separate=f.planar==2 && !palette;
    let encoded_width=(f.width-1)/slots+1;
    let plane_channels=if palette {1} else if separate {bytes} else {mul(ch,bytes)?};
    let planes=if separate {ch} else {1};
    let plane_len=mul(mul(encoded_width,f.height)?,plane_channels)?;
    let scratch_len=mul(plane_len,planes)?;
    let output_len=mul(mul(mul(f.width,f.height)?,ch)?,bytes)?;
    if output_len>limits.max_output || scratch_len>limits.max_output {return Err(Error::Limit);}
    let colors=if slots==8 {2} else if slots==2 {16} else {256};
    let map=if palette {
        let t=required_tag(s,d,320,le,budget)?;
        if t.kind!=3 || t.count!=3*colors {return Err(Error::Invalid);}
        if !checked_range(s.len(),t.data,6*colors) {return Err(Error::Truncated);}
        t
    } else {Tag {kind:3,count:0,data:0}};
    let l=RasterLayout {w:f.width,h:f.height,ch,bytes,slots,palette,separate,le,white:f.photo==0,alpha:f.alpha,
        encoded_width,plane_channels,planes,plane_len,scratch_len,output_len,colors,map};
    assert(l.valid());
    assert(plane_len>0) by(nonlinear_arith) requires encoded_width>0,f.height>0,plane_channels>0,plane_len as int==encoded_width as int*f.height*plane_channels;
    Ok(l)
}

fn read_extended_geometry(s:&[u8],d:Directory,le:bool,f:ExtFormat,l:RasterLayout,limits:&Limits,budget:&mut Budget) -> (r:Result<Geometry,Error>)
    requires d.readable(s@,le),f.matches(s@,d,le,*limits),layout_matches(l,f,le),
    ensures final(budget).remaining<=old(budget).remaining,
        old(budget).remaining as int-final(budget).remaining<=7*(d.count as int),
        geometry_profile(s@,d,le,l,*limits) && old(budget).remaining as int>=7*(d.count as int) ==> r.is_ok(),
        r.is_ok() && geometry_profile(s@,d,le,l,*limits) ==> forall|p:int| 0<=p<l.planes ==>
            window_valid(r.unwrap().offsets,p,segment_count(r.unwrap())) && window_valid(r.unwrap().counts,p,segment_count(r.unwrap())),
        r.is_ok() ==> r.unwrap().tiled==lookup(s@,d,324,le).is_some(),
        r.is_ok() ==> r.unwrap().positive() && !r.unwrap().bilevel && !r.unwrap().lsb && !r.unwrap().white && r.unwrap().alpha==0,
        r.is_ok() ==> r.unwrap().width==l.encoded_width && r.unwrap().height==l.h && r.unwrap().channels==l.plane_channels,
        r.is_ok() ==> r.unwrap().packbits==(f.compression==32773) && r.unwrap().lzw==(f.compression==5),
        r.is_ok() ==> r.unwrap().across as int==(r.unwrap().width as int-1)/(r.unwrap().block_width as int)+1,
        r.is_ok() ==> !r.unwrap().tiled ==> r.unwrap().block_width==r.unwrap().width,
        r.is_ok() ==> 0<segment_count(r.unwrap())<=limits.max_segments,
        r.is_ok() ==> r.unwrap().offsets.count==segment_count(r.unwrap())*(l.planes as int) && r.unwrap().counts.count==segment_count(r.unwrap())*(l.planes as int),
        r.is_ok() ==> forall|p:int| 0<=p<l.planes ==> (Geometry {offsets:window(r.unwrap().offsets,p,segment_count(r.unwrap())),counts:window(r.unwrap().counts,p,segment_count(r.unwrap())),..r.unwrap()})==geom_model(s@,d,le,l,p),
        r.is_ok() ==> r.unwrap().offsets==lookup(s@,d,if r.unwrap().tiled {324} else {273},le).unwrap()
            && r.unwrap().counts==lookup(s@,d,if r.unwrap().tiled {325} else {279},le).unwrap(),
{
    let ghost eligible=geometry_profile(s@,d,le,l,*limits) && budget.remaining as int>=7*(d.count as int);
    reveal(ext_fields);reveal(geometry_profile);
    let slots=l.slots;let encoded_width=l.encoded_width;let planes=l.planes;let plane_channels=l.plane_channels;
    let tiled=find(s,d,324,le,budget)?.is_some();
    let (original_bw,bh,offsets,counts)=if tiled {
        if find(s,d,273,le,budget)?.is_some() || find(s,d,279,le,budget)?.is_some() {return Err(Error::Invalid);}
        let tw=scalar(s,d,322,0,le,budget)?;let th=scalar(s,d,323,0,le,budget)?;
        if tw==0 || th==0 || tw%16!=0 || th%16!=0 {return Err(Error::Invalid);}
        (tw,th,required_tag(s,d,324,le,budget)?,required_tag(s,d,325,le,budget)?)
    } else {
        if find(s,d,322,le,budget)?.is_some() || find(s,d,323,le,budget)?.is_some() || find(s,d,325,le,budget)?.is_some() {return Err(Error::Invalid);}
        let rows=scalar(s,d,278,4294967295,le,budget)?;
        if rows==0 {return Err(Error::Invalid);}
        (f.width,rows,required_tag(s,d,273,le,budget)?,required_tag(s,d,279,le,budget)?)
    };
    let bw=(original_bw-1)/slots+1;
    let across=(encoded_width-1)/bw+1;
    let down=(f.height-1)/bh+1;
    assert(eligible ==> geom_model(s@,d,le,l,0).across==across && geom_model(s@,d,le,l,0).block_height==bh);
    assert(0<across as int*down<=across as int*down*planes) by(nonlinear_arith) requires across>0,down>0,planes>0;
    let n=mul(across,down)?;
    let total=mul(n,planes)?;
    if total>limits.max_segments {return Err(Error::Limit);}
    if offsets.count!=total || counts.count!=total {return Err(Error::Invalid);}
    let base=Geometry {width:encoded_width,height:f.height,channels:plane_channels,alpha:0,
        block_width:bw,block_height:bh,across,offsets,counts,le,white:false,bilevel:false,lsb:false,
        packbits:f.compression==32773,lzw:f.compression==5,tiled};
    assert(0<across as int*down<=across as int*down*planes) by(nonlinear_arith) requires across>0,down>0,planes>0;
    assert(segment_count(geom_model(s@,d,le,l,0))==n);
    assert forall|p:int| geometry_profile(s@,d,le,l,*limits) && 0<=p<l.planes implies
        window_valid(base.offsets,p,segment_count(base)) && window_valid(base.counts,p,segment_count(base)) by {
        reveal(geometry_profile);
        assert(geometry_windows(s@,d,le,l,p));
        reveal(geometry_windows);
        assert(window_valid(lookup(s@,d,if tiled {324} else {273},le).unwrap(),p,segment_count(geom_model(s@,d,le,l,0))));
        assert(window_valid(lookup(s@,d,if tiled {325} else {279},le).unwrap(),p,segment_count(geom_model(s@,d,le,l,0))));
    }
    Ok(base)
}

pub fn inspect_extended<'a>(s:&'a [u8],page:usize,limits:&Limits,budget:&mut Budget) -> (r:Result<ExtendedImage<'a>,Error>)
    ensures final(budget).remaining<=old(budget).remaining,
        supported_extended_profile(s@,page,*limits) && old(budget).remaining as int>=extended_inspection_work_bound(page,*limits) ==> r.is_ok(),
        r.is_ok() ==> r.unwrap().well_formed(),
        r.is_ok() ==> r.unwrap().metadata_matches_source(),
        r.is_ok() ==> r.unwrap().selected_page()==page,
        r.is_ok() ==> r.unwrap().byte_len()<=limits.max_output,
        r.is_ok() ==> r.unwrap().scratch_size()<=limits.max_output,
{
    let mut scratch=[0u8;0];
    inspect_extended_capture(s,page,limits,budget,&mut scratch,false)
}

fn inspect_extended_capture<'a>(s:&'a [u8],page:usize,limits:&Limits,budget:&mut Budget,scratch:&mut [u8],capture:bool) -> (r:Result<ExtendedImage<'a>,Error>)
    ensures final(scratch).len()==old(scratch).len(),
        forall|k:int| 0<=k<old(scratch).len() && (!capture || k>=limits.max_output) ==> final(scratch)@[k]==old(scratch)@[k],
        r.is_ok() && capture ==> r.unwrap().source_compression()==8 || r.unwrap().source_compression()==32946,
        r.is_ok() && capture ==> old(scratch).len()>=r.unwrap().scratch_size(),
        r.is_ok() && capture ==> forall|k:int| 0<=k<r.unwrap().scratch_size() ==> final(scratch)@[k]==r.unwrap().raw_byte(k),
        r.is_ok() && capture ==> forall|k:int| r.unwrap().scratch_size()<=k<old(scratch).len() ==> final(scratch)@[k]==old(scratch)@[k],
        final(budget).remaining<=old(budget).remaining,
        r.is_ok() ==> old(budget).remaining as int-final(budget).remaining<=extended_inspection_work_bound(page,*limits),
        r.is_ok() && capture ==> r.unwrap().work_len()<=extended_inspection_work_bound(page,*limits)+2*(limits.max_output as int),
        supported_extended_profile(s@,page,*limits) && old(budget).remaining as int>=extended_inspection_work_bound(page,*limits)
            && (!capture || (supported_deflate_profile(s@,page,*limits) && old(scratch).len()>=limits.max_output)) ==> r.is_ok(),
        r.is_ok() ==> r.unwrap().well_formed(),
        r.is_ok() ==> r.unwrap().metadata_matches_source(),
        r.is_ok() ==> r.unwrap().selected_page()==page,
        r.is_ok() ==> r.unwrap().byte_len()<=limits.max_output,
        r.is_ok() ==> r.unwrap().scratch_size()<=limits.max_output,
{
    let ghost eligible=(!capture || (supported_deflate_profile(s@,page,*limits) && old(scratch).len()>=limits.max_output)) && supported_extended_profile(s@,page,*limits) && budget.remaining as int>=extended_inspection_work_bound(page,*limits);
    let ghost unit=8*(limits.max_input as int)+2*(limits.max_output as int)+65;
    let ghost plane_bound=(limits.max_segments as int)*unit;
    let ghost tail=20*(limits.max_tags as int)+4+4*plane_bound;
    reveal(supported_extended_profile);reveal(supported_deflate_profile);
    if s.len()>limits.max_input || page>=limits.max_directories {return Err(Error::Limit);}
    let header=header(s)?;let le=header.little_endian;
    let mut off=header.first_ifd;let mut p=0usize;
    while p<page
        invariant p<=page,budget.remaining<=old(budget).remaining,
            plane_bound==(limits.max_segments as int)*unit,unit==8*(limits.max_input as int)+2*(limits.max_output as int)+65,
            eligible==((!capture || (supported_deflate_profile(s@,page,*limits) && old(scratch).len()>=limits.max_output)) && supported_extended_profile(s@,page,*limits) && old(budget).remaining as int>=extended_inspection_work_bound(page,*limits)),
            tail==20*(limits.max_tags as int)+4+4*((limits.max_segments as int)*(8*(limits.max_input as int)+2*(limits.max_output as int)+65)),
            old(budget).remaining as int-budget.remaining<=p as int*(limits.max_tags as int+1),
            header_valid(s@),le==(s@[0]==73),header.first_ifd==word32(s@,4,le),
            off==nth_ifd(s@,header.first_ifd as int,le,p as nat),
        decreases page-p,
    {
        reveal(supported_extended_profile);reveal(supported_deflate_profile);
        assert(eligible ==> directory_valid(s@,off as int,le,*limits));
        proof {if eligible {traversal_budget(old(budget).remaining as int,budget.remaining as int,p as int,page as int,limits.max_tags as int,tail);}}
        if off==0 {return Err(Error::PageNotFound);}
        off=directory(s,off,le,limits,budget)?.next;
        assert((p as int+1)*(limits.max_tags as int+1)==p as int*(limits.max_tags as int+1)+limits.max_tags+1) by(nonlinear_arith);
        p=p+1;
    }
    if off==0 {return Err(Error::PageNotFound);}
    proof {if eligible {traversal_budget(old(budget).remaining as int,budget.remaining as int,p as int,page as int,limits.max_tags as int,tail);}}
    assert((page as int+1)*(limits.max_tags as int+1)==page as int*(limits.max_tags as int+1)+limits.max_tags+1) by(nonlinear_arith);
    let d=directory(s,off,le,limits,budget)?;
    assert(old(budget).remaining as int-budget.remaining<=(page as int+1)*(limits.max_tags as int+1));
    assert(eligible ==> budget.remaining as int>=tail);
    let f=read_ext_format(s,d,le,limits,budget)?;
    if capture && f.compression!=8 && f.compression!=32946 {return Err(Error::Unsupported);}
    reveal(ext_fields);
    let l=read_raster_layout(s,d,le,f,limits,budget)?;
    let base=read_extended_geometry(s,d,le,f,l,limits,budget)?;
    if capture && scratch.len()<l.scratch_len {return Err(Error::OutputTooSmall);}
    let encoded_width=l.encoded_width;let plane_channels=l.plane_channels;let planes=l.planes;
    let plane_len=l.plane_len;let scratch_len=l.scratch_len;let output_len=l.output_len;
    let offsets=base.offsets;let counts=base.counts;
    let n=mul(base.across,(base.height-1)/base.block_height+1)?;
    assert(eligible ==> l==layout_model(s@,d,le));
    assert(eligible ==> budget.remaining as int>=4*plane_bound);
    let ghost before_planes=budget.remaining;
    assert(old(budget).remaining as int-before_planes<=extended_inspection_work_bound(page,*limits)-4*plane_bound);
    reveal(storage_profile);reveal(plane_work);
    proof {if eligible {
        assert forall|q:int| 0<=q<planes implies 0<=#[trigger] plane_work(s@,d,le,l,q) by {
            assert(storage_plane_profile(s@,d,le,l,q,*limits));
            reveal(storage_plane_profile);
            let g=geom_model(s@,d,le,l,q);
            if is_deflate(s@,d,le) {reveal(dp::profile);} else {reveal(plane_profile);}
        }
        planes_work_monotone(s@,d,le,l,0,planes as nat);
    }}
    reveal_with_fuel(planes_work,1);
    let mut geometry=[base;4];
    let mut decode_work=add(scratch_len,output_len)?;
    let mut plane=0usize;
    while plane<planes
        invariant eligible==((!capture || (supported_deflate_profile(s@,page,*limits) && old(scratch).len()>=limits.max_output)) && supported_extended_profile(s@,page,*limits) && old(budget).remaining as int>=extended_inspection_work_bound(page,*limits)),
            eligible ==> geometry_profile(s@,d,le,l,*limits) && storage_profile(s@,d,le,l,*limits),
            eligible ==> forall|q:int| 0<=q<planes ==> 0<=#[trigger] plane_work(s@,d,le,l,q),
            eligible ==> before_planes as int>=4*plane_bound,
            eligible ==> forall|q:int| 0<=q<planes ==> window_valid(offsets,q,n as int) && window_valid(counts,q,n as int),
            before_planes as int-budget.remaining<=plane as int*plane_bound,
            old(budget).remaining as int-before_planes<=extended_inspection_work_bound(page,*limits)-4*plane_bound,
            capture ==> decode_work as int<=scratch_len as int+output_len+plane as int*plane_bound,
            plane_bound==(limits.max_segments as int)*unit,unit==8*(limits.max_input as int)+2*(limits.max_output as int)+65,
            0<n<=limits.max_segments,s.len()<=limits.max_input,
            base.tiled==lookup(s@,d,324,le).is_some(),
            decode_work as int==scratch_len as int+output_len+planes_work(s@,d,le,l,plane as nat),
            f.matches(s@,d,le,*limits),
            base.offsets==lookup(s@,d,if base.tiled {324} else {273},le).unwrap(),
            base.counts==lookup(s@,d,if base.tiled {325} else {279},le).unwrap(),
            plane<=planes,1<=planes<=4,plane_len>0,l.valid(),layout_matches(l,f,le),
            scratch_len==l.scratch_len,output_len==l.output_len,planes==l.planes,plane_len==l.plane_len,
            scratch_len<=limits.max_output,output_len<=limits.max_output,
            base.positive(),!base.bilevel,!base.lsb,!base.white,base.alpha==0,
            base.width==encoded_width,base.height==f.height,base.channels==plane_channels,
            base.packbits==(f.compression==32773),base.lzw==(f.compression==5),
            plane_len as int==base.width as int*base.height*base.channels,
            base.across as int==(base.width as int-1)/(base.block_width as int)+1,
            !base.tiled ==> base.block_width==base.width,
            n as int==segment_count(base),base.offsets==offsets,base.counts==counts,
            forall|q:int| 0<=q<plane ==> geometry@[q]==geom_model(s@,d,le,l,q),
            forall|q:int| 0<=q<plane ==> if f.compression==8 || f.compression==32946 {dp::ready(s@,geometry@[q],plane_len as int)} else {plane_ready(s@,geometry@[q],plane_len as int)},
            forall|q:int| 0<=q<planes ==> (Geometry {offsets:window(offsets,q,n as int),counts:window(counts,q,n as int),..base})==geom_model(s@,d,le,l,q),
            budget.remaining<=old(budget).remaining,
            scratch.len()==old(scratch).len(),capture ==> scratch.len()>=scratch_len && (f.compression==8 || f.compression==32946),
            forall|k:int| 0<=k<scratch.len() && (!capture || k>=scratch_len) ==> scratch@[k]==old(scratch)@[k],
            capture ==> forall|k:int| 0<=k<plane as int*plane_len ==> scratch@[k]==dp::raw(s@,geometry@[k/(plane_len as int)],k%(plane_len as int)),
        decreases planes-plane,
    {
        reveal(geometry_profile);reveal(storage_profile);reveal(plane_work);
        assert(segment_count(geom_model(s@,d,le,l,0))==n);
        assert(eligible ==> window_valid(offsets,plane as int,n as int) && window_valid(counts,plane as int,n as int));
        assert(plane_bound>0) by(nonlinear_arith) requires limits.max_segments>=n,n>0,unit>=65,plane_bound==(limits.max_segments as int)*unit;
        proof {if eligible {traversal_budget(before_planes as int,budget.remaining as int,plane as int,3,plane_bound-1,0);}}
        let ghost before=budget.remaining;
        assert(0<=n as int*(dp::quota(s.len() as int,limits.max_output as int)+1)<=plane_bound) by(nonlinear_arith)
            requires 0<=n<=limits.max_segments,s.len()<=limits.max_input,unit==8*(limits.max_input as int)+2*(limits.max_output as int)+65,plane_bound==(limits.max_segments as int)*unit;
        assert(0<=n as int*(s.len() as int+1)<=plane_bound) by(nonlinear_arith)
            requires 0<=n<=limits.max_segments,s.len()<=limits.max_input,unit==8*(limits.max_input as int)+2*(limits.max_output as int)+65,plane_bound==(limits.max_segments as int)*unit;
        let offsets=tag_window(offsets,plane,n)?;
        let counts=tag_window(counts,plane,n)?;
        let g=Geometry {offsets,counts,..base};
        assert(eligible ==> storage_plane_profile(s@,d,le,l,plane as int,*limits));
        reveal(storage_plane_profile);
        let ghost prior_scratch=scratch@;
        let work=if capture {
            assert(0<=plane as int*plane_len<=scratch_len) by(nonlinear_arith)
                requires plane<planes,plane_len>0,scratch_len as int==plane_len as int*planes;
            let start=mul(plane,plane_len)?;
            assert(start as int+plane_len<=scratch_len) by(nonlinear_arith)
                requires plane<planes,scratch_len as int==plane_len as int*planes,start as int==plane as int*plane_len;
            let (head,tail)=scratch.split_at_mut(start);let (part,rest)=tail.split_at_mut(plane_len);
            assert(head@==prior_scratch.subrange(0,start as int));
            assert(rest@==prior_scratch.subrange(start as int+plane_len,prior_scratch.len() as int));
            let work=if !g.tiled {dp::validate_into(s,g,plane_len,limits.max_output,budget,part,true)?}
                else {let work=dp::validate(s,g,plane_len,limits.max_output,budget)?;dp::decode(s,g,plane_len,part)?;work};
            assert forall|k:int| 0<=k<plane_len implies part@[k]==dp::raw(s@,g,k) by {};
            work
        } else if f.compression==8 || f.compression==32946 {dp::validate(s,g,plane_len,limits.max_output,budget)?} else {validate_plane(s,g,plane_len,budget)?};
        proof {
            if eligible {planes_work_monotone(s@,d,le,l,(plane+1) as nat,planes as nat);}
            planes_work_step(s@,d,le,l,plane as nat);
        }
        assert(capture ==> work as int<=plane_bound);
        decode_work=add(decode_work,work)?;
        assert((plane as int+1)*plane_bound==plane as int*plane_bound+plane_bound) by(nonlinear_arith);
        geometry[plane]=g;
        proof {
            if capture {
                assert forall|k:int| 0<=k<(plane as int+1)*plane_len implies scratch@[k]==dp::raw(s@,geometry@[k/(plane_len as int)],k%(plane_len as int)) by {
                    quotient_remainder(k,plane_len as int);
                    assert(0<=k/(plane_len as int)<=plane) by(nonlinear_arith)
                        requires plane_len>0,k==plane_len as int*(k/(plane_len as int))+k%(plane_len as int),
                            0<=k%(plane_len as int)<plane_len,0<=k<(plane as int+1)*plane_len;
                    if k<plane as int*plane_len {
                        assert(k/(plane_len as int)<plane) by(nonlinear_arith)
                            requires plane_len>0,k==plane_len as int*(k/(plane_len as int))+k%(plane_len as int),
                                0<=k%(plane_len as int)<plane_len,k<plane as int*plane_len;
                        assert(scratch@[k]==prior_scratch[k]);
                    }
                    else {
                        assert(0<=k-plane as int*plane_len<plane_len) by(nonlinear_arith)
                            requires plane as int*plane_len<=k<(plane as int+1)*plane_len;
                        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(k,plane_len as int,plane as int,k-plane as int*plane_len);
                    }
                }
            }
        }
        plane=plane+1;
    }
    assert(0<=plane as int*plane_bound<=4*plane_bound) by(nonlinear_arith)
        requires plane<=4,plane_bound>=0;
    assert(plane as int*plane_len==scratch_len) by(nonlinear_arith)
        requires plane==planes,planes==l.planes,plane_len==l.plane_len,scratch_len==l.scratch_len,l.scratch_len as int==l.plane_len as int*l.planes;
    Ok(ExtendedImage {input:s,format:f,layout:l,geometry,directory:d,page,limits:*limits,decode_work})
}

/// Decode a Deflate TIFF into caller-owned buffers while validating strips.
/// Other compressions return Unsupported before writing. Errors may leave a
/// partial scratch prefix; output is unchanged on failure. The original inspection
/// API retains its prevalidation contract. Use deflate_decode_work_limit for a
/// sufficient budget and buffers of limits.max_output for sufficient capacity.
pub fn decode_deflate_into<'a>(s:&'a [u8],page:usize,limits:&Limits,output:&mut [u8],scratch:&mut [u8],budget:&mut Budget) -> (r:Result<ExtendedImage<'a>,Error>)
    ensures final(budget).remaining<=old(budget).remaining,
        supported_deflate_profile(s@,page,*limits) && old(budget).remaining as int>=deflate_decode_work_bound(page,*limits)
            && old(scratch).len()>=limits.max_output && old(output).len()>=limits.max_output ==> r.is_ok(),
        r.is_err() ==> final(output)@==old(output)@,
        final(output).len()==old(output).len(),final(scratch).len()==old(scratch).len(),
        r.is_ok() ==> r.unwrap().well_formed() && r.unwrap().metadata_matches_source(),
        r.is_ok() ==> r.unwrap().selected_page()==page && r.unwrap().byte_len()<=limits.max_output && r.unwrap().scratch_size()<=limits.max_output,
        r.is_ok() ==> forall|k:int| 0<=k<r.unwrap().byte_len() ==> final(output)@[k]==r.unwrap().decoded_byte(k),
        r.is_ok() ==> forall|k:int| r.unwrap().byte_len()<=k<old(output).len() ==> final(output)@[k]==old(output)@[k],
        forall|k:int| limits.max_output<=k<old(scratch).len() ==> final(scratch)@[k]==old(scratch)@[k],
{
    let image=inspect_extended_capture(s,page,limits,budget,scratch,true)?;
    if output.len()<image.output_len() {return Err(Error::OutputTooSmall);}
    spend(budget,image.decode_work_limit())?;
    image.render_native(scratch,output);
    Ok(image)
}

}
