use verified_tiff::{container,native,native_samples as ns,Budget,Error};
#[test]
fn floating_predictor3_significance_planes_rows_channels_and_frames(){
    for bytes in [4,8]{for ch in [1,3,4,64]{for out_le in [false,true]{
        let row=7*ch*bytes;let mut raw=Vec::new();let mut expected=Vec::new();
        for y in 0..3{
            let words:Vec<u64>=(0..7*ch).map(|i|[0,1,0x8000000000000000,0x7ff812345678abcd,u64::MAX][(i+y)%5]).collect();
            let shuffled:Vec<u8>=(0..bytes).flat_map(|j|words.iter().map(move |v|(v>>(8*(bytes-1-j))) as u8)).collect();
            for (i,v) in shuffled.iter().enumerate(){raw.push(v.wrapping_sub(if i>=ch{shuffled[i-ch]}else{0}));}
            for v in words{for j in 0..bytes{expected.push((v>>(8*if out_le{j}else{bytes-1-j})) as u8);}}
        }
        let mut scratch=vec![0x5a;raw.len()+11];let mut out=vec![0xa5;raw.len()+17];
        verified_tiff::native_fp::decode(&raw,row,ch,bytes,out_le,&mut scratch,&mut out).unwrap();
        assert_eq!(&out[..raw.len()],expected);assert!(out[raw.len()..].iter().all(|v|*v==0xa5));assert!(scratch[raw.len()..].iter().all(|v|*v==0x5a));
        for (badrow,badch,badbytes) in [(0,ch,bytes),(row-1,ch,bytes),(row,0,bytes),(row,65,bytes),(row,ch,3)]{
            out.fill(0xa5);assert!(verified_tiff::native_fp::decode(&raw,badrow,badch,badbytes,out_le,&mut scratch,&mut out).is_err());assert!(out.iter().all(|v|*v==0xa5));
        }
    }}}
}
#[test]
fn ccitt_rle_row_alignment_fill_order_work_and_errors(){
    let source=[0x98u8,0x35,0x14];let fee=64*source.len()+4*16+64*2+1;
    for lsb in [false,true]{
        let s=source.map(|v|if lsb{v.reverse_bits()}else{v});let mut out=[0xa5;21];let mut b=Budget{remaining:fee-1};
        assert_eq!(verified_tiff::native_rle::decode(&s,8,2,lsb,&mut out,&mut b),Err(Error::Limit));assert_eq!(out,[0xa5;21]);
        b.remaining=fee;verified_tiff::native_rle::decode(&s,8,2,lsb,&mut out,&mut b).unwrap();assert_eq!(b.remaining,0);
        assert_eq!(&out[..8],&[0;8]);assert_eq!(&out[8..16],&[255;8]);assert_eq!(&out[16..],&[0xa5;5]);
        for n in 0..s.len(){out.fill(0xa5);assert!(verified_tiff::native_rle::decode(&s[..n],8,2,lsb,&mut out,&mut Budget{remaining:fee}).is_err());assert_eq!(&out[16..],&[0xa5;5]);}
    }
    for malformed in [vec![0x99,0x35,0x14],vec![0x98,0x35,0x14,0]]{
        assert!(verified_tiff::native_rle::decode(&malformed,8,2,false,&mut [0;16],&mut Budget{remaining:10000}).is_err());
    }
}
#[test]
fn exact_native_words_prediction_rows_channels_and_frames(){
    for bytes in [1,2,4,8]{for ch in [1,2,3,4,8,64]{for le in [false,true]{for out_le in [false,true]{
        let max=if bytes==8{u64::MAX}else{(1u64<<(8*bytes))-1};
        let mut source=Vec::new();let mut expected=Vec::new();
        for y in 0..3{let mut previous=vec![0u64;ch];for x in 0..7{for c in 0..ch{
            let v=(if x%2==0{max}else{0}).wrapping_add((y*57+x*373+c*997) as u64)&max;
            let delta=v.wrapping_sub(previous[c])&max;previous[c]=v;
            for j in 0..bytes{source.push((delta>>(8*if le{j}else{bytes-1-j})) as u8);expected.push((v>>(8*if out_le{j}else{bytes-1-j})) as u8);}
        }}}
        let mut out=vec![0xa5;source.len()+17];ns::convert(&source,7*ch*bytes,ch,bytes,le,true,out_le,&mut out).unwrap();
        assert_eq!(&out[..source.len()],expected);assert!(out[source.len()..].iter().all(|v|*v==0xa5));
        let mut plain=vec![0;source.len()];ns::convert(&expected,7*ch*bytes,ch,bytes,out_le,false,out_le,&mut plain).unwrap();assert_eq!(plain,expected);
    }}}}
}
#[test]
fn native_stream_acceptance_budget_and_failure_frames(){
    for (codec,input) in [(1,vec![65,66,67]),(32773,vec![2,65,66,67])]{
        let fee=native::work_limit(input.len(),3).unwrap();let mut out=[0xa5;7];let mut b=Budget{remaining:fee-1};
        assert_eq!(native::decode(&input,codec,3,3,3,&mut out,&mut b),Err(Error::Limit));assert_eq!(out,[0xa5;7]);
        b.remaining=fee;native::decode(&input,codec,3,3,3,&mut out,&mut b).unwrap();assert_eq!(&out[..3],b"ABC");assert_eq!(b.remaining,0);assert_eq!(&out[3..],&[0xa5;4]);
    }
    for s in [&[2u8,65,66][..],&[3,65,66,67,68][..]]{
        let mut out=[0xa5;7];assert!(native::decode(s,32773,3,3,3,&mut out,&mut Budget{remaining:1000}).is_err());assert_eq!(&out[3..],&[0xa5;4]);
    }
    assert!(native::work_limit(usize::MAX,1).is_err());
    let mut out=[0xa5;8];for (row,ch,bytes) in [(0,1,1),(3,2,1),(4,65,1),(4,1,3),(3,1,1)]{
        assert!(ns::convert(&[0;4],row,ch,bytes,true,true,true,&mut out).is_err());assert_eq!(out,[0xa5;8]);
    }
    let mut all:Vec<u8>=(0..=255).collect();ns::reverse_bits(&mut all);for i in 0..256{assert_eq!(all[i],(i as u8).reverse_bits());}
}
#[test]
fn bigtiff_64bit_ranges_and_header_acceptance(){
    for le in [false,true]{
        let mut s=[0u8;20];s[..2].copy_from_slice(if le{b"II"}else{b"MM"});
        let put16=|s:&mut[u8],at:usize,v:u16|s[at..at+2].copy_from_slice(&if le{v.to_le_bytes()}else{v.to_be_bytes()});
        let put64=|s:&mut[u8],at:usize,v:u64|s[at..at+8].copy_from_slice(&if le{v.to_le_bytes()}else{v.to_be_bytes()});
        put16(&mut s,2,43);put16(&mut s,4,8);put64(&mut s,8,(1u64<<40)+17);
        let h=container::header(&s).unwrap();assert!(h.big);assert_eq!(h.first_ifd,(1u64<<40)+17);assert_eq!(h.little_endian,le);
        s[6]=1;assert!(container::header(&s).is_err());s[6]=0;
        put16(&mut s,0,273);put16(&mut s,2,16);put64(&mut s,4,2);put64(&mut s,12,1u64<<40);
        let e=container::entry(&s,1u64<<32,true,le,(1u64<<40)+16).unwrap();assert_eq!(e.offset,1u64<<40);assert_eq!(e.length,16);assert!(!e.inline);
        assert!(container::entry(&s,1u64<<32,true,le,(1u64<<40)+15).is_err());
        put64(&mut s,4,u64::MAX);assert!(container::entry(&s,16,true,le,u64::MAX).is_err());
        put64(&mut s,4,1);let e=container::entry(&s,1u64<<40,true,le,(1u64<<40)+20).unwrap();assert!(e.inline);assert_eq!(e.offset,(1u64<<40)+12);
    }
}
#[test]
fn ojpeg_scan_relabeling_preserves_entropy_and_checks_source(){
    let raw=[255,218,0,12,3,1,0,2,17,3,17,0,63,0,10,20,255,0,30];
    let mut renamed=raw;native::renumber_ojpeg_scan(&mut renamed,3,[1,2,3],[0,17,17]).unwrap();
    assert_eq!(&renamed[..14],&[255,218,0,12,3,0,0,1,17,2,34,0,63,0]);assert_eq!(&renamed[14..],&raw[14..]);
    let mut bad=raw;assert!(native::renumber_ojpeg_scan(&mut bad,3,[0,1,2],[0,17,17]).is_err());assert_eq!(bad,raw);
}

