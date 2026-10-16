//! Independent-codec test adapter; outside the verified core.
use std::{env,fs,io::{self,Write}};
use verified_tiff::{native_webp as w,Budget};
fn main(){
    let args:Vec<_>=env::args().collect();
    if args.len()!=4{eprintln!("usage: webp-native file width height");std::process::exit(2);}
    let s=fs::read(&args[1]).unwrap();let width:usize=args[2].parse().unwrap();let height:usize=args[3].parse().unwrap();
    let cap=match w::scratch_len(width,height){Ok(n)=>n,Err(e)=>{eprintln!("dimensions: {e:?}");std::process::exit(1);}};
    if cap>16_777_216{std::process::exit(1);}
    let mut scratch=vec![[0x5a;4];cap+13];let mut budget=Budget {remaining:usize::MAX};
    let count=match w::required_groups(&s,width,height,&mut scratch,&mut budget){Ok(n)=>n,Err(e)=>{eprintln!("preparation: {e:?}");std::process::exit(1);}};
    if count>4096{std::process::exit(1);}
    let mut groups=vec![w::EntropyGroup::empty();count+1];let total=width*height;let mut pixels=vec![[0xa5;4];total+17];
    let mut short=Budget{remaining:w::work_limit(s.len(),width,height,count).unwrap()-1};
    assert!(w::decode(&s,width,height,&mut groups,&mut scratch,&mut pixels,&mut short).is_err());
    assert!(pixels.iter().all(|p|*p==[0xa5;4]));
    let before=budget.remaining;let result=w::decode(&s,width,height,&mut groups,&mut scratch,&mut pixels,&mut budget);
    assert!(scratch[cap..].iter().all(|v|*v==[0x5a;4]));assert!(pixels[total..].iter().all(|v|*v==[0xa5;4]));
    if let Err(e)=result{eprintln!("decode: {e:?}");std::process::exit(1);}
    assert_eq!(before-budget.remaining,w::work_limit(s.len(),width,height,count).unwrap());
    let mut out=io::stdout().lock();for pixel in &pixels[..total]{out.write_all(pixel).unwrap();}
}
