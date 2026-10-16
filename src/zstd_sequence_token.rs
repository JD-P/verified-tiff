//! Sequence extra bits, interleaved FSE transitions and repeat-offset history.
use super::*;
use crate::zstd_bits::{self as zb,Reverse};
use crate::zstd_table::{Table,TableModel};
use crate::zstd_sequence_tables as zs;
verus! {
pub(super) open spec fn lengths(code:int,matched:bool) -> (int,int) {
    if matched{if code==0 {(3int,0int)} else if code==1 {(4int,0int)} else if code==2 {(5int,0int)} else if code==3 {(6int,0int)} else if code==4 {(7int,0int)} else if code==5 {(8int,0int)} else if code==6 {(9int,0int)} else if code==7 {(10int,0int)} else if code==8 {(11int,0int)} else if code==9 {(12int,0int)} else if code==10 {(13int,0int)} else if code==11 {(14int,0int)} else if code==12 {(15int,0int)} else if code==13 {(16int,0int)} else if code==14 {(17int,0int)} else if code==15 {(18int,0int)} else if code==16 {(19int,0int)} else if code==17 {(20int,0int)} else if code==18 {(21int,0int)} else if code==19 {(22int,0int)} else if code==20 {(23int,0int)} else if code==21 {(24int,0int)} else if code==22 {(25int,0int)} else if code==23 {(26int,0int)} else if code==24 {(27int,0int)} else if code==25 {(28int,0int)} else if code==26 {(29int,0int)} else if code==27 {(30int,0int)} else if code==28 {(31int,0int)} else if code==29 {(32int,0int)} else if code==30 {(33int,0int)} else if code==31 {(34int,0int)} else if code==32 {(35int,1int)} else if code==33 {(37int,1int)} else if code==34 {(39int,1int)} else if code==35 {(41int,1int)} else if code==36 {(43int,2int)} else if code==37 {(47int,2int)} else if code==38 {(51int,3int)} else if code==39 {(59int,3int)} else if code==40 {(67int,4int)} else if code==41 {(83int,4int)} else if code==42 {(99int,5int)} else if code==43 {(131int,7int)} else if code==44 {(259int,8int)} else if code==45 {(515int,9int)} else if code==46 {(1027int,10int)} else if code==47 {(2051int,11int)} else if code==48 {(4099int,12int)} else if code==49 {(8195int,13int)} else if code==50 {(16387int,14int)} else if code==51 {(32771int,15int)} else if code==52 {(65539int,16int)} else {(0int,0int)}}else{if code==0 {(0int,0int)} else if code==1 {(1int,0int)} else if code==2 {(2int,0int)} else if code==3 {(3int,0int)} else if code==4 {(4int,0int)} else if code==5 {(5int,0int)} else if code==6 {(6int,0int)} else if code==7 {(7int,0int)} else if code==8 {(8int,0int)} else if code==9 {(9int,0int)} else if code==10 {(10int,0int)} else if code==11 {(11int,0int)} else if code==12 {(12int,0int)} else if code==13 {(13int,0int)} else if code==14 {(14int,0int)} else if code==15 {(15int,0int)} else if code==16 {(16int,1int)} else if code==17 {(18int,1int)} else if code==18 {(20int,1int)} else if code==19 {(22int,1int)} else if code==20 {(24int,2int)} else if code==21 {(28int,2int)} else if code==22 {(32int,3int)} else if code==23 {(40int,3int)} else if code==24 {(48int,4int)} else if code==25 {(64int,6int)} else if code==26 {(128int,7int)} else if code==27 {(256int,8int)} else if code==28 {(512int,9int)} else if code==29 {(1024int,10int)} else if code==30 {(2048int,11int)} else if code==31 {(4096int,12int)} else if code==32 {(8192int,13int)} else if code==33 {(16384int,14int)} else if code==34 {(32768int,15int)} else if code==35 {(65536int,16int)} else {(0int,0int)}}
}
fn length(code:usize,matched:bool) -> (r:(usize,usize))
    requires if matched{code<53}else{code<36},
    ensures (r.0 as int,r.1 as int)==lengths(code as int,matched),r.0<=65539,r.1<=16,
        matched ==> r.0>=3,
{if matched{match code {0=>(3usize,0usize),1=>(4usize,0usize),2=>(5usize,0usize),3=>(6usize,0usize),4=>(7usize,0usize),5=>(8usize,0usize),6=>(9usize,0usize),7=>(10usize,0usize),8=>(11usize,0usize),9=>(12usize,0usize),10=>(13usize,0usize),11=>(14usize,0usize),12=>(15usize,0usize),13=>(16usize,0usize),14=>(17usize,0usize),15=>(18usize,0usize),16=>(19usize,0usize),17=>(20usize,0usize),18=>(21usize,0usize),19=>(22usize,0usize),20=>(23usize,0usize),21=>(24usize,0usize),22=>(25usize,0usize),23=>(26usize,0usize),24=>(27usize,0usize),25=>(28usize,0usize),26=>(29usize,0usize),27=>(30usize,0usize),28=>(31usize,0usize),29=>(32usize,0usize),30=>(33usize,0usize),31=>(34usize,0usize),32=>(35usize,1usize),33=>(37usize,1usize),34=>(39usize,1usize),35=>(41usize,1usize),36=>(43usize,2usize),37=>(47usize,2usize),38=>(51usize,3usize),39=>(59usize,3usize),40=>(67usize,4usize),41=>(83usize,4usize),42=>(99usize,5usize),43=>(131usize,7usize),44=>(259usize,8usize),45=>(515usize,9usize),46=>(1027usize,10usize),47=>(2051usize,11usize),48=>(4099usize,12usize),49=>(8195usize,13usize),50=>(16387usize,14usize),51=>(32771usize,15usize),52=>(65539usize,16usize),_ => (0usize,0usize)}}else{match code {0=>(0usize,0usize),1=>(1usize,0usize),2=>(2usize,0usize),3=>(3usize,0usize),4=>(4usize,0usize),5=>(5usize,0usize),6=>(6usize,0usize),7=>(7usize,0usize),8=>(8usize,0usize),9=>(9usize,0usize),10=>(10usize,0usize),11=>(11usize,0usize),12=>(12usize,0usize),13=>(13usize,0usize),14=>(14usize,0usize),15=>(15usize,0usize),16=>(16usize,1usize),17=>(18usize,1usize),18=>(20usize,1usize),19=>(22usize,1usize),20=>(24usize,2usize),21=>(28usize,2usize),22=>(32usize,3usize),23=>(40usize,3usize),24=>(48usize,4usize),25=>(64usize,6usize),26=>(128usize,7usize),27=>(256usize,8usize),28=>(512usize,9usize),29=>(1024usize,10usize),30=>(2048usize,11usize),31=>(4096usize,12usize),32=>(8192usize,13usize),33=>(16384usize,14usize),34=>(32768usize,15usize),35=>(65536usize,16usize),_ => (0usize,0usize)}}}
pub(super) open spec fn states_ok(t:Seq<TableModel>,s:[usize;3]) -> bool {
    t.len()==3 && s[0]<zb::pow(t[0].log as int) && s[1]<zb::pow(t[1].log as int) && s[2]<zb::pow(t[2].log as int)
}
#[derive(Clone,Copy)]
pub(super) struct Token {pub literals:usize,pub matched:usize,pub offset:usize,pub states:[usize;3],pub p:usize}
pub(super) closed spec fn token(s:Seq<u8>,low:int,p:int,t:Seq<TableModel>,state:[usize;3],last:bool) -> Option<Token> {
    let l=t[0].cells[state[0] as int];let o=t[1].cells[state[1] as int];let m=t[2].cells[state[2] as int];
    if l.symbol>=36 || o.symbol>=32 || m.symbol>=53{None}else{
        let (lb,ln)=lengths(l.symbol as int,false);let (mb,mn)=lengths(m.symbol as int,true);
        let on=o.symbol as int;let q=p-on-mn-ln;
        if q<low{None}else{
            let offset=zb::pow(on)+zb::value(s,p-on,on as nat);
            let matched=mb+zb::value(s,p-on-mn,mn as nat);let literals=lb+zb::value(s,q,ln as nat);
            if last {Some(Token {literals:literals as usize,matched:matched as usize,offset:offset as usize,states:state,p:q as usize})}
            else {
                let end=q-l.bits-m.bits-o.bits;
                if end<low{None}else{
                    let ls=l.base as int+zb::value(s,q-l.bits,l.bits as nat);
                    let ms=m.base as int+zb::value(s,q-l.bits-m.bits,m.bits as nat);
                    let os=o.base as int+zb::value(s,end,o.bits as nat);
                    Some(Token {literals:literals as usize,matched:matched as usize,offset:offset as usize,states:[ls as usize,os as usize,ms as usize],p:end as usize})
                }
            }
        }
    }
}
pub(super) fn read(s:&[u8],bits:&mut Reverse,tables:&[Table;3],state:[usize;3],last:bool) -> (r:Result<Token,Error>)
    requires old(bits).valid(s@),zs::valid(*tables),states_ok(zs::models(*tables),state),
    ensures final(bits).valid(s@),final(bits).low==old(bits).low,final(bits).end==old(bits).end,
        r.is_ok()==token(s@,old(bits).low as int,old(bits).p as int,zs::models(*tables),state,last).is_some(),
        r.is_ok() ==> r.unwrap()==token(s@,old(bits).low as int,old(bits).p as int,zs::models(*tables),state,last).unwrap(),
        r.is_ok() ==> r.unwrap().p==final(bits).p && states_ok(zs::models(*tables),r.unwrap().states),
        r.is_ok() ==> r.unwrap().literals<=131071 && 3<=r.unwrap().matched<=131074 && 1<=r.unwrap().offset<=4294967295,
{
    reveal(token);
    let l=tables[0].cells[state[0]];let o=tables[1].cells[state[1]];let m=tables[2].cells[state[2]];
    if l.symbol>=36 || o.symbol>=32 || m.symbol>=53{return Err(Error::Invalid);}
    let (lb,ln)=length(l.symbol as usize,false);let (mb,mn)=length(m.symbol as usize,true);
    let on=o.symbol as usize;
    let offset=zb::power(on) as usize+bits.read(s,on)? as usize;
    let matched=mb+bits.read(s,mn)? as usize;
    let literals=lb+bits.read(s,ln)? as usize;
    let states=if last{state}else{
        let ls=l.base as usize+bits.read(s,l.bits as usize)? as usize;
        let ms=m.base as usize+bits.read(s,m.bits as usize)? as usize;
        let os=o.base as usize+bits.read(s,o.bits as usize)? as usize;
        [ls,os,ms]
    };
    Ok(Token {literals,matched,offset,states,p:bits.p})
}
pub(super) closed spec fn offset_model(previous:[usize;3],value:usize,zero:bool) -> Option<[usize;3]> {
    if value>3{Some([(value-3) as usize,previous[0],previous[1]])}
    else if value==0{None}
    else{
        let which=value as int-1+if zero{1int}else{0int};
        if which==0{Some(previous)}else if which==1{Some([previous[1],previous[0],previous[2]])}
        else if which==2{Some([previous[2],previous[0],previous[1]])}
        else if previous[0]<=1{None}else{Some([(previous[0]-1) as usize,previous[0],previous[1]])}
    }
}
pub(super) fn resolve(previous:[usize;3],value:usize,zero:bool) -> (r:Result<[usize;3],Error>)
    requires previous[0]>0,previous[1]>0,previous[2]>0,
    ensures r.is_ok()==offset_model(previous,value,zero).is_some(),
        r.is_ok() ==> r.unwrap()==offset_model(previous,value,zero).unwrap(),
        r.is_ok() ==> r.unwrap()[0]>0 && r.unwrap()[1]>0 && r.unwrap()[2]>0,
{
    reveal(offset_model);
    if value>3{return Ok([value-3,previous[0],previous[1]]);}
    if value==0{return Err(Error::Invalid);}
    let which=value-1+if zero{1usize}else{0};
    if which==0{Ok(previous)}else if which==1{Ok([previous[1],previous[0],previous[2]])}
    else if which==2{Ok([previous[2],previous[0],previous[1]])}
    else{if previous[0]<=1{return Err(Error::Invalid);}Ok([previous[0]-1,previous[0],previous[1]])}
}
}
