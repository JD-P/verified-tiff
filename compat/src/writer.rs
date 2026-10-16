//! Metadata-only temporary TIFF construction for read-side codec clients.
//! This does not implement image encoding or modify an opened reader source.
use super::*;
fn append_number(b:&mut Buf<u8>,v:u64,w:usize,le:bool)->R<()>{let a=if le{v.to_le_bytes()}else{v.to_be_bytes()};b.extend(if le{&a[..w]}else{&a[8-w..]})}
impl Handle{
    pub fn write_check(&mut self)->R<()>{
        if !self.writing{return Err((NOTIMPL,"reader is not writable"));}
        let le=self.header.little_endian;
        let sc=|key,d|scalar(&self.data,&self.meta.tags,key,d,le);
        let width=sc(256,0)?;let height=sc(257,0)?;let ch=sc(277,1)?;let planar=sc(284,1)?;
        if width==0||height==0||width>u32::MAX as u64||height>u32::MAX as u64||ch==0||ch>64||!matches!(planar,1|2){return Err((INVALID,"invalid temporary image geometry"));}
        let tiled=find(&self.meta.tags,322).is_some();
        let bw=if tiled{sc(322,0)?}else{width};let bh=if tiled{sc(323,0)?}else{sc(278,height)?.min(height)};
        if bw==0||bh==0||bw>u32::MAX as u64||bh>u32::MAX as u64{return Err((INVALID,"invalid temporary block geometry"));}
        let count=product(product(((width-1)/bw+1) as usize,((height-1)/bh+1) as usize)?,if planar==2{ch as usize}else{1})?;
        if count>self.limits.max_segments{return Err((LIMIT,"temporary segment count"));}
        let kind=if self.header.big{16}else{4};let bytes=product(count,container::field_size(kind))?;
        let zeros=Buf::zeros(bytes,self.limits.max_output,&self.source.mem)?;
        for tag in if tiled{[324,325]}else{[273,279]}{if find(&self.meta.tags,tag).is_none(){self.put(tag,kind,count,&zeros)?;}}
        // Validate assembled geometry using an independent copy; existing tag
        // pointers remain stable until a successful mutation/selection.
        let mut data=Buf::new(self.data.len(),&self.source.mem)?;data.extend(&self.data)?;
        let mut tags=Buf::new(self.meta.tags.len(),&self.source.mem)?;tags.extend(&self.meta.tags)?;
        let (data,meta)=assemble(data,tags,0,0,le,&self.limits,false,&self.source.mem)?;
        self.data=data;self.meta=meta;self.converted=Buf::new(0,&self.source.mem)?;Ok(())
    }
    fn directory_bytes(&mut self)->R<()>{
        self.write_check()?;
        let big=self.header.big;let le=self.header.little_endian;
        let hs=if big{16}else{8};let cw=if big{8}else{2};let ew=if big{20}else{12};let slot=if big{8}else{4};
        let table=product(self.meta.tags.len(),ew)?.checked_add(hs+cw+slot).ok_or((LIMIT,"directory size"))?;
        let mut total=table;
        for t in self.meta.tags.iter(){let n=product(t.count,container::field_size(t.kind))?;if n>slot{total=total.checked_add(n).and_then(|n|n.checked_add(7)).ok_or((LIMIT,"directory size"))?;}}
        if !big&&total>u32::MAX as usize{return Err((LIMIT,"Classic TIFF size"));}
        let mut out=Buf::new(total,&self.source.mem)?;
        out.extend(if le{b"II"}else{b"MM"})?;append_number(&mut out,if big{43}else{42},2,le)?;
        if big{append_number(&mut out,8,2,le)?;append_number(&mut out,0,2,le)?;}
        append_number(&mut out,hs as u64,slot,le)?;append_number(&mut out,self.meta.tags.len() as u64,cw,le)?;
        let mut offset=table;
        for t in self.meta.tags.iter(){
            let n=product(t.count,container::field_size(t.kind))?;
            append_number(&mut out,t.key as u64,2,le)?;append_number(&mut out,t.kind as u64,2,le)?;append_number(&mut out,t.count as u64,slot,le)?;
            if n<=slot{out.extend(&self.data[t.at..t.at+n])?;for _ in n..slot{out.push(0)?;}}
            else{offset=offset.checked_add(7).ok_or((LIMIT,"directory size"))?&!7;append_number(&mut out,offset as u64,slot,le)?;offset+=n;}
        }
        append_number(&mut out,0,slot,le)?;
        for t in self.meta.tags.iter(){let n=product(t.count,container::field_size(t.kind))?;if n>slot{while out.len()%8!=0{out.push(0)?;}out.extend(&self.data[t.at..t.at+n])?;}}
        if out.len()>self.limits.max_input{return Err((LIMIT,"temporary file limit"));}
        self.rendered=Some(out);Ok(())
    }
}
#[no_mangle]
pub unsafe extern "C" fn vt_write_check(p:*mut c_void)->i32{guard(0,||{handle(p)?.write_check()?;Ok(1)})}
#[no_mangle]
pub unsafe extern "C" fn vt_directory_bytes(p:*mut c_void,n:*mut usize)->*const u8{guard(ptr::null(),||{if n.is_null(){return Err((INVALID,"null directory output"));}let h=handle(p)?;h.directory_bytes()?;let b=h.rendered.as_ref().unwrap();*n=b.len();Ok(b.as_ptr())})}
