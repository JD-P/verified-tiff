/* Test-only libtiff oracle. Emits interleaved native samples, with 16-bit
 * unsigned samples in big-endian order, LogLuv16 and IEEE 64-bit samples in little-endian order; palettes emit RGB8 from TIFF's 16-bit map. */
#include <tiffio.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main(int argc,char **argv) {
    if(argc!=4)return 2;
    TIFF *t=TIFFOpen(argv[1],"r");
    if(!t || !TIFFSetDirectory(t,(tdir_t)strtoul(argv[3],NULL,10)))return 2;
    uint32_t w=0,h=0,bw=0,bh=0;uint16_t ch=0,bits=0,planar=0,photo=0,format=0;
    if(!TIFFGetField(t,TIFFTAG_IMAGEWIDTH,&w)||!TIFFGetField(t,TIFFTAG_IMAGELENGTH,&h))return 2;
    TIFFGetFieldDefaulted(t,TIFFTAG_SAMPLESPERPIXEL,&ch);
    TIFFGetFieldDefaulted(t,TIFFTAG_BITSPERSAMPLE,&bits);
    TIFFGetFieldDefaulted(t,TIFFTAG_PLANARCONFIG,&planar);
    TIFFGetField(t,TIFFTAG_PHOTOMETRIC,&photo);
    TIFFGetFieldDefaulted(t,TIFFTAG_SAMPLEFORMAT,&format);
    int logluv=photo==PHOTOMETRIC_LOGLUV;
    if(logluv&&!TIFFSetField(t,TIFFTAG_SGILOGDATAFMT,SGILOGDATAFMT_16BIT))return 2;
    int palette=photo==PHOTOMETRIC_PALETTE;
    if(!w||!h||ch<1||ch>4||(planar!=1&&planar!=2))return 2;
    if(palette ? (ch!=1||(bits!=1&&bits!=4&&bits!=8)) : (bits!=8&&bits!=16&&!(bits==64&&format==SAMPLEFORMAT_IEEEFP)))return 2;
    uint16_t *map[3]={0};
    if(palette&&!TIFFGetField(t,TIFFTAG_COLORMAP,&map[0],&map[1],&map[2]))return 2;
    unsigned bytes=bits==64?8:bits==16?2:1,outch=palette?3:ch;
    uint64_t len=(uint64_t)w*h*outch*bytes;
    if(len>67108864)return 2;
    int tiled=TIFFIsTiled(t);
    if(tiled){TIFFGetField(t,TIFFTAG_TILEWIDTH,&bw);TIFFGetField(t,TIFFTAG_TILELENGTH,&bh);}
    else {bw=w;TIFFGetFieldDefaulted(t,TIFFTAG_ROWSPERSTRIP,&bh);}
    if(!bw||!bh)return 2;
    tmsize_t cap=tiled?TIFFTileSize(t):TIFFStripSize(t);
    if(cap<=0||cap>67108864)return 2;
    unsigned char *block=malloc((size_t)cap),*out=calloc((size_t)len,1);
    if(!block||!out)return 2;
    unsigned planes=planar==2?ch:1,sc=planar==2?1:ch;
    for(unsigned plane=0;plane<planes;plane++)for(uint64_t y=0;y<h;y+=bh)for(uint64_t x=0;x<w;x+=bw){
        uint64_t rows=tiled?bh:((h-y<bh)?h-y:bh);
        uint64_t stride=((uint64_t)bw*sc*bits+7)/8;
        if(stride*rows>(uint64_t)cap)return 2;
        tmsize_t got=tiled?TIFFReadEncodedTile(t,TIFFComputeTile(t,(uint32_t)x,(uint32_t)y,0,(uint16_t)plane),block,cap)
            :TIFFReadEncodedStrip(t,TIFFComputeStrip(t,(uint32_t)y,(uint16_t)plane),block,cap);
        if(got<0||(uint64_t)got<stride*rows)return 2;
        for(uint64_t r=0;r<rows&&y+r<h;r++)for(uint64_t c=0;c<bw&&x+c<w;c++){
            uint64_t p=((y+r)*w+x+c)*outch*bytes;
            if(palette){unsigned index=(block[r*stride+c*bits/8]>>(8-bits-(c*bits)%8))&((1u<<bits)-1);
                for(unsigned a=0;a<3;a++)out[p+a]=(unsigned char)(map[a][index]>>8);
            }else for(unsigned a=0;a<sc;a++){
                unsigned dest=planar==2?plane:a;uint64_t src=r*stride+(c*sc+a)*bytes;
                if(bits==64){uint64_t v;memcpy(&v,block+src,8);for(unsigned b=0;b<8;b++)out[p+dest*8+b]=(unsigned char)(v>>(8*b));}
                else if(bits==16){uint16_t v;memcpy(&v,block+src,2);out[p+dest*2]=(unsigned char)(logluv?v:v>>8);out[p+dest*2+1]=(unsigned char)(logluv?v>>8:v);}
                else out[p+dest]=block[src];
            }
        }
    }
    FILE *f=fopen(argv[2],"wb");if(!f)return 2;
    if(fwrite(out,1,(size_t)len,f)!=(size_t)len||fclose(f)!=0)return 2;
    free(out);free(block);TIFFClose(t);return 0;
}
