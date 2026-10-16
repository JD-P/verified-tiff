/*
 * Copyright (c) 1988-1997 Sam Leffler
 * Copyright (c) 1991-1997 Silicon Graphics, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software and
 * its documentation for any purpose is hereby granted without fee, provided
 * that (i) the above copyright notices and this permission notice appear in
 * all copies of the software and related documentation, and (ii) the names of
 * Sam Leffler and Silicon Graphics may not be used in any advertising or
 * publicity relating to the software without the specific, prior written
 * permission of Sam Leffler and Silicon Graphics.
 *
 * THE SOFTWARE IS PROVIDED "AS-IS" AND WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS, IMPLIED OR OTHERWISE, INCLUDING WITHOUT LIMITATION, ANY
 * WARRANTY OF MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE.
 *
 * IN NO EVENT SHALL SAM LEFFLER OR SILICON GRAPHICS BE LIABLE FOR
 * ANY SPECIAL, INCIDENTAL, INDIRECT OR CONSEQUENTIAL DAMAGES OF ANY KIND,
 * OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
 * WHETHER OR NOT ADVISED OF THE POSSIBILITY OF DAMAGE, AND ON ANY THEORY OF
 * LIABILITY, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
 * OF THIS SOFTWARE.
 */

/*
 * CIE L*a*b* to CIE XYZ and CIE XYZ to RGB conversion routines are taken
 * from the VIPS library (http://www.vips.ecs.soton.ac.uk) with
 * the permission of John Cupitt, the VIPS author.
 */

/*
 * TIFF Library.
 *
 * Color space conversion routines.
 */

/* Adapted for the unverified public conversion-helper ABI. No codec parsing. */
#define TIFFmin(a,b) ((a)<(b)?(a):(b))
#define TIFFmax(a,b) ((a)>(b)?(a):(b))
#define TIFFroundup_32(a,b) (((a)+(b)-1)/(b)*(b))
static void TIFFCIELab16ToXYZ(TIFFCIELabToRGB *,uint32_t,int32_t,int32_t,float *,float *,float *);
#include <math.h>

/* Bit transport only: all float-to-index decisions execute in verified Rust. */
static uint32_t color_index(float value,uint32_t cap){uint32_t bits;memcpy(&bits,&value,4);return vt_color_index(bits,cap);}

/*
 * Convert color value from the CIE L*a*b* 1976 space to CIE XYZ.
 */
void TIFFCIELabToXYZ(TIFFCIELabToRGB *cielab, uint32_t l, int32_t a, int32_t b,
                     float *X, float *Y, float *Z)
{
    if(!cielab||!X||!Y||!Z||l>255||a< -128||a>127||b< -128||b>127){fail(NULL,EINVAL,__func__,"invalid Lab input");return;}
    TIFFCIELab16ToXYZ(cielab, l * 257, a * 256, b * 256, X, Y, Z);
}

/*
 * For CIELab encoded in 16 bits, L is an unsigned integer range [0,65535].
 * The a* and b* components are signed integers range [-32768,32767]. The 16
 * bit chrominance values are encoded as 256 times the 1976 CIE a* and b*
 * values
 */
static void TIFFCIELab16ToXYZ(TIFFCIELabToRGB *cielab, uint32_t l, int32_t a,
                       int32_t b, float *X, float *Y, float *Z)
{
    float L = (float)l * 100.0F / 65535.0F;
    float cby, tmp;

    if (L < 8.856F)
    {
        *Y = (L * cielab->Y0) / 903.292F;
        cby = 7.787F * (*Y / cielab->Y0) + 16.0F / 116.0F;
    }
    else
    {
        cby = (L + 16.0F) / 116.0F;
        *Y = cielab->Y0 * cby * cby * cby;
    }

    tmp = (float)a / 256.0F / 500.0F + cby;
    if (tmp < 0.2069F)
        *X = cielab->X0 * (tmp - 0.13793F) / 7.787F;
    else
        *X = cielab->X0 * tmp * tmp * tmp;

    tmp = cby - (float)b / 256.0F / 200.0F;
    if (tmp < 0.2069F)
        *Z = cielab->Z0 * (tmp - 0.13793F) / 7.787F;
    else
        *Z = cielab->Z0 * tmp * tmp * tmp;
}

#define RINT(R) ((uint32_t)((R) > 0 ? ((R) + 0.5) : ((R)-0.5)))
/*
 * Convert color value from the XYZ space to RGB.
 */
