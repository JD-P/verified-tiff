# Working on verified-tiff

Read `README.md`, `docs/profile.md`, `docs/verification.md`, and
`docs/trusted-boundary.md` before changing the decoder. The full inspection
acceptance theorem is now checked against `supported_profile`; preserve it.
The next tasks in `docs/next-steps.md` concern model review, stronger fuzzing,
performance, and subsequent codecs.

The executable parsing and decoding implementation is `src/lib.rs`, inside
`verus!`. The OS/CLI adapter is `src/main.rs`. Keep the core allocation-free,
input-borrowing, and explicitly bounded. Do not add runtime recursion,
unchecked arithmetic, implicit allocation, or an FFI decoder to the core.

Use the pinned versions in `toolchain-lock.json`, `Cargo.lock`, and
`rust-toolchain.toml`. Bootstrap with `python3 scripts/bootstrap.py`, then:

```bash
bash scripts/verify.sh
python3 scripts/upstream.py
bash scripts/test.sh
bash scripts/test-compat.sh
bash scripts/fuzz-smoke.sh 20000
```

Formal contracts are part of the implementation. Verify the entire crate;
ordinary Cargo compilation does not run proofs. Never add `assume`, `admit`,
external proof bodies or project axioms outside the explicitly authorized
`trusted-float` and `trusted-simd` boundaries, or configuration tricks that cause
verification to see a different implementation. Keep the intentionally false
contract failing for its stated postcondition and the always-error parser
mutation failing its acceptance contract. Preserve meaningful semantic
and acceptance claims when refactoring; memory safety alone is not the
project's end goal.

Use independent expected pixels and the separately built libtiff oracle.
libtiff is only a test dependency. Its full C suite runs against libtiff,
while 22 unchanged originals and nine transcoded upstream fixtures, plus
four-codec generated profile matrices, exercise this reader. Do not confuse
these claims. The smoke fuzzer is deterministic
mutation testing, not a coverage-guided fuzz campaign.

After relevant changes, update the profile, proof ledger, tests, and evidence
together. Evidence is a recorded run, not a substitute for running the current
source. Keep imported fixture provenance and licenses. New codecs and broader
TIFF support require explicit specifications and additional proof boundaries.
The optimized uncompressed row copies and full-width scatter retain the original
pixel, buffer-frame and conservative budget contracts. Preserve the bulk-copy
and shortcut-index negative checks, scalar fallbacks, and insufficient-budget
partial-prefix behavior. Benchmark against a freshly built baseline on the same
host; keep before/after outputs exact, including JPEG. See docs/optimization.md.
The JPEG/Deflate profiling pass is documented in docs/profiling.md. Preserve
the fixed Huffman cache refinement and aligned-byte equivalence, including
both negative shortcut checks. Constant/table borrows must keep the same
integer IDCT, entropy, framing and conservative resource contracts.
PackBits has a stream validator, a constant-size cursor, and verified scatter
mapping for packed bilevel and 8-bit samples. Preserve its exact-expansion and
row-boundary rules. Callers must use `decode_work_limit()` for compressed or alpha
decode budgets; `output_len()` remains the output-buffer size.

LZW uses a pure byte-string dictionary model refined by a fixed prefix table
and phrase buffer. Preserve modern TIFF early-change widths, legacy LSB packing/late-change widths, initial Clear, exact
expansion, the fixed dictionary bound, and strict trailing-byte policy. EOI
is required except for the exact-output, sub-byte ending immediately after
a data code documented in docs/profile.md. Preserve that explicit compatibility
predicate, its witness, both negative checks, and tests/lzw_tail.py; it must
never permit missing pixels, a trailing Clear without EOI, or whole junk bytes.
Legacy selection is per segment; its full dictionary freezes at 4096 entries.
Modern full-table data still requires Clear. Preserve tests/lzw_legacy.py, its
independent references and all three legacy negative checks.
The codec entry predicates are opaque to keep inspection's solver context
small; reveal them only in their own verified helpers. Predictor 2 for float64 is implemented separately in `float64.rs`; integer predictor support remains excluded from that original whole-image profile.
The separate `native_samples` API now proves integer/signed/float word prediction;
see docs/native-compatibility.md for its C integration and narrower proof boundary.

Alpha supports 8-bit Gray+A/RGBA with one trailing ExtraSamples=1/2 channel.
The output is straight alpha: preserve alpha bytes, unassociate with nearest
rounding/clipping, and handle associated zero-alpha colors explicitly. Keep
`storage_pixel` versus `output_pixel` semantics and the in-place conversion
proof. The CLI uses PAM for alpha; do not silently drop transparency. The
alpha oracle returns native samples, with an independent rational reference
for normalization. Alpha decode reserves an additional output-length pass.

