#ifndef VERIFIED_TIFF_CAPI_H
#define VERIFIED_TIFF_CAPI_H
#include "tiffio.h"
#ifdef __cplusplus
extern "C" {
#endif
/* Thread-local diagnostic, including calls without a TIFF handle. A returned
 * string is borrowed until the next diagnostic on this thread. */
int VTIFFLastErrorCode(void);
const char *VTIFFLastError(void);
void VTIFFClearError(void);
/* Applied at open; limits affect input, each image/scratch/adapter output, and
 * each inspect/decode operation. Zero is a real zero limit, not unlimited. */
int VTIFFOpenOptionsSetLimits(TIFFOpenOptions *, uint64_t input_bytes,
                              uint64_t output_bytes, uint64_t work_units);
#ifdef __cplusplus
}
#endif
#endif
