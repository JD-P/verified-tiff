/* Test-only independent decoder using pinned upstream libtiff. */
#include <tiffio.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
int main(int argc, char **argv) {
    if (argc != 4) return 2;
    TIFF *t = TIFFOpen(argv[1], "r");
    if (!t) return 2;
    if (!TIFFSetDirectory(t, (tdir_t)strtoul(argv[3], NULL, 10))) return 2;
    uint32_t w=0,h=0;
    if (!TIFFGetField(t,TIFFTAG_IMAGEWIDTH,&w) || !TIFFGetField(t,TIFFTAG_IMAGELENGTH,&h)
        /* The original lzw-single-strip fixture contains 24,335,990 pixels.
         * Keep the test-only RGBA allocation bounded to 128 MiB. */
        || !w || !h || (uint64_t)w*h > 33554432) return 2;
    void *raw=NULL;
    uint16_t compression=0,fill=0;
    TIFFGetFieldDefaulted(t,TIFFTAG_COMPRESSION,&compression);
    TIFFGetFieldDefaulted(t,TIFFTAG_FILLORDER,&fill);
    if (TIFFIsTiled(t) && compression==COMPRESSION_NONE && fill==FILLORDER_LSB2MSB) {
        /* 4.7.1's RGBA tile path compares raw buffer capacity to tile size.
         * Its bit-reversal path rounds library allocations up to 1024 bytes,
         * rejecting smaller valid tiles. Supply exact-sized caller storage
         * through the public API; libtiff still interprets and decodes bytes. */
        tmsize_t size=TIFFTileSize(t);
        if (size<=0 || size>67108864) return 2;
        raw=calloc(1,(size_t)size);
        if (!raw || !TIFFReadBufferSetup(t,raw,size)) return 2;
    }
    uint32_t *pixels=calloc((size_t)w*h,sizeof(uint32_t));
    if (!pixels) return 2;
    if (!TIFFReadRGBAImageOriented(t,w,h,pixels,ORIENTATION_TOPLEFT,1)) return 2;
    FILE *f=fopen(argv[2],"wb");
    if (!f) return 2;
    fprintf(f,"P6\n%u %u\n255\n",w,h);
    for (size_t i=0;i<(size_t)w*h;i++) {
        unsigned char rgb[3]={TIFFGetR(pixels[i]),TIFFGetG(pixels[i]),TIFFGetB(pixels[i])};
        if(fwrite(rgb,1,3,f)!=3) return 2;
    }
    fclose(f);free(pixels);TIFFClose(t);free(raw);return 0;
}
