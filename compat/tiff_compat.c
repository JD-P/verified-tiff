/* Unverified C ABI/varargs/I/O adapter. No TIFF codec implementation lives
 * here. */
#define _POSIX_C_SOURCE 200809L
#include "bridge.h"
#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <math.h>
#include <stdatomic.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
_Static_assert(sizeof(void *) == 8 && sizeof(tmsize_t) == 8,
               "Linux LP64 ABI required");
struct TIFFOpenOptions {
  size_t input, output, work, single, cumulative;
  int warn_unknown;
  int unsupported;
  unsigned invalid_env;
  TIFFErrorHandlerExtR error, warning;
  void *error_data, *warning_data;
};
struct tiff {
  void *state;
  char *name,*owned_name;
  int fd,owned_fd,mode;
  thandle_t client;
  TIFFReadWriteProc read, write;
  TIFFSeekProc seek;
  TIFFCloseProc close;
  TIFFSizeProc size;
  TIFFMapFileProc map;
  TIFFUnmapFileProc unmap;
  struct TIFFOpenOptions options;
  uint32_t row, strip, tile, flags, directory_kind;
  struct vt_field_node *fields;
  struct vt_client_info *client_info;
  TIFFTagMethods methods;
  size_t field_count;
  int field_error, directory_written;
  uint16_t *default_transfer;size_t transfer_count;float default_reference[6];
};
static _Thread_local int last_code;
static _Thread_local char last_message[512];
static _Thread_local int reporting;
static void default_error(const char *module, const char *fmt, va_list ap) {
  if (module)
    fprintf(stderr, "%s: ", module);
  vfprintf(stderr, fmt, ap);
  fputc('\n', stderr);
}
static _Atomic(TIFFErrorHandler) error_handler = default_error,
                                 warning_handler = default_error;
static _Atomic(TIFFErrorHandlerExt) error_ext, warning_ext;
static void dispatch(TIFF *t, int warning, thandle_t client, const char *module,
                     const char *fmt, va_list ap) {
  va_list cp;
  va_copy(cp, ap);
  vsnprintf(last_message, sizeof(last_message), fmt, cp);
  va_end(cp);
  if (reporting)
    return;
  reporting = 1;
  TIFFErrorHandlerExtR h =
      t ? (warning ? t->options.warning : t->options.error) : NULL;
  int handled = 0;
  if (h) {
    va_copy(cp, ap);
    handled = h(t, warning ? t->options.warning_data : t->options.error_data,
                module, fmt, cp);
    va_end(cp);
  }
  if (!handled) {
    TIFFErrorHandler g =
        atomic_load(warning ? &warning_handler : &error_handler);
    if (g) {
      va_copy(cp, ap);
      g(module, fmt, cp);
      va_end(cp);
    }
    TIFFErrorHandlerExt e = atomic_load(warning ? &warning_ext : &error_ext);
    if (e) {
      va_copy(cp, ap);
      e(client, module, fmt, cp);
      va_end(cp);
    }
  }
  reporting = 0;
}
void TIFFError(const char *m, const char *f, ...) {
  va_list ap;
  va_start(ap, f);
  dispatch(NULL, 0, NULL, m, f, ap);
  va_end(ap);
}
void TIFFWarning(const char *m, const char *f, ...) {
  va_list ap;
  va_start(ap, f);
  dispatch(NULL, 1, NULL, m, f, ap);
  va_end(ap);
}
void TIFFErrorExt(thandle_t c, const char *m, const char *f, ...) {
  va_list ap;
  va_start(ap, f);
  dispatch(NULL, 0, c, m, f, ap);
  va_end(ap);
}
void TIFFWarningExt(thandle_t c, const char *m, const char *f, ...) {
  va_list ap;
  va_start(ap, f);
  dispatch(NULL, 1, c, m, f, ap);
  va_end(ap);
}
void TIFFErrorExtR(TIFF *t, const char *m, const char *f, ...) {
  va_list ap;
  va_start(ap, f);
  dispatch(t, 0, t ? t->client : NULL, m, f, ap);
  va_end(ap);
}
void TIFFWarningExtR(TIFF *t, const char *m, const char *f, ...) {
  va_list ap;
  va_start(ap, f);
  dispatch(t, 1, t ? t->client : NULL, m, f, ap);
  va_end(ap);
}
TIFFErrorHandler TIFFSetErrorHandler(TIFFErrorHandler p) {
  return atomic_exchange(&error_handler, p);
}
TIFFErrorHandler TIFFSetWarningHandler(TIFFErrorHandler p) {
  return atomic_exchange(&warning_handler, p);
}
TIFFErrorHandlerExt TIFFSetErrorHandlerExt(TIFFErrorHandlerExt p) {
  return atomic_exchange(&error_ext, p);
}
TIFFErrorHandlerExt TIFFSetWarningHandlerExt(TIFFErrorHandlerExt p) {
  return atomic_exchange(&warning_ext, p);
}
static void fail(TIFF *t, int code, const char *fn, const char *message) {
  last_code = code;
  TIFFErrorExtR(t, fn, "%s", message);
  errno = code;
}
void vt_not_implemented_for(TIFF *t, const char *name) {
  fail(t, ENOSYS, name, "not implemented by verified-tiff");
}
void vt_not_implemented(const char *name) {
  vt_not_implemented_for(NULL, name);
}
static void bridge_fail(TIFF *t, const char *name) {
  fail(t, vt_error_code(), name, vt_error_message());
}
static int valid(TIFF *t, const char *fn) {
  if (t && t->state)
    return 1;
  fail(t, EINVAL, fn, "invalid/null TIFF handle");
  return 0;
}
int VTIFFLastErrorCode(void) { return last_code; }
const char *VTIFFLastError(void) { return last_message; }
void VTIFFClearError(void) {
  last_code = 0;
  last_message[0] = 0;
}
const char *TIFFGetVersion(void) {
  return "verified-tiff 0.1.0; partial read-only libtiff 4.7.1 C API; adapter "
         "is not formally verified";
}
static size_t configured_limit(const char *name, size_t fallback, int *invalid) {
  const char *s=getenv(name);
  if(!s)return fallback;
  if(!*s){*invalid=1;return fallback;}
  size_t n=0;
  for(const unsigned char *p=(const unsigned char *)s;*p;p++){
    if(*p<'0'||*p>'9'||n>(SIZE_MAX-(*p-'0'))/10){*invalid=1;return fallback;}
    n=n*10+(*p-'0');
  }
  return n;
}
static struct TIFFOpenOptions defaults(void) {
  struct TIFFOpenOptions o = {0};
  int bad=0;
  o.input=configured_limit("VTIFF_MAX_FILE_BYTES",PTRDIFF_MAX,&bad);
  if(bad||o.input>PTRDIFF_MAX)o.invalid_env|=1;
  bad=0;o.output=configured_limit("VTIFF_MAX_BUFFER_BYTES",268435456,&bad);
  if(bad||o.output>PTRDIFF_MAX)o.invalid_env|=2;
  bad=0;o.work=configured_limit("VTIFF_MAX_WORK",1000000000,&bad);
  if(bad)o.invalid_env|=4;
  bad=0;o.single=configured_limit("VTIFF_MAX_SINGLE_ALLOC",268435456,&bad);
  if(bad||o.single>PTRDIFF_MAX)o.invalid_env|=8;
  bad=0;o.cumulative=configured_limit("VTIFF_MAX_TOTAL_ALLOC",536870912,&bad);
  if(bad||o.cumulative>PTRDIFF_MAX)o.invalid_env|=16;
  return o;
}
TIFFOpenOptions *TIFFOpenOptionsAlloc(void) {
  TIFFOpenOptions *o = malloc(sizeof(*o));
  if (o)
    *o = defaults();
  else
    fail(NULL, ENOMEM, __func__, "allocation failed");
  return o;
}
void TIFFOpenOptionsFree(TIFFOpenOptions *o) { free(o); }
int VTIFFOpenOptionsSetLimits(TIFFOpenOptions *o, uint64_t in, uint64_t out,
                              uint64_t work) {
  if (!o || in > PTRDIFF_MAX || out > PTRDIFF_MAX) {
    fail(NULL, EINVAL, __func__, "invalid options/limits");
    return 0;
  }
  o->input = (size_t)in;
  o->output = (size_t)out;
  o->work = (size_t)work;
  o->invalid_env &= ~7u;
  return 1;
}
void TIFFOpenOptionsSetErrorHandlerExtR(TIFFOpenOptions *o,
                                        TIFFErrorHandlerExtR h, void *u) {
  if (!o) {
    fail(NULL, EINVAL, __func__, "null options");
    return;
  }
  o->error = h;
  o->error_data = u;
}
void TIFFOpenOptionsSetWarningHandlerExtR(TIFFOpenOptions *o,
                                          TIFFErrorHandlerExtR h, void *u) {
  if (!o) {
    fail(NULL, EINVAL, __func__, "null options");
    return;
  }
  o->warning = h;
  o->warning_data = u;
}
/* Limits count requested, live per-handle heap buffers. Zero disables a
 * standard allocation limit; extension input/output/work zero remains zero. */