#[test]
fn component_permutation_and_failure_frames(){
    for width in 1..=8{
        let raw:Vec<u8>=(0..width*19).map(|i|(i*73) as u8).collect();
        let expected:Vec<u8>=raw.chunks_exact(width).flat_map(|c|c.iter().rev().copied()).collect();
        let mut out=vec![0xa5;raw.len()+11];ns::swap_components(&raw,width,&mut out).unwrap();
        assert_eq!(&out[..raw.len()],expected);assert_eq!(&out[raw.len()..],&[0xa5;11]);
        if width>1{out.fill(0xa5);assert!(ns::swap_components(&raw[..raw.len()-1],width,&mut out).is_err());assert!(out.iter().all(|v|*v==0xa5));}
    }
}
#[test]
fn resumable_dictionary_and_packet_transitions(){
    use verified_tiff::native_stream as stream;
    let mut phrase=[0xa5;4096];let mut lzw=stream::Lzw::new(false);
    assert!(lzw.code(65,&mut phrase).is_err());assert_eq!(lzw.code(256,&mut phrase),Ok(0));
    assert_eq!(lzw.code(65,&mut phrase),Ok(1));assert_eq!(phrase[0],65);
    assert_eq!(lzw.code(258,&mut phrase),Ok(2));assert_eq!(&phrase[..2],b"AA");
    assert_eq!(lzw.code(259,&mut phrase),Ok(3));assert_eq!(&phrase[..3],b"AAA");
    assert!(lzw.code(4096,&mut phrase).is_err());assert_eq!(lzw.code(257,&mut phrase),Ok(0));
    assert_eq!(stream::packet(&[254,17],0,3,0,3),Ok((2,1,3,true)));
    assert!(stream::packet(&[254,17],0,2,0,3).is_err());
    assert!(stream::packet(&[2,1,2],0,3,0,3).is_err());
}
#[test]
fn resumable_zlib_checksum_and_budget(){
    use verified_tiff::native_stream::Inflater;
    // zlib stored block containing ABC, with independently calculated Adler32.
    let wire=[0x78,0x01,0x01,3,0,0xfc,0xff,65,66,67,1,0x8d,0,0xc7];
    for corrupt in [false,true]{
        let mut s=wire;if corrupt{s[13]^=1;}
        let mut decoder=Inflater::new(&s,3).unwrap();let (mut at,mut bit)=(2,0);let mut output=Vec::new();let mut budget=Budget{remaining:100};
        let ended=loop{match decoder.step(&s,at,bit,s.len()-4,&mut budget){
            Ok((a,b,event,byte))=>{at=a;bit=b;if event==1{output.push(byte);}if event==2{break true;}},Err(_)=>break false,
        }};
        assert_eq!(ended,!corrupt);assert_eq!(output,b"ABC");
    }
    let mut decoder=Inflater::new(&wire,3).unwrap();assert!(decoder.step(&wire,2,0,wire.len()-4,&mut Budget{remaining:0}).is_err());
}
