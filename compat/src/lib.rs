//! Unverified ABI adapter. The verified decoder remains a separate dependency.
//! All foreign pointers must obey the C API's validity, ownership and alias rules.
use std::{
    cell::RefCell,
    ffi::{c_char, c_void},
    os::fd::FromRawFd,
    panic::{catch_unwind, AssertUnwindSafe},
    ptr,
};
use verified_tiff::{
    decode_jpeg_into, inspect_raster, read32, Budget, Error, Limits,
};
type R<T> = Result<T, (i32, &'static str)>;
const INVALID: i32 = 22;
const NOTIMPL: i32 = 38;
const LIMIT: i32 = 27;
const NOMEM: i32 = 12;
fn err(e: Error) -> (i32, &'static str) {
    match e {
        Error::Unsupported => (NOTIMPL, "TIFF format not implemented by verified-tiff"),
        Error::Limit => (LIMIT, "verified-tiff resource limit"),
        Error::Truncated => (INVALID, "truncated TIFF input"),
        Error::Invalid => (INVALID, "invalid TIFF input"),
        Error::PageNotFound => (INVALID, "directory not found"),
        Error::OutputTooSmall => (INVALID, "output buffer too small"),
    }
}
thread_local! {static ERROR:RefCell<(i32,[u8;256])>=const{RefCell::new((0,[0;256]))};}
fn report(e: (i32, &str)) {
    ERROR.with(|a| {
        let mut a = a.borrow_mut();
        a.0 = e.0;
        a.1.fill(0);
        let n = e.1.len().min(255);
        a.1[..n].copy_from_slice(&e.1.as_bytes()[..n]);
    });
}
fn guard<T: Copy>(fallback: T, f: impl FnOnce() -> R<T>) -> T {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(Ok(v)) => v,
        Ok(Err(e)) => {
            report(e);
            fallback
        }
        Err(_) => {
            report((5, "internal adapter panic"));
            fallback
        }
    }
}
#[no_mangle]
pub extern "C" fn vt_error_code() -> i32 {
    ERROR.with(|a| a.borrow().0)
}
#[no_mangle]
pub extern "C" fn vt_error_message() -> *const c_char {
    ERROR.with(|a| a.borrow().1.as_ptr().cast())
}
fn product(a: usize, b: usize) -> R<usize> {
    verified_tiff::mul(a,b).map_err(err)
}
mod storage;
mod stream;
mod directory;
mod decoder;
mod lerc;
mod webp;
mod color;
mod metadata;
mod writer;
use storage::{Buf,Mem,Memory,Source,Backend,ReadProc,SeekProc};
use directory::*;
use verified_tiff::container;
#[derive(Clone,Copy)]
struct Tag{key:u16,kind:u16,count:usize,at:usize,remote:Option<u64>}
struct Meta{
    tags:Buf<Tag>,off:u64,next:u64,le:bool,width:usize,height:usize,ch:usize,bits:usize,
    format:u16,photo:u64,planar:usize,rows:usize,bw:usize,bh:usize,tiled:bool,
    offsets:Striles,counts:Striles,map:Buf<u16>,extra:Buf<u16>,subifds:Buf<u64>,
    compression:u16,predictor:u16,orientation:usize,fill:u16,
}
struct Handle{
    source:Source,header:container::Header,data:Buf<u8>,meta:Meta,page:u32,limits:Limits,
    work:usize,chain_steps:usize,stream:Option<stream::Cached>,cache:Option<(usize,Buf<u8>)>,rendered:Option<Buf<u8>>,jpeg_rgb:bool,logluv16:bool,chop:bool,converted:Buf<metadata::Converted>,directory_kind:u32,writing:bool,
}
impl Handle{
    fn open(mut source:Source,output:usize,work:usize,flags:u32)->R<Self>{
        if source.len>source.input as u64{return Err((LIMIT,"input byte limit"));}
        let writing=flags&4!=0;
        let header=if writing{container::Header{little_endian:flags&16==0,big:flags&8!=0,first_ifd:0}}else{let b=source.read(0,source.len.min(16) as usize)?;container::header(&b).map_err(err)?};
        let limits=Limits{max_input:source.input,max_output:output,..Limits::conservative()};
        let chop=flags&1!=0;
        let (data,meta)=if writing||flags&2!=0{empty_metadata(&source.mem,header)?}else{metadata(&mut source,header,header.first_ifd,&limits,chop)?};
        let converted=Buf::new(0,&source.mem)?;
        Ok(Self{writing,converted,directory_kind:0,source,header,data,meta,page:0,limits,work,chain_steps:0,stream:None,cache:None,rendered:None,jpeg_rgb:false,logluv16:false,chop})
    }
    fn set_offset(&mut self,off:u64,page:u32)->R<()>{
        let (data,meta)=metadata(&mut self.source,self.header,off,&self.limits,self.chop)?;
        self.converted=Buf::new(0,&self.source.mem)?;self.directory_kind=0;self.data=data;self.meta=meta;self.page=page;self.chain_steps=0;self.stream=None;self.cache=None;self.rendered=None;self.jpeg_rgb=false;self.logluv16=false;Ok(())
    }
    fn source_channels(&self)->R<usize>{Ok(self.meta.ch)}
    fn raw_row(&self,w:usize)->R<usize>{
        if self.ycbcr_raw(){let (hs,vs)=self.sampling()?;return Ok(product((w-1)/hs+1,hs*vs+2)?/vs);}
        if self.meta.photo==6 && !self.jpeg_rgb{return Err((NOTIMPL,"native subsampled YCbCr not implemented; request JPEGCOLORMODE_RGB"));}
        let row=self.meta.row_bytes(w)?;
        if self.meta.compression==34676 && self.meta.photo==32845 && !self.logluv16{product(row,2)}else{Ok(row)}
    }
}
fn empty_metadata(mem:&Mem,header:container::Header)->R<(Buf<u8>,Meta)>{
    Ok((Buf::new(0,mem)?,Meta{tags:Buf::new(0,mem)?,off:0,next:header.first_ifd,le:header.little_endian,width:0,height:0,ch:1,bits:1,format:1,photo:1,planar:1,rows:0,bw:0,bh:0,tiled:false,offsets:Striles::empty(mem)?,counts:Striles::empty(mem)?,map:Buf::new(0,mem)?,extra:Buf::new(0,mem)?,subifds:Buf::new(0,mem)?,compression:1,predictor:1,orientation:1,fill:1}))
}
// Safety: only the C shim can call these hidden exports. A live handle must
// be exclusively accessed for the duration of a call; C owns its lifetime.
unsafe fn handle<'a>(p: *mut c_void) -> R<&'a mut Handle> {
    if p.is_null() {
        Err((INVALID, "null TIFF handle"))
    } else {
        Ok(&mut *p.cast::<Handle>())
    }
}
#[no_mangle]
pub unsafe extern "C" fn vt_open_fd(fd:i32,input:usize,output:usize,work:usize,single:usize,cumulative:usize,base:usize,flags:u32)->*mut c_void{
    guard(ptr::null_mut(),||{
        if fd<0{return Err((INVALID,"invalid file descriptor"));}
        let file=std::mem::ManuallyDrop::new(std::fs::File::from_raw_fd(fd));
        let len=file.metadata().map_err(|_|(5,"input stat failed"))?.len();
        let mem=Memory::new(single,cumulative,base.checked_add(std::mem::size_of::<Handle>()).ok_or((LIMIT,"handle size overflow"))?)?;
        let source=Source{read_buffer:None,backend:Backend::Fd(fd),len,mem,input};
        Ok(Box::into_raw(Box::new(Handle::open(source,output,work,flags)?)).cast())
    })
}
#[no_mangle]
pub unsafe extern "C" fn vt_open_client(client:*mut c_void,read:ReadProc,seek:SeekProc,len:u64,input:usize,output:usize,work:usize,single:usize,cumulative:usize,base:usize,flags:u32)->*mut c_void{
    guard(ptr::null_mut(),||{
        let mem=Memory::new(single,cumulative,base.checked_add(std::mem::size_of::<Handle>()).ok_or((LIMIT,"handle size overflow"))?)?;
        let source=Source{read_buffer:None,backend:Backend::Client{client,read,seek},len,mem,input};
        Ok(Box::into_raw(Box::new(Handle::open(source,output,work,flags)?)).cast())
    })
}
#[no_mangle]
pub unsafe extern "C" fn vt_close(h: *mut c_void) {
    guard((), || {
        if !h.is_null() {
            drop(Box::from_raw(h.cast::<Handle>()));
        }
        Ok(())
    });
}
#[no_mangle]
pub unsafe extern "C" fn vt_select(p: *mut c_void, page: u32) -> i32 {
    guard(0, || {
        let h = handle(p)?;
        let off=page_offset(&mut h.source,h.header,Some(page),&h.limits)?.0;
        h.set_offset(off,page)?;
        Ok(1)
    })
}
// Scalar metadata operations; output points to a C-owned uint64_t.
#[no_mangle]
pub unsafe extern "C" fn vt_scalar(p: *mut c_void, key: u32, defaulted: i32, out: *mut u64) -> i32 {
    guard(-1, || {
        let h = handle(p)?;
        if out.is_null() {
            return Err((INVALID, "null metadata output"));
        }
        let m = &h.meta;
        let def = match key {
            258 | 266 | 274 | 277 | 284 | 317 | 339 | 259 | 263 | 332 | 531 => Some(1),
            296 => Some(2),
            292 | 293 | 254 | 280 => Some(0),
            281 => Some((1u64 << m.bits.min(16))-1),
            334 => Some(4),
            278 => Some(u32::MAX as u64),
            _ => None,
        };
        let value = if m.compression==34676 && m.photo==32845 && !h.logluv16 && (key==258||key==339){if key==258{32}else{3}} else if key == 65538 {
            h.jpeg_rgb as u64
        } else if key == 65560 {
            if h.logluv16 {
                1
            } else {
                0
            }
        } else if key==278 && m.rows>0 {m.rows as u64} else {
            let Some(t) = find(&m.tags, key as u16) else {
                if defaulted != 0 {
                    if let Some(v) = def {
                        *out = v;
                        return Ok(1);
                    }
                }
                return Ok(0);
            };
            value(&h.data, t, 0, m.le)?
        };
        *out = value;
        Ok(1)
    })
}
#[no_mangle]
pub unsafe extern "C" fn vt_info(p: *mut c_void, key: u32) -> u64 {
    guard(u64::MAX, || {
        let h = handle(p)?;
        let m = &h.meta;
        Ok(match key {
            0 => m.width as u64,
            1 => m.height as u64,
            2 => h.page as u64,
            3 => m.off as u64,
            4 => m.next as u64,
            5 => m.tiled as u64,
            6 => m.le as u64,
            7 => m.offsets.len() as u64,
            8 => m.bw as u64,
            9 => m.bh as u64,
            10 => m.rows as u64,
            11 => m.planar as u64,
            12 => h.source_channels()? as u64,
            13 => h.limits.max_output as u64,
            14 => h.header.big as u64,
            15 => h.meta.tags.len() as u64,
            _ => return Err((INVALID, "unknown adapter info")),
        })
    })
}
#[no_mangle]
pub unsafe extern "C" fn vt_array(p: *mut c_void, key: u32, len: *mut usize) -> *const c_void {
    guard(ptr::null(), || {
        let h = handle(p)?;
        if len.is_null() {
            return Err((INVALID, "null array length"));
        }
        let m = &mut h.meta;
        if find(&m.tags,key as u16).is_none(){return Ok(ptr::null());}
        if matches!(key,273|324){m.offsets.materialize(&mut h.source)?;}
        if matches!(key,279|325){m.counts.materialize(&mut h.source)?;}
        let (at, n) = match key {
            273 | 324 => (m.offsets.values.as_ptr().cast(), m.offsets.len()),
            279 | 325 => (m.counts.values.as_ptr().cast(), m.counts.len()),
            320 => (m.map.as_ptr().cast(), m.map.len()),
            338 => (m.extra.as_ptr().cast(), m.extra.len()),
            330 => (m.subifds.as_ptr().cast(),m.subifds.len()),
            _ => return Err((NOTIMPL, "metadata array not implemented")),
        };
        *len = n;
        Ok(if n == 0 { ptr::null() } else { at })
    })
}
#[no_mangle]
pub unsafe extern "C" fn vt_count_dirs(p: *mut c_void) -> u32 {
    guard(0, || {
        let h = handle(p)?;
        Ok(page_offset(&mut h.source,h.header,None,&h.limits)?.1)
    })
}
#[no_mangle]
pub unsafe extern "C" fn vt_set_offset(p: *mut c_void, offset: u64) -> i32 {
    guard(0, || {
        let h = handle(p)?;
        h.set_offset(offset,u32::MAX)?;
        Ok(1)
    })
}
#[no_mangle]
pub unsafe extern "C" fn vt_size(p: *mut c_void, kind: u32, rows: u32) -> i64 {
    guard(-1, || {
        let h = handle(p)?;
        let m = &h.meta;
        if h.meta.width==0 {return Err((INVALID,"no selected image directory"));}
        if kind > 3 {
            return Err((INVALID, "invalid size operation"));
        }
        if kind >= 2 && !m.tiled {
            return Err((INVALID, "tile size requested for strip image"));
        }
        let w = if kind == 2 || kind == 3 {
            m.bw
        } else {
            m.width
        };
        let n = if kind == 0 || kind == 2 {
            1
        } else if rows == u32::MAX {
            m.bh
        } else {
            rows as usize
        };
        let size=if kind==1||kind==3{if n==0{0}else{let (row,rows)=h.native_layout(w,n)?;product(row,rows)?}}else{h.raw_row(w)?};
        if size > i64::MAX as usize {
            return Err((LIMIT, "size overflow"));
        }
        Ok(size as i64)
    })
}
#[no_mangle]
pub unsafe extern "C" fn vt_mode(p: *mut c_void, tag: u32, v: i32) -> i32 {
    guard(0, || {
        let h = handle(p)?;
        h.stream=None;h.cache=None;
        match tag {
            65538 if h.meta.photo == 6 && (v == 0 || v == 1) => h.jpeg_rgb = v == 1,
            65560 if h.meta.photo == 32845 && (v==0||v==1) => h.logluv16 = v==1,
            _ => return Err((NOTIMPL, "TIFFSetField mutation not implemented")),
        }
        Ok(1)
    })
}
#[no_mangle]
pub unsafe extern "C" fn vt_rgba_ok(p: *mut c_void) -> i32 {
    guard(0, || {
        let h = handle(p)?;
        h.rgba_ok()?;
        Ok(1)
    })
}
// Output lives until the next mutating operation. C copies only after success.
#[no_mangle]
pub unsafe extern "C" fn vt_read(p:*mut c_void,kind:u32,a:u32,b:u32,c:u32,d:u32,len:*mut usize)->*const u8{
    guard(ptr::null(),||{
        let h=handle(p)?;if len.is_null(){return Err((INVALID,"null length output"));}
        if h.meta.width==0{return Err((INVALID,"no selected image directory"));}
        h.read(kind,a as usize,b as usize,c as usize,d as usize,usize::MAX)?;
        let result=h.rendered.as_ref().ok_or((5,"missing output"))?;
        *len=result.len();Ok(result.as_ptr())
    })
}
#[no_mangle]
pub unsafe extern "C" fn vt_next(p:*mut c_void)->i32{
    guard(0,||{let h=handle(p)?;let off=h.meta.next;if off==0{return Ok(0);}
        if off==h.meta.off{return Err((40,"cyclic directory chain"));}
        let steps=verified_tiff::adapter_bounds::directory_step(h.chain_steps,h.limits.max_directories).map_err(err)?;
        let page=if h.meta.off==0{0}else{h.page.saturating_add(1)};h.set_offset(off,page)?;h.chain_steps=steps;Ok(1)})
}
#[no_mangle]
pub unsafe extern "C" fn vt_field(p:*mut c_void,key:u32,kind:*mut u16,count:*mut usize)->*const u8{
    guard(ptr::null(),||{let h=handle(p)?;if kind.is_null()||count.is_null(){return Err((INVALID,"null metadata output"));}let Some(t)=find(&h.meta.tags,key as u16) else{return Ok(ptr::null());};if t.remote.is_some(){return Err((NOTIMPL,"use strile accessor for deferred arrays"));}*kind=t.kind;*count=t.count;Ok(h.data.as_ptr().add(t.at))})
}
#[no_mangle]
pub unsafe extern "C" fn vt_real(p:*mut c_void,key:u32,out:*mut f32)->i32{
    guard(-1,||{let h=handle(p)?;if out.is_null(){return Err((INVALID,"null metadata output"));}let Some(t)=find(&h.meta.tags,key as u16) else{return Ok(0);};if t.count!=1{return Err((INVALID,"scalar real field count"));}
        let v=match t.kind{5=>{let a=u32at(&h.data,t.at,h.meta.le)?;let b=u32at(&h.data,t.at+4,h.meta.le)?;if b==0{return Err((INVALID,"zero rational denominator"));}a as f32/b as f32},11=>f32::from_bits(u32at(&h.data,t.at,h.meta.le)?),12=>f64::from_bits(verified_tiff::native_samples::read_word(&h.data,t.at,8,h.meta.le).map_err(err)?) as f32,_=>return Err((NOTIMPL,"real field type not implemented"))};*out=v;Ok(1)})
}
#[no_mangle]
pub unsafe extern "C" fn vt_tag(p:*mut c_void,index:usize)->u32{guard(u32::MAX,||{let h=handle(p)?;Ok(h.meta.tags.get(index).ok_or((INVALID,"tag index out of range"))?.key as u32)})}
#[no_mangle]
pub unsafe extern "C" fn vt_region(p:*mut c_void,x:u32,y:u32,w:u32,ht:u32,orientation:u32,len:*mut usize)->*const u8{
    guard(ptr::null(),||{let h=handle(p)?;if len.is_null(){return Err((INVALID,"null length"));}if x as usize>=h.meta.width||y as usize>=h.meta.height||w as usize>h.meta.width-x as usize||ht as usize>h.meta.height-y as usize{return Err((INVALID,"RGBA region outside image"));}h.rgba(x as usize,y as usize,w as usize,ht as usize,orientation as usize)?;let b=h.rendered.as_ref().ok_or((5,"missing RGBA output"))?;*len=b.len();Ok(b.as_ptr())})
}

