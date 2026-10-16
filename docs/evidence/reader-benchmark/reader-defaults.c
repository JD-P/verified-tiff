/* Same fresh-handle C API workload for the adapter and libtiff 4.7.1.
 * Timing excludes input loading, output allocation, hashing and dumping.
 * No cached decoder survives an operation. File mode is for separate RSS runs. */
#define _POSIX_C_SOURCE 200809L
#include <tiffio.h>
#ifdef VERIFIED_ADAPTER
#include <verified_tiff.h>
#endif
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
struct source {const char *path,*mode;uint8_t *data;size_t size,at;int file;};
static tmsize_t rd(thandle_t c,void *out,tmsize_t n){struct source *s=c;if(n<0)return -1;size_t k=(size_t)n;if(k>s->size-s->at)k=s->size-s->at;memcpy(out,s->data+s->at,k);s->at+=k;return (tmsize_t)k;}
static tmsize_t wr(thandle_t c,void *p,tmsize_t n){(void)c;(void)p;(void)n;return -1;}
static toff_t seekto(thandle_t c,toff_t off,int whence){struct source *s=c;uint64_t base=whence==SEEK_SET?0:whence==SEEK_CUR?s->at:whence==SEEK_END?s->size:UINT64_MAX;if(base>s->size||off>s->size-base)return (toff_t)-1;s->at=(size_t)(base+off);return s->at;}
static toff_t sz(thandle_t c){return ((struct source*)c)->size;}
static int cl(thandle_t c){(void)c;return 0;}
static TIFF *open_source(struct source *s){s->at=0;TIFFOpenOptions *o=TIFFOpenOptionsAlloc();assert(o);
#ifdef VERIFIED_ADAPTER
 assert(VTIFFOpenOptionsSetLimits(o,UINT64_C(1)<<34,UINT64_C(1)<<30,UINT64_C(1000000000000)));
 TIFFOpenOptionsSetMaxSingleMemAlloc(o,1<<30);TIFFOpenOptionsSetMaxCumulatedMemAlloc(o,(tmsize_t)2<<30);
#endif
 TIFF *t=s->file?TIFFOpenExt(s->path,s->mode,o):TIFFClientOpenExt("benchmark",s->mode,s,rd,wr,seekto,cl,sz,NULL,NULL,o);TIFFOpenOptionsFree(o);assert(t);return t;}
