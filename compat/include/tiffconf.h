/* Compatibility header configuration, Linux LP64. No codec feature macros:
 * availability is queried through TIFFIsCODECConfigured at runtime. */
#ifndef _TIFFCONF_
#define _TIFFCONF_
#include <stddef.h>
#include <stdint.h>
#include <inttypes.h>
#define TIFF_SSIZE_T ptrdiff_t
#define HAVE_IEEEFP 1
#define HOST_FILLORDER FILLORDER_LSB2MSB
#if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
#define HOST_BIGENDIAN 1
#else
#define HOST_BIGENDIAN 0
#endif
#endif
