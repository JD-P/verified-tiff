/*
 * Copyright (c) 1997 Greg Ward Larson
 * Copyright (c) 1997 Silicon Graphics, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software and
 * its documentation for any purpose is hereby granted without fee, provided
 * that (i) the above copyright notices and this permission notice appear in
 * all copies of the software and related documentation, and (ii) the names of
 * Sam Leffler, Greg Larson and Silicon Graphics may not be used in any
 * advertising or publicity relating to the software without the specific,
 * prior written permission of Sam Leffler, Greg Larson and Silicon Graphics.
 *
 * THE SOFTWARE IS PROVIDED "AS-IS" AND WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS, IMPLIED OR OTHERWISE, INCLUDING WITHOUT LIMITATION, ANY
 * WARRANTY OF MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE.
 *
 * IN NO EVENT SHALL SAM LEFFLER, GREG LARSON OR SILICON GRAPHICS BE LIABLE
 * FOR ANY SPECIAL, INCIDENTAL, INDIRECT OR CONSEQUENTIAL DAMAGES OF ANY KIND,
 * OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
 * WHETHER OR NOT ADVISED OF THE POSSIBILITY OF DAMAGE, AND ON ANY THEORY OF
 * LIABILITY, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
 * OF THIS SOFTWARE.
 */

/* Public numerical helpers only; SGILog codec parsing is not imported. */
#include "uv_table.inc"
#define M_LN2 0.69314718055994530942
#define U_NEU 0.210526316
#define V_NEU 0.473684211
#define UVSCALE 410.
double
    LogL16toY(int p16) /* compute luminance from 16-bit LogL */
{
    int Le = p16 & 0x7fff;
    double Y;

    if (!Le)
        return (0.);
    Y = exp(M_LN2 / 256. * (Le + .5) - M_LN2 * 64.);
    return (!(p16 & 0x8000) ? Y : -Y);
}
void
    XYZtoRGB24(float *xyz, uint8_t *rgb)
{
    if(!xyz||!rgb||!isfinite(xyz[0])||!isfinite(xyz[1])||!isfinite(xyz[2])){fail(NULL,EINVAL,__func__,"invalid XYZ input");return;}
    double r, g, b;
    /* assume CCIR-709 primaries */
    r = 2.690 * xyz[0] + -1.276 * xyz[1] + -0.414 * xyz[2];
    g = -1.022 * xyz[0] + 1.978 * xyz[1] + 0.044 * xyz[2];
    b = 0.061 * xyz[0] + -0.224 * xyz[1] + 1.163 * xyz[2];
    /* assume 2.0 gamma for speed */
    /* could use integer sqrt approx., but this is probably faster */
    rgb[0] = (uint8_t)((r <= 0.) ? 0 : (r >= 1.) ? 255 : (int)(256. * sqrt(r)));
    rgb[1] = (uint8_t)((g <= 0.) ? 0 : (g >= 1.) ? 255 : (int)(256. * sqrt(g)));
    rgb[2] = (uint8_t)((b <= 0.) ? 0 : (b >= 1.) ? 255 : (int)(256. * sqrt(b)));
}
double
    LogL10toY(int p10) /* compute luminance from 10-bit LogL */
{
    if (p10 == 0)
        return (0.);
    return (exp(M_LN2 / 64. * (p10 + .5) - M_LN2 * 12.));
}
int
    uv_decode(double *up, double *vp, int c) /* decode (u',v') index */
{
    unsigned int upper, lower;
    int ui;
    unsigned int vi;

    if (!up||!vp||c < 0 || c >= UV_NDIVS)
        return (-1);
    lower = 0; /* binary search */
    upper = UV_NVS;
    while (upper - lower > 1)
    {
        vi = (lower + upper) >> 1;
        ui = c - uv_row[vi].ncum;
        if (ui > 0)
            lower = vi;
        else if (ui < 0)
            upper = vi;
        else
        {
            lower = vi;
            break;
        }
    }
    vi = lower;
    ui = c - uv_row[vi].ncum;
    *up = uv_row[vi].ustart + (ui + .5) * UV_SQSIZ;
    *vp = UV_VSTART + (vi + .5) * UV_SQSIZ;
    return (0);
}
void
    LogLuv24toXYZ(uint32_t p, float *XYZ)
{
    if(!XYZ)return;
    int Ce;
    double L, u, v, s, x, y;
    /* decode luminance */
    L = LogL10toY(p >> 14 & 0x3ff);
    if (L <= 0.)
    {
        XYZ[0] = XYZ[1] = XYZ[2] = 0.;
        return;
    }
    /* decode color */
    Ce = p & 0x3fff;
    if (uv_decode(&u, &v, Ce) < 0)
    {
        u = U_NEU;
        v = V_NEU;
    }
    s = 1. / (6. * u - 16. * v + 12.);
    x = 9. * u * s;
    y = 4. * v * s;
    /* convert to XYZ */
    XYZ[0] = (float)(x / y * L);
    XYZ[1] = (float)L;
    XYZ[2] = (float)((1. - x - y) / y * L);
}
void
    LogLuv32toXYZ(uint32_t p, float *XYZ)
{
    if(!XYZ)return;
    double L, u, v, s, x, y;
    /* decode luminance */
    L = LogL16toY((int)p >> 16);
    if (L <= 0.)
    {
        XYZ[0] = XYZ[1] = XYZ[2] = 0.;
        return;
    }
    /* decode color */
    u = 1. / UVSCALE * ((p >> 8 & 0xff) + .5);
    v = 1. / UVSCALE * ((p & 0xff) + .5);
    s = 1. / (6. * u - 16. * v + 12.);
    x = 9. * u * s;
    y = 4. * v * s;
    /* convert to XYZ */
    XYZ[0] = (float)(x / y * L);
    XYZ[1] = (float)L;
    XYZ[2] = (float)((1. - x - y) / y * L);
}
#undef M_LN2
#undef U_NEU
#undef V_NEU
#undef UVSCALE
