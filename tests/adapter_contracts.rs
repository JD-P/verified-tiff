use verified_tiff::{adapter_meta as meta,adapter_bounds as bounds,adapter_geometry as geometry,adapter_window as window,adapter_color as color};

#[test]
fn ink_names_exact_payload() {
    for s in [b"".as_slice(),b"cyan",b"cyan\0magenta"] {assert!(meta::ink_names(s).is_err());}
    for (s,n) in [(b"\0".as_slice(),1),(b"cyan\0magenta\0",2),(b"\0\0x\0",3)] {
        assert_eq!(meta::ink_names(s).unwrap(),n);
    }
    assert_eq!(meta::ink_names(&vec![0;65535]).unwrap(),65535);
    assert!(meta::ink_names(&vec![0;65536]).is_err());
    // Exhaust all short byte strings against an independent iterator model.
    for len in 0..10 {for bits in 0..1usize<<len {
        let s:Vec<u8>=(0..len).map(|i|if bits>>i&1==0{0}else{b'x'}).collect();
        let got=meta::ink_names(&s);
        assert_eq!(got.is_ok(),s.last()==Some(&0));
        if let Ok(n)=got {assert_eq!(usize::from(n),s.iter().filter(|&&v|v==0).count());}
    }}
}

#[test]
fn metadata_integer_conversions_match_wide_reference(){
    let kinds=[1u16,2,3,4,6,7,8,9,13,16,17,18];
    let values=[0,1,127,128,255,256,32767,32768,65535,65536,2147483647,2147483648,4294967295,4294967296,i64::MAX as u64,1<<63,u64::MAX];
    for source in kinds {for target in kinds {for v in values {
        let n=match source{6=>v as i8 as i128,8=>v as i16 as i128,9=>v as i32 as i128,17=>v as i64 as i128,_=>v as i128};
        let (lo,hi)=match target{6=>(-128,127),8=>(-32768,32767),9=>(i32::MIN as i128,i32::MAX as i128),17=>(i64::MIN as i128,i64::MAX as i128),1|2|7=>(0,255),3=>(0,65535),4|13=>(0,u32::MAX as i128),_=>(0,u64::MAX as i128)};
        let actual=meta::convert_integer(v,source,target);
        assert_eq!(actual.is_ok(),(lo..=hi).contains(&n));
        if let Ok(actual)=actual{assert_eq!(actual,n as u64);}
    }}}
    for invalid in [0,5,10,11,12,14,15,65535]{assert!(meta::convert_integer(1,invalid,4).is_err());assert!(meta::convert_integer(1,4,invalid).is_err());}
}
#[test]
fn schemas_do_not_change_argument_shape_with_stored_count(){
    assert_eq!(meta::get_plan(3,1,false,0,false,1),1);
    assert_eq!(meta::get_plan(3,1,false,0,false,2),0);
    assert_eq!(meta::get_plan(3,-1,true,2,true,65535),3);
    assert_eq!(meta::get_plan(3,-1,true,2,true,65536),0);
    assert_eq!(meta::get_plan(12,-3,true,4,true,u32::MAX as usize),4);
    assert!(!meta::registration(3,-3,-1,true));
    assert!(meta::registration(3,-3,-3,true));
    assert!(meta::payload(17,1,2,12).is_ok());
    assert!(meta::payload(16,1,2,12).is_err());
    assert!(meta::payload(usize::MAX,0,usize::MAX,12).is_err());
}
#[test]
fn crops_match_coordinate_reference_and_do_not_touch_padding(){
    for w in 1..9 {for h in 1..9 {for vw in 1..=w {for vh in 1..=h {
        for fx in [false,true]{for fy in [false,true]{for y in 0..vh {for x in 0..vw {for n in 1..=vw-x {
            let (at,bytes)=geometry::crop_span(w,h,vw,vh,x,y,n,fx,fy).unwrap();
            let mut reference=Vec::new();
            for ix in x..x+n {let dx=if fx{vw-1-ix}else{ix};let dy=if fy{vh-1-y}else{y};reference.push(((h-vh+dy)*w+dx)*4);}
            reference.sort();assert_eq!(at,reference[0]);assert_eq!(at+bytes,reference.last().unwrap()+4);
            assert!(at+bytes<=w*h*4);
        }}}}}
    }}}}
    assert!(geometry::crop_span(usize::MAX,2,1,1,0,0,1,false,false).is_err());
    assert!(geometry::crop_span(8,8,8,8,8,0,1,false,false).is_err());
}
#[test]
fn windows_preserve_exact_unread_suffix_and_source_position(){
    let source:Vec<u8>=(0..1009).map(|i|(i*73) as u8).collect();
    for cap in 1..41 {
        let mut buffer=vec![0xa5;cap];let(mut len,mut at,mut loaded)=(0,0,0);let mut seen=Vec::new();
        loop{
            let (rest,n,next)=window::refill_plan(cap,len,at,loaded,source.len()).unwrap();
            let before=buffer.clone();window::compact(&mut buffer,at,len).unwrap();
            assert_eq!(&buffer[..rest],&before[at..len]);
            buffer[rest..rest+n].copy_from_slice(&source[loaded..next]);loaded=next;len=rest+n;at=0;
            let take=len.min(1+loaded%7);seen.extend_from_slice(&buffer[..take]);at+=take;
            assert_eq!(loaded-len+at,seen.len());
            if seen.len()==source.len(){break;}
        }
        assert_eq!(seen,source);
    }
    let mut b=[4,5,6];assert!(window::compact(&mut b,2,1).is_err());assert_eq!(b,[4,5,6]);
}
#[test]
fn allocation_and_row_rounding_boundaries_fail_closed(){
    assert_eq!(bounds::claim(60,40,40,100).unwrap(),100);
    assert!(bounds::claim(60,41,40,100).is_err());assert!(bounds::claim(usize::MAX,1,0,0).is_err());
    assert!(bounds::release(1,2).is_err());assert_eq!(bounds::release(100,40).unwrap(),60);
    assert!(bounds::allocation(usize::MAX,8).is_err());
    for row in 1..33 {let total=row*19;for at in 0..=total {for n in 0..=total-at {
        let (first,last)=bounds::row_range(total,row,at,n).unwrap();assert_eq!(first,at/row*row);assert_eq!(last,(at+n).div_ceil(row)*row);
    }}}
    let total=usize::MAX-7;assert_eq!(bounds::row_range(total,8,total-1,1).unwrap(),(total-8,total));
    assert!(bounds::source_offset(u64::MAX,1,0,u64::MAX).is_err());
}
#[test]
fn float_index_matches_hardware_saturating_cast(){
    let mut bits=17u32;
    for _ in 0..200_000 {bits=bits.wrapping_mul(1664525).wrapping_add(1013904223);for cap in [0,255,1500,u32::MAX]{assert_eq!(color::f32_index(bits,cap),(f32::from_bits(bits) as u32).min(cap),"{bits:08x}");}}
    for bits in [0,1,0x7fffff,0x800000,0x3f800000,0x4f7fffff,0x4f800000,0x7f800000,0x7fc00000,0xff800000,0xffffffff]{assert_eq!(color::f32_index(bits,u32::MAX),f32::from_bits(bits) as u32);}
    assert_eq!(color::ycbcr_tables(i32::MAX,i32::MAX,i32::MIN,i32::MAX,i32::MAX),[255,255,0]);
}
#[test]
fn output_commit_has_exact_frames_and_failure_atomicity(){
    let mut out=[0xa5;19];bounds::commit(&[1,2,3],&mut out,7).unwrap();
    assert_eq!(&out[..7],&[0xa5;7]);assert_eq!(&out[7..10],&[1,2,3]);assert_eq!(&out[10..],&[0xa5;9]);
    let before=out;assert!(bounds::commit(&[9,9],&mut out,18).is_err());assert_eq!(out,before);
}

#[test]
fn output_commit_initializes_uninitialized_c_storage(){
    let mut out=[std::mem::MaybeUninit::uninit();11];
    bounds::commit_uninit(&[2,3,5,7],&mut out).unwrap();
    for (i,v) in [2,3,5,7].into_iter().enumerate(){assert_eq!(unsafe{out[i].assume_init()},v);}
    assert!(bounds::commit_uninit(&[0;12],&mut out).is_err());
}
