use verified_tiff::{inspect_raster, inspect_extended, inspection_work_limit, extended_inspection_work_limit, raster_inspection_work_limit, Budget, Error, Limits};

const CASES: &[(&[u8], &[u8])] = &[
        (include_bytes!("fixtures/palette1.tif"), include_bytes!("fixtures/palette1.pixels")),
        (include_bytes!("fixtures/palette4.tif"), include_bytes!("fixtures/palette4.pixels")),
        (include_bytes!("fixtures/palette8.tif"), include_bytes!("fixtures/palette8.pixels")),
        (include_bytes!("fixtures/rgb16.tif"), include_bytes!("fixtures/rgb16.pixels")),
        (include_bytes!("fixtures/gray16.tif"), include_bytes!("fixtures/gray16.pixels")),
        (include_bytes!("fixtures/planar-alpha.tif"), include_bytes!("fixtures/planar-alpha.pixels")),
        (include_bytes!("fixtures/deflate-stored.tif"), include_bytes!("fixtures/deflate-stored.pixels")),
        (include_bytes!("fixtures/deflate-fixed.tif"), include_bytes!("fixtures/deflate-fixed.pixels")),
        (include_bytes!("fixtures/deflate-bilevel.tif"), include_bytes!("fixtures/deflate-bilevel.pixels")),
        (include_bytes!("fixtures/deflate-palette.tif"), include_bytes!("fixtures/deflate-palette.pixels")),
        (include_bytes!("fixtures/deflate-planar16.tif"), include_bytes!("fixtures/deflate-planar16.pixels")),
        (include_bytes!("fixtures/deflate-alpha.tif"), include_bytes!("fixtures/deflate-alpha.pixels")),
        (include_bytes!("fixtures/deflate-dynamic.tif"), include_bytes!("fixtures/deflate-dynamic.pixels")),
        (include_bytes!("fixtures/deflate-witness.tif"), include_bytes!("fixtures/deflate-witness.pixels")),
        (include_bytes!("fixtures/acceptance-palette.tif"), include_bytes!("fixtures/acceptance-palette.pixels")),
        (include_bytes!("fixtures/acceptance-gray16.tif"), include_bytes!("fixtures/acceptance-gray16.pixels")),
        (include_bytes!("fixtures/acceptance-planar.tif"), include_bytes!("fixtures/acceptance-planar.pixels")),
        (include_bytes!("fixtures/acceptance-deflate.tif"), include_bytes!("fixtures/acceptance-deflate.pixels")),
        (include_bytes!("fixtures/acceptance-deflate-bilevel.tif"), include_bytes!("fixtures/acceptance-deflate-bilevel.pixels")),
    ];
#[test]
fn broader_formats_pixels_quotas_and_buffer_frames() {
    for &(source, expected) in CASES {
        let image=inspect_raster(source,0,&Limits::conservative(),&mut Budget {remaining:1_000_000}).unwrap();
        assert_eq!(image.output_len(),expected.len());
        let mut output=vec![0xa5;image.output_len()+11];
        let mut scratch=vec![0x5a;image.scratch_len()+13];
        let quota=image.decode_work_limit();
        let mut low=Budget {remaining:quota-1};
        assert_eq!(image.decode_with_scratch(&mut output,&mut scratch,&mut low),Err(Error::Limit));
        assert_eq!(low.remaining,quota-1);
        assert!(output.iter().all(|v|*v==0xa5));
        assert!(scratch.iter().all(|v|*v==0x5a));
        for (outlen,scratchlen) in [(image.output_len()-1,image.scratch_len()),(image.output_len(),image.scratch_len()-1)] {
            let mut b=Budget {remaining:quota};
            assert_eq!(image.decode_with_scratch(&mut output[..outlen],&mut scratch[..scratchlen],&mut b),Err(Error::OutputTooSmall));
            assert_eq!(b.remaining,quota);
            assert!(output.iter().all(|v|*v==0xa5));
            assert!(scratch.iter().all(|v|*v==0x5a));
        }
        let mut exact=Budget {remaining:quota};
        image.decode_with_scratch(&mut output,&mut scratch,&mut exact).unwrap();
        assert_eq!(exact.remaining,0);
        assert_eq!(&output[..expected.len()],expected);
        assert!(output[expected.len()..].iter().all(|v|*v==0xa5));
        assert!(scratch[image.scratch_len()..].iter().all(|v|*v==0x5a));
    }
}

