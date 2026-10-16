/* Test-only independent codec/storage oracle. Returns native interleaved
 * samples, preserving associated alpha and WhiteIsZero polarity. */
#include <tiffio.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main(int argc,char **argv) {
    if(argc!=4) return 2;
    TIFF *t=TIFFOpen(argv[1],"r");
    if(!t || !TIFFSetDirectory(t,(tdir_t)strtoul(argv[3],NULL,10))) return 2;
    uint32_t w=0,h=0,bw=0,bh=0;uint16_t ch=0,bits=0,planar=0;
    if(!TIFFGetField(t,TIFFTAG_IMAGEWIDTH,&w) || !TIFFGetField(t,TIFFTAG_IMAGELENGTH,&h)) return 2;
    TIFFGetFieldDefaulted(t,TIFFTAG_SAMPLESPERPIXEL,&ch);
    TIFFGetFieldDefaulted(t,TIFFTAG_BITSPERSAMPLE,&bits);
    TIFFGetFieldDefaulted(t,TIFFTAG_PLANARCONFIG,&planar);
    if(!w || !h || (ch!=2 && ch!=4) || bits!=8 || planar!=PLANARCONFIG_CONTIG || (uint64_t)w*h>67108864/ch) return 2;
    int tiled=TIFFIsTiled(t);
    if(tiled) {
        if(!TIFFGetField(t,TIFFTAG_TILEWIDTH,&bw) || !TIFFGetField(t,TIFFTAG_TILELENGTH,&bh)) return 2;
    } else {bw=w;TIFFGetFieldDefaulted(t,TIFFTAG_ROWSPERSTRIP,&bh);}
    if(!bw || !bh) return 2;
    tmsize_t cap=tiled?TIFFTileSize(t):TIFFStripSize(t);
    if(cap<=0 || cap>67108864) return 2;
    unsigned char *block=malloc((size_t)cap),*pixels=calloc((size_t)w*h,ch);
    if(!block || !pixels) return 2;
    for(uint64_t y=0;y<h;y+=bh) for(uint64_t x=0;x<w;x+=bw) {
        uint64_t rows=tiled?bh:((h-y<bh)?h-y:bh);
        if((uint64_t)bw*rows>(uint64_t)cap/ch) return 2;
        uint64_t needed=(uint64_t)bw*rows*ch;
        if(needed>(uint64_t)cap) return 2;
        tmsize_t got=tiled ? TIFFReadEncodedTile(t,TIFFComputeTile(t,(uint32_t)x,(uint32_t)y,0,0),block,cap)
            : TIFFReadEncodedStrip(t,TIFFComputeStrip(t,(uint32_t)y,0),block,cap);
        if(got<0 || (uint64_t)got<needed) return 2;
        uint64_t cols=(w-x<bw)?w-x:bw;
        for(uint64_t r=0;r<rows && y+r<h;r++)
            memcpy(pixels+((y+r)*w+x)*ch,block+r*bw*ch,(size_t)cols*ch);
    }
    FILE *f=fopen(argv[2],"wb");if(!f) return 2;
    size_t len=(size_t)w*h*ch;
    if(fwrite(pixels,1,len,f)!=len || fclose(f)!=0) return 2;
    free(pixels);free(block);TIFFClose(t);return 0;
}
