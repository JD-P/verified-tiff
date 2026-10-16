//! Test/diagnostic adapter for the native Zstandard API; not part of the core.
use std::{env,fs,io::{self,Write}};
use verified_tiff::{native_zstd as z,Budget};
fn main(){
    let a:Vec<_>=env::args().collect();
    if a.len()!=4{eprintln!("usage: zstd-native frame needed cap");std::process::exit(2);}
    let s=fs::read(&a[1]).unwrap();let needed:usize=a[2].parse().unwrap();let cap:usize=a[3].parse().unwrap();
    let mut out=vec![0xa5;cap+17];let mut scratch=vec![0x5a;z::scratch_len(cap)+13];
    let work=match z::work_limit(&s,cap){Ok(n)=>n,Err(e)=>{eprintln!("framing: {e:?}");std::process::exit(1);}};
    let mut budget=Budget{remaining:work};
    match z::decode(&s,needed,cap,&mut out,&mut scratch,&mut budget){
        Ok(n)=>{assert_eq!(budget.remaining,0);assert!(out[cap..].iter().all(|v|*v==0xa5));assert!(scratch[z::scratch_len(cap)..].iter().all(|v|*v==0x5a));io::stdout().write_all(&out[..n]).unwrap();},
        Err(e)=>{assert!(out[cap..].iter().all(|v|*v==0xa5));assert!(scratch[z::scratch_len(cap)..].iter().all(|v|*v==0x5a));eprintln!("decode: {e:?}");std::process::exit(1);}
    }
}
