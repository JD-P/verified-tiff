#define _POSIX_C_SOURCE 200809L
#include <tiffio.h>
#include <verified_tiff.h>
#include <assert.h>
#include <errno.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
struct memory {unsigned char p[65536];size_t n,at;int reads,closed;};
static tmsize_t rd(thandle_t v,void *p,tmsize_t n){struct memory *m=v;m->reads++;if(n<0||m->at>m->n||(size_t)n>m->n-m->at)return -1;memcpy(p,m->p+m->at,(size_t)n);m->at+=(size_t)n;return n;}
static tmsize_t wr(thandle_t v,void *p,tmsize_t n){struct memory *m=v;if(n<0||m->at>sizeof(m->p)||(size_t)n>sizeof(m->p)-m->at)return -1;memcpy(m->p+m->at,p,(size_t)n);m->at+=(size_t)n;if(m->n<m->at)m->n=m->at;return n;}
static toff_t sk(thandle_t v,toff_t n,int w){struct memory *m=v;if(w!=SEEK_SET||n>sizeof(m->p))return (toff_t)-1;m->at=(size_t)n;return n;}
static toff_t sz(thandle_t v){return ((struct memory*)v)->n;}
static int cl(thandle_t v){((struct memory*)v)->closed++;return 0;}
static TIFFExtendProc previous;
static void extend(TIFF *t){
 const TIFFFieldInfo fields[]={
  {33550,-1,-1,TIFF_DOUBLE,FIELD_CUSTOM,1,1,"ModelPixelScaleTag"},
  {34735,-1,-1,TIFF_SHORT,FIELD_CUSTOM,1,1,"GeoKeyDirectoryTag"},
  {34736,-1,-1,TIFF_DOUBLE,FIELD_CUSTOM,1,1,"GeoDoubleParamsTag"},
  {34737,-1,-1,TIFF_ASCII,FIELD_CUSTOM,1,0,"GeoAsciiParamsTag"},
  {65001,-3,-3,TIFF_SLONG,FIELD_CUSTOM,1,1,"SignedTest"}};
 if(previous){previous(t);}assert(TIFFMergeFieldInfo(t,fields,5)==0);
}
int main(int argc,char **argv){
 assert(argc>=3);TIFFSetErrorHandler(NULL);TIFFSetWarningHandler(NULL);
 const char *mode=argv[1];
 if(!strcmp(mode,"environment-override")){
  assert(argc==4);TIFFOpenOptions *o=TIFFOpenOptionsAlloc();assert(o);
  if(atoi(argv[3]))assert(VTIFFOpenOptionsSetLimits(o,1U<<20,1U<<20,100000000));
  TIFFOpenOptionsSetMaxSingleMemAlloc(o,1U<<20);
  TIFFOpenOptionsSetMaxCumulatedMemAlloc(o,4U<<20);
  TIFF *t=TIFFOpenExt(argv[2],"rc",o);TIFFOpenOptionsFree(o);
  if(atoi(argv[3])){assert(t);unsigned char out[2048];assert(TIFFReadEncodedStrip(t,0,out,sizeof(out))>0);TIFFClose(t);}
  else assert(!t&&errno==EINVAL);
  return 0;
 }
 if(!strcmp(mode,"environment")){
  assert(argc==4);int expect=atoi(argv[3]);TIFF *t=TIFFOpen(argv[2],"rc");
  if(expect==0){assert(!t&&(errno==EINVAL||errno==EFBIG));return 0;}assert(t);
  unsigned char out[2048];memset(out,0xa5,sizeof(out));tmsize_t n=TIFFReadEncodedStrip(t,0,out,sizeof(out));
  if(expect==1){assert(n<0&&errno==EFBIG);for(size_t i=0;i<sizeof(out);i++)assert(out[i]==0xa5);}else assert(n>0);
  TIFFClose(t);return 0;
 }
 if(!strcmp(mode,"transfer")){
  TIFF *t=TIFFOpen(argv[2],"rc");assert(t);uint16_t *a=NULL,*b=NULL,*c=NULL;
  assert(TIFFGetField(t,TIFFTAG_TRANSFERFUNCTION,&a,&b,&c));assert(a[0]==0&&a[255]==65535&&b[0]==65535&&b[255]==0&&c[0]==17&&c[255]==17);
  TIFFClose(t);return 0;
 }
 if(!strcmp(mode,"metadata")){
  previous=TIFFSetTagExtender(extend);TIFF *t=TIFFOpen(argv[2],"rc");assert(t);
  uint16_t n;double *d=NULL;uint16_t *v=NULL;char *str=NULL;
  assert(TIFFGetField(t,33550,&n,&d)&&n==3&&d[0]==2.5&&d[2]==0);
  assert(TIFFGetField(t,34735,&n,&v)&&n==8&&v[4]==1024);
  assert(TIFFGetField(t,34737,&str)&&!strcmp(str,"WGS 84|"));
  uint32_t n32=0;int32_t *iv=NULL;assert(TIFFGetField(t,65001,&n32,&iv)&&n32==3&&iv[0]==-123&&iv[2]==2147483647);
  const TIFFField *f=TIFFFieldWithTag(t,65002);assert(f&&TIFFFieldIsAnonymous(f)&&TIFFFieldPassCount(f)&&TIFFFieldSetGetCountSize(f)==4);
  float *fv=NULL;assert(TIFFGetField(t,65002,&n32,&fv)&&n32==2&&fv[0]==1.25f&&fv[1]==-3.5f);
  for(unsigned i=0;i<8;i++){uint64_t sentinel=123;uint32_t tags[]={33723,34118,37724,32781,33423,36867,318,319};VTIFFClearError();assert(!TIFFGetField(t,tags[i],&sentinel)&&sentinel==123&&VTIFFLastErrorCode()==0);}
  uint16_t *ta=NULL,*tb=NULL,*tc=NULL;assert(TIFFGetFieldDefaulted(t,TIFFTAG_TRANSFERFUNCTION,&ta,&tb,&tc));assert(ta[0]==0&&ta[255]==65535&&tb[255]==65535&&tc[255]==65535);
  assert(!TIFFGetField(t,TIFFTAG_TRANSFERFUNCTION,&ta,&tb,&tc));
  float *ref=NULL;assert(TIFFGetFieldDefaulted(t,TIFFTAG_REFERENCEBLACKWHITE,&ref)&&ref[0]==0&&ref[1]==255);
  uint64_t exif=0,gps=0;assert(TIFFGetField(t,TIFFTAG_EXIFIFD,&exif)&&TIFFGetField(t,TIFFTAG_GPSIFD,&gps));
  assert(TIFFReadEXIFDirectory(t,exif));float exposure=0;assert(TIFFGetField(t,EXIFTAG_EXPOSURETIME,&exposure)&&fabsf(exposure-0.125f)<1e-6f);
  assert(TIFFGetField(t,EXIFTAG_DATETIMEORIGINAL,&str)&&!strcmp(str,"2026:10:15 12:34:56"));
  assert(TIFFReadGPSDirectory(t,gps));unsigned char *version=NULL;assert(TIFFGetField(t,GPSTAG_VERSIONID,&version)&&version[0]==2&&version[1]==3);
  assert(TIFFGetField(t,GPSTAG_LATITUDE,&d)&&d[0]==51&&d[1]==30&&d[2]==0);
  uint64_t selected=TIFFCurrentDirOffset(t);assert(!TIFFReadEXIFDirectory(t,UINT64_MAX)&&TIFFCurrentDirOffset(t)==selected);
  assert(TIFFSetDirectory(t,0));assert(TIFFGetField(t,33550,&n,&d)&&d[0]==2.5);
  double change[]={5,6,7};assert(TIFFSetField(t,33550,3,change));assert(TIFFGetField(t,33550,&n,&d)&&n==3&&d[2]==7);
  TIFFClose(t);TIFFSetTagExtender(previous);return 0;
 }
 if(!strcmp(mode,"writer")){
  for(int big=0;big<2;big++)for(int le=0;le<2;le++)for(int tiles=0;tiles<2;tiles++){
   struct memory m={0};const char *modes[]={"wb+","wl+","wb8+","wl8+"};
   TIFF *t=TIFFClientOpen("temporary",modes[big*2+le],&m,rd,wr,sk,cl,sz,NULL,NULL);assert(t);
   assert(TIFFSetField(t,256,16U)&&TIFFSetField(t,257,16U));
   assert(TIFFSetField(t,258,8)&&TIFFSetField(t,277,3)&&TIFFSetField(t,262,2));
   assert(TIFFSetField(t,259,1)&&TIFFSetField(t,284,1)&&TIFFSetField(t,339,1));
   if(tiles){assert(TIFFSetField(t,322,16U)&&TIFFSetField(t,323,16U));}else assert(TIFFSetField(t,278,16U));
   assert(TIFFWriteCheck(t,tiles,"test")&&TIFFWriteDirectory(t));assert(TIFFFlush(t));TIFFClose(t);assert(m.closed==1&&m.n>0);
   if(argc>3){char path[1024];snprintf(path,sizeof(path),"%s-%d-%d-%d.tif",argv[3],big,le,tiles);FILE *f=fopen(path,"wb");assert(f&&fwrite(m.p,1,m.n,f)==m.n);fclose(f);}
   m.at=0;t=TIFFClientOpen("temporary","rc",&m,rd,NULL,sk,cl,sz,NULL,NULL);assert(t&&TIFFIsBigTIFF(t)==big&&TIFFIsTiled(t)==tiles);
   unsigned char input[768],out[784];for(size_t i=0;i<sizeof(input);i++)input[i]=(unsigned char)i;
   int reads=m.reads;memset(out,0xa5,sizeof(out));assert(TIFFReadFromUserBuffer(t,0,input,sizeof(input),out,sizeof(input)));assert(m.reads==reads&&!memcmp(input,out,sizeof(input)));
   for(size_t i=768;i<784;i++)assert(out[i]==0xa5);
   memset(out,0xa5,sizeof(out));assert(!TIFFReadFromUserBuffer(t,0,input,767,out,768));for(size_t i=0;i<sizeof(out);i++)assert(out[i]==0xa5);
   assert(!TIFFReadFromUserBuffer(t,1,input,768,out,768));assert(!TIFFReadFromUserBuffer(t,0,input,768,out,769));
   assert(TIFFReadFromUserBuffer(t,0,input,768,out,7)&&!memcmp(input,out,7)&&out[7]==0xa5);
   TIFFClose(t);assert(m.closed==2);
  }return 0;
 }
 if(!strcmp(mode,"clone")){
  TIFF *source=TIFFOpen(argv[2],"rc");assert(source);uint16_t photo=0;assert(TIFFGetField(source,262,&photo));
  if(photo==32845)assert(TIFFSetField(source,TIFFTAG_SGILOGDATAFMT,SGILOGDATAFMT_16BIT));
  if(photo==6)assert(TIFFSetField(source,TIFFTAG_JPEGCOLORMODE,JPEGCOLORMODE_RGB));
  uint32_t w,h;assert(TIFFGetField(source,256,&w)&&TIFFGetField(source,257,&h));int tiled=TIFFIsTiled(source);
  uint32_t bh=h;if(tiled){assert(TIFFGetField(source,322,&w)&&TIFFGetField(source,323,&h));bh=h;}
  else{TIFFGetFieldDefaulted(source,278,&bh);if(bh<h)h=bh;bh=h;}
  tmsize_t size=tiled?TIFFTileSize(source):TIFFStripSize(source);uint64_t insize=TIFFGetStrileByteCount(source,0);assert(size>0&&insize>0&&insize<65536);
  void *expected=malloc((size_t)size),*out=malloc((size_t)size),*input=malloc((size_t)insize);assert(expected&&out&&input);
  tmsize_t n=tiled?TIFFReadEncodedTile(source,0,expected,size):TIFFReadEncodedStrip(source,0,expected,size);assert(n>0);
  assert((tiled?TIFFReadRawTile(source,0,input,(tmsize_t)insize):TIFFReadRawStrip(source,0,input,(tmsize_t)insize))==(tmsize_t)insize);
  struct memory m={0};TIFF *t=TIFFClientOpen("temporary",TIFFIsBigEndian(source)?"wb+":"wl+",&m,rd,wr,sk,cl,sz,NULL,NULL);assert(t);
  assert(TIFFSetField(t,256,w)&&TIFFSetField(t,257,h));
  const uint32_t scalar_tags[]={258,259,262,266,277,284,317,339};
  for(size_t i=0;i<sizeof(scalar_tags)/sizeof(*scalar_tags);i++){uint16_t v=0;if(TIFFGetFieldDefaulted(source,scalar_tags[i],&v))assert(TIFFSetField(t,scalar_tags[i],v));}
  if(tiled){assert(TIFFSetField(t,322,w)&&TIFFSetField(t,323,h));}else assert(TIFFSetField(t,278,bh));
  uint16_t count=0,*extras=NULL;if(TIFFGetField(source,338,&count,&extras))assert(TIFFSetField(t,338,count,extras));
  uint32_t bytes=0;void *tables=NULL;if(TIFFGetField(source,347,&bytes,&tables))assert(TIFFSetField(t,347,bytes,tables));
  uint16_t hs=0,vs=0;if(TIFFGetField(source,530,&hs,&vs))assert(TIFFSetField(t,530,hs,vs));
  for(uint32_t tag=292;tag<=293;tag++){uint32_t v=0;if(TIFFGetField(source,tag,&v))assert(TIFFSetField(t,tag,v));}
  assert(TIFFWriteCheck(t,tiled,"GDAL-style clone")&&TIFFWriteDirectory(t));TIFFClose(t);
  t=TIFFClientOpen("temporary","rc",&m,rd,NULL,sk,cl,sz,NULL,NULL);assert(t);
  if(photo==32845)assert(TIFFSetField(t,TIFFTAG_SGILOGDATAFMT,SGILOGDATAFMT_16BIT));
  if(photo==6)assert(TIFFSetField(t,TIFFTAG_JPEGCOLORMODE,JPEGCOLORMODE_RGB));
  int reads=m.reads;assert(TIFFReadFromUserBuffer(t,0,input,(tmsize_t)insize,out,n));assert(m.reads==reads&&!memcmp(expected,out,(size_t)n));
  TIFFClose(t);TIFFClose(source);free(input);free(out);free(expected);return 0;
 }
 if(!strcmp(mode,"readset")){
  TIFF *t=TIFFOpen(argv[2],"rc");assert(t);tmsize_t n=TIFFStripSize(t);assert(n>0);void *a=malloc((size_t)n),*b=malloc((size_t)n);assert(a&&b);
  assert(TIFFReadEncodedStrip(t,0,a,n)==n);assert(TIFFSetField(t,TIFFTAG_SAMPLEFORMAT,SAMPLEFORMAT_IEEEFP));
  uint16_t v=0;assert(TIFFGetField(t,TIFFTAG_SAMPLEFORMAT,&v)&&v==SAMPLEFORMAT_IEEEFP);assert(TIFFReadEncodedStrip(t,0,b,n)==n&&!memcmp(a,b,(size_t)n));
  free(a);free(b);TIFFClose(t);return 0;
 }
 if(!strcmp(mode,"logluv")){
  TIFF *t=TIFFOpen(argv[2],"rc");assert(t&&TIFFSetField(t,TIFFTAG_SGILOGDATAFMT,SGILOGDATAFMT_FLOAT));
  tmsize_t n=TIFFStripSize(t);assert(n>0);void *out=malloc((size_t)n);assert(out);
  for(uint32_t k=0;k<TIFFNumberOfStrips(t);k++){tmsize_t got=TIFFReadEncodedStrip(t,k,out,n);assert(got>0);assert(fwrite(out,1,(size_t)got,stdout)==(size_t)got);}
  free(out);TIFFClose(t);return 0;
 }
 assert(0);
}
