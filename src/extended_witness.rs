// Concrete non-vacuity witnesses for each extended representation.
use super::*;
use crate::deflate as df;
use crate::deflate_bits::*;
verus! {
spec fn bytes(mode:u8) -> Seq<u8> {
    Seq::new(200,|i:int| {
        if i==0 { 73u8 } else if i==1 { 73u8 } else if i==2 { 42u8 } else if i==4 { 8u8 } else if i==8 { 11u8 } else if i==11 { 1u8 } else if i==12 { 3u8 } else if i==14 { 1u8 } else if i==18 { 1u8 } else if i==22 { 1u8 } else if i==23 { 1u8 } else if i==24 { 3u8 } else if i==26 { 1u8 } else if i==30 { 1u8 } else if i==34 { 2u8 } else if i==35 { 1u8 } else if i==36 { 3u8 } else if i==38 { if mode==0 { 1u8 } else { if mode==1 { 1u8 } else { if mode==2 { 3u8 } else { if mode==3 { 1u8 } else { 1u8 } } } } } else if i==42 { if mode==0 { 1u8 } else { if mode==1 { 16u8 } else { if mode==2 { 146u8 } else { if mode==3 { 8u8 } else { 1u8 } } } } } else if i==46 { 3u8 } else if i==47 { 1u8 } else if i==48 { 3u8 } else if i==50 { 1u8 } else if i==54 { if mode==0 { 1u8 } else { if mode==1 { 1u8 } else { if mode==2 { 1u8 } else { if mode==3 { 8u8 } else { 8u8 } } } } } else if i==58 { 6u8 } else if i==59 { 1u8 } else if i==60 { 3u8 } else if i==62 { 1u8 } else if i==66 { if mode==0 { 3u8 } else { if mode==1 { 1u8 } else { if mode==2 { 2u8 } else { if mode==3 { 1u8 } else { 1u8 } } } } } else if i==70 { 17u8 } else if i==71 { 1u8 } else if i==72 { 4u8 } else if i==74 { if mode==0 { 1u8 } else { if mode==1 { 1u8 } else { if mode==2 { 3u8 } else { if mode==3 { 1u8 } else { 1u8 } } } } } else if i==78 { if mode==0 { 188u8 } else { if mode==1 { 188u8 } else { if mode==2 { 152u8 } else { if mode==3 { 188u8 } else { 188u8 } } } } } else if i==82 { 21u8 } else if i==83 { 1u8 } else if i==84 { 3u8 } else if i==86 { 1u8 } else if i==90 { if mode==0 { 1u8 } else { if mode==1 { 1u8 } else { if mode==2 { 3u8 } else { if mode==3 { 1u8 } else { 1u8 } } } } } else if i==94 { 22u8 } else if i==95 { 1u8 } else if i==96 { 4u8 } else if i==98 { 1u8 } else if i==102 { 1u8 } else if i==106 { 23u8 } else if i==107 { 1u8 } else if i==108 { 4u8 } else if i==110 { if mode==0 { 1u8 } else { if mode==1 { 1u8 } else { if mode==2 { 3u8 } else { if mode==3 { 1u8 } else { 1u8 } } } } } else if i==114 { if mode==0 { 1u8 } else { if mode==1 { 2u8 } else { if mode==2 { 164u8 } else { if mode==3 { 12u8 } else { 12u8 } } } } } else if i==118 { 28u8 } else if i==119 { 1u8 } else if i==120 { 3u8 } else if i==122 { 1u8 } else if i==126 { if mode==0 { 1u8 } else { if mode==1 { 1u8 } else { if mode==2 { 2u8 } else { if mode==3 { 1u8 } else { 1u8 } } } } } else if i==130 { 64u8 } else if i==131 { 1u8 } else if i==132 { 3u8 } else if i==134 { 6u8 } else if i==138 { 176u8 } else if i==146 { 8u8 } else if i==148 { 8u8 } else if i==150 { 8u8 } else if i==152 { 188u8 } else if i==156 { 189u8 } else if i==160 { 190u8 } else if i==164 { 1u8 } else if i==168 { 1u8 } else if i==172 { 1u8 } else if i==177 { 17u8 } else if i==179 { 34u8 } else if i==181 { 51u8 } else if i==183 { 68u8 } else if i==185 { 85u8 } else if i==187 { 102u8 } else if i==188 { if mode==0 { 33u8 } else { if mode==1 { 33u8 } else { if mode==2 { 33u8 } else { if mode==3 { 120u8 } else { 120u8 } } } } } else if i==189 { if mode==0 { 66u8 } else { if mode==1 { 66u8 } else { if mode==2 { 66u8 } else { if mode==3 { 1u8 } else { 1u8 } } } } } else if i==190 { if mode==0 { 99u8 } else { if mode==1 { 99u8 } else { if mode==2 { 99u8 } else { if mode==3 { 1u8 } else { 1u8 } } } } } else if i==191 { if mode==0 { 0u8 } else { if mode==1 { 0u8 } else { if mode==2 { 0u8 } else { if mode==3 { 1u8 } else { 1u8 } } } } } else if i==193 { if mode==0 { 0u8 } else { if mode==1 { 0u8 } else { if mode==2 { 0u8 } else { if mode==3 { 254u8 } else { 254u8 } } } } } else if i==194 { if mode==0 { 0u8 } else { if mode==1 { 0u8 } else { if mode==2 { 0u8 } else { if mode==3 { 255u8 } else { 255u8 } } } } } else if i==195 { if mode==0 { 0u8 } else { if mode==1 { 0u8 } else { if mode==2 { 0u8 } else { if mode==3 { 65u8 } else { 65u8 } } } } } else if i==197 { if mode==0 { 0u8 } else { if mode==1 { 0u8 } else { if mode==2 { 0u8 } else { if mode==3 { 66u8 } else { 66u8 } } } } } else if i==199 { if mode==0 { 0u8 } else { if mode==1 { 0u8 } else { if mode==2 { 0u8 } else { if mode==3 { 66u8 } else { 66u8 } } } } } else {0u8}
    })
}
spec fn limits() -> Limits {
    Limits {max_input:200,max_dimension:1,max_output:3,max_directories:1,max_tags:11,max_segments:3}
}
proof fn deflated_byte(mode:u8)
    requires mode==3 || mode==4,
    ensures df::result(bytes(mode),188,12,1,162)==Some(seq![65u8]),
{
    let s=bytes(mode);
    reveal(field);
    assert(field(s,1520,1)==1) by(compute);
    assert(field(s,1521,2)==0) by(compute);
    assert(field(s,1528,16)==1) by(compute);
    assert(field(s,1544,16)==65534) by(compute);
    assert(field(s,1560,8)==65) by(compute);
    assert(df::adler1(seq![65u8])==66) by(compute);
    assert(df::adler2(seq![65u8])==66) by(compute);
    let v0=df::initial(s,188,12).unwrap();
    assert(df::initial(s,188,12).is_some());
    let v1=df::Model {pos:1560,mode:1,left:1,last:true,..v0};
    let v2=df::Model {pos:1568,left:0,bytes:seq![65u8],..v1};
    let v3=df::Model {mode:4,..v2};let v4=df::Model {mode:5,..v3};
    reveal(df::transition);
    assert(df::transition(s,196,1,v0)==Some(v1));
    assert(df::transition(s,196,1,v1)==Some(v2)) by {assert(Seq::<u8>::empty().push(65) =~= seq![65u8]);}
    assert(df::transition(s,196,1,v2)==Some(v3));
    assert(df::transition(s,196,1,v3)==Some(v4));
    reveal_with_fuel(df::finish,1);
    assert(df::finish(s,196,1,v4,158)==Some(seq![65u8]));
    assert(df::finish(s,196,1,v3,159)==Some(seq![65u8]));
    assert(df::finish(s,196,1,v2,160)==Some(seq![65u8]));
    assert(df::finish(s,196,1,v1,161)==Some(seq![65u8]));
    assert(df::finish(s,196,1,v0,162)==Some(seq![65u8]));
    reveal(df::result);
}

proof fn witness_directory(mode:u8)
    requires mode<5,
    ensures directory_valid(bytes(mode),8,true,limits()),
        lookup(bytes(mode),Directory {start:10,count:11,next:0},256,true)==Some(tag_model(bytes(mode),10,true)),
        lookup(bytes(mode),Directory {start:10,count:11,next:0},257,true)==Some(tag_model(bytes(mode),22,true)),
        lookup(bytes(mode),Directory {start:10,count:11,next:0},258,true)==Some(tag_model(bytes(mode),34,true)),
        lookup(bytes(mode),Directory {start:10,count:11,next:0},259,true)==Some(tag_model(bytes(mode),46,true)),
        lookup(bytes(mode),Directory {start:10,count:11,next:0},262,true)==Some(tag_model(bytes(mode),58,true)),
        lookup(bytes(mode),Directory {start:10,count:11,next:0},273,true)==Some(tag_model(bytes(mode),70,true)),
        lookup(bytes(mode),Directory {start:10,count:11,next:0},277,true)==Some(tag_model(bytes(mode),82,true)),
        lookup(bytes(mode),Directory {start:10,count:11,next:0},278,true)==Some(tag_model(bytes(mode),94,true)),
        lookup(bytes(mode),Directory {start:10,count:11,next:0},279,true)==Some(tag_model(bytes(mode),106,true)),
        lookup(bytes(mode),Directory {start:10,count:11,next:0},284,true)==Some(tag_model(bytes(mode),118,true)),
        lookup(bytes(mode),Directory {start:10,count:11,next:0},320,true)==Some(tag_model(bytes(mode),130,true)),
        forall|id:u16| id!=256 && id!=257 && id!=258 && id!=259 && id!=262 && id!=273 && id!=277 && id!=278 && id!=279 && id!=284 && id!=320 ==> #[trigger] lookup(bytes(mode),Directory {start:10,count:11,next:0},id,true).is_none(),
{
    let s=bytes(mode);let lim=limits();let d=Directory {start:10,count:11,next:0};
    reveal(tag_valid);reveal(alpha_field_valid);reveal(tag_model);reveal(field_size);reveal(value_valid);reveal(tag_value);
    assert forall|i:int| 0<=i<11 implies #[trigger] tag_valid(s,10+12*i,true) by {
        if i==0 {} else if i==1 {} else if i==2 {} else if i==3 {} else if i==4 {} else if i==5 {} else if i==6 {} else if i==7 {} else if i==8 {} else if i==9 {} else {assert(i==10);}
    }
    assert forall|i:int| 0<=i<11 implies #[trigger] word16(s,10+12*i,true)==seq![256int,257,258,259,262,273,277,278,279,284,320][i] by {
        if i==0 {} else if i==1 {} else if i==2 {} else if i==3 {} else if i==4 {} else if i==5 {} else if i==6 {} else if i==7 {} else if i==8 {} else if i==9 {} else {assert(i==10);}
    }
    lookup_at(s,d,256,true,0);
    lookup_at(s,d,257,true,1);
    lookup_at(s,d,258,true,2);
    lookup_at(s,d,259,true,3);
    lookup_at(s,d,262,true,4);
    lookup_at(s,d,273,true,5);
    lookup_at(s,d,277,true,6);
    lookup_at(s,d,278,true,7);
    lookup_at(s,d,279,true,8);
    lookup_at(s,d,284,true,9);
    lookup_at(s,d,320,true,10);
    assert forall|id:u16| id!=256 && id!=257 && id!=258 && id!=259 && id!=262 && id!=273 && id!=277 && id!=278 && id!=279 && id!=284 && id!=320 implies #[trigger] lookup(s,d,id,true).is_none() by {
        assert forall|i:int| 0<=i<11 implies #[trigger] word16(s,10+12*i,true)!=id by {
            if i==0 {} else if i==1 {} else if i==2 {} else if i==3 {} else if i==4 {} else if i==5 {} else if i==6 {} else if i==7 {} else if i==8 {} else if i==9 {} else {assert(i==10);}
        }
        lookup_missing(s,d,id,true);
    }
    reveal(directory_valid);
    assert forall|i:int| 1<=i<11 implies #[trigger] word16(s,10+12*(i-1),true)<#[trigger] word16(s,10+12*i,true) by {
        if i==1 {} else if i==2 {} else if i==3 {} else if i==4 {} else if i==5 {} else if i==6 {} else if i==7 {} else if i==8 {} else if i==9 {} else {assert(i==10);}
    }
    assert(directory_valid(s,8,true,lim));
}

