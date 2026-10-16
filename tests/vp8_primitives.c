/* Independent libwebp arithmetic/transform oracle. Test dependency only. */
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include "src/dsp/dsp.h"
#include "src/utils/bit_reader_utils.h"
#include "src/utils/bit_reader_inl_utils.h"
extern VP8CPUInfo VP8GetCPUInfo;
int main(int argc,char**argv){
 if(argc!=3)return 2;FILE*in=fopen(argv[1],"rb");FILE*out=fopen(argv[2],"wb");if(!in||!out)return 3;
 VP8GetCPUInfo=NULL;VP8DspInit();int mode;
 while((mode=fgetc(in))!=EOF){
  if(mode==0){uint8_t bytes[256],probs[256];if(fread(bytes,1,256,in)!=256||fread(probs,1,256,in)!=256)return 4;
   VP8BitReader br;VP8InitBitReader(&br,bytes,256);for(int i=0;i<256;i++)fputc(VP8GetBit(&br,probs[i],"test"),out);
  }else if(mode==13){uint8_t e[8],area[16*BPS]={0};int simple=fgetc(in),edge=fgetc(in),limit=fgetc(in),inner=fgetc(in),hev=fgetc(in);if(fread(e,1,8,in)!=8)return 4;for(int j=0;j<16;j++)memcpy(area+j*BPS,e,8);if(simple)VP8SimpleHFilter16(area+4,BPS,limit);else if(edge)VP8HFilter16(area+4,BPS,limit,inner,hev);else VP8HFilter16i(area,BPS,limit,inner,hev);if(!simple&&!edge){area[6]=e[6];area[7]=e[7];}fwrite(area,1,8,out);
  }else if(mode>=3){uint8_t e[13],area[5*BPS]={0};if(fread(e,1,13,in)!=13)return 4;uint8_t*dst=area+BPS+1;dst[-BPS-1]=e[0];for(int j=0;j<8;j++)dst[j-BPS]=e[j+1];for(int j=0;j<4;j++)dst[j*BPS-1]=e[j+9];VP8PredLuma4[mode-3](dst);for(int j=0;j<4;j++)fwrite(dst+j*BPS,1,4,out);
  }else{int16_t c[16];for(int i=0;i<16;i++){int lo=fgetc(in),hi=fgetc(in);if(hi<0)return 4;c[i]=(int16_t)(lo+256*hi);}
   if(mode==1){int16_t dst[256]={0};VP8TransformWHT(c,dst);for(int i=0;i<16;i++){unsigned v=(uint16_t)dst[16*i];fputc(v&255,out);fputc(v>>8,out);}}
   else{uint8_t dst[4*BPS]={0};for(int y=0;y<4;y++)if(fread(dst+y*BPS,1,4,in)!=4)return 4;VP8Transform(c,dst,0);for(int y=0;y<4;y++)fwrite(dst+y*BPS,1,4,out);}
  }
 }fclose(in);return fclose(out)!=0;
}
