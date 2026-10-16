/* Test-only libtiff JPEG oracle. Export expanded native components with nearest
 * chroma replication, before any RGB conversion, so filtering policies agree. */
#include <tiffio.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
int main(int argc,char **argv) {
    if(argc!=3)return 2;TIFF *t=TIFFOpen(argv[1],"r");if(!t)return 2;
    uint32_t w=0,h=0,bw=0,bh=0;uint16_t ch=0,bits=0,photo=0,comp=0,hs=1,vs=1;
    TIFFGetField(t,TIFFTAG_IMAGEWIDTH,&w);TIFFGetField(t,TIFFTAG_IMAGELENGTH,&h);
    TIFFGetFieldDefaulted(t,TIFFTAG_SAMPLESPERPIXEL,&ch);TIFFGetFieldDefaulted(t,TIFFTAG_BITSPERSAMPLE,&bits);
    TIFFGetField(t,TIFFTAG_PHOTOMETRIC,&photo);TIFFGetFieldDefaulted(t,TIFFTAG_COMPRESSION,&comp);
    if(!w||!h||bits!=8||(ch!=1&&ch!=3)||(uint64_t)w*h*ch>67108864)return 2;
    if(comp==COMPRESSION_JPEG)TIFFSetField(t,TIFFTAG_JPEGCOLORMODE,JPEGCOLORMODE_RAW);
    if(photo==PHOTOMETRIC_YCBCR)TIFFGetFieldDefaulted(t,TIFFTAG_YCBCRSUBSAMPLING,&hs,&vs);
    if(!hs||!vs||hs>4||vs>4)return 2;
    int tiled=TIFFIsTiled(t);
    if(tiled){TIFFGetField(t,TIFFTAG_TILEWIDTH,&bw);TIFFGetField(t,TIFFTAG_TILELENGTH,&bh);}
    else{bw=w;TIFFGetFieldDefaulted(t,TIFFTAG_ROWSPERSTRIP,&bh);if(bh>h)bh=h;}
    if(!bw||!bh)return 2;
    tmsize_t cap=tiled?TIFFTileSize(t):TIFFStripSize(t);if(cap<=0||cap>67108864)return 2;
    unsigned char *block=calloc((size_t)cap,1),*out=calloc((size_t)w*h*ch,1);if(!block||!out)return 2;
    for(uint64_t y=0;y<h;y+=bh)for(uint64_t x=0;x<w;x+=bw){
        tmsize_t got=tiled?TIFFReadEncodedTile(t,TIFFComputeTile(t,(uint32_t)x,(uint32_t)y,0,0),block,cap):TIFFReadEncodedStrip(t,TIFFComputeStrip(t,(uint32_t)y,0),block,cap);
        if(got<0)return 2;
        for(uint64_t r=0;r<bh&&y+r<h;r++)for(uint64_t c=0;c<bw&&x+c<w;c++){
            uint64_t dest=((y+r)*w+x+c)*ch;
            if(photo==PHOTOMETRIC_YCBCR){
                uint64_t at=((r/vs)*((bw+hs-1)/hs)+c/hs)*(hs*vs+2);
                if(at+hs*vs+2>(uint64_t)got)return 2;
                out[dest]=block[at+(r%vs)*hs+c%hs];out[dest+1]=block[at+hs*vs];out[dest+2]=block[at+hs*vs+1];
            }else{uint64_t at=(r*bw+c)*ch;if(at+ch>(uint64_t)got)return 2;for(unsigned a=0;a<ch;a++)out[dest+a]=block[at+a];}
        }
    }
    float *rbw=NULL;float fallback[6]={0,255,128,255,128,255};
    if(photo==PHOTOMETRIC_YCBCR)TIFFGetFieldDefaulted(t,TIFFTAG_REFERENCEBLACKWHITE,&rbw);if(!rbw)rbw=fallback;
    printf("%u %u %u %u %u %u %.9g %.9g %.9g %.9g %.9g %.9g\n",w,h,ch,photo,hs,vs,rbw[0],rbw[1],rbw[2],rbw[3],rbw[4],rbw[5]);
    FILE *f=fopen(argv[2],"wb");if(!f)return 2;size_t len=(size_t)w*h*ch;if(fwrite(out,1,len,f)!=len||fclose(f))return 2;
    free(out);free(block);TIFFClose(t);return 0;
}