proof fn witness_fields(mode:u8)
    requires mode<5,
    ensures ext_fields(bytes(mode),Directory {start:10,count:11,next:0},true,limits()),
        scalar_model(bytes(mode),Directory {start:10,count:11,next:0},256,0,true)==(1),
        scalar_model(bytes(mode),Directory {start:10,count:11,next:0},257,0,true)==(1),
        scalar_model(bytes(mode),Directory {start:10,count:11,next:0},277,1,true)==(if mode==2 {3int} else {1int}),
        scalar_model(bytes(mode),Directory {start:10,count:11,next:0},262,999,true)==(if mode==0 {3int} else if mode==2 {2int} else {1int}),
        scalar_model(bytes(mode),Directory {start:10,count:11,next:0},259,1,true)==(if mode>=3 {8int} else {1int}),
        scalar_model(bytes(mode),Directory {start:10,count:11,next:0},338,0,true)==(0),
        scalar_model(bytes(mode),Directory {start:10,count:11,next:0},284,1,true)==(if mode==2 {2int} else {1int}),
        bits_model(bytes(mode),Directory {start:10,count:11,next:0},true)==(if mode==0 || mode==4 {1int} else if mode==1 {16int} else {8int}),
{
    let s=bytes(mode);let lim=limits();let d=Directory {start:10,count:11,next:0};
    witness_directory(mode);
    reveal(tag_model);reveal(field_size);reveal(value_valid);reveal(tag_value);reveal(alpha_field_valid);
    let bit_tag=lookup(s,d,258,true).unwrap();let ch=if mode==2 {3int} else {1int};
    assert(word16(s,36,true)==3) by(compute);
    assert(word32(s,38,true)==if mode==2 {3int} else {1int}) by(compute);
    assert(word32(s,42,true)==if mode==2 {146int} else if mode==0 || mode==4 {1int} else if mode==1 {16int} else {8int});
    assert(field_size(3)==2);
    assert(lookup(s,d,258,true)==Some(tag_model(s,34,true)));
    assert(bit_tag==(Tag {kind:3,count:if mode==2 {3usize} else {1usize},data:if mode==2 {146usize} else {42usize}}));
    let depth=if mode==0 || mode==4 {1int} else if mode==1 {16int} else {8int};
    assert forall|i:int| 0<=i<ch implies value_valid(s,bit_tag,i) && #[trigger] tag_value(s,bit_tag,i,true)==depth by {
        if i==0 {} else if i==1 {} else {assert(i==2);}
    }
    assert(array_of(s,bit_tag,ch,depth,true));
    reveal(ext_fields);reveal(ext_bits);reveal(scalar_model);reveal(scalar_valid);reveal(bits_model);reveal(array_of);
    assert(lookup(s,d,338,true).is_none());
    assert(scalar_model(s,d,338,0,true)==0);
    assert(scalar_model(s,d,262,999,true)==if mode==0 {3int} else if mode==2 {2int} else {1int});
    assert(scalar_model(s,d,277,1,true)==ch);
    assert(bits_model(s,d,true)==depth);
    assert(ext_bits(s,d,ch,true));
    assert(scalar_valid(s,d,256,true));
    assert(scalar_valid(s,d,257,true));
    assert(scalar_valid(s,d,277,true));
    assert(scalar_valid(s,d,262,true));
    assert(scalar_valid(s,d,259,true));
    assert(scalar_valid(s,d,274,true));
    assert(scalar_valid(s,d,284,true));
    assert(scalar_valid(s,d,266,true));
    assert(scalar_valid(s,d,317,true));
    assert(scalar_model(s,d,256,0,true)==1);
    assert(scalar_model(s,d,257,0,true)==1);
    assert(scalar_model(s,d,259,1,true)==if mode>=3 {8int} else {1int});
    assert(scalar_model(s,d,274,1,true)==1);
    assert(scalar_model(s,d,284,1,true)==if mode==2 {2int} else {1int});
    assert(scalar_model(s,d,266,1,true)==1);
    assert(scalar_model(s,d,317,1,true)==1);
    assert(lookup(s,d,339,true).is_none());
    assert(alpha_field_valid(s,d,true));
    let w=scalar_model(s,d,256,0,true);let h=scalar_model(s,d,257,0,true);
    let photo=scalar_model(s,d,262,999,true);
    assert(scalar_valid(s, d, 256, true) && scalar_valid(s, d, 257, true));
    assert(scalar_valid(s, d, 277, true) && scalar_valid(s, d, 262, true));
    assert(0 < w <= lim.max_dimension && 0 < h <= lim.max_dimension);
    assert((1 <= ch <= 4));
    assert((if scalar_model(s,d,338,0,true)==0 {ch==1 || ch==3} else {ch==2 || ch==4}));
    assert(((ch <= 2 && (photo == 0 || photo == 1)) || (ch >= 3 && photo == 2) || (ch == 1 && photo == 3)));
    assert(w==1 && h==1 && 1<=ch<=3 && lim.max_output==3);
    assert(0 <= w * h <= usize::MAX && 0 <= w * h * ch <= lim.max_output) by(nonlinear_arith)
        requires w==1,h==1,1<=ch<=3,lim.max_output==3;
    assert(scalar_valid(s, d, 259, true));
    assert((scalar_model(s, d, 259, 1, true) == 1 || scalar_model(s, d, 259, 1, true) == 32773 || scalar_model(s, d, 259, 1, true) == 5 || scalar_model(s,d,259,1,true)==8 || scalar_model(s,d,259,1,true)==32946));
    assert((scalar_model(s, d, 259, 1, true) != 1 ==> scalar_model(s, d, 266, 1, true) == 1));
    assert(scalar_valid(s, d, 274, true) && scalar_model(s, d, 274, 1, true) == 1);
    assert(scalar_valid(s, d, 284, true) && (scalar_model(s, d, 284, 1, true) == 1 || scalar_model(s, d, 284, 1, true) == 2));
    assert(scalar_valid(s, d, 266, true));
    assert(scalar_model(s, d, 266, 1, true) == 1);
    assert(scalar_valid(s, d, 317, true) && scalar_model(s, d, 317, 1, true) == 1);
    assert(alpha_field_valid(s,d,true));
    assert(ext_bits(s, d, ch, true));
    assert((match lookup(s, d, 339, true) {
        None => true, Some(t) => array_of(s, t, ch, 1, true),
    }));
    assert(lim.max_dimension==1 && lim.max_output==3);
    assert(ext_fields(s,d,true,lim)) by {
        if mode==0 {} else if mode==1 {} else if mode==2 {} else if mode==3 {} else {assert(mode==4);}
    }
}

