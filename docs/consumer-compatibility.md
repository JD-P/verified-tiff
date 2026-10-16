# Consumer compatibility work

The reader now implements the requested metadata, resource configuration,
read-side fields, color conversion, custom directories, temporary decoder
machinery and codec additions. Native codecs include CCITT RLE, Predictor 3,
dictionary-free Zstandard, Lerc2 v4 and full static VP8/VP8L with alpha. The
specific supported layouts and proof boundaries are documented below.

Consumers' independent JPEG/libjpeg paths are outside the requested scope.
No system libraries are replaced. The original whole-image Rust/CLI profiles
remain unchanged; new features below belong to the C reader.

| Priority | Implemented behavior | Boundary |
| --- | --- | --- |
| Optional metadata | Absent queries return zero quietly without modifying outputs; standard TIFF/EXIF/GPS descriptors, aligned host-type values and anonymous tags | Valid foreign varargs/pointers required; malformed present values can still fail |
| Resource settings | Large seekable files open without an arbitrary 64 MiB file ceiling; environment and per-open allocation/output/work limits | Finite per-read work and allocation budgets; not process-wide CPU deadlines |
| Read-side fields | SampleFormat, orientation, photometric/extra-sample metadata and registered custom fields; JPEG RGB; LogLuv float XYZ/native16 | In-memory interpretation only; reader geometry/codec changes and disk editing remain unsupported |
| RGBA colors | CMYK8/16 and CIELab8/16, including separate planes; non-JPEG contiguous YCbCr8 clumps, sampling factors 1/2/4 | Numerical/color presentation in unverified adapter; no ICC color management |
| Custom metadata | TIFFSetTagExtender/TIFFMergeFieldInfo, typed GeoTIFF-style arrays, anonymous field introspection, EXIF/GPS non-image directories | Not arbitrary private libtiff structure compatibility |
| Codecs/prediction | CCITT Modified Huffman/RLE, both FillOrders; dictionary-free ZSTD; Predictor 3 for IEEE32/64 with LZW/Deflate/ZSTD; Lerc2 v4; static VP8/VP8L with alpha | Separate native proof boundaries; see [LERC](lerc.md) and [WebP](webp-native.md) |
| Temporary decoding | TIFFReadFromUserBuffer; callback-based single-directory metadata writer, Classic/BigTIFF and either byte order; WriteCheck/WriteDirectory/Flush | No pixel encoding, multipage writing, path/fd writing, appending or in-place editing |

## Limits

Settings are read when options/handles are constructed. Decimal byte/count
values only; malformed/overflowing values fail opening. Explicit per-open
options take precedence over the environment, including replacing malformed
values for the settings they override. An invalid environment value for a
different, unoverridden setting still fails opening. Invalid negative standard
allocation setters remain errors and are not repaired by later setters.

| Variable | Default |
| --- | --- |
| VTIFF_MAX_FILE_BYTES | PTRDIFF_MAX, with seekable reads and no whole-file allocation |
| VTIFF_MAX_BUFFER_BYTES | 268435456 (256 MiB) |
| VTIFF_MAX_WORK | 1000000000 core work units per read |
| VTIFF_MAX_SINGLE_ALLOC | 268435456 (256 MiB) |
| VTIFF_MAX_TOTAL_ALLOC | 536870912 (512 MiB) |

Input/output/work zero is a zero allowance; allocation zero follows libtiff's
unlimited convention. C field definitions and typed-value caches are charged
to the handle too. Growth accounts for old and replacement buffers while both
are alive. Limits still include 4096 tags/field definitions, 1024 directories
and 1048576 segments. The CLI retains its earlier defaults.

## Field behavior

The generated descriptors reproduce libtiff 4.7.1's public calling conventions;
`compat/generate_fields.py` derives them from its pinned source. RATIONAL and
SRATIONAL values convert to the descriptor's float/double type; integer
narrowing is checked; anonymous fields use a 32-bit count and pointer. Typed
arrays have u64 alignment and survive further getters. Borrowed pointers are
invalidated by successful field mutation, directory selection or close.
EXIF/GPS selection validates the raw directory without requiring image geometry;
GPS tag zero is meaningful. Failed selection retains the previous directory.

Read-side SampleFormat updates preserve stored sample bits and invalidate
decoded caches. LogLuv defaults to XYZ float32, matching libtiff; mode 1 returns
the original native16 components. The verified native chroma mapping is
injective, so the adapter recovers encoded chroma before XYZ conversion.
Float XYZ uses libm arithmetic and is compared numerically with libtiff; it is
not a newly proved real-valued color theorem.

