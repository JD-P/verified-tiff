/* Independent libtiff fixture writer: controlled images, codec/layout variants.
 */
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <tiffio.h>
static uint32_t random_state = 0x54494646;
static uint32_t random_word(void) {
  random_state ^= random_state << 13;
  random_state ^= random_state >> 17;
  random_state ^= random_state << 5;
  return random_state;
}
int main(int argc, char **argv) {
  assert(argc == 8);
  unsigned compression = atoi(argv[2]), bits = atoi(argv[3]),
           ch = atoi(argv[4]), noise = atoi(argv[5]), tiled = atoi(argv[6]),
           predictor = atoi(argv[7]);
  const uint32_t w = 1024, h = 1024, bw = tiled ? 128 : w,
                 bh = tiled ? 128 : 64;
  TIFF *t = TIFFOpen(argv[1], "wl");
  assert(t);
  assert(TIFFSetField(t, TIFFTAG_IMAGEWIDTH, w) &&
         TIFFSetField(t, TIFFTAG_IMAGELENGTH, h));
  assert(TIFFSetField(t, TIFFTAG_BITSPERSAMPLE, bits) &&
         TIFFSetField(t, TIFFTAG_SAMPLESPERPIXEL, ch));
  assert(TIFFSetField(t, TIFFTAG_PHOTOMETRIC,
                      bits == 1 ? PHOTOMETRIC_MINISWHITE
                      : ch == 1 ? PHOTOMETRIC_MINISBLACK
                                : PHOTOMETRIC_RGB));
  assert(TIFFSetField(t, TIFFTAG_PLANARCONFIG, PLANARCONFIG_CONTIG));
  assert(TIFFSetField(t, TIFFTAG_COMPRESSION, compression));
  assert(TIFFSetField(t, TIFFTAG_SAMPLEFORMAT,
                      bits == 64 ? SAMPLEFORMAT_IEEEFP : SAMPLEFORMAT_UINT));
  if (tiled)
    assert(TIFFSetField(t, TIFFTAG_TILEWIDTH, bw) &&
           TIFFSetField(t, TIFFTAG_TILELENGTH, bh));
  else
    assert(TIFFSetField(t, TIFFTAG_ROWSPERSTRIP, bh));
  if (compression == COMPRESSION_JPEG)
    assert(TIFFSetField(t, TIFFTAG_JPEGQUALITY, 85));
  if (predictor > 1)
    assert(TIFFSetField(t, TIFFTAG_PREDICTOR, predictor));
  size_t stride = (size_t)bw * ch * bits / 8, n = stride * bh;
  uint8_t *block = calloc(1, n);
  assert(block);
  for (uint32_t y0 = 0; y0 < h; y0 += bh)
    for (uint32_t x0 = 0; x0 < w; x0 += bw) {
      memset(block, 0, n);
      for (uint32_t y = 0; y < bh; y++)
        for (uint32_t x = 0; x < bw; x++)
          for (unsigned c = 0; c < ch; c++) {
            uint32_t xx = x0 + x, yy = y0 + y;
            size_t i = ((size_t)y * bw + x) * ch + c;
            if (bits == 1) {
              unsigned v = yy % 32 < 2 || (yy % 32 < 23 && xx % 11 < 2 &&
                                           (xx / 120 + yy / 32) % 5 != 0);
              block[y * stride + x / 8] |= (uint8_t)(v << (7 - x % 8));
            } else if (bits == 8)
              block[i] =
                  (uint8_t)(noise ? random_word() : (xx / 8 + yy / 8 + c * 67));
            else if (bits == 16) {
              uint16_t v = (uint16_t)(xx * 31 + yy * 17 + c * 1193);
              memcpy(block + 2 * i, &v, 2);
            } else {
              double v = (double)(xx + yy * 7 + c * 13) / 17.0;
              memcpy(block + 8 * i, &v, 8);
            }
          }
      uint32_t index =
          tiled ? TIFFComputeTile(t, x0, y0, 0, 0) : TIFFComputeStrip(t, y0, 0);
      assert((tiled ? TIFFWriteEncodedTile(t, index, block, (tmsize_t)n)
                    : TIFFWriteEncodedStrip(t, index, block, (tmsize_t)n)) ==
             (tmsize_t)n);
    }
  free(block);
  TIFFClose(t);
  return 0;
}