void TIFFOpenOptionsSetMaxSingleMemAlloc(TIFFOpenOptions *o, tmsize_t n) {
  if (!o || n < 0) { if (o) o->unsupported=1; fail(NULL, EINVAL, __func__, "invalid allocation limit"); return; }
  o->single=(size_t)n;
  o->invalid_env &= ~8u;
}
void TIFFOpenOptionsSetMaxCumulatedMemAlloc(TIFFOpenOptions *o, tmsize_t n) {
  if (!o || n < 0) { if (o) o->unsupported=1; fail(NULL, EINVAL, __func__, "invalid allocation limit"); return; }
  o->cumulative=(size_t)n;
  o->invalid_env &= ~16u;
}
void TIFFOpenOptionsSetWarnAboutUnknownTags(TIFFOpenOptions *o, int n) {
  if (!o) { fail(NULL, EINVAL, __func__, "null options"); return; }
  o->warn_unknown=n!=0;
}
static int readmode(const char *mode) {
  if (!mode || *mode!='r') return 0;
  for (const char *p=mode+1;*p;p++) if (!strchr("bcCmMBDOh",*p)) return 0;
  return 1;
}
static int writemode(const char *mode){
  if(!mode||*mode!='w')return 0;
  for(const char *p=mode+1;*p;p++)if(!strchr("bl8+",*p))return 0;
  return 1;
}
static uint32_t modeflags(const char *mode) {
  uint32_t flags=1; /* upstream's usual strip-chopping default */
  for (const char *p=mode+1;*p;p++) {
    if (*p=='C') flags|=1;
    if (*p=='c') flags&=~1u;
    if (*p=='h') flags|=2;
  }
  if(*mode=='w'){flags=4;if(strchr(mode,'8'))flags|=8;if(strchr(mode,'b'))flags|=16;}
  return flags;
}
static void unknown_warnings(TIFF *t);
static void free_fields(TIFF *t);
static void free_client_info(TIFF *t);
static int default_getfield(TIFF *,uint32_t,va_list);
static int default_setfield(TIFF *,uint32_t,va_list);
static int initialise_fields(TIFF *t, unsigned kind);
static int generic_getfield(TIFF *t, uint32_t tag, va_list ap, int def);
static int generic_setfield(TIFF *t,uint32_t tag,va_list ap);
static TIFF *new_handle(const char *name, const char *mode,
                        TIFFOpenOptions *options) {
  if (!readmode(mode) && !writemode(mode)) {
    vt_not_implemented("TIFFOpen unsupported read/write mode");
    return NULL;
  }
  if (!name) {
    fail(NULL, EINVAL, "TIFFOpen", "null name");
    return NULL;
  }
  struct TIFFOpenOptions limits = options ? *options : defaults();
  if(limits.unsupported||limits.invalid_env){fail(NULL,EINVAL,"TIFFOpen","invalid configured resource limit");return NULL;}
  size_t namelen = strnlen(name,4096);
  if(namelen==4096){fail(NULL,EINVAL,"TIFFOpen","name exceeds 4095 bytes");return NULL;}
  namelen++;
  if ((limits.single && (sizeof(TIFF)>limits.single || namelen>limits.single)) ||
      (limits.cumulative && sizeof(TIFF)+namelen>limits.cumulative)) {
    TIFF temporary = {0}; temporary.options=limits;
    fail(&temporary, EFBIG, "TIFFOpen", "handle allocation limit"); return NULL;
  }
  TIFF *t = calloc(1, sizeof(*t));
  if (!t) {
    fail(NULL, ENOMEM, "TIFFOpen", "allocation failed");
    return NULL;
  }
  t->name = strdup(name);
  if (!t->name) {
    free(t);
    fail(NULL, ENOMEM, "TIFFOpen", "allocation failed");
    return NULL;
  }
  t->owned_name=t->name;t->owned_fd=-1;t->mode=writemode(mode)?O_RDWR:O_RDONLY;
  t->options = limits;
  t->methods.vgetfield=default_getfield;t->methods.vsetfield=default_setfield;
  t->fd = -1;
  t->flags=modeflags(mode);
  t->row = t->strip = t->tile = UINT32_MAX;
  return t;
}
static void destroy(TIFF *t) {
  if (!t)
    return;
  if(t->default_transfer){vt_mem_release(t->state,t->transfer_count*sizeof(uint16_t));free(t->default_transfer);}
  free_client_info(t);
  free_fields(t);
  vt_close(t->state);
  free(t->owned_name);
  free(t);
}
TIFF *TIFFFdOpenExt(int fd, const char *name, const char *mode,
                    TIFFOpenOptions *options) {
  if(!readmode(mode)){vt_not_implemented("TIFFFdOpen unsupported write mode");return NULL;}
  TIFF *t = new_handle(name, mode, options);
  if (!t)
    return NULL;
  t->state =
      vt_open_fd(fd, t->options.input, t->options.output, t->options.work, t->options.single, t->options.cumulative, sizeof(*t)+strlen(t->name)+1, t->flags);
  if (!t->state) {
    bridge_fail(t, __func__);
    destroy(t);
    return NULL;
  }
  t->fd = fd;t->owned_fd=fd;
  if (!initialise_fields(t,0)) {destroy(t);return NULL;}
  unknown_warnings(t);
  return t;
}
TIFF *TIFFFdOpen(int fd, const char *name, const char *mode) {
  return TIFFFdOpenExt(fd, name, mode, NULL);
}
TIFF *TIFFOpenExt(const char *name, const char *mode,
                  TIFFOpenOptions *options) {
  if (!readmode(mode)) {
    vt_not_implemented("TIFFOpen unsupported read/write mode");
    return NULL;
  }
  if (!name) {
    fail(NULL, EINVAL, __func__, "null filename");
    return NULL;
  }
  int fd = open(name, O_RDONLY | O_CLOEXEC);
  if (fd < 0) {
    fail(NULL, errno, __func__, "cannot open input");
    return NULL;
  }
  TIFF *t = TIFFFdOpenExt(fd, name, mode, options);
  if (!t) {
    int e = errno;
    close(fd);
    errno = e;
  }
  return t;
}
TIFF *TIFFOpen(const char *name, const char *mode) {
  return TIFFOpenExt(name, mode, NULL);
}
TIFF *TIFFClientOpenExt(const char *name, const char *mode, thandle_t client,
                        TIFFReadWriteProc read, TIFFReadWriteProc write,
                        TIFFSeekProc seek, TIFFCloseProc close,
                        TIFFSizeProc size, TIFFMapFileProc map,
                        TIFFUnmapFileProc unmap, TIFFOpenOptions *o) {
  TIFF *t = new_handle(name, mode, o);
  if (!t)
    return NULL;
  t->client = client;
  t->read = read;
  t->write = write;
  t->seek = seek;
  t->close = close;
  t->size = size;
  t->map = map;
  t->unmap = unmap;
  if (!read || !seek || !size || ((t->flags&4)&&!write)) {
    fail(t, EINVAL, __func__, "read/seek/size callbacks required");
    destroy(t);
    return NULL;
  }
  uint64_t n = size(client);
  if (n > t->options.input || n > PTRDIFF_MAX) {
    fail(t, EFBIG, __func__, "input limit");
    destroy(t);
    return NULL;
  }
  t->state=vt_open_client(client, read, seek, n, t->options.input,
      t->options.output, t->options.work, t->options.single, t->options.cumulative,
      sizeof(*t)+strlen(t->name)+1, t->flags);
  if (!t->state) {
    bridge_fail(t, __func__);
    destroy(t);
    return NULL;
  }
  if (!initialise_fields(t,0)) {destroy(t);return NULL;}
  unknown_warnings(t);
  return t;
}
TIFF *TIFFClientOpen(const char *n, const char *m, thandle_t c,
                     TIFFReadWriteProc r, TIFFReadWriteProc w, TIFFSeekProc s,
                     TIFFCloseProc cl, TIFFSizeProc sz, TIFFMapFileProc mp,
                     TIFFUnmapFileProc u) {
  return TIFFClientOpenExt(n, m, c, r, w, s, cl, sz, mp, u, NULL);
}
void TIFFCleanup(TIFF *t) { destroy(t); }
void TIFFClose(TIFF *t) {
  if (!t)
    return;
  if((t->flags&4)&&!t->directory_written)(void)TIFFWriteDirectory(t);
  int fd = t->owned_fd;
  TIFFCloseProc close_cb = t->close;
  thandle_t c = t->client;
  destroy(t);
  if (close_cb)
    close_cb(c);
  else if (fd >= 0)
    close(fd);
}
const char *TIFFFileName(TIFF *t) { return t ? t->name : NULL; }
const char *TIFFSetFileName(TIFF *t, const char *n) {
  if(!valid(t,__func__)||!n)return NULL;
  const char *old=t->name;t->name=(char*)n;return old;
}
int TIFFFileno(TIFF *t) { return t ? t->fd : -1; }
thandle_t TIFFClientdata(TIFF *t) { return t ? t->client : NULL; }
int TIFFGetMode(TIFF *t) { return valid(t, __func__) ? t->mode : -1; }
TIFFReadWriteProc TIFFGetReadProc(TIFF *t) { return t ? t->read : NULL; }
TIFFReadWriteProc TIFFGetWriteProc(TIFF *t) { return t ? t->write : NULL; }
TIFFSeekProc TIFFGetSeekProc(TIFF *t) { return t ? t->seek : NULL; }
TIFFCloseProc TIFFGetCloseProc(TIFF *t) { return t ? t->close : NULL; }
TIFFSizeProc TIFFGetSizeProc(TIFF *t) { return t ? t->size : NULL; }
TIFFMapFileProc TIFFGetMapFileProc(TIFF *t) { return t ? t->map : NULL; }
TIFFUnmapFileProc TIFFGetUnmapFileProc(TIFF *t) { return t ? t->unmap : NULL; }
static uint64_t info(TIFF *t, unsigned key) {
  if (!valid(t, "TIFF metadata"))
    return 0;
  uint64_t v = vt_info(t->state, key);
  if (v == UINT64_MAX) {
    bridge_fail(t, "TIFF metadata");
    return 0;
  }
  return v;
}
int TIFFIsTiled(TIFF *t) { return (int)info(t, 5); }
int TIFFIsBigEndian(TIFF *t) { return valid(t, __func__) ? !info(t, 6) : 0; }
int TIFFIsByteSwapped(TIFF *t) {
  return valid(t, __func__) ? (TIFFIsBigEndian(t) != HOST_BIGENDIAN) : 0;
}
int TIFFIsMSB2LSB(TIFF *t) { return valid(t, __func__) ? 1 : 0; }
int TIFFIsBigTIFF(TIFF *t) { return (int)info(t, 14); }
int TIFFIsUpSampled(TIFF *t) {
  uint64_t photo = 0, rgb = 0;
  return valid(t, __func__) && vt_scalar(t->state, 262, 1, &photo) == 1 &&
         photo == 6 && vt_scalar(t->state, 65538, 1, &rgb) == 1 && rgb;
}
tdir_t TIFFCurrentDirectory(TIFF *t) { return (tdir_t)info(t, 2); }
uint64_t TIFFCurrentDirOffset(TIFF *t) { return info(t, 3); }
uint32_t TIFFCurrentRow(TIFF *t) { return t ? t->row : UINT32_MAX; }
uint32_t TIFFCurrentStrip(TIFF *t) { return t ? t->strip : UINT32_MAX; }
uint32_t TIFFCurrentTile(TIFF *t) { return t ? t->tile : UINT32_MAX; }
int TIFFLastDirectory(TIFF *t) { return valid(t, __func__) && info(t, 4) == 0; }
tdir_t TIFFNumberOfDirectories(TIFF *t) {
  if (!valid(t, __func__))
    return 0;
  uint32_t n = vt_count_dirs(t->state);
  if (!n)
    bridge_fail(t, __func__);
  return n;
}
int TIFFSetDirectory(TIFF *t, tdir_t d) {
  if (!valid(t, __func__))
    return 0;
  if (!vt_select(t->state, d)) {
    bridge_fail(t, __func__);
    return 0;
  }
  t->row = t->strip = t->tile = UINT32_MAX;
  if (!initialise_fields(t,0)) return 0;
  unknown_warnings(t);
  return 1;
}
int TIFFReadDirectory(TIFF *t) {
  if (!valid(t, __func__) || TIFFLastDirectory(t))
    return 0;
  if (!vt_next(t->state)) { bridge_fail(t, __func__); return 0; }
  t->row=t->strip=t->tile=UINT32_MAX;
  if (!initialise_fields(t,0)) return 0;
  unknown_warnings(t);
  return 1;
}
int TIFFSetSubDirectory(TIFF *t, uint64_t off) {
  if (!valid(t, __func__))
    return 0;
  if (!vt_set_offset(t->state, off)) {
    bridge_fail(t, __func__);
    return 0;
  }
  t->row = t->strip = t->tile = UINT32_MAX;
  if (!initialise_fields(t,0)) return 0;
  unknown_warnings(t);
  return 1;
}
static int getfield(TIFF *t, uint32_t tag, va_list ap, int def) {
  if (!valid(t, "TIFFGetField"))
    return 0;
  if (t->directory_kind) return generic_getfield(t,tag,ap,def);
  uint64_t v = 0;
  int kind = 0;
  switch (tag) {
  case 254:
  case 256:
  case 257:
  case 278:
  case 322:
  case 323:
  case 292:
  case 293:
    kind = 32;
    break;
  case 263:
  case 264:
  case 265:
  case 280:
  case 281:
  case 332:
  case 334:
  case 531:
  case 258:
  case 259:
  case 262:
  case 266:
  case 274:
  case 277:
  case 284:
  case 296:
  case 317:
  case 339:
    kind = 16;
    break;
  case TIFFTAG_JPEGCOLORMODE:
  case TIFFTAG_SGILOGDATAFMT:
    kind = -32;
    break;
  case TIFFTAG_STRIPOFFSETS:
  case TIFFTAG_STRIPBYTECOUNTS:
  case TIFFTAG_TILEOFFSETS:
  case TIFFTAG_TILEBYTECOUNTS: {
    if (((tag == 324 || tag == 325) != TIFFIsTiled(t)))
      return 0;
    size_t n = 0;
    const void *p = vt_array(t->state, tag, &n);
    if (!n)
      return 0;
    uint64_t **out = va_arg(ap, uint64_t **);
    if (!out)
      goto bad;
    *out = (uint64_t *)p;
    return 1;
  }
  case TIFFTAG_EXTRASAMPLES: {
    size_t n = 0;
    const void *p = vt_array(t->state, tag, &n);
    if (!n && !def)
      return 0;
    uint16_t *count = va_arg(ap, uint16_t *);
    uint16_t **out = va_arg(ap, uint16_t **);
    if (!count || !out)
      goto bad;
    *count = (uint16_t)n;
    *out = (uint16_t *)p;
    return 1;
  }
  case TIFFTAG_COLORMAP: {
    size_t n = 0;
    const uint16_t *p = vt_array(t->state, tag, &n);
    if (!p || !n)
      return 0;
    uint16_t **r = va_arg(ap, uint16_t **), **g = va_arg(ap, uint16_t **),
             **b = va_arg(ap, uint16_t **);
    if (!r || !g || !b)
      goto bad;
    *r = (uint16_t *)p;
    *g = (uint16_t *)p + n / 3;
    *b = (uint16_t *)p + 2 * n / 3;
    return 1;
  }
  case TIFFTAG_SUBIFD: {
    size_t n=0; const void *p=vt_array(t->state,tag,&n);
    if (!n) return 0;
    uint16_t *count=va_arg(ap,uint16_t *); uint64_t **out=va_arg(ap,uint64_t **);
    if (!count || !out || n>UINT16_MAX) goto bad;
    *count=(uint16_t)n; *out=(uint64_t *)p; return 1;
  }
  case TIFFTAG_XRESOLUTION: case TIFFTAG_YRESOLUTION:
  case TIFFTAG_XPOSITION: case TIFFTAG_YPOSITION: {
    float value; int ok=vt_real(t->state,tag,&value);
    if (ok<0) { bridge_fail(t,"TIFFGetField"); return 0; } if (!ok) return 0;
    float *out=va_arg(ap,float *); if (!out) goto bad;
    *out=value; return 1;
  }
  case TIFFTAG_YCBCRSUBSAMPLING: case TIFFTAG_PAGENUMBER:
  case TIFFTAG_HALFTONEHINTS: case TIFFTAG_DOTRANGE: {
    uint16_t values[2]; int ok=vt_pair(t->state,tag,def,values);
    if (ok<0) { bridge_fail(t,"TIFFGetField"); return 0; } if (!ok) return 0;
    uint16_t *a=va_arg(ap,uint16_t *), *b=va_arg(ap,uint16_t *);
    if (!a || !b) goto bad;
    *a=values[0]; *b=values[1]; return 1;
  }
  case TIFFTAG_EXIFIFD: case TIFFTAG_GPSIFD:
    kind=64; break;
  /* InkNames has a counted setter, but its getter takes only char **. */
  case TIFFTAG_INKNAMES: case TIFFTAG_DOCUMENTNAME: case TIFFTAG_IMAGEDESCRIPTION:
  case TIFFTAG_MAKE: case TIFFTAG_MODEL: case TIFFTAG_PAGENAME:
  case TIFFTAG_SOFTWARE: case TIFFTAG_DATETIME: case TIFFTAG_ARTIST:
  case TIFFTAG_HOSTCOMPUTER: case TIFFTAG_COPYRIGHT: case TIFFTAG_TARGETPRINTER: {
    uint16_t type=0; size_t n=0; const uint8_t *p=vt_field(t->state,tag,&type,&n);
    if (!p) return 0;
    if (type!=TIFF_ASCII) goto bad;
    char **out=va_arg(ap,char **); if (!out) goto bad;
    *out=(char *)p; return 1;
  }
  case TIFFTAG_ICCPROFILE: case TIFFTAG_XMLPACKET: case TIFFTAG_PHOTOSHOP:
  case TIFFTAG_JPEGTABLES: {
    uint16_t type=0; size_t n=0; const uint8_t *p=vt_field(t->state,tag,&type,&n);
    if (!p) return 0;
    if ((type!=TIFF_BYTE && type!=TIFF_UNDEFINED) || n>UINT32_MAX) goto bad;
    uint32_t *count=va_arg(ap,uint32_t *); void **out=va_arg(ap,void **);
    if (!count || !out) goto bad;
    *count=(uint32_t)n; *out=(void *)p; return 1;
  }
  default:
    return generic_getfield(t,tag,ap,def);
  }
  {
    int ok = vt_scalar(t->state, tag, def, &v);
    if (ok < 0) {
      bridge_fail(t, "TIFFGetField");
      return 0;
    }
    if (!ok)
      return 0;
  }
  if (kind == 16) {
    uint16_t *out = va_arg(ap, uint16_t *);
    if (!out || v > UINT16_MAX)
      goto bad;
    *out = (uint16_t)v;
  } else if (kind == 64) {
    uint64_t *out=va_arg(ap,uint64_t *); if (!out) goto bad;
    *out=v;
  } else if (kind == -32) {
    int *out = va_arg(ap, int *);
    if (!out || v > INT_MAX)
      goto bad;
    *out = (int)v;
  } else {
    uint32_t *out = va_arg(ap, uint32_t *);
    if (!out || v > UINT32_MAX)
      goto bad;
    *out = (uint32_t)v;
  }
  return 1;
bad:
  fail(t, EINVAL, "TIFFGetField", "null output or metadata width mismatch");
  return 0;
}
static int default_getfield(TIFF *t,uint32_t k,va_list ap){return getfield(t,k,ap,0);}
int TIFFVGetField(TIFF *t, uint32_t k, va_list ap) {
  if(!valid(t,__func__)||!t->methods.vgetfield)return 0;
  return t->methods.vgetfield(t,k,ap);
}
int TIFFVGetFieldDefaulted(TIFF *t, uint32_t k, va_list ap) {
  va_list cp;va_copy(cp,ap);int r=TIFFVGetField(t,k,cp);va_end(cp);
  return r?r:getfield(t,k,ap,1);
}
int TIFFGetField(TIFF *t, uint32_t k, ...) {
  va_list ap;
  va_start(ap, k);
  int r = TIFFVGetField(t, k, ap);
  va_end(ap);
  return r;
}
int TIFFGetFieldDefaulted(TIFF *t, uint32_t k, ...) {
  va_list ap;
  va_start(ap, k);
  int r = TIFFVGetFieldDefaulted(t, k, ap);
  va_end(ap);
  return r;
}
static int default_setfield(TIFF *t, uint32_t tag, va_list ap) {
  if (!valid(t, __func__))
    return 0;
  if (tag != TIFFTAG_JPEGCOLORMODE && tag != TIFFTAG_SGILOGDATAFMT) {
    return generic_setfield(t,tag,ap);
  }
  if (!vt_mode(t->state, tag, va_arg(ap, int))) {
    bridge_fail(t, __func__);
    return 0;
  }
  return 1;
}
int TIFFVSetField(TIFF *t,uint32_t tag,va_list ap){
  if(!valid(t,__func__)||!t->methods.vsetfield)return 0;
  return t->methods.vsetfield(t,tag,ap);
}
int TIFFSetField(TIFF *t, uint32_t tag, ...) {
  va_list ap;
  va_start(ap, tag);
  int r = TIFFVSetField(t, tag, ap);
  va_end(ap);
  return r;
}
static uint64_t size64(TIFF *t, unsigned kind, uint32_t rows) {
  if (!valid(t, "TIFF size"))
    return 0;
  int64_t n = vt_size(t->state, kind, rows);
  if (n < 0) {
    bridge_fail(t, "TIFF size");
    return 0;
  }
  return (uint64_t)n;
}
uint64_t TIFFScanlineSize64(TIFF *t) { return size64(t, 0, 1); }
tmsize_t TIFFScanlineSize(TIFF *t) { return (tmsize_t)TIFFScanlineSize64(t); }
uint64_t TIFFRasterScanlineSize64(TIFF *t) {
  uint64_t n = TIFFScanlineSize64(t);
  return info(t, 11) == 2 ? n * info(t, 12) : n;
}
tmsize_t TIFFRasterScanlineSize(TIFF *t) {
  return (tmsize_t)TIFFRasterScanlineSize64(t);
}
uint64_t TIFFStripSize64(TIFF *t) { return size64(t, 1, UINT32_MAX); }
tmsize_t TIFFStripSize(TIFF *t) { return (tmsize_t)TIFFStripSize64(t); }
uint64_t TIFFVStripSize64(TIFF *t, uint32_t n) { return size64(t, 1, n); }
tmsize_t TIFFVStripSize(TIFF *t, uint32_t n) {
  return (tmsize_t)TIFFVStripSize64(t, n);
}
uint64_t TIFFTileRowSize64(TIFF *t) { return size64(t, 2, 1); }
tmsize_t TIFFTileRowSize(TIFF *t) { return (tmsize_t)TIFFTileRowSize64(t); }
uint64_t TIFFTileSize64(TIFF *t) { return size64(t, 3, UINT32_MAX); }
tmsize_t TIFFTileSize(TIFF *t) { return (tmsize_t)TIFFTileSize64(t); }
uint64_t TIFFVTileSize64(TIFF *t, uint32_t n) { return size64(t, 3, n); }
tmsize_t TIFFVTileSize(TIFF *t, uint32_t n) {
  return (tmsize_t)TIFFVTileSize64(t, n);
}
uint32_t TIFFNumberOfStrips(TIFF *t) { return (uint32_t)info(t, 7); }
uint32_t TIFFNumberOfTiles(TIFF *t) { return (uint32_t)info(t, 7); }
static uint64_t strile(TIFF *t, uint32_t index, int counts, int *error) {
  if (error)
    *error = 1;
  if (!valid(t, "TIFFGetStrile"))
    return 0;
  uint64_t v=0;
  if(!vt_strile(t->state,index,counts,&v)){bridge_fail(t,"TIFFGetStrile");return 0;}
  if(error)*error=0;
  return v;
}
uint64_t TIFFGetStrileOffsetWithErr(TIFF *t, uint32_t k, int *e) {
  return strile(t, k, 0, e);
}
uint64_t TIFFGetStrileByteCountWithErr(TIFF *t, uint32_t k, int *e) {
  return strile(t, k, 1, e);
}
uint64_t TIFFGetStrileOffset(TIFF *t, uint32_t k) {
  return strile(t, k, 0, NULL);
}
uint64_t TIFFGetStrileByteCount(TIFF *t, uint32_t k) {
  return strile(t, k, 1, NULL);
}
uint64_t TIFFRawStripSize64(TIFF *t, uint32_t k) {
  return TIFFGetStrileByteCount(t, k);
}
tmsize_t TIFFRawStripSize(TIFF *t, uint32_t k) {
  int error = 0;
  uint64_t n = strile(t, k, 1, &error);
  if (error)
    return -1;
  if (n > PTRDIFF_MAX) {
    fail(t, EFBIG, __func__, "raw strip size overflow");
    return -1;
  }
  return (tmsize_t)n;
}
int TIFFCheckTile(TIFF *t, uint32_t x, uint32_t y, uint32_t z, uint16_t s) {
  if (!valid(t, __func__))
    return 0;
  if (!TIFFIsTiled(t) || x >= info(t, 0) || y >= info(t, 1) || z != 0 ||
      (info(t, 11) == 2 ? s >= info(t, 12) : s != 0)) {
    fail(t, EINVAL, __func__, "invalid tile coordinates/plane");
    return 0;
  }
  return 1;
}
uint32_t TIFFComputeTile(TIFF *t, uint32_t x, uint32_t y, uint32_t z,
                         uint16_t s) {
  if (!TIFFCheckTile(t, x, y, z, s))
    return UINT32_MAX;
  uint64_t across = (info(t, 0) - 1) / info(t, 8) + 1;
  uint64_t down = (info(t, 1) - 1) / info(t, 9) + 1;
  return (uint32_t)((y / info(t, 9)) * across + x / info(t, 8) +
                    (info(t, 11) == 2 ? s * across * down : 0));
}
uint32_t TIFFComputeStrip(TIFF *t, uint32_t row, uint16_t s) {
  if (!valid(t, __func__))
    return UINT32_MAX;
  if (TIFFIsTiled(t) || row >= info(t, 1) ||
      (info(t, 11) == 2 ? s >= info(t, 12) : s != 0)) {
    fail(t, EINVAL, __func__, "invalid strip row/plane");
    return UINT32_MAX;
  }
  uint64_t per = (info(t, 1) - 1) / info(t, 9) + 1;
  return (uint32_t)(row / info(t, 9) + (info(t, 11) == 2 ? s * per : 0));
}
static tmsize_t read_bytes(TIFF *t, unsigned kind, uint32_t a, uint32_t b,
                           void *buf, tmsize_t size) {
  if (!valid(t, "TIFF read"))
    return -1;
  if (size < -1 || (!buf && size != 0)) {
    fail(t, EINVAL, "TIFF read", "invalid destination/size");
    return -1;
  }
  if (size==0) {
    if (a>=info(t,7) || (kind==1 && TIFFIsTiled(t)) || (kind==2 && !TIFFIsTiled(t))) {
      fail(t,EINVAL,"TIFF read","invalid segment/layout"); return -1;
    }
    return 0;
  }
  size_t n = 0;
  const uint8_t *p = vt_read_size(t->state, kind, a, b, size<0?SIZE_MAX:(size_t)size, &n);
  if (!p) {
    bridge_fail(t, "TIFF read");
    return -1;
  }
  if (size >= 0 && (uint64_t)size < n)
    n = (size_t)size;
  if (n && !vt_commit(buf,size<0?n:(size_t)size,0,p,n)){bridge_fail(t,"TIFF read");return -1;}
  return (tmsize_t)n;
}
int TIFFReadScanline(TIFF *t, void *buf, uint32_t row, uint16_t sample) {
  tmsize_t n = read_bytes(t, 0, row, sample, buf, -1);
  if (n < 0)
    return -1;
  t->row = row + 1;
  t->strip = TIFFComputeStrip(t, row, sample);
  return 1;
}
tmsize_t TIFFReadEncodedStrip(TIFF *t, uint32_t k, void *b, tmsize_t n) {
  tmsize_t r = read_bytes(t, 1, k, 0, b, n);
  if (r >= 0) {
    t->strip = k;
    uint64_t per = (info(t, 1) - 1) / info(t, 9) + 1;
    t->row = (uint32_t)((k % per) * info(t, 9));
  }
  return r;
}
tmsize_t TIFFReadEncodedTile(TIFF *t, uint32_t k, void *b, tmsize_t n) {
  tmsize_t r = read_bytes(t, 2, k, 0, b, n);
  if (r >= 0) {
    t->tile = k;
    uint64_t across = (info(t, 0) - 1) / info(t, 8) + 1;
    uint64_t down = (info(t, 1) - 1) / info(t, 9) + 1;
    t->row = (uint32_t)(((k / across) % down) * info(t, 9));
  }
  return r;
}
tmsize_t TIFFReadRawStrip(TIFF *t, uint32_t k, void *b, tmsize_t n) {
  if (!valid(t, __func__) || TIFFIsTiled(t)) {
    fail(t, EINVAL, __func__, "not a striped image");
    return -1;
  }
  return read_bytes(t, 3, k, 0, b, n);
}
tmsize_t TIFFReadRawTile(TIFF *t, uint32_t k, void *b, tmsize_t n) {
  if (!valid(t, __func__) || !TIFFIsTiled(t)) {
    fail(t, EINVAL, __func__, "not a tiled image");
    return -1;
  }
  return read_bytes(t, 3, k, 0, b, n);
}
tmsize_t TIFFReadTile(TIFF *t, void *b, uint32_t x, uint32_t y, uint32_t z,
                      uint16_t s) {
  uint32_t k = TIFFComputeTile(t, x, y, z, s);
  return k == UINT32_MAX ? -1 : TIFFReadEncodedTile(t, k, b, -1);
}
static int rgba(TIFF *t, unsigned kind, uint32_t a, uint32_t b,
                uint32_t orientation, uint32_t *out) {
  if (!valid(t, "TIFF RGBA"))
    return 0;
  if (!out) {
    fail(t, EINVAL, "TIFF RGBA", "null raster");
    return 0;
  }
  size_t len = 0;
  const uint8_t *p = vt_read(t->state, kind, a, b, orientation, 0, &len);
  if (!p) {
    bridge_fail(t, "TIFF RGBA");
    return 0;
  }
  size_t skip=0;
  if(kind==4 && b>info(t,1)){
    uint64_t words=(uint64_t)(b-info(t,1))*a;
    if(words>PTRDIFF_MAX/sizeof(*out)){fail(t,EFBIG,"TIFF RGBA","raster extent overflow");return 0;}
    skip=(size_t)words;
  }
  size_t offset=skip*sizeof(*out);
  if(len>SIZE_MAX-offset||!vt_commit((uint8_t*)out,offset+len,offset,p,len)){bridge_fail(t,"TIFF RGBA");return 0;}
  return 1;
}
int TIFFReadRGBAImageOriented(TIFF *t, uint32_t w, uint32_t h, uint32_t *p,
                              int o, int stop) {
  (void)stop;
  return rgba(t, 4, w, h, (uint32_t)o, p);
}
int TIFFReadRGBAImage(TIFF *t, uint32_t w, uint32_t h, uint32_t *p, int stop) {
  return TIFFReadRGBAImageOriented(t, w, h, p, ORIENTATION_BOTLEFT, stop);
}
int TIFFReadRGBAStripExt(TIFF *t, uint32_t row, uint32_t *p, int stop) {
  (void)stop;
  return rgba(t, 5, row, 0, 4, p);
}
int TIFFReadRGBAStrip(TIFF *t, uint32_t row, uint32_t *p) {
  return TIFFReadRGBAStripExt(t, row, p, 0);
}
int TIFFReadRGBATileExt(TIFF *t, uint32_t x, uint32_t y, uint32_t *p,
                        int stop) {
  (void)stop;
  return rgba(t, 6, x, y, 4, p);
}
int TIFFReadRGBATile(TIFF *t, uint32_t x, uint32_t y, uint32_t *p) {
  return TIFFReadRGBATileExt(t, x, y, p, 0);
}
int TIFFRGBAImageOK(TIFF *t, char message[1024]) {
  if (!valid(t, __func__))
    return 0;
  if (!vt_rgba_ok(t->state)) {
    bridge_fail(t, __func__);
    if (message)
      snprintf(message, 1024, "%s", VTIFFLastError());
    return 0;
  }
  if (message)
    message[0] = 0;
  return 1;
}
int TIFFRGBAImageBegin(TIFFRGBAImage *img, TIFF *t, int stop,
                       char message[1024]) {
  if (!img) {
    fail(t, EINVAL, __func__, "null RGBA state");
    return 0;
  }
  memset(img, 0, sizeof(*img));
  if (!TIFFRGBAImageOK(t, message))
    return 0;
  img->tif = t;
  img->width = (uint32_t)info(t, 0);
  img->height = (uint32_t)info(t, 1);
  img->stoponerr = stop;
  TIFFGetFieldDefaulted(t, TIFFTAG_ORIENTATION, &img->orientation);
  img->req_orientation = 4;
  img->isContig = info(t, 11) == 1;
  img->get = TIFFRGBAImageGet;
  TIFFGetFieldDefaulted(t, 258, &img->bitspersample);
  TIFFGetFieldDefaulted(t, 277, &img->samplesperpixel);
  TIFFGetField(t, 262, &img->photometric);
  uint16_t count = 0, *extra = NULL;
  if (TIFFGetField(t, TIFFTAG_EXTRASAMPLES, &count, &extra) && count)
    img->alpha = extra[0];
  if (img->photometric == PHOTOMETRIC_PALETTE)
    TIFFGetField(t, TIFFTAG_COLORMAP, &img->redcmap, &img->greencmap,
                 &img->bluecmap);
  return 1;
}
int TIFFRGBAImageGet(TIFFRGBAImage *img, uint32_t *out, uint32_t w,
                     uint32_t h) {
  if (!img) {
    fail(NULL, EINVAL, __func__, "null RGBA state");
    return 0;
  }
  if (img->put.any || img->get != TIFFRGBAImageGet) {
    vt_not_implemented_for(img->tif, "custom TIFFRGBAImage callbacks");
    return 0;
  }
  if (!valid(img->tif,__func__) || !out || img->row_offset<0 || img->col_offset<0 || (uint32_t)img->row_offset>=img->height) {
    fail(img->tif,EINVAL,__func__,"invalid RGBA region/output"); return 0;
  }
  uint32_t available=img->height-(uint32_t)img->row_offset;
  uint32_t rows=h<available?h:available;
  uint64_t skip=(uint64_t)(h-rows)*w;
  if(skip>PTRDIFF_MAX/sizeof(*out)){fail(img->tif,EFBIG,__func__,"RGBA raster overflow");return 0;}
  size_t n=0; const uint8_t *p=vt_region(img->tif->state,(uint32_t)img->col_offset,
      (uint32_t)img->row_offset,w,rows,img->req_orientation,&n);
  if (!p) {bridge_fail(img->tif,__func__); return 0;}
  size_t offset=(size_t)skip*sizeof(*out);
  if(n>SIZE_MAX-offset||!vt_commit((uint8_t*)out,offset+n,offset,p,n)){bridge_fail(img->tif,__func__);return 0;}return 1;
}
void TIFFRGBAImageEnd(TIFFRGBAImage *img) {
  if (img)
    memset(img, 0, sizeof(*img));
}
/* Standard allocation/byte utilities obey their usual C pointer contracts. */
void *_TIFFmalloc(tmsize_t n) {
  if (n < 0) {
    fail(NULL, EINVAL, __func__, "negative size");
    return NULL;
  }
  void *p = malloc(n ? (size_t)n : 1);
  if (!p)
    fail(NULL, ENOMEM, __func__, "allocation failed");
  return p;
}
void *_TIFFcalloc(tmsize_t n, tmsize_t s) {
  if (n < 0 || s < 0 || (s && n > PTRDIFF_MAX / s)) {
    fail(NULL, EINVAL, __func__, "invalid/overflowing size");
    return NULL;
  }
  size_t bytes=(size_t)n*(size_t)s;
  void *p = calloc(1,bytes ? bytes : 1);
  if (!p)
    fail(NULL, ENOMEM, __func__, "allocation failed");
  return p;
}
void *_TIFFrealloc(void *p, tmsize_t n) {
  if (n < 0) {
    fail(NULL, EINVAL, __func__, "negative size");
    return NULL;
  }
  void *q = realloc(p, n ? (size_t)n : 1);
  if (!q)
    fail(NULL, ENOMEM, __func__, "allocation failed");
  return q;
}
void _TIFFfree(void *p) { free(p); }
void _TIFFmemset(void *p, int v, tmsize_t n) {
  if (n < 0 || (!p && n)) {
    fail(NULL, EINVAL, __func__, "invalid buffer/length");
    return;
  }
  if (n)
    memset(p, v, (size_t)n);
}
void _TIFFmemcpy(void *d, const void *s, tmsize_t n) {
  if (n < 0 || ((!d || !s) && n)) {
    fail(NULL, EINVAL, __func__, "invalid buffer/length");
    return;
  }
  if (n)
    memcpy(d, s, (size_t)n);
}
int _TIFFmemcmp(const void *a, const void *b, tmsize_t n) {
  if (n < 0 || ((!a || !b) && n)) {
    fail(NULL, EINVAL, __func__, "invalid buffer/length");
    return 0;
  }
  return n ? memcmp(a, b, (size_t)n) : 0;
}
static void swap(void *ptr, tmsize_t n, size_t width) {
  if (n < 0 || (!ptr && n) || (uint64_t)n > PTRDIFF_MAX / width) {
    fail(NULL, EINVAL, "TIFFSwab", "invalid buffer/count");
    return;
  }
  uint8_t *p = ptr;
  for (tmsize_t i = 0; i < n; i++, p += width)
    for (size_t a = 0; a < width / 2; a++) {
      uint8_t v = p[a];
      p[a] = p[width - 1 - a];
      p[width - 1 - a] = v;
    }
}
#define SWAB(Name, Type, Width)                                                \
  void TIFFSwab##Name(Type *p) { swap(p, 1, Width); }                          \
  void TIFFSwabArrayOf##Name(Type *p, tmsize_t n) { swap(p, n, Width); }
