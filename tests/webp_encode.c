/* Pinned libwebp test encoder. No production linkage. */
#include <stdio.h>
#include <stdlib.h>
#include <webp/encode.h>
int main(int argc,char **argv){
  if(argc!=9&&argc!=16)return 2;
  int w=atoi(argv[2]),h=atoi(argv[3]);
  if(w<1||h<1||w>16384||h>16384||(size_t)w*h>33554432)return 2;
  size_t n=(size_t)w*h*4;unsigned char *data=malloc(n);if(!data)return 3;
  FILE *f=fopen(argv[1],"rb");if(!f||fread(data,1,n,f)!=n)return 3;fclose(f);
  WebPConfig config;WebPPicture pic;WebPMemoryWriter writer;
  if(!WebPConfigInit(&config)||!WebPPictureInit(&pic))return 3;
  config.lossless=atoi(argv[4]);config.quality=(float)atof(argv[5]);config.method=atoi(argv[6]);
  config.image_hint=(WebPImageHint)atoi(argv[7]);config.exact=1;
  if(argc==16){config.alpha_compression=atoi(argv[9]);config.alpha_filtering=atoi(argv[10]);config.alpha_quality=atoi(argv[11]);config.partitions=atoi(argv[12]);config.filter_type=atoi(argv[13]);config.filter_strength=atoi(argv[14]);config.segments=atoi(argv[15]);}
  if(!WebPValidateConfig(&config))return 3;
  pic.width=w;pic.height=h;pic.use_argb=1;
  if(!WebPPictureImportRGBA(&pic,data,w*4))return 3;free(data);
  WebPMemoryWriterInit(&writer);pic.writer=WebPMemoryWrite;pic.custom_ptr=&writer;
  if(!WebPEncode(&config,&pic)){fprintf(stderr,"encode %d\n",pic.error_code);return 4;}
  f=fopen(argv[8],"wb");if(!f||fwrite(writer.mem,1,writer.size,f)!=writer.size)return 3;fclose(f);
  WebPMemoryWriterClear(&writer);WebPPictureFree(&pic);return 0;
}
