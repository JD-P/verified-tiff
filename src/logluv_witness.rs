// Whole-TIFF witness mirrors the unchanged 1x1 upstream LogLuv fixture.
use super::*;
use crate::{fax_meta as m,logluv_format as f,logluv as c,logluv_segment as seg,logluv_tiff as t};
verus! {
pub(super) open spec fn bytes() -> Seq<u8> {Seq::new(166,|i:int| if i==0 {73u8} else if i==1 {73u8} else if i==2 {42u8} else if i==4 {16u8} else if i==8 {1u8} else if i==10 {1u8} else if i==12 {1u8} else if i==13 {86u8} else if i==14 {1u8} else if i==15 {194u8} else if i==16 {11u8} else if i==19 {1u8} else if i==20 {3u8} else if i==22 {1u8} else if i==26 {1u8} else if i==30 {1u8} else if i==31 {1u8} else if i==32 {3u8} else if i==34 {1u8} else if i==38 {1u8} else if i==42 {2u8} else if i==43 {1u8} else if i==44 {3u8} else if i==46 {3u8} else if i==50 {154u8} else if i==54 {3u8} else if i==55 {1u8} else if i==56 {3u8} else if i==58 {1u8} else if i==62 {116u8} else if i==63 {135u8} else if i==66 {6u8} else if i==67 {1u8} else if i==68 {3u8} else if i==70 {1u8} else if i==74 {77u8} else if i==75 {128u8} else if i==78 {17u8} else if i==79 {1u8} else if i==80 {4u8} else if i==82 {1u8} else if i==86 {8u8} else if i==90 {21u8} else if i==91 {1u8} else if i==92 {3u8} else if i==94 {1u8} else if i==98 {3u8} else if i==102 {22u8} else if i==103 {1u8} else if i==104 {3u8} else if i==106 {1u8} else if i==110 {170u8} else if i==111 {2u8} else if i==114 {23u8} else if i==115 {1u8} else if i==116 {4u8} else if i==118 {1u8} else if i==122 {8u8} else if i==126 {28u8} else if i==127 {1u8} else if i==128 {3u8} else if i==130 {1u8} else if i==134 {1u8} else if i==138 {83u8} else if i==139 {1u8} else if i==140 {3u8} else if i==142 {3u8} else if i==146 {160u8} else if i==154 {16u8} else if i==156 {16u8} else if i==158 {16u8} else if i==160 {2u8} else if i==162 {2u8} else if i==164 {2u8} else {0u8})}
pub(super) open spec fn limits() -> Limits {Limits {max_input:166,max_output:10,max_dimension:1,max_tags:11,max_segments:1,max_directories:1}}
spec fn metadata() -> m::Meta {m::Meta {tags:[Tag {kind:3,count:1,data:26},Tag {kind:3,count:1,data:38},Tag {kind:3,count:3,data:154},Tag {kind:3,count:1,data:62},Tag {kind:3,count:1,data:74},Tag {kind:0,count:0,data:0},Tag {kind:4,count:1,data:86},Tag {kind:0,count:0,data:0},Tag {kind:3,count:1,data:98},Tag {kind:3,count:1,data:110},Tag {kind:4,count:1,data:122},Tag {kind:3,count:1,data:134},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:3,count:3,data:160},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0}],count:11,next:0,off:16,le:true}}
proof fn directory()
    ensures metadata().matches(bytes()),m::directory_ok(bytes(),16,true,11),f::fields(bytes(),metadata(),limits()),
        f::model(bytes(),metadata()).w==1,f::model(bytes(),metadata()).h==1,f::model(bytes(),metadata()).segments==1,
{
    let s=bytes();reveal(m::entry);reveal(tag_valid);reveal(tag_model);reveal(field_size);reveal(tag_value);reveal(value_valid);
    reveal_with_fuel(m::records,12);
    assert(m::records(s,18,true,11)==Some(metadata().tags@));
    reveal(f::fields);
    assert forall|i:int| 0<=i<32 && f::is_scalar(i) implies m::number(s,metadata().tags@[i],f::default(i),true).is_some() by {
        if i==0 {} else if i==1 {} else if i==3 {} else if i==4 {} else if i==5 {} else if i==7 {} else if i==8 {} else if i==9 {} else if i==11 {} else if i==12 {} else if i==13 {} else if i==14 {} else {assert(i==17);}
    }
    assert(f::numbers(s,metadata()) =~= seq![1usize,1,0,34676,32845,1,0,1,3,682,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    assert(f::uniform(s,metadata().tags@[2],3,16,true,false));
    assert(f::uniform(s,metadata().tags@[18],3,2,true,false));
    let fmt=f::model(s,metadata());assert(fmt.w==1 && fmt.h==1 && fmt.bw==1 && fmt.bh==1);
    let v=f::numbers(s,metadata());assert(v[0]==1 && v[1]==1 && v[9]==682);
    assert(metadata().tags@[15].kind==0);
    assert((v[0] as int)*(v[1] as int)*6==6) by(nonlinear_arith) requires v[0]==1,v[1]==1;
    assert(fmt.output==6);assert(fmt.alpha==0);
    assert(fmt.across==1);
    assert(fmt.segments==1) by(nonlinear_arith)
        requires fmt.segments==((((v[0] as int-1)/(v[0] as int)+1)*((v[1] as int-1)/(v[1] as int)+1)) as usize),v[0]==1,v[1]==1;
    assert(f::fields(s,metadata(),limits()));
}
pub(super) proof fn witness()
    ensures supported_logluv_profile(bytes(),0,limits()),logluv_inspection_work_bound(0,limits())==327,
{
    assert(logluv_inspection_work_bound(0,limits())==327) by(compute);
    directory();let s=bytes();let fmt=f::model(s,metadata());
    reveal(t::segment_supported);reveal(seg::plan);reveal(tag_value);reveal(value_valid);
    reveal_with_fuel(c::sg_valid,5);
    assert(c::sg_valid(s,8,16,1,4));
    assert(seg::plan(s,fmt,0,limits())==Some(seg::Plan {off:8,count:8,raw:4,pixels:10,bytes:6,work:29}));
    assert(seg::accepted(s,fmt,0,limits()));
    assert(t::segment_supported(s,fmt,0,limits()));
    reveal_with_fuel(t::work_prefix,2);
    assert(t::work_prefix(s,fmt,1,limits())==29);
    t::profile_from_parts(s,metadata(),limits());
}
}
