/* The same consumer is compiled once against libtiff and once against ours. */
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <tiffio.h>
int main(int argc, char **argv) {
  if (argc < 4)
    return 2;
  TIFFSetErrorHandler(NULL);
  TIFF *t = TIFFOpen(argv[1], "r");
  if (!t)
    return 3;
  if (argc > 4 && !TIFFSetDirectory(t, (tdir_t)strtoul(argv[4], NULL, 10))) {
    TIFFClose(t);
    return 4;
  }
  uint32_t w = 0, h = 0;
  uint16_t photo = 0, planar = 0, ch = 0;
  if (!TIFFGetField(t, 256, &w) || !TIFFGetField(t, 257, &h) ||
      !TIFFGetField(t, 262, &photo) ||
      !TIFFGetFieldDefaulted(t, 284, &planar) ||
      !TIFFGetFieldDefaulted(t, 277, &ch)) {
    TIFFClose(t);
    return 5;
  }
  if (photo == PHOTOMETRIC_LOGLUV &&
      !TIFFSetField(t, TIFFTAG_SGILOGDATAFMT, SGILOGDATAFMT_16BIT)) {
    TIFFClose(t);
    return 6;
  }
  uint16_t compression=1; TIFFGetFieldDefaulted(t,TIFFTAG_COMPRESSION,&compression);
  if (photo == PHOTOMETRIC_YCBCR && (compression==6||compression==7) &&
      !TIFFSetField(t, TIFFTAG_JPEGCOLORMODE, JPEGCOLORMODE_RGB)) {
    TIFFClose(t);
    return 6;
  }
  FILE *f = fopen(argv[3], "wb");
  if (!f) {
    TIFFClose(t);
    return 7;
  }
  int rgba = !strcmp(argv[2], "rgba"), scan = !strcmp(argv[2], "scan"),
      raw = !strcmp(argv[2], "raw");
  if (rgba) {
    if ((uint64_t)w * h > 33554432) {
      fclose(f);
      TIFFClose(t);
      return 8;
    }
    size_t n = (size_t)w * h * 4;
    uint8_t *p = malloc(n + 16);
    assert(p);
    memset(p, 0xa5, n + 16);
    int orientation = argc > 5 ? atoi(argv[5]) : ORIENTATION_TOPLEFT;
    if (!TIFFReadRGBAImageOriented(t, w, h, (uint32_t *)p, orientation, 1)) {
      free(p);
      TIFFClose(t);
      fclose(f);
      return 9;
    }
    for (size_t i = n; i < n + 16; i++)
      assert(p[i] == 0xa5);
    assert(fwrite(p, 1, n, f) == n);
    free(p);
  } else if (!strcmp(argv[2], "rgba-segments")) {
    int tiled = TIFFIsTiled(t);
    uint32_t bw = w, bh = 0;
    if (tiled) {
      assert(TIFFGetField(t, TIFFTAG_TILEWIDTH, &bw));
      assert(TIFFGetField(t, TIFFTAG_TILELENGTH, &bh));
    } else
      assert(TIFFGetFieldDefaulted(t, TIFFTAG_ROWSPERSTRIP, &bh));
    assert(bw && bh);
    if (bh > h && !tiled)
      bh = h;
    size_t cap = (size_t)bw * bh * 4;
    assert(cap < 67108864);
    uint8_t *p = malloc(cap + 16);
    assert(p);
    for (uint32_t y = 0; y < h; y += bh)
      for (uint32_t x = 0; x < w; x += bw) {
        size_t n = tiled ? cap : (size_t)w * (bh < h - y ? bh : h - y) * 4;
        memset(p, 0xa5, cap + 16);
        int ok = tiled ? TIFFReadRGBATile(t, x, y, (uint32_t *)p)
                       : TIFFReadRGBAStrip(t, y, (uint32_t *)p);
        if (!ok) {
          free(p);
          fclose(f);
          TIFFClose(t);
          return 14;
        }
        for (size_t i = n; i < cap + 16; i++)
          assert(p[i] == 0xa5);
        assert(fwrite(p, 1, n, f) == n);
      }
    free(p);
  } else if (scan) {
    tmsize_t n = TIFFScanlineSize(t);
    if (n <= 0 || n > 67108864) {
      fclose(f);
      TIFFClose(t);
      return 10;
    }
    uint8_t *p = malloc((size_t)n + 16);
    assert(p);
    for (uint16_t s = 0; s < (planar == 2 ? ch : 1); s++)
      for (uint32_t y = 0; y < h; y++) {
        memset(p, 0xa5, (size_t)n + 16);
        if (TIFFReadScanline(t, p, y, s) != 1) {
          free(p);
          fclose(f);
          TIFFClose(t);
          return 11;
        }
        for (size_t i = (size_t)n; i < (size_t)n + 16; i++)
          assert(p[i] == 0xa5);
        assert(fwrite(p, 1, (size_t)n, f) == (size_t)n);
      }
    free(p);
  } else {
    int tiled = TIFFIsTiled(t);
    uint32_t count = tiled ? TIFFNumberOfTiles(t) : TIFFNumberOfStrips(t);
    for (uint32_t k = 0; k < count; k++) {
      tmsize_t cap = raw     ? (tmsize_t)TIFFGetStrileByteCount(t, k)
                     : tiled ? TIFFTileSize(t)
                             : TIFFStripSize(t);
      if (cap <= 0 || cap > 67108864) {
        fclose(f);
        TIFFClose(t);
        return 12;
      }
      uint8_t *p = malloc((size_t)cap + 16);
      assert(p);
      memset(p, 0xa5, (size_t)cap + 16);
      tmsize_t n = raw ? (tiled ? TIFFReadRawTile(t, k, p, cap)
                                : TIFFReadRawStrip(t, k, p, cap))
                       : (tiled ? TIFFReadEncodedTile(t, k, p, cap)
                                : TIFFReadEncodedStrip(t, k, p, cap));
      if (n < 0) {
        free(p);
        fclose(f);
        TIFFClose(t);
        return 13;
      }
      assert(n <= cap);
      for (size_t i = (size_t)n; i < (size_t)cap + 16; i++)
        assert(p[i] == 0xa5);
      assert(fwrite(p, 1, (size_t)n, f) == (size_t)n);
      free(p);
    }
  }
  assert(fclose(f) == 0);
  TIFFClose(t);
  return 0;
}