#[no_mangle]
pub unsafe extern "C" fn vt_pair(p:*mut c_void,key:u32,defaulted:i32,out:*mut u16)->i32{
    guard(-1,||{let h=handle(p)?;if out.is_null(){return Err((INVALID,"null pair output"));}
        let pair=if let Some(t)=find(&h.meta.tags,key as u16){if t.count!=2{return Err((INVALID,"pair field count"));}[u16::try_from(value(&h.data,t,0,h.meta.le)?).map_err(|_|(INVALID,"pair overflow"))?,u16::try_from(value(&h.data,t,1,h.meta.le)?).map_err(|_|(INVALID,"pair overflow"))?]}else if defaulted!=0 && key==530{[2,2]}else{return Ok(0);};
        *out=pair[0];*out.add(1)=pair[1];Ok(1)})
}

#[no_mangle]
pub unsafe extern "C" fn vt_user_buffer(p:*mut c_void,k:u32,input:*const u8,n:usize,wanted:usize)->*const u8{
    guard(ptr::null(),||{
        let h=handle(p)?;h.native_ok()?;
        if n>isize::MAX as usize||n>h.limits.max_input||wanted>h.limits.max_output{return Err((LIMIT,"user buffer limit"));}
        if input.is_null()&&n!=0{return Err((INVALID,"null user input"));}
        let mut encoded=Buf::new(n,&h.source.mem)?;
        if n!=0{encoded.extend(std::slice::from_raw_parts(input,n))?;}
        let mut budget=Budget{remaining:h.work};
        let decoded=h.decode_segment(k as usize,encoded,&mut budget)?;
        if wanted>decoded.len(){return Err((INVALID,"user output exceeds decoded segment"));}
        h.rendered=Some(decoded);Ok(h.rendered.as_ref().unwrap().as_ptr())
    })
}

