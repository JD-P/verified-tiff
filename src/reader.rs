use super::*;

verus! {

/// Output sample container type. Float64 and LogLuv16 bytes are little-endian.
/// LogLuv luminance retains its sign/log-magnitude encoding; see ColorModel.
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum SampleFormat {UnsignedInteger,SignedInteger,IeeeFloat}
/// Color interpretation independent of numeric container and channel count.
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum ColorModel {Gray,Rgb,LogLuv}

enum Kind<'a> {Basic(Image<'a>),Extended(ExtendedImage<'a>),Jpeg(JpegImage<'a>),Fax(FaxImage<'a>),Webp(WebpImage<'a>),Logluv(LogluvImage<'a>)}
pub struct Raster<'a> {inner:Kind<'a>}

impl<'a> Raster<'a> {
    pub closed spec fn well_formed(&self) -> bool {match &self.inner {Kind::Basic(i)=>i.well_formed(),Kind::Extended(i)=>i.well_formed(),Kind::Jpeg(i)=>i.well_formed(),Kind::Fax(i)=>i.well_formed(),Kind::Webp(i)=>i.well_formed(),Kind::Logluv(i)=>i.well_formed()}}
    pub closed spec fn byte_len(&self) -> nat {match &self.inner {Kind::Basic(i)=>i.byte_len() as nat,Kind::Extended(i)=>i.byte_len(),Kind::Jpeg(i)=>i.byte_len(),Kind::Fax(i)=>i.byte_len(),Kind::Webp(i)=>i.byte_len(),Kind::Logluv(i)=>i.byte_len()}}
    pub closed spec fn scratch_size(&self) -> nat {match &self.inner {Kind::Basic(i)=>0nat,Kind::Extended(i)=>i.scratch_size(),Kind::Jpeg(i)=>i.scratch_size(),Kind::Fax(i)=>i.scratch_size(),Kind::Webp(i)=>i.scratch_size(),Kind::Logluv(i)=>i.scratch_size()}}
    pub closed spec fn work_len(&self) -> nat {match &self.inner {Kind::Basic(i)=>i.work_len() as nat,Kind::Extended(i)=>i.work_len(),Kind::Jpeg(i)=>i.work_len(),Kind::Fax(i)=>i.work_len(),Kind::Webp(i)=>i.work_len(),Kind::Logluv(i)=>i.work_len()}}
    pub closed spec fn metadata_matches_source(&self) -> bool {match &self.inner {Kind::Basic(i)=>i.metadata_matches_source(),Kind::Extended(i)=>i.metadata_matches_source(),Kind::Jpeg(i)=>i.metadata_matches_source(),Kind::Fax(i)=>i.metadata_matches_source(),Kind::Webp(i)=>i.metadata_matches_source(),Kind::Logluv(i)=>i.metadata_matches_source()}}
    pub closed spec fn decoded_byte(&self,k:int) -> u8 {match &self.inner {Kind::Basic(i)=>i.decoded_byte(k),Kind::Extended(i)=>i.decoded_byte(k),Kind::Jpeg(i)=>i.decoded_byte(k),Kind::Fax(i)=>i.decoded_byte(k),Kind::Webp(i)=>i.decoded_byte(k),Kind::Logluv(i)=>i.decoded_byte(k)}}
    pub fn width(&self) -> (r:usize) {match &self.inner {Kind::Basic(i)=>i.width(),Kind::Extended(i)=>i.width(),Kind::Jpeg(i)=>i.width(),Kind::Fax(i)=>i.width(),Kind::Webp(i)=>i.width(),Kind::Logluv(i)=>i.width()}}
    pub fn height(&self) -> (r:usize) {match &self.inner {Kind::Basic(i)=>i.height(),Kind::Extended(i)=>i.height(),Kind::Jpeg(i)=>i.height(),Kind::Fax(i)=>i.height(),Kind::Webp(i)=>i.height(),Kind::Logluv(i)=>i.height()}}
    pub fn channels(&self) -> (r:usize) {match &self.inner {Kind::Basic(i)=>i.channels(),Kind::Extended(i)=>i.channels(),Kind::Jpeg(i)=>i.channels(),Kind::Fax(i)=>i.channels(),Kind::Webp(i)=>i.channels(),Kind::Logluv(i)=>i.channels()}}
    pub fn bits_per_sample(&self) -> (r:usize) {match &self.inner {Kind::Basic(i)=>i.bits_per_sample(),Kind::Extended(i)=>i.bits_per_sample(),Kind::Jpeg(i)=>i.bits_per_sample(),Kind::Fax(i)=>i.bits_per_sample(),Kind::Webp(i)=>i.bits_per_sample(),Kind::Logluv(i)=>i.bits_per_sample()}}
    pub closed spec fn floating_point(&self) -> bool {match &self.inner {Kind::Extended(i)=>i.floating_point(),_=>false}}
    pub closed spec fn source_predictor(&self) -> nat {match &self.inner {Kind::Extended(i)=>i.source_predictor(),_=>1}}
    pub closed spec fn logarithmic_color(&self) -> bool {match &self.inner {Kind::Logluv(_)=>true,_=>false}}
    pub fn color_model(&self) -> (r:ColorModel)
        ensures (r==ColorModel::LogLuv)==self.logarithmic_color(),
    {match &self.inner {Kind::Logluv(_)=>ColorModel::LogLuv,_=>if self.channels()<=2 {ColorModel::Gray} else {ColorModel::Rgb}}}
    pub fn sample_format(&self) -> (r:SampleFormat)
        ensures (r==SampleFormat::IeeeFloat)==self.floating_point(),
            (r==SampleFormat::SignedInteger)==self.logarithmic_color(),
    {match &self.inner {Kind::Extended(i)=>if i.is_floating_point() {SampleFormat::IeeeFloat} else {SampleFormat::UnsignedInteger},Kind::Logluv(_)=>SampleFormat::SignedInteger,_=>SampleFormat::UnsignedInteger}}
    pub fn predictor(&self) -> (r:usize) ensures r==self.source_predictor()
    {match &self.inner {Kind::Extended(i)=>i.predictor(),_=>1}}
    pub fn compression(&self) -> (r:usize) {match &self.inner {Kind::Basic(i)=>i.compression(),Kind::Extended(i)=>i.compression(),Kind::Jpeg(i)=>i.compression(),Kind::Fax(i)=>i.compression(),Kind::Webp(i)=>i.compression(),Kind::Logluv(i)=>i.compression()}}
    pub fn page_index(&self) -> (r:usize) {match &self.inner {Kind::Basic(i)=>i.page_index(),Kind::Extended(i)=>i.page_index(),Kind::Jpeg(i)=>i.page_index(),Kind::Fax(i)=>i.page_index(),Kind::Webp(i)=>i.page_index(),Kind::Logluv(i)=>i.page_index()}}
    pub fn directory_entry_count(&self) -> (r:usize) {match &self.inner {Kind::Basic(i)=>i.directory_entry_count(),Kind::Extended(i)=>i.directory_entry_count(),Kind::Jpeg(i)=>i.directory_entry_count(),Kind::Fax(i)=>i.directory_entry_count(),Kind::Webp(i)=>i.directory_entry_count(),Kind::Logluv(i)=>i.directory_entry_count()}}
    pub fn alpha_mode(&self) -> (r:AlphaMode) {match &self.inner {Kind::Basic(i)=>i.alpha_mode(),Kind::Extended(i)=>i.alpha_mode(),Kind::Jpeg(i)=>i.alpha_mode(),Kind::Fax(i)=>i.alpha_mode(),Kind::Webp(i)=>i.alpha_mode(),Kind::Logluv(i)=>i.alpha_mode()}}
    pub fn has_alpha(&self) -> (r:bool) {match &self.inner {Kind::Basic(i)=>i.has_alpha(),Kind::Extended(i)=>i.has_alpha(),Kind::Jpeg(i)=>i.has_alpha(),Kind::Fax(i)=>i.has_alpha(),Kind::Webp(i)=>i.has_alpha(),Kind::Logluv(i)=>i.has_alpha()}}
    pub fn planar_configuration(&self) -> (r:usize) {match &self.inner {Kind::Basic(i)=>1,Kind::Extended(i)=>i.planar_configuration(),Kind::Jpeg(i)=>i.planar_configuration(),Kind::Fax(i)=>i.planar_configuration(),Kind::Webp(i)=>i.planar_configuration(),Kind::Logluv(i)=>i.planar_configuration()}}
    pub fn output_bits_per_sample(&self) -> (r:usize) {match &self.inner {Kind::Basic(i)=>8,Kind::Extended(i)=>i.output_bits_per_sample(),Kind::Jpeg(i)=>i.output_bits_per_sample(),Kind::Fax(i)=>i.output_bits_per_sample(),Kind::Webp(i)=>i.output_bits_per_sample(),Kind::Logluv(i)=>i.output_bits_per_sample()}}
    pub fn output_len(&self) -> (r:usize) ensures r==self.byte_len() {match &self.inner {Kind::Basic(i)=>i.output_len(),Kind::Extended(i)=>i.output_len(),Kind::Jpeg(i)=>i.output_len(),Kind::Fax(i)=>i.output_len(),Kind::Webp(i)=>i.output_len(),Kind::Logluv(i)=>i.output_len()}}
    pub fn scratch_len(&self) -> (r:usize) ensures r==self.scratch_size() {match &self.inner {Kind::Basic(i)=>0,Kind::Extended(i)=>i.scratch_len(),Kind::Jpeg(i)=>i.scratch_len(),Kind::Fax(i)=>i.scratch_len(),Kind::Webp(i)=>i.scratch_len(),Kind::Logluv(i)=>i.scratch_len()}}
    pub fn decode_work_limit(&self) -> (r:usize) ensures r==self.work_len() {match &self.inner {Kind::Basic(i)=>i.decode_work_limit(),Kind::Extended(i)=>i.decode_work_limit(),Kind::Jpeg(i)=>i.decode_work_limit(),Kind::Fax(i)=>i.decode_work_limit(),Kind::Webp(i)=>i.decode_work_limit(),Kind::Logluv(i)=>i.decode_work_limit()}}
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
        match &self.inner {Kind::Basic(i)=>i.decode_into(output,budget),Kind::Extended(i)=>i.decode_with_scratch(output,scratch,budget),Kind::Jpeg(i)=>i.decode_with_scratch(output,scratch,budget),Kind::Fax(i)=>i.decode_with_scratch(output,scratch,budget),Kind::Webp(i)=>i.decode_with_scratch(output,scratch,budget),Kind::Logluv(i)=>i.decode_with_scratch(output,scratch,budget)}
    }
    /// Zero-scratch convenience. Broader formats use decode_with_scratch.
    pub fn decode_into(&self,output:&mut [u8],budget:&mut Budget) -> (r:Result<(),Error>)
        requires self.well_formed(),
        ensures final(output).len()==old(output).len(),final(budget).remaining<=old(budget).remaining,
            r.is_ok()==(self.scratch_size()==0 && old(output).len()>=self.byte_len() && old(budget).remaining>=self.work_len()),
            r.is_ok() ==> final(budget).remaining as int+self.work_len()==old(budget).remaining,
            r.is_ok() ==> forall|k:int| 0<=k<self.byte_len() ==> final(output)@[k]==self.decoded_byte(k),
            forall|k:int| self.byte_len()<=k<old(output).len() ==> final(output)@[k]==old(output)@[k],
    {
        let mut scratch=[0u8;0];
        self.decode_with_scratch(output,&mut scratch,budget)
    }
}

