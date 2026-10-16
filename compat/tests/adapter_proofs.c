#define _POSIX_C_SOURCE 200809L
#include <tiffio.h>
#include <assert.h>
#include <stdint.h>
#include <limits.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>

int main(int argc,char **argv){
    assert(argc==2);TIFF *t=TIFFOpen(argv[1],"rc");assert(t);
    TIFFFieldInfo field={65001,1,1,TIFF_SHORT,FIELD_CUSTOM,1,0,"BoundedShort"};
    assert(TIFFMergeFieldInfo(t,&field,1)==0);
    struct {uint16_t value;unsigned char guard[14];} out;
    memset(&out,0xa5,sizeof(out));unsigned char saved[sizeof(out)];memcpy(saved,&out,sizeof(out));
    // The file supplies either a DOUBLE, an oversized integer, or two SHORTs.
    // A registered scalar SHORT always retains its two-byte calling convention.
    assert(TIFFGetField(t,65001,&out.value)==0);assert(!memcmp(saved,&out,sizeof(out)));
    assert(TIFFSetField(t,65001,65535));assert(TIFFGetField(t,65001,&out.value));assert(out.value==65535);
    for(size_t i=0;i<sizeof(out.guard);i++)assert(out.guard[i]==0xa5);
    TIFFFieldInfo bad={65002,-3,-1,TIFF_SHORT,FIELD_CUSTOM,1,1,"MismatchedCountABI"};
    assert(TIFFMergeFieldInfo(t,&bad,1)==-1);
    unsigned char buffer[67];assert(TIFFReadBufferSetup(t,buffer,sizeof(buffer)));
    unsigned char *raw=malloc(3);assert(raw);raw[2]=0xa5;
    assert(TIFFReadEncodedStrip(t,0,raw,2)==2);assert(raw[0]==17&&raw[1]==41&&raw[2]==0xa5);free(raw);
    TIFFClose(t); // Must not free the borrowed stack buffer.

    TIFFDisplay display={{{3.2410f,-1.5374f,-.4986f},{-.9692f,1.8760f,.0416f},{.0556f,-.2040f,1.0570f}},100,100,100,255,255,255,1,1,1,2.4f,2.4f,2.4f};
    float white[]={96.425f,100,82.468f};TIFFCIELabToRGB lab;
    assert(TIFFCIELabToRGBInit(&lab,&display,white)==0);
    uint32_t r=71,g=72,b=73;lab.range=CIELABTORGB_TABLE_RANGE+1;
    TIFFXYZToRGB(&lab,1,2,3,&r,&g,&b);assert(r==71&&g==72&&b==73);
    assert(TIFFCIELabToRGBInit(&lab,&display,white)==0);
    lab.gstep=0.f;lab.bstep=NAN;TIFFXYZToRGB(&lab,10,20,30,&r,&g,&b);assert(r<=255&&g<=255&&b<=255);
    display.d_Y0G=0;display.d_YCG=nextafterf(0.f,1.f);
    assert(TIFFCIELabToRGBInit(&lab,&display,white)==-1);
    size_t n=((sizeof(TIFFYCbCrToRGB)+sizeof(long)-1)/sizeof(long))*sizeof(long)+4*256+2*256*sizeof(int)+3*256*sizeof(int32_t);
    TIFFYCbCrToRGB *yc=malloc(n);assert(yc);float luma[]={.299f,.587f,.114f},ref[]={0,255,128,255,128,255};assert(TIFFYCbCrToRGBInit(yc,luma,ref)==0);
    yc->Y_tab[0]=INT_MAX;yc->Cr_r_tab[0]=INT_MAX;yc->Cb_b_tab[0]=INT_MIN;yc->Cb_g_tab[0]=yc->Cr_g_tab[0]=INT_MAX;
    TIFFYCbCrtoRGB(yc,0,0,0,&r,&g,&b);assert(r==255&&g==255&&b==0);free(yc);
    return 0;
}
