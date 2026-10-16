use verified_tiff::{inspect_fax_tiff, inspect_raster, fax_inspection_work_limit, raster_inspection_work_limit, Budget, Error, Limits};
const CASES:&[(&[u8],&[u8])]=&[
(include_bytes!("fixtures/fax-g3-witness.tif"),include_bytes!("fixtures/fax-g3-witness.pixels")),
(include_bytes!("fixtures/fax-g4-witness.tif"),include_bytes!("fixtures/fax-g4-witness.pixels")),
(include_bytes!("fixtures/fax-g3.tif"),include_bytes!("fixtures/fax-g3.pixels")),
(include_bytes!("fixtures/fax-mixed.tif"),include_bytes!("fixtures/fax-mixed.pixels")),
(include_bytes!("fixtures/fax-g4-tile.tif"),include_bytes!("fixtures/fax-g4-tile.pixels")),
(include_bytes!("fixtures/fax-lsb.tif"),include_bytes!("fixtures/fax-lsb.pixels")),
(include_bytes!("fixtures/fax-noeol.tif"),include_bytes!("fixtures/fax-noeol.pixels")),
];
#[test]
fn fax_pixels_exact_quotas_and_buffer_frames() {
    for &(source,expected) in CASES {
        let lim=Limits::conservative();
        let mut ib=Budget{remaining:fax_inspection_work_limit(0,&lim).unwrap()};
        let direct=inspect_fax_tiff(source,0,&lim,&mut ib).unwrap();
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
        assert_eq!(&out[..expected.len()],expected);
        assert!(out[expected.len()..].iter().all(|v|*v==0xa5));assert!(scratch[image.scratch_len()..].iter().all(|v|*v==0x5a));
    }
}
#[test]
fn fax_budget_formula_and_overflow() {
    let l=Limits{max_input:101,max_output:1,max_dimension:1,max_tags:7,max_segments:1,max_directories:1};
    assert_eq!(fax_inspection_work_limit(0,&l),Ok(6669));
    assert_eq!(fax_inspection_work_limit(usize::MAX,&l),Err(Error::Limit));
    assert_eq!(fax_inspection_work_limit(0,&Limits{max_input:usize::MAX,..l}),Err(Error::Limit));
    assert_eq!(fax_inspection_work_limit(0,&Limits{max_output:usize::MAX,max_input:usize::MAX,max_segments:0,..l}),Ok(136));
}
#[test]
fn concrete_fax_witnesses_are_exact() {
    for &(source,expected) in &CASES[..2] {
        let l=Limits{max_input:101,max_output:1,max_dimension:1,max_tags:7,max_segments:1,max_directories:1};
        let mut budget=Budget{remaining:fax_inspection_work_limit(0,&l).unwrap()};
        let image=inspect_fax_tiff(source,0,&l,&mut budget).unwrap();
        let mut out=[0];let mut scratch=[0];let mut work=Budget{remaining:image.decode_work_limit()};
        image.decode_with_scratch(&mut out,&mut scratch,&mut work).unwrap();assert_eq!(&out,expected);assert_eq!(work.remaining,0);
    }
}
