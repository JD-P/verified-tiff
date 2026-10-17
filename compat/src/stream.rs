//! Bounded input windows and resumable native reads at the unverified I/O
//! boundary. Packet, LZW dictionary, bit and Deflate steps are verified core
//! operations. Complete validation precedes exposure of any requested bytes.
//! Full Deflate requests validate while decoding into private staging, and
//! admitted LZW segments decode whole in one verified pass. Other partial
//! requests validate first and then replay through a bounded stream.
use super::*;
use verified_tiff::{native_stream as core,native_samples as ns,native,adapter_window as window,adapter_bounds as checked};
const WINDOW:usize=65536;
struct Input{data:Buf<u8>,len:usize,loaded:usize,count:usize,off:u64,at:usize,bit:usize,reverse:bool}
impl Input{
    fn new(source:&mut Source,off:u64,count:usize,reverse:bool)->R<Self>{
        if !checked::source_range(source.len,off,count){return Err((INVALID,"segment outside input"));}
        let data=Buf::zeros(count.min(WINDOW),source.input,&source.mem)?;
        let mut s=Self{data,len:0,loaded:0,count,off,at:0,bit:0,reverse};s.ensure(source,WINDOW)?;Ok(s)
    }
    #[inline(always)]
    fn ensure(&mut self,source:&mut Source,n:usize)->R<()>{
        if self.len-self.at>=n||self.loaded==self.count{return Ok(());}
        self.refill(source)
    }
    #[cold]
    fn refill(&mut self,source:&mut Source)->R<()>{
        let (rest,n,loaded)=window::refill_plan(self.data.len(),self.len,self.at,self.loaded,self.count).map_err(err)?;
        window::compact(&mut self.data,self.at,self.len).map_err(err)?;
        // Publish cursor updates only after the exact checked source span arrives.
        let off=checked::source_offset(self.off,self.loaded,n,source.len).map_err(err)?;
        source.read_into(off,&mut self.data[rest..rest+n])?;
        if self.reverse{ns::reverse_bits(&mut self.data[rest..rest+n]);}
        self.at=0;self.loaded=loaded;self.len=rest+n;Ok(())
    }
    fn remaining_bits(&self)->R<u128>{window::remaining(self.count,self.loaded,self.len,self.at,self.bit).map_err(err)}
}
enum Engine{
    Plain,
    Pack{data:usize,left:usize,repeat:bool},
    Lzw{state:core::Lzw,phrase:[u8;4096],at:usize,len:usize,previous_data:bool},
    Zlib{state:core::Inflater},
}
pub struct Stream{input:Input,engine:Buf<Engine>,pub produced:usize,needed:usize,cap:usize,row:usize,done:bool,discard:bool,budget:Budget}
impl Stream{
    fn new(source:&mut Source,off:u64,count:usize,codec:u16,row:usize,needed:usize,cap:usize,reverse:bool,quota:usize)->R<Self>{
        if row==0||needed==0||needed>cap{return Err((INVALID,"stream geometry"));}
        let mut input=Input::new(source,off,count,reverse)?;
        let mut engine=Buf::new(1,&source.mem)?;
        let state=match codec{
            1=>{if count<needed{return Err((INVALID,"short uncompressed segment"));}Engine::Plain},
            32773=>Engine::Pack{data:0,left:0,repeat:false},
            5=>{if input.len<2{return Err((INVALID,"short LZW stream"));}Engine::Lzw{state:core::Lzw::new(input.data[0]==0&&input.data[1]&1!=0),phrase:[0;4096],at:0,len:0,previous_data:false}},
            8|32946=>{let state=core::Inflater::new(&input.data[..input.len],cap).map_err(err)?;input.at=2;Engine::Zlib{state}},
            _=>return Err((NOTIMPL,"codec requires a complete segment")),
        };engine.push(state)?;
        Ok(Self{input,engine,produced:0,needed,cap,row,done:false,discard:false,budget:Budget{remaining:quota}})
    }
    fn charge(&mut self,n:usize)->R<()>{verified_tiff::spend(&mut self.budget,n).map_err(err)}
    /// Return the next bounded batch. A zero-length result means a validated
    /// end marker, not a need-input indication. The caller owns I/O progress.
    fn pull(&mut self,source:&mut Source,out:&mut[u8])->R<usize>{
        let mut written=0;
        while written<out.len()&&!self.done{
            if !matches!(&self.engine[0],Engine::Zlib{..}){self.charge(1)?;}
            match &mut self.engine[0]{
                Engine::Plain=>{
                    if self.produced==self.needed{self.done=true;continue;}
                    self.input.ensure(source,1)?;
                    let n=(out.len()-written).min(self.needed-self.produced).min(self.input.len-self.input.at);
                    if n==0{return Err((INVALID,"short native segment"));}
                    out[written..written+n].copy_from_slice(&self.input.data[self.input.at..self.input.at+n]);
                    self.input.at+=n;self.produced+=n;written+=n;self.charge(n)?;
                }
                Engine::Pack{data,left,repeat}=>{
                    if *left>0{
                        let n=(*left).min(out.len()-written);
                        if *repeat{out[written..written+n].fill(self.input.data[*data]);}
                        else{out[written..written+n].copy_from_slice(&self.input.data[*data..*data+n]);*data+=n;}
                        *left-=n;self.produced+=n;written+=n;self.charge(n)?;
                    }else{
                        self.input.ensure(source,129)?;
                        if self.input.at==self.input.len{if self.produced!=self.needed{return Err((INVALID,"short PackBits stream"));}self.done=true;continue;}
                        let (next,at,n,r)=core::packet(&self.input.data[..self.input.len],self.input.at,self.row,self.produced,self.needed).map_err(err)?;
                        self.input.at=next;*data=at;*left=n;*repeat=r;
                    }
                }
                Engine::Lzw{state,phrase,at,len,previous_data}=>{
                    if *at<*len{
                        let n=(*len-*at).min(out.len()-written);
                        if !self.discard{out[written..written+n].copy_from_slice(&phrase[*at..*at+n]);}
                        *at+=n;self.produced+=n;written+=n;self.charge(n)?;
                    }else{
                        if self.produced==self.needed&&*previous_data&&self.input.remaining_bits()?<8{self.done=true;continue;}
                        self.input.ensure(source,3)?;
                        let (code,next,bit)=core::lzw_code(&self.input.data[..self.input.len],self.input.at,self.input.bit,state.width(),state.is_legacy()).map_err(err)?;
                        self.input.at=next;self.input.bit=bit;
                        *len=if self.discard{state.skip(code)}else{state.code(code,phrase)}.map_err(err)?;*at=0;
                        if *len>self.needed-self.produced{return Err((INVALID,"excess LZW output"));}
                        *previous_data=code!=256&&code!=257;
                        if code==257{
                            if self.produced!=self.needed||self.input.remaining_bits()?>=8{return Err((INVALID,"LZW ending/length"));}
                            self.done=true;
                        }else if *len>0{
                            // Emit at once, charging the unit its own pass would.
                            let n=(*len).min(out.len()-written);
                            verified_tiff::spend(&mut self.budget,1).map_err(err)?;
                            if !self.discard{out[written..written+n].copy_from_slice(&phrase[..n]);}
                            *at=n;self.produced+=n;written+=n;self.charge(n)?;
                        }
                    }
                }
                Engine::Zlib{state}=>{
                    // One window binding and budget loop per bounded output run.
                    // The core yields with room for the largest next header.
                    self.input.ensure(source,1024)?;
                    if state.mode()==4&&self.input.loaded!=self.input.count{return Err((INVALID,"bytes after zlib checksum"));}
                    if self.input.len<4{return Err((INVALID,"short zlib window"));}
                    let last=self.input.loaded==self.input.count;
                    let space=out.len()-written;
                    let (at,bit,n,done)=state.pull(&self.input.data[..self.input.len],self.input.at,self.input.bit,self.input.len-4,last,&mut out[written..],space,&mut self.budget).map_err(err)?;
                    self.input.at=at;self.input.bit=bit;
                    written+=n;self.produced+=n;
                    if done{if self.produced<self.needed||self.produced>self.cap{return Err((INVALID,"zlib output length"));}self.done=true;}
                }
            }
        }
        Ok(written)
    }
    /// Validate the rest of the segment, consuming the stream so it cannot be
    /// read again. LZW phrases are counted, not expanded; each step, check and
    /// charge is otherwise that of a pull into the discarded buffer.
    fn finish(mut self,source:&mut Source)->R<()>{self.discard=true;let mut discard=[0u8;4096];while !self.done{self.pull(source,&mut discard)?;}Ok(())}
    fn range(&mut self,source:&mut Source,start:usize,out:&mut[u8])->R<()>{
        if start<self.produced||start>self.needed||out.len()>self.needed-start{return Err((INVALID,"stream range"));}
        let mut discard=[0u8;4096];
        while self.produced<start{let n=(start-self.produced).min(discard.len());if self.pull(source,&mut discard[..n])?!=n{return Err((INVALID,"short stream before requested range"));}}
        let mut at=0;while at<out.len(){let n=self.pull(source,&mut out[at..])?;if n==0{return Err((INVALID,"short stream range"));}at+=n;}
        Ok(())
    }
}
pub struct Cached{pub k:usize,pub stream:Stream}
impl Handle{
    /// Optional full LZW/Deflate staging is limited to 1 MiB even with unlimited
    /// deployment limits. Larger/tightly-budgeted requests retain row replay.
    pub fn rgba_full_segment(&mut self,k:usize,needed:usize)->R<bool>{
        if !matches!(self.meta.compression,5|8|32946)||needed>1048576||needed>self.limits.max_output{return Ok(false);}
        let count=usize::try_from(self.meta.counts.get(&mut self.source,k)?).map_err(|_|(LIMIT,"segment count overflow"))?;
        if needed<=65536&&count<=65536{return Ok(false);}
        let engine=std::mem::size_of::<Engine>();let window=count.min(WINDOW);
        let copies=if self.meta.predictor==1&&self.meta.le==cfg!(target_endian="little"){1}else{3};
        let total=product(needed,copies)?.checked_add(window).and_then(|n|n.checked_add(engine)).ok_or((LIMIT,"RGBA staging overflow"))?;
        Ok(window<=self.source.input&&self.source.mem.fits(needed.max(window).max(engine),total))
    }
    pub fn rgba_cached_segment(&mut self,k:usize,needed:usize)->R<bool>{
        if !self.streaming_codec(){return Ok(true);}
        if self.meta.compression==1||needed>65536{return Ok(false);}
        Ok(self.meta.counts.get(&mut self.source,k)?<=65536)
    }
    pub fn streaming_codec(&self)->bool{matches!(self.meta.compression,1|32773|5|8|32946)}
    pub fn transform_native(&self,raw:Buf<u8>,row:usize)->R<Buf<u8>>{
        let m=&self.meta;let n=raw.len();let mem=&self.source.mem;
        if !(1..=6).contains(&m.format)||(m.format==3&&!matches!(m.bits,16|24|32|64))||(matches!(m.format,5|6)&&!matches!(m.bits,16|32|64|128)){return Err((NOTIMPL,"native sample format"));}
        if m.predictor==3{
            if m.format!=3||!matches!(m.bits,16|24|32|64)||!matches!(m.compression,5|8|32946|50000){return Err((NOTIMPL,"floating predictor layout"));}
            let mut scratch=Buf::zeros(n,self.limits.max_output,mem)?;let mut out=Buf::zeros(n,self.limits.max_output,mem)?;
            let ch=if m.planar==2{1}else{m.ch};
            let r=match (ch,m.bits){
                (1,16)=>verified_tiff::native_fp::decode(&raw,row,1,2,cfg!(target_endian="little"),&mut scratch,&mut out),
                (1,24)=>verified_tiff::native_fp::decode(&raw,row,1,3,cfg!(target_endian="little"),&mut scratch,&mut out),
                (1,32)=>verified_tiff::native_fp::decode(&raw,row,1,4,cfg!(target_endian="little"),&mut scratch,&mut out),
                (1,64)=>verified_tiff::native_fp::decode(&raw,row,1,8,cfg!(target_endian="little"),&mut scratch,&mut out),
                _=>verified_tiff::native_fp::decode(&raw,row,ch,m.bits/8,cfg!(target_endian="little"),&mut scratch,&mut out),
            };r.map_err(err)?;Ok(out)
        }else if m.predictor==2{
            if !matches!(m.bits,8|16|32|64)||matches!(m.format,5|6)||!matches!(m.compression,5|8|32946|50000){return Err((NOTIMPL,"horizontal predictor layout"));}
            let mut out=Buf::zeros(n,self.limits.max_output,mem)?;
            ns::convert(&raw,row,if m.planar==2{1}else{m.ch},m.bits/8,m.le,true,cfg!(target_endian="little"),&mut out).map_err(err)?;Ok(out)
        }else if m.predictor==1{
            let bytes=if matches!(m.format,5|6){m.bits/16}else{m.bits/8};
            if m.le!=cfg!(target_endian="little")&&matches!(m.bits,16|24|32|64|128)&&bytes>1{
                let mut out=Buf::zeros(n,self.limits.max_output,mem)?;ns::swap_components(&raw,bytes,&mut out).map_err(err)?;Ok(out)
            }else{Ok(raw)}
        }else{Err((NOTIMPL,"sample predictor"))}
    }
    /// LZW segments with at most 1 MiB of input and output decode whole in one
    /// verified pass when memory admits them, as cached small segments do:
    /// complete requests into their staging, partial ones into the cache.
    fn lzw_whole(&self,needed:usize,count:usize)->bool{
        let copies=if self.meta.predictor==1&&self.meta.le==cfg!(target_endian="little"){1}else{3};
        needed<=1048576&&count<=1048576&&count<=self.source.input&&needed<=self.limits.max_output
            &&self.source.mem.fits(needed.max(count),needed*copies+count)
    }
    pub fn native_range(&mut self,k:usize,start:usize,len:usize,budget:&mut Budget)->R<Buf<u8>>{
        let rows=self.meta.segment_rows(k)?;let (row,rows)=self.native_layout(self.meta.bw,rows)?;let needed=product(row,rows)?;
        if start>needed||len>needed-start{return Err((INVALID,"native range outside segment"));}
        let off=self.meta.offsets.get(&mut self.source,k)?;
        let count=usize::try_from(self.meta.counts.get(&mut self.source,k)?).map_err(|_|(LIMIT,"segment count overflow"))?;
        if len==0{return Buf::new(0,&self.source.mem);}
        if self.meta.compression==1&&self.streaming_codec(){
            if count<needed||off>self.source.len||count as u64>self.source.len-off{return Err((INVALID,"uncompressed segment range"));}
            let (first,last)=checked::row_range(needed,row,start,len).map_err(err)?;let end=start+len;
            if last-first>self.limits.max_output{return Err((LIMIT,"native range output limit"));}
            let cost=native::work_limit(last-first,last-first).map_err(err)?;
            if cost>budget.remaining{return Err((LIMIT,"native range work limit"));}budget.remaining-=cost;
            let mut raw=self.source.read(checked::source_offset(off,first,last-first,self.source.len).map_err(err)?,last-first)?;
            if self.meta.fill==2{ns::reverse_bits(&mut raw);}
            let raw=self.transform_native(raw,row)?;
            if first==start&&last==end{return Ok(raw);}
            let mut out=Buf::zeros(len,self.limits.max_output,&self.source.mem)?;out.copy_from_slice(&raw[start-first..end-first]);return Ok(out);
        }
        let small=needed<=65536&&count<=65536;let complete=start==0&&len==needed;
        let whole=!small&&self.meta.compression==5&&self.lzw_whole(needed,count);
        if !self.streaming_codec()||small||(whole&&!complete){
            if whole&&!self.cache.as_ref().is_some_and(|(i,_)|*i==k){
                // Retain the stream's conservative validation-plus-replay charge.
                let extra=native::work_limit(count,needed).map_err(err)?;
                if extra>budget.remaining{return Err((LIMIT,"stream validation/decode budget"));}budget.remaining-=extra;
            }
            self.segment(k,budget)?;let mut out=Buf::zeros(len,self.limits.max_output,&self.source.mem)?;
            out.copy_from_slice(&self.cache.as_ref().unwrap().1[start..start+len]);return Ok(out);
        }
        let (first,last)=checked::row_range(needed,row,start,len).map_err(err)?;let end=start+len;
        let mut raw=Buf::zeros(last-first,self.limits.max_output,&self.source.mem)?;
        // A complete native request already owns its entire staging buffer.
        // Decode once into that private buffer, validate the ending/checksum,
        // then return it. No caller-visible bytes are committed before success.
        if complete&&matches!(self.meta.compression,5|8|32946){
            self.stream=None;self.cache=None;
            let cap=if matches!(self.meta.compression,8|32946)&&!self.meta.tiled{product(row,self.meta.bh)?}else{needed};
            let quota=native::work_limit(count,cap).map_err(err)?;
            // Retain the former conservative validation-plus-replay charge.
            let total=product(quota,2)?;if total>budget.remaining{return Err((LIMIT,"stream validation/decode budget"));}budget.remaining-=total;
            if whole{
                let mut encoded=self.source.read(off,count)?;
                if self.meta.fill==2{ns::reverse_bits(&mut encoded);}
                native::decode(&encoded,5,row,needed,cap,&mut raw,&mut Budget{remaining:quota}).map_err(err)?;
                return self.transform_native(raw,row);
            }
            let mut stream=Stream::new(&mut self.source,off,count,self.meta.compression,row,needed,cap,self.meta.fill==2,quota)?;
            stream.range(&mut self.source,0,&mut raw)?;
            stream.finish(&mut self.source)?;
            return self.transform_native(raw,row);
        }
        // Keep a single validated, resumable segment for sequential reads.
        // Backward/random access resets its cursor and validates a fresh source.
        if !self.stream.as_ref().is_some_and(|c|c.k==k&&c.stream.produced<=first){
            self.stream=None;self.cache=None;
            let off=self.meta.offsets.get(&mut self.source,k)?;let count=usize::try_from(self.meta.counts.get(&mut self.source,k)?).map_err(|_|(LIMIT,"segment count overflow"))?;
            let cap=if matches!(self.meta.compression,8|32946)&&!self.meta.tiled{product(row,self.meta.bh)?}else{needed};
            let quota=native::work_limit(count,cap).map_err(err)?;
            let total=product(quota,2)?;if total>budget.remaining{return Err((LIMIT,"stream validation/decode budget"));}budget.remaining-=total;
            let codec=self.meta.compression;let reverse=self.meta.fill==2;
            let check=Stream::new(&mut self.source,off,count,codec,row,needed,cap,reverse,quota)?;
            check.finish(&mut self.source)?;
            let stream=Stream::new(&mut self.source,off,count,codec,row,needed,cap,reverse,quota)?;
            self.stream=Some(Cached{k,stream});
        }
        let mut cached=self.stream.take().unwrap();
        cached.stream.range(&mut self.source,first,&mut raw)?;
        let native=self.transform_native(raw,row)?;
        self.stream=Some(cached);
        if first==start&&last==end{return Ok(native);}
        let mut out=Buf::zeros(len,self.limits.max_output,&self.source.mem)?;out.copy_from_slice(&native[start-first..end-first]);
        Ok(out)
    }
}
