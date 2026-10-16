//! Deterministic mutation smoke fuzzing. This is not a coverage-guided fuzzer.
use verified_tiff::{inspect_raster as inspect, Budget, Limits};
fn next(s: &mut u64) -> u64 { *s ^= *s << 13; *s ^= *s >> 7; *s ^= *s << 17; *s }
fn main() {
    let iterations: usize = std::env::args().nth(1).map(|s|s.parse().expect("iteration count")).unwrap_or(20000);
    let mut seed=0x5449464650524f4fu64;
    let corpus: [&[u8];70]=[
        include_bytes!("../tests/fixtures/webp-witness.tif"),
        include_bytes!("../tests/fixtures/webp-rgb.tif"),
        include_bytes!("../tests/fixtures/webp-alpha.tif"),
        include_bytes!("../tests/fixtures/webp-associated.tif"),
        include_bytes!("../tests/fixtures/webp-tile.tif"),
        include_bytes!("../tests/fixtures/webp-copy.tif"),

        include_bytes!("../tests/fixtures/acceptance-float64-plain.tif"),
        include_bytes!("../tests/fixtures/acceptance-float64-predict2.tif"),
        include_bytes!("../tests/fixtures/float64-gray-be.tif"),
        include_bytes!("../tests/fixtures/float64-planar.tif"),
        include_bytes!("../tests/fixtures/float64-none.tif"),
        include_bytes!("../tests/fixtures/float64-packbits.tif"),

        include_bytes!("../tests/fixtures/fax-g3-witness.tif"),
        include_bytes!("../tests/fixtures/fax-g4-witness.tif"),
        include_bytes!("../tests/fixtures/fax-g3.tif"),
        include_bytes!("../tests/fixtures/fax-mixed.tif"),
        include_bytes!("../tests/fixtures/fax-g4-tile.tif"),
        include_bytes!("../tests/fixtures/fax-lsb.tif"),
        include_bytes!("../tests/fixtures/fax-noeol.tif"),

        include_bytes!("../tests/fixtures/jpeg-gray.tif"),
        include_bytes!("../tests/fixtures/jpeg-rgb.tif"),
        include_bytes!("../tests/fixtures/jpeg-ycbcr.tif"),
        include_bytes!("../tests/fixtures/jpeg-tile.tif"),
        include_bytes!("../tests/fixtures/ojpeg-tables.tif"),
        include_bytes!("../tests/fixtures/ojpeg-header.tif"),
        include_bytes!("../tests/fixtures/gray-strip.tif"),
        include_bytes!("../tests/fixtures/rgb-be-tile.tif"),
        include_bytes!("../tests/fixtures/white-before.tif"),
        include_bytes!("../tests/fixtures/multipage.tif"),
        include_bytes!("../tests/fixtures/bilevel-strip.tif"),
        include_bytes!("../tests/fixtures/bilevel-lsb-tile.tif"),
        include_bytes!("../tests/fixtures/packbits-bilevel-tile.tif"),
        include_bytes!("../tests/fixtures/packbits-gray-strip.tif"),
        include_bytes!("../tests/fixtures/packbits-rgb-tile.tif"),
        include_bytes!("../tests/fixtures/lzw-bilevel-tile.tif"),
        include_bytes!("../tests/fixtures/lzw-gray-strip.tif"),
        include_bytes!("../tests/fixtures/lzw-rgb-tile.tif"),
        include_bytes!("../tests/fixtures/lzw-dictionary.tif"),
        include_bytes!("../tests/fixtures/lzw-tail-witness.tif"),
        include_bytes!("../tests/fixtures/lzw-legacy-witness.tif"),
        include_bytes!("../tests/fixtures/lzw-legacy-full.tif"),
        include_bytes!("../tests/fixtures/alpha-2-1-none.tif"),
        include_bytes!("../tests/fixtures/alpha-2-1-packbits.tif"),
        include_bytes!("../tests/fixtures/alpha-2-1-lzw.tif"),
        include_bytes!("../tests/fixtures/alpha-2-2-none.tif"),
        include_bytes!("../tests/fixtures/alpha-2-2-packbits.tif"),
        include_bytes!("../tests/fixtures/alpha-2-2-lzw.tif"),
        include_bytes!("../tests/fixtures/alpha-4-1-none.tif"),
        include_bytes!("../tests/fixtures/alpha-4-1-packbits.tif"),
        include_bytes!("../tests/fixtures/alpha-4-1-lzw.tif"),
        include_bytes!("../tests/fixtures/alpha-4-2-none.tif"),
        include_bytes!("../tests/fixtures/alpha-4-2-packbits.tif"),
        include_bytes!("../tests/fixtures/alpha-4-2-lzw.tif"),

        include_bytes!("../tests/fixtures/palette1.tif"),
        include_bytes!("../tests/fixtures/palette4.tif"),
        include_bytes!("../tests/fixtures/palette8.tif"),
        include_bytes!("../tests/fixtures/rgb16.tif"),
        include_bytes!("../tests/fixtures/gray16.tif"),
        include_bytes!("../tests/fixtures/planar-alpha.tif"),
        include_bytes!("../tests/fixtures/deflate-stored.tif"),
        include_bytes!("../tests/fixtures/deflate-fixed.tif"),
        include_bytes!("../tests/fixtures/deflate-bilevel.tif"),
        include_bytes!("../tests/fixtures/deflate-palette.tif"),
        include_bytes!("../tests/fixtures/deflate-planar16.tif"),
        include_bytes!("../tests/fixtures/deflate-alpha.tif"),
        include_bytes!("../tests/fixtures/deflate-dynamic.tif"),
        include_bytes!("../tests/upstream/logluv-3c-16b.tiff"),
        include_bytes!("../tests/fixtures/logluv-strip.tif"),
        include_bytes!("../tests/fixtures/logluv-tile.tif"),
        include_bytes!("../tests/fixtures/logluv-pages.tif"),
    ];
    let limits=Limits {max_input:32768,max_dimension:128,max_output:16384,
        max_directories:8,max_tags:128,max_segments:128};
    let mut accepted=0;
    let mut jpeg_accepted=0;let mut jpeg_decoded=0;
    let mut fax_accepted=0;let mut fax_decoded=0;
    let mut float_accepted=0;let mut float_decoded=0;
    let mut webp_accepted=0;let mut webp_decoded=0;
    let mut logluv_accepted=0;let mut logluv_decoded=0;
    let mut errors=[0usize;6];
    for n in 0..iterations {
        let mut data=corpus[n%corpus.len()].to_vec();
        for _ in 0..next(&mut seed)%9 {
            let pos=next(&mut seed) as usize % data.len();
            data[pos]=next(&mut seed) as u8;
        }
        if n%3==0 {let len=next(&mut seed) as usize%(data.len()+1);data.truncate(len);}
        if n%17==0 {for b in &mut data {*b=next(&mut seed) as u8;}}
        let work=next(&mut seed) as usize % if n%corpus.len()<6 {100_000_000} else {200_000};
        let mut budget=Budget{remaining:work};
        let page=next(&mut seed) as usize%10;
        match inspect(&data,page,&limits,&mut budget) {
            Ok(image)=>{
                accepted+=1;
                let logluv=image.compression()==34676;if logluv {logluv_accepted+=1;}
                let webp=image.compression()==50001;if webp {webp_accepted+=1;}
                let float=image.sample_format()==verified_tiff::SampleFormat::IeeeFloat;
                if float {float_accepted+=1;}
                let jpeg=image.compression()==6 || image.compression()==7;
                if jpeg {jpeg_accepted+=1;}
                let fax=image.compression()==3 || image.compression()==4;
                if fax {fax_accepted+=1;}
                assert!(image.output_len()<=limits.max_output);
                let len=if n%5==0 {image.output_len().saturating_sub(1)} else {image.output_len()+4};
                let mut output=vec![0xa5;len];
                let mut scratch=vec![0x5a;image.scratch_len()+3];
                let decoded=image.decode_with_scratch(&mut output,&mut scratch,&mut budget);
                if jpeg && decoded.is_ok() {jpeg_decoded+=1;}
                if fax && decoded.is_ok() {fax_decoded+=1;}
                if float && decoded.is_ok() {float_decoded+=1;}
                if webp && decoded.is_ok() {webp_decoded+=1;}
                if logluv && decoded.is_ok() {logluv_decoded+=1;}
                assert!(scratch[image.scratch_len()..].iter().all(|b|*b==0x5a));
                if len>image.output_len(){assert!(output[image.output_len()..].iter().all(|b|*b==0xa5));}
            },
            Err(e)=>{let k=match e {verified_tiff::Error::Truncated=>0,verified_tiff::Error::Invalid=>1,
                verified_tiff::Error::Unsupported=>2,verified_tiff::Error::Limit=>3,
                verified_tiff::Error::OutputTooSmall=>4,verified_tiff::Error::PageNotFound=>5};errors[k]+=1;}
        }
        assert!(budget.remaining<=work);
    }
    println!("seed=0x5449464650524f4f iterations={iterations} accepted={accepted} errors={errors:?} jpeg_accepted={jpeg_accepted} jpeg_decoded={jpeg_decoded} fax_accepted={fax_accepted} fax_decoded={fax_decoded} float_accepted={float_accepted} float_decoded={float_decoded} webp_accepted={webp_accepted} webp_decoded={webp_decoded} logluv_accepted={logluv_accepted} logluv_decoded={logluv_decoded}");
}
