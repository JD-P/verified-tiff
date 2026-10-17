use verified_tiff::{decode_jpeg_into, inspect_jpeg_tiff, inspect_raster, jpeg_decode_work_limit, jpeg_inspection_work_limit, raster_inspection_work_limit, Budget, Error, Limits};
const CASES:&[(&[u8],&[u8])]=&[
(include_bytes!("fixtures/jpeg-witness.tif"),include_bytes!("fixtures/jpeg-witness.pixels")),
(include_bytes!("fixtures/jpeg-gray.tif"),include_bytes!("fixtures/jpeg-gray.pixels")),
(include_bytes!("fixtures/jpeg-rgb.tif"),include_bytes!("fixtures/jpeg-rgb.pixels")),
(include_bytes!("fixtures/jpeg-ycbcr.tif"),include_bytes!("fixtures/jpeg-ycbcr.pixels")),
(include_bytes!("fixtures/jpeg-tile.tif"),include_bytes!("fixtures/jpeg-tile.pixels")),
(include_bytes!("fixtures/ojpeg-tables.tif"),include_bytes!("fixtures/ojpeg-tables.pixels")),
(include_bytes!("fixtures/ojpeg-header.tif"),include_bytes!("fixtures/ojpeg-header.pixels")),
];
#[test]
fn jpeg_pixels_exact_quotas_and_buffer_frames() {
    for &(source,expected) in CASES {
        let lim=Limits::conservative();
        let mut ib=Budget{remaining:jpeg_inspection_work_limit(0,&lim).unwrap()};
        let direct=inspect_jpeg_tiff(source,0,&lim,&mut ib).unwrap();
        let mut ib=Budget{remaining:raster_inspection_work_limit(0,&lim).unwrap()};
        let image=inspect_raster(source,0,&lim,&mut ib).unwrap();
        assert_eq!(image.output_len(),expected.len());
        assert_eq!(image.output_len(),direct.output_len());
        assert_eq!(image.scratch_len(),direct.scratch_len());
        let mut out=vec![0xa5;image.output_len()+11];let mut scratch=vec![0x5a;image.scratch_len()+13];
        let quota=image.decode_work_limit();
        for (o,s,b,e) in [(image.output_len()-1,image.scratch_len(),quota,Error::OutputTooSmall),(image.output_len(),image.scratch_len()-1,quota,Error::OutputTooSmall),(image.output_len(),image.scratch_len(),quota-1,Error::Limit)] {
            let mut budget=Budget{remaining:b};
            assert_eq!(image.decode_with_scratch(&mut out[..o],&mut scratch[..s],&mut budget),Err(e));
            assert_eq!(budget.remaining,b);assert!(out.iter().all(|v|*v==0xa5));assert!(scratch.iter().all(|v|*v==0x5a));
        }
        let mut budget=Budget{remaining:quota};image.decode_with_scratch(&mut out,&mut scratch,&mut budget).unwrap();
        assert_eq!(budget.remaining,0);
        assert!(out[..expected.len()].iter().zip(expected).all(|(a,b)|a.abs_diff(*b)<=3));
        assert!(out[expected.len()..].iter().all(|v|*v==0xa5));assert!(scratch[image.scratch_len()..].iter().all(|v|*v==0x5a));
    }
}
#[test]
fn jpeg_budget_formula_and_overflow() {
    let l=Limits{max_input:300,max_output:65,max_dimension:1,max_tags:10,max_segments:1,max_directories:1};
    assert_eq!(jpeg_inspection_work_limit(0,&l),Ok(205910));
    assert_eq!(jpeg_inspection_work_limit(usize::MAX,&l),Err(Error::Limit));
    assert_eq!(jpeg_inspection_work_limit(0,&Limits{max_input:usize::MAX,..l}),Err(Error::Limit));
    assert_eq!(jpeg_inspection_work_limit(0,&Limits{max_output:usize::MAX,max_segments:0,..l}),Ok(1339));
    assert_eq!(jpeg_decode_work_limit(0,&l),Ok(205910+601+3138*65));
    assert_eq!(jpeg_decode_work_limit(usize::MAX,&l),Err(Error::Limit));
    assert_eq!(jpeg_decode_work_limit(0,&Limits{max_output:usize::MAX,max_segments:0,..l}),Ok(1339));
}
#[test]
fn one_pass_jpeg_matches_inspected_decode_budgets_and_frames() {
    for &(source,expected) in CASES {
        let lim=Limits::conservative();
        let quota=jpeg_inspection_work_limit(0,&lim).unwrap();
        let mut ib=Budget{remaining:quota};
        let image=inspect_jpeg_tiff(source,0,&lim,&mut ib).unwrap();
        let (olen,slen)=(image.output_len(),image.scratch_len());
        let mut reference=vec![0;olen];let mut scratch=vec![0;slen];
        image.decode_with_scratch(&mut reference,&mut scratch,&mut Budget{remaining:image.decode_work_limit()}).unwrap();
        let exact=quota-ib.remaining+image.decode_work_limit();
        let mut out=vec![0xa5;olen+11];let mut scratch=vec![0x5a;slen+13];
        let mut budget=Budget{remaining:exact};
        let one=decode_jpeg_into(source,0,&lim,&mut out,&mut scratch,&mut budget).unwrap();
        assert_eq!(budget.remaining,0);assert_eq!(&out[..olen],&reference[..]);
        assert!(out[..olen].iter().zip(expected).all(|(a,b)|a.abs_diff(*b)<=3));
        assert_eq!((one.width(),one.height(),one.channels(),one.output_len(),one.scratch_len(),one.decode_work_limit()),
            (image.width(),image.height(),image.channels(),olen,slen,image.decode_work_limit()));
        assert!(out[olen..].iter().all(|v|*v==0xa5));assert!(scratch[slen..].iter().all(|v|*v==0x5a));
        for b in [0,exact-1] {
            out.fill(0xa5);scratch.fill(0x5a);
            assert!(decode_jpeg_into(source,0,&lim,&mut out,&mut scratch,&mut Budget{remaining:b}).is_err());
            assert!(out.iter().all(|v|*v==0xa5));assert!(scratch[slen..].iter().all(|v|*v==0x5a));
        }
        for (o,s) in [(olen-1,slen),(olen,slen-1)] {
            out.fill(0xa5);
            assert_eq!(decode_jpeg_into(source,0,&lim,&mut out[..o],&mut scratch[..s],&mut Budget{remaining:exact}).err(),Some(Error::OutputTooSmall));
            assert!(out.iter().all(|v|*v==0xa5));
        }
        let fit=Limits{max_input:source.len(),max_output:slen.max(olen),..lim};
        let mut out=vec![0xa5;fit.max_output];let mut scratch=vec![0x5a;fit.max_output];
        decode_jpeg_into(source,0,&fit,&mut out,&mut scratch,&mut Budget{remaining:jpeg_decode_work_limit(0,&fit).unwrap()}).unwrap();
        assert_eq!(&out[..olen],&reference[..]);
    }
}

#[test]
fn concrete_gray_witness_is_exact() {
    let source=include_bytes!("fixtures/jpeg-witness.tif");
    let l=Limits{max_input:271,max_output:65,max_dimension:1,max_tags:10,max_segments:1,max_directories:1};
    let mut budget=Budget{remaining:jpeg_inspection_work_limit(0,&l).unwrap()};
    let image=inspect_jpeg_tiff(source,0,&l,&mut budget).unwrap();
    assert_eq!(image.decode_work_limit(),3413);assert_eq!(image.scratch_len(),65);
    let mut out=[0];let mut scratch=[0;65];let mut work=Budget{remaining:3413};
    image.decode_with_scratch(&mut out,&mut scratch,&mut work).unwrap();assert_eq!(out,[128]);
}