#[test]
fn sufficient_inspection_budgets_and_checked_overflow() {
    let limits=Limits::conservative();
    let extended=extended_inspection_work_limit(0,&limits).unwrap();
    let raster=raster_inspection_work_limit(0,&limits).unwrap();
    assert_eq!(raster,extended+inspection_work_limit(0,&limits).unwrap()+verified_tiff::jpeg_inspection_work_limit(0,&limits).unwrap()+verified_tiff::fax_inspection_work_limit(0,&limits).unwrap()+verified_tiff::webp_inspection_work_limit(0,&limits).unwrap()+verified_tiff::logluv_inspection_work_limit(0,&limits).unwrap());
    for &(source,expected) in CASES {
        let mut eb=Budget {remaining:extended};
        let image=inspect_extended(source,0,&limits,&mut eb).unwrap();
        assert_eq!(image.output_len(),expected.len());
        assert!(eb.remaining<extended);
        let mut rb=Budget {remaining:raster};
        let image=inspect_raster(source,0,&limits,&mut rb).unwrap();
        assert_eq!(image.output_len(),expected.len());
        assert!(rb.remaining<raster);
    }
    let tight=Limits {max_input:200,max_dimension:1,max_output:3,max_directories:3,max_tags:11,max_segments:3};
    assert_eq!(extended_inspection_work_limit(0,&tight),Ok(20288));
    assert_eq!(raster_inspection_work_limit(0,&tight),Ok(3156960));
    let mut chain=include_bytes!("fixtures/acceptance-deflate.tif").to_vec();
    chain[142..146].copy_from_slice(&8u32.to_le_bytes());
    for page in 0..3 {
        let mut budget=Budget {remaining:raster_inspection_work_limit(page,&tight).unwrap()};
        let image=inspect_raster(&chain,page,&tight,&mut budget).unwrap();
        assert_eq!(image.page_index(),page);
        let mut output=[0];let mut scratch=[0];
        let mut work=Budget {remaining:image.decode_work_limit()};
        image.decode_with_scratch(&mut output,&mut scratch,&mut work).unwrap();
        assert_eq!(output,[65]);
    }
    assert_eq!(extended_inspection_work_limit(usize::MAX,&limits),Err(Error::Limit));
    assert_eq!(raster_inspection_work_limit(usize::MAX,&limits),Err(Error::Limit));
    let empty=Limits {max_input:usize::MAX,max_output:usize::MAX,max_dimension:1,max_directories:1,max_tags:0,max_segments:0};
    assert_eq!(extended_inspection_work_limit(0,&empty),Ok(5));
    let huge=Limits {max_segments:1,..empty};
    assert_eq!(extended_inspection_work_limit(0,&huge),Err(Error::Limit));
}

