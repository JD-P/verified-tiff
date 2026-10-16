//! Test adapter for the TIFF/JPEG path before exercising the public dispatcher.
use verified_tiff::{inspect_jpeg_tiff,Limits,Budget};
fn main() -> Result<(),Box<dyn std::error::Error>> {
    let a:Vec<String>=std::env::args().collect();let data=std::fs::read(&a[1])?;let mut b=Budget {remaining:usize::MAX};
    let i=inspect_jpeg_tiff(&data,0,&Limits::conservative(),&mut b).map_err(|e|format!("{e:?}"))?;
    let mut out=vec![0;i.output_len()];let mut tmp=vec![0;i.scratch_len()];
    i.decode_with_scratch(&mut out,&mut tmp,&mut b).map_err(|e|format!("{e:?}"))?;
    std::fs::write(&a[2],out)?;println!("{} {} {} scratch={} work={}",i.width(),i.height(),i.channels(),i.scratch_len(),i.decode_work_limit());Ok(())
}