#[no_mangle]
pub unsafe extern "C" fn vt_strile(p:*mut c_void,k:u32,counts:i32,out:*mut u64)->i32{
    guard(0,||{let h=handle(p)?;if out.is_null(){return Err((INVALID,"null strile output"));}
        *out=if counts!=0{h.meta.counts.get(&mut h.source,k as usize)?}else{h.meta.offsets.get(&mut h.source,k as usize)?};Ok(1)})
}

#[no_mangle]
pub unsafe extern "C" fn vt_read_buffer(p:*mut c_void,buffer:*mut u8,n:usize)->i32{
    guard(0,||{let h=handle(p)?;if n==0||n>isize::MAX as usize{return Err((INVALID,"invalid read buffer size"));}
        let b=if buffer.is_null(){storage::ReadBuffer::Owned(Buf::zeros(n,h.limits.max_input,&h.source.mem)?)}else{storage::ReadBuffer::Borrowed(buffer,n)};
        h.source.read_buffer=Some(b);h.stream=None;h.cache=None;Ok(1)})
}
#[no_mangle]
pub unsafe extern "C" fn vt_unset(p:*mut c_void,key:u32)->i32{
    guard(0,||{let h=handle(p)?;
        if h.directory_kind==0&&key==334&&find(&h.meta.tags,333).is_some(){return Err((INVALID,"NumberOfInks is required by InkNames"));}
        let mut tags=Buf::new(h.meta.tags.len(),&h.source.mem)?;
        for t in h.meta.tags.iter(){if t.key as u32!=key{tags.push(*t)?;}}
        h.meta.tags=tags;h.converted=Buf::new(0,&h.source.mem)?;h.stream=None;h.cache=None;h.rendered=None;Ok(1)})
}
#[no_mangle]
pub unsafe extern "C" fn vt_free_directory(p:*mut c_void)->i32{
    guard(0,||{let h=handle(p)?;let (data,mut meta)=empty_metadata(&h.source.mem,h.header)?;
        meta.off=h.meta.off;meta.next=h.meta.next;
        h.meta=meta;h.data=data;h.converted=Buf::new(0,&h.source.mem)?;h.stream=None;h.cache=None;h.rendered=None;Ok(1)})
}

