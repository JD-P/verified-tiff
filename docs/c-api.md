# Partial libtiff C compatibility interface

The [C shim hardening and coverage report](c-shim-hardening.md) records the current boundary tests, failure injection, fixes and 100% measured C function/line/branch coverage. This is test coverage, not a C proof.


`compat/` is a **reader-oriented, unverified Linux LP64 C ABI adapter** around the
Verus-verified Rust primitives. It targets libtiff 4.7.1's public `tiffio.h` and
exports all 198 functions with their standard symbol versions and SONAME 6.
There are 170 implemented or explicitly restricted entry points and 28
unsupported entry points (all generated ENOSYS stubs).
The exact inventory is [compat/api.json](../compat/api.json).

Export coverage is not full feature coverage. Image encoding, codec registration,
private symbols, Windows APIs, C++ stream wrappers and other ABI majors remain
outside this implementation. A restricted metadata-only callback writer supports
read-side temporary decoder contexts. See [consumer compatibility](consumer-compatibility.md)
for custom fields, EXIF/GPS, limits, color conversions, RLE/Predictor 3, and the
completed native LERC and static WebP codec additions. See [LERC](lerc.md) and [WebP](webp-native.md) for sample, bitstream and proof boundaries.

## Build and link

After pinned-toolchain bootstrap and oracle setup:

```bash
bash scripts/build-compat.sh
cc -Icompat/include your_reader.c -Ltarget/compat \
   -Wl,-rpath,"$PWD/target/compat" -ltiff -o your_reader
LD_LIBRARY_PATH="$PWD/target/compat" ./your_existing_reader input.tif
```

The build audits all public exports, keeps private bridge symbols hidden and
has no production libtiff/libjpeg/libwebp/LERC/Zstandard/zlib dependency. Those libraries are
test oracles only. Do not replace a distribution package based on these tests.
No system installation is performed by the build or test scripts.

## Opening, ownership and options

`TIFFOpen`, `TIFFFdOpen`, `TIFFClientOpen` and their `Ext` variants work with
read modes `r`, `rb`, `rc`, `rC`, `rm`, `rM`, `rB`, `rD`, `rO`, `rh`, and
combinations of these flags. `C` enables and `c` disables uncompressed
single-strip chopping; it is enabled by default, with an approximately 8 KiB
strip target. `B` requests the default MSB-first output. `h` reads only the
header until directory selection. `D`/`O` are accepted optimization hints;
strile metadata arrays are currently eager. No mode uses mmap, so `M` cannot
force it. `r+`, path/fd writing, LSB-output flags and unknown flags fail with
`ENOSYS`. Callback writing has the restricted temporary-directory behavior
described in [consumer compatibility](consumer-compatibility.md).

File descriptors are borrowed by the Rust backend and owned by the C handle
only after a successful open. `TIFFClose` closes the descriptor or invokes
the client close callback. `TIFFCleanup` releases the handle without closing
its source. Failed opens retain caller-owned resources. Client read/seek/size
callbacks are required; short positive reads work. File reads use `pread`.
The input is no longer snapshotted; its backing source must remain available.

Default file size is PTRDIFF_MAX, with seekable reads and no full-file snapshot.
Output/scratch buffers and single allocations default to 256 MiB, total live
adapter allocations to 512 MiB, and core work to one billion units per read.
`VTIFFOpenOptionsSetLimits` overrides file/output/work limits before an `Ext`
open; zero is an actual zero limit. Unmodified applications can configure the
five `VTIFF_MAX_*` environment variables documented in consumer-compatibility.md.
Metadata retains the 4096-tag, 1024-directory and 1048576-segment limits.

`TIFFOpenOptionsSetMaxSingleMemAlloc` and `SetMaxCumulatedMemAlloc` constrain
requested heap capacities for live adapter buffers plus a conservative fixed
handle/name charge. Standard zero means unlimited. Buffers are charged before
allocation and released on drop; temporary metadata, encoded bytes, decoding
scratch, cached segments and rendered output all count. Allocator rounding,
bookkeeping, stack space and caller allocations are outside these counters.
The legacy `_TIFFmalloc` utilities are not per-handle allocations. Tiny runtime
bookkeeping allocations can still abort on process-wide OOM. These limits
are not lifetime CPU or wall-clock deadlines. Adapter metadata/render loops
are structurally bounded but are outside the verified work counter.

Global and per-handle error/warning handlers work, including `ExtR` forms.
`TIFFOpenOptionsSetWarnAboutUnknownTags` controls warnings for tags the adapter
does not recognize; it defaults to false. Invalid negative memory limits
invalidate the options instead of silently discarding the request.

## Metadata and directories

Classic TIFF and BigTIFF, both byte orders, main-directory traversal, counting,
selection and image SubIFDs are supported. `TIFFSetSubDirectory` accepts a
valid image-directory offset, not just an already seen main directory.
Failed selection preserves the old directory and its borrowed arrays.
A SubIFD selected by offset has an unspecified main-chain index (`UINT32_MAX`);
use its directory offset or return to the main chain with `TIFFSetDirectory`.

