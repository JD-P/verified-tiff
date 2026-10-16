//! Unverified operating-system adapter; all TIFF interpretation is in the core.
use std::io::{Read, Write};
use verified_tiff::{inspect_raster as inspect, AlphaMode, Budget, Limits, SampleFormat, ColorModel};

fn run() -> Result<(), String> {
    let mut limits = Limits::conservative();
    let mut page = 0;
    let mut work = 100_000_000usize;
    let mut paths = Vec::new();
    let mut metadata_only = false;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--help" || arg == "-h" {
            println!("verified-tiff INPUT [OUTPUT.pnm-or-pam-or-npy] [--inspect] [--page N] [--work N]\n  [--max-input N] [--max-output N] [--max-dimension N]\n  [--max-directories N] [--max-tags N] [--max-segments N]");
            return Ok(());
        }
        if arg == "--inspect" { metadata_only = true; continue; }
        if arg.starts_with("--") {
            let n: usize = args.next().ok_or("missing option value")?.parse().map_err(|_| "invalid number")?;
            match arg.as_str() {
                "--page" => page = n,
                "--work" => work = n,
                "--max-input" => limits.max_input = n,
                "--max-output" => limits.max_output = n,
                "--max-dimension" => limits.max_dimension = n,
                "--max-directories" => limits.max_directories = n,
                "--max-tags" => limits.max_tags = n,
                "--max-segments" => limits.max_segments = n,
                _ => return Err(format!("unknown option {arg}")),
            }
        } else { paths.push(arg); }
    }
    if paths.len() != if metadata_only { 1 } else { 2 } {
        return Err("usage: verified-tiff INPUT OUTPUT.pnm-or-pam-or-npy; or INPUT --inspect".into());
    }
    let mut file = std::fs::File::open(&paths[0]).map_err(|e| e.to_string())?;
    let mut input = Vec::new();
    let mut chunk = [0u8; 65536];
    loop {
        let n = file.read(&mut chunk).map_err(|e| e.to_string())?;
        if n == 0 { break; }
        let new_len = input.len().checked_add(n).ok_or("input length overflow")?;
        if new_len > limits.max_input { return Err("Limit: input exceeds max-input".into()); }
        input.try_reserve_exact(n).map_err(|_| "input allocation failed")?;
        input.extend_from_slice(&chunk[..n]);
    }
    let mut budget = Budget { remaining: work };
    let image = inspect(&input, page, &limits, &mut budget).map_err(|e| format!("{e:?}"))?;
    if !metadata_only {
        let mut output = Vec::new();
        output.try_reserve_exact(image.output_len()).map_err(|_| "output allocation failed")?;
        output.resize(image.output_len(), 0);
        let mut scratch=Vec::new();
        scratch.try_reserve_exact(image.scratch_len()).map_err(|_| "scratch allocation failed")?;
        scratch.resize(image.scratch_len(),0);
        image.decode_with_scratch(&mut output,&mut scratch, &mut budget).map_err(|e| format!("{e:?}"))?;
        let mut target = std::fs::File::create(&paths[1]).map_err(|e| e.to_string())?;
        let maxval=if image.output_bits_per_sample()==16 {65535} else {255};
        if image.sample_format()!=SampleFormat::UnsignedInteger {
            let dtype=if image.sample_format()==SampleFormat::IeeeFloat {"<f8"} else {"<i2"};
            let shape=if image.channels()==1 {format!("({}, {})",image.height(),image.width())}
                else {format!("({}, {}, {})",image.height(),image.width(),image.channels())};
            let mut header=format!("{{'descr': '{dtype}', 'fortran_order': False, 'shape': {shape}, }}");
            let padding=(64-(10+header.len()+1)%64)%64;
            header.extend(std::iter::repeat_n(' ',padding));header.push('\n');
            let header_len=u16::try_from(header.len()).map_err(|_|"NPY header too large")?;
            target.write_all(b"\x93NUMPY\x01\x00").map_err(|e|e.to_string())?;
            target.write_all(&header_len.to_le_bytes()).map_err(|e|e.to_string())?;
            target.write_all(header.as_bytes()).map_err(|e|e.to_string())?;
        } else if image.has_alpha() {
            let tuple=if image.channels()==2 {"GRAYSCALE_ALPHA"} else {"RGB_ALPHA"};
            write!(target,"P7\nWIDTH {}\nHEIGHT {}\nDEPTH {}\nMAXVAL {maxval}\nTUPLTYPE {tuple}\nENDHDR\n",
                image.width(),image.height(),image.channels()).map_err(|e|e.to_string())?;
        } else {
            let magic=if image.channels()==1 {"P5"} else {"P6"};
            write!(target,"{magic}\n{} {}\n{maxval}\n",image.width(),image.height()).map_err(|e|e.to_string())?;
        }
        target.write_all(&output).map_err(|e| e.to_string())?;
    }
    let source_alpha=match image.alpha_mode() {AlphaMode::None=>"none",AlphaMode::Associated=>"associated",AlphaMode::Unassociated=>"unassociated"};
    let output_alpha=if image.has_alpha() {"straight"} else {"none"};
    let sample_format=match image.sample_format() {SampleFormat::IeeeFloat=>"ieee_float",SampleFormat::SignedInteger=>"signed_integer",SampleFormat::UnsignedInteger=>"unsigned_integer"};
    let color_model=match image.color_model() {ColorModel::Gray=>"gray",ColorModel::Rgb=>"rgb",ColorModel::LogLuv=>"logluv16"};
    let output_order=if image.sample_format()!=SampleFormat::UnsignedInteger {"little"} else if image.output_bits_per_sample()==16 {"big"} else {"not_applicable"};
    println!("{{\"width\":{},\"height\":{},\"channels\":{},\"bits_per_sample\":{},\"compression\":{},\"predictor\":{},\"sample_format\":\"{}\",\"color_model\":\"{}\",\"output_byte_order\":\"{}\",\"source_alpha\":\"{}\",\"output_alpha\":\"{}\",\"output_bits_per_sample\":{},\"planar_configuration\":{},\"scratch_bytes\":{},\"output_bytes\":{},\"decode_work\":{},\"work_used\":{}}}",
        image.width(),image.height(),image.channels(),image.bits_per_sample(),image.compression(),image.predictor(),sample_format,color_model,output_order,source_alpha,output_alpha,image.output_bits_per_sample(),image.planar_configuration(),image.scratch_len(),image.output_len(),image.decode_work_limit(),work-budget.remaining);
    Ok(())
}

fn main() {
    if let Err(e) = run() { eprintln!("verified-tiff: {e}"); std::process::exit(2); }
}
