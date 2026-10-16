//! Test adapter; writes native little-endian samples followed by mask bytes.
use std::{env,fs,io::{self,Write}};
use verified_tiff::{native_lerc as l,Budget};
fn main(){
    let a:Vec<_>=env::args().collect();
    if a.len()!=2{eprintln!("usage: lerc-native blob");std::process::exit(2);}
    let s=fs::read(&a[1]).unwrap();
    let info=match l::inspect(&s){Ok(v)=>v,Err(e)=>{eprintln!("header: {e:?}");std::process::exit(1);}};
    assert!(info.samples<16777216);
    let mut out=vec![0xdeadbeef;info.samples+17];let mut mask=vec![0xa5;info.mask_len+13];
    let work=l::work_limit(&s).unwrap();let mut budget=Budget{remaining:work};
    let mut short=Budget{remaining:work-1};
    assert!(l::decode(&s,&mut out,&mut mask,false,&mut short).is_err());
    assert!(out.iter().all(|v|*v==0xdeadbeef));assert!(mask.iter().all(|v|*v==0xa5));
    let result=l::decode(&s,&mut out,&mut mask,false,&mut budget);
    assert!(out[info.samples..].iter().all(|v|*v==0xdeadbeef));
    assert!(mask[info.mask_len..].iter().all(|v|*v==0xa5));
    if let Err(e)=result{eprintln!("decode: {e:?}");std::process::exit(1);}
    assert_eq!(result.unwrap(),info);assert_eq!(budget.remaining,0);
    let n=match info.datatype{0|1=>1,2|3=>2,4|5|6=>4,_=>8};
    let mut stdout=io::stdout().lock();
    for v in &out[..info.samples]{stdout.write_all(&v.to_le_bytes()[..n]).unwrap();}
    stdout.write_all(&mask[..info.mask_len]).unwrap();
}