Use libtiff's exact varargs output pointer types:

| Fields | Getter arguments |
| --- | --- |
| Dimensions, strip/tile dimensions, SubfileType, Group3/4Options | `uint32_t *` |
| Sample/color/layout enums, bit depth, predictor, resolution unit, min/max sample values | `uint16_t *` |
| Strip/tile offsets and byte counts | `uint64_t **` |
| ExtraSamples | `uint16_t *count`, `uint16_t **values` |
| SubIFD | `uint16_t *count`, `uint64_t **offsets` |
| ColorMap | Three `uint16_t **` channel arrays |
| X/YResolution and X/YPosition | `float *` |
| PageNumber, YCbCrSubsampling, HalftoneHints, DotRange | Two `uint16_t *` |
| DocumentName, ImageDescription, Make, Model, PageName, Software, DateTime, Artist, HostComputer, Copyright, TargetPrinter | `char **` |
| ICCProfile, XMLPacket, Photoshop, JPEGTables | `uint32_t *bytes`, `void **data` |
| EXIFIFD/GPSIFD pointers | `uint64_t *` |
| JpegColorMode/SGILogDataFmt | `int *` |

Metadata arrays and strings are borrowed, read-only and valid until a successful
field mutation, directory change or close. ASCII getters provide a terminating
NUL. Absent optional fields return zero without writing outputs or emitting an
error. Defaulted getters provide standard supported defaults, including color
coefficients/reference white and transfer tables. ICC bytes are exposed without
applying a color-management transform.

Field lookup/introspection covers the generated standard TIFF/EXIF/GPS schemas,
registered custom definitions and anonymous stored tags. TIFFSetTagExtender and
TIFFMergeFieldInfo support GeoTIFF-style consumers. Non-image EXIF/GPS directories
can be selected without image geometry. Typed metadata is endian converted,
aligned and narrowing checked; field-count/varargs conventions remain C ABI
requirements. TIFFSetField supports read-side sample interpretation and custom
metadata, JPEG RGB mode and LogLuv float/native16 modes. It changes handle state,
not an opened reader's file. Geometry/codec replacement on readers and filename
mutation remain unsupported.

## Pixel and raw reads

Scanlines, encoded strips/tiles, coordinate tile reads, raw strips/tiles,
geometry/index/size functions, whole-image RGBA, RGBA strips/tiles and basic
`TIFFRGBAImage` state all work within the supported formats. Large uncompressed, PackBits, LZW and Deflate segments support bounded range
reads and resumable scanlines; other codecs retain the one-segment cache.
RGBA uses its output raster plus row or active-segment storage. Pixel reads commit
to the caller only after success. Raw reads do not preflight image codecs.

Native reads preserve palette indices, associated-alpha samples, packed row
padding and padded tile samples. Word samples use host byte order. Generic
codecs transport packed integer widths 1–64, IEEE16/24/32/64 and
complex component pairs, including higher-bit alpha. Horizontal Predictor 2
retains its 8/16/32/64-bit non-complex restrictions; Predictor 3 supports
IEEE16/24/32/64. CCITT RLE is available through the new verified block primitive. See
[native compatibility](native-compatibility.md) for the exact combinations.
YCbCr JPEG native reads still require `JPEGCOLORMODE_RGB`; native subsampled
JPEG YCbCr is unavailable. Non-JPEG YCbCr8 supports native sampling clumps
through strips/tiles and RGBA conversion. LogLuv defaults to float32 XYZ;
`SGILOGDATAFMT_16BIT` selects the encoded native components. Scanline reads of tiled images remain unsupported.

Native reads retain storage order. RGBA honors source and requested
orientations 1–8 **as libtiff does**: 5–8 alias the corresponding flip cases;
its RGBA API does not transpose output dimensions for these values. Strips and
tiles use the bottom-up raster convention and zero-fill RGBA edge padding.
RGBA image reads accept cropped widths/heights. When requested height exceeds
available image rows, the unused leading raster rows remain untouched, matching
libtiff. `TIFFRGBAImageGet` accepts nonzero row/column offsets and smaller regions. Custom get/put callbacks remain
unsupported. Output words are host-order `0xAABBGGRR`.

Straight RGB alpha is premultiplied with nearest rounding; associated RGB is
preserved. Gray8+A follows pinned libtiff's gray/alpha convention. Gray16 uses
the high byte; RGB16 and its alpha use nearest scaling. Gray16+A preserves
the gray and alpha high bytes, a deliberate difference from libtiff's opaque
Gray16 display path; native alpha samples are exact. CMYK8/16 and CIELab8/16 presentation are supported, including separate planes.
Float/signed/LogLuv RGBA conversion is unsupported. JPEG retains the specified integer transform and nearest
chroma; identity with every libjpeg configuration is not claimed.

