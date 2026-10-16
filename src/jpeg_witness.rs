// Concrete whole-profile non-vacuity witness, mirrored in tests/jpeg_witness.py.
use super::*;
use crate::{jpeg_meta as m,jpeg_format as f,jpeg_header as h,jpeg_huffman as hf,jpeg_bits as bits,jpeg_block as b,jpeg_scan as scan,jpeg_segment as seg,jpeg_container as c,jpeg_tiff as t,jpeg as j};
verus! {
pub(super) open spec fn bytes() -> Seq<u8> {Seq::new(271,|i:int| if i==0 {73u8} else if i==1 {73u8} else if i==2 {42u8} else if i==4 {8u8} else if i==8 {10u8} else if i==11 {1u8} else if i==12 {3u8} else if i==14 {1u8} else if i==18 {1u8} else if i==22 {1u8} else if i==23 {1u8} else if i==24 {3u8} else if i==26 {1u8} else if i==30 {1u8} else if i==34 {2u8} else if i==35 {1u8} else if i==36 {3u8} else if i==38 {1u8} else if i==42 {8u8} else if i==46 {3u8} else if i==47 {1u8} else if i==48 {3u8} else if i==50 {1u8} else if i==54 {7u8} else if i==58 {6u8} else if i==59 {1u8} else if i==60 {3u8} else if i==62 {1u8} else if i==66 {1u8} else if i==70 {17u8} else if i==71 {1u8} else if i==72 {4u8} else if i==74 {1u8} else if i==78 {134u8} else if i==82 {21u8} else if i==83 {1u8} else if i==84 {3u8} else if i==86 {1u8} else if i==90 {1u8} else if i==94 {22u8} else if i==95 {1u8} else if i==96 {4u8} else if i==98 {1u8} else if i==102 {1u8} else if i==106 {23u8} else if i==107 {1u8} else if i==108 {4u8} else if i==110 {1u8} else if i==114 {137u8} else if i==118 {28u8} else if i==119 {1u8} else if i==120 {3u8} else if i==122 {1u8} else if i==126 {1u8} else if i==134 {255u8} else if i==135 {216u8} else if i==136 {255u8} else if i==137 {219u8} else if i==139 {67u8} else if i==205 {255u8} else if i==206 {192u8} else if i==208 {11u8} else if i==209 {8u8} else if i==211 {1u8} else if i==213 {1u8} else if i==214 {1u8} else if i==215 {1u8} else if i==216 {17u8} else if i==218 {255u8} else if i==219 {196u8} else if i==221 {38u8} else if i==223 {1u8} else if i==240 {16u8} else if i==241 {1u8} else if i==258 {255u8} else if i==259 {218u8} else if i==261 {8u8} else if i==262 {1u8} else if i==263 {1u8} else if i==266 {63u8} else if i==268 {63u8} else if i==269 {255u8} else if i==270 {217u8} else if 141<=i<205 {1u8} else {0u8})}
pub(super) open spec fn limits() -> Limits {Limits {max_input:271,max_dimension:1,max_output:65,max_tags:10,max_segments:1,max_directories:1}}
spec fn zero() -> Seq<usize> {Seq::new(32,|i:int| 0usize)}
spec fn config() -> Seq<usize> {zero().update(0,141).update(4,223).update(8,241).update(12,1).update(13,1).update(14,1).update(15,1).update(16,1).update(17,1).update(30,1).update(31,1)}
spec fn cursor() -> bits::Bits {bits::Bits {at:268,end:271,byte:0,left:0}}
spec fn metadata() -> m::Meta {m::Meta {tags:[
Tag {kind:3,count:1,data:18},Tag {kind:3,count:1,data:30},Tag {kind:3,count:1,data:42},Tag {kind:3,count:1,data:54},Tag {kind:3,count:1,data:66},Tag {kind:0,count:0,data:0},Tag {kind:4,count:1,data:78},Tag {kind:0,count:0,data:0},Tag {kind:3,count:1,data:90},Tag {kind:4,count:1,data:102},Tag {kind:4,count:1,data:114},Tag {kind:3,count:1,data:126},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0}],count:10,next:0,off:8,le:true}}
proof fn directory()
    ensures metadata().matches(bytes()),m::directory_ok(bytes(),8,true,10),f::fields(bytes(),metadata(),limits()),
        f::model(bytes(),metadata()).w==1,f::model(bytes(),metadata()).h==1,f::model(bytes(),metadata()).segments==1,
        f::model(bytes(),metadata()).compression==7,
{
    let s=bytes();reveal(m::entry);reveal(tag_valid);reveal(tag_model);reveal(field_size);reveal(tag_value);reveal(value_valid);
    reveal_with_fuel(m::records,11);
    assert(m::records(s,10,true,10)==Some(metadata().tags@));
    assert(metadata().matches(s));
    reveal(f::fields);
    assert forall|i:int| 0<=i<32 && f::is_scalar(i) implies m::number(s,metadata().tags@[i],f::default(i),true).is_some() by {
        if i==0 {} else if i==1 {} else if i==3 {} else if i==4 {} else if i==5 {} else if i==7 {} else if i==8 {} else if i==9 {} else if i==11 {} else if i==12 {} else if i==13 {} else if i==14 {} else if i==20 {} else if i==21 {} else if i==22 {} else if i==23 {} else {assert(i==29);}
    }
    assert(f::numbers(s,metadata()) =~= seq![1usize,1,0,7,1,1,0,1,1,1,0,1,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,0]);
    assert(f::model(s,metadata()).valid());
    assert(f::uniform(s,metadata().tags@[2],1,8,true,false));
    assert(f::fields(s,metadata(),limits()));
}
proof fn tables()
    ensures h::quant_ok(bytes(),141,205),hf::table_ok(bytes(),223,258),hf::table_ok(bytes(),241,258),
        hf::count(bytes(),223,16)==1,hf::count(bytes(),241,16)==1,
{
    let s=bytes();
    assert forall|i:int| 0<=i<64 implies #[trigger] s[141+i]>0 by {}
    assert(hf::count(s,223,16)==1) by(compute);assert(hf::count(s,241,16)==1) by(compute);
    assert(hf::first(s,223,1)==0) by(compute);
    assert(hf::first(s,223,2)==2) by(compute);
    assert(hf::first(s,223,3)==4) by(compute);
    assert(hf::first(s,223,4)==8) by(compute);
    assert(hf::first(s,223,5)==16) by(compute);
    assert(hf::first(s,223,6)==32) by(compute);
    assert(hf::first(s,223,7)==64) by(compute);
    assert(hf::first(s,223,8)==128) by(compute);
    assert(hf::first(s,223,9)==256) by(compute);
    assert(hf::first(s,223,10)==512) by(compute);
    assert(hf::first(s,223,11)==1024) by(compute);
    assert(hf::first(s,223,12)==2048) by(compute);
    assert(hf::first(s,223,13)==4096) by(compute);
    assert(hf::first(s,223,14)==8192) by(compute);
    assert(hf::first(s,223,15)==16384) by(compute);
    assert(hf::first(s,223,16)==32768) by(compute);
    assert forall|n:nat| 1<=n<=16 implies 0<=#[trigger] hf::first(s,223,n) && hf::first(s,223,n)+s[223+n-1]<crate::deflate_bits::pow(n as int) by {
        if n==1 {} else if n==2 {} else if n==3 {} else if n==4 {} else if n==5 {} else if n==6 {} else if n==7 {} else if n==8 {} else if n==9 {} else if n==10 {} else if n==11 {} else if n==12 {} else if n==13 {} else if n==14 {} else if n==15 {} else {assert(n==16);}
    }
    assert(hf::first(s,241,1)==0) by(compute);
    assert(hf::first(s,241,2)==2) by(compute);
    assert(hf::first(s,241,3)==4) by(compute);
    assert(hf::first(s,241,4)==8) by(compute);
    assert(hf::first(s,241,5)==16) by(compute);
    assert(hf::first(s,241,6)==32) by(compute);
    assert(hf::first(s,241,7)==64) by(compute);
    assert(hf::first(s,241,8)==128) by(compute);
    assert(hf::first(s,241,9)==256) by(compute);
    assert(hf::first(s,241,10)==512) by(compute);
    assert(hf::first(s,241,11)==1024) by(compute);
    assert(hf::first(s,241,12)==2048) by(compute);
    assert(hf::first(s,241,13)==4096) by(compute);
    assert(hf::first(s,241,14)==8192) by(compute);
    assert(hf::first(s,241,15)==16384) by(compute);
    assert(hf::first(s,241,16)==32768) by(compute);
    assert forall|n:nat| 1<=n<=16 implies 0<=#[trigger] hf::first(s,241,n) && hf::first(s,241,n)+s[241+n-1]<crate::deflate_bits::pow(n as int) by {
        if n==1 {} else if n==2 {} else if n==3 {} else if n==4 {} else if n==5 {} else if n==6 {} else if n==7 {} else if n==8 {} else if n==9 {} else if n==10 {} else if n==11 {} else if n==12 {} else if n==13 {} else if n==14 {} else if n==15 {} else {assert(n==16);}
    }

}
proof fn header()
    ensures h::finish(bytes(),h::Model {v:zero(),at:136,end:271,sub:271,kind:0},275)==Some(h::Model {v:config(),at:268,end:271,sub:258,kind:0}),
{
    let s=bytes();tables();reveal(h::step);reveal(h::frame);reveal(h::scan);
    let a=h::Model {v:zero(),at:136,end:271,sub:271,kind:0};
    let b=h::Model {at:140,sub:205,kind:1,..a};
    let d=h::Model {at:205,v:zero().update(0,141),..b};
    let e=h::Model {kind:0,..d};
    let fv=d.v.update(12,1).update(13,1).update(14,1).update(15,1).update(16,1).update(17,1).update(30,1);
    let g=h::Model {at:218,v:fv,..e};
    let i=h::Model {at:222,sub:258,kind:2,..g};
    let k=h::Model {at:240,v:fv.update(4,223),..i};
    let l=h::Model {at:258,v:fv.update(4,223).update(8,241),..k};
    let n=h::Model {kind:0,..l};
    let o=h::Model {at:268,v:config(),..n};
    assert(h::step(s,a)==Some(b));assert(h::step(s,b)==Some(d));assert(h::step(s,d)==Some(e));
    assert(h::frame_ok(fv));
    assert(h::frame(s,209,218,e.v).is_some());
    assert(h::frame(s,209,218,e.v).unwrap() =~= fv);
    assert(h::step(s,e)==Some(g));assert(h::step(s,g)==Some(i));assert(h::step(s,i)==Some(k));assert(h::step(s,k)==Some(l));assert(h::step(s,l)==Some(n));
    assert(h::scan_ok(s,config()));
    assert(h::scan(s,262,268,n.v).is_some());
    assert(h::scan(s,262,268,n.v).unwrap() =~= config());
    assert(h::step(s,n)==Some(o));
    reveal_with_fuel(h::finish,1);
    assert(h::finish(s,o,266)==Some(o));assert(h::finish(s,n,267)==Some(o));assert(h::finish(s,l,268)==Some(o));assert(h::finish(s,k,269)==Some(o));assert(h::finish(s,i,270)==Some(o));assert(h::finish(s,g,271)==Some(o));assert(h::finish(s,e,272)==Some(o));assert(h::finish(s,d,273)==Some(o));assert(h::finish(s,b,274)==Some(o));assert(h::finish(s,a,275)==Some(o));
}
proof fn entropy()
    ensures scan::finish(bytes(),config(),scan::initial(cursor()),1,false).is_some(),
{
    let s=bytes();let z=Seq::new(64,|i:int| 0i64);let dc=hf::Huff {base:223};let ac=hf::Huff {base:241};
    let x=cursor();let y=bits::Bits {at:269,byte:63,left:7,..x};let end=bits::Bits {left:6,..y};
    reveal(hf::symbol);reveal(bits::one);reveal(bits::field);reveal(b::step);
    assert(bits::one(s,x)==Some((0usize,y)));
    assert(hf::symbol(s,x,223,1,0,0,0)==Some((0u8,y)));
    assert(bits::one(s,y)==Some((0usize,end)));
    assert(hf::symbol(s,y,241,1,0,0,0)==Some((0u8,end)));
    let a=b::initial(x,0);let d=b::Model {bits:y,k:1,..a};let e=b::Model {bits:end,done:true,..d};
    assert(z.update(0,0)==z);
    assert(b::step(s,141,dc,ac,a)==Some(d));assert(b::step(s,141,dc,ac,d)==Some(e));
    reveal_with_fuel(b::finish,3);assert(b::finish(s,141,dc,ac,a,65)==Some(e));
    reveal(scan::step);reveal(scan::restart);reveal_with_fuel(scan::finish,2);
    assert(scan::terminated(s,end,false));
    assert(scan::step(s,config(),scan::initial(x)).is_some());
}
proof fn parts()
    ensures metadata().matches(bytes()),m::directory_ok(bytes(),8,true,10),
        f::fields(bytes(),metadata(),limits()),
        c::base_model(bytes(),f::model(bytes(),metadata()))==Some(zero()),
        f::model(bytes(),metadata()).segments==1,
        t::segment_supported(bytes(),f::model(bytes(),metadata()),zero(),0,limits()),
        t::work_prefix(bytes(),f::model(bytes(),metadata()),zero(),1)==3413,
{
    directory();header();entropy();let s=bytes();let f=f::model(s,metadata());
    reveal(tag_value);reveal(value_valid);reveal(field_size);reveal(c::base_model);reveal(seg::header_model);reveal(c::header_model);reveal(seg::plan);
    assert(seg::adjusted(f,zero(),0).update(30,0)==zero());
    assert(zero().update(31,0)==zero());
    assert(tag_value(s,f.offsets,0,true)==134 && tag_value(s,f.counts,0,true)==137);
    assert(value_valid(s,f.offsets,0) && value_valid(s,f.counts,0));
    assert(j::start(s,134,271,zero())==Some(h::Model {v:zero(),at:136,end:271,sub:271,kind:0}));
    assert(c::header_model(s,134,137,zero())==Some(h::Model {v:config(),at:268,end:271,sub:258,kind:0}));
    assert(seg::header_model(s,f,zero(),0)==Some(h::Model {v:config(),at:268,end:271,sub:258,kind:0}));
    assert(seg::plan(s,f,zero(),0)==Some(seg::Model {v:config(),bits:cursor()}));
    reveal(j::eligible);reveal(t::segment_supported);reveal_with_fuel(t::work_prefix,2);
}
pub(super) proof fn witness() ensures supported_jpeg_profile(bytes(),0,limits()), {
    parts();t::profile_from_parts(bytes(),metadata(),limits());
}
}
