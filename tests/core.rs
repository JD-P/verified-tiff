use verified_tiff::{inspect, inspection_work_limit, read16, read32, checked_range, AlphaMode, Budget, Error, Limits};

const GRAY: &[u8] = include_bytes!("fixtures/gray-strip.tif");
const EXPECTED: &[u8] = include_bytes!("fixtures/gray-strip.pixels");

#[test]
fn byte_readers_and_overflow_edges() {
    assert_eq!(read16(&[1,2],0,true),Ok(513));
    assert_eq!(read16(&[1,2],0,false),Ok(258));
    assert_eq!(read32(&[255;4],0,true),Ok(u32::MAX));
    assert_eq!(read32(&[1,2,3,4],0,false),Ok(0x01020304));
    assert_eq!(read16(&[1,2],usize::MAX,true),Err(Error::Truncated));
    assert!(!checked_range(usize::MAX,usize::MAX,1));
    assert!(checked_range(usize::MAX,usize::MAX,0));
}

#[test]
fn public_decode_and_tail_frame() {
    let mut b=Budget{remaining:10000};
    let image=inspect(GRAY,0,&Limits::conservative(),&mut b).unwrap();
    let mut out=vec![123;image.output_len()+8];
    let before=b.remaining;
    image.decode_into(&mut out,&mut b).unwrap();
    assert_eq!(&out[..image.output_len()],EXPECTED);
    assert_eq!(&out[image.output_len()..],&[123;8]);
    assert_eq!(before-b.remaining,image.output_len());
}

#[test]
fn insufficient_output_does_not_write_or_spend() {
    let mut b=Budget{remaining:10000};
    let image=inspect(GRAY,0,&Limits::conservative(),&mut b).unwrap();
    let before=b.remaining;let mut out=[123;1];
    assert_eq!(image.decode_into(&mut out,&mut b),Err(Error::OutputTooSmall));
    assert_eq!(out,[123]);assert_eq!(before,b.remaining);
}

#[test]
fn exact_decode_work_boundary() {
    let mut b=Budget{remaining:10000};
    let image=inspect(GRAY,0,&Limits::conservative(),&mut b).unwrap();
    let mut out=vec![123;image.output_len()+5];
    b.remaining=image.output_len()-1;
    assert_eq!(image.decode_into(&mut out,&mut b),Err(Error::Limit));
    assert_eq!(b.remaining,0);
    assert_eq!(&out[..image.output_len()-1],&EXPECTED[..image.output_len()-1]);
    assert!(out[image.output_len()-1..].iter().all(|v|*v==123));
    b.remaining=image.output_len();
    image.decode_into(&mut out,&mut b).unwrap();assert_eq!(b.remaining,0);
    assert_eq!(&out[..image.output_len()],EXPECTED);
    assert_eq!(&out[image.output_len()..],&[123;5]);
}

#[test]
fn borrowed_input_cannot_be_swapped() {
    // The descriptor borrows the original bytes and exposes no field setters.
    let mut b=Budget{remaining:10000};
    let image=inspect(GRAY,0,&Limits::conservative(),&mut b).unwrap();
    assert_eq!((image.width(),image.height(),image.channels()),(7,5,1));
    assert_eq!(image.alpha_mode(),AlphaMode::None);
    assert!(!image.has_alpha());
}

#[test]
fn formal_profile_witness_decodes_with_sufficient_budget() {
    let limits=Limits {max_input:87,max_dimension:1,max_output:1,
        max_directories:1,max_tags:6,max_segments:1};
    let work=inspection_work_limit(0,&limits).unwrap();
    assert_eq!(work,214);
    for bilevel in [false,true] {
        let mut data=*include_bytes!("fixtures/profile-witness.tif");
        if bilevel { data[42]=1; }
        let mut budget=Budget{remaining:work};
        let image=inspect(&data,0,&limits,&mut budget).unwrap();
        assert_eq!(image.bits_per_sample(),if bilevel {1} else {8});
        let mut output=[0];
        image.decode_into(&mut output,&mut Budget{remaining:1}).unwrap();
        assert_eq!(output,if bilevel {[0]} else {[123]});
    }
}