The independently checked cropped Gray16/Gray+A tile pixels deliberately avoid
libtiff 4.7.1's documented `fromskew` byte-stride bug; those cases do not claim
exact equality with its RGBA renderer. Native tile bytes remain exact.

## Reader extensions

`TIFFAccessTagMethods` exposes live get/set/print hooks. Named client information,
read buffer setup, field unsetting, directory freeing, custom-directory reading,
codec enumeration and directory printing are implemented. File-name, client-data,
file-number and mode setters return their prior values. Public Lab/YCbCr and
LogL/LogLuv decoding helpers are available. See [reader additions](reader-features.md)
for ownership, private field-array ABI, streaming limits and exact proof scope.

## Failures and trust

Unsupported APIs report `errno = ENOSYS`, update `VTIFFLastErrorCode`/`VTIFFLastError`,
invoke the configured diagnostic and return a failure sentinel without writing
caller output. Reads normally return -1, boolean APIs zero and pointer APIs
NULL. `VTIFFClearError` clears the thread-local extension state; successes do
not automatically clear old diagnostics. Invalid input uses `EINVAL`, resource
limits `EFBIG`, allocation failure `ENOMEM` and I/O failure `EIO`.

C pointers must be valid, aligned, correctly typed and sufficiently large.
Borrowed arrays must not be modified or reused as output destinations. A
handle requires exclusive access and cannot be closed or reentered during its
callbacks. Separate handles work on separate threads. Callbacks are trusted
caller code and must honor declared buffer and return-value contracts.

The C shim, Rust FFI adapter, seekable metadata assembly, synthetic container
adaptation, allocator accounting and RGBA presentation are **not formally
verified**. The core verifies local records, streams and sample transforms;
there is no end-to-end theorem for the new BigTIFF/C pipeline. The old complete
Classic TIFF acceptance proof is preserved. C undefined behavior, foreign
unwinding, malicious callbacks and allocator aborts are outside the guarantee.

## Validation

```bash
bash scripts/test-compat.sh
VTIFF_COMPAT_OUTPUT="$PWD/target/compat-asan" VTIFF_SANITIZE=1 \
  ASAN_OPTIONS=detect_leaks=0:abort_on_error=1 UBSAN_OPTIONS=halt_on_error=1 \
  bash scripts/test-compat.sh
# Optional installed real consumer:
python3 compat/tests/pillow_smoke.py
```

The C shim and consumers are instrumented by ASan/UBSan; Rust internals are not.
LeakSanitizer is disabled because of the environment's tracing restrictions;
no successful leak check is claimed. Suites include 2,000 deterministic C API
mutations, native and RGBA oracle matrices, failure frames, ownership, options,
metadata, lazy I/O and BigTIFF/SubIFDs. These are not coverage-guided fuzzing.
The unchanged upstream 108-test suite tests upstream libtiff itself, not this
adapter. Original core, corpus, Rust/Python and negative-proof gates still run.

Compression 50000 now uses the verified, dictionary-free Zstandard segment API,
with Predictors 1/2/3 and bounded final-strip padding. See [zstd.md](zstd.md).
TIFFIsCODECConfigured reports it available; unsupported dictionaries and
concatenated/skippable frames still fail explicitly.

Contiguous unsigned 8/16-bit RGB/gray RGBA requests now use specialized verified
row kernels. Cached segments are borrowed, and completely covered Deflate
segments up to 1 MiB can use private one-pass staging when allocation limits
admit it. Tight budgets and larger segments retain bounded replay. Caller
buffers remain unchanged on any failed read; see [rgba-optimization.md](rgba-optimization.md).

The kernel pass extends optional complete staging to LZW under the same 1 MiB
RGBA admission cap, and applies it to fully covered CMYK/Lab segments. Packed
1/2/4-bit gray and 1/2/4/8-bit palette presentation expands one source byte
through a fixed 8 KiB table. Lab's 1,501-entry gamma table is built once per
RGBA request; contiguous CMYK/Lab samples use direct native loads. These are
tested adapter presentation changes, not new end-to-end proof claims.

## Checked adapter decisions

The [adapter proof extension](adapter-proofs.md) supersedes earlier statements that all metadata conversion, crop placement, window bookkeeping and allocation arithmetic are unverified. The listed executable decisions now have checked contracts. C pointer validity, varargs extraction, ownership/event matching, I/O correctness and complete pipeline composition remain outside those local proofs. No new trusted primitive or format is introduced.


## Codec metadata and ink consistency

The [metadata consistency pass](metadata-contracts.md) adds the missing codec
and fax reception schemas and fixes the special InkNames getter ABI. Image
InkNames payloads use a verified exact acceptance/counting helper, and the
adapter derives NumberOfInks before publishing directory state. The fresh
whole-crate run has 1,809 verified obligations and zero errors. The counting
contract does not turn C varargs, schema extraction or directory publication
into end-to-end verified behavior; those remain covered by integration and
sanitizer tests. All emitted C function/line/branch coverage remains 100%.