The CLI uses `inspect_raster`/`Raster` for palettes, planar data and unsigned
16-bit grayscale/RGB. Preserve the original `inspect`/`Image` API and acceptance
proof. New core modules are verified with the entire crate. Read
`docs/extended-formats.md` before editing their geometry or caller-scratch
pipeline; it states the full extended and public-raster acceptance theorems
and their checked sufficient-budget calculators. Preserve these contracts,
the concrete witnesses, and the always-error negative inspector checks. Include tests/extended.py and tests/corpus.py in relevant
validation. Use native_oracle.c for full-precision/separate-plane comparisons.

Deflate is specified in docs/deflate.md and implemented entirely inside Verus.
Keep its pure byte-string state-machine model, fixed 32 KiB ring refinement,
checksum validation, exact stream acceptance, and output/scatter proofs.
Compression 8/32946 uses zlib wrappers and Predictor/FillOrder 1. Preserve the
bounded short-final-strip padding policy and reject bytes after the checksum.
Run tests/deflate.py and the original 500x500 fixture. Preserve the original
inspection theorem and the separately checked read_geometry layout helper.

JPEG/OJPEG is specified in docs/jpeg.md. Preserve byte-level entropy/header
acceptance, whole-image supported_jpeg_profile acceptance, its concrete witness,
integer IDCT/color/scatter semantics, buffer frames, and the JPEG negative checks.
The numerical proof is relative to the explicit fixed-point transform, not
real-cosine conformance. Generated references use independently decoded original
JPEG segments; original corpus references use libtiff native components with
matching nearest chroma. Do not conflate bounded JPEG error with exact lossless
pixels. Include tests/jpeg.py, jpeg_api.rs and corpus.py in validation.

CCITT Group 3/4 is specified in docs/fax.md and implemented in fax*.rs. Preserve
the full supported_fax_profile theorem and public union, bounded code/run/row
models, both FillOrders, exact pixels and scratch frames. Missing-EOL recovery
is restricted to pure 1D T4Options=0 selected before decoding. Keep duplicate
2D endpoints, 1D zero-pair normalization and strict row/end-marker policies.
Do not use bitmap-only reference rows or add damaged-row filling. Run fax.py,
fax_api.rs and original corpus comparisons; preserve the two profile witnesses
and four fax negative gates as well as every earlier gate.

Float64 extends the complete extended profile; read `docs/float64.md` before
changing it. Preserve bit-exact little-endian IEEE output, per-channel modular
prediction and row/tile resets. Do not replace integer bit transport with
floating-point arithmetic or silently narrow the output. Keep SampleFormat
validation, native scratch limits, the two witnesses, three float negative
gates, NPY adapter tests, float_api.rs/float64.py and original corpus checks.

Lossless WebP is the explicit single-group VP8L subset in docs/webp.md.
Preserve supported_webp_profile and public union acceptance, shared validation/
decode semantics, zero-bit Huffman leaves, previous-nonzero code-length repeat
rules, plane distances, overlapping copies, and cache hashing/update order.
Preserve the caller-scratch and exact termination policies, inverse green,
straight-alpha conversion and RGB compaction. The WebP alpha hint never changes
TIFF's requested output channels. Keep the concrete witness and all five WebP
negative checks. Run webp.py/webp_api.rs and the unchanged original comparison.
Libwebp is test-only. Broader transforms or entropy grouping need new explicit
models and resource bounds, not an unverified decoder dependency.

SGILog32/LogLuv16 is specified in docs/logluv.md and implemented in logluv*.rs.
Preserve exact plane/row expansion, zero-only trailing no-ops, integer native
component semantics, the explicit ColorModel, and little-endian NPY output.
Do not present encoded native components as RGB or linear signed luminance.
Keep supported_logluv_profile, the public six-profile union, exact budgets,
caller-scratch frames, the original 166-byte witness and all five LogLuv
negative gates. Run logluv.py/logluv_api.rs and corpus.py. Independent libtiff
encoding/decoding and rational Python conversion are test-only dependencies.

The optional C API is in `compat/`, an ordinary Rust workspace member and C
shim, deliberately outside the verified core. Read docs/c-api.md before edits.
Foreign-pointer/varargs operations belong only there; never move unsafe or
unverified adapter behavior into the core. Unsupported APIs must report
ENOSYS with a failure sentinel and preserve caller output. Regenerate/audit
every public header symbol; keep standard symbol versions and private bridge
symbols hidden. Keep the complete pinned headers and their upstream license.
Do not infer full compatibility from exported symbols or from upstream's own
108 tests. Run scripts/test-compat.sh and its ASan/UBSan variant for boundary
changes. Preserve native-data restrictions rather than inventing samples lost
by core normalization. Do not install over the host's system libtiff.

