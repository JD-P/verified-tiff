# Native LERC for TIFF

The C reader supports Compression 34887, including LERC alone, LERC + Deflate,
and LERC + Zstandard. `LercParameters` (50674) is `[4, wrapper]`, where wrapper
is 0, 1 or 2. Version 4 is the Lerc2 bitstream version selected by libtiff 4.7.1;
it is distinct from the test library's release number, Esri LERC 4.0.0.
Standalone Lerc1 and Lerc2 bitstream versions other than 4 are outside this TIFF
profile. This is a decoder; no LERC encoder is provided.

## Samples and layouts

All eight LERC types are supported: signed/unsigned 8-, 16- and 32-bit integers,
IEEE32 and IEEE64. Depth is 1–64; block width is 1–16. Classic TIFF and BigTIFF,
strips, padded tiles, contiguous or separate samples, and both TIFF byte orders
use the same native engine. Predictor and FillOrder must both be 1.

Decoding covers constant images and depths, raw one-sweep storage, block
constants/raw/reduced offsets, bit stuffing and lookup tables, byte Huffman
and delta-Huffman modes, and RLE validity masks. Consecutive band blobs can
reuse the previous mask. Fletcher32 and the exact declared blob ending are
validated. The native API consumes one blob and reports its length; the TIFF
adapter consumes exactly the required bands and rejects trailing data.

The native result contains sample bits in `u64` slots and a packed MSB-first
validity mask. Invalid slots are zero. The TIFF adapter maps invalid float
samples to canonical quiet NaNs and supports an unsigned-byte RGB + binary
unassociated-alpha mask. Floating multiband data is transposed into TIFF's
interleaved sample order. Masks are preserved across bands only within one
segment. Nonfinite extrema in an encoded header are rejected.

LERC's typed payload has its own little-endian encoding, independent of the
TIFF directory byte order. Our adapter returns native-host sample words.
The pinned libtiff reference applies an additional generic swap for big-endian
TIFFs: the test explicitly reverses that reference-only swap before comparing
its samples. A single shared mask across several float bands is also accepted
here and compared directly with Esri; libtiff rejects that layout. These two
differences are not counted as byte-for-byte upstream TIFF agreement.

## Verification boundary and resources

`native_lerc::{inspect,work_limit,decode}` and `lerc_*` are inside Verus.
`inspect` checks bounded metadata; `decode` checks checksum, masks, entropy,
exact consumption and reconstructed samples. Its contract equates success
with the explicit `decoded` relation, sufficient buffers and sufficient work.
On success, output/mask prefixes equal that relation; their suffixes are
unchanged on every return. Failed decoding can alter the prefixes. The C
adapter stages results and keeps application output unchanged on failure.

Float reconstruction uses [native hardware operations](native-float.md) for
finite multiplication, addition and typed conversions. Five scalar primitive
contracts in `trusted-float` are explicitly trusted; the decoder and software
fallback remain verified against the original exact bit-level models. This is
not a theorem about real arithmetic or Esri's implementation. Non-finite bit
patterns preserve historical software behavior, including NaN payloads. The
hardware environment assumptions and source audit are documented separately.

The software reference retains the proved leading-zero normalization, sticky
alignment, rounding and packing kernels. Boundary tests compare native results
directly against that independent executable reference as well as Esri.

Work is `8 * encoded_bytes + 4096 * samples + 8 * pixels + 256` per blob.
Caller-owned scratch consists of sample words and a packed validity mask.
The wrapper expands into a separately bounded adapter buffer using verified
zlib or Zstandard code. Allocation, TIFF field interpretation, band assembly,
NaN/alpha presentation and FFI remain unverified adapter operations. This API
does not expand the original `Raster` acceptance theorem.

Use the existing `VTIFF_MAX_WORK`, output and allocation options for large
segments; these conservative charges are abstract work allowances. They are
not measured CPU instructions. Existing defaults can reject large segments.

## Independent checks

`scripts/lerc-dependency.py` builds the checksum-pinned Esri test library.
`tests/lerc.py` checks 864 encoded/decoded cases across all sample types,
dimensions, depths, masks and entropy patterns; another 32 cases cover full-range
integers, extreme/subnormal floats, all-invalid masks and depth 64. The C matrix checks 384 TIFF
layouts, 18 alpha/float-band layouts, six shared-mask layouts, all wrapper
modes, exact native output and the separately built libtiff LERC oracle.
Truncation, checksum corruption, extra data, insufficient work and C mutation
checks exercise failure paths. Negative proof gates break checksum validation,
rounding and acceptance. All prior semantic gates are retained.

The kernel optimization adds a 64-bit split path for wide integer rounding and
a direct normal binary64-to-binary32 narrowing path. Both refine the existing
software IEEE model, including ties-to-even, mantissa carry and overflow. The
original path remains for subnormals and exceptional values. That earlier software pass introduced no native FP or new trusted cast; the
current hardware pass is described above. Tests cover exponent boundaries,
rounding ties and hardware comparisons; semantic mutations are rejected.

The [word-arithmetic pass](lerc-arithmetic.md) additionally proves 64-bit jammed
addition/subtraction for normal results with sufficient rounding separation,
using the original wide path for deep cancellation and exceptional ranges.
Normal packing handles its possible carry without a second normalization scan;
normal scaling products use 64-bit rounding decisions. The original numerical
models, independently rounded multiply/add evaluation and budgets are unchanged.

The [throughput pass](lerc-throughput.md) reads packed integers through bounded
five-byte windows, reuses one block scratch array, specializes float block loops,
and packs native words with fixed-width adapter stores. The public word-buffer
API and its exact acceptance/work/frame contracts remain unchanged.
