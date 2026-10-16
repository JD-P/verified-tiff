#define _POSIX_C_SOURCE 200809L
#include <tiffio.h>
#include <verified_tiff.h>
#include <assert.h>
#include <stdint.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
static int warnings;
static int warning(TIFF *t,void *u,const char *m,const char *f,va_list ap){(void)t;(void)u;(void)m;(void)f;(void)ap;warnings++;return 1;}
struct memory{unsigned char *p;size_t n,at,pixels,reads,bytes;int closed;};
static tmsize_t rd(thandle_t v,void *p,tmsize_t n){
  struct memory *m=v;assert(n>=0 && (size_t)n<=m->n-m->at);
  if(m->at>=m->pixels){m->reads++;m->bytes+=(size_t)n;}
  memcpy(p,m->p+m->at,(size_t)n);m->at+=(size_t)n;return n;
}
static toff_t sk(thandle_t v,toff_t n,int whence){struct memory *m=v;if(whence!=SEEK_SET||n>m->n)return (toff_t)-1;m->at=(size_t)n;return n;}
static toff_t sz(thandle_t v){return ((struct memory *)v)->n;}
static int cl(thandle_t v){((struct memory *)v)->closed++;return 0;}
static TIFFOpenOptions *limits(uint64_t in,uint64_t out,uint64_t work,size_t single,size_t cumulative){
  TIFFOpenOptions *o=TIFFOpenOptionsAlloc();assert(o);assert(VTIFFOpenOptionsSetLimits(o,in,out,work));
  TIFFOpenOptionsSetMaxSingleMemAlloc(o,(tmsize_t)single);TIFFOpenOptionsSetMaxCumulatedMemAlloc(o,(tmsize_t)cumulative);return o;
}
static void unchanged(const unsigned char *b,size_t n){for(size_t i=0;i<n;i++)assert(b[i]==0xa5);}
int main(int argc,char **argv){
  assert(argc>=3);const char *mode=argv[1],*path=argv[2];TIFFSetErrorHandler(NULL);TIFFSetWarningHandler(NULL);
  if(!strcmp(mode,"bad")){assert(!TIFFOpen(path,"rc"));return 0;}
  if(!strcmp(mode,"seek")){
    FILE *f=fopen(path,"rb");assert(f);assert(!fseek(f,0,SEEK_END));long n=ftell(f);assert(n>0);rewind(f);
    struct memory m={0};m.n=(size_t)n;m.p=malloc(m.n);assert(m.p);assert(fread(m.p,1,m.n,f)==m.n);fclose(f);m.pixels=strtoull(argv[3],NULL,10);
    TIFFOpenOptions *o=limits(1<<22,8192,100000,1<<17,1<<19);
    TIFF *t=TIFFClientOpenExt("tracked","rOc",&m,rd,NULL,sk,cl,sz,NULL,NULL,o);assert(t);assert(m.reads==0);
    assert(TIFFNumberOfStrips(t)==8192);unsigned char out[4112];memset(out,0xa5,sizeof(out));
    assert(TIFFReadEncodedStrip(t,0,out,-1)==4096);assert(m.reads==1 && m.bytes<100);
    for(size_t i=0;i<4096;i++)assert(out[i]==57);
    unchanged(out+4096,16);
    size_t reads=m.reads;assert(TIFFReadScanline(t,out,0,0)==1);assert(m.reads==reads);
    memset(out,0xa5,sizeof(out));assert(TIFFReadEncodedStrip(t,8191,out,-1)==-1);unchanged(out,sizeof(out));
    assert(TIFFReadScanline(t,out,0,0)==1);TIFFClose(t);assert(m.closed==1);TIFFOpenOptionsFree(o);free(m.p);return 0;
  }
  if(!strcmp(mode,"limits")){
    unsigned char out[12304];
    for(int kind=0;kind<2;kind++){
      TIFFOpenOptions *o=limits(1<<20,1<<20,1000000,kind?20000:8000,kind?12000:0);
      TIFF *t=TIFFOpenExt(path,"rc",o);assert(t);memset(out,0xa5,sizeof(out));
      assert(TIFFReadEncodedStrip(t,0,out,-1)==-1);assert(errno==EFBIG);unchanged(out,sizeof(out));
      TIFFClose(t);TIFFOpenOptionsFree(o);
    }
    TIFFOpenOptions *o=limits(1<<20,1<<20,1000000,0,0);TIFF *t=TIFFOpenExt(path,"rc",o);assert(t);
    assert(TIFFReadEncodedStrip(t,0,out,-1)==12288);TIFFClose(t);TIFFOpenOptionsFree(o);return 0;
  }
  if(!strcmp(mode,"work")){
    TIFFOpenOptions *o=limits(1<<20,1<<20,strtoull(argv[3],NULL,10),0,0);
    TIFF *t=TIFFOpenExt(path,"rc",o);assert(t);unsigned char out[156];memset(out,0xa5,sizeof(out));
    assert(!TIFFReadRGBAImageOriented(t,7,5,(uint32_t *)out,1,1));assert(errno==EFBIG);unchanged(out,sizeof(out));
    assert(TIFFReadEncodedStrip(t,0,out,-1)==63);assert(TIFFReadEncodedStrip(t,1,out,-1)==42);
    TIFFClose(t);TIFFOpenOptionsFree(o);return 0;
  }
  if(!strcmp(mode,"sparse")){
    TIFFOpenOptions *o=limits(UINT64_C(1)<<34,1<<20,1000000,1<<20,1<<22);
    TIFF *t=TIFFOpenExt(path,"rc",o);assert(t && TIFFIsBigTIFF(t));assert(TIFFCurrentDirOffset(t)>UINT32_MAX);
    assert(TIFFGetStrileOffset(t,0)>UINT32_MAX);unsigned char out[1024];
    for(uint32_t i=0;i<TIFFNumberOfStrips(t);i++){tmsize_t n=TIFFReadEncodedStrip(t,i,out,-1);assert(n>0 && n<=1024);assert(fwrite(out,1,(size_t)n,stdout)==(size_t)n);}
    TIFFClose(t);TIFFOpenOptionsFree(o);return 0;
  }
  if(!strcmp(mode,"metadata")){
    TIFFOpenOptions *o=TIFFOpenOptionsAlloc();assert(o);TIFFOpenOptionsSetWarningHandlerExtR(o,warning,NULL);TIFFOpenOptionsSetWarnAboutUnknownTags(o,1);
    TIFF *t=TIFFOpenExt(path,"rC",o);assert(t);assert(warnings==1);assert(TIFFIsBigTIFF(t)==atoi(argv[3]));
    float x=0,y=0;assert(TIFFGetField(t,TIFFTAG_XRESOLUTION,&x)&&x==300);assert(TIFFGetField(t,TIFFTAG_YRESOLUTION,&y)&&y==288);
    char *s=NULL;assert(TIFFGetField(t,TIFFTAG_IMAGEDESCRIPTION,&s)&&!strcmp(s,"Image description"));
    assert(TIFFGetField(t,TIFFTAG_SOFTWARE,&s)&&!strcmp(s,"Verified TIFF tests"));assert(TIFFGetField(t,TIFFTAG_ARTIST,&s)&&!strcmp(s,"Author"));
    uint32_t n=0;void *p=NULL;const unsigned char icc[]={0,'I','C','C',255,0,'p','r','o','f','i','l','e'};
    assert(TIFFGetField(t,TIFFTAG_ICCPROFILE,&n,&p)&&n==sizeof(icc)&&!memcmp(p,icc,n));
    assert(TIFFGetField(t,TIFFTAG_XMLPACKET,&n,&p)&&n==6&&!memcmp(p,"<xmp/>",6));
    uint16_t a=0,b=0;assert(TIFFGetField(t,TIFFTAG_PAGENUMBER,&a,&b)&&a==7&&b==12);
    assert(TIFFGetFieldDefaulted(t,TIFFTAG_YCBCRSUBSAMPLING,&a,&b)&&a==2&&b==2);
    const TIFFField *field=TIFFFieldWithTag(t,TIFFTAG_ICCPROFILE);assert(field);
    assert(TIFFFieldTag(field)==TIFFTAG_ICCPROFILE&&TIFFFieldDataType(field)==TIFF_UNDEFINED&&TIFFFieldPassCount(field)==1);
    assert(TIFFFieldReadCount(field)==TIFF_VARIABLE2&&TIFFFieldSetGetCountSize(field)==4&&TIFFFieldSetGetSize(field)==1&&!TIFFFieldIsAnonymous(field));
    assert(TIFFFieldWithName(t,"ICC Profile")==field);assert(TIFFGetTagListCount(t)==6);
    int found=0;for(int i=0;i<TIFFGetTagListCount(t);i++)found+=TIFFGetTagListEntry(t,i)==TIFFTAG_ICCPROFILE;assert(found==1);
    TIFFClose(t);TIFFOpenOptionsFree(o);
    for(const char **m=(const char *[]){"r","rb","rc","rC","rm","rM","rD","rO","rB",NULL};*m;m++){t=TIFFOpen(path,*m);assert(t);TIFFClose(t);}
    t=TIFFOpen(path,"rh");assert(t && TIFFCurrentDirOffset(t)==0);uint32_t w=77;assert(!TIFFGetField(t,256,&w)&&w==77);assert(TIFFReadDirectory(t));assert(TIFFGetField(t,256,&w)&&w==7);TIFFClose(t);return 0;
  }
  if(!strcmp(mode,"chop")){
    TIFF *t=TIFFOpen(path,"rc");assert(t && TIFFNumberOfStrips(t)==1);TIFFClose(t);
    t=TIFFOpen(path,"rC");assert(t && TIFFNumberOfStrips(t)==5);uint32_t rows;assert(TIFFGetField(t,278,&rows)&&rows==2);
    unsigned char out[8192];for(uint32_t i=0;i<5;i++){assert(TIFFReadEncodedStrip(t,i,out,-1)==8192);for(size_t j=0;j<8192;j++)assert(out[j]==(unsigned char)j);}TIFFClose(t);return 0;
  }
  TIFF *t=TIFFOpen(path,"rc");assert(t);
  if(!strcmp(mode,"subifd")){
    assert(TIFFIsBigTIFF(t)==atoi(argv[3]));assert(TIFFNumberOfDirectories(t)==1);uint16_t n;uint64_t *offs;
    assert(TIFFGetField(t,TIFFTAG_SUBIFD,&n,&offs)&&n==1);uint64_t child=offs[0];
    assert(TIFFSetSubDirectory(t,child));uint32_t w;assert(TIFFGetField(t,256,&w)&&w==3);unsigned char row[16];assert(TIFFReadScanline(t,row,0,0)==1);
    assert(!TIFFSetSubDirectory(t,UINT64_MAX));assert(TIFFCurrentDirOffset(t)==child);assert(TIFFGetField(t,256,&w)&&w==3);
    assert(TIFFSetDirectory(t,0));assert(TIFFGetField(t,256,&w)&&w==7);
  }else if(!strcmp(mode,"cycle")){
    assert(TIFFNumberOfDirectories(t)==0 && errno==ELOOP);uint64_t off=TIFFCurrentDirOffset(t);
    assert(!TIFFSetDirectory(t,2));assert(TIFFCurrentDirOffset(t)==off);
    assert(!TIFFReadDirectory(t) && errno==ELOOP);
  }else assert(0);
  TIFFClose(t);return 0;
}
