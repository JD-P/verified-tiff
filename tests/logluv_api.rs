use verified_tiff::{inspect_logluv_tiff,inspect_raster,logluv_inspection_work_limit,raster_inspection_work_limit,Budget,ColorModel,SampleFormat,Error,Limits};
const ORIGINAL:&[u8]=include_bytes!("upstream/logluv-3c-16b.tiff");
const CASES:&[(&[u8],usize,&[u8])]=&[
    (ORIGINAL,0,&[0,0,1,27,184,60]),
    (include_bytes!("fixtures/logluv-strip.tif"),0,include_bytes!("fixtures/logluv-strip.pixels")),
    (include_bytes!("fixtures/logluv-tile.tif"),0,include_bytes!("fixtures/logluv-tile.pixels")),
    (include_bytes!("fixtures/logluv-pages.tif"),0,include_bytes!("fixtures/logluv-pages.pixels")),
    (include_bytes!("fixtures/logluv-pages.tif"),1,include_bytes!("fixtures/logluv-pages-page1.pixels")),
];
#[test]
fn logluv_pixels_quotas_and_buffer_frames() {
    for &(source,page,expected) in CASES {
        let lim=Limits::conservative();
        let mut budget=Budget{remaining:logluv_inspection_work_limit(page,&lim).unwrap()};
        let direct=inspect_logluv_tiff(source,page,&lim,&mut budget).unwrap();
        let mut budget=Budget{remaining:raster_inspection_work_limit(page,&lim).unwrap()};
        let image=inspect_raster(source,page,&lim,&mut budget).unwrap();
        assert_eq!(image.page_index(),page);assert_eq!(image.color_model(),ColorModel::LogLuv);
        assert_eq!(image.sample_format(),SampleFormat::SignedInteger);assert_eq!(image.channels(),3);
        assert_eq!(image.output_bits_per_sample(),16);assert_eq!(image.compression(),34676);assert!(!image.has_alpha());
        assert_eq!(image.output_len(),expected.len());assert_eq!(image.output_len(),direct.output_len());
        assert_eq!(image.scratch_len(),direct.scratch_len());assert_eq!(image.decode_work_limit(),direct.decode_work_limit());
        let mut out=vec![0xa5;image.output_len()+11];let mut scratch=vec![0x5a;image.scratch_len()+13];
        let quota=image.decode_work_limit();
        for (o,s,b,e) in [(image.output_len()-1,image.scratch_len(),quota,Error::OutputTooSmall),
                          (image.output_len(),image.scratch_len()-1,quota,Error::OutputTooSmall),
                          (image.output_len(),image.scratch_len(),quota-1,Error::Limit)] {
            let mut budget=Budget{remaining:b};
            assert_eq!(image.decode_with_scratch(&mut out[..o],&mut scratch[..s],&mut budget),Err(e));
            assert_eq!(budget.remaining,b);assert!(out.iter().all(|v|*v==0xa5));assert!(scratch.iter().all(|v|*v==0x5a));
        }
        let mut budget=Budget{remaining:quota};image.decode_with_scratch(&mut out,&mut scratch,&mut budget).unwrap();
        assert_eq!(budget.remaining,0);assert_eq!(&out[..expected.len()],expected);
        assert!(out[expected.len()..].iter().all(|v|*v==0xa5));assert!(scratch[image.scratch_len()..].iter().all(|v|*v==0x5a));
    }
}
#[test]
fn logluv_concrete_witness_and_budget_overflow() {
    let l=Limits{max_input:166,max_output:10,max_dimension:1,max_tags:11,max_segments:1,max_directories:1};
    assert_eq!(logluv_inspection_work_limit(0,&l),Ok(327));
    let mut budget=Budget{remaining:327};let image=inspect_logluv_tiff(ORIGINAL,0,&l,&mut budget).unwrap();
    assert_eq!((image.width(),image.height(),image.output_len(),image.scratch_len(),image.decode_work_limit()),(1,1,6,10,29));
    let mut out=[0;6];let mut scratch=[0;10];let mut budget=Budget{remaining:29};
    image.decode_with_scratch(&mut out,&mut scratch,&mut budget).unwrap();
    assert_eq!(out,[0,0,1,27,184,60]);assert_eq!(budget.remaining,0);
    assert_eq!(logluv_inspection_work_limit(usize::MAX,&l),Err(Error::Limit));
    assert_eq!(logluv_inspection_work_limit(0,&Limits{max_input:usize::MAX,..l}),Err(Error::Limit));
    assert_eq!(logluv_inspection_work_limit(0,&Limits{max_output:usize::MAX,..l}),Err(Error::Limit));
    assert_eq!(logluv_inspection_work_limit(0,&Limits{max_input:usize::MAX,max_output:usize::MAX,max_segments:0,..l}),Ok(140));
}
