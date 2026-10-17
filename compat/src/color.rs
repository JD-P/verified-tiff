//! C RGBA presentation, outside the verified decoder boundary. Color samples
//! come from verified native decoding; these numerical conversions are tested.
use super::*;
use verified_tiff::{adapter_geometry as geometry,adapter_color as color};
impl Handle {
    pub fn ycbcr_raw(&self)->bool{self.meta.photo==6&&!matches!(self.meta.compression,6|7)}
    pub fn sampling(&self)->R<(usize,usize)>{
        let m=&self.meta;
        if m.bits!=8||m.ch!=3||m.planar!=1||m.predictor!=1{return Err((NOTIMPL,"YCbCr requires contiguous 8-bit samples without prediction"));}
        let (h,v)=if let Some(t)=find(&m.tags,530){if t.count!=2{return Err((INVALID,"YCbCr subsampling count"));}(value(&self.data,t,0,m.le)? as usize,value(&self.data,t,1,m.le)? as usize)}else{(2,2)};
        if !matches!(h,1|2|4)||!matches!(v,1|2|4){return Err((INVALID,"YCbCr subsampling"));}Ok((h,v))
    }
    pub fn native_layout(&self,w:usize,h:usize)->R<(usize,usize)>{
        if self.ycbcr_raw(){let (hs,vs)=self.sampling()?;Ok((product((w-1)/hs+1,hs*vs+2)?,(h-1)/vs+1))}
        else{Ok((self.raw_row(w).or_else(|_|self.meta.row_bytes(w))?,h))}
    }
    fn reals<const N:usize>(&self,key:u16,defaults:[f32;N])->R<[f32;N]>{
        let Some(t)=find(&self.meta.tags,key)else{return Ok(defaults);};if t.count!=N{return Err((INVALID,"color metadata count"));}
        let mut a=defaults;for (i,v) in a.iter_mut().enumerate(){*v=metadata::real(&self.data,t.at+i*container::field_size(t.kind),t.kind,self.meta.le)? as f32;if !v.is_finite(){return Err((INVALID,"nonfinite color metadata"));}}Ok(a)
    }
    pub fn rgba_color(&mut self,x:usize,y:usize,w:usize,h:usize,orientation:usize)->R<()>{
        if !(1..=8).contains(&orientation)||w==0||h==0{return Err((INVALID,"RGBA dimensions/orientation"));}
        let m=&self.meta;let width=m.width;let height=m.height;let bw=m.bw;let bh=m.bh;let bits=m.bits;let ch=m.ch;let planar=m.planar;let photo=m.photo;
        let alpha=m.extra.iter().position(|v|*v==1||*v==2).map(|i|(ch-m.extra.len()+i,m.extra[i]));
        let (fx,fy)=geometry::orientation(m.orientation,orientation).map_err(err)?;
        let right=x.checked_add(w).ok_or((LIMIT,"RGBA extent"))?.min(width);let bottom=y.checked_add(h).ok_or((LIMIT,"RGBA extent"))?.min(height);
        if x>=right||y>=bottom{return Err((INVALID,"empty RGBA region"));}
        let across=(width-1)/bw+1;let row=m.row_bytes(bw)?;
        let (hs,vs)=if photo==6{self.sampling()?}else{(1,1)};
        let d50=96.4250f32+100.+82.4680;let defaults=[96.4250/d50,100./d50];let white=if photo==8{self.reals(318,defaults)?}else{defaults};
        if white[1]<=0.{return Err((INVALID,"white point"));}
        let reference=[white[0]/white[1]*100.,100.,(1.-white[0]-white[1])/white[1]*100.];
        let mut gamma=[0u8;1501];
        if photo==8{for (i,v) in gamma.iter_mut().enumerate(){let q=255.*((i as f64/1500.).powf(1./(2.4f32 as f64)) as f32);*v=(q+0.5).min(255.) as u8;}}
        let luma=if photo==6{self.reals(529,[0.299,0.587,0.114])?}else{[0.299,0.587,0.114]};let rbw=if photo==6{self.reals(532,[0.,255.,128.,255.,128.,255.])?}else{[0.,255.,128.,255.,128.,255.]};
        if luma.iter().any(|v|*v<0.||*v>1.)||luma[1]<=0.||rbw.iter().any(|v|*v<0.||*v>255.){return Err((INVALID,"YCbCr conversion coefficients"));}
        let mut out=Buf::zeros(product(product(w,h)?,4)?,self.limits.max_output,&self.source.mem)?;
        let mut budget=Budget{remaining:self.work};if out.len()>budget.remaining{return Err((LIMIT,"RGBA color work limit"));}budget.remaining-=out.len();
        for by in y/bh..=(bottom-1)/bh{for bx in x/bw..=(right-1)/bw{
            let k=by*across+bx;let rr=right.min((bx+1)*bw);let bb=bottom.min((by+1)*bh);let left=x.max(bx*bw);let top=y.max(by*bh);
            let mut band=None;let mut band_index=usize::MAX;
            let needed=product(row,self.meta.segment_rows(k)?)?;
            let complete=left==bx*bw&&rr==width.min((bx+1)*bw)&&top==by*bh&&bb==height.min((by+1)*bh);
            let full=if photo!=6&&planar==1&&complete&&self.rgba_full_segment(k,needed)?{Some(self.native_range(k,0,needed,&mut budget)?)}else{None};
            for yy in top..bb{
                let ly=yy-by*bh;
                let mut words=Buf::new(if photo==6||planar==1{0}else{product(rr-left,ch)?},&self.source.mem)?;
                let mut direct=None;
                if photo==6{
                    let index=ly/vs;
                    if index!=band_index{
                        let bytes=product((bw-1)/hs+1,hs*vs+2)?;
                        band=Some(self.native_range(k,index*bytes,bytes,&mut budget)?);band_index=index;
                    }
                }else if planar==1{
                    if full.is_none(){direct=Some(self.native_range(k,ly*row,row,&mut budget)?);}
                }else{
                    for _ in 0..product(rr-left,ch)?{words.push(0u16)?;}
                    for plane in 0..if planar==2{ch}else{1}{
                        let data=self.native_range(geometry::segment(across,(height-1)/bh+1,if planar==2{ch}else{1},bx,by,plane).map_err(err)?,ly*row,row,&mut budget)?;
                        for xx in left..rr{for c in 0..if planar==2{1}else{ch}{
                            let ci=if planar==2{plane}else{c};let index=(xx-bx*bw)*if planar==2{1}else{ch}+c;
                            words[(xx-left)*ch+ci]=decoder::sample(&data,0,index,bits)? as u16;
                        }}
                    }
                }
                // One destination span per row run; a mirrored run is written last-first.
                let n=rr-left;
                let (dest,_)=geometry::crop_span(w,h,right-x,bottom-y,left-x,yy-y,n,fx,fy).map_err(err)?;
                let data=if let Some(data)=&full{Some(&data[ly*row..ly*row+row])}else{direct.as_deref()};
                let span=&mut out[dest..dest+n*4];
                if let (Some(data),true)=(data,photo!=6&&bits==8){
                    // Contiguous 8-bit CMYK/Lab pixels: the same conversions, read in place.
                    let src=&data[(left-bx*bw)*ch..(rr-bx*bw)*ch];
                    for (i,p) in src.chunks_exact(ch).enumerate(){
                        let mut q=if photo==5{color::cmyk(p[0] as u16,p[1] as u16,p[2] as u16,p[3] as u16,false)}
                            else{lab(p[0] as u16,p[1] as u16,p[2] as u16,bits,reference,&gamma)};
                        if let Some((a,kind))=alpha{q[3]=p[a];if kind==2{for c in 0..3{q[c]=((q[c] as u32*q[3] as u32+127)/255) as u8;}}}
                        let to=geometry::span_pixel(w,h,right-x,bottom-y,left-x,yy-y,n,fx,fy,i);
                        decoder::pack(&mut span[to..to+4],q);
                    }
                    continue;
                }
                for i in 0..n{
                let xx=left+i;let lx=xx-bx*bw;let off=i*ch;
                let component=|c:usize|->u16{
                    if let Some(data)=data{let p=(lx*ch+c)*(bits/8);if bits==8{data[p] as u16}else{u16::from_ne_bytes([data[p],data[p+1]])}}else{words[off+c]}
                };
                let mut q=if photo==6{
                    let at=(lx/hs)*(hs*vs+2);let data=band.as_ref().unwrap();
                    ycbcr(data[at+(ly%vs)*hs+lx%hs],data[at+hs*vs],data[at+hs*vs+1],luma,rbw)
                }else if photo==5{
                    color::cmyk(component(0),component(1),component(2),component(3),bits==16)
                }else{lab(component(0),component(1),component(2),bits,reference,&gamma)};
                if let Some((a,kind))=alpha{q[3]=if bits==8{component(a) as u8}else{((component(a) as u32+128)/257) as u8};if kind==2{for c in 0..3{q[c]=((q[c] as u32*q[3] as u32+127)/255) as u8;}}}
                let to=geometry::span_pixel(w,h,right-x,bottom-y,left-x,yy-y,n,fx,fy,i);
                decoder::pack(&mut span[to..to+4],q);
            }}
        }}
        self.rendered=Some(out);Ok(())
    }
}
fn ycbcr(y:u8,cb:u8,cr:u8,l:[f32;3],r:[f32;6])->[u8;4]{
    let fix=|v:f32|((v.clamp(0.,2.)*65536.)+0.5) as i64;
    let a=2.-2.*l[0];let b=2.-2.*l[2];let d=[fix(a),-fix(l[0]*a/l[1]),fix(b),-fix(l[2]*b/l[1])];
    let code=|v:f32,lo:f32,hi:f32,range:f32|(((v-lo as i32 as f32)*range)/if hi==lo{1.}else{hi-lo}).clamp(-4096.,4096.) as i64;
    let y=code(y as f32,r[0],r[1],255.);let cb=code(cb as f32-128.,r[2]-128.,r[3]-128.,127.);let cr=code(cr as f32-128.,r[4]-128.,r[5]-128.,127.);
    [(y+((d[0]*cr+32768)>>16)).clamp(0,255) as u8,(y+((d[1]*cr+d[3]*cb+32768)>>16)).clamp(0,255) as u8,(y+((d[2]*cb+32768)>>16)).clamp(0,255) as u8,255]
}
fn lab(l:u16,a:u16,b:u16,bits:usize,white:[f32;3],gamma:&[u8;1501])->[u8;4]{
    let l=if bits==8{l as u32*257}else{l as u32} as f32*100./65535.;
    let a=if bits==8{a as u8 as i8 as f32}else{a as i16 as f32/256.};let b=if bits==8{b as u8 as i8 as f32}else{b as i16 as f32/256.};
    let (y,cy)=if l<8.856{let y=l*white[1]/903.292;(y,7.787*(y/white[1])+16./116.)}else{let cy=(l+16.)/116.;(white[1]*cy*cy*cy,cy)};
    let f=|v:f32,w:f32|if v<0.2069{w*(v-0.13793)/7.787}else{w*v*v*v};let x=f(a/500.+cy,white[0]);let z=f(cy-b/200.,white[2]);
    let values=[3.2410*x-1.5374*y-0.4986*z,-0.9692*x+1.8760*y+0.0416*z,0.0556*x-0.2040*y+1.0570*z];let mut out=[0,0,0,255];
    for c in 0..3{let i=color::f32_index(((values[c].clamp(1.,100.)-1.)/(99./1500.)).to_bits(),1500) as usize;out[c]=gamma[i];}out
}

#[no_mangle]
pub extern "C" fn vt_color_index(bits:u32,cap:u32)->u32{color::f32_index(bits,cap)}
#[no_mangle]
pub extern "C" fn vt_ycbcr_tables(y:i32,rr:i32,bb:i32,cg:i32,rg:i32)->u32{
    let v=color::ycbcr_tables(y,rr,bb,cg,rg);u32::from_le_bytes([v[0],v[1],v[2],0])
}
