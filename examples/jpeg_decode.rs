//! Small OS adapter for exercising the reusable JPEG segment API.
use verified_tiff::{inspect_jpeg,Limits,Budget};
fn main() -> Result<(),Box<dyn std::error::Error>> {
    let a:Vec<String>=std::env::args().collect();
    let data=std::fs::read(&a[1])?;
    let mut budget=Budget {remaining:usize::MAX};
    let image=inspect_jpeg(&data,true,&Limits::conservative(),&mut budget).map_err(|e| format!("{e:?}"))?;
    let mut out=vec![0u8;image.output_len()];let mut scratch=vec![0u8;image.scratch_len()];
    image.decode_with_scratch(&mut out,&mut scratch,&mut budget).map_err(|e| format!("{e:?}"))?;
    std::fs::write(&a[2],out)?;
    println!("{} {} {}",image.width(),image.height(),image.channels());Ok(())
}
