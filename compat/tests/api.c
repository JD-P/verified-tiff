#define _POSIX_C_SOURCE 200809L
#include <assert.h>
#include <errno.h>
#include <fcntl.h>
#include <pthread.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <tiffio.h>
#include <unistd.h>
#include <verified_tiff.h>
static unsigned errors;
static void handler(const char *m, const char *fmt, va_list ap) {
  char text[512];
  (void)m;
  vsnprintf(text, sizeof(text), fmt, ap);
  assert(text[0]);
  errors++;
}
static int local_errors;
static int local_handler(TIFF *t, void *user, const char *m, const char *fmt,
                         va_list ap) {
  (void)t;
  (void)m;
  (void)fmt;
  (void)ap;
  (*(int *)user)++;
  return 1;
}
struct memory {
  unsigned char *data;
  size_t len, at;
  int closed;
};
static tmsize_t rd(thandle_t c, void *p, tmsize_t n) {
  struct memory *m = c;
  if (n > 7)
    n = 7;
  if ((size_t)n > m->len - m->at)
    n = (tmsize_t)(m->len - m->at);
  memcpy(p, m->data + m->at, (size_t)n);
  m->at += (size_t)n;
  return n;
}
static toff_t seek_mem(thandle_t c, toff_t n, int whence) {
  struct memory *m = c;
  if (whence != SEEK_SET || n > m->len)
    return (toff_t)-1;
  m->at = (size_t)n;
  return n;
}
static toff_t size_mem(thandle_t c) { return ((struct memory *)c)->len; }
static int close_mem(thandle_t c) {
  ((struct memory *)c)->closed++;
  return 0;
}
static int vget(TIFF *t, uint32_t tag, ...) {
  va_list ap;
  va_start(ap, tag);
  int ok = TIFFVGetField(t, tag, ap);
  va_end(ap);
  return ok;
}
static int vdef(TIFF *t, uint32_t tag, ...) {
  va_list ap;
  va_start(ap, tag);
  int ok = TIFFVGetFieldDefaulted(t, tag, ap);
  va_end(ap);
  return ok;
}
static void *thread_test(void *unused) {
  (void)unused;
  VTIFFClearError();
  assert(VTIFFLastErrorCode() == 0);
  TIFFSetErrorHandler(NULL);
  assert(!TIFFWriteCustomDirectory(NULL,NULL));
  assert(VTIFFLastErrorCode() == ENOSYS);
  return NULL;
}
int main(int argc, char **argv) {
  assert(argc == 6);
  const char *gray = argv[1], *pages = argv[2], *tile = argv[3],
             *alpha = argv[4], *logluv = argv[5];
  TIFFSetErrorHandler(handler);
  TIFF *t = TIFFOpen(gray, "r");
  assert(t);
  uint32_t w = 0, h = 0;
  uint16_t ch = 0, bits = 0;
  assert(vget(t, TIFFTAG_IMAGEWIDTH, &w));
  assert(TIFFGetField(t, TIFFTAG_IMAGELENGTH, &h));
  assert(vdef(t, TIFFTAG_SAMPLESPERPIXEL, &ch));
  assert(TIFFGetFieldDefaulted(t, TIFFTAG_BITSPERSAMPLE, &bits));
  assert(w && h && ch == 1 && bits == 8);
  uint16_t resolution = 0;
  assert(TIFFGetFieldDefaulted(t, TIFFTAG_RESOLUTIONUNIT, &resolution) &&
         resolution == RESUNIT_INCH);
  uint32_t options = 9;
  assert(TIFFGetFieldDefaulted(t, TIFFTAG_GROUP3OPTIONS, &options) &&
         options == 0);
  uint32_t sentinel = 0x12345678;
  VTIFFClearError();
  assert(!TIFFGetField(t, 65535, &sentinel));
  assert(sentinel == 0x12345678 && errno == 0);
  tmsize_t n = TIFFScanlineSize(t);
  assert(n == (tmsize_t)w);
  uint8_t *buf = malloc((size_t)n + 16);
  assert(buf);
  memset(buf, 0xa5, (size_t)n + 16);
  assert(TIFFReadScanline(t, buf, 0, 0) == 1);
  for (size_t i = (size_t)n; i < (size_t)n + 16; i++)
    assert(buf[i] == 0xa5);
  assert(TIFFCurrentRow(t) == 1 && TIFFCurrentStrip(t) == 0);
  memset(buf, 0xa5, (size_t)n + 16);
  assert(TIFFReadScanline(t, buf, h, 0) == -1);
  for (size_t i = 0; i < (size_t)n + 16; i++)
    assert(buf[i] == 0xa5);
  assert(TIFFReadEncodedStrip(t, 0, buf, -2) == -1);
  assert(TIFFReadEncodedStrip(t, UINT32_MAX, NULL, 0) == -1);
  assert(TIFFReadEncodedTile(t, 0, NULL, 0) == -1);
  assert(TIFFReadEncodedStrip(t, 0, NULL, 0) == 0);
  assert(TIFFReadEncodedStrip(t, 0, buf, 1) == 1);
  for (size_t i = 1; i < (size_t)n + 16; i++)
    assert(buf[i] == 0xa5);
  free(buf);
  uint64_t *offset = NULL, *counts = NULL;
  assert(TIFFGetField(t, TIFFTAG_STRIPOFFSETS, &offset));
  assert(TIFFGetField(t, TIFFTAG_STRIPBYTECOUNTS, &counts));
  assert(TIFFRawStripSize(t, UINT32_MAX) == -1);
  assert(offset[0] == TIFFGetStrileOffset(t, 0));
  assert(counts[0] == TIFFRawStripSize64(t, 0));
  int e = 0;
  assert(!TIFFGetStrileOffsetWithErr(t, UINT32_MAX, &e) && e);
  tdir_t old = TIFFCurrentDirectory(t);
  uint64_t off = TIFFCurrentDirOffset(t);
  assert(!TIFFSetDirectory(t, 1024));
  assert(TIFFCurrentDirectory(t) == old && TIFFCurrentDirOffset(t) == off);
  assert(TIFFSetSubDirectory(t, off));
  uint32_t unchanged = 0xdeadbeef;
  assert(!TIFFReadRGBAImage(t, w + 1, h, &unchanged, 1));
  assert(unchanged == 0xdeadbeef);
  assert(!TIFFSetField(t, TIFFTAG_IMAGEWIDTH, 7));
  assert(errno == ENOSYS);
  assert(!TIFFWriteDirectory(t));
  assert(errno == ENOSYS);
  char rgba_message[1024];
  TIFFRGBAImage image;
  assert(TIFFRGBAImageBegin(&image, t, 1, rgba_message));
  assert(image.width == w && image.height == h && image.bitspersample == 8);
  uint32_t *raster = malloc((size_t)w * h * 4);
  assert(raster);
  assert(TIFFRGBAImageGet(&image, raster, w, h));
  image.row_offset = 1;
  assert(TIFFRGBAImageGet(&image, raster, w, h-1));
  memset(raster,0xa5,(size_t)w*h*4);
  assert(TIFFRGBAImageGet(&image, raster, w, h));
  for(uint32_t x=0;x<w;x++)assert(raster[x]==0xa5a5a5a5);
  image.row_offset=(int)h;
  assert(!TIFFRGBAImageGet(&image, raster, w, h) && errno == EINVAL);
  TIFFRGBAImageEnd(&image);
  assert(!image.tif);
  free(raster);
  int fd = TIFFFileno(t);
  assert(fd >= 0);
  TIFFCleanup(t);
  assert(fcntl(fd, F_GETFD) != -1);
  assert(close(fd) == 0);
  t = TIFFOpen(pages, "r");
  assert(t);
  assert(TIFFNumberOfDirectories(t) == 2);
  assert(!TIFFLastDirectory(t));
  assert(TIFFReadDirectory(t));
  assert(TIFFCurrentDirectory(t) == 1 && TIFFLastDirectory(t));
  assert(!TIFFReadDirectory(t));
  assert(TIFFSetDirectory(t, 0));
  TIFFClose(t);
  t = TIFFOpen(tile, "r");
  assert(t && TIFFIsTiled(t));
  assert(TIFFCheckTile(t, 0, 0, 0, 0));
  assert(TIFFComputeTile(t, 0, 0, 0, 0) == 0);
  assert(!TIFFCheckTile(t, 0, 0, 1, 0));
  size_t tn = (size_t)TIFFTileSize(t);
  buf = malloc(tn + 16);
  assert(buf);
  memset(buf, 0xa5, tn + 16);
  uint32_t last = TIFFNumberOfTiles(t) - 1;
  assert(TIFFReadEncodedTile(t, last, buf, (tmsize_t)tn) == (tmsize_t)tn);
  for (size_t i = tn; i < tn + 16; i++)
    assert(buf[i] == 0xa5);
  assert(TIFFReadEncodedTile(t, 1, buf, (tmsize_t)tn) == (tmsize_t)tn);
  assert(TIFFCurrentRow(t) == 0);
  free(buf);
  TIFFClose(t);
  t = TIFFOpen(alpha, "r");
  assert(t);
  buf = malloc((size_t)TIFFTileSize(t));
  assert(buf);
  assert(TIFFReadEncodedTile(t, 0, buf, -1) == TIFFTileSize(t));
  char msg[1024];
  assert(TIFFRGBAImageOK(t, msg) && !msg[0]);
  free(buf);
  TIFFClose(t);
  t = TIFFOpen(logluv, "r");
  assert(t);
  unsigned char luv[6] = {0};
  float xyz[3]={1,1,1};
  assert(TIFFScanlineSize(t)==12 && TIFFReadEncodedStrip(t,0,xyz,12)==12);
  assert(xyz[0]==0 && xyz[1]==0 && xyz[2]==0);
  assert(TIFFSetField(t, TIFFTAG_SGILOGDATAFMT, SGILOGDATAFMT_16BIT));
  int datafmt = -1;
  assert(TIFFGetField(t, TIFFTAG_SGILOGDATAFMT, &datafmt) &&
         datafmt == SGILOGDATAFMT_16BIT);
  assert(TIFFReadEncodedStrip(t, 0, luv, 6) == 6);
  int16_t comps[3];
  memcpy(comps, luv, 6);
  assert(comps[0] == 0 && comps[1] == 6913 && comps[2] == 15544);
  TIFFClose(t);
  assert(!TIFFOpen(gray, "w") && errno == ENOSYS);
  assert(!TIFFOpen(gray, "r+") && errno == ENOSYS);
  TIFFOpenOptions *opt = TIFFOpenOptionsAlloc();
  assert(opt);
  TIFFOpenOptionsSetErrorHandlerExtR(opt, local_handler, &local_errors);
  assert(VTIFFOpenOptionsSetLimits(opt, 1, 1024, 100000));
  assert(!TIFFOpenExt(gray, "r", opt));
  assert(local_errors == 1 && errno == EFBIG);
  TIFFOpenOptionsFree(opt);
  opt = TIFFOpenOptionsAlloc();
  TIFFOpenOptionsSetErrorHandlerExtR(opt, local_handler, &local_errors);
  t = TIFFOpenExt(gray, "r", opt);
  assert(t);
  int before = local_errors;
  assert(!TIFFWriteDirectory(t));
  assert(local_errors == before + 1);
  TIFFClose(t);
  TIFFOpenOptionsFree(opt);
  opt = TIFFOpenOptionsAlloc();
  TIFFOpenOptionsSetMaxSingleMemAlloc(opt, 100);
  assert(!TIFFOpenExt(gray, "r", opt) && errno == EFBIG);
  TIFFOpenOptionsFree(opt);
  FILE *file = fopen(gray, "rb");
  assert(file);
  assert(!fseek(file, 0, SEEK_END));
  long len = ftell(file);
  assert(len > 0);
  rewind(file);
  struct memory mem = {malloc((size_t)len), (size_t)len, 0, 0};
  assert(mem.data);
  assert(fread(mem.data, 1, mem.len, file) == mem.len);
  fclose(file);
  t = TIFFClientOpen("memory", "r", &mem, rd, NULL, seek_mem, close_mem,
                     size_mem, NULL, NULL);
  assert(t);
  assert(TIFFClientdata(t) == &mem && TIFFGetReadProc(t) == rd &&
         TIFFGetSeekProc(t) == seek_mem);
  TIFFClose(t);
  assert(mem.closed == 1);
  free(mem.data);
  fd = open(gray, O_RDONLY);
  assert(fd >= 0);
  t = TIFFFdOpen(fd, "fd", "r");
  assert(t);
  TIFFClose(t);
  assert(fcntl(fd, F_GETFD) == -1 && errno == EBADF);
  uint16_t sw = 0x1234;
  TIFFSwabShort(&sw);
  assert(sw == 0x3412);
  uint8_t br = 1;
  TIFFReverseBits(&br, 1);
  assert(br == 128 && TIFFGetBitRevTable(1)[1] == 128);
  assert(TIFFDataWidth(TIFF_DOUBLE) == 8);
  assert(TIFFIsCODECConfigured(COMPRESSION_LZW));
  assert(TIFFIsCODECConfigured(COMPRESSION_ZSTD));
  assert(TIFFIsCODECConfigured(COMPRESSION_LERC));
  assert(!_TIFFcalloc(PTRDIFF_MAX, 2));
  TIFFSetErrorHandler(NULL);
  VTIFFClearError();
  pthread_t thread;
  assert(pthread_create(&thread, NULL, thread_test, NULL) == 0);
  assert(pthread_join(thread, NULL) == 0);
  assert(VTIFFLastErrorCode() == 0);
  assert(errors > 5);
  puts("C API: metadata/varargs, directory rollback, buffer frames, "
       "fd/callback ownership, limits, stubs, TLS and utilities passed");
  return 0;
}
