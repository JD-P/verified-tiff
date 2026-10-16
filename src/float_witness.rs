// Concrete float64 TIFF acceptance witnesses, with and without Predictor 2.
use super::*;
verus! {
spec fn bytes(predict:bool) -> Seq<u8> {
    let b=seq![73u8,73u8,42u8,0u8,8u8,0u8,0u8,0u8,9u8,0u8,0u8,1u8,3u8,0u8,1u8,0u8,0u8,0u8,2u8,0u8,0u8,0u8,1u8,1u8,3u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,2u8,1u8,3u8,0u8,1u8,0u8,0u8,0u8,64u8,0u8,0u8,0u8,3u8,1u8,3u8,0u8,1u8,0u8,0u8,0u8,5u8,0u8,0u8,0u8,6u8,1u8,3u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,17u8,1u8,4u8,0u8,1u8,0u8,0u8,0u8,122u8,0u8,0u8,0u8,23u8,1u8,4u8,0u8,1u8,0u8,0u8,0u8,38u8,0u8,0u8,0u8,61u8,1u8,3u8,0u8,1u8,0u8,0u8,0u8,1u8,0u8,0u8,0u8,83u8,1u8,3u8,0u8,1u8,0u8,0u8,0u8,3u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8,128u8,63u8,224u8,15u8,248u8,3u8,254u8,0u8,255u8,128u8,63u8,224u8,15u8,248u8,3u8,254u8,0u8,255u8,128u8,0u8,160u8,0u8,8u8,0u8,2u8,0u8,0u8,128u8,0u8,32u8,0u8,8u8,0u8,2u8,0u8,0u8,128u8,128u8];
    b.update(102,if predict {2u8} else {1u8})
}
spec fn limits() -> Limits {Limits {max_input:160,max_dimension:2,max_output:16,max_directories:1,max_tags:9,max_segments:1}}
proof fn stream(predict:bool)
    ensures lzw_valid(bytes(predict),122,160,16),
{
    let s=bytes(predict);
    reveal(lzw_valid);reveal(lz_legacy);reveal(lz_bits);reveal(power2);reveal(lz_initial);reveal(lz_after);reveal(lz_data_ok);reveal(lz_defined);reveal(lz_phrase);
    assert(lz_bits(bytes(false),976,9)==256) by(compute);
    assert(lz_bits(bytes(true),976,9)==256) by(compute);
    assert(lz_bits(s,976,9)==256);
    assert(lz_bits(bytes(false),985,9)==255) by(compute);
    assert(lz_bits(bytes(true),985,9)==255) by(compute);
    assert(lz_bits(s,985,9)==255);
    assert(lz_bits(bytes(false),994,9)==256) by(compute);
    assert(lz_bits(bytes(true),994,9)==256) by(compute);
    assert(lz_bits(s,994,9)==256);
    assert(lz_bits(bytes(false),1003,9)==255) by(compute);
    assert(lz_bits(bytes(true),1003,9)==255) by(compute);
    assert(lz_bits(s,1003,9)==255);
    assert(lz_bits(bytes(false),1012,9)==256) by(compute);
    assert(lz_bits(bytes(true),1012,9)==256) by(compute);
    assert(lz_bits(s,1012,9)==256);
    assert(lz_bits(bytes(false),1021,9)==255) by(compute);
    assert(lz_bits(bytes(true),1021,9)==255) by(compute);
    assert(lz_bits(s,1021,9)==255);
    assert(lz_bits(bytes(false),1030,9)==256) by(compute);
    assert(lz_bits(bytes(true),1030,9)==256) by(compute);
    assert(lz_bits(s,1030,9)==256);
    assert(lz_bits(bytes(false),1039,9)==255) by(compute);
    assert(lz_bits(bytes(true),1039,9)==255) by(compute);
    assert(lz_bits(s,1039,9)==255);
    assert(lz_bits(bytes(false),1048,9)==256) by(compute);
    assert(lz_bits(bytes(true),1048,9)==256) by(compute);
    assert(lz_bits(s,1048,9)==256);
    assert(lz_bits(bytes(false),1057,9)==255) by(compute);
    assert(lz_bits(bytes(true),1057,9)==255) by(compute);
    assert(lz_bits(s,1057,9)==255);
    assert(lz_bits(bytes(false),1066,9)==256) by(compute);
    assert(lz_bits(bytes(true),1066,9)==256) by(compute);
    assert(lz_bits(s,1066,9)==256);
    assert(lz_bits(bytes(false),1075,9)==255) by(compute);
    assert(lz_bits(bytes(true),1075,9)==255) by(compute);
    assert(lz_bits(s,1075,9)==255);
    assert(lz_bits(bytes(false),1084,9)==256) by(compute);
    assert(lz_bits(bytes(true),1084,9)==256) by(compute);
    assert(lz_bits(s,1084,9)==256);
    assert(lz_bits(bytes(false),1093,9)==255) by(compute);
    assert(lz_bits(bytes(true),1093,9)==255) by(compute);
    assert(lz_bits(s,1093,9)==255);
    assert(lz_bits(bytes(false),1102,9)==256) by(compute);
    assert(lz_bits(bytes(true),1102,9)==256) by(compute);
    assert(lz_bits(s,1102,9)==256);
    assert(lz_bits(bytes(false),1111,9)==255) by(compute);
    assert(lz_bits(bytes(true),1111,9)==255) by(compute);
    assert(lz_bits(s,1111,9)==255);
    assert(lz_bits(bytes(false),1120,9)==256) by(compute);
    assert(lz_bits(bytes(true),1120,9)==256) by(compute);
    assert(lz_bits(s,1120,9)==256);
    assert(lz_bits(bytes(false),1129,9)==2) by(compute);
    assert(lz_bits(bytes(true),1129,9)==2) by(compute);
    assert(lz_bits(s,1129,9)==2);
    assert(lz_bits(bytes(false),1138,9)==256) by(compute);
    assert(lz_bits(bytes(true),1138,9)==256) by(compute);
    assert(lz_bits(s,1138,9)==256);
    assert(lz_bits(bytes(false),1147,9)==0) by(compute);
    assert(lz_bits(bytes(true),1147,9)==0) by(compute);
    assert(lz_bits(s,1147,9)==0);
    assert(lz_bits(bytes(false),1156,9)==256) by(compute);
    assert(lz_bits(bytes(true),1156,9)==256) by(compute);
    assert(lz_bits(s,1156,9)==256);
    assert(lz_bits(bytes(false),1165,9)==0) by(compute);
    assert(lz_bits(bytes(true),1165,9)==0) by(compute);
    assert(lz_bits(s,1165,9)==0);
    assert(lz_bits(bytes(false),1174,9)==256) by(compute);
    assert(lz_bits(bytes(true),1174,9)==256) by(compute);
    assert(lz_bits(s,1174,9)==256);
    assert(lz_bits(bytes(false),1183,9)==0) by(compute);
    assert(lz_bits(bytes(true),1183,9)==0) by(compute);
    assert(lz_bits(s,1183,9)==0);
    assert(lz_bits(bytes(false),1192,9)==256) by(compute);
    assert(lz_bits(bytes(true),1192,9)==256) by(compute);
    assert(lz_bits(s,1192,9)==256);
    assert(lz_bits(bytes(false),1201,9)==0) by(compute);
    assert(lz_bits(bytes(true),1201,9)==0) by(compute);
    assert(lz_bits(s,1201,9)==0);
    assert(lz_bits(bytes(false),1210,9)==256) by(compute);
    assert(lz_bits(bytes(true),1210,9)==256) by(compute);
    assert(lz_bits(s,1210,9)==256);
    assert(lz_bits(bytes(false),1219,9)==0) by(compute);
    assert(lz_bits(bytes(true),1219,9)==0) by(compute);
    assert(lz_bits(s,1219,9)==0);
    assert(lz_bits(bytes(false),1228,9)==256) by(compute);
    assert(lz_bits(bytes(true),1228,9)==256) by(compute);
    assert(lz_bits(s,1228,9)==256);
    assert(lz_bits(bytes(false),1237,9)==0) by(compute);
    assert(lz_bits(bytes(true),1237,9)==0) by(compute);
    assert(lz_bits(s,1237,9)==0);
    assert(lz_bits(bytes(false),1246,9)==256) by(compute);
    assert(lz_bits(bytes(true),1246,9)==256) by(compute);
    assert(lz_bits(s,1246,9)==256);
    assert(lz_bits(bytes(false),1255,9)==0) by(compute);
    assert(lz_bits(bytes(true),1255,9)==0) by(compute);
    assert(lz_bits(s,1255,9)==0);
    assert(lz_bits(bytes(false),1264,9)==257) by(compute);
    assert(lz_bits(bytes(true),1264,9)==257) by(compute);
    assert(lz_bits(s,1264,9)==257);
    reveal_with_fuel(lz_valid,3);
    assert(lz_valid(s,1264,160,lz_initial(),0,0,false));
    assert(lz_after(lz_initial(),256,0)==lz_initial());
    assert(lz_valid(s,1255,160,lz_initial(),256,1,false));
    assert(lz_after(lz_initial(),256,0)==lz_initial());
    assert(lz_valid(s,1246,160,lz_initial(),0,1,false));
    assert(lz_valid(s,1237,160,lz_initial(),256,2,false));
    assert(lz_after(lz_initial(),256,0)==lz_initial());
    assert(lz_valid(s,1228,160,lz_initial(),0,2,false));
    assert(lz_valid(s,1219,160,lz_initial(),256,3,false));
    assert(lz_after(lz_initial(),256,0)==lz_initial());
    assert(lz_valid(s,1210,160,lz_initial(),0,3,false));
    assert(lz_valid(s,1201,160,lz_initial(),256,4,false));
    assert(lz_after(lz_initial(),256,0)==lz_initial());
    assert(lz_valid(s,1192,160,lz_initial(),0,4,false));
    assert(lz_valid(s,1183,160,lz_initial(),256,5,false));
    assert(lz_after(lz_initial(),256,0)==lz_initial());
    assert(lz_valid(s,1174,160,lz_initial(),0,5,false));
    assert(lz_valid(s,1165,160,lz_initial(),256,6,false));
    assert(lz_after(lz_initial(),256,0)==lz_initial());
    assert(lz_valid(s,1156,160,lz_initial(),0,6,false));
    assert(lz_valid(s,1147,160,lz_initial(),256,7,false));
    assert(lz_after(lz_initial(),256,2)==lz_initial());
    assert(lz_valid(s,1138,160,lz_initial(),2,7,false));
    assert(lz_valid(s,1129,160,lz_initial(),256,8,false));
    assert(lz_after(lz_initial(),256,255)==lz_initial());
    assert(lz_valid(s,1120,160,lz_initial(),255,8,false));
    assert(lz_valid(s,1111,160,lz_initial(),256,9,false));
    assert(lz_after(lz_initial(),256,255)==lz_initial());
    assert(lz_valid(s,1102,160,lz_initial(),255,9,false));
    assert(lz_valid(s,1093,160,lz_initial(),256,10,false));
    assert(lz_after(lz_initial(),256,255)==lz_initial());
    assert(lz_valid(s,1084,160,lz_initial(),255,10,false));
    assert(lz_valid(s,1075,160,lz_initial(),256,11,false));
    assert(lz_after(lz_initial(),256,255)==lz_initial());
    assert(lz_valid(s,1066,160,lz_initial(),255,11,false));
    assert(lz_valid(s,1057,160,lz_initial(),256,12,false));
    assert(lz_after(lz_initial(),256,255)==lz_initial());
    assert(lz_valid(s,1048,160,lz_initial(),255,12,false));
    assert(lz_valid(s,1039,160,lz_initial(),256,13,false));
    assert(lz_after(lz_initial(),256,255)==lz_initial());
    assert(lz_valid(s,1030,160,lz_initial(),255,13,false));
    assert(lz_valid(s,1021,160,lz_initial(),256,14,false));
    assert(lz_after(lz_initial(),256,255)==lz_initial());
    assert(lz_valid(s,1012,160,lz_initial(),255,14,false));
    assert(lz_valid(s,1003,160,lz_initial(),256,15,false));
    assert(lz_after(lz_initial(),256,255)==lz_initial());
    assert(lz_valid(s,994,160,lz_initial(),255,15,false));
    assert(lz_valid(s,985,160,lz_initial(),256,16,false));
}
proof fn one_segment(g:Geometry)
    requires g.across==1,g.height==1,g.block_height>0,
    ensures segment_count(g)==1,
{
    reveal(segment_count);
    vstd::arithmetic::div_mod::lemma_div_of0(g.block_height as int);
}
proof fn float_profile_has_witness(predict:bool)
    ensures supported_extended_profile(bytes(predict),0,limits()),
        extended_inspection_work_bound(0,limits())==5702,
{
    let s=bytes(predict);let lim=limits();let d=Directory {start:10,count:9,next:0};
    reveal(tag_valid);reveal(tag_model);reveal(field_size);reveal(value_valid);reveal(tag_value);
    assert forall|i:int| 0<=i<9 implies #[trigger] tag_valid(s,10+12*i,true) by {if i==0 {} else if i==1 {} else if i==2 {} else if i==3 {} else if i==4 {} else if i==5 {} else if i==6 {} else if i==7 {} else {assert(i==8);}}
    assert forall|i:int| 0<=i<9 implies #[trigger] word16(s,10+12*i,true)==seq![256int,257int,258int,259int,262int,273int,279int,317int,339int][i] by {if i==0 {} else if i==1 {} else if i==2 {} else if i==3 {} else if i==4 {} else if i==5 {} else if i==6 {} else if i==7 {} else {assert(i==8);}}
    lookup_at(s,d,256,true,0);
    lookup_at(s,d,257,true,1);
    lookup_at(s,d,258,true,2);
    lookup_at(s,d,259,true,3);
    lookup_at(s,d,262,true,4);
    lookup_at(s,d,273,true,5);
    lookup_at(s,d,279,true,6);
    lookup_at(s,d,317,true,7);
    lookup_at(s,d,339,true,8);
    assert forall|id:u16| id!=256 && id!=257 && id!=258 && id!=259 && id!=262 && id!=273 && id!=279 && id!=317 && id!=339 implies #[trigger] lookup(s,d,id,true).is_none() by {
        assert forall|i:int| 0<=i<9 implies #[trigger] word16(s,10+12*i,true)!=id by {if i==0 {} else if i==1 {} else if i==2 {} else if i==3 {} else if i==4 {} else if i==5 {} else if i==6 {} else if i==7 {} else {assert(i==8);}}
        lookup_missing(s,d,id,true);
    }
    reveal(directory_valid);
    assert forall|i:int| 1<=i<9 implies #[trigger] word16(s,10+12*(i-1),true)<#[trigger] word16(s,10+12*i,true) by {if i==0 {} else if i==1 {} else if i==2 {} else if i==3 {} else if i==4 {} else if i==5 {} else if i==6 {} else if i==7 {} else {assert(i==8);}}
    assert(directory_valid(s,8,true,lim));
    reveal(alpha_field_valid);reveal(ext_fields);reveal(ext_bits);
    reveal(scalar_model);reveal(scalar_valid);reveal(bits_model);reveal(array_of);
    assert(lookup(s,d,258,true)==Some(Tag {kind:3,count:1,data:42}));
    assert(lookup(s,d,339,true)==Some(Tag {kind:3,count:1,data:114}));
    assert(array_of(s,Tag {kind:3,count:1,data:42},1,64,true));
    assert(array_of(s,Tag {kind:3,count:1,data:114},1,3,true));
    assert(scalar_valid(s,d,256,true));
    assert(scalar_model(s,d,256,0,true)==2);
    assert(scalar_valid(s,d,257,true));
    assert(scalar_model(s,d,257,0,true)==1);
    assert(scalar_valid(s,d,277,true));
    assert(scalar_model(s,d,277,1,true)==1);
    assert(scalar_valid(s,d,262,true));
    assert(scalar_model(s,d,262,999,true)==1);
    assert(scalar_valid(s,d,259,true));
    assert(scalar_model(s,d,259,1,true)==5);
    assert(scalar_valid(s,d,338,true));
    assert(scalar_model(s,d,338,0,true)==0);
    assert(scalar_valid(s,d,284,true));
    assert(scalar_model(s,d,284,1,true)==1);
    assert(scalar_valid(s,d,274,true));
    assert(scalar_model(s,d,274,1,true)==1);
    assert(scalar_valid(s,d,266,true));
    assert(scalar_model(s,d,266,1,true)==1);
    assert(scalar_valid(s,d,317,true));
    assert(scalar_model(s,d,317,1,true)==if predict {2int} else {1int});
    assert(bits_model(s,d,true)==64);
    assert(alpha_field_valid(s,d,true));
    assert(ext_bits(s,d,1,true));
    assert(0<=2int*1*1<=16);
    assert(ext_fields(s,d,true,lim));
    reveal(layout_model);reveal(raster_layout_profile);
    let l=layout_model(s,d,true);
    assert(l.w==2);assert(l.h==1);assert(l.ch==1);assert(l.bytes==8);
    assert(l.slots==1);assert(!l.separate);assert(l.encoded_width==2);assert(l.plane_channels==8);assert(l.planes==1);
    assert(l.output_len==16);assert(l.plane_len==16);assert(l.scratch_len==16);
    assert(l.valid());assert(raster_layout_profile(s,d,true,lim));
    reveal(geom_model);reveal(window);reveal(segment_count);reveal(geometry_profile);reveal(geometry_windows);reveal(window_valid);
    let g=geom_model(s,d,true,l,0);
    assert(lookup(s,d,324,true).is_none());assert(lookup(s,d,278,true).is_none());
    assert(scalar_model(s,d,278,4294967295,true)==4294967295);
    assert(g.width==2);assert(g.height==1);assert(g.channels==8);assert(g.block_width==2);assert(g.block_height==4294967295);assert(g.across==1);
    assert(lookup(s,d,273,true)==Some(Tag {kind:4,count:1,data:78}));
    assert(lookup(s,d,279,true)==Some(Tag {kind:4,count:1,data:90}));
    one_segment(g);
    assert(segment_count(g)==1);
    assert(g.offsets==(Tag {kind:4,count:1,data:78}));
    assert(g.counts==(Tag {kind:4,count:1,data:90}));
    assert(segment_count(g)==1);
    assert forall|p:int| 0<=p<l.planes implies geometry_windows(s,d,true,l,p) by {assert(p==0);}
    assert(geometry_profile(s,d,true,l,lim));
    stream(predict);
    reveal(segment_valid);assert(segment_valid(s,g,false,0));
    reveal(plane_profile);reveal_with_fuel(codec_work_prefix,2);
    assert(codec_work_prefix(s,g,1)==54);
    assert(plane_profile(s,g));
    reveal(storage_plane_profile);reveal(plane_work);reveal_with_fuel(planes_work,2);reveal(storage_profile);
    assert forall|p:int| 0<=p<l.planes implies storage_plane_profile(s,d,true,l,p,lim) by {assert(p==0);}
    assert(storage_profile(s,d,true,l,lim));
    reveal(supported_extended_profile);
}
}