void TIFFXYZToRGB(TIFFCIELabToRGB *cielab, float X, float Y, float Z,
                  uint32_t *r, uint32_t *g, uint32_t *b)
{
    size_t i;
    float Yr, Yg, Yb;
    if(!cielab||!r||!g||!b||!isfinite(X)||!isfinite(Y)||!isfinite(Z)||fabsf(X)>1e10f||fabsf(Y)>1e10f||fabsf(Z)>1e10f){fail(NULL,EINVAL,__func__,"invalid XYZ input");return;}
    if(cielab->range<0||cielab->range>CIELABTORGB_TABLE_RANGE){fail(NULL,EINVAL,__func__,"invalid Lab table range");return;}
    float *matrix = &cielab->display.d_mat[0][0];

    /* Multiply through the matrix to get luminosity values. */
    Yr = matrix[0] * X + matrix[1] * Y + matrix[2] * Z;
    Yg = matrix[3] * X + matrix[4] * Y + matrix[5] * Z;
    Yb = matrix[6] * X + matrix[7] * Y + matrix[8] * Z;

    /* Clip input */
    Yr = TIFFmax(Yr, cielab->display.d_Y0R);
    Yg = TIFFmax(Yg, cielab->display.d_Y0G);
    Yb = TIFFmax(Yb, cielab->display.d_Y0B);

    /* Avoid overflow in case of wrong input values */
    Yr = TIFFmin(Yr, cielab->display.d_YCR);
    Yg = TIFFmin(Yg, cielab->display.d_YCG);
    Yb = TIFFmin(Yb, cielab->display.d_YCB);

    /* Turn luminosity to colour value. */
    i = color_index((Yr - cielab->display.d_Y0R) / cielab->rstep,(uint32_t)cielab->range);
    *r = color_index(cielab->Yr2r[i]+0.5f,cielab->display.d_Vrwr);

    i = color_index((Yg - cielab->display.d_Y0G) / cielab->gstep,(uint32_t)cielab->range);
    *g = color_index(cielab->Yg2g[i]+0.5f,cielab->display.d_Vrwg);

    i = color_index((Yb - cielab->display.d_Y0B) / cielab->bstep,(uint32_t)cielab->range);
    *b = color_index(cielab->Yb2b[i]+0.5f,cielab->display.d_Vrwb);

    /* color_index already proves each channel is bounded by its cap. */
}
#undef RINT

/*
 * Allocate conversion state structures and make look_up tables for
 * the Yr,Yb,Yg <=> r,g,b conversions.
 */
int TIFFCIELabToRGBInit(TIFFCIELabToRGB *cielab, const TIFFDisplay *display,
                        float *refWhite)
{
    size_t i;
    double dfGamma;

    if(!cielab||!display||!refWhite)return -1;
    for(int c=0;c<3;c++)if(!isfinite(refWhite[c])||refWhite[c]<=0||refWhite[c]>1e10f)return -1;
    const float parameters[]={display->d_YCR,display->d_YCG,display->d_YCB,display->d_Y0R,display->d_Y0G,display->d_Y0B,display->d_gammaR,display->d_gammaG,display->d_gammaB};
    for(size_t c=0;c<sizeof(parameters)/sizeof(*parameters);c++)if(!isfinite(parameters[c])||parameters[c]<0||parameters[c]>1e10f)return -1;
    if(display->d_YCR<=display->d_Y0R||display->d_YCG<=display->d_Y0G||display->d_YCB<=display->d_Y0B||display->d_gammaR<=0||display->d_gammaG<=0||display->d_gammaB<=0)return -1;
    for(int r=0;r<3;r++)for(int c=0;c<3;c++)if(!isfinite(display->d_mat[r][c])||fabsf(display->d_mat[r][c])>1e10f)return -1;
    if(display->d_Vrwr>4294967040U||display->d_Vrwg>4294967040U||display->d_Vrwb>4294967040U)return -1;
    if((display->d_YCR-display->d_Y0R)/CIELABTORGB_TABLE_RANGE<=0.f || (display->d_YCG-display->d_Y0G)/CIELABTORGB_TABLE_RANGE<=0.f || (display->d_YCB-display->d_Y0B)/CIELABTORGB_TABLE_RANGE<=0.f)return -1;
    cielab->range = CIELABTORGB_TABLE_RANGE;

    _TIFFmemcpy(&cielab->display, display, sizeof(TIFFDisplay));

    /* Red */
    dfGamma = 1.0 / cielab->display.d_gammaR;
    cielab->rstep =
        (cielab->display.d_YCR - cielab->display.d_Y0R) / cielab->range;
    for (i = 0; i <= (size_t)cielab->range; i++)
    {
        cielab->Yr2r[i] = cielab->display.d_Vrwr *
                          ((float)pow((double)i / cielab->range, dfGamma));
    }

    /* Green */
    dfGamma = 1.0 / cielab->display.d_gammaG;
    cielab->gstep =
        (cielab->display.d_YCG - cielab->display.d_Y0G) / cielab->range;
    for (i = 0; i <= (size_t)cielab->range; i++)
    {
        cielab->Yg2g[i] = cielab->display.d_Vrwg *
                          ((float)pow((double)i / cielab->range, dfGamma));
    }

    /* Blue */
    dfGamma = 1.0 / cielab->display.d_gammaB;
    cielab->bstep =
        (cielab->display.d_YCB - cielab->display.d_Y0B) / cielab->range;
    for (i = 0; i <= (size_t)cielab->range; i++)
    {
        cielab->Yb2b[i] = cielab->display.d_Vrwb *
                          ((float)pow((double)i / cielab->range, dfGamma));
    }

    /* Init reference white point */
    cielab->X0 = refWhite[0];
    cielab->Y0 = refWhite[1];
    cielab->Z0 = refWhite[2];

    return 0;
}

