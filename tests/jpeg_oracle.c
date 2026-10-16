/* Test-only independent JPEG reference. No part is linked into the Rust core. */
#include <stdio.h>
#include <stdlib.h>
#include <setjmp.h>
#include <jpeglib.h>
struct error_state {struct jpeg_error_mgr base;jmp_buf jump;};
static void fail(j_common_ptr c) {longjmp(((struct error_state *)c->err)->jump,1);}
int main(int argc,char **argv) {
    if(argc!=3&&argc!=4)return 2;
    FILE *in=fopen(argv[1],"rb"),*out=fopen(argv[2],"wb");if(!in||!out)return 2;
    struct jpeg_decompress_struct c;struct error_state e;
    c.err=jpeg_std_error(&e.base);e.base.error_exit=fail;
    if(setjmp(e.jump)){jpeg_destroy_decompress(&c);fclose(in);fclose(out);return 1;}
    jpeg_create_decompress(&c);jpeg_stdio_src(&c,in);jpeg_read_header(&c,TRUE);
    c.do_fancy_upsampling=FALSE;c.dct_method=JDCT_ISLOW;
    c.out_color_space=c.num_components==1?JCS_GRAYSCALE:(argc==4?JCS_YCbCr:JCS_RGB);
    jpeg_start_decompress(&c);
    size_t n=(size_t)c.output_width*c.output_components;unsigned char *row=malloc(n);if(!row)return 2;
    while(c.output_scanline<c.output_height){JSAMPROW p=row;jpeg_read_scanlines(&c,&p,1);if(fwrite(row,1,n,out)!=n)return 2;}
    jpeg_finish_decompress(&c);jpeg_destroy_decompress(&c);free(row);fclose(in);fclose(out);return 0;
}
