/* Identical read consumer compiled against our adapter and pinned libtiff. */
#define _POSIX_C_SOURCE 200809L
#include <tiffio.h>
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#ifndef ORACLE
#include <verified_tiff.h>
#endif
struct hooks{TIFFVGetMethod get;TIFFVSetMethod set;int got,set_count,printed;uint32_t value;};
static int get_hook(TIFF *t,uint32_t tag,va_list ap){struct hooks *h=TIFFGetClientInfo(t,"test");h->got++;if(tag==65002){*va_arg(ap,uint32_t*)=h->value;return 1;}return h->get(t,tag,ap);}
static int set_hook(TIFF *t,uint32_t tag,va_list ap){struct hooks *h=TIFFGetClientInfo(t,"test");h->set_count++;if(tag==65002){va_list cp;va_copy(cp,ap);h->value=va_arg(cp,uint32_t);va_end(cp);return h->set(t,tag,ap);}return h->set(t,tag,ap);}
static void print_hook(TIFF *t,FILE *f,long flags){(void)flags;struct hooks *h=TIFFGetClientInfo(t,"test");h->printed++;fputs("hook-value\n",f);}
static void save(const char *path,const void *p,size_t n){FILE *f=fopen(path,"wb");assert(f);assert(fwrite(p,1,n,f)==n);assert(!fclose(f));}
struct io {uint8_t *data,*buffer;size_t len,at,buffer_size,maxread;int reads,closed;};
static tmsize_t read_io(thandle_t c,void *p,tmsize_t n){struct io *m=c;assert(n>=0);m->reads++;if((size_t)n>m->maxread)m->maxread=(size_t)n;
#ifndef ORACLE
  if(m->buffer){assert(p==m->buffer);assert((size_t)n<=m->buffer_size);}
#endif
  size_t len=(size_t)n;if(len>m->len-m->at)len=m->len-m->at;memcpy(p,m->data+m->at,len);m->at+=len;return (tmsize_t)len;}
