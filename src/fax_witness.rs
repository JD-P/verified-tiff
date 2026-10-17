// Concrete Group 3 and Group 4 whole-profile non-vacuity witnesses.
use super::*;
use crate::{fax_meta as m,fax_format as f,fax_bits as b,fax_row as r,fax_frame as e,fax as c,fax_segment as seg,fax_tiff as t};
verus! {
pub(super) open spec fn bytes(g4:bool) -> Seq<u8> {Seq::new(101,|i:int| if i==0 {73u8} else if i==1 {73u8} else if i==2 {42u8} else if i==4 {8u8} else if i==8 {7u8} else if i==11 {1u8} else if i==12 {3u8} else if i==14 {1u8} else if i==18 {1u8} else if i==22 {1u8} else if i==23 {1u8} else if i==24 {3u8} else if i==26 {1u8} else if i==30 {1u8} else if i==34 {3u8} else if i==35 {1u8} else if i==36 {3u8} else if i==38 {1u8} else if i==42 {if g4 {4u8} else {3u8}} else if i==46 {6u8} else if i==47 {1u8} else if i==48 {3u8} else if i==50 {1u8} else if i==58 {17u8} else if i==59 {1u8} else if i==60 {4u8} else if i==62 {1u8} else if i==66 {98u8} else if i==70 {22u8} else if i==71 {1u8} else if i==72 {4u8} else if i==74 {1u8} else if i==78 {1u8} else if i==82 {23u8} else if i==83 {1u8} else if i==84 {4u8} else if i==86 {1u8} else if i==90 {if g4 {1u8} else {3u8}} else if i==98 {if g4 {128u8} else {0u8}} else if i==99 {if g4 {0u8} else {17u8}} else if i==100 {if g4 {0u8} else {192u8}} else {0u8})}
pub(super) open spec fn limits() -> Limits {Limits {max_input:101,max_output:1,max_dimension:1,max_tags:7,max_segments:1,max_directories:1}}
spec fn metadata() -> m::Meta {m::Meta {tags:[Tag {kind:3,count:1,data:18},Tag {kind:3,count:1,data:30},Tag {kind:0,count:0,data:0},Tag {kind:3,count:1,data:42},Tag {kind:3,count:1,data:54},Tag {kind:0,count:0,data:0},Tag {kind:4,count:1,data:66},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:4,count:1,data:78},Tag {kind:4,count:1,data:90},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0} ],count:7,next:0,off:8,le:true}}
proof fn directory(g4:bool)
    ensures metadata().matches(bytes(g4)),m::directory_ok(bytes(g4),8,true,7),f::fields(bytes(g4),metadata(),limits()),
        f::model(bytes(g4),metadata()).w==1,f::model(bytes(g4),metadata()).h==1,f::model(bytes(g4),metadata()).segments==1,
        f::model(bytes(g4),metadata()).compression==if g4 {4usize} else {3usize},
{
    let s=bytes(g4);reveal(m::entry);reveal(tag_valid);reveal(tag_model);reveal(field_size);reveal(tag_value);reveal(value_valid);
    reveal_with_fuel(m::records,8);
    assert(m::records(s,10,true,7)==Some(metadata().tags@));
    reveal(f::fields);
    assert forall|i:int| 0<=i<32 && f::is_scalar(i) implies m::number(s,metadata().tags@[i],f::default(i),true).is_some() by {
        if i==0 {} else if i==1 {} else if i==3 {} else if i==4 {} else if i==5 {} else if i==7 {} else if i==8 {} else if i==9 {} else if i==11 {} else if i==12 {} else if i==13 {} else if i==14 {} else if i==19 {} else {assert(i==20);}
    }
    assert(f::numbers(s,metadata()) =~= seq![1usize,1,0,if g4 {4usize} else {3usize},0,1,0,1,1,1,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    let fmt=f::model(s,metadata());
    assert(fmt.w==1 && fmt.h==1 && fmt.bw==1 && fmt.bh==1);
    let v=f::numbers(s,metadata());let aa=(v[0] as int-1)/(v[0] as int)+1;
    let hh=if v[9]<v[1] {v[9]} else {v[1]};let dd=(v[1] as int-1)/(hh as int)+1;
    assert(aa==1 && dd==1);
    assert(fmt.segments==(aa*dd) as usize);
    assert(aa*dd==1) by(nonlinear_arith) requires aa==1,dd==1;
    assert(fmt.across==1 && fmt.segments==1 && fmt.output==1);
    assert(fmt.photo==0 && fmt.compression==if g4 {4usize} else {3usize});
    assert(fmt.valid()) by {if g4 {} else {}}
    assert(f::fields(s,metadata(),limits()));
}
proof fn stream(g4:bool)
    ensures seg::accepted(bytes(g4),f::model(bytes(g4),metadata()),0,limits()),
{
    directory(g4);let s=bytes(g4);let f=f::model(s,metadata());
    reveal(tag_value);reveal(value_valid);reveal(seg::plan);
    let bits=b::Bits {pos:784,end:if g4 {792usize} else {808usize},lsb:0usize};
    let config=e::Config {group4:g4,mixed:false,aligned:false,missing:false};
    reveal(e::zero_bits);reveal(e::eol);reveal(b::one);
    if !g4 {assert(e::zero_bits(bytes(false),b::Bits {pos:784,end:808,lsb:0usize},11)) by(compute);}
    assert(seg::plan(s,f,0,limits()).is_some());
    assert(seg::plan(s,f,0,limits()).unwrap().bits==bits && seg::plan(s,f,0,limits()).unwrap().config==config);
    reveal(c::step);reveal(r::step);reveal(r::mode);reveal(b::run);reveal(b::symbol);
    let start=if g4 {bits} else {b::Bits {pos:796,..bits}};
    let end=b::Bits {pos:if g4 {785usize} else {802usize},..bits};
    if g4 {assert(e::line(s,bits,config)==Some((start,true)));assert(r::mode(s,start,0,0,7)==Some((0i8,end)));}
    else {
        assert(e::eol(bytes(false),b::Bits {pos:784,end:808,lsb:0usize},0,19)==Some(b::Bits {pos:796,end:808,lsb:0usize})) by(compute);
        assert(e::line(s,bits,config)==Some((start,false)));
        assert(b::symbol(bytes(false),b::Bits {pos:796,end:808,lsb:0usize},crate::fax_tables::white(),0,13)==Some((1usize,b::Bits {pos:802,end:808,lsb:0usize}))) by(compute);
        assert(b::run(s,start,false,1,0,1026)==Some((1usize,end)));
    }
    let row=r::Model {bits:end,a:1,bi:if g4 {2usize} else {1usize},ends:seq![1u32]};
    assert(r::step(s,seq![1u32],1,g4,r::begin(start))==Some(row));
    reveal_with_fuel(r::finish,2);
    assert(r::finish(s,seq![1u32],1,g4,r::begin(start),(start.end-start.pos+1) as nat)==Some(row));
    let out=c::Model {bits:end,reference:seq![1u32],history:seq![seq![1u32]]};
    assert(c::step(s,config,1,c::initial(bits,1))==Some(out));
    if g4 {assert(e::zero_bits(bytes(true),b::Bits {pos:785,end:792,lsb:0usize},7)) by(compute);} else {assert(e::zero_bits(bytes(false),b::Bits {pos:802,end:808,lsb:0usize},6)) by(compute);}
    assert(e::terminated(s,end,config));reveal_with_fuel(c::finish,2);
    assert(c::finish(s,config,1,c::initial(bits,1),1)==Some(out));
}
pub(super) proof fn witness(g4:bool) ensures supported_fax_profile(bytes(g4),0,limits()), {
    directory(g4);stream(g4);let s=bytes(g4);let f=f::model(s,metadata());
    reveal(t::segment_supported);reveal(seg::plan);reveal(tag_value);reveal(value_valid);reveal_with_fuel(t::work_prefix,2);
    assert(t::segment_supported(s,f,0,limits()));
    assert(0<=t::work_prefix(s,f,1,limits())<=usize::MAX);
    t::profile_from_parts(s,metadata(),limits());
}
}