SWAB(Short, uint16_t, 2)
SWAB(Long, uint32_t, 4)
SWAB(Long8, uint64_t, 8) SWAB(Float, float, 4)
    SWAB(Double, double, 8) void TIFFSwabArrayOfTriples(uint8_t *p,
                                                        tmsize_t n) {
  swap(p, n, 3);
}
static uint8_t reverse_byte(uint8_t v) {
  v = (uint8_t)((v >> 4) | (v << 4));
  v = (uint8_t)(((v & 0xcc) >> 2) | ((v & 0x33) << 2));
  return (uint8_t)(((v & 0xaa) >> 1) | ((v & 0x55) << 1));
}
void TIFFReverseBits(uint8_t *p, tmsize_t n) {
  if (n < 0 || (!p && n)) {
    fail(NULL, EINVAL, __func__, "invalid buffer/count");
    return;
  }
  for (tmsize_t i = 0; i < n; i++)
    p[i] = reverse_byte(p[i]);
}
static const unsigned char identity_bits[256] = {
    0,   1,   2,   3,   4,   5,   6,   7,   8,   9,   10,  11,  12,  13,  14,
    15,  16,  17,  18,  19,  20,  21,  22,  23,  24,  25,  26,  27,  28,  29,
    30,  31,  32,  33,  34,  35,  36,  37,  38,  39,  40,  41,  42,  43,  44,
    45,  46,  47,  48,  49,  50,  51,  52,  53,  54,  55,  56,  57,  58,  59,
    60,  61,  62,  63,  64,  65,  66,  67,  68,  69,  70,  71,  72,  73,  74,
    75,  76,  77,  78,  79,  80,  81,  82,  83,  84,  85,  86,  87,  88,  89,
    90,  91,  92,  93,  94,  95,  96,  97,  98,  99,  100, 101, 102, 103, 104,
    105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119,
    120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134,
    135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149,
    150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164,
    165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179,
    180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194,
    195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209,
    210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224,
    225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239,
    240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254,
    255};
