use super::*;
use super::extended::*;

verus! {

// Output byte geometry and its exact relationship to native, planar storage.
#[derive(Clone,Copy)]
pub(super) struct RasterLayout {
    pub w:usize,pub h:usize,pub ch:usize,pub bytes:usize,pub slots:usize,
    pub palette:bool,pub separate:bool,pub le:bool,pub white:bool,pub alpha:usize,
    pub encoded_width:usize,pub plane_channels:usize,pub planes:usize,
    pub plane_len:usize,pub scratch_len:usize,pub output_len:usize,
    pub colors:usize,pub map:Tag,
}

impl RasterLayout {
    pub(super) open spec fn valid(self) -> bool {
        &&& self.w>0 && self.h>0 && 1<=self.ch<=4
        &&& self.bytes==1 || self.bytes==2 || self.bytes==8
        &&& (self.bytes==8 ==> !self.white && !self.palette && self.alpha==0 && self.slots==1 && (self.ch==1 || self.ch==3))
        &&& self.alpha<=2
        &&& (self.alpha!=0 ==> self.bytes==1 && (self.ch==2 || self.ch==4) && !self.palette)
        &&& (self.palette ==> self.ch==3 && self.bytes==1 && (self.slots==1 || self.slots==2 || self.slots==8) && !self.separate && self.alpha==0)
        &&& (!self.palette ==> self.slots==1 || (self.slots==8 && self.ch==1 && self.bytes==1 && self.alpha==0))
        &&& self.encoded_width as int==(self.w as int-1)/(self.slots as int)+1
        &&& self.plane_channels as int==(if self.palette {1int} else if self.separate {self.bytes as int} else {self.ch as int*self.bytes})
        &&& self.planes==(if self.separate {self.ch} else {1usize})
        &&& self.plane_len as int==self.encoded_width as int*self.h*self.plane_channels
        &&& self.scratch_len as int==self.plane_len as int*self.planes
        &&& self.output_len as int==self.w as int*self.h*self.ch*self.bytes
        &&& self.colors==(if self.slots==8 {2usize} else if self.slots==2 {16usize} else {256usize})
    }
    pub(super) open spec fn source_index(self,k:int) -> int {
        if self.palette || self.slots==8 {
            let p=k/(self.ch as int);
            (p/(self.w as int))*self.encoded_width+(p%(self.w as int))/(self.slots as int)
        } else {
            let stride=self.ch as int*self.bytes;
            let p=k/stride;let c=k%stride;
            let sc=if self.bytes==2 && self.le {c-c%2+1-c%2} else {c};
            if self.separate {(sc/(self.bytes as int))*self.plane_len+p*self.bytes+sc%(self.bytes as int)}
            else {p*stride+sc}
        }
    }
    pub(super) fn index(self,k:usize) -> (r:usize)
        requires self.valid(),k<self.output_len,
        ensures r==self.source_index(k as int),r<self.scratch_len,
    {
        if self.palette || self.slots==8 {
            let p=k/self.ch;let x=p%self.w;let y=p/self.w;
            proof {
                quotient_remainder(k as int,self.ch as int);quotient_remainder(p as int,self.w as int);
                assert(p<self.w as int*self.h && y<self.h) by(nonlinear_arith)
                    requires k<self.output_len,self.output_len as int==self.w as int*self.h*self.ch,
                        k as int==self.ch as int*(p as int)+k%self.ch,self.ch>0,p as int==self.w as int*y+x,0<=x<self.w,self.w>0;
                vstd::arithmetic::div_mod::lemma_div_is_ordered(x as int,self.w as int-1,self.slots as int);
                assert(0<=y as int*self.encoded_width<=y as int*self.encoded_width+x/self.slots<self.scratch_len) by(nonlinear_arith)
                    requires 0<=y<self.h,0<=x/self.slots<self.encoded_width,self.encoded_width>0,
                        self.scratch_len as int==self.encoded_width as int*self.h;
            }
            y*self.encoded_width+x/self.slots
        } else {
            assert(0<self.ch as int*self.bytes<=32) by(nonlinear_arith) requires 1<=self.ch<=4,1<=self.bytes<=8;
            let stride=self.ch*self.bytes;let p=k/stride;let c=k%stride;
            let sc=if self.bytes==2 && self.le {
                proof {quotient_remainder(c as int,2);
                    vstd::arithmetic::div_mod::lemma_div_is_ordered(c as int,stride as int-1,2);
                    assert((stride as int-1)/2==self.ch as int-1) by(nonlinear_arith) requires stride as int==self.ch as int*2,self.ch>0;
                }
                c-c%2+1-c%2
            } else {c};
            proof {
                quotient_remainder(k as int,stride as int);
                assert(sc<stride);
                assert(k as int==p as int*stride+c) by(nonlinear_arith) requires k as int==stride as int*p+c;
                assert(self.output_len as int==self.w as int*self.h*stride) by(nonlinear_arith)
                    requires self.output_len as int==self.w as int*self.h*self.ch*self.bytes,stride as int==self.ch as int*self.bytes;
                assert(p<self.w as int*self.h) by(nonlinear_arith)
                    requires stride>0,k as int==p as int*stride+c,k<self.output_len,
                        self.output_len as int==self.w as int*self.h*stride,0<=c;
            }
            if self.separate {
                let plane=sc/self.bytes;let byte=sc%self.bytes;
                proof {
                    quotient_remainder(sc as int,self.bytes as int);
                    assert(sc as int==plane as int*self.bytes+byte) by(nonlinear_arith) requires sc as int==self.bytes as int*plane+byte;
                    assert(plane<self.ch) by(nonlinear_arith) requires sc<stride,stride as int==self.ch as int*self.bytes,self.bytes>0,sc as int==plane as int*self.bytes+byte,byte>=0;
                    assert(0<=p as int*self.bytes<=p as int*self.bytes+byte<self.plane_len) by(nonlinear_arith)
                        requires p<self.w as int*self.h,self.plane_len as int==self.w as int*self.h*self.bytes,0<=byte<self.bytes,self.bytes>0;
                    assert(0<=plane as int*self.plane_len<=plane as int*self.plane_len+p*self.bytes+byte<self.scratch_len) by(nonlinear_arith)
                        requires plane<self.ch,0<=p as int*self.bytes+byte<self.plane_len,self.scratch_len as int==self.plane_len as int*self.ch;
                }
                plane*self.plane_len+p*self.bytes+byte
            } else {
                assert(0<=p as int*stride<=p as int*stride+sc<self.scratch_len) by(nonlinear_arith)
                    requires p<self.w as int*self.h,sc<stride,stride>0,self.scratch_len as int==self.w as int*self.h*stride;
                p*stride+sc
            }
        }
    }
    pub(super) open spec fn palette_index(self,k:int,byte:u8) -> int {
        let slot=((k/3)%(self.w as int))%(self.slots as int);
        let shift=if self.slots==8 {7-slot} else if self.slots==2 {(1-slot)*4} else {0int};
        (byte as int/power2(shift))%(self.colors as int)
    }
    pub(super) open spec fn converted(self,s:Seq<u8>,raw:Seq<u8>,k:int) -> u8 {
        let v=raw[self.source_index(k)];
        if self.palette {
            (word16(s,self.map.data as int+2*((k%3)*self.colors+self.palette_index(k,v)),self.le)/256) as u8
        } else if self.slots==8 {
            let bit=(v as int/power2(7-k%(self.w as int)%8))%2;
            if (bit==0)!=self.white {0u8} else {255u8}
        } else if self.alpha!=0 && k%(self.ch as int)!=self.ch-1 {
            straight_color(v,raw[self.source_index(k-k%(self.ch as int)+self.ch-1)],self.alpha==1,self.white)
        } else if self.white && self.alpha==0 {(255-v) as u8} else {v}
    }
    pub(super) proof fn identity(self,s:Seq<u8>,raw:Seq<u8>)
        requires self.valid(),self.bytes==1,self.slots==1,!self.palette,!self.separate,!self.white,self.alpha!=1,
            raw.len()>=self.scratch_len,
        ensures self.scratch_len==self.output_len,
            forall|k:int| 0<=k<self.output_len ==> self.converted(s,raw,k)==raw[k],
    {
        assert(self.scratch_len==self.output_len) by(nonlinear_arith)
            requires self.encoded_width==self.w,self.plane_channels==self.ch,self.planes==1,
                self.scratch_len==self.plane_len,self.plane_len as int==self.w as int*self.h*self.ch,
                self.output_len as int==self.w as int*self.h*self.ch;
        assert forall|k:int| 0<=k<self.output_len implies self.converted(s,raw,k)==raw[k] by {
            quotient_remainder(k,self.ch as int);
            assert(self.source_index(k)==k) by(nonlinear_arith)
                requires self.source_index(k)==k/(self.ch as int)*self.ch+k%(self.ch as int),
                    k==self.ch as int*(k/(self.ch as int))+k%(self.ch as int);
        }
    }
    pub(super) proof fn interleaved16(self,s:Seq<u8>,raw:Seq<u8>)
        requires self.valid(),self.bytes==2,self.slots==1,!self.palette,!self.separate,!self.white,
            raw.len()>=self.scratch_len,
        ensures self.scratch_len==self.output_len,self.output_len%2==0,
            forall|k:int| 0<=k<self.output_len ==> self.converted(s,raw,k)==raw[if self.le {k+1-2*(k%2)} else {k}],
    {
        assert(self.scratch_len==self.output_len && self.output_len%2==0) by(nonlinear_arith)
            requires self.encoded_width==self.w,self.plane_channels as int==self.ch as int*2,self.planes==1,
                self.scratch_len==self.plane_len,self.plane_len as int==self.w as int*self.h*self.plane_channels,
                self.output_len as int==self.w as int*self.h*self.ch*2;
        assert forall|k:int| 0<=k<self.output_len implies self.converted(s,raw,k)==raw[if self.le {k+1-2*(k%2)} else {k}] by {
            let stride=self.ch as int*2;
            quotient_remainder(k,stride);
            vstd::arithmetic::div_mod::lemma_mod_mod(k,2,self.ch as int);
            assert(k%stride%2==k%2);
            assert(self.source_index(k)==if self.le {k+1-2*(k%2)} else {k}) by(nonlinear_arith)
                requires k==stride*(k/stride)+k%stride,k%stride%2==k%2,
                    self.source_index(k)==k/stride*stride+(if self.le {k%stride+1-2*(k%stride%2)} else {k%stride});
        }
    }
    pub(super) fn convert(self,s:&[u8],raw:&[u8],k:usize) -> (r:u8)
        requires self.valid(),self.bytes!=8,k<self.output_len,raw.len()>=self.scratch_len,
            self.palette ==> range_ok(s.len() as int,self.map.data as int,6*(self.colors as int)),
        ensures r==self.converted(s@,raw@.subrange(0,self.scratch_len as int),k as int),
    {
        let v=raw[self.index(k)];
        if self.palette {
            let slot=((k/3)%self.w)%self.slots;
            let shift=if self.slots==8 {7-slot} else if self.slots==2 {(1-slot)*4} else {0};
            let index=(v as usize/bit_divisor(shift) as usize)%self.colors;
            let c=k%3;
            assert(0<=2*(c as int*self.colors+index)+2<=6*(self.colors as int)) by(nonlinear_arith)
                requires c<3,index<self.colors,self.colors>0;
            let off=self.map.data+2*(c*self.colors+index);
            let hi=if self.le {s[off+1]} else {s[off]};
            assert(word16(s@,off as int,self.le)/256==hi) by(nonlinear_arith)
                requires self.le ==> word16(s@,off as int,self.le)==s@[off as int] as int+256*(hi as int),
                    !self.le ==> word16(s@,off as int,self.le)==256*(hi as int)+s@[off as int+1] as int,
                    0<=s@[off as int]<=255,0<=s@[off as int+1]<=255;
            hi
        } else if self.slots==8 {
            let bit=(v/bit_divisor(7-(k%self.w)%8))%2;
            if (bit==0)!=self.white {0} else {255}
        } else if self.alpha!=0 && k%self.ch!=self.ch-1 {
            let c=k%self.ch;
            proof {
                quotient_remainder(k as int,self.ch as int);
                let pixels=self.w as int*self.h;
                let q=k as int/(self.ch as int);
                assert(q<pixels) by(nonlinear_arith)
                    requires k as int==self.ch as int*q+c,0<=c,self.ch>0,k<self.output_len,
                        self.output_len as int==pixels*self.ch;
                assert(k as int-c+self.ch-1<self.output_len) by(nonlinear_arith)
                    requires k as int==self.ch as int*q+c,q+1<=pixels,self.ch>0,
                        self.output_len as int==pixels*self.ch;

            }
            let a=raw[self.index(k-c+self.ch-1)];
            convert_color(v,a,self.alpha==1,self.white)
        } else if self.white && self.alpha==0 {255-v} else {v}
    }
}

}