proof fn extended_profile_has_witness(mode:u8)
    requires mode<5,
    ensures supported_extended_profile(bytes(mode),0,limits()),
        extended_inspection_work_bound(0,limits())==20288,
{
    let s=bytes(mode);let lim=limits();let d=Directory {start:10,count:11,next:0};
    witness_directory(mode);witness_fields(mode);
    reveal(tag_model);reveal(field_size);reveal(value_valid);reveal(tag_value);reveal(alpha_field_valid);
    reveal(scalar_model);reveal(scalar_valid);reveal(bits_model);reveal(array_of);
    assert(field_size(3)==2 && field_size(4)==4);
    reveal(layout_model);
    let l=layout_model(s,d,true);
    assert(word16(s,132,true)==3) by(compute);
    assert(word32(s,134,true)==6) by(compute);
    assert(word32(s,138,true)==176) by(compute);
    assert(tag_model(s,130,true)==(Tag {kind:3,count:6,data:176}));
    assert(lookup(s,d,320,true)==Some(tag_model(s,130,true)));
    assert(lookup(s,d,320,true)==Some(Tag {kind:3,count:6,data:176}));
    assert(l.valid()) by {
        if mode==0 {} else if mode==1 {} else if mode==2 {} else if mode==3 {} else {assert(mode==4);}
    }
    reveal(raster_layout_profile);
    assert(raster_layout_profile(s,d,true,lim));
    reveal(geometry_profile);reveal(geometry_windows);reveal(window_valid);reveal(geom_model);reveal(window);reveal(segment_count);
    assert(lookup(s,d,324,true).is_none());
    assert(word16(s,96,true)==4 && word32(s,98,true)==1 && word32(s,102,true)==1) by(compute);
    assert(tag_model(s,94,true)==(Tag {kind:4,count:1,data:102}));
    assert(scalar_model(s,d,278,4294967295,true)==1);
    assert(l.w==1 && l.h==1 && l.encoded_width==1);
    let g0=geom_model(s,d,true,l,0);
    assert(g0.width==1 && g0.height==1 && g0.block_width==1 && g0.block_height==1 && g0.across==1);
    assert(g0.channels==if mode==1 {2usize} else {1usize});
    assert(segment_count(g0)==1);
    assert(lookup(s,d,273,true)==Some(Tag {kind:4,count:if mode==2 {3usize} else {1usize},data:if mode==2 {152usize} else {78usize}}));
    assert(lookup(s,d,279,true)==Some(Tag {kind:4,count:if mode==2 {3usize} else {1usize},data:if mode==2 {164usize} else {114usize}}));
    assert forall|p:int| 0<=p<l.planes implies geometry_windows(s,d,true,l,p) by {
        if p==0 {} else if p==1 {} else {assert(p==2);}
        reveal(geometry_windows);reveal(window_valid);
        assert(window_valid(lookup(s,d,273,true).unwrap(),p,1));
        assert(window_valid(lookup(s,d,279,true).unwrap(),p,1));
    }
    assert(geometry_profile(s,d,true,l,lim));
    if mode>=3 {deflated_byte(mode);}
    reveal(plane_work);reveal(storage_profile);
    assert forall|p:int| 0<=p<l.planes implies
        storage_plane_profile(s,d,true,l,p,lim) by {
        reveal(storage_plane_profile);
        if p==0 {} else if p==1 {} else {assert(p==2);}
        let g=geom_model(s,d,true,l,p);
        assert(segment_count(g)==1);
        if mode>=3 {
            reveal(dp::profile);reveal_with_fuel(dp::work_prefix,2);
            assert(dp::decoded(s,g,0)==Some(seq![65u8]));
            assert(dp::segment_ok(s,g,0));
        } else {
            reveal(plane_profile);reveal(segment_valid);reveal_with_fuel(codec_work_prefix,2);
            assert(segment_valid(s,g,false,0));
        }
    }
    reveal_with_fuel(planes_work,4);
    reveal_with_fuel(codec_work_prefix,2);reveal_with_fuel(dp::work_prefix,2);
    assert(storage_profile(s,d,true,l,lim));
    reveal(supported_extended_profile);
}
}
