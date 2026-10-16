/* Test-only independent encoder: libtiff packs supplied LogLuv32 words. */
#include <tiffio.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
int main(int argc,char **argv) {
    if(argc!=7)return 2;
    unsigned long aw=strtoul(argv[3],NULL,10),ah=strtoul(argv[4],NULL,10),ar=strtoul(argv[5],NULL,10);
    if(!aw||!ah||!ar||aw>16384||ah>16384||ar>16384||(uint64_t)aw*ah>16777216)return 2;
    uint32_t w=(uint32_t)aw,h=(uint32_t)ah,rows=(uint32_t)ar;
    size_t n=(size_t)w*h;uint32_t *data=malloc(n*sizeof(*data));if(!data)return 2;
    FILE *f=fopen(argv[1],"rb");if(!f)return 2;
    for(size_t i=0;i<n;i++){unsigned char b[4];if(fread(b,1,4,f)!=4)return 2;data[i]=(uint32_t)b[0]|((uint32_t)b[1]<<8)|((uint32_t)b[2]<<16)|((uint32_t)b[3]<<24);}
    if(fgetc(f)!=EOF||fclose(f)!=0)return 2;
    TIFF *t=TIFFOpen(argv[2],argv[6][0]=='l'?"wl":"wb");if(!t)return 2;
    if(!TIFFSetField(t,TIFFTAG_IMAGEWIDTH,w)||!TIFFSetField(t,TIFFTAG_IMAGELENGTH,h)||
       !TIFFSetField(t,TIFFTAG_COMPRESSION,COMPRESSION_SGILOG)||!TIFFSetField(t,TIFFTAG_PHOTOMETRIC,PHOTOMETRIC_LOGLUV)||
       !TIFFSetField(t,TIFFTAG_PLANARCONFIG,PLANARCONFIG_CONTIG)||!TIFFSetField(t,TIFFTAG_ROWSPERSTRIP,rows)||
       !TIFFSetField(t,TIFFTAG_SGILOGDATAFMT,SGILOGDATAFMT_RAW))return 2;
    /* In 4.7.1, TIFFWriteEncodedStrip byte-swaps RAW 32-bit input before
     * LogLuvEncode32 reads native integers; SGILog setup-encode does not disable
     * that generic hook. Pre-swap once so the independent codec sees the
     * specified words, for either file byte order. Decoder setup does disable
     * swapping. This compensates only the test API input, never its bitstream. */
    if(TIFFIsByteSwapped(t))TIFFSwabArrayOfLong(data,(tmsize_t)n);
    for(uint32_t y=0;y<h;y+=rows){uint32_t r=h-y<rows?h-y:rows;
        if(TIFFWriteEncodedStrip(t,TIFFComputeStrip(t,y,0),data+(size_t)y*w,(tmsize_t)((size_t)r*w*4))<0)return 2;
    }
    TIFFClose(t);free(data);return 0;
}
