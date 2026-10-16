//! Public native codec rejection, allocation boundaries and insufficient work.
use verified_tiff::{native_lerc as l,native_webp as w,native_vp8 as v,Budget};
#[test]
fn malformed_and_zero_budget_preserve_suffixes(){
 let mut state=73185u64;
 for size in 0..1024{
  let mut bytes=vec![0;size];for b in &mut bytes{state=state.wrapping_mul(6364136223846793005).wrapping_add(1);*b=(state>>32) as u8;}
  let mut words=[0xdeadbeefu64;17];let mut mask=[0xa5;13];let mut budget=Budget{remaining:0};
  assert!(l::decode(&bytes,&mut words,&mut mask,false,&mut budget).is_err());assert_eq!(words,[0xdeadbeef;17]);assert_eq!(mask,[0xa5;13]);
  let cap=w::scratch_len(3,5).unwrap();let mut scratch=vec![[0x5a;4];cap+7];let mut pixels=[[0xa5;4];32];let mut groups=[w::EntropyGroup::empty();1];
  assert!(w::required_groups(&bytes,3,5,&mut scratch,&mut budget).is_err());assert!(w::decode(&bytes,3,5,&mut groups,&mut scratch,&mut pixels,&mut budget).is_err());assert_eq!(pixels,[[0xa5;4];32]);assert!(scratch.iter().all(|p|*p==[0x5a;4]));
  let shape=v::storage(3,5).unwrap();let mut yuv=vec![0xa5;shape.bytes+17];let mut columns=vec![v::Context::empty();shape.columns];let mut filters=vec![v::Filter::empty();shape.blocks];
  assert!(v::decode_yuv(&bytes,3,5,&mut yuv,&mut columns,&mut filters,&mut budget).is_err());assert!(yuv.iter().all(|b|*b==0xa5));assert_eq!(budget.remaining,0);
 }
}