static double now(void){struct timespec t;assert(!clock_gettime(CLOCK_MONOTONIC,&t));return t.tv_sec+t.tv_nsec*1e-9;}
static uint64_t hash(const uint8_t *p,size_t n){uint64_t h=UINT64_C(14695981039346656037);for(size_t i=0;i<n;i++)h=(h^p[i])*UINT64_C(1099511628211);return h;}
struct image {uint32_t w,h,bw,bh,n,cw,ch,x,y;uint16_t planes;size_t row,block,bytes;int tiled;uint8_t *out;};
static void settings(TIFF *t){uint16_t photo=0;TIFFGetField(t,TIFFTAG_PHOTOMETRIC,&photo);if(photo==PHOTOMETRIC_YCBCR)assert(TIFFSetField(t,TIFFTAG_JPEGCOLORMODE,JPEGCOLORMODE_RGB));if(photo==PHOTOMETRIC_LOGLUV)assert(TIFFSetField(t,TIFFTAG_SGILOGDATAFMT,SGILOGDATAFMT_16BIT));}
static void setup(TIFF *t,struct image *im,const char *mode){
 assert(TIFFGetField(t,TIFFTAG_IMAGEWIDTH,&im->w)&&TIFFGetField(t,TIFFTAG_IMAGELENGTH,&im->h));
 if(!strncmp(mode,"open",4)||!strncmp(mode,"lookup",6)){im->bytes=24;return;}
 settings(t);im->tiled=TIFFIsTiled(t);im->row=(size_t)TIFFScanlineSize(t);uint16_t pc=1,spp=1;TIFFGetFieldDefaulted(t,TIFFTAG_PLANARCONFIG,&pc);TIFFGetFieldDefaulted(t,TIFFTAG_SAMPLESPERPIXEL,&spp);im->planes=pc==2?spp:1;
 if(!strcmp(mode,"rgba")){im->bytes=(size_t)im->w*im->h*4;return;}
 if(!strcmp(mode,"crop")){im->cw=im->w<128?im->w:128;im->ch=im->h<128?im->h:128;im->x=(im->w-im->cw)/2;im->y=(im->h-im->ch)/2;im->bytes=(size_t)im->cw*im->ch*4;return;}
 if(!strcmp(mode,"firstrow")){assert(!im->tiled);im->bytes=im->row;return;}
 if(!strcmp(mode,"scan")){assert(!im->tiled);im->bytes=im->row*im->h*im->planes;return;}
 assert(!strcmp(mode,"native"));im->n=im->tiled?TIFFNumberOfTiles(t):TIFFNumberOfStrips(t);im->block=(size_t)(im->tiled?TIFFTileSize(t):TIFFStripSize(t));
 if(im->tiled)im->bytes=im->block*im->n;else im->bytes=im->row*im->h*im->planes;
 assert(im->bytes&&im->bytes<=((size_t)1<<30));
}
static void operation(struct source *s,struct image *im,const char *mode){
 TIFF *t=open_source(s);
 if(!strncmp(mode,"open",4)||!strncmp(mode,"lookup",6)){
   uint64_t v[3]={0};uint32_t w=0,h=0;assert(TIFFGetField(t,TIFFTAG_IMAGEWIDTH,&w)&&TIFFGetField(t,TIFFTAG_IMAGELENGTH,&h));v[0]=w;v[1]=h;
   if(!strncmp(mode,"lookup",6)){uint32_t n=TIFFIsTiled(t)?TIFFNumberOfTiles(t):TIFFNumberOfStrips(t);int error=0;v[2]=TIFFGetStrileOffsetWithErr(t,n-1,&error);assert(!error);}
   memcpy(im->out,v,sizeof(v));
 }else if(!strcmp(mode,"rgba")){assert(TIFFReadRGBAImageOriented(t,im->w,im->h,(uint32_t*)im->out,ORIENTATION_TOPLEFT,1));}
 else if(!strcmp(mode,"crop")){TIFFRGBAImage img;char error[1024];assert(TIFFRGBAImageBegin(&img,t,1,error));img.req_orientation=ORIENTATION_TOPLEFT;img.row_offset=(int)im->y;img.col_offset=(int)im->x;assert(TIFFRGBAImageGet(&img,(uint32_t*)im->out,im->cw,im->ch));TIFFRGBAImageEnd(&img);}
 else{
   settings(t);
   if(!strcmp(mode,"scan")||!strcmp(mode,"firstrow")){uint32_t h=!strcmp(mode,"firstrow")?1:im->h;uint16_t planes=!strcmp(mode,"firstrow")?1:im->planes;for(uint16_t p=0;p<planes;p++)for(uint32_t y=0;y<h;y++)assert(TIFFReadScanline(t,im->out+((size_t)p*h+y)*im->row,y,p)==1);}
   else{size_t at=0;for(uint32_t k=0;k<im->n;k++){size_t n=im->tiled?im->block:(size_t)TIFFVStripSize(t,(uint32_t)((im->bytes-at)/im->row));if(n>im->block)n=im->block;if(n>im->bytes-at)n=im->bytes-at;tmsize_t got=im->tiled?TIFFReadEncodedTile(t,k,im->out+at,(tmsize_t)n):TIFFReadEncodedStrip(t,k,im->out+at,(tmsize_t)n);assert(got==(tmsize_t)n);at+=n;}assert(at==im->bytes);}
 }
 TIFFClose(t);
}
static long peak_rss(void){FILE *f=fopen("/proc/self/status","r");assert(f);char line[256];long n=-1;while(fgets(line,sizeof(line),f))if(sscanf(line,"VmHWM: %ld kB",&n)==1)break;fclose(f);return n;}
int main(int argc,char **argv){
 if(argc==2&&!strcmp(argv[1],"--version")){puts(TIFFGetVersion());return 0;}
 assert(argc==5||argc==6);const char *mode=argv[2];double ms=atof(argv[3]);struct source s={.path=argv[1],.mode=strstr(mode,"lazy")?"rmOc":"rm",.file=!strcmp(argv[4],"file")};
 TIFFSetWarningHandler(NULL);
 if(!s.file){FILE *f=fopen(s.path,"rb");assert(f&&!fseek(f,0,SEEK_END));long n=ftell(f);assert(n>0);s.size=(size_t)n;s.data=malloc(s.size);assert(s.data);rewind(f);assert(fread(s.data,1,s.size,f)==s.size);fclose(f);}
 struct image im={0};TIFF *prepared=open_source(&s);setup(prepared,&im,mode);TIFFClose(prepared);im.out=calloc(im.bytes,1);assert(im.out);
 double start=now();operation(&s,&im,mode);double elapsed=now()-start;uint64_t count=1;
 if(ms>0){uint64_t batch=(uint64_t)((ms/1000)/(elapsed>1e-9?elapsed:1e-9))+1;if(batch>100000)batch=100000;start=now();count=0;do{for(uint64_t i=0;i<batch;i++)operation(&s,&im,mode);count+=batch;elapsed=now()-start;}while(elapsed<ms/1000);}
 long rss=peak_rss();uint64_t digest=hash(im.out,im.bytes);
 if(argc==6){FILE *f=fopen(argv[5],"wb");assert(f&&fwrite(im.out,1,im.bytes,f)==im.bytes);assert(!fclose(f));}
 printf("{\"width\":%u,\"height\":%u,\"bytes\":%zu,\"iterations\":%llu,\"seconds\":%.9g,\"ns_per_operation\":%.9g,\"peak_rss_kib\":%ld,\"hash\":\"%016llx\"}\n",im.w,im.h,im.bytes,(unsigned long long)count,elapsed,elapsed*1e9/count,rss,(unsigned long long)digest);
 free(im.out);free(s.data);return 0;
}