/// Union of the zero-scratch and caller-scratch mathematical TIFF profiles.
pub open spec fn supported_raster_profile(s:Seq<u8>,page:usize,limits:Limits) -> bool {
    supported_profile(s,page,limits) || supported_extended_profile(s,page,limits) || supported_jpeg_profile(s,page,limits) || supported_fax_profile(s,page,limits) || supported_webp_profile(s,page,limits) || supported_logluv_profile(s,page,limits)
}
pub open spec fn raster_inspection_work_bound(page:usize,limits:Limits) -> int {
    inspection_work_bound(page,limits)+extended_inspection_work_bound(page,limits)+jpeg_inspection_work_bound(page,limits)+fax_inspection_work_bound(page,limits)+webp_inspection_work_bound(page,limits)+logluv_inspection_work_bound(page,limits)
}
pub fn raster_inspection_work_limit(page:usize,limits:&Limits) -> (r:Result<usize,Error>)
    ensures r.is_ok()==(raster_inspection_work_bound(page,*limits)<=usize::MAX),
        r.is_ok() ==> r.unwrap()==raster_inspection_work_bound(page,*limits),
{
    add(add(add(add(add(inspection_work_limit(page,limits)?,extended_inspection_work_limit(page,limits)?)?,jpeg_inspection_work_limit(page,limits)?)?,fax_inspection_work_limit(page,limits)?)?,webp_inspection_work_limit(page,limits)?)?,logluv_inspection_work_limit(page,limits)?)
}