#[test]
fn formal_packbits_witness_decodes_with_sufficient_budget() {
    let limits=Limits {max_input:100,max_dimension:1,max_output:1,
        max_directories:1,max_tags:7,max_segments:1};
    let work=inspection_work_limit(0,&limits).unwrap();
    assert_eq!(work,247);
    for bilevel in [false,true] {
        let mut data=*include_bytes!("fixtures/packbits-profile-witness.tif");
        if bilevel { data[42]=1; }
        let image=inspect(&data,0,&limits,&mut Budget{remaining:work}).unwrap();
        assert_eq!(image.decode_work_limit(),4);
        let mut output=[0];
        image.decode_into(&mut output,&mut Budget{remaining:4}).unwrap();
        assert_eq!(output,if bilevel {[0]} else {[123]});
    }
}

#[test]
fn formal_lzw_witness_decodes_with_sufficient_budget() {
    let limits=Limits {max_input:102,max_dimension:1,max_output:1,
        max_directories:1,max_tags:7,max_segments:1};
    let work=inspection_work_limit(0,&limits).unwrap();
    assert_eq!(work,249);
    for bilevel in [false,true] {
        let mut data=*include_bytes!("fixtures/lzw-profile-witness.tif");
        if bilevel { data[42]=1; }
        let image=inspect(&data,0,&limits,&mut Budget{remaining:work}).unwrap();
        assert_eq!(image.decode_work_limit(),6);
        let mut output=[0];
        image.decode_into(&mut output,&mut Budget{remaining:6}).unwrap();
        assert_eq!(output,if bilevel {[0]} else {[123]});
    }
}

#[test]
fn lzw_tail_witness_budget_and_frame() {
    let limits=Limits {max_input:101,max_dimension:1,max_output:1,
        max_directories:1,max_tags:7,max_segments:1};
    let work=inspection_work_limit(0,&limits).unwrap();
    assert_eq!(work,248);
    for bilevel in [false,true] {
        let mut data=*include_bytes!("fixtures/lzw-tail-witness.tif");
        assert_eq!(&data[98..],&[128,30,224]); // Same stream as the formal witness.
        if bilevel {data[42]=1;}
        let image=inspect(&data,0,&limits,&mut Budget{remaining:work}).unwrap();
        assert_eq!(image.decode_work_limit(),5);
        let mut output=[0xa5;5];let mut budget=Budget{remaining:4};
        assert_eq!(image.decode_into(&mut output,&mut budget),Err(Error::Limit));
        assert_eq!(output,[0xa5;5]);assert_eq!(budget.remaining,4);
        budget.remaining=5;
        assert_eq!(image.decode_into(&mut [],&mut budget),Err(Error::OutputTooSmall));
        assert_eq!(budget.remaining,5);
        image.decode_into(&mut output,&mut budget).unwrap();
        assert_eq!(budget.remaining,0);assert_eq!(output[0],if bilevel {0} else {123});
        assert_eq!(&output[1..],&[0xa5;4]);
    }
}

