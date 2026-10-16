#ifndef VT_BRIDGE_H
#define VT_BRIDGE_H
#include "include/verified_tiff.h"
#include <stddef.h>
#include <stdint.h>
void *vt_open_fd(int, size_t, size_t, size_t, size_t, size_t, size_t, uint32_t);
void *vt_open_client(thandle_t, TIFFReadWriteProc, TIFFSeekProc, uint64_t, size_t, size_t, size_t, size_t, size_t, size_t, uint32_t);
int vt_next(void *);
int vt_clientdata(void *,thandle_t);
const uint8_t *vt_read_size(void *,uint32_t,uint32_t,uint32_t,size_t,size_t *);
int vt_read_buffer(void *,uint8_t *,size_t);
int vt_unset(void *,uint32_t);
int vt_free_directory(void *);
int vt_strile(void *,uint32_t,int,uint64_t *);
const uint8_t *vt_field(void *, uint32_t, uint16_t *, size_t *);
int vt_real(void *, uint32_t, float *);
int vt_pair(void *, uint32_t, int, uint16_t *);
uint32_t vt_tag(void *, size_t);
const uint8_t *vt_region(void *,uint32_t,uint32_t,uint32_t,uint32_t,uint32_t,size_t *);
void vt_close(void *);
int vt_put_real(void *,uint32_t,uint16_t,uint16_t,size_t,const void *);
int vt_put(void *, uint32_t, uint16_t, size_t, const uint8_t *);
const uint8_t *vt_user_buffer(void *,uint32_t,const uint8_t *,size_t,size_t);
int vt_write_check(void *);
const uint8_t *vt_directory_bytes(void *,size_t *);
int vt_typed(void *, uint32_t, uint16_t, const uint8_t **, size_t *);
int vt_custom_dir(void *, uint64_t, uint32_t);
int vt_mem_claim(void *, size_t);
void vt_mem_release(void *, size_t);
int vt_error_code(void);
const char *vt_error_message(void);
int vt_select(void *, uint32_t);
int vt_scalar(void *, uint32_t, int, uint64_t *);
uint64_t vt_info(void *, uint32_t);
const void *vt_array(void *, uint32_t, size_t *);
uint32_t vt_count_dirs(void *);
int vt_set_offset(void *, uint64_t);
int64_t vt_size(void *, uint32_t, uint32_t);
int vt_mode(void *, uint32_t, int);
int vt_rgba_ok(void *);
const uint8_t *vt_read(void *, uint32_t, uint32_t, uint32_t, uint32_t, uint32_t,
                       size_t *);
void vt_not_implemented(const char *);
void vt_not_implemented_for(TIFF *, const char *);
uint8_t vt_field_plan(uint16_t,int,int,size_t,int,size_t,size_t);
int vt_field_registration(uint16_t,int,int,int);
uint16_t vt_host_type(uint16_t);
uint32_t vt_color_index(uint32_t,uint32_t);
uint32_t vt_ycbcr_tables(int32_t,int32_t,int32_t,int32_t,int32_t);
int vt_commit(uint8_t *,size_t,size_t,const uint8_t *,size_t);
#endif
