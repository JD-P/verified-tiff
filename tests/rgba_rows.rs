use verified_tiff::native_rgba::row;

fn reference(raw:&[u8],n:usize,ch:usize,wide:bool,rgb:bool,invert:bool,alpha:usize,premul:bool,le:bool,reverse:bool)->Vec<u8>{
    let max=if wide {65535u32}else{255};
    let sample=|i:usize|if wide {let a=[raw[2*i],raw[2*i+1]];(if le {u16::from_le_bytes(a)}else{u16::from_be_bytes(a)}) as u32}else{raw[i] as u32};
    let color=|v:u32|if wide&&!rgb {(v>>8) as u8}else{((v*255+max/2)/max) as u8};
    let mut out=Vec::new();
    for i in 0..n{
        let p=if reverse {n-1-i}else{i};let mut v=[0u8;4];
        for c in 0..3{let x=sample(p*ch+if rgb {c}else{0});v[c]=color(if invert {max-x}else{x});}
        v[3]=if alpha<ch{let a=sample(p*ch+alpha);if wide {color(a)}else{a as u8}}else{255};
        if premul{for c in 0..3{v[c]=((v[c] as u32*v[3] as u32+127)/255) as u8;}}
        if !le{v.reverse();}out.extend(v);
    }out
}
#[test]
fn rows_match_independent_integer_reference_and_preserve_suffix(){
    for wide in [false,true]{for le in [false,true]{for reverse in [false,true]{
        for (rgb,ch,invert,alpha,premul) in [(false,1,false,1,false),(false,1,true,1,false),(false,2,false,1,false),(false,3,true,2,false),(true,3,false,3,false),(true,4,false,3,false),(true,4,false,3,true),(true,6,false,5,true),(true,64,false,37,true)]{
            let n=257;let words:Vec<u16>=(0..n*ch).map(|i|[0,1,127,128,129,255,256,257,32767,32768,65406,65407,65534,65535,(i*7331) as u16][i%15]).collect();
            let raw:Vec<u8>=words.iter().flat_map(|&v|if wide {if le{v.to_le_bytes().to_vec()}else{v.to_be_bytes().to_vec()}}else{vec![v as u8]}).collect();
            let expected=reference(&raw,n,ch,wide,rgb,invert,alpha,premul,le,reverse);
            let mut out=vec![0xa5;n*4+19];row(&raw,n,ch,wide,rgb,invert,alpha,premul,le,reverse,&mut out).unwrap();
            assert_eq!(&out[..n*4],expected);assert_eq!(&out[n*4..],&[0xa5;19]);
        }
    }}}
}
#[test]
fn every_16bit_rounding_value_and_every_8bit_alpha_pair(){
    for rgb in [false,true]{for invert in [false,true]{if rgb&&invert{continue;}
        let ch=if rgb{3}else{1};let raw:Vec<u8>=(0u32..65536).flat_map(|v|std::iter::repeat_n((v as u16).to_le_bytes(),ch).flatten()).collect();
        let mut out=vec![0;65536*4];row(&raw,65536,ch,true,rgb,invert,ch,false,true,false,&mut out).unwrap();
        assert_eq!(out,reference(&raw,65536,ch,true,rgb,invert,ch,false,true,false));
    }}
    let raw:Vec<u8>=(0..256).flat_map(|a|(0..256).flat_map(move |v|[v as u8,v as u8,v as u8,a as u8])).collect();
    let mut out=vec![0;raw.len()];row(&raw,65536,4,false,true,false,3,true,true,false,&mut out).unwrap();
    assert_eq!(out,reference(&raw,65536,4,false,true,false,3,true,true,false));
}
#[test]
fn invalid_rows_are_atomic_including_overflow_and_short_buffers(){
    let raw=[17;100];
    for (n,ch,wide,rgb,invert,alpha,premul) in [(1,0,false,false,false,0,false),(1,65,false,false,false,65,false),(1,2,false,true,false,2,false),(1,3,false,true,true,3,false),(1,3,false,true,false,2,false),(1,3,false,true,false,4,false),(1,1,false,false,false,1,true),(1,3,false,true,false,3,true),(usize::MAX,3,false,true,false,3,false),(20,3,true,true,false,3,false),(26,3,false,true,false,3,false)]{
        let mut out=[0xa5;100];assert!(row(&raw,n,ch,wide,rgb,invert,alpha,premul,true,false,&mut out).is_err());assert_eq!(out,[0xa5;100]);
    }
    let mut out=[0xa5;7];row(&[],0,3,false,true,false,3,false,true,false,&mut out).unwrap();assert_eq!(out,[0xa5;7]);
}