static tmsize_t write_io(thandle_t c,void *p,tmsize_t n){(void)c;(void)p;(void)n;return -1;}
static toff_t seek_io(thandle_t c,toff_t off,int whence){struct io *m=c;if(whence!=SEEK_SET||off>m->len)return (toff_t)-1;m->at=(size_t)off;return off;}
static toff_t size_io(thandle_t c){return ((struct io*)c)->len;}
static int close_io(thandle_t c){((struct io*)c)->closed++;return 0;}
/* Pinned libtiff 4.7.1 private field-array ABI, independent of our shim. */
struct field_layout {uint32_t tag;short read_count,write_count;TIFFDataType type;uint32_t anonymous;int convention;unsigned short bit;unsigned char change,pass;char *name;const TIFFFieldArray *children;};
struct array_layout {int type;uint32_t allocated,count;const struct field_layout *fields;};
int main(int argc,char **argv){
  assert(argc>=4);TIFFSetErrorHandler(NULL);TIFFSetWarningHandler(NULL);
  const char *mode=argv[1];
  TIFFOpenOptions *options=TIFFOpenOptionsAlloc();assert(options);
#ifndef ORACLE
  if(!strcmp(mode,"croptight")){
    TIFFOpenOptionsSetMaxSingleMemAlloc(options,524288);TIFFOpenOptionsSetMaxCumulatedMemAlloc(options,524288);
    assert(VTIFFOpenOptionsSetLimits(options,INT64_MAX,524288,1000000000));
  }
  if(!strcmp(mode,"stream")||!strcmp(mode,"lazy")||!strcmp(mode,"cropbounded")){
    TIFFOpenOptionsSetMaxSingleMemAlloc(options,131072);TIFFOpenOptionsSetMaxCumulatedMemAlloc(options,393216);
    assert(VTIFFOpenOptionsSetLimits(options,INT64_MAX,131072,1000000000));
  }
#endif
  TIFF *t=TIFFOpenExt(argv[2],"rOc",options);TIFFOpenOptionsFree(options);if(!t)return 3;
  if(!strcmp(mode,"crop")||!strcmp(mode,"cropbounded")||!strcmp(mode,"croptight")){
    assert(argc>=7);uint32_t w=(uint32_t)strtoul(argv[4],0,10),h=(uint32_t)strtoul(argv[5],0,10);int orientation=atoi(argv[6]);
    size_t n=(size_t)w*h*4;uint32_t *p=malloc(n+16);assert(p);memset(p,0xa5,n+16);
    int ok;
    if(argc>8){TIFFRGBAImage image;char msg[1024];assert(TIFFRGBAImageBegin(&image,t,1,msg));image.req_orientation=(uint16_t)orientation;image.col_offset=atoi(argv[7]);image.row_offset=atoi(argv[8]);ok=TIFFRGBAImageGet(&image,p,w,h);TIFFRGBAImageEnd(&image);}
    else ok=TIFFReadRGBAImageOriented(t,w,h,p,orientation,1);
    if(!ok){free(p);TIFFClose(t);return 4;}
    for(size_t i=n;i<n+16;i++){assert(((uint8_t*)p)[i]==0xa5);}save(argv[3],p,n);free(p);
  }else if(!strcmp(mode,"stream")){
    uint32_t height=0;assert(TIFFGetField(t,TIFFTAG_IMAGELENGTH,&height));tmsize_t row=TIFFScanlineSize(t);assert(row>0&&row<65536);
    uint8_t *p=malloc((size_t)row+16);assert(p);FILE *f=fopen(argv[3],"wb");assert(f);
    for(uint32_t y=0;y<height;y++){memset(p,0xa5,(size_t)row+16);if(TIFFReadScanline(t,p,y,0)!=1){free(p);fclose(f);TIFFClose(t);return 5;}for(size_t j=(size_t)row;j<(size_t)row+16;j++)assert(p[j]==0xa5);assert(fwrite(p,1,(size_t)row,f)==(size_t)row);}
    assert(TIFFReadScanline(t,p,0,0)==1);assert(!fclose(f));free(p);
    /* A prefix read must not allocate the decoded strip. */
    uint8_t prefix[33];memset(prefix,0xa5,sizeof(prefix));
#ifdef ORACLE
    uint16_t predictor=1;TIFFGetFieldDefaulted(t,TIFFTAG_PREDICTOR,&predictor);if(predictor==1){
#endif
    assert(TIFFReadEncodedStrip(t,0,prefix,17)==17);for(size_t i=17;i<33;i++)assert(prefix[i]==0xa5);
#ifdef ORACLE
    }
#endif
    memset(prefix,0xa5,sizeof(prefix));assert(TIFFReadRawStrip(t,0,prefix,7)==7);for(size_t i=7;i<33;i++)assert(prefix[i]==0xa5);
  }else if(!strcmp(mode,"lazy")){
    uint32_t n=TIFFNumberOfStrips(t);assert(n>=100000);int e=0;
    uint64_t a=TIFFGetStrileOffsetWithErr(t,n-1,&e);assert(a&&!e);assert(TIFFGetStrileByteCountWithErr(t,n-1,&e)==1&&!e);
    uint8_t b=0;assert(TIFFReadEncodedStrip(t,n-1,&b,1)==1);assert(b==73);save(argv[3],&b,1);
#ifndef ORACLE
    uint64_t *p=(void*)1;assert(!TIFFGetField(t,TIFFTAG_STRIPOFFSETS,&p));assert(p==(void*)1);assert(TIFFReadScanline(t,&b,n-1,0)==1);
#endif
  }else if(!strcmp(mode,"api")){
    assert(!TIFFGetClientInfo(t,"missing"));struct hooks h={0};TIFFSetClientInfo(t,&h,"test");assert(TIFFGetClientInfo(t,"test")==&h);
    TIFFTagMethods *m=TIFFAccessTagMethods(t);assert(m&&m->vgetfield&&m->vsetfield);h.get=m->vgetfield;h.set=m->vsetfield;m->vgetfield=get_hook;m->vsetfield=set_hook;m->printdir=print_hook;
    TIFFFieldInfo custom={65002,1,1,TIFF_LONG,FIELD_CUSTOM,1,0,"Hook"};assert(TIFFMergeFieldInfo(t,&custom,1)==0);
    assert(TIFFSetField(t,65002,(uint32_t)123456));uint32_t v=0;assert(TIFFGetField(t,65002,&v)&&v==123456);assert(h.got&&h.set_count);
    assert(TIFFSetField(t,TIFFTAG_IMAGEDESCRIPTION,"temporary"));char *s=NULL;assert(TIFFGetField(t,TIFFTAG_IMAGEDESCRIPTION,&s)&&!strcmp(s,"temporary"));assert(TIFFUnsetField(t,TIFFTAG_IMAGEDESCRIPTION));s=(char*)1;assert(!TIFFGetField(t,TIFFTAG_IMAGEDESCRIPTION,&s)&&s==(char*)1);
    FILE *f=tmpfile();assert(f);TIFFPrintDirectory(t,f,TIFFPRINT_STRIPS);assert(h.printed==1);assert(ftell(f)>0);fclose(f);
    uint8_t buffer[8192];assert(TIFFReadBufferSetup(t,buffer,sizeof(buffer)));uint8_t *pixels=malloc((size_t)TIFFScanlineSize(t));assert(pixels);assert(TIFFReadScanline(t,pixels,0,0)==1);assert(TIFFReadBufferSetup(t,NULL,8192));assert(TIFFReadScanline(t,pixels,0,0)==1);free(pixels);
    TIFFCodec *codecs=TIFFGetConfiguredCODECs();assert(codecs);int found=0;for(TIFFCodec *c=codecs;c->name;c++){assert(c->init);assert(TIFFIsCODECConfigured(c->scheme));assert(TIFFFindCODEC(c->scheme));if(c->scheme==1)found=1;}assert(found);_TIFFfree(codecs);assert(!TIFFFindCODEC(65535));
    TIFFFreeDirectory(t);assert(TIFFSetDirectory(t,0));save(argv[3],"ok",2);
  }else if(!strcmp(mode,"custom")){
    assert(argc==5);toff_t offset=(toff_t)strtoull(argv[4],NULL,10);
    const struct field_layout fields[]={
      {65000,1,1,TIFF_LONG,0,6,FIELD_CUSTOM,1,0,"Scalar",NULL},
      {65001,TIFF_VARIABLE2,TIFF_VARIABLE2,TIFF_SHORT,0,42,FIELD_CUSTOM,1,1,"Vector",NULL},
      {65002,TIFF_VARIABLE,TIFF_VARIABLE,TIFF_ASCII,0,1,FIELD_CUSTOM,1,0,"Text",NULL}};
    const struct array_layout array={3,0,3,fields};
    for(int i=0;i<3;i++){
      assert(TIFFReadCustomDirectory(t,offset,(const TIFFFieldArray*)&array));uint32_t v=0,n=0;uint16_t *values=NULL;char *text=NULL;
      assert(TIFFGetField(t,65000,&v)&&v==123456);assert(TIFFGetField(t,65001,&n,&values)&&n==3&&values[0]==2&&values[1]==500&&values[2]==65535);
      assert(TIFFGetField(t,65002,&text)&&!strcmp(text,"custom metadata"));assert(!strcmp(TIFFFieldName(TIFFFieldWithTag(t,65000)),"Scalar"));assert(TIFFSetDirectory(t,0));
    }
#ifndef ORACLE
    struct field_layout bad=fields[0];bad.convention=51;const struct array_layout invalid={3,0,1,&bad};toff_t previous=TIFFCurrentDirOffset(t);
    assert(!TIFFReadCustomDirectory(t,offset,(const TIFFFieldArray*)&invalid));assert(TIFFCurrentDirOffset(t)==previous);
    assert(TIFFReadCustomDirectory(t,offset,NULL));uint32_t n=0;uint32_t *values=NULL;assert(TIFFGetField(t,65000,&n,&values)&&n==1&&values[0]==123456);assert(TIFFSetDirectory(t,0));
#endif
    save(argv[3],"ok",2);
  }else if(!strcmp(mode,"client")){
    TIFFClose(t);FILE *f=fopen(argv[2],"rb");assert(f);assert(!fseek(f,0,SEEK_END));long len=ftell(f);assert(len>0);rewind(f);uint8_t *bytes=malloc((size_t)len);assert(bytes);assert(fread(bytes,1,(size_t)len,f)==(size_t)len);fclose(f);
    struct io a={0},b={0};a.data=b.data=bytes;a.len=b.len=(size_t)len;
    t=TIFFClientOpen("source","rc",&a,read_io,write_io,seek_io,close_io,size_io,NULL,NULL);assert(t);
    assert(TIFFSetClientdata(t,&b)==&a&&TIFFClientdata(t)==&b);
    char name[]="renamed";const char *previous=TIFFSetFileName(t,name);assert(previous&&!strcmp(previous,"source")&&TIFFFileName(t)==name);
    int fd=TIFFFileno(t);assert(TIFFSetFileno(t,91)==fd&&TIFFFileno(t)==91);assert(TIFFSetFileno(t,fd)==91);
    int mode0=TIFFGetMode(t);assert(TIFFSetMode(t,123)==mode0&&TIFFGetMode(t)==123);assert(TIFFSetMode(t,mode0)==123);
    uint8_t buffer[8192];assert(TIFFReadBufferSetup(t,buffer,sizeof(buffer)));b.buffer=buffer;b.buffer_size=sizeof(buffer);b.maxread=0;
    tmsize_t n=TIFFScanlineSize(t);uint8_t *row=malloc((size_t)n);assert(row&&TIFFReadScanline(t,row,0,0)==1&&b.reads>0);assert(b.maxread<=sizeof(buffer));free(row);
    TIFFClose(t);assert(a.closed==0&&b.closed==1);free(bytes);save(argv[3],"ok",2);return 0;
  }else if(!strcmp(mode,"rejectrgba")){
    uint32_t w=0,h=0;assert(TIFFGetField(t,TIFFTAG_IMAGEWIDTH,&w)&&TIFFGetField(t,TIFFTAG_IMAGELENGTH,&h));
    size_t size=(size_t)w*h*4;uint32_t *rgba=malloc(size+16);assert(rgba);memset(rgba,0xa5,size+16);
    assert(!TIFFReadRGBAImageOriented(t,w,h,rgba,ORIENTATION_TOPLEFT,1));for(size_t i=0;i<size+16;i++)assert(((uint8_t*)rgba)[i]==0xa5);free(rgba);save(argv[3],"ok",2);
  }else if(!strcmp(mode,"reject")){
    tmsize_t n=TIFFScanlineSize(t);uint8_t *row=malloc((size_t)n+16);assert(row);memset(row,0xa5,(size_t)n+16);assert(TIFFReadScanline(t,row,0,0)==-1);for(size_t i=0;i<(size_t)n+16;i++)assert(row[i]==0xa5);free(row);
    /* Complete reads may validate while decoding into private staging, but
     * checksum/truncation/end errors must never expose that partial result. */
    n=TIFFStripSize(t);assert(n>0);uint8_t *strip=malloc((size_t)n+16);assert(strip);memset(strip,0xa5,(size_t)n+16);
    assert(TIFFReadEncodedStrip(t,0,strip,n)==-1);for(size_t i=0;i<(size_t)n+16;i++)assert(strip[i]==0xa5);free(strip);
    uint32_t w=0,h=0;assert(TIFFGetField(t,TIFFTAG_IMAGEWIDTH,&w)&&TIFFGetField(t,TIFFTAG_IMAGELENGTH,&h));
    size_t size=(size_t)w*h*4;uint32_t *rgba=malloc(size+16);assert(rgba);memset(rgba,0xa5,size+16);
    assert(!TIFFReadRGBAImageOriented(t,w,h,rgba,ORIENTATION_TOPLEFT,1));for(size_t i=0;i<size+16;i++)assert(((uint8_t*)rgba)[i]==0xa5);free(rgba);save(argv[3],"ok",2);
  }else if(!strcmp(mode,"logcolor")){
    FILE *f=fopen(argv[3],"wb");assert(f);
    for(int i=0;i<4096;i++){double y[]={LogL16toY(i*17),LogL10toY(i%1024)};assert(fwrite(y,sizeof(double),2,f)==2);float xyz[3];uint8_t rgb[3];LogLuv24toXYZ((uint32_t)i*4093,xyz);assert(fwrite(xyz,4,3,f)==3);XYZtoRGB24(xyz,rgb);assert(fwrite(rgb,1,3,f)==3);LogLuv32toXYZ((uint32_t)i*104729,xyz);assert(fwrite(xyz,4,3,f)==3);double u=0,v=0;int ok=uv_decode(&u,&v,i*4);assert(fwrite(&ok,sizeof(ok),1,f)==1);assert(fwrite(&u,8,1,f)==1&&fwrite(&v,8,1,f)==1);}
    fclose(f);
  }else if(!strcmp(mode,"colors")){
    TIFFDisplay d={{{3.2410f,-1.5374f,-.4986f},{-.9692f,1.8760f,.0416f},{.0556f,-.2040f,1.0570f}},100,100,100,255,255,255,1,1,1,2.4f,2.4f,2.4f};
    float white[]={96.425f,100,82.468f};TIFFCIELabToRGB lab;assert(TIFFCIELabToRGBInit(&lab,&d,white)==0);
    size_t n=((sizeof(TIFFYCbCrToRGB)+sizeof(long)-1)/sizeof(long))*sizeof(long)+4*256+2*256*sizeof(int)+3*256*sizeof(int32_t);
    TIFFYCbCrToRGB *ycc=malloc(n);assert(ycc);float luma[]={.299f,.587f,.114f},ref[]={0,255,128,255,128,255};assert(TIFFYCbCrToRGBInit(ycc,luma,ref)==0);
    FILE *f=fopen(argv[3],"wb");assert(f);
    for(int i=0;i<4096;i++){float xyz[3];uint32_t rgb[3];TIFFCIELabToXYZ(&lab,(uint32_t)(i%256),i%256-128,(i*31)%256-128,xyz,xyz+1,xyz+2);TIFFXYZToRGB(&lab,xyz[0],xyz[1],xyz[2],rgb,rgb+1,rgb+2);assert(fwrite(rgb,4,3,f)==3);TIFFYCbCrtoRGB(ycc,(uint32_t)(i%256),(i*17)%256,(i*43)%256,rgb,rgb+1,rgb+2);assert(fwrite(rgb,4,3,f)==3);}
    fclose(f);free(ycc);
  }else{TIFFClose(t);return 6;}
  TIFFClose(t);return 0;
}
