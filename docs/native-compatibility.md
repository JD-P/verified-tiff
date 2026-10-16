# Seekable native decoding and broader C compatibility

The later [consumer compatibility pass](consumer-compatibility.md) adds custom
metadata/directories, practical limits, read-side setters, color conversion,
CCITT RLE, Predictor 3 and GDAL-style temporary decoder contexts.

This delivery implements the five reader priorities in the C interface:
opening/options/ordinary metadata; exact native samples and edge tiles;
horizontal prediction and orientation; independent segments; and BigTIFF,
SubIFDs and broader sample formats. The original `Image`, `Raster` and CLI
profiles are unchanged. New formats are exposed through the C reader and the
new verified `container`, `native`, and `native_samples` building blocks.

## Data path

A file descriptor uses `pread`; a client handle retains its read/seek callbacks.
Opening parses the header and selected directory without reading pixels. Strip
and tile offset/count tables use a 256-entry page cache; pointer-returning array
getters materialize the complete table on explicit request. Both `rD`/`rO` and
ordinary read opens use this lazy behavior. Uncompressed range reads seek directly.
Large PackBits/LZW/Deflate reads validate with bounded scratch, then retain one
resumable decoder for sequential ranges. Other codecs retain a one-segment cache.
RGBA traverses only intersecting segments, with row scratch for generic codecs.
Raw prefix reads fetch only the requested prefix. No complete normalized image
is cached, and pixel-read failures leave caller output unchanged.

The source must remain available until close and stable during operations.
Changing a file after opening is not a supported snapshot-consistency model.

Generic native decoding accepts compression 1, 32773, 5, 8 and 32946. It retains
palette indices, associated-alpha values, unused packed row bits, and every
sample in padded edge tiles. Standard 16/24/32/64-bit words are returned in host byte
order; other packed widths retain their stored byte sequence, as libtiff does. FillOrder 2 reverses encoded bytes before
these codecs, as libtiff does. Packed integer widths 1–64 and IEEE16/24/32/64 are supported.
Complex samples use component-pair transport; see [reader additions](reader-features.md). Signed and floating values are
transported as exact bit patterns, including NaNs. Up to 64 uniform channels
and contiguous/separate planes are accepted. Native transport does not imply
an RGBA display conversion for every sample format or photometric value.

Horizontal Predictor 2 is modular, per channel, and resets each row. It works
with 8/16/32/64-bit words and LZW/Deflate, including signed/floating bit patterns.
The pinned libtiff predictor rejects 64-bit words, so those cases use independent
source expectations and the native transform proof rather than an oracle pass
claim. Predictor 3 supports IEEE16/24/32/64 with LZW/Deflate/ZSTD through
`native_fp`; mixed per-channel widths/formats remain unsupported. RGBA supports unsigned gray/palette/RGB and supported JPEG
YCbCr, through 16 bits, with associated/unassociated alpha. The later pass
adds CMYK/Lab and non-JPEG YCbCr display conversion. Unsigned 16-bit
Gray+A and RGBA native reads are exact; higher-bit native alpha is also exact.
Float/signed/LogLuv RGBA conversion remains unsupported.

JPEG/OJPEG, fax and LogLuv retain their documented codec subsets. WebP now uses the complete static VP8/VP8L native engine; LERC v4 supports all eight sample types and plain/Deflate/Zstandard wrappers. See [WebP](webp-native.md) and [LERC](lerc.md).
The adapter builds a bounded one-segment Classic TIFF view and calls the
existing verified inspector/decoder. This makes them usable inside BigTIFF
without duplicating codec implementations. Full tile dimensions retain padding.
For WebP, the native engine preserves stored alpha components so the
stored color components survive. Fax output is packed into native rows.

For OJPEG interchange headers, only shared tables and, when necessary, the
bounded SOS marker in the first strip are additionally read. No unrelated
strip entropy is decoded. The shared header is parsed by `native::jpeg_shared`.
Tables are copied per component; `renumber_ojpeg_scan` checks the original
component IDs/selectors, rewrites the SOS for those copies, and preserves all
entropy bytes. Three original OJPEG native outputs, cropped to image bounds for padded tiles,
are required to equal the original whole-image Rust decoder byte for byte. Synthetic container assembly
and composition with original TIFF metadata remain unverified adapter work.

## BigTIFF and directories

