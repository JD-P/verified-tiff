/* Same C consumer for real libtiff and the compatibility adapter. Timing is
 * in-process; disk I/O, correctness hashing and output dumping are excluded. */
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
struct memory {
  uint8_t *data;
  size_t length, position;
};
static tmsize_t rd(thandle_t p, void *out, tmsize_t count) {
  struct memory *m = p;
  if (count < 0)
    return -1;
  size_t n = (size_t)count;
  if (n > m->length - m->position)
    n = m->length - m->position;
  memcpy(out, m->data + m->position, n);
  m->position += n;
  return (tmsize_t)n;
}
static tmsize_t wr(thandle_t p, void *data, tmsize_t n) {
  (void)p;
  (void)data;
  (void)n;
  return -1;
}
static toff_t sk(thandle_t p, toff_t off, int whence) {
  struct memory *m = p;
  uint64_t base = whence == SEEK_SET   ? 0
                  : whence == SEEK_CUR ? m->position
                  : whence == SEEK_END ? m->length
                                       : UINT64_MAX;
  if (base == UINT64_MAX || off > m->length || base > m->length - off)
    return (toff_t)-1;
  m->position = base + off;
  return m->position;
}
static toff_t sz(thandle_t p) { return ((struct memory *)p)->length; }
static int cl(thandle_t p) {
  (void)p;
  return 0;
}
static TIFF *open_memory(struct memory *m) {
  m->position = 0;
#ifdef VERIFIED_ADAPTER
  TIFFOpenOptions *o = TIFFOpenOptionsAlloc();
  assert(o);
  assert(VTIFFOpenOptionsSetLimits(o, 268435456, 268435456,
                                   UINT64_C(1000000000000)));
  TIFF *t = TIFFClientOpenExt("benchmark", "rm", m, rd, wr, sk, cl, sz, NULL,
                              NULL, o);
  TIFFOpenOptionsFree(o);
  return t;
#else
  return TIFFClientOpen("benchmark", "rm", m, rd, wr, sk, cl, sz, NULL, NULL);
#endif
}
static double now(void) {
  struct timespec ts;
  assert(!clock_gettime(CLOCK_MONOTONIC, &ts));
  return ts.tv_sec + ts.tv_nsec * 1e-9;
}
static uint64_t hash(const uint8_t *p, size_t n) {
  uint64_t v = UINT64_C(14695981039346656037);
  for (size_t i = 0; i < n; i++)
    v = (v ^ p[i]) * UINT64_C(1099511628211);
  return v;
}
struct image {
  uint32_t w, h, bw, bh;
  uint16_t ch, bits, photo, format;
  int tiled;
  size_t row, bytes, block_bytes;
  uint8_t *out, *block;
};
static void setup(TIFF *t, struct image *im, int rgba) {
  assert(TIFFGetField(t, TIFFTAG_IMAGEWIDTH, &im->w) &&
         TIFFGetField(t, TIFFTAG_IMAGELENGTH, &im->h));
  TIFFGetFieldDefaulted(t, TIFFTAG_BITSPERSAMPLE, &im->bits);
  TIFFGetFieldDefaulted(t, TIFFTAG_SAMPLESPERPIXEL, &im->ch);
  TIFFGetField(t, TIFFTAG_PHOTOMETRIC, &im->photo);
  TIFFGetFieldDefaulted(t, TIFFTAG_SAMPLEFORMAT, &im->format);
  uint16_t planar = 0;
  TIFFGetFieldDefaulted(t, TIFFTAG_PLANARCONFIG, &planar);
  assert(planar == PLANARCONFIG_CONTIG);
  if (rgba) {
    im->bytes = (size_t)im->w * im->h * 4;
    return;
  }
  if (im->photo == PHOTOMETRIC_YCBCR)
    assert(TIFFSetField(t, TIFFTAG_JPEGCOLORMODE, JPEGCOLORMODE_RGB));
  if (im->photo == PHOTOMETRIC_LOGLUV)
    assert(TIFFSetField(t, TIFFTAG_SGILOGDATAFMT, SGILOGDATAFMT_16BIT));
  assert(im->bits == 1 || im->bits == 8 || im->bits == 16 || im->bits == 64);
  assert(im->photo != PHOTOMETRIC_PALETTE);
  im->tiled = TIFFIsTiled(t);
  if (im->tiled) {
    assert(TIFFGetField(t, TIFFTAG_TILEWIDTH, &im->bw));
    assert(TIFFGetField(t, TIFFTAG_TILELENGTH, &im->bh));
  } else {
    im->bw = im->w;
    TIFFGetFieldDefaulted(t, TIFFTAG_ROWSPERSTRIP, &im->bh);
    if (im->bh > im->h)
      im->bh = im->h;
  }
  im->row = (size_t)im->w * im->ch * (im->bits == 1 ? 1 : im->bits / 8);
  im->bytes = im->row * im->h;
  im->block_bytes = (size_t)(im->tiled ? TIFFTileSize(t) : TIFFStripSize(t));
  assert(im->block_bytes && im->bytes <= 268435456);
}
static void native(TIFF *t, struct image *im) {
  unsigned bytes = im->bits == 1 ? 1 : im->bits / 8;
  size_t stride = ((size_t)im->bw * im->ch * im->bits + 7) / 8;
  for (uint32_t y = 0; y < im->h; y += im->bh)
    for (uint32_t x = 0; x < im->w; x += im->bw) {
      uint32_t rows = im->h - y < im->bh ? im->h - y : im->bh,
               cols = im->w - x < im->bw ? im->w - x : im->bw;
      int direct = !im->tiled && im->bits >= 8;
      uint8_t *block = direct ? im->out + y * im->row : im->block;
      uint32_t index = im->tiled ? TIFFComputeTile(t, x, y, 0, 0)
                                 : TIFFComputeStrip(t, y, 0);
      tmsize_t cap = (tmsize_t)(im->tiled ? im->block_bytes : stride * rows);
      tmsize_t n = im->tiled ? TIFFReadEncodedTile(t, index, block, cap)
                             : TIFFReadEncodedStrip(t, index, block, cap);
      assert(n == cap);
      if (im->bits == 1) {
        for (uint32_t yy = 0; yy < rows; yy++)
          for (uint32_t xx = 0; xx < cols; xx++) {
            unsigned bit = (block[yy * stride + xx / 8] >> (7 - xx % 8)) & 1;
            im->out[(y + yy) * im->row + x + xx] =
                (uint8_t)(255 * (im->photo == PHOTOMETRIC_MINISWHITE ? 1 - bit
                                                                     : bit));
          }
      } else if (!direct) {
        for (uint32_t yy = 0; yy < rows; yy++)
          memcpy(im->out + (y + yy) * im->row + x * im->ch * bytes,
                 block + yy * stride, (size_t)cols * im->ch * bytes);
      }
    }
  // Canonical representation matches the Rust public raster, not TIFF storage.
  if (im->bits == 16 && im->format == SAMPLEFORMAT_UINT) {
    for (size_t i = 0; i < im->bytes; i += 2) {
      uint16_t v;
      memcpy(&v, im->out + i, 2);
      if (im->photo == PHOTOMETRIC_MINISWHITE)
        v = (uint16_t)~v;
      im->out[i] = (uint8_t)(v >> 8);
      im->out[i + 1] = (uint8_t)v;
    }
  } else if (im->bits == 8 && im->photo == PHOTOMETRIC_MINISWHITE) {
    for (size_t i = 0; i < im->bytes; i++)
      im->out[i] = (uint8_t)~im->out[i];
  }
}
static void run(struct memory *m, struct image *im, TIFF *prepared,
                const char *mode) {
  if (!strcmp(mode, "decode")) {
    native(prepared, im);
    return;
  }
  TIFF *t = open_memory(m);
  assert(t);
  if (!strcmp(mode, "rgba")) {
    assert(TIFFReadRGBAImageOriented(t, im->w, im->h, (uint32_t *)im->out,
                                     ORIENTATION_TOPLEFT, 1));
  } else {
    struct image info = *im;
    setup(t, &info, 0);
    assert(info.bytes == im->bytes);
    native(t, im);
  }
  TIFFClose(t);
}
int main(int argc, char **argv) {
  if (argc == 2 && !strcmp(argv[1], "--version")) {
    puts(TIFFGetVersion());
    return 0;
  }
  assert(argc == 4 || argc == 5);
  FILE *f = fopen(argv[1], "rb");
  assert(f);
  assert(!fseek(f, 0, SEEK_END));
  long size = ftell(f);
  assert(size > 0 && size <= 268435456);
  rewind(f);
  struct memory m = {malloc((size_t)size), (size_t)size, 0};
  assert(m.data);
  assert(fread(m.data, 1, m.length, f) == m.length);
  fclose(f);
  TIFFSetWarningHandler(NULL);
  struct image im = {0};
  TIFF *t = open_memory(&m);
  assert(t);
  setup(t, &im, !strcmp(argv[2], "rgba"));
  im.out = calloc(1, im.bytes);
  im.block = calloc(1, im.block_bytes ? im.block_bytes : 1);
  assert(im.out && im.block);
  run(&m, &im, t, argv[2]);
  if (argc == 5) {
    f = fopen(argv[4], "wb");
    assert(f);
    assert(fwrite(im.out, 1, im.bytes, f) == im.bytes);
    assert(!fclose(f));
  }
  double start = now();
  run(&m, &im, t, argv[2]);
  double one = now() - start;
  uint64_t iterations =
      (uint64_t)(atof(argv[3]) / 1000.0 / (one > 1e-9 ? one : 1e-9)) + 1;
  if (iterations > 1000000)
    iterations = 1000000;
  start = now();
  uint64_t completed = 0;
  double elapsed;
  do {
    for (uint64_t i = 0; i < iterations; i++)
      run(&m, &im, t, argv[2]);
    completed += iterations;
    elapsed = now() - start;
  } while (elapsed < atof(argv[3]) / 1000.0);
  iterations = completed;
  printf(
      "{\"mode\":\"%s\",\"iterations\":%llu,\"seconds\":%.9g,\"ns_per_image\":%"
      ".9g,\"width\":%u,\"height\":%u,\"bytes\":%zu,\"hash\":\"%016llx\"}\n",
      argv[2], (unsigned long long)iterations, elapsed,
      elapsed * 1e9 / iterations, im.w, im.h, im.bytes,
      (unsigned long long)hash(im.out, im.bytes));
  TIFFClose(t);
  free(im.out);
  free(im.block);
  free(m.data);
  return 0;
}
