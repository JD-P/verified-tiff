# Read-side APIs, bounded reads, scientific samples and crops

This pass addresses the next three reader priorities: read-side API compatibility,
large-image allocation behavior and scientific native sample types. It also fixes
cropped RGBA reads. It does not change consumer programs that bypass libtiff to
call another decoder directly, add image encoding, or broaden the original
whole-image `Image`/`Raster` acceptance theorem.

## Read-side API compatibility

The public inventory now contains 170 implemented/restricted functions and 28
explicit ENOSYS stubs, out of the same 198 pinned libtiff 4.7.1 header symbols.
Twenty-five former stubs now have read-side implementations:

| Family | Implemented behavior |
| --- | --- |
| Extension hooks | `TIFFAccessTagMethods` returns live get/set/print methods. Wrappers can delegate to the saved methods. Image-directory selection reinstalls defaults before invoking the tag extender. |
| Client information | `TIFFGetClientInfo`/`TIFFSetClientInfo` store named caller pointers. Names/nodes are owned and allocation-accounted; caller data is never freed. |
| Buffers | `TIFFReadBufferSetup` uses a positive-size caller buffer for actual I/O staging, or allocates an owned buffer when passed NULL. Borrowed buffers are never freed and must remain valid until replacement/close. Size zero/negative fails. |
| Directory state | `TIFFUnsetField` clears tag presence and converted/cached values; internal geometry remains until directory reselection. `TIFFFreeDirectory` releases selected metadata and decode buffers while retaining directory navigation state. |
| Custom directories | `TIFFReadCustomDirectory` reads non-image IFDs, including typed custom schemas and anonymous fields. A NULL schema requests anonymous field conventions. |
| Codec discovery | `TIFFFindCODEC` and `TIFFGetConfiguredCODECs` describe configured readers. The returned allocated list is freed with `_TIFFfree`. The init function validates the selected compression; external codec registration remains unsupported. |
| Printing | `TIFFPrintDirectory` prints known/custom tag values, optional strile arrays and the registered print hook. Its text format is not byte-for-byte libtiff output. |
| Handle setters | File-name, client-data, file-number and mode setters return the old value. File names are borrowed. Client-data changes redirect client callbacks and invalidate decode state. File-number changes do not transfer descriptor ownership; changing the mode value does not enable unsupported writing. |
| Numerical color helpers | Lab/XYZ/RGB and YCbCr initialization/conversion; `LogL16toY`, `LogL10toY`, `LogLuv24toXYZ`, `LogLuv32toXYZ`, `uv_decode`, `XYZtoRGB24`. |

`TIFFFieldArray` is opaque publicly. Non-NULL custom schemas use the pinned
libtiff 4.7.1 private field-array ABI, with scalar and counted-array conventions;
arbitrary private layouts and custom RGBA get/put callbacks are not supported.
A failed custom-directory parse preserves the previously selected directory and
rolls back newly allocated field definitions. Successful replacement discards
stale custom-directory definitions. Field pointers follow directory lifetime.

Color helper implementations adapt the pinned libtiff numerical routines, with
license notices retained in `compat/color_api.inc`, `compat/logluv_api.inc` and
`compat/uv_table.inc`. They are explicitly unverified presentation code and do
not decode compressed payloads. Native libtiff/codec libraries remain test-only.

## Large-image memory behavior

Offset/count tables are deferred for both Classic TIFF and BigTIFF, regardless
of `rD`/`rO` hints. Individual strile access loads at most 256 table entries per
page. Generated strip-chop arrays also remain implicit. Pointer-returning
`TIFFGetField` array getters materialize the full requested array, subject to the
normal limits; allocation failure leaves the caller pointer unchanged.

Uncompressed ranges seek directly to the requested rows. Large PackBits, LZW
and Deflate strips/tiles use a 64 KiB input window and fixed codec state rather
than encoded/decoded segment-sized allocations for partial requests. A complete
validation pass checks expansion, strict endings and the Deflate checksum before
exposing requested rows; a second, resumable pass supplies those rows. Sequential
reads retain progress; backward access or switching segments resets it. A full
native Deflate segment request already requires a private segment-sized result
buffer: it now decodes into that buffer and validates the ending/checksum once,
then returns it. Failures still leave application storage unchanged. Conservative
validation-plus-replay quota reservations remain unchanged.

Native scanline reads and encoded-strip/tile prefixes allocate requested rows
rather than the complete segment. Raw prefix reads fetch only the requested
bytes after checking the declared segment range. Predictor/endian transforms
operate on complete rows; a very wide row can still exceed configured limits.
RGBA rendering uses the requested raster and row/color-band scratch.