#[test]
fn combined_deflate_matches_prevalidated_pixels_budgets_and_frames() {
    use verified_tiff::{decode_deflate_into,deflate_decode_work_limit};
    for &(source,expected) in CASES {
        let limits=Limits {max_input:source.len(),max_output:expected.len(),..Limits::conservative()};
        let quota=deflate_decode_work_limit(0,&limits).unwrap();
        let mut before=Budget {remaining:quota};
        let old=inspect_extended(source,0,&limits,&mut before).unwrap();
        if ![8,32946].contains(&old.compression()) {continue;}
        let exact=quota-before.remaining+old.decode_work_limit();
        let mut output=vec![0xa5;expected.len()+11];
        let mut scratch=vec![0x5a;limits.max_output+13];
        let mut budget=Budget {remaining:exact};
        let image=decode_deflate_into(source,0,&limits,&mut output,&mut scratch,&mut budget).unwrap();
        assert_eq!(budget.remaining,0);
        assert_eq!(&output[..expected.len()],expected);
        assert_eq!((image.width(),image.height(),image.channels()),(old.width(),old.height(),old.channels()));
        assert!(output[expected.len()..].iter().all(|&v|v==0xa5));
        assert!(scratch[old.scratch_len()..].iter().all(|&v|v==0x5a));
        for budget in [0,exact-1] {
            output.fill(0xa5);scratch.fill(0x5a);
            let r=decode_deflate_into(source,0,&limits,&mut output,&mut scratch,&mut Budget {remaining:budget});
            assert!(matches!(r,Err(Error::Limit)));
            assert!(output.iter().all(|&v|v==0xa5));
            assert!(scratch[old.scratch_len()..].iter().all(|&v|v==0x5a));
        }
        for (olen,slen) in [(expected.len()-1,old.scratch_len()),(expected.len(),old.scratch_len()-1)] {
            output.fill(0xa5);
            let r=decode_deflate_into(source,0,&limits,&mut output[..olen],&mut scratch[..slen],&mut Budget {remaining:quota});
            assert!(matches!(r,Err(Error::OutputTooSmall)));
            assert!(output.iter().all(|&v|v==0xa5));
        }
        output.fill(0xa5);scratch.fill(0x5a);
        decode_deflate_into(source,0,&limits,&mut output,&mut scratch,&mut Budget {remaining:quota}).unwrap();
        assert_eq!(&output[..expected.len()],expected);
    }
    assert_eq!(deflate_decode_work_limit(usize::MAX,&Limits::conservative()),Err(Error::Limit));
}

#[test]
fn combined_deflate_mutations_preserve_acceptance_and_padding_checks() {
    use verified_tiff::{decode_deflate_into,deflate_decode_work_limit};
    let sources: &[&[u8]]=&[
        include_bytes!("fixtures/acceptance-deflate.tif"),
        include_bytes!("fixtures/deflate-fixed.tif"),
        include_bytes!("fixtures/deflate-dynamic.tif"),
        include_bytes!("upstream/deflate-last-strip-extra-data.tiff"),
    ];
    for &original in sources {
        let limits=Limits {max_input:original.len()+1,max_output:1_000_000,..Limits::conservative()};
        let quota=deflate_decode_work_limit(0,&limits).unwrap();
        let mut out=vec![0xa5;limits.max_output+1];let mut scratch=vec![0x5a;limits.max_output+1];
        for mutation in 0..65 {
            let mut data=original.to_vec();
            if mutation>0 {let at=(mutation-1)*(data.len()-1)/63;data[at]^=1<<((mutation-1)%8);}
            let old=inspect_extended(&data,0,&limits,&mut Budget {remaining:quota});
            out.fill(0xa5);scratch.fill(0x5a);
            let new=decode_deflate_into(&data,0,&limits,&mut out,&mut scratch,&mut Budget {remaining:quota});
            match old {
                Ok(image) if [8,32946].contains(&image.compression())=>{
                    let new=new.unwrap();assert_eq!(new.output_len(),image.output_len());
                    let mut expected=vec![0;image.output_len()];let mut temp=vec![0;image.scratch_len()];
                    image.decode_with_scratch(&mut expected,&mut temp,&mut Budget {remaining:quota}).unwrap();
                    assert_eq!(&out[..expected.len()],expected);
                    assert!(out[expected.len()..].iter().all(|&v|v==0xa5));
                },
                _=>{assert!(new.is_err());assert!(out.iter().all(|&v|v==0xa5));}
            }
            assert_eq!(scratch[limits.max_output],0x5a);
        }
    }
}
