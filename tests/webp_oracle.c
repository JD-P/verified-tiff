/* Test-only independent VP8L oracle. Never linked into the Rust core. */
#include <stdio.h>
#include <stdlib.h>
#include <webp/decode.h>
int main(int argc,char **argv) {
    if(argc!=3)return 2;
    FILE *f=fopen(argv[1],"rb");if(!f)return 3;
    if(fseek(f,0,SEEK_END))return 3;long n=ftell(f);if(n<0 || n>134217728)return 3;
    rewind(f);unsigned char *s=malloc((size_t)n);if(!s)return 3;
    if(fread(s,1,(size_t)n,f)!=(size_t)n)return 3;fclose(f);
    int w,h;unsigned char *rgba=WebPDecodeRGBA(s,(size_t)n,&w,&h);free(s);if(!rgba)return 4;
    f=fopen(argv[2],"wb");if(!f)return 3;size_t count=(size_t)w*(size_t)h*4;
    int ok=fwrite(rgba,1,count,f)==count && fclose(f)==0;WebPFree(rgba);
    if(!ok)return 3;printf("%d %d\n",w,h);return 0;
}
