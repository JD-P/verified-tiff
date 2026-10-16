// Concrete whole-TIFF profile witness: one opaque black RGBA pixel.
use super::*;
use crate::{fax_meta as m,webp_format as f,webp_container as c,webp_header as hd,webp_huffman as hf,webp_token as tok,webp as w,webp_segment as seg,webp_tiff as t,deflate_bits as db};
verus! {
pub(super) open spec fn bytes() -> Seq<u8> {Seq::new(160,|i:int| if i==0 {73u8} else if i==1 {73u8} else if i==2 {42u8} else if i==4 {8u8} else if i==8 {9u8} else if i==11 {1u8} else if i==12 {3u8} else if i==14 {1u8} else if i==18 {1u8} else if i==22 {1u8} else if i==23 {1u8} else if i==24 {3u8} else if i==26 {1u8} else if i==30 {1u8} else if i==34 {2u8} else if i==35 {1u8} else if i==36 {3u8} else if i==38 {4u8} else if i==42 {122u8} else if i==46 {3u8} else if i==47 {1u8} else if i==48 {3u8} else if i==50 {1u8} else if i==54 {81u8} else if i==55 {195u8} else if i==58 {6u8} else if i==59 {1u8} else if i==60 {3u8} else if i==62 {1u8} else if i==66 {2u8} else if i==70 {17u8} else if i==71 {1u8} else if i==72 {4u8} else if i==74 {1u8} else if i==78 {130u8} else if i==82 {21u8} else if i==83 {1u8} else if i==84 {3u8} else if i==86 {1u8} else if i==90 {4u8} else if i==94 {23u8} else if i==95 {1u8} else if i==96 {4u8} else if i==98 {1u8} else if i==102 {30u8} else if i==106 {82u8} else if i==107 {1u8} else if i==108 {3u8} else if i==110 {1u8} else if i==114 {2u8} else if i==122 {8u8} else if i==124 {8u8} else if i==126 {8u8} else if i==128 {8u8} else if i==130 {82u8} else if i==131 {73u8} else if i==132 {70u8} else if i==133 {70u8} else if i==134 {22u8} else if i==138 {87u8} else if i==139 {69u8} else if i==140 {66u8} else if i==141 {80u8} else if i==142 {86u8} else if i==143 {80u8} else if i==144 {56u8} else if i==145 {76u8} else if i==146 {9u8} else if i==150 {47u8} else if i==155 {136u8} else if i==156 {136u8} else if i==157 {254u8} else if i==158 {7u8} else {0u8})}
pub(super) open spec fn limits() -> Limits {Limits {max_input:160,max_output:4,max_dimension:1,max_tags:9,max_segments:1,max_directories:1}}
spec fn metadata() -> m::Meta {m::Meta {tags:[Tag {kind:3,count:1,data:18},Tag {kind:3,count:1,data:30},Tag {kind:3,count:4,data:122},Tag {kind:3,count:1,data:54},Tag {kind:3,count:1,data:66},Tag {kind:0,count:0,data:0},Tag {kind:4,count:1,data:78},Tag {kind:0,count:0,data:0},Tag {kind:3,count:1,data:90},Tag {kind:0,count:0,data:0},Tag {kind:4,count:1,data:102},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:3,count:1,data:114},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0},Tag {kind:0,count:0,data:0}],count:9,next:0,off:8,le:true}}
proof fn directory()
    ensures metadata().matches(bytes()),m::directory_ok(bytes(),8,true,9),f::fields(bytes(),metadata(),limits()),
        f::model(bytes(),metadata()).w==1,f::model(bytes(),metadata()).h==1,f::model(bytes(),metadata()).segments==1,
{
    let s=bytes();reveal(m::entry);reveal(tag_valid);reveal(tag_model);reveal(field_size);reveal(tag_value);reveal(value_valid);
    reveal_with_fuel(m::records,10);
    assert(m::records(s,10,true,9)==Some(metadata().tags@));
    reveal(f::fields);
    assert forall|i:int| 0<=i<32 && f::is_scalar(i) implies m::number(s,metadata().tags@[i],f::default(i),true).is_some() by {
        if i==0 {} else if i==1 {} else if i==3 {} else if i==4 {} else if i==5 {} else if i==7 {} else if i==8 {} else if i==9 {} else if i==11 {} else if i==12 {} else if i==13 {} else if i==14 {} else {assert(i==17);}
    }
    assert(f::numbers(s,metadata()) =~= seq![1usize,1,0,50001,2,1,0,1,4,4294967295,0,1,1,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    assert(f::uniform(s,metadata().tags@[2],4,8,true,false));
    let fmt=f::model(s,metadata());assert(fmt.w==1 && fmt.h==1 && fmt.bw==1 && fmt.bh==1);
    assert(fmt.across==1 && fmt.segments==1 && fmt.output==4 && fmt.alpha==2);
    assert(f::fields(s,metadata(),limits()));
}
spec fn leaf(sym:int) -> Seq<u8> {Seq::new(2328,|i:int|if i==sym {1u8} else {0u8})}
proof fn count_leaf(sym:int,value:int,n:nat)
    requires 0<=sym<2328,n<=2328,
    ensures hf::number(leaf(sym),value,n)==(if value==0 {n as int-if sym<n {1int} else {0int}} else if value==1 && sym<n {1int} else {0int}),
    decreases n,
{if n>0 {count_leaf(sym,value,(n-1) as nat);reveal_with_fuel(hf::number,1);}}
proof fn space_leaf(sym:int,n:nat)
    requires 0<=sym<2328,1<=n<=15,
    ensures hf::space(leaf(sym),n)==db::pow(n as int-1),
    decreases n,
{
    count_leaf(sym,n as int,2328);reveal_with_fuel(hf::space,2);
    if n>1 {space_leaf(sym,(n-1) as nat);db::pow_bound(n as int-1);}
}
proof fn rank_leaf(sym:int,i:nat)
    requires 0<=i<=sym<2328,
    ensures hf::ranked(leaf(sym),1,0,i)==sym,
    decreases sym-i,
{reveal_with_fuel(hf::ranked,1);if i<sym {rank_leaf(sym,i+1);}}
proof fn leaf_ok(sym:int)
    requires 0<=sym<2328,
    ensures hf::tree_ok(leaf(sym),false),hf::huff(leaf(sym)).counts[0]==2327,hf::huff(leaf(sym)).counts[1]==1,hf::ranked(leaf(sym),1,0,0)==sym,
{
    count_leaf(sym,0,2328);count_leaf(sym,1,2328);rank_leaf(sym,0);
    assert forall|n:nat| 1<=n<=15 implies #[trigger] hf::space(leaf(sym),n)>=0 by {space_leaf(sym,n);db::pow_bound(n as int-1);}
}
proof fn simple(p:int,nb:nat,sym:int,q:int,alphabet:int)
    requires 0<=sym<alphabet<=2328,(nb==1 || nb==8),q==p+3+nb,p>=0,q<=1272,
        db::field(bytes(),p,1)==1,db::field(bytes(),p+1,1)==0,db::field(bytes(),p+2,1)==if nb==1 {0int} else {1int},db::field(bytes(),p+3,nb)==sym,
    ensures hd::table(bytes(),p,159,alphabet)==Some((q,hf::huff(leaf(sym)))),
{
    leaf_ok(sym);reveal(hd::table);
    let nbm=if db::field(bytes(),p+2,1)==0 {1nat} else {8nat};
    assert(nbm==nb);
    let am=db::field(bytes(),p+3,nbm);let bm=db::field(bytes(),p+3+nbm,8);
    let lens=Seq::new(2328,|i:int|if i==am || (db::field(bytes(),p+1,1)==1 && i==bm) {1u8} else {0u8});
    assert(lens =~= leaf(sym));
    assert(hf::tree_ok(lens,false));
    assert(hd::table(bytes(),p,159,alphabet).is_some());
}
proof fn stream()
    ensures seg::accepted(bytes(),f::model(bytes(),metadata()),0,limits()),
{
    directory();let s=bytes();let fmt=f::model(s,metadata());
    reveal(tag_value);reveal(value_valid);reveal(seg::plan);reveal(c::container);reveal(db::field);
    assert(db::field(bytes(),1240,1)==0 && db::field(bytes(),1241,1)==0 && db::field(bytes(),1242,1)==0) by(compute);
    let cfg=c::Config {bits:db::Bits {at:155,bit:3,end:159},w:1,h:1,subtract:false,cache:0};
    assert(c::container(s,130,30,1,1)==Some(cfg));
    assert(seg::plan(s,fmt,0,limits()).is_some() && seg::plan(s,fmt,0,limits()).unwrap().config==cfg);
    assert(db::field(bytes(),1243,1)==1 && db::field(bytes(),1244,1)==0 && db::field(bytes(),1245,1)==0 && db::field(bytes(),1246,1)==0) by(compute);
    simple(1243,1,0,1247,280);
    assert(db::field(bytes(),1247,1)==1 && db::field(bytes(),1248,1)==0 && db::field(bytes(),1249,1)==0 && db::field(bytes(),1250,1)==0) by(compute);
    simple(1247,1,0,1251,256);
    assert(db::field(bytes(),1251,1)==1 && db::field(bytes(),1252,1)==0 && db::field(bytes(),1253,1)==0 && db::field(bytes(),1254,1)==0) by(compute);
    simple(1251,1,0,1255,256);
    assert(db::field(bytes(),1255,1)==1 && db::field(bytes(),1256,1)==0 && db::field(bytes(),1257,1)==1 && db::field(bytes(),1258,8)==255) by(compute);
    simple(1255,8,255,1266,256);
    assert(db::field(bytes(),1266,1)==1 && db::field(bytes(),1267,1)==0 && db::field(bytes(),1268,1)==0 && db::field(bytes(),1269,1)==0) by(compute);
    simple(1266,1,0,1270,40);
    let h=c::GroupModel {g:hf::huff(leaf(0)),r:hf::huff(leaf(0)),b:hf::huff(leaf(0)),a:hf::huff(leaf(255)),d:hf::huff(leaf(0))};
    reveal(c::group);assert(c::group(s,1243,159,0)==Some((1270,h)));
    leaf_ok(0);leaf_ok(255);reveal(hf::decode);reveal(tok::token);reveal(w::step);
    assert(tok::token(s,1270,159,h,1,0,0,1)==Some((1270,tok::literal(0,0,0,255))));
    let v=w::push(w::initial(1270),[0u8,0,0,255],0);
    assert(w::step(s,cfg,h,1,w::initial(1270))==Some(v));
    reveal_with_fuel(w::finish,2);assert(w::finish(s,cfg,h,1,w::initial(1270),1)==Some(v));
    reveal(w::result);assert(w::result(s,cfg)==Some(v));
}
pub(super) proof fn witness() ensures supported_webp_profile(bytes(),0,limits()), {
    directory();stream();let s=bytes();let fmt=f::model(s,metadata());
    reveal(t::segment_supported);reveal(seg::plan);reveal(tag_value);reveal(value_valid);reveal_with_fuel(t::work_prefix,2);
    assert(t::segment_supported(s,fmt,0,limits()));
    assert(0<=t::work_prefix(s,fmt,1,limits())<=usize::MAX);
    t::profile_from_parts(s,metadata(),limits());
}
}