#[test]
fn legacy_lzw_witness_and_full_dictionary_frames() {
    let limits=Limits {max_input:102,max_dimension:1,max_output:1,
        max_directories:1,max_tags:7,max_segments:1};
    let work=inspection_work_limit(0,&limits).unwrap();
    assert_eq!(work,249);
    for bilevel in [false,true] {
        let mut data=*include_bytes!("fixtures/lzw-legacy-witness.tif");
        assert_eq!(&data[98..],&[0,247,4,4]);
        if bilevel {data[42]=1;}
        let image=inspect(&data,0,&limits,&mut Budget{remaining:work}).unwrap();
        assert_eq!(image.decode_work_limit(),6);
        let mut output=[0xa5;5];let mut budget=Budget{remaining:5};
        assert_eq!(image.decode_into(&mut output,&mut budget),Err(Error::Limit));
        assert_eq!(output,[0xa5;5]);assert_eq!(budget.remaining,5);
        budget.remaining=6;
        assert_eq!(image.decode_into(&mut [],&mut budget),Err(Error::OutputTooSmall));
        assert_eq!(budget.remaining,6);
        image.decode_into(&mut output,&mut budget).unwrap();
        assert_eq!(budget.remaining,0);assert_eq!(output[0],if bilevel {0} else {123});
        assert_eq!(&output[1..],&[0xa5;4]);
    }
    let data=include_bytes!("fixtures/lzw-legacy-full.tif");
    let expected=include_bytes!("fixtures/lzw-legacy-full.pixels");
    let image=inspect(data,0,&Limits::conservative(),&mut Budget{remaining:1_000_000}).unwrap();
    let mut output=vec![0xa5;expected.len()+7];
    let mut budget=Budget{remaining:image.decode_work_limit()};
    image.decode_into(&mut output,&mut budget).unwrap();
    assert_eq!(&output[..expected.len()],expected);assert_eq!(&output[expected.len()..],&[0xa5;7]);
    assert_eq!(budget.remaining,0);
}

#[test]
fn sufficient_budget_calculator_handles_overflow() {
    let limits=Limits::conservative();
    let expected=1024usize*4097+19*4096+4+1048576*(67108864+1)+67108864;
    assert_eq!(inspection_work_limit(1023,&limits),Ok(expected));
    assert_eq!(inspection_work_limit(usize::MAX,&limits),Err(Error::Limit));
    let mut huge=limits;huge.max_tags=usize::MAX;
    assert_eq!(inspection_work_limit(0,&huge),Err(Error::Limit));
    huge=limits;huge.max_output=usize::MAX;
    assert_eq!(inspection_work_limit(0,&huge),Err(Error::Limit));
}

#[test]
fn packbits_output_budget_and_frame() {
    let cases: [(&[u8], &[u8], usize); 3] = [
        (include_bytes!("fixtures/packbits-bilevel-tile.tif"), include_bytes!("fixtures/packbits-bilevel-tile.pixels"),1),
        (include_bytes!("fixtures/packbits-gray-strip.tif"), include_bytes!("fixtures/packbits-gray-strip.pixels"),8),
        (include_bytes!("fixtures/packbits-rgb-tile.tif"), include_bytes!("fixtures/packbits-rgb-tile.pixels"),8),
    ];
    for (data,expected,bits) in cases {
        let limits=Limits::conservative();
        let mut inspection=Budget{remaining:inspection_work_limit(0,&limits).unwrap()};
        let image=inspect(data,0,&limits,&mut inspection).unwrap();
        assert_eq!(image.compression(),32773);
        assert_eq!(image.bits_per_sample(),bits);
        assert!(image.decode_work_limit()>image.output_len());
        let mut output=vec![123;image.output_len()+8];
        let mut work=Budget{remaining:image.decode_work_limit()-1};
        assert_eq!(image.decode_into(&mut output,&mut work),Err(Error::Limit));
        assert_eq!(work.remaining,image.decode_work_limit()-1);
        assert!(output.iter().all(|v|*v==123));
        work.remaining=image.decode_work_limit();
        let mut short=vec![123;image.output_len()-1];
        assert_eq!(image.decode_into(&mut short,&mut work),Err(Error::OutputTooSmall));
        assert_eq!(work.remaining,image.decode_work_limit());
        assert!(short.iter().all(|v|*v==123));
        image.decode_into(&mut output,&mut work).unwrap();
        assert_eq!(work.remaining,0);
        assert_eq!(&output[..image.output_len()],expected);
        assert_eq!(&output[image.output_len()..],&[123;8]);
    }
}