This is bounded **memory**, not constant-time random access: compressed prefixes
still validate the entire strip/tile. There is one active stream per handle,
so alternating separate planes can replay validation. Small segments retain a
cache. JPEG/OJPEG, ZSTD, LERC, WebP, fax and LogLuv retain complete-segment decoder
scratch, bounded by the existing limits. An exceptionally large segment in
those formats can still require raising limits or rechunking the TIFF.

## Scientific native samples

| Representation | Behavior |
| --- | --- |
| Packed unsigned/signed integers | Widths 1–64; rows remain packed, with stored unused bits preserved. |
| Ordinary word widths | 16/24/32/64-bit native word transport uses host byte order; other packed widths retain the stored byte sequence, matching libtiff's postprocessing. |
| IEEE floating point | 16-, 24-, 32- and 64-bit samples are transported as exact bits, including signed zero and NaN payloads; no narrowing or floating arithmetic. |
| Complex samples | SampleFormat 5/6 pair widths 16/32/64/128; endian reversal is per component, preserving real/imaginary order. Oracle coverage includes complex integers 32/64/128 and complex floats 64/128. Nonstandard tiny component formats are opaque bit transport. |
| Predictor 3 | IEEE16/24/32/64 with LZW, Deflate and ZSTD, retaining exact per-row/per-channel shuffle and accumulation. |
| Predictor 2 | Existing non-complex 8/16/32/64-bit modular prediction, with LZW/Deflate/ZSTD. Packed and complex Predictor 2 remain unsupported. |

These additions apply to generic native strip/tile/scanline transport, including
contiguous/separate planes and uniform channels. Mixed per-channel widths or
formats and display conversion of signed, floating or complex samples remain
unsupported. LERC and image-specific codecs retain their own sample profiles.

## Cropped RGBA reads

`TIFFReadRGBAImage[Oriented]` accepts positive cropped widths/heights, with width
at most the image width. `TIFFRGBAImageGet` also accepts bounded nonzero column
and row offsets. Raster placement follows libtiff's requested orientation,
including its flip aliases for orientations 5–8. A height greater than available
rows leaves leading unused raster rows untouched; edge-tile padding remains zero.
Failure leaves caller pixel storage unchanged. The documented independent
Gray16/Gray+A edge reference still avoids libtiff's `fromskew` display bug.

## Formal boundary and evidence

`native_stream` contains allocation-free Verus refinements of existing packet,
LZW dictionary/bit and Deflate transition models. They prove exact local
acceptance, output/phrase correspondence, dictionary/history transitions,
checksum-completion semantics and step work conservation. `swap_components`
proves component byte order and suffix preservation; Predictor 3's proof now
includes two- and three-byte components. All original whole-stream/image
contracts remain intact. There is no new end-to-end theorem for input-window
continuity, lazy I/O, C hooks or crop composition; those remain tested adapter
code. Four new negative gates cover step/phrase/packet/permutation corruption.

The Deflate batch API additionally proves exact equivalence to a bounded sequence
of the existing transitions, output bytes, checksum completion, suffix preservation
and two work units per transition (the former adapter loop plus core step).
Input-window binding now occurs once per batch. Partial reads keep their complete
validation-before-exposure policy. New negative gates reject corrupt batch bytes,
incorrect charging and an always-error batch; the LERC top-bit refinement has its
own corruption gate. See [reader-optimization.md](reader-optimization.md).

The subsequent [RGBA pass](rgba-optimization.md) specializes contiguous
8/16-bit RGB/gray conversion and borrows cached native segments. A fully covered
Deflate segment may use private one-pass staging when it is at most 1 MiB and
the conservative scratch admission check fits both allocation limits. Otherwise
the row replay path remains. Crops, padding, orientation and alpha presentation
retain their previous pixels; all caller output remains unchanged on failure.

`compat/tests/reader_features.py` adds 12 test methods, using independent bytes
and a consumer compiled against both implementations. Cases include packed
widths, half/24-bit/complex samples, predicted streaming with both FillOrders,
200,000-entry strile tables, malformed complete streams, tag/client/buffer hooks,
custom directories, numerical color helpers, orientation/crop matrices and large
color crops. Bounded-read cases impose 128 KiB single allocations, 384 KiB total
live allocations and 128 KiB output buffers while reading larger segments.

Fresh full proof/test/oracle/sanitizer/fuzz results are recorded in
`docs/evidence/reader-summary.json` and the `reader-*` logs. C sanitizers cover
C code and foreign-buffer accesses; Rust is not sanitizer-instrumented.
LeakSanitizer is unavailable under this runner's ptrace setup and is disabled.