static const unsigned char reversed_bits[256] = {
    0,  128, 64, 192, 32, 160, 96,  224, 16, 144, 80, 208, 48, 176, 112, 240,
    8,  136, 72, 200, 40, 168, 104, 232, 24, 152, 88, 216, 56, 184, 120, 248,
    4,  132, 68, 196, 36, 164, 100, 228, 20, 148, 84, 212, 52, 180, 116, 244,
    12, 140, 76, 204, 44, 172, 108, 236, 28, 156, 92, 220, 60, 188, 124, 252,
    2,  130, 66, 194, 34, 162, 98,  226, 18, 146, 82, 210, 50, 178, 114, 242,
    10, 138, 74, 202, 42, 170, 106, 234, 26, 154, 90, 218, 58, 186, 122, 250,
    6,  134, 70, 198, 38, 166, 102, 230, 22, 150, 86, 214, 54, 182, 118, 246,
    14, 142, 78, 206, 46, 174, 110, 238, 30, 158, 94, 222, 62, 190, 126, 254,
    1,  129, 65, 193, 33, 161, 97,  225, 17, 145, 81, 209, 49, 177, 113, 241,
    9,  137, 73, 201, 41, 169, 105, 233, 25, 153, 89, 217, 57, 185, 121, 249,
    5,  133, 69, 197, 37, 165, 101, 229, 21, 149, 85, 213, 53, 181, 117, 245,
    13, 141, 77, 205, 45, 173, 109, 237, 29, 157, 93, 221, 61, 189, 125, 253,
    3,  131, 67, 195, 35, 163, 99,  227, 19, 147, 83, 211, 51, 179, 115, 243,
    11, 139, 75, 203, 43, 171, 107, 235, 27, 155, 91, 219, 59, 187, 123, 251,
    7,  135, 71, 199, 39, 167, 103, 231, 23, 151, 87, 215, 55, 183, 119, 247,
    15, 143, 79, 207, 47, 175, 111, 239, 31, 159, 95, 223, 63, 191, 127, 255};