#[test]
fn lzw_output_budget_and_frame() {
    let cases: [(&[u8], &[u8], usize); 3] = [
        (include_bytes!("fixtures/lzw-bilevel-tile.tif"), include_bytes!("fixtures/lzw-bilevel-tile.pixels"),1),
        (include_bytes!("fixtures/lzw-gray-strip.tif"), include_bytes!("fixtures/lzw-gray-strip.pixels"),8),
        (include_bytes!("fixtures/lzw-rgb-tile.tif"), include_bytes!("fixtures/lzw-rgb-tile.pixels"),8),
    ];
    for (data,expected,bits) in cases {
        let limits=Limits::conservative();
        let mut inspection=Budget{remaining:inspection_work_limit(0,&limits).unwrap()};
        let image=inspect(data,0,&limits,&mut inspection).unwrap();
        assert_eq!(image.compression(),5);
        assert_eq!(image.bits_per_sample(),bits);
        assert!(image.decode_work_limit()>image.output_len());
        let mut output=vec![123;image.output_len()+8];
        let mut work=Budget{remaining:image.decode_work_limit()-1};
        assert_eq!(image.decode_into(&mut output,&mut work),Err(Error::Limit));
        assert_eq!(work.remaining,image.decode_work_limit()-1);
        assert!(output.iter().all(|v|*v==123));
        work.remaining=image.decode_work_limit();
        let mut short=vec![123;image.output_len()-1];
        assert_eq!(image.decode_into(&mut short,&mut work),Err(Error::OutputTooSmall));
        assert_eq!(work.remaining,image.decode_work_limit());
        assert!(short.iter().all(|v|*v==123));
        image.decode_into(&mut output,&mut work).unwrap();
        assert_eq!(work.remaining,0);
        assert_eq!(&output[..image.output_len()],expected);
        assert_eq!(&output[image.output_len()..],&[123;8]);
    }
}

#[test]
fn bilevel_expansion_budget_and_frame() {
    let cases: [(&[u8], &[u8]); 2] = [
        (include_bytes!("fixtures/bilevel-strip.tif"), include_bytes!("fixtures/bilevel-strip.pixels")),
        (include_bytes!("fixtures/bilevel-lsb-tile.tif"), include_bytes!("fixtures/bilevel-lsb-tile.pixels")),
    ];
    for (data,expected) in cases {
        let limits=Limits::conservative();
        let mut inspection=Budget{remaining:inspection_work_limit(0,&limits).unwrap()};
        let image=inspect(data,0,&limits,&mut inspection).unwrap();
        assert_eq!(image.channels(),1);
        assert_eq!(image.bits_per_sample(),1);
        assert_eq!(image.output_len(),image.width()*image.height());
        let mut short=vec![123;image.output_len()-1];
        let mut work=Budget{remaining:image.output_len()};
        assert_eq!(image.decode_into(&mut short,&mut work),Err(Error::OutputTooSmall));
        assert!(short.iter().all(|v|*v==123));
        assert_eq!(work.remaining,image.output_len());
        let mut output=vec![123;image.output_len()+8];
        image.decode_into(&mut output,&mut work).unwrap();
        assert_eq!(work.remaining,0);
        assert_eq!(&output[..image.output_len()],expected);
        assert_eq!(&output[image.output_len()..],&[123;8]);
    }
}

