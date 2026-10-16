//! Typed metadata transport at the unverified C boundary.
use super::*;
use verified_tiff::{native_samples::read_word,adapter_meta as checked};
pub struct Converted{key:u32,kind:u16,n:usize,data:Buf<u64>}
fn signed(v:u64,w:usize)->i64{match w{1=>v as i8 as i64,2=>v as i16 as i64,4=>v as i32 as i64,_=>v as i64}}
pub fn real(s:&[u8],at:usize,kind:u16,le:bool)->R<f64>{
    let w=container::field_size(kind);let v=read_word(s,at,if kind==5||kind==10{4}else{w},le).map_err(err)?;
    Ok(match kind{
        5|10=>{let d=read_word(s,at+4,4,le).map_err(err)?;if d==0{return Err((INVALID,"zero rational denominator"));}if kind==10{(v as i32 as f64)/(d as i32 as f64)}else{v as f64/d as f64}},
        11=>f32::from_bits(v as u32) as f64,12=>f64::from_bits(v),6|8|9|17=>signed(v,w) as f64,_=>v as f64})
}
impl Handle{
    pub fn put(&mut self,key:u16,kind:u16,count:usize,raw:&[u8])->R<()>{
        let w=container::field_size(kind);
        if w==0||raw.len()!=product(count,w)?{return Err((INVALID,"field storage size"));}
        if self.directory_kind==0&&key==334{
            let candidate=Tag{key,kind,count,at:0,remote:None};
            if count!=1||value(raw,candidate,0,self.meta.le)?>65535{return Err((INVALID,"invalid NumberOfInks"));}
            if let Some(names)=find(&self.meta.tags,333){
                let len=checked::payload(self.data.len(),names.at,names.count,names.kind).map_err(err)?;
                let actual=checked::ink_names(&self.data[names.at..names.at+len]).map_err(err)?;
                if value(raw,candidate,0,self.meta.le)?!=actual as u64{return Err((INVALID,"NumberOfInks disagrees with InkNames"));}
            }
        }
        let old=find(&self.meta.tags,key);
        let n=self.meta.tags.len()+usize::from(old.is_none());
        if n>self.limits.max_tags{return Err((LIMIT,"directory tag limit"));}
        let total=self.data.len().checked_add(raw.len()).and_then(|v|v.checked_add(1)).ok_or((LIMIT,"metadata size overflow"))?;
        let mut data=Buf::new(total,&self.source.mem)?;let mut tags=Buf::new(n,&self.source.mem)?;
        for t in self.meta.tags.iter(){if t.key!=key{if t.remote.is_some(){tags.push(*t)?;continue;}let at=data.len();let bytes=product(t.count,container::field_size(t.kind))?;data.extend(&self.data[t.at..t.at+bytes])?;data.push(0)?;tags.push(Tag{at,..*t})?;}}
        let at=data.len();data.extend(raw)?;data.push(0)?;tags.push(Tag{key,kind,count,at,remote:None})?;
        tags.sort_by_key(|t|t.key);
        let (data,meta)=if self.directory_kind!=0||self.writing{
            if self.directory_kind==0{normalize_inks(&mut data,&mut tags,self.meta.le,&self.limits)?;}
            let (_,mut m)=empty_metadata(&self.source.mem,self.header)?;m.tags=tags;m.off=self.meta.off;m.next=self.meta.next;(data,m)
        }else{assemble(data,tags,self.meta.off,self.meta.next,self.meta.le,&self.limits,self.chop,&self.source.mem)?};
        self.converted=Buf::new(0,&self.source.mem)?;self.data=data;self.meta=meta;self.stream=None;self.cache=None;self.rendered=None;Ok(())
    }
    fn typed(&mut self,key:u32,kind:u16)->R<Option<(*const u8,usize)>>{
        if let Some(v)=self.converted.iter().find(|v|v.key==key&&v.kind==kind){return Ok(Some((v.data.as_ptr().cast(),v.n)));}
        let Some(t)=find(&self.meta.tags,u16::try_from(key).map_err(|_|(INVALID,"tag overflow"))?)else{return Ok(None);};
        if t.remote.is_some(){return Err((NOTIMPL,"use strile accessor for deferred arrays"));}
        checked::payload(self.data.len(),t.at,t.count,t.kind).map_err(err)?;
        let w=container::field_size(kind);let sw=container::field_size(t.kind);
        if w==0||matches!(kind,5|10){return Err((INVALID,"invalid requested metadata type"));}
        let bytes=product(t.count,w)?.checked_add(if kind==2{1}else{0}).ok_or((LIMIT,"metadata size overflow"))?;
        let words=bytes.checked_add(7).ok_or((LIMIT,"metadata size overflow"))?/8;
        let mut data=Buf::new(words,&self.source.mem)?;for _ in 0..words{data.push(0u64)?;}
        for i in 0..t.count{
            let at=t.at+product(i,sw)?;
            let value=if matches!(kind,11|12){let v=real(&self.data,at,t.kind,self.meta.le)?;if kind==11{(v as f32).to_bits() as u64}else{v.to_bits()}}
            else{
                let v=read_word(&self.data,at,sw,self.meta.le).map_err(err)?;
                checked::convert_integer(v,t.kind,kind).map_err(err)?
            };
            // Native representation in a u64-aligned allocation; write bytes
            // without aliasing typed foreign pointers.
            let b=value.to_ne_bytes();let from=if cfg!(target_endian="big"){8-w}else{0};
            for j in 0..w{let p=i*w+j;let mut a=data[p/8].to_ne_bytes();a[p%8]=b[from+j];data[p/8]=u64::from_ne_bytes(a);}
        }
        let ptr=data.as_ptr().cast();self.converted.reserve(self.converted.len()+1)?;
        self.converted.push(Converted{key,kind,n:t.count,data})?;Ok(Some((ptr,t.count)))
    }
}
#[no_mangle]
pub unsafe extern "C" fn vt_put(p:*mut c_void,key:u32,kind:u16,count:usize,input:*const u8)->i32{
    guard(0,||{let h=handle(p)?;let key=u16::try_from(key).map_err(|_|(INVALID,"tag overflow"))?;
        let w=container::field_size(kind);let len=product(count,w)?;
        if w==0||len>isize::MAX as usize||(len>0&&input.is_null()){return Err((INVALID,"invalid field input"));}
        if !h.writing&&h.directory_kind==0&&matches!(key,256|257|258|259|277|278|284|317|322|323|273|279|324|325){return Err((NOTIMPL,"read handle geometry mutation not implemented"));}
        let mut raw=Buf::zeros(len,h.limits.max_input,&h.source.mem)?;
        if len>0{raw.copy_from_slice(std::slice::from_raw_parts(input,len));}
        if h.meta.le!=cfg!(target_endian="little") {let stride=if kind==5||kind==10{4}else{w};for word in raw.chunks_exact_mut(stride){word.reverse();}}
        h.put(key,kind,count,&raw)?;Ok(1)})
}
#[no_mangle]
pub unsafe extern "C" fn vt_typed(p:*mut c_void,key:u32,kind:u16,out:*mut *const u8,n:*mut usize)->i32{
    guard(-1,||{if out.is_null()||n.is_null(){return Err((INVALID,"null typed output"));}match handle(p)?.typed(key,kind)?{Some((a,b))=>{*out=a;*n=b;Ok(1)},None=>Ok(0)}})
}
#[no_mangle]
pub unsafe extern "C" fn vt_custom_dir(p:*mut c_void,off:u64,kind:u32)->i32{
    guard(0,||{let h=handle(p)?;let (data,tags,next)=raw_metadata(&mut h.source,h.header,off,&h.limits,false)?;
        let (_,mut meta)=empty_metadata(&h.source.mem,h.header)?;meta.tags=tags;meta.off=off;meta.next=next;
        let converted=Buf::new(0,&h.source.mem)?;
        h.converted=converted;h.meta=meta;h.data=data;h.page=u32::MAX;h.directory_kind=kind;h.stream=None;h.cache=None;h.rendered=None;Ok(1)})
}
#[no_mangle]
pub unsafe extern "C" fn vt_mem_claim(p:*mut c_void,n:usize)->i32{guard(0,||{handle(p)?.source.mem.claim(n)?;Ok(1)})}
#[no_mangle]
pub unsafe extern "C" fn vt_mem_release(p:*mut c_void,n:usize){guard((),||{handle(p)?.source.mem.release(n);Ok(())});}
#[no_mangle]
pub unsafe extern "C" fn vt_put_real(p:*mut c_void,key:u32,kind:u16,host:u16,count:usize,input:*const u8)->i32{
    guard(0,||{let h=handle(p)?;if !matches!(host,11|12)||!matches!(kind,5|10)||(count>0&&input.is_null()){return Err((INVALID,"rational input"));}
        let len=product(count,8)?;if len>isize::MAX as usize{return Err((LIMIT,"rational input size"));}
        let mut raw=Buf::new(len,&h.source.mem)?;
        for i in 0..count{
            let v=if host==11{ptr::read_unaligned(input.add(product(i,4)?).cast::<f32>()) as f64}else{ptr::read_unaligned(input.add(product(i,8)?).cast::<f64>())};
            let max=if kind==5{u32::MAX as f64}else{i32::MAX as f64};
            if !v.is_finite()||(kind==5&&v<0.)||v.abs()>max{return Err((INVALID,"rational range"));}
            let den=if v==0.{1.}else{max.min((max/v.abs()).floor()).max(1.)};let num=(v*den).round();
            let a=if kind==5{num as u32}else{num as i32 as u32};let b=den as u32;
            raw.extend(&if h.meta.le{a.to_le_bytes()}else{a.to_be_bytes()})?;raw.extend(&if h.meta.le{b.to_le_bytes()}else{b.to_be_bytes()})?;
        }
        h.put(u16::try_from(key).map_err(|_|(INVALID,"tag overflow"))?,kind,count,&raw)?;Ok(1)})
}

// Stateless ABI glue: no file metadata is allowed to choose the calling shape.
#[no_mangle]
pub extern "C" fn vt_field_plan(kind:u16,fixed:i32,pass:i32,count_width:usize,array:i32,size:usize,n:usize)->u8{
    if size!=container::field_size(kind){return 0;}
    checked::get_plan(kind,fixed,pass!=0,count_width,array!=0,n)
}
#[no_mangle]
pub extern "C" fn vt_field_registration(kind:u16,rc:i32,wc:i32,pass:i32)->i32{
    i32::from(checked::registration(kind,rc,wc,pass!=0))
}
#[no_mangle]
pub extern "C" fn vt_host_type(kind:u16)->u16{checked::host_type(kind)}