The v20 throughput changes are documented in docs/throughput.md. Preserve the
word-reader, transform, native-copy and combined-acceptance negative gates.
The combined Deflate API is opt-in: strips validate/decode once, tiles retain
the original fallback. Preserve its supported-profile acceptance, sufficient
budget calculator, exact pixels and failure-atomic output (scratch may be
partial). Keep all original inspect/decode guarantees and conservative quotas.

The five C reader additions are documented in docs/native-compatibility.md.
Preserve native::decode exact acceptance/bytes/budget, native_samples word,
prediction, row/channel reset and suffix-frame contracts, and container local
Classic/BigTIFF header/entry acceptance and source/range correspondence.
OJPEG shared-header/SOS relabeling is checked, but synthetic container assembly
is not. Keep all four reject-native checks and native_api.rs. Do not claim
end-to-end verification of the BigTIFF/C pipeline or broaden original Raster
acceptance claims. Preserve seekable segment isolation, cumulative/single
allocation accounting, read failure atomicity and exact stored native samples.
Run both C suites including coverage.py and sanitizer variants; its generated
fixtures independently specify native words, padding, prediction and BigTIFF.
The optional Pillow smoke test requires an installed Pillow and confirms actual
loader bindings. Keep sources, API inventory, proof ledger and delivery evidence
consistent.

Native Zstandard is specified in docs/zstd.md. Preserve dictionary-free frame,
FSE/Huffman, history, checksum, exact acceptance/work and suffix-frame contracts.
Keep its raw witness, all six reject-zstd checks, zstd_api.rs, zstd.py and the
C integration matrix. Native dependencies remain test-only. Do not broaden
original Raster acceptance or introduce permissive checksum/trailing-frame
handling.

Native Lerc2 v4 and full static WebP use the separate boundaries in
docs/lerc.md and docs/webp-native.md. Preserve exact native acceptance, pixel,
checksum, work and suffix contracts, the exact IEEE bit models (now using the documented trusted hardware primitives
for finite arithmetic), VP8 integer
transforms/filters, all VP8L transforms and entropy groups, and ALPH filters.
Keep all twelve reject-codecs gates and every earlier gate. Native LERC and
libwebp remain test-only; preserve their provenance notices. Run the native
and C matrices, malformed-input checks and C sanitizer variant. Never broaden
the original Raster theorem to claim end-to-end C integration verification.


Read-side extensions are specified in docs/reader-features.md. Preserve lazy
strile metadata, allocation accounting, failure-atomic cropped/partial reads,
resumable validation before exposure, and the original strict codec endings.
`native_stream` proves local packet/dictionary/Deflate transitions; it does not
verify adapter input-window continuity. Preserve all four reader negative gates,
component permutation/suffix contracts, IEEE16/24 Predictor 3 proofs, and
compat/tests/reader_features.py in ordinary and C sanitizer runs. Keep complete
segment fallback limitations explicit. Numerical public color helpers remain
licensed, unverified C presentation code, not a native codec dependency.

Deflate/LERC optimization is documented in docs/reader-optimization.md.
Preserve LERC's top-bit equivalence to the original software IEEE model,
Deflate batch exact acceptance/state/bytes/frame/work contracts, all four
reject-reader-optimization gates and the scalar API. Complete native Deflate
segments may decode once only while output is private until checksum/ending
validation. Partial reads retain validation-before-exposure and bounded replay.
Keep conservative quotas, tests/stream_batch.rs, the independent zlib fixtures,
numeric boundary tests and C failed-read buffer checks. Before/after benchmarks
must require exact output bytes including JPEG and current full-crate proof hashes.

RGBA optimization is documented in docs/rgba-optimization.md. Preserve
native_rgba's exact acceptance, 8/16-bit component and alpha rounding, byte order,
reversal, failure atomicity and suffix contracts, plus all four reject-rgba
checks. Keep grayscale alpha semantics distinct from RGB premultiplication.
Adapter cached-segment borrowing and optional complete Deflate staging must
retain allocation accounting, the 1 MiB admission cap, conservative quotas,
bounded crop/tight-memory fallback and unchanged caller output on any failure.
Run rgba_rows.rs, the expanded C reader matrix and its sanitizer variant;
compare every before/after byte including JPEG before reporting speedups.

