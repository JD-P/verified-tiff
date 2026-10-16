# Continue this project

The working first version is intentionally limited to the profile in
`profile.md`. The parser acceptance theorem, sufficient work bound, segment
coverage lemma, PackBits/LZW stream/output proofs, and concrete witnesses for both
depths and all three compressions, plus alpha conversion and Gray+A/RGBA
witnesses for both alpha conventions, are now checked. Preserve
these guarantees as the project develops. The broader palette/planar/16-bit/
Deflate profile now also has complete inspection acceptance, checked extended
and public budget calculators, five concrete TIFF witnesses, and negative
extended/public inspector checks.

JPEG/OJPEG now has a separate sequential eight-bit profile, whole-image
acceptance and resource proofs, and independent decoder comparisons. Preserve
its specified integer-transform and nearest-chroma behavior. For future
OpenJPEG work, reuse bounds, geometry, API and proof patterns; JPEG 2000
entropy coding and wavelet transforms need new specifications.

CCITT Group 3/4 has separate byte/run/row models, exact output and whole-image
acceptance proofs. Preserve the missing-EOL and duplicate-endpoint policies in
`fax.md`; do not broaden damaged-stream recovery merely to match libtiff.

1. **Review the model against TIFF 6.0.** Check tag types/defaults, both byte
   orders, bilevel FillOrder/polarity and byte-padded rows, full padded edge tiles, short final strips, field order/range policy,
   PackBits row boundaries, LZW early width changes/dictionary updates/Clear/EOI,
   Deflate tree construction, zlib/checksum validation and bounded final-strip padding,
   strict trailing-data policies, ExtraSamples and alpha normalization/clipping, and main-directory
   selection, SGILog row/plane framing and native LogLuv encoding, float64 bit preservation and per-channel/row/tile prediction resets. The proof checks the model, so specification
   review remains essential. Compare disagreements with libtiff against the
   standard instead of automatically copying libtiff's behavior.
2. **Improve fuzzing.** The supplied deterministic mutation target is a smoke
   test, not a coverage-guided campaign. Add an instrumented libFuzzer target
   for inspection/decode and another for differential decoding within the
   supported profile. Preserve minimized regressions as fixtures. Assert
   resource bounds and output framing as well as absence of crashes.
3. **Continue measured optimization.** See `rgba-optimization.md` for the latest
   controlled results. Word-based LZW/Deflate reads, exact JPEG transform
   factorization, JPEG row mapping, tile-row copies, shared bilevel span geometry, alpha/native-layout
   shortcuts and contiguous Deflate strip writes are implemented. The opt-in
   combined Deflate API avoids replaying validated strips; its tiled fallback,
   JPEG/LZW pipelines and partial compressed C reads still do separate validation
   and decode. The C reader now batches streaming Deflate, stages complete native
   Deflate requests in one pass, and uses a proved top-bit refinement for LERC.
   Specialized, verified 8/16-bit RGB/gray rows, fused RGB alpha presentation,
   cached-segment borrowing and memory-admitted complete RGBA Deflate staging
   are now implemented. The earlier Deflate/LERC pass recorded two small-strip
   regressions; its original measurements remain in `reader-optimization.md`.
   Large RGB Deflate and LERC are substantially faster but still trail libtiff.
   Remaining work includes reusable Huffman prefix tables, bulk Deflate
   stored/match loops, packed bilevel expansion and adapter allocation/replay.
   Retain all semantic/acceptance contracts and benchmark each change: a tested
   JPEG speculative lookahead implementation was removed because it regressed
   textured images. Do not trade exact JPEG output for faster approximate math.
4. **Add codecs one at a time.** PackBits and LZW are implemented for bilevel, Gray8, and
   RGB8 with optional 8-bit alpha. Deflate now supports all current representations. CCITT Group 3/4 now supports bilevel scans with explicit compatibility rules.
   Float64 and its Predictor 2 are implemented. The single-group WebP lossless subset now covers the original opaque-alpha fixture. The native C path now includes predictor/color/palette transforms, entropy meta-images, lossy VP8 and ALPH with separate checked models. The native C path now supports integer Predictor 2 and float32. Predictor 3 and byte-aligned CCITT RLE now have separate verified native APIs used by the C reader. Dictionary-free ZSTD now has a verified native segment API with full block/literal/FSE modes and checksum validation. Lerc2 v4 and static VP8/VP8L now have verified native APIs. A subsequent task can integrate these native formats into the complete whole-image acceptance model. Give each codec its own input/output relation,
   termination argument, strict output limit, success theorem, truncation
   tests, and differential corpus. Predictor support is also a separate
   specification, not an implicit addition to a compression tag. Do not
   import unsafe codec implementations under a "verified"
   claim.

Use `bash scripts/verify.sh` as the proof gate. Ordinary `cargo test` is not a
substitute. Changes to the core must verify the whole crate, continue rejecting
the negative proof example and the always-error parser mutation, and pass the supported-image matrix and malformed
input tests. Keep toolchain pins, source hashes, proof scope, and evidence
consistent. The executable code verified by Verus must remain the code Cargo
builds; no proof-only surrogate implementation or new trust escape is allowed.

All 31 original first-main images now decode. SGILog32 `logluv-3c-16b.tiff`
returns exact native LogLuv16 components. The C reader also exposes XYZ float32 through an unverified numerical adapter; the original CLI remains native16. Display RGB conversion, LogL and SGILog24 remain future work. `quad-lzw-compat.tiff` now decodes exactly through
legacy LSB packing, late width changes and the bounded frozen dictionary.
`lzw-single-strip.tiff` now decodes exactly under the specified LZW sub-byte
termination exception. Preserve its exact-output condition and both negative
checks; do not broaden this into stopping at arbitrary trailing data.

The partial reader-oriented C API is implemented; see `c-api.md` and
`consumer-compatibility.md`. Typed custom metadata, EXIF/GPS directories and a
metadata-only temporary writer are now available. Future adapter work includes
deferred strile-array loading and broader codec subsets. Exact indices/associated samples/tile padding, independent
segments, BigTIFF, SubIFDs and broader native widths are now implemented through
new verified primitives; see `native-compatibility.md`. Broader ABI support and formal verification of the
FFI/metadata/output adapter remain separate tasks. Keep current ENOSYS failures
explicit until their actual contracts are implemented and checked.


The subsequent reader pass is documented in [reader-features.md](reader-features.md):
read extension APIs, lazy strile arrays, bounded generic-codec reads, additional
scientific samples and cropped RGBA are implemented. Remaining reader work
includes bounded streaming for the specialized codecs, custom RGBA callbacks,
mixed per-channel sample layouts and unsupported display/compression profiles.
The public LogL/LogLuv numerical decoding helpers now work; that does not add
LogL or SGILog24 TIFF image decoding.
