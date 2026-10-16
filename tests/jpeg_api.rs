use verified_tiff::{inspect_jpeg_tiff, inspect_raster, jpeg_inspection_work_limit, raster_inspection_work_limit, Budget, Error, Limits};
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
