# Reader adoption status

This archive completes the requested reader additions, including the remaining
LERC and broader static WebP work. Consumer binaries' independent JPEG decoding
paths are outside this work.

| Priority | Delivered | Remaining restriction |
| --- | --- | --- |
| 1. Optional metadata | Quiet absent-field queries, standard TIFF/EXIF/GPS field descriptors, host-typed values, anonymous tags | Unsupported present special conventions still fail explicitly |
| 2. Resource configuration | Seekable files without the old 64 MiB file ceiling; separate output, work, single-allocation and live-allocation budgets; environment and per-open settings | Limits are per handle/read, not process-wide deadlines |
| 3. Read-side setters | SampleFormat and other supported interpretation/custom fields; JPEG RGB; LogLuv float XYZ and native16 output | Reader geometry mutation and on-disk tag editing remain unsupported |
| 4. RGBA conversion | CMYK8/16, CIELab8/16, separate planes, non-JPEG YCbCr8 sampling clumps | No ICC transform; numerical presentation is in the unverified adapter |
| 5. Custom/nonimage directories | Tag extenders, legacy field registration, typed GeoTIFF fields, EXIF/GPS directory selection | No private libtiff structure compatibility |
| 6. Additional codecs/prediction | CCITT RLE, IEEE32/64 Predictor 3, dictionary-free Zstandard, Lerc2 v4 with all three wrappers, static VP8/VP8L and alpha | No codec encoders or animated WebP; precise native proof boundaries in [LERC](lerc.md) and [WebP](webp-native.md) |
| 7. GDAL read machinery | TIFFReadFromUserBuffer and metadata-only callback writing, WriteCheck, WriteDirectory and Flush; Classic/BigTIFF, both byte orders | No pixel encoding or general file writer; tested API sequence, not an installed GDAL |

The current source also fixes explicit option precedence over malformed
environment settings, makes the compatibility suite build its Rust comparison
binary on a clean checkout, and uses a separate upstream LogLuv test oracle.
CMYK16 and planar Lab now receive pixel comparisons rather than success-only
checks. The installed ImageMagick smoke test verifies actual reader-symbol
binding to the replacement library.

## Validation on this source

The authoritative codec receipt is `evidence/codecs-summary.json`; logs use the
`codecs-` prefix. It records fresh whole-crate verification, all prior and new
negative gates, the complete runtime/C suites, original corpus, mutation tests
and C ASan/UBSan results. See [validation](validation.md) for counts. Earlier
`adoption-*` logs retain the installed-consumer comparisons from the preceding
checkpoint. The 108 upstream tests exercise libtiff itself.

The adapter still exports 198 public functions: 145 have implemented/restricted
behavior and 53 remain explicit stubs in that historical codec delivery.
The subsequent [reader pass](reader-features.md) raises this to 170
implemented/restricted functions and 28 explicit stubs. Codec expansion changes the supported
reads behind those functions. LERC and libwebp are independent test oracles;
production decoding uses the verified Rust engines.

The verified core remains allocation-free and does not call native codec
libraries. The C ABI, metadata assembly, temporary writer, allocation accounting,
and display/XYZ conversion remain outside the formal proof. Existing whole-image
profile theorems are preserved; this delivery does not claim an end-to-end
C/BigTIFF theorem or full libtiff compatibility. No new performance result is
claimed.