#[no_mangle]
pub unsafe extern "C" fn vt_read_size(p:*mut c_void,kind:u32,a:u32,b:u32,wanted:usize,len:*mut usize)->*const u8{
    guard(ptr::null(),||{let h=handle(p)?;if len.is_null(){return Err((INVALID,"null length"));}
        if h.meta.width==0{return Err((INVALID,"no selected image directory"));}
        h.read(kind,a as usize,b as usize,0,0,wanted)?;
        let out=h.rendered.as_ref().ok_or((5,"missing read output"))?;*len=out.len();Ok(out.as_ptr())})
}

#[no_mangle]
pub unsafe extern "C" fn vt_clientdata(p:*mut c_void,client:*mut c_void)->i32{
    guard(0,||{let h=handle(p)?;if let Backend::Client{client:current,..}=&mut h.source.backend{*current=client;}
        h.stream=None;h.cache=None;h.rendered=None;Ok(1)})
}

/// Caller owns a writable destination of `capacity` bytes and a
/// readable source of `n` bytes. The bridge rejects overlap before creating
/// references. The checked core proves the exact copy and unchanged frame.
#[no_mangle]
pub unsafe extern "C" fn vt_commit(dst:*mut u8,capacity:usize,at:usize,src:*const u8,n:usize)->i32{
    guard(0,||{
        if capacity>isize::MAX as usize||n>isize::MAX as usize||!verified_tiff::checked_range(capacity,at,n){return Err((LIMIT,"output commit extent"));}
        if n==0{return Ok(1);}
        if dst.is_null()||src.is_null(){return Err((INVALID,"null output commit"));}
        let de=(dst as usize).checked_add(capacity).ok_or((INVALID,"destination address wrap"))?;
        let se=(src as usize).checked_add(n).ok_or((INVALID,"source address wrap"))?;
        if (dst as usize)<se&&(src as usize)<de{return Err((INVALID,"overlapping output commit"));}
        // MaybeUninit permits C's ordinary uninitialized malloc buffers. The
        // checked loop writes every byte before it can be treated as initialized.
        let output=std::slice::from_raw_parts_mut(dst.add(at).cast::<std::mem::MaybeUninit<u8>>(),n);
        let input=std::slice::from_raw_parts(src,n);
        verified_tiff::adapter_bounds::commit_uninit(input,output).map_err(err)?;Ok(1)
    })
}