#[test]
fn alpha_output_budget_and_frame() {
    let cases: [(&[u8], &[u8], usize, AlphaMode, usize); 12] = [
        (include_bytes!("fixtures/alpha-2-1-none.tif"), include_bytes!("fixtures/alpha-2-1-none.pixels"),2,AlphaMode::Associated,1),
        (include_bytes!("fixtures/alpha-2-1-packbits.tif"), include_bytes!("fixtures/alpha-2-1-packbits.pixels"),2,AlphaMode::Associated,32773),
        (include_bytes!("fixtures/alpha-2-1-lzw.tif"), include_bytes!("fixtures/alpha-2-1-lzw.pixels"),2,AlphaMode::Associated,5),
        (include_bytes!("fixtures/alpha-2-2-none.tif"), include_bytes!("fixtures/alpha-2-2-none.pixels"),2,AlphaMode::Unassociated,1),
        (include_bytes!("fixtures/alpha-2-2-packbits.tif"), include_bytes!("fixtures/alpha-2-2-packbits.pixels"),2,AlphaMode::Unassociated,32773),
        (include_bytes!("fixtures/alpha-2-2-lzw.tif"), include_bytes!("fixtures/alpha-2-2-lzw.pixels"),2,AlphaMode::Unassociated,5),
        (include_bytes!("fixtures/alpha-4-1-none.tif"), include_bytes!("fixtures/alpha-4-1-none.pixels"),4,AlphaMode::Associated,1),
        (include_bytes!("fixtures/alpha-4-1-packbits.tif"), include_bytes!("fixtures/alpha-4-1-packbits.pixels"),4,AlphaMode::Associated,32773),
        (include_bytes!("fixtures/alpha-4-1-lzw.tif"), include_bytes!("fixtures/alpha-4-1-lzw.pixels"),4,AlphaMode::Associated,5),
        (include_bytes!("fixtures/alpha-4-2-none.tif"), include_bytes!("fixtures/alpha-4-2-none.pixels"),4,AlphaMode::Unassociated,1),
        (include_bytes!("fixtures/alpha-4-2-packbits.tif"), include_bytes!("fixtures/alpha-4-2-packbits.pixels"),4,AlphaMode::Unassociated,32773),
        (include_bytes!("fixtures/alpha-4-2-lzw.tif"), include_bytes!("fixtures/alpha-4-2-lzw.pixels"),4,AlphaMode::Unassociated,5),
    ];
    for (data,expected,ch,mode,codec) in cases {
        let limits=Limits::conservative();
        let image=inspect(data,0,&limits,&mut Budget{remaining:inspection_work_limit(0,&limits).unwrap()}).unwrap();
        assert!(image.has_alpha());assert_eq!(image.alpha_mode(),mode);
        assert_eq!(image.channels(),ch);assert_eq!(image.compression(),codec);
        assert_eq!(image.bits_per_sample(),8);
        assert!(image.decode_work_limit()>=2*image.output_len());
        if codec==1 {assert_eq!(image.decode_work_limit(),2*image.output_len());}
        let mut output=vec![123;image.output_len()+8];
        let mut work=Budget{remaining:image.decode_work_limit()-1};
        assert_eq!(image.decode_into(&mut output,&mut work),Err(Error::Limit));
        assert_eq!(work.remaining,image.decode_work_limit()-1);
        assert!(output.iter().all(|v|*v==123));
        work.remaining=image.decode_work_limit();
        let mut short=vec![123;image.output_len()-1];
        assert_eq!(image.decode_into(&mut short,&mut work),Err(Error::OutputTooSmall));
        assert_eq!(work.remaining,image.decode_work_limit());
        assert!(short.iter().all(|v|*v==123));
        image.decode_into(&mut output,&mut work).unwrap();
        assert_eq!(work.remaining,0);
        assert_eq!(&output[..image.output_len()],expected);
        assert_eq!(&output[image.output_len()..],&[123;8]);
    }
}

#[test]
fn formal_alpha_witness_decodes_with_sufficient_budget() {
    let limits=Limits {max_input:122,max_dimension:1,max_output:4,
        max_directories:1,max_tags:8,max_segments:1};
    let quota=inspection_work_limit(0,&limits).unwrap();assert_eq!(quota,292);
    for rgb in [false,true] {
        for associated in [false,true] {
            let mut data=*include_bytes!("fixtures/alpha-profile-witness.tif");
            if !rgb {
                data[38]=2;data[42]=8;data[44]=8;data[54]=1;
                data[78]=2;data[90]=2;data[119]=128;
            }
            data[102]=if associated {1} else {2};
            let image=inspect(&data,0,&limits,&mut Budget{remaining:quota}).unwrap();
            assert_eq!(image.alpha_mode(),if associated {AlphaMode::Associated} else {AlphaMode::Unassociated});
            let mut output=vec![0;image.output_len()];
            image.decode_into(&mut output,&mut Budget{remaining:2*image.output_len()}).unwrap();
            let expected=match (rgb,associated) {
                (false,false)=>vec![32,128],(false,true)=>vec![64,128],
                (true,false)=>vec![32,64,96,128],(true,true)=>vec![64,128,191,128],
            };
            assert_eq!(output,expected);
        }
    }
}