Non-JPEG YCbCr strip/tile reads return packed sampling clumps. The size APIs
round to whole clumps, including edge padding. Scanlines with vertical
subsampling require strip/tile reads instead. RGBA expands nearest chroma and
honors YCbCrCoefficients/ReferenceBlackWhite; Lab honors WhitePoint. Colors
are rendered only after all separate component planes are available. CMYK
black is a color component, not alpha; explicit extra alpha is handled separately.

## Local verified additions

`native_fp::decode` has exact success conditions for geometry and caller
buffers; it composes the verified byte-prefix accumulator with a proved
significance-plane permutation. Prediction resets per row and per channel,
and sample/row/tile bits including NaNs are preserved. Output and scratch
suffixes remain unchanged. There is no floating-point arithmetic in this core.

`native_rle::decode` reuses the verified CCITT codebook, run endpoints, row
semantics and pixel renderer. Its independent stream predicate describes
byte-aligned 1D rows, zero alignment padding and exact termination. Both
FillOrders work. Width is bounded at 65536; caller output and checked work
quotas bound the rest. Exact acceptance, decoded bits, work conservation and
suffix frames are checked. Damaged-row recovery is not introduced. Native
packed unused row bits are zeroed; libtiff leaves those bits caller-dependent.

Negative gates corrupt the Predictor 3 permutation and remove RLE row
alignment; both must fail their semantic contracts. Existing gates remain.
These are local block/transform proofs, **not** an end-to-end C/BigTIFF theorem.

## Temporary writer and user buffers

The writer is restricted to TIFFClientOpen/Ext modes `w`, optionally `b`/`l`,
`8` and `+`, with read/write/seek/size callbacks. It serializes metadata and
zero-valued strip/tile offset/count arrays into a temporary file. Validation
and allocation finish before writes begin. A callback I/O failure can leave
partial temporary bytes; caller-provided output from decoding remains atomic.
TIFFClose flushes a dirty writer, while TIFFCleanup releases it without I/O.

TIFFReadFromUserBuffer uses a bounded owned copy of the supplied encoded data,
never modifies that input, and does not reuse/overwrite the source segment
cache. It validates a complete segment even for a requested output prefix.
Too-large output requests and malformed streams fail without changing caller
output. This intentionally provides stronger failure behavior than libtiff's
partial output. The ordinary source must remain stable throughout a handle's
lifetime. OJPEG shared tables can still require source metadata reads.

The regression harness reproduces GDAL's temporary-decoder sequence with
strips/tiles, endian variants, BigTIFF and compressed input. This is a test of
the observed API sequence, not a claim to have run an installed full GDAL.

## Evidence

`compat/tests/consumers.py` adds 128 Predictor-3 files, 128 CMYK/Lab files,
32 RLE files, 72 YCbCr cases, Classic/BigTIFF metadata/EXIF/GPS cases, native
float read-side updates, LogLuv/libtiff numerical comparisons and temporary
decoder contexts. Each format uses independent sample/stream construction.
RLE comparisons mask only unused row-padding bits and also compare RGBA pixels.
Every CMYK/Lab matrix case now has a pixel reference: CMYK16 uses independent
rational conversion, and planar Lab uses the identical logical samples in an
interleaved file decoded by the pinned libtiff oracle.
YCbCr 4x4 odd edges use independently specified pixels: libtiff 4.7.1's renderer
disagrees there and its `putcontig8bitYCbCr44tile` uses a 4x2 `fromskew` size.
Other supported reference samplings compare exactly; the implementation does
not reproduce that edge-rendering bug.

`compat/tests/consumer_smoke.py` optionally runs installed ImageMagick,
Leptonica and GdkPixbuf in isolated processes and checks actual loader paths.
It includes CMYK, Lab, RLE, Group 4 and a 65 MiB seekable file; ImageMagick also
checks float32 and Predictor 3. The Pillow smoke test remains separate.
Sanitizers cover the C boundary, not Rust instrumentation; deterministic
mutation tests are not a coverage-guided fuzz campaign.

The subsequent Zstandard addition is specified in [zstd.md](zstd.md), including
its exact native acceptance, work, byte-string output and checksum boundaries.
The current delivery and its remaining work are listed in
[adoption-status.md](adoption-status.md).