Both byte orders, 16-byte BigTIFF headers, 64-bit IFD counts, 20-byte entries,
8-byte inline payloads, LONG8/SLONG8/IFD8 types and 64-bit offsets/counts are
recognized. Entry payload multiplication and range checks use the verified
record API. Sparse-file tests place both directories and image data above
4 GiB. The later C default file limit admits such seekable files without allocating
their complete contents.
Memory is bounded by metadata and active segments, not the numerical file size.

Main-directory counting/selection detects cycles and respects the existing
1,024-directory limit. `TIFFGetField(TIFFTAG_SUBIFD, uint16_t*, uint64_t**)`
returns child offsets; `TIFFSetSubDirectory` selects an arbitrary valid image
IFD, including SubIFDs. Failed selections preserve the selected directory.
`TIFFReadDirectory` follows the selected IFD's next pointer, including a SubIFD
chain. `TIFFSetDirectory` returns to the main chain. EXIF/GPS pointer tags can
be read, but their nonimage directory readers remain stubs.

## Verification scope

All executable code in the three new core modules is inside `verus!`, uses
safe Rust and explicit bounds, and is checked with the entire existing crate
at `--no-cheating --rlimit 100`. Existing codec, raster, profile-acceptance,
resource and failure-frame contracts are preserved.

| Primitive | Checked contract |
| --- | --- |
| `container::header` | Exact Classic/BigTIFF header acceptance, byte order, variant and first-IFD value |
| `container::entry` | Exact local entry acceptance, type/count/length, inline/offset value, overflow and file-range bounds |
| `native::decode` | Exact stream acceptance plus sufficient budget/buffer equivalence; exact native bytes; output suffix preservation; exact charged quota |
| `native::work_limit` | Checked `8 * encoded_bytes + 3 * padded_capacity + 64` calculation |
| `native_samples::convert` | Exact layout acceptance, endian words, modular per-channel prefix sum with row resets, exact destination bytes and suffix frame |
| `native_samples::reverse_bits` | Every output byte equals the specified bit reversal |
| `native::jpeg_shared` | Exact bounded shared-header model and charged parser quota |
| `native::renumber_ojpeg_scan` | Exact checked SOS preconditions and replacement bytes; unchanged entropy/suffix; unchanged buffer on failure |

**There is no new end-to-end BigTIFF or C-handle acceptance theorem.** Seekable
I/O, directory traversal/selection, geometry assembly, metadata policies,
allocations, caching, codec selection, synthetic TIFF construction and RGBA
presentation are unverified. The new modules prove local records and supplied
segment/sample transforms; their composition with a TIFF file is an explicit
adapter boundary. The original whole-image acceptance theorem still covers
its original Classic TIFF profile, not every newly accepted C file.

Four new negative proof checks reject a disabled predictor, wrong BigTIFF
first-offset result, wrong SOS selector and an always-error native decoder.
They are additional to all 40 earlier negative checks. Runtime checks include
native source words, wraparound, row/channel resets, malformed records,
low budgets, bit reversal, output frames and scan-header relabeling.

## Integration evidence

The C suites now include 640 generated broader-format files, 544 of which are
also compared exactly to the pinned native libtiff consumer; 96 64-bit
Predictor-2 cases use independent expected bytes. They exercise both byte
orders, Classic/BigTIFF, separate/contiguous planes, strips/tiles and four
codecs. Further tests cover 32 RGBA16-alpha cases, 64 source/output orientation pairs,
16 oriented strip/tile cases including edge padding, 24 packed
FillOrder cases, nonzero edge/row padding, ordinary metadata, memory options,
SubIFDs, malformed/cyclic directories, sparse offsets above 4 GiB, callback
read tracking, failed-read buffer frames and standard `rC` chopping.

The callback fixture describes a 32 MiB raster with a valid first compressed
strip and corrupt subsequent strips. Its 8 KiB output limit and 512 KiB live
allocation limit allow opening and reading the first strip. Opening makes
zero pixel reads; the first read fetches only its compressed strip; a cached
scanline makes no additional pixel read; requesting a corrupt strip fails
without changing caller output.

The optional `compat/tests/pillow_smoke.py` forces an already installed Pillow
to use libtiff, interposes our library in isolated subprocesses and requires
loader bindings to our `TIFFClientOpen` entry point. It compares 12 files with
Pillow's normal decoder. This is an actual consumer smoke test, not a claim
that every Pillow mode or every libtiff deployment works.

See the current evidence logs and [C API](c-api.md) for remaining restrictions.
