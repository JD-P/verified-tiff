/* Test-only float XYZ reference, linked solely to pinned upstream libtiff. */
#include <tiffio.h>
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int main(int argc, char **argv) {
  assert(argc == 3 && !strcmp(argv[1], "logluv"));
  TIFF *t = TIFFOpen(argv[2], "rc");
  assert(t && TIFFSetField(t, TIFFTAG_SGILOGDATAFMT, SGILOGDATAFMT_FLOAT));
  tmsize_t n = TIFFStripSize(t);
  assert(n > 0);
  void *out = malloc((size_t)n);
  assert(out);
  for (uint32_t k = 0; k < TIFFNumberOfStrips(t); k++) {
    tmsize_t got = TIFFReadEncodedStrip(t, k, out, n);
    assert(got > 0 && fwrite(out, 1, (size_t)got, stdout) == (size_t)got);
  }
  free(out);
  TIFFClose(t);
  return 0;
}