const unsigned char *TIFFGetBitRevTable(int reversed) {
  return reversed ? reversed_bits : identity_bits;
}
int TIFFDataWidth(TIFFDataType type) {
  switch (type) {
  case TIFF_BYTE:
  case TIFF_ASCII:
  case TIFF_SBYTE:
  case TIFF_UNDEFINED:
    return 1;
  case TIFF_SHORT:
  case TIFF_SSHORT:
    return 2;
  case TIFF_LONG:
  case TIFF_SLONG:
  case TIFF_FLOAT:
  case TIFF_IFD:
    return 4;
  case TIFF_RATIONAL:
  case TIFF_SRATIONAL:
  case TIFF_DOUBLE:
  case TIFF_LONG8:
  case TIFF_SLONG8:
  case TIFF_IFD8:
    return 8;
  default:
    return 0;
  }
}
int TIFFIsCODECConfigured(uint16_t c) {
  switch (c) {
  case 2:
  case 1:
  case 3:
  case 4:
  case 5:
  case 6:
  case 7:
  case 8:
  case 32946:
  case 32773:
  case 34676:
  case 34887:
  case 50000:
  case 50001:
    return 1;
  default:
    return 0;
  }
}

#include "fields.inc"

#include "writer.inc"

#include "reader_api.inc"
#include "color_api.inc"

#include "logluv_api.inc"