Kernel optimization is documented in docs/kernel-optimization.md. Preserve the
Deflate match-run transition/output/work/frame refinement, LERC rounding and
normal narrowing equivalence, VP8 filter/DC transform and VP8L coordinate
models, Predictor 3 coordinates and all eight reject-kernel gates. Packed RGBA
lookup tables and Lab gamma tables remain unverified adapter presentation code.
LZW complete staging has the same private-output, conservative-budget and 1 MiB
optional RGBA admission rules as Deflate. Partial reads still validate endings.
Run the packed crop matrix, exhaustive VP8 DC oracle and LERC tie/carry tests;
retain exact before/after comparisons including JPEG.

LERC word arithmetic is documented in docs/lerc-arithmetic.md. Preserve the
original scaled/sum/packed/rounded/narrowed models, align_word/combine_words and
packed_jam refinements, post-cancellation guard and wide fallback. Normal
packing must retain ties, carry and overflow semantics. Multiplication and
addition round separately before typed conversion; do not silently fuse them.
Keep all five reject-lerc-arithmetic gates and earlier numeric gates. Run the
hardware exponent/gap/cancellation/product tests, native and C LERC matrices,
full-crate verification and exact previous/new outputs before claiming gains.


Hardware floating point is explicitly authorized by the user (2026-10-15).
Read docs/native-float.md. Five scalar operations in trusted-float/src/lib.rs
are trusted; all decoder dispatch, acceptance, bounds and output contracts
remain verified. Preserve the exact bit models and exceptional-value fallback.
Never describe the primitive bodies as verified or claim --no-cheating: that
flag rejects imported trusted contracts too. Run the source audit with its
reviewed trusted-float/audit.json, full-crate verification, all prior negative
gates plus six native-FP gates, software-reference differential tests, native/C
LERC matrices, C sanitizers, and exact before/after output comparisons. Keep
the pinned boundary source and imported metadata hashes in proof evidence.

LERC throughput is documented in docs/lerc-throughput.md. Preserve the five-byte
word reader's equivalence to zstd_bits::value (including backward tail windows),
in-place unpacking, coordinate and float-loop refinement of finish, conservative
mask shortcut, all five reject-lerc-throughput gates, and tests/lerc_word.rs.
The hardware trust boundary is unchanged. Run full verification, native/C LERC
matrices and sanitizers, and exact before/after comparisons before claiming gains.

SSE2 is explicitly authorized by the user (2026-10-15). Read docs/webp-simd.md.
Eleven fixed-size integer operations and CPU dispatch live in trusted-simd, with
reviewed source hashes. Intrinsic bodies are trusted, not proved. Preserve the
scalar fallback, forced-scalar tests, exact wide transform models, full-crate
proof and all prior gates plus the SIMD semantic/precondition gates. No codec
parsing, pointers, buffer lengths, entropy decisions or allocations belong in
that boundary. CPU dispatch uses x86-64 baseline, x86 runtime detection, and
scalar elsewhere. Do not claim ARM execution testing from a forced-scalar x86
build. Record both imported boundaries and current source hashes in evidence.

The adapter proof extension is documented in docs/adapter-proofs.md. Preserve
all five adapter_* modules and their live production call sites. Keep exact
schema/count acceptance, integer value preservation, crop placement, window
source-position conservation, bounded accounting and MaybeUninit output
initialization. Do not reintroduce C float-to-index casts or a plain byte slice
of uninitialized caller output. Retain all ten adapter negative gates and the
Rust/C boundary tests. No end-to-end C proof or ownership/event completeness
is claimed; keep the residual boundary explicit in evidence and release notes.

The C shim hardening is documented in docs/c-shim-hardening.md. Preserve the
failure-atomic field registration, channel-specific Lab steps, zero-product
allocation behavior and bounded pointer extents. Run scripts/test-c-coverage.sh
for C changes: every emitted C function, line and branch must be covered, with
no exclusion list. Preserve the separate real-backend integration and fault-
injection unit evidence, allocator ledger and sanitizer run. Coverage is not
formal verification or complete path coverage. Do not relabel earlier Rust
proof receipts as fresh runs; unchanged source hashes permit explicit reuse.


Preserve the metadata consistency rules in docs/metadata-contracts.md: generated
schemas must account for pinned codec field declarations, known tags must never
fall back to anonymous argument shapes, and InkNames has a single char ** getter
despite its counted setter. Exact-payload NUL validation and counting are verified;
image-directory publication must derive NumberOfInks atomically and reject a
conflicting count setter. Keep the three ink negative gates and independent
metadata ABI/normalization regressions in compat/tests/metadata_contracts.py.