pub fn inspect_raster<'a>(s:&'a [u8],page:usize,limits:&Limits,budget:&mut Budget) -> (r:Result<Raster<'a>,Error>)
    ensures final(budget).remaining<=old(budget).remaining,
        supported_profile(s@,page,*limits) && old(budget).remaining as int>=inspection_work_bound(page,*limits) ==> r.is_ok(),
        supported_raster_profile(s@,page,*limits) && old(budget).remaining as int>=raster_inspection_work_bound(page,*limits) ==> r.is_ok(),
        r.is_ok() ==> r.unwrap().well_formed() && r.unwrap().metadata_matches_source(),
        r.is_ok() ==> r.unwrap().byte_len()<=limits.max_output && r.unwrap().scratch_size()<=limits.max_output,
{
    // Bound the first attempt and charge its actual work even when it fails.
    // The second inspector validates independently; no failed-probe state is reused.
    let quota=match inspection_work_limit(page,limits) {
        Ok(bound)=>if bound<budget.remaining {bound} else {budget.remaining},
        Err(_)=>budget.remaining,
    };
    let mut probe=Budget {remaining:quota};
    let first=inspect(s,page,limits,&mut probe);
    spend(budget,quota-probe.remaining)?;
    match first {
        Ok(i)=>Ok(Raster {inner:Kind::Basic(i)}),
        Err(_)=>{
            let quota=match extended_inspection_work_limit(page,limits) {
                Ok(bound)=>if bound<budget.remaining {bound} else {budget.remaining},
                Err(_)=>budget.remaining,
            };
            let mut probe=Budget {remaining:quota};
            let second=inspect_extended(s,page,limits,&mut probe);
            spend(budget,quota-probe.remaining)?;
            match second {
                Ok(i)=>Ok(Raster {inner:Kind::Extended(i)}),
                Err(previous)=>{
                    let quota=match jpeg_inspection_work_limit(page,limits) {
                        Ok(bound)=>if bound<budget.remaining {bound} else {budget.remaining},
                        Err(_)=>budget.remaining,
                    };
                    let mut probe=Budget {remaining:quota};
                    let third=inspect_jpeg_tiff(s,page,limits,&mut probe);
                    spend(budget,quota-probe.remaining)?;
                    match third {
                        Ok(i)=>Ok(Raster {inner:Kind::Jpeg(i)}),
                        Err(jpeg_error)=>{
                            let quota=match fax_inspection_work_limit(page,limits) {
                                Ok(bound)=>if bound<budget.remaining {bound} else {budget.remaining},
                                Err(_)=>budget.remaining,
                            };
                            let mut probe=Budget {remaining:quota};
                            let fourth=inspect_fax_tiff(s,page,limits,&mut probe);
                            spend(budget,quota-probe.remaining)?;
                            match fourth {
                                Ok(i)=>Ok(Raster {inner:Kind::Fax(i)}),
                                Err(fax_error)=>{
                                    let quota=match webp_inspection_work_limit(page,limits) {
                                        Ok(bound)=>if bound<budget.remaining {bound} else {budget.remaining},
                                        Err(_)=>budget.remaining,
                                    };
                                    let mut probe=Budget {remaining:quota};
                                    let fifth=inspect_webp_tiff(s,page,limits,&mut probe);
                                    spend(budget,quota-probe.remaining)?;
                                    match fifth {
                                        Ok(i)=>Ok(Raster {inner:Kind::Webp(i)}),
                                        Err(webp_error)=>match inspect_logluv_tiff(s,page,limits,budget) {
                                            Ok(i)=>Ok(Raster {inner:Kind::Logluv(i)}),
                                            Err(Error::Unsupported)=>if webp_error!=Error::Unsupported {Err(webp_error)} else if fax_error!=Error::Unsupported {Err(fax_error)} else if jpeg_error!=Error::Unsupported {Err(jpeg_error)} else {Err(previous)},
                                            Err(e)=>Err(e),
                                        },
                                    }
                                },
                            }
                        },
                    }
                },
            }
        },
    }
}

}
