use verified_tiff::{native_zstd as z,Budget,Error};
fn header(size:usize,checksum:bool)->Vec<u8>{
    let mut b=vec![0x28,0xb5,0x2f,0xfd];let check=if checksum{4}else{0};
    if size<256{b.extend([0x20|check,size as u8]);}
    else if size<65792{b.push(0x60|check);b.extend(((size-256) as u16).to_le_bytes());}
    else{b.push(0xa0|check);b.extend((size as u32).to_le_bytes());}b
}
fn block(b:&mut Vec<u8>,kind:usize,size:usize,last:bool,data:&[u8]){
    let v=((size<<3)|(kind<<1)|last as usize) as u32;b.extend(&v.to_le_bytes()[..3]);b.extend(data);
}
fn good(s:&[u8],expected:&[u8],needed:usize,cap:usize){
    let fee=z::work_limit(s,cap).unwrap();let mut out=vec![0xa5;cap+17];let mut scratch=vec![0x5a;z::scratch_len(cap)+13];
    let mut b=Budget{remaining:fee};let n=z::decode(s,needed,cap,&mut out,&mut scratch,&mut b).unwrap();
    assert_eq!(n,expected.len());assert_eq!(&out[..n],expected);assert_eq!(b.remaining,0);
    assert!(out[cap..].iter().all(|v|*v==0xa5));assert!(scratch[z::scratch_len(cap)..].iter().all(|v|*v==0x5a));
    out.fill(0xa5);scratch.fill(0x5a);b.remaining=fee-1;
    assert_eq!(z::decode(s,needed,cap,&mut out,&mut scratch,&mut b),Err(Error::Limit));
    assert!(out.iter().all(|v|*v==0xa5));assert!(scratch.iter().all(|v|*v==0x5a));
}
#[test]
fn raw_rle_mixed_blocks_empty_and_unknown_content_size(){
    let mut s=header(0,false);block(&mut s,0,0,true,&[]);good(&s,&[],0,0);
    let mut s=header(2007,false);block(&mut s,0,3,false,b"abc");block(&mut s,1,2000,false,b"X");block(&mut s,0,4,true,b"tail");
    let expected=[&b"abc"[..],&vec![b'X';2000],&b"tail"[..]].concat();good(&s,&expected,2007,2007);good(&s,&expected,7,2100);
    let mut unknown=vec![0x28,0xb5,0x2f,0xfd,0,0x08];unknown.extend(&s[7..]);good(&unknown,&expected,2007,2100);
    let mut s=header(3,true);block(&mut s,0,3,true,b"abc");s.extend([0x99,0x09,0x77,0xad]);good(&s,b"abc",3,3);
}
#[test]
fn independent_compressed_sequence_and_overlapping_match(){
    // Three RLE FSE tables: literal length 3, offset code 2 (distance 3),
    // match code 47 (2051 + 946). The final reverse bit stream has 13 bits.
    let s=[0x28,0xb5,0x2f,0xfd,0x60,0xb8,0x0a,0x5d,0,0,0x18,b'a',b'b',b'c',1,0x54,3,2,47,0xb2,0x33];
    good(&s,&b"abc".repeat(1000),3000,3000);
}
#[test]
fn truncation_trailers_checksums_dictionary_ids_and_buffer_frames(){
    let mut s=header(3,true);block(&mut s,0,3,true,b"abc");s.extend([0x99,0x09,0x77,0xad]);
    let mut bad:Vec<Vec<u8>>=(0..s.len()).map(|n|s[..n].to_vec()).collect();
    let mut trailing=s.clone();trailing.push(0);bad.push(trailing);
    let mut checksum=s.clone();*checksum.last_mut().unwrap()^=1;bad.push(checksum);
    let mut reserved=s.clone();reserved[4]|=8;bad.push(reserved);
    let mut dictionary=s.clone();dictionary[4]|=1;dictionary.insert(5,7);bad.push(dictionary);
    let mut wrong_size=s.clone();wrong_size[5]=4;bad.push(wrong_size);
    let mut reserved_block=s.clone();reserved_block[6]|=6;bad.push(reserved_block);
    for b in bad{
        let mut out=[0xa5;10];let mut scratch=[0x5a;12];let mut budget=Budget{remaining:1000000};
        assert!(z::decode(&b,3,3,&mut out,&mut scratch,&mut budget).is_err());
        assert_eq!(&out[3..],&[0xa5;7]);assert_eq!(&scratch[3..],&[0x5a;9]);
    }
    for (outlen,scratchlen) in [(2,3),(3,2)]{
        let mut out=vec![0xa5;outlen];let mut scratch=vec![0x5a;scratchlen];
        assert_eq!(z::decode(&s,3,3,&mut out,&mut scratch,&mut Budget{remaining:1000000}),Err(Error::OutputTooSmall));
        assert!(out.iter().all(|v|*v==0xa5));assert!(scratch.iter().all(|v|*v==0x5a));
    }
    assert!(z::work_limit(&s,usize::MAX).is_err());
}
