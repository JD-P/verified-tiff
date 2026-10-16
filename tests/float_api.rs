use verified_tiff::{inspect_extended,inspect_raster,extended_inspection_work_limit,raster_inspection_work_limit,SampleFormat,AlphaMode,Budget,Error,Limits};
const CASES:&[(&[u8],&[u8],usize)]=&[
    (include_bytes!("fixtures/acceptance-float64-plain.tif"),include_bytes!("fixtures/acceptance-float64-plain.pixels"),1),
    (include_bytes!("fixtures/acceptance-float64-predict2.tif"),include_bytes!("fixtures/acceptance-float64-predict2.pixels"),2),
    (include_bytes!("fixtures/float64-gray-be.tif"),include_bytes!("fixtures/float64-gray-be.pixels"),2),
    (include_bytes!("fixtures/float64-planar.tif"),include_bytes!("fixtures/float64-planar.pixels"),2),
    (include_bytes!("fixtures/float64-none.tif"),include_bytes!("fixtures/float64-none.pixels"),1),
    (include_bytes!("fixtures/float64-packbits.tif"),include_bytes!("fixtures/float64-packbits.pixels"),1),
];
#[test]
fn float64_pixels_metadata_budgets_and_frames() {
    let limits=Limits::conservative();
    for &(source,expected,predictor) in CASES {
        let image=inspect_raster(source,0,&limits,&mut Budget {remaining:raster_inspection_work_limit(0,&limits).unwrap()}).unwrap();
        assert_eq!(image.sample_format(),SampleFormat::IeeeFloat);
        assert_eq!(image.predictor(),predictor);assert_eq!(image.bits_per_sample(),64);assert_eq!(image.output_bits_per_sample(),64);
        assert_eq!(image.alpha_mode(),AlphaMode::None);assert!(!image.has_alpha());
        assert_eq!(image.output_len(),expected.len());assert_eq!(image.scratch_len(),expected.len());
        let mut output=vec![0xa5;image.output_len()+11];let mut scratch=vec![0x5a;image.scratch_len()+13];
        let quota=image.decode_work_limit();let mut low=Budget {remaining:quota-1};
        assert_eq!(image.decode_with_scratch(&mut output,&mut scratch,&mut low),Err(Error::Limit));
        assert_eq!(low.remaining,quota-1);assert!(output.iter().all(|v|*v==0xa5));assert!(scratch.iter().all(|v|*v==0x5a));
        for (o,s) in [(image.output_len()-1,image.scratch_len()),(image.output_len(),image.scratch_len()-1)] {
            let mut b=Budget {remaining:quota};
            assert_eq!(image.decode_with_scratch(&mut output[..o],&mut scratch[..s],&mut b),Err(Error::OutputTooSmall));
            assert_eq!(b.remaining,quota);assert!(output.iter().all(|v|*v==0xa5));assert!(scratch.iter().all(|v|*v==0x5a));
        }
        let mut work=Budget {remaining:quota};image.decode_with_scratch(&mut output,&mut scratch,&mut work).unwrap();
        assert_eq!(work.remaining,0);assert_eq!(&output[..expected.len()],expected);
        assert!(output[expected.len()..].iter().all(|v|*v==0xa5));assert!(scratch[expected.len()..].iter().all(|v|*v==0x5a));
    }
}
#[test]
fn float64_formal_witness_budgets_and_modular_overflow() {
    let limits=Limits {max_input:160,max_dimension:2,max_output:16,max_directories:1,max_tags:9,max_segments:1};
    assert_eq!(extended_inspection_work_limit(0,&limits),Ok(5702));
    for &(source,expected,predictor) in &CASES[..2] {
        let image=inspect_extended(source,0,&limits,&mut Budget {remaining:5702}).unwrap();
        assert!(image.is_floating_point());assert_eq!(image.predictor(),predictor);
        let mut output=[0u8;16];let mut scratch=[0u8;16];
        assert_eq!(image.decode_work_limit(),86);
        image.decode_with_scratch(&mut output,&mut scratch,&mut Budget {remaining:86}).unwrap();
        assert_eq!(output,expected);assert_eq!(u64::from_le_bytes(output[..8].try_into().unwrap()),u64::MAX);
        assert_eq!(u64::from_le_bytes(output[8..].try_into().unwrap()),if predictor==2 {1} else {2});
    }
}
#[test]
fn numeric_format_distinguishes_existing_integer_outputs() {
    for source in [include_bytes!("fixtures/rgb16.tif").as_slice(),include_bytes!("fixtures/gray-strip.tif").as_slice(),include_bytes!("fixtures/jpeg-gray.tif").as_slice(),include_bytes!("fixtures/fax-g3-witness.tif").as_slice()] {
        let image=inspect_raster(source,0,&Limits::conservative(),&mut Budget {remaining:1_000_000}).unwrap();
        assert_eq!(image.sample_format(),SampleFormat::UnsignedInteger);assert_eq!(image.predictor(),1);
    }
}
