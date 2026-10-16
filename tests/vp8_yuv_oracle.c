/* Pinned libwebp YUV output oracle, with default normative filtering. */
#include <stdio.h>
#include <stdlib.h>
#include <webp/decode.h>
int main(int argc,char**argv){if(argc!=3)return 2;FILE*f=fopen(argv[1],"rb");if(!f)return 3;fseek(f,0,SEEK_END);long n=ftell(f);rewind(f);if(n<=0||n>64*1024*1024)return 3;unsigned char*s=malloc((size_t)n);if(!s||fread(s,1,(size_t)n,f)!=(size_t)n)return 3;fclose(f);int w,h,stride,uvstride;unsigned char*u,*v,*y=WebPDecodeYUV(s,(size_t)n,&w,&h,&u,&v,&stride,&uvstride);free(s);if(!y)return 4;f=fopen(argv[2],"wb");if(!f)return 3;for(int j=0;j<h;j++)fwrite(y+j*stride,1,(size_t)w,f);for(int j=0;j<(h+1)/2;j++)fwrite(u+j*uvstride,1,(size_t)(w+1)/2,f);for(int j=0;j<(h+1)/2;j++)fwrite(v+j*uvstride,1,(size_t)(w+1)/2,f);WebPFree(y);return fclose(f)!=0;}
