//! Unverified seekable I/O and per-handle requested-allocation accounting.
use super::*;
use verified_tiff::adapter_bounds as checked;
use std::{ops::{Deref,DerefMut},sync::{Arc,atomic::{AtomicUsize,AtomicBool,Ordering}},os::unix::fs::FileExt};

pub struct Memory {single:usize,cumulative:usize,used:AtomicUsize,poisoned:AtomicBool}
pub type Mem=Arc<Memory>;
impl Memory {
    /// Conservative admission check for optional scratch. Actual allocations
    /// still claim their bytes; a failed check selects the bounded row path.
    pub fn fits(&self,single:usize,total:usize)->bool{
        !self.poisoned.load(Ordering::Relaxed) && checked::claim(0,single,self.single,0).is_ok() && checked::claim(self.used.load(Ordering::Relaxed),total,0,self.cumulative).is_ok()
    }
    pub fn new(single:usize,cumulative:usize,base:usize)->R<Mem>{
        let m=Arc::new(Self{single,cumulative,used:AtomicUsize::new(0),poisoned:AtomicBool::new(false)});
        m.claim(base)?;Ok(m)
    }
    pub fn claim(&self,n:usize)->R<()> {
        if self.poisoned.load(Ordering::Relaxed){return Err((INVALID,"invalid allocation ledger"));}
        self.used.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|old|checked::claim(old,n,self.single,self.cumulative).ok())
            .map(|_|()).map_err(|_|(LIMIT,"allocation limit or overflow"))
    }
    pub fn release(&self,n:usize){
        if self.used.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|old|checked::release(old,n).ok()).is_err(){
            self.poisoned.store(true,Ordering::Relaxed);
        }
    }
}
pub struct Buf<T>{v:Vec<T>,mem:Mem,charge:usize,capacity:usize}
impl<T> Buf<T>{
    pub fn truncate(&mut self,n:usize){self.v.truncate(n);}
    pub fn reserve(&mut self,n:usize)->R<()> {
        if n<=self.capacity{return Ok(());}
        let charge=checked::allocation(n,std::mem::size_of::<T>()).map_err(err)?;
        // Account for the old and replacement allocations while growing.
        self.mem.claim(charge)?;
        let mut v=Vec::new();
        if v.try_reserve_exact(n).is_err(){self.mem.release(charge);return Err((NOMEM,"allocation failed"));}
        v.append(&mut self.v);self.v=v;
        self.mem.release(self.charge);self.charge=charge;self.capacity=n;Ok(())
    }
    pub fn new(n:usize,mem:&Mem)->R<Self>{
        let charge=checked::allocation(n,std::mem::size_of::<T>()).map_err(err)?;
        mem.claim(charge)?;
        let mut v=Vec::new();
        if v.try_reserve_exact(n).is_err(){mem.release(charge);return Err((NOMEM,"allocation failed"));}
        Ok(Self{v,mem:mem.clone(),charge,capacity:n})
    }
    pub fn push(&mut self,value:T)->R<()>{if self.v.len()==self.capacity{return Err((LIMIT,"buffer capacity"));}self.v.push(value);Ok(())}
}
impl<T:Clone> Buf<T>{
    pub fn filled(n:usize,value:T,mem:&Mem)->R<Self>{let mut b=Self::new(n,mem)?;b.v.resize(n,value);Ok(b)}
pub fn extend(&mut self,s:&[T])->R<()>{if s.len()>self.capacity-self.v.len(){return Err((LIMIT,"buffer capacity"));}self.v.extend_from_slice(s);Ok(())}}
impl Buf<u8>{pub fn zeros(n:usize,cap:usize,mem:&Mem)->R<Self>{if n>cap{return Err((LIMIT,"buffer byte limit"));}let mut b=Self::new(n,mem)?;b.v.resize(n,0);Ok(b)}}
impl<T> Deref for Buf<T>{type Target=[T];fn deref(&self)->&[T]{&self.v}}
impl<T> DerefMut for Buf<T>{fn deref_mut(&mut self)->&mut[T]{&mut self.v}}
impl<T> Drop for Buf<T>{fn drop(&mut self){self.mem.release(self.charge);}}
pub type ReadProc=unsafe extern "C" fn(*mut c_void,*mut c_void,isize)->isize;
pub type SeekProc=unsafe extern "C" fn(*mut c_void,u64,i32)->u64;
pub enum Backend{Fd(i32),Client{client:*mut c_void,read:ReadProc,seek:SeekProc}}
pub enum ReadBuffer{Owned(Buf<u8>),Borrowed(*mut u8,usize)}
pub struct Source{pub read_buffer:Option<ReadBuffer>,pub backend:Backend,pub len:u64,pub mem:Mem,pub input:usize}
impl Source{
    pub fn read(&mut self,off:u64,n:usize)->R<Buf<u8>>{
        if !checked::source_range(self.len,off,n){return Err((INVALID,"read outside input"));}
        let mut out=Buf::zeros(n,self.input,&self.mem)?;
        self.read_into(off,&mut out)?;
        Ok(out)
    }
    pub fn read_into(&mut self,off:u64,out:&mut[u8])->R<()>{
        if !checked::source_range(self.len,off,out.len()){return Err((INVALID,"read outside input"));}
        // The configured buffer is borrowed from the caller or owned here;
        // it is used as the I/O staging buffer, never freed when borrowed.
        if let Some(mut buffer)=self.read_buffer.take(){
            let result=(||{
                let bytes=match &mut buffer{ReadBuffer::Owned(b)=>&mut **b,ReadBuffer::Borrowed(p,n)=>unsafe{std::slice::from_raw_parts_mut(*p,*n)}};
                for (i,chunk) in out.chunks_mut(bytes.len()).enumerate(){
                    self.direct_read(off+(i*bytes.len()) as u64,&mut bytes[..chunk.len()])?;
                    chunk.copy_from_slice(&bytes[..chunk.len()]);
                }Ok(())
            })();
            self.read_buffer=Some(buffer);result
        }else{self.direct_read(off,out)}
    }
    fn direct_read(&mut self,off:u64,out:&mut[u8])->R<()>{
        match &self.backend{
            Backend::Fd(fd)=>{
                let file=std::mem::ManuallyDrop::new(unsafe{std::fs::File::from_raw_fd(*fd)});
                file.read_exact_at(out,off).map_err(|_|(5,"input read failed"))?;
            }
            Backend::Client{client,read,seek}=>unsafe{
                if seek(*client,off,0)!=off{return Err((5,"callback seek failed"));}
                let mut at=0;
                while at<out.len(){
                    let want=(out.len()-at).min(65536);
                    let got=read(*client,out.as_mut_ptr().add(at).cast(),want as isize);
                    if got<=0 || got as usize>want{return Err((5,"short/invalid callback read"));}
                    at+=got as usize;
                }
            }
        }
        Ok(())
    }

}
