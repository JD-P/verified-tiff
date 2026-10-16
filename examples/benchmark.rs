//! Benchmark harness only; not part of the verified implementation.
use std::{env, fs, hint::black_box, time::Instant};
use verified_tiff::{inspect_raster, Budget, Limits};
const WORK: usize = 1_000_000_000_000;
fn hash(s: &[u8]) -> u64 {
    s.iter().fold(14695981039346656037, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(1099511628211)
    })
}
fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() == 4 || args.len() == 5,
        "INPUT inspect|decode|total|combined MILLISECONDS [OUTPUT]"
    );
    let data = fs::read(&args[1]).unwrap();
    let mode = &args[2];
    let target = args[3].parse::<f64>().unwrap() / 1000.0;
    let limits = Limits {
        max_input: 268435456,
        max_output: 268435456,
        ..Limits::conservative()
    };
    let mut budget = Budget { remaining: WORK };
    let image = inspect_raster(&data, 0, &limits, &mut budget).unwrap();
    let inspect_work = WORK - budget.remaining;
    let mut output = vec![0; image.output_len()];
    let mut scratch = vec![0; image.scratch_len()];
    image
        .decode_with_scratch(&mut output, &mut scratch, &mut Budget { remaining: WORK })
        .unwrap();
    let mut run = || {
        let mut budget = Budget { remaining: WORK };
        match mode.as_str() {
            "inspect" => {
                black_box(inspect_raster(black_box(&data), 0, &limits, &mut budget).unwrap());
            }
            "decode" => {
                image
                    .decode_with_scratch(
                        black_box(&mut output),
                        black_box(&mut scratch),
                        &mut budget,
                    )
                    .unwrap();
            }
            "combined" => {
                verified_tiff::decode_deflate_into(black_box(&data),0,&limits,
                    black_box(&mut output),black_box(&mut scratch),&mut budget).unwrap();
            }
            "total" => {
                let fresh = inspect_raster(black_box(&data), 0, &limits, &mut budget).unwrap();
                fresh
                    .decode_with_scratch(
                        black_box(&mut output),
                        black_box(&mut scratch),
                        &mut budget,
                    )
                    .unwrap();
            }
            _ => panic!("unknown mode"),
        }
        black_box(budget.remaining);
    };
    run(); // Warm caches and codec paths. Never time process startup or disk I/O.
    let start = Instant::now();
    run();
    let one = start.elapsed().as_secs_f64();
    let iterations = ((target / one.max(1e-9)).ceil() as u64).clamp(1, 1_000_000);
    let start = Instant::now();
    let mut completed = 0;
    loop {
        for _ in 0..iterations {
            run();
        }
        completed += iterations;
        if start.elapsed().as_secs_f64() >= target {
            break;
        }
    }
    let seconds = start.elapsed().as_secs_f64();
    if args.len() == 5 { fs::write(&args[4], &output).unwrap(); }
    let iterations = completed;
    println!("{{\"engine\":\"rust\",\"mode\":\"{}\",\"iterations\":{},\"seconds\":{},\"ns_per_image\":{},\"width\":{},\"height\":{},\"bytes\":{},\"scratch_bytes\":{},\"inspect_work\":{},\"decode_work\":{},\"hash\":\"{:016x}\"}}",mode,iterations,seconds,seconds*1e9/iterations as f64,image.width(),image.height(),output.len(),scratch.len(),inspect_work,image.decode_work_limit(),hash(&output));
}
