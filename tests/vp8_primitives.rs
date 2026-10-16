use verified_tiff::Error;
use vstd::prelude::*;
fn bit_divisor(n:usize)->u8{1u8<<n}
#[path="../src/zstd_bits.rs"] mod zstd_bits;
#[path="../src/vp8_bool.rs"] mod vp8_bool;
#[path="../src/vp8_tree.rs"] mod vp8_tree;
#[path="../src/vp8_coeff.rs"] mod vp8_coeff;
#[path="../src/vp8_transform.rs"] mod vp8_transform;
#[path="../src/vp8_predict.rs"] mod vp8_predict;
#[path="../src/vp8_filter_kernel.rs"] mod vp8_filter_kernel;
#[test]
fn upstream_arithmetic_and_transforms(){
 let mut state=0x73352ba10623995du64;
 let mut next=||{state=state.wrapping_mul(6364136223846793005).wrapping_add(1);(state>>32) as u32};
 let mut input=Vec::new();let mut expected=Vec::new();
 for _ in 0..1000{
  let mut data=[0u8;256];let mut probs=[0u8;256];for v in &mut data{*v=next() as u8;}data[0]%=255;for p in &mut probs{*p=next() as u8;}
  input.push(0);input.extend(data);input.extend(probs);let mut r=vp8_bool::start(0,256);
  for p in probs{let (b,t)=vp8_bool::bit(&data,r,p).unwrap();r=t;expected.push(b as u8);}
 }
 for i in 0..10000{for kind in 1..=2{
  input.push(kind);let mut c=[0i32;16];for v in &mut c{*v=if i<100 || kind==2{((next()%4096) as i32)-2048}else{next() as i16 as i32};input.extend((*v as i16).to_le_bytes());}
  let got=vp8_transform::transform(&c,kind==1);
  if kind==1{for v in got{expected.extend((v as i16).to_le_bytes());}}
  else{for v in got{let pred=next() as u8;input.push(pred);expected.push((v+pred as i32).clamp(0,255) as u8);}}
 }}
 // Every legal DC coefficient checks the shortcut against upstream. AC
 // impulses at every position exercise the fallback and signed rounding.
 for dc in -32768i32..=32767{for kind in 1..=2{
  input.push(kind);let mut c=[0i32;16];c[0]=dc;for v in c{input.extend((v as i16).to_le_bytes());}
  let got=vp8_transform::transform(&c,kind==1);
  if kind==1{for v in got{expected.extend((v as i16).to_le_bytes());}}
  else{for v in got{let pred=127u8;input.push(pred);expected.push((v+pred as i32).clamp(0,255) as u8);}}
 }}
 for at in 1..16{for ac in [-32768i32,-2048,-1,1,2047,32767]{for kind in 1..=2{
  input.push(kind);let mut c=[0i32;16];c[0]=-13;c[at]=ac;for v in c{input.extend((v as i16).to_le_bytes());}
  let got=vp8_transform::transform(&c,kind==1);
  if kind==1{for v in got{expected.extend((v as i16).to_le_bytes());}}
  else{for v in got{input.push(127);expected.push((v+127).clamp(0,255) as u8);}}
 }}}
 for i in 0..1000{for mode in 0..10{let mut e=[0u8;13];for v in &mut e{*v=if i<2{(i*255) as u8}else{next() as u8};}input.push(mode+3);input.extend(e);for k in 0..16{expected.push(vp8_predict::predict4(&e,mode as usize,k));}}}
 for i in 0..100000{let simple=next()%2!=0;let edge=next()%2!=0;let limit=(next()%194) as u8;let inner=(next()%64) as u8;let hev=(next()%3) as u8;let mut e=[0u8;8];let base=(next()%240) as u8;for v in &mut e{*v=if i%2==0{next() as u8}else{base+(next()%16) as u8};}input.extend([13,simple as u8,edge as u8,limit,inner,hev]);input.extend(e);expected.extend(vp8_filter_kernel::filter(&e,simple,edge,limit,inner,hev));}
 std::fs::write("target/vp8-primitives.in",input).unwrap();
 let status=std::process::Command::new("target/vp8-primitives-oracle").args(["target/vp8-primitives.in","target/vp8-primitives.out"]).status().unwrap();assert!(status.success());
 let actual=std::fs::read("target/vp8-primitives.out").unwrap();assert_eq!(actual.len(),expected.len());
 for (i,(a,b)) in actual.iter().zip(&expected).enumerate(){assert_eq!(a,b,"upstream primitive byte {i}");}
}

// Frozen pre-SSE2 source is an independent regression oracle for wide values
// beyond the defined range of libwebp's 16-bit SIMD transform.
#[path="reference/vp8_transform_scalar.rs"] mod scalar_transform;
#[test]
fn wide_transform_matches_previous_scalar() {
 let mut seed=0x1da3c65b44aa3269u64;
 for _ in 0..20000 {
  let c=std::array::from_fn(|_|{seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;seed as i16 as i32});
  for walsh in [false,true] {assert_eq!(vp8_transform::transform(&c,walsh),scalar_transform::transform(&c,walsh));}
 }
}
