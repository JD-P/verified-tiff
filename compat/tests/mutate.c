/* Bounded deterministic mutations through the public C callback interface.
 * This exercises adapter ownership, error paths, native reads and RGBA copies;
 * it is not coverage-guided fuzzing or a substitute for core verification. */
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <tiffio.h>
#include <verified_tiff.h>
struct memory {
  uint8_t *bytes;
  size_t size, at;
  unsigned closed;
};
static tmsize_t read_mem(thandle_t p, void *out, tmsize_t n) {
  struct memory *m = p;
  if (n < 0)
    return -1;
  size_t take = (size_t)n;
  if (take > m->size - m->at)
    take = m->size - m->at;
  memcpy(out, m->bytes + m->at, take);
  m->at += take;
  return (tmsize_t)take;
}
static toff_t seek_mem(thandle_t p, toff_t n, int origin) {
  struct memory *m = p;
  if (origin != SEEK_SET || n > m->size)
    return (toff_t)-1;
  m->at = (size_t)n;
  return n;
}
static toff_t size_mem(thandle_t p) { return ((struct memory *)p)->size; }
static int close_mem(thandle_t p) {
  ((struct memory *)p)->closed++;
  return 0;
}
static uint64_t state = UINT64_C(0x4341504954494646);
static uint64_t random_word(void) {
  state ^= state << 13;
  state ^= state >> 7;
  state ^= state << 17;
  return state;
}
int main(int argc, char **argv) {
  assert(argc > 1);
  TIFFSetErrorHandler(NULL);
  TIFFSetWarningHandler(NULL);
  TIFFOpenOptions *options = TIFFOpenOptionsAlloc();
  assert(options);
  const char *work_env = getenv("VTIFF_FUZZ_WORK");
  size_t work = work_env ? (size_t)strtoull(work_env, NULL, 10) : 2000000;
  assert(VTIFFOpenOptionsSetLimits(options, 1 << 20, 1 << 20, work));
  unsigned accepted = 0, decoded = 0;
  const unsigned iterations = 2000;
  for (unsigned i = 0; i < iterations; i++) {
    const char *path = argv[1 + i % (unsigned)(argc - 1)];
    FILE *f = fopen(path, "rb");
    assert(f);
    assert(!fseek(f, 0, SEEK_END));
    long length = ftell(f);
    assert(length > 0 && length < 1 << 20);
    rewind(f);
    uint8_t *data = malloc((size_t)length);
    assert(data);
    assert(fread(data, 1, (size_t)length, f) == (size_t)length);
    fclose(f);
    size_t size = (size_t)length;
    switch (i % 5) {
    case 0:
      break; /* Reach decoding regularly, not just early rejection. */
    case 1:
      size = (size_t)(random_word() % size);
      break;
    case 2:
      data[random_word() % size] ^= (uint8_t)(1u << (random_word() % 8));
      break;
    case 3:
      for (unsigned j = 0; j < 4; j++)
        data[random_word() % size] = (uint8_t)random_word();
      break;
    case 4:
      if (size >= 4) {
        size_t at = random_word() % (size - 3);
        memset(data + at, 0xff, 4);
      }
      break;
    }
    struct memory memory = {data, size, 0, 0};
    TIFF *t = TIFFClientOpenExt(path, "r", &memory, read_mem, NULL, seek_mem,
                                close_mem, size_mem, NULL, NULL, options);
    if (t) {
      accepted++;
      uint32_t w = 0, h = 0;
      uint16_t photo = 0;
      assert(TIFFGetField(t, TIFFTAG_IMAGEWIDTH, &w));
      assert(TIFFGetField(t, TIFFTAG_IMAGELENGTH, &h));
      TIFFGetField(t, TIFFTAG_PHOTOMETRIC, &photo);
      if (photo == PHOTOMETRIC_LOGLUV)
        TIFFSetField(t, TIFFTAG_SGILOGDATAFMT, SGILOGDATAFMT_16BIT);
      if (photo == PHOTOMETRIC_YCBCR)
        TIFFSetField(t, TIFFTAG_JPEGCOLORMODE, JPEGCOLORMODE_RGB);
      tmsize_t n = TIFFIsTiled(t) ? TIFFTileSize(t) : TIFFScanlineSize(t);
      if (n > 0 && n <= 1 << 20) {
        uint8_t *p = malloc((size_t)n + 16);
        assert(p);
        memset(p, 0xa5, (size_t)n + 16);
        int ok = TIFFIsTiled(t) ? TIFFReadEncodedTile(t, 0, p, n) >= 0
                                : TIFFReadScanline(t, p, 0, 0) == 1;
        if (ok)
          decoded++;
        for (size_t j = (size_t)n; j < (size_t)n + 16; j++)
          assert(p[j] == 0xa5);
        if (!ok)
          for (size_t j = 0; j < (size_t)n; j++)
            assert(p[j] == 0xa5);
        free(p);
      }
      uint64_t bytes = (uint64_t)w * h * 4;
      if (bytes && bytes <= 1 << 20) {
        uint8_t *p = malloc((size_t)bytes + 16);
        assert(p);
        memset(p, 0xa5, (size_t)bytes + 16);
        int ok = TIFFReadRGBAImage(t, w, h, (uint32_t *)p, 1);
        for (size_t j = (size_t)bytes; j < (size_t)bytes + 16; j++)
          assert(p[j] == 0xa5);
        if (!ok)
          for (size_t j = 0; j < (size_t)bytes; j++)
            assert(p[j] == 0xa5);
        free(p);
      }
      TIFFClose(t);
      assert(memory.closed == 1);
    } else
      assert(memory.closed == 0);
    free(data);
  }
  TIFFOpenOptionsFree(options);
  assert(accepted && decoded);
  printf("C ABI mutation smoke: seed=0x4341504954494646 iterations=%u seeds=%d "
         "accepted=%u native_decoded=%u\n",
         iterations, argc - 1, accepted, decoded);
  return 0;
}
