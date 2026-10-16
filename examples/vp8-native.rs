use std::io::{self,Write};
use verified_tiff::{native_vp8 as vp,Budget};
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1);}}
fn run()->Result<(),String>{
 let args:Vec<_>=std::env::args().collect();if args.len()!=4 && args.len()!=5{return Err("file width height".into());}
 let data=std::fs::read(&args[1]).map_err(|e|e.to_string())?;let w=args[2].parse::<usize>().map_err(|e|e.to_string())?;let h=args[3].parse::<usize>().map_err(|e|e.to_string())?;
 let info=vp::storage(w,h).map_err(|e|format!("storage {e:?}"))?;if info.bytes>64*1024*1024{return Err("test cap".into());}
 let mut pixels=vec![0x75;info.bytes+7];let mut cols=vec![vp::Context::empty();info.columns+1];let mut filters=vec![vp::Filter::empty();info.blocks+1];
 let fee=vp::work_limit(data.len(),w,h).map_err(|e|format!("work {e:?}"))?;let mut budget=Budget {remaining:fee};
 let mut short=Budget{remaining:fee-1};assert!(vp::decode_yuv(&data,w,h,&mut pixels,&mut cols,&mut filters,&mut short).is_err());assert!(pixels.iter().all(|p|*p==0x75));
 vp::decode_yuv(&data,w,h,&mut pixels,&mut cols,&mut filters,&mut budget).map_err(|e|format!("decode {e:?}"))?;
 assert_eq!(budget.remaining,0);assert_eq!(&pixels[info.bytes..],&[0x75;7]);
 if args.len()==5{let mut rgba=vec![[0xa5;4];w*h+5];let mut work=Budget {remaining:w*h*1000};vp::render_rgba(&pixels,w,h,&mut rgba,&mut work).map_err(|e|format!("rgb {e:?}"))?;assert_eq!(work.remaining,0);assert_eq!(&rgba[w*h..],&[[0xa5;4];5]);let mut out=io::stdout().lock();for p in &rgba[..w*h]{out.write_all(p).map_err(|e|e.to_string())?;}return Ok(());}
 let mut out=io::stdout().lock();for y in 0..h{out.write_all(&pixels[y*info.width..y*info.width+w]).map_err(|e|e.to_string())?;}
 for base in [info.y_len,info.y_len+info.chroma_len]{for y in 0..h.div_ceil(2){let at=base+y*(info.width/2);out.write_all(&pixels[at..at+w.div_ceil(2)]).map_err(|e|e.to_string())?;}}
 Ok(())
}