/*
 * Convert color value from the YCbCr space to RGB.
 * The colorspace conversion algorithm comes from the IJG v5a code;
 * see below for more information on how it works.
 */
#define SHIFT 16
#define FIX(x) ((int32_t)((x) * (1L << SHIFT) + 0.5))
#define ONE_HALF ((int32_t)(1 << (SHIFT - 1)))
#define Code2V(c, RB, RW, CR)                                                  \
    ((((c) - (int32_t)(RB)) * (float)(CR)) /                                   \
     (float)(((RW) - (RB) != 0) ? ((RW) - (RB)) : 1))
/* !((f)>=(min)) written that way to deal with NaN */
#define CLAMP(f, min, max)                                                     \
    ((!((f) >= (min))) ? (min) : (f) > (max) ? (max) : (f))
#define HICLAMP(f, max) ((f) > (max) ? (max) : (f))

void TIFFYCbCrtoRGB(TIFFYCbCrToRGB *ycbcr, uint32_t Y, int32_t Cb, int32_t Cr,
                    uint32_t *r, uint32_t *g, uint32_t *b)
{
    if(!ycbcr||!r||!g||!b||!ycbcr->Y_tab||!ycbcr->Cr_r_tab||!ycbcr->Cb_b_tab||!ycbcr->Cb_g_tab||!ycbcr->Cr_g_tab)return;
    Y = HICLAMP(Y,255);Cb=CLAMP(Cb,0,255);Cr=CLAMP(Cr,0,255);
    uint32_t q=vt_ycbcr_tables(ycbcr->Y_tab[Y],ycbcr->Cr_r_tab[Cr],ycbcr->Cb_b_tab[Cb],ycbcr->Cb_g_tab[Cb],ycbcr->Cr_g_tab[Cr]);
    *r=q&255;*g=(q>>8)&255;*b=(q>>16)&255;
}

/* Clamp function for sanitization purposes. Normally clamping should not */
/* occur for well behaved chroma and refBlackWhite coefficients */
static float CLAMPw(float v, float vmin, float vmax)
{
    if (v < vmin)
    {
        /* printf("%f clamped to %f\n", v, vmin); */
        return vmin;
    }
    if (v > vmax)
    {
        /* printf("%f clamped to %f\n", v, vmax); */
        return vmax;
    }
    return v;
}

/*
 * Initialize the YCbCr->RGB conversion tables.  The conversion
 * is done according to the 6.0 spec:
 *
 *    R = Y + Cr*(2 - 2*LumaRed)
 *    B = Y + Cb*(2 - 2*LumaBlue)
 *    G =   Y
 *        - LumaBlue*Cb*(2-2*LumaBlue)/LumaGreen
 *        - LumaRed*Cr*(2-2*LumaRed)/LumaGreen
 *
 * To avoid floating point arithmetic the fractional constants that
 * come out of the equations are represented as fixed point values
 * in the range 0...2^16.  We also eliminate multiplications by
 * pre-calculating possible values indexed by Cb and Cr (this code
 * assumes conversion is being done for 8-bit samples).
 */
