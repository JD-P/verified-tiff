/* Independent public-ABI regression consumer, also linked to libtiff 4.7.1. */
#include <tiffio.h>
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static tmsize_t read_file(thandle_t f,void *p,tmsize_t n){assert(n>=0);return (tmsize_t)fread(p,1,(size_t)n,f);}
static tmsize_t write_file(thandle_t f,void *p,tmsize_t n){assert(n>=0);return (tmsize_t)fwrite(p,1,(size_t)n,f);}
static toff_t seek_file(thandle_t f,toff_t n,int whence){if(n>INT64_MAX||fseek(f,(long)n,whence))return (toff_t)-1;return (toff_t)ftell(f);}
static toff_t size_file(thandle_t f){long at=ftell(f);assert(at>=0&&!fseek(f,0,SEEK_END));long end=ftell(f);assert(end>=0&&!fseek(f,at,SEEK_SET));return (toff_t)end;}
static int close_file(thandle_t f){return fclose(f);}

struct codec_field {uint32_t tag;TIFFDataType type;int size,count_size;};
static const struct codec_field codecs[]={
 {292,TIFF_LONG,4,0},{293,TIFF_LONG,4,0},{326,TIFF_LONG,4,0},
 {327,TIFF_SHORT,2,0},{328,TIFF_LONG,4,0},{317,TIFF_SHORT,2,0},
 {347,TIFF_UNDEFINED,1,4},{512,TIFF_SHORT,2,0},{513,TIFF_LONG8,8,0},
 {514,TIFF_LONG8,8,0},{515,TIFF_SHORT,2,0},{519,TIFF_LONG8,8,4},
 {520,TIFF_LONG8,8,4},{521,TIFF_LONG8,8,4},{50674,TIFF_LONG,4,4},
 {34908,TIFF_LONG,4,0},{34909,TIFF_ASCII,1,0},{34910,TIFF_LONG,4,0},{34911,TIFF_ASCII,1,0}
};
static void codec(TIFF *t,uint32_t tag,int present){
 const struct codec_field *c=NULL;
 for(size_t i=0;i<sizeof(codecs)/sizeof(*codecs);i++)if(codecs[i].tag==tag)c=codecs+i;
 assert(c);const TIFFField *f=TIFFFieldWithTag(t,tag);assert(f);
#ifndef ORACLE
 TIFFFieldInfo conflicting={tag,-3,-3,TIFF_BYTE,FIELD_CUSTOM,1,1,"MustNotOverrideBuiltin"};
 assert(TIFFMergeFieldInfo(t,&conflicting,1)==0);assert(TIFFFieldWithTag(t,tag)==f);
#endif
 assert(!TIFFFieldIsAnonymous(f));assert(TIFFFieldDataType(f)==c->type);
 assert(TIFFFieldSetGetSize(f)==c->size);assert(TIFFFieldSetGetCountSize(f)==c->count_size);
 assert(TIFFFieldPassCount(f)==(c->count_size!=0));
 struct {uint32_t before;uint16_t v;unsigned char after[18];} s16;
 struct {uint32_t before,v;unsigned char after[16];} s32;
 struct {uint64_t before,v;unsigned char after[16];} s64;
 memset(&s16,0xa5,sizeof(s16));memset(&s32,0xa5,sizeof(s32));memset(&s64,0xa5,sizeof(s64));
 unsigned char copy16[sizeof(s16)],copy32[sizeof(s32)],copy64[sizeof(s64)];
 memcpy(copy16,&s16,sizeof(s16));memcpy(copy32,&s32,sizeof(s32));memcpy(copy64,&s64,sizeof(s64));
 if(c->type==TIFF_ASCII){
  struct {uintptr_t before;char *p;uintptr_t after;} ptr={17,NULL,29};
  assert(TIFFGetField(t,tag,&ptr.p)==present);assert(ptr.before==17&&ptr.after==29);
  if(present){assert(ptr.p);puts(ptr.p);}else{assert(!ptr.p);}
 }else if(c->count_size){
  struct {uint32_t before,n,after;} count={17,0xa5a5a5a5,29};
  struct {uintptr_t before;void *p;uintptr_t after;} ptr={17,NULL,29};
  assert(TIFFGetField(t,tag,&count.n,&ptr.p)==present);
  assert(count.before==17&&count.after==29&&ptr.before==17&&ptr.after==29);
  if(!present){assert(count.n==0xa5a5a5a5&&ptr.p==NULL);return;}
  printf("%u",count.n);
  for(uint32_t i=0;i<count.n;i++){
   uint64_t v=c->size==8?((uint64_t*)ptr.p)[i]:c->size==4?((uint32_t*)ptr.p)[i]:((uint8_t*)ptr.p)[i];
   printf(" %llu",(unsigned long long)v);
  }puts("");
 }else{
  int r=c->size==2?TIFFGetField(t,tag,&s16.v):c->size==4?TIFFGetField(t,tag,&s32.v):TIFFGetField(t,tag,&s64.v);
  assert(r==present);
  uint64_t v=c->size==2?s16.v:c->size==4?s32.v:s64.v;
  if(present){s16.v=0xa5a5;s32.v=0xa5a5a5a5;s64.v=UINT64_C(0xa5a5a5a5a5a5a5a5);}
  assert(!memcmp(copy16,&s16,sizeof(s16)));assert(!memcmp(copy32,&s32,sizeof(s32)));assert(!memcmp(copy64,&s64,sizeof(s64)));
  if(present)printf("%llu\n",(unsigned long long)v);
 }
}
static void ink_check(TIFF *t,uint16_t expected,const char *bytes,size_t len){
 struct {uint16_t before,n,after;} n={17,0,29};
 struct {uintptr_t before;char *p;uintptr_t after;} names={17,NULL,29};
 assert(TIFFGetField(t,TIFFTAG_NUMBEROFINKS,&n.n));assert(n.n==expected&&n.before==17&&n.after==29);
 // Exactly one output argument: InkNames does NOT use its setter's count ABI.
 assert(TIFFGetField(t,TIFFTAG_INKNAMES,&names.p));assert(names.p&&names.before==17&&names.after==29);
 if(bytes)assert(!memcmp(names.p,bytes,len));
}
static void mutate(TIFF *t){
 const char one[]="black",two[]="cyan\0magenta",three[]="c\0m\0y";
 assert(TIFFSetField(t,TIFFTAG_NUMBEROFINKS,9));
 assert(TIFFSetField(t,TIFFTAG_INKNAMES,(int)sizeof(two),two));ink_check(t,2,two,sizeof(two));
 assert(!TIFFSetField(t,TIFFTAG_NUMBEROFINKS,9));ink_check(t,2,two,sizeof(two));
 assert(TIFFSetField(t,TIFFTAG_NUMBEROFINKS,2));
 assert(!TIFFSetField(t,TIFFTAG_INKNAMES,0,""));
 assert(!TIFFSetField(t,TIFFTAG_INKNAMES,4,"cyan"));
 assert(!TIFFSetField(t,TIFFTAG_INKNAMES,7,"cyan\0xy"));
#ifndef ORACLE
 assert(!TIFFSetField(t,TIFFTAG_INKNAMES,1,NULL));
 assert(!TIFFGetField(t,TIFFTAG_INKNAMES,NULL));
 assert(!TIFFUnsetField(t,TIFFTAG_NUMBEROFINKS));ink_check(t,2,two,sizeof(two));
#endif
 assert(TIFFSetField(t,TIFFTAG_INKNAMES,(int)sizeof(three),three));ink_check(t,3,three,sizeof(three));
 assert(TIFFSetField(t,TIFFTAG_INKNAMES,(int)sizeof(one),one));ink_check(t,1,one,sizeof(one));
#ifndef ORACLE
 assert(TIFFUnsetField(t,TIFFTAG_INKNAMES));assert(TIFFUnsetField(t,TIFFTAG_NUMBEROFINKS));
 uint16_t n=0;char *p=NULL;assert(!TIFFGetField(t,TIFFTAG_INKNAMES,&p));assert(!TIFFGetField(t,TIFFTAG_NUMBEROFINKS,&n));
 assert(TIFFSetField(t,TIFFTAG_INKNAMES,(int)sizeof(two),two));ink_check(t,2,two,sizeof(two));
#endif
}
int main(int argc,char **argv){
 assert(argc>=3);TIFFSetErrorHandler(NULL);TIFFSetWarningHandler(NULL);
 const char *mode=argv[1];
 TIFF *t;
 if(!strcmp(mode,"writer")){
  assert(argc==4);FILE *f=fopen(argv[2],"w+b");assert(f);
  t=TIFFClientOpen("temporary inks",argv[3],f,read_file,write_file,seek_file,close_file,size_file,NULL,NULL);
 }else{t=TIFFOpen(argv[2],"rc");}
 if(!strcmp(mode,"reject")){assert(!t);return 0;}assert(t);
 if(!strcmp(mode,"codec")){assert(argc==5);codec(t,(uint32_t)strtoul(argv[3],NULL,10),atoi(argv[4]));}
 else if(!strcmp(mode,"ink")){
  assert(argc==5);uint16_t n=(uint16_t)strtoul(argv[3],NULL,10);size_t len=strtoul(argv[4],NULL,10);
  ink_check(t,n,NULL,0);char *p=NULL;assert(TIFFGetField(t,TIFFTAG_INKNAMES,&p));printf("%u\n",n);assert(fwrite(p,1,len,stdout)==len);
 }else if(!strcmp(mode,"mutate")||!strcmp(mode,"writer")){
  mutate(t);
  if(!strcmp(mode,"mutate")){unsigned char b=0;assert(TIFFReadEncodedStrip(t,0,&b,1)==1&&b==37);}
  else{
   assert(TIFFSetField(t,TIFFTAG_IMAGEWIDTH,1u));assert(TIFFSetField(t,TIFFTAG_IMAGELENGTH,1u));
   assert(TIFFSetField(t,TIFFTAG_BITSPERSAMPLE,8));assert(TIFFSetField(t,TIFFTAG_SAMPLESPERPIXEL,1));
   assert(TIFFSetField(t,TIFFTAG_PHOTOMETRIC,1));assert(TIFFSetField(t,TIFFTAG_ROWSPERSTRIP,1u));
#ifndef ORACLE
   assert(TIFFWriteCheck(t,0,"metadata test"));assert(TIFFWriteDirectory(t));
#endif
  }
#ifndef ORACLE
 }else if(!strcmp(mode,"select")){
  uint64_t first=TIFFCurrentDirOffset(t);char *p=NULL;
  assert(!TIFFGetField(t,TIFFTAG_INKNAMES,&p));assert(TIFFReadDirectory(t));ink_check(t,2,"cyan\0magenta\0",13);
  assert(!TIFFReadDirectory(t));ink_check(t,2,"cyan\0magenta\0",13);
  assert(TIFFSetSubDirectory(t,first));assert(!TIFFGetField(t,TIFFTAG_INKNAMES,&p));
 }else if(!strcmp(mode,"namespace")){
  assert(TIFFReadEXIFDirectory(t,TIFFCurrentDirOffset(t)));
  TIFFFieldInfo fields[]={{333,1,1,TIFF_LONG,FIELD_CUSTOM,1,0,"Private333"},{334,1,1,TIFF_LONG,FIELD_CUSTOM,1,0,"Private334"}};
  assert(TIFFMergeFieldInfo(t,fields,2)==0);assert(TIFFSetField(t,333,42u));assert(TIFFSetField(t,334,79u));
  uint32_t a=0,b=0;assert(TIFFGetField(t,333,&a)&&a==42);assert(TIFFGetField(t,334,&b)&&b==79);
  assert(TIFFUnsetField(t,334));
#endif
 }else{assert(0);}
 TIFFClose(t);return 0;
}