int TIFFYCbCrToRGBInit(TIFFYCbCrToRGB *ycbcr, float *luma, float *refBlackWhite)
{
    TIFFRGBValue *clamptab;
    int i;

#define LumaRed luma[0]
#define LumaGreen luma[1]
#define LumaBlue luma[2]

    if(!ycbcr||!luma||!refBlackWhite)return -1;
    for(int c=0;c<3;c++)if(!isfinite(luma[c])||luma[c]<0||luma[c]>1)return -1;
    if(luma[1]<=0)return -1;
    for(int c=0;c<6;c++)if(!isfinite(refBlackWhite[c])||refBlackWhite[c]<0||refBlackWhite[c]>255)return -1;
    clamptab =
        (TIFFRGBValue *)((uint8_t *)ycbcr +
                         TIFFroundup_32(sizeof(TIFFYCbCrToRGB), sizeof(long)));
    _TIFFmemset(clamptab, 0, 256); /* v < 0 => 0 */
    ycbcr->clamptab = (clamptab += 256);
    for (i = 0; i < 256; i++)
        clamptab[i] = (TIFFRGBValue)i;
    _TIFFmemset(clamptab + 256, 255, 2 * 256); /* v > 255 => 255 */
    ycbcr->Cr_r_tab = (int *)(clamptab + 3 * 256);
    ycbcr->Cb_b_tab = ycbcr->Cr_r_tab + 256;
    ycbcr->Cr_g_tab = (int32_t *)(ycbcr->Cb_b_tab + 256);
    ycbcr->Cb_g_tab = ycbcr->Cr_g_tab + 256;
    ycbcr->Y_tab = ycbcr->Cb_g_tab + 256;

    {
        /* Validated finite luma in [0,1] makes f1/f3 lie in [0,2].
         * Green is strictly positive, so f2/f4 are nonnegative (possibly
         * +Inf for a tiny denominator) and need only an upper clamp. */
        float f1 = 2 - 2 * LumaRed;
        int32_t D1 = FIX(f1);
        float f2 = LumaRed * f1 / LumaGreen;
        int32_t D2 = -FIX(TIFFmin(f2, 2.0F));
        float f3 = 2 - 2 * LumaBlue;
        int32_t D3 = FIX(f3);
        float f4 = LumaBlue * f3 / LumaGreen;
        int32_t D4 = -FIX(TIFFmin(f4, 2.0F));
        int x;

#undef LumaBlue
#undef LumaGreen
#undef LumaRed

        /*
         * i is the actual input pixel value in the range 0..255
         * Cb and Cr values are in the range -128..127 (actually
         * they are in a range defined by the ReferenceBlackWhite
         * tag) so there is some range shifting to do here when
         * constructing tables indexed by the raw pixel data.
         */
        for (i = 0, x = -128; i < 256; i++, x++)
        {
            int32_t Cr = (int32_t)CLAMPw(Code2V(x, refBlackWhite[4] - 128.0F,
                                                refBlackWhite[5] - 128.0F, 127),
                                         -128.0F * 32, 128.0F * 32);
            int32_t Cb = (int32_t)CLAMPw(Code2V(x, refBlackWhite[2] - 128.0F,
                                                refBlackWhite[3] - 128.0F, 127),
                                         -128.0F * 32, 128.0F * 32);

            ycbcr->Cr_r_tab[i] = (int32_t)((D1 * Cr + ONE_HALF) >> SHIFT);
            ycbcr->Cb_b_tab[i] = (int32_t)((D3 * Cb + ONE_HALF) >> SHIFT);
            ycbcr->Cr_g_tab[i] = D2 * Cr;
            ycbcr->Cb_g_tab[i] = D4 * Cb + ONE_HALF;
            ycbcr->Y_tab[i] = (int32_t)CLAMPw(
                Code2V(x + 128, refBlackWhite[0], refBlackWhite[1], 255),
                -128.0F * 32, 128.0F * 32);
        }
    }

    return 0;
}
#undef HICLAMP
#undef CLAMP
#undef Code2V
#undef SHIFT
#undef ONE_HALF
#undef FIX

#undef TIFFmin
#undef TIFFmax
#undef TIFFroundup_32
