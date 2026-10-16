# Trusted boundary

The [adapter proof extension](adapter-proofs.md) moves local metadata, geometry,
window, accounting, output-initialization and color-index decisions into checked
executable code. It adds no project assumptions. C pointer validity, ownership
event completeness, callbacks and whole-pipeline composition remain unproved.

The [WebP SSE2 boundary](webp-simd.md) adds eleven fixed-size integer lane contracts and CPU feature dispatch. Their intrinsic bodies and dispatch are trusted, not proved. Decoder composition and buffer/work contracts remain verified. This supersedes earlier no-new-trust descriptions for these WebP kernels; see the exact operations, fallback and audit.


The proof relies on the pinned Verus implementation and encoding, bundled Z3,
Verus's erasure/lifetime checking, the relevant upstream `vstd` and Rust
standard-library specifications, Rust 1.98.1 compilation, LLVM, and the target
machine. This is SMT verification, not a separately kernel-checked Lean proof
or a verified compiler chain. Cargo dependencies and release archives are
version/checksum pinned; the optional test-build CMake fallback is version-pinned.
Imported contracts can still contain mistakes.

Decoder sources contain no axioms, `assume`, admitted proofs, unchecked external
contracts, `external_body`, or unsafe code. The separately audited
`trusted-float` dependency adds five explicitly assumed scalar hardware
arithmetic/conversion contracts. Its bodies are not verified. See
[native floating point](native-float.md) for exact operations, bit-model
correspondence and required execution environment. The full-crate verifier run
omits `--no-cheating`, which also rejects imported assumptions. A source audit
rejects proof bypasses in decoder code and pins both the FP and SIMD boundary
sources. The SIMD dependency adds eleven integer operation contracts and CPU
dispatch, as described above.
Source audit and verification are complementary; neither proves the trusted
bodies or the hardware assumptions.

`native_rgba` adds checked, allocation-free 8/16-bit RGB/gray row presentation
with exact acceptance, pixels, alpha rounding, endianness, reversal, failure
atomicity and suffix framing. Inlining specializes ordinary safe Rust; it adds
no trusted primitive. The compatibility adapter selects rows, borrows segment
buffers and checks admission for optional full Deflate staging. Those decisions
remain outside an end-to-end proof. The final output initialization loop is now
checked; its FFI pointer-validity preconditions remain caller obligations.

The optimized uncompressed row copy uses safe slice splitting and
`copy_from_slice`, relying on their upstream `vstd`/standard-library contracts
within the existing trusted boundary. Row mapping, inversion, exact output,
buffer frames and the full-width scatter shortcut are checked in the full
crate. No project-defined trusted bulk-copy primitive or new axiom is added.

`Image::decode_into`'s formal precondition is the descriptor invariant. The
safe public Rust API obtains descriptors only through `inspect`, keeps their
fields private, and borrows the immutable source. Unsafe callers that forge
values or violate Rust's aliasing rules are outside that contract. Independent
objects do not share state. No ambient I/O, callbacks, or codec FFI occurs in
the core. LZW dictionaries and phrase buffers are fixed-size arrays; their
byte-string models are erased ghost state. No external codec contract is added.

LZW's exact-output sub-byte termination exception is part of the explicit
mathematical profile, not an unverified fallback. It changes accepted framing
without weakening the exact pixel or resource contracts. Acceptance does not
certify a complete EOI for these compatibility streams. The test-only Python
string dictionary and libtiff oracle independently compare the original
24,335,990-pixel regression. The RGBA oracle permits up to 33,554,432 pixels
(128 MiB of RGBA storage) to handle that scan; production limits are unchanged.

Legacy LZW is likewise an explicit verified model extension: LSB packing,
late width changes and a frozen 4,096-entry dictionary. Independent Python
string dictionaries and libtiff compare the unchanged original and generated
cases. The model intentionally accepts continued frozen-table references past
libtiff's finite extra-entry capacity; this is a documented profile choice,
not a universal equivalence claim for all libtiff inputs.

The CLI and development scripts are ordinary Rust/Python/shell. Allocation
uses fallible reservation and checks declared limits before growth. Its input
read loop also has a fixed 64 KiB temporary buffer. The core budget excludes
OS reads/writes, allocation overhead, filesystem behavior, and scheduler time.
Callers choose limits appropriate to their application; a generous finite
budget can still be expensive. Error output, CLI JSON, and output-file handling
are tested, not formally proved.

The libtiff oracle is a separate executable used only by tests. Its C code and
codec dependencies are not linked into the Rust decoder and are not covered by
the Verus proof. Alpha tests use a second oracle that returns native samples through
`TIFFReadEncodedStrip`/`TIFFReadEncodedTile`; an independent rational-arithmetic
reference normalizes them. This avoids libtiff RGBA rendering changing the
alpha convention. The production decoder never calls either oracle.
Its behavior is an independent comparison, not the definition
of TIFF correctness. Copied upstream fixtures retain their original license.

The extended core is in `extended.rs`, `extended_image.rs`, `raster_layout.rs`,
and `reader.rs`, all verified in the same full-crate run. The proof-only
`extended_witness.rs` establishes concrete members of the extended profile. Caller-owned scratch
is bounded separately by max_output and its composition is proved. The new
`native_oracle.c` remains an unverified test dependency, not runtime code.

Deflate adds `deflate_bits.rs`, `deflate_header.rs`, `deflate.rs`, and
`deflate_plane.rs` to the same verified core. Its mathematical byte-string
history is ghost state, erased from the executable; the real history is a
fixed 32 KiB array. No zlib dependency or external codec contract is introduced.
Python zlib and libtiff's zlib linkage are test-only oracles. See
[the Deflate model and limits](deflate.md).

The profiled JPEG and Deflate optimizations retain this boundary. Borrowed
constant arrays and tables use ordinary safe Rust; the Huffman ordering cache
and aligned-byte shortcut refine the unchanged mathematical models with checked
proofs. The Linux signal sampler in `benchmarks/` is unverified test tooling,
preloaded only into profiling processes and never linked into the core. Reported
performance measurements run with that sampler disabled.

JPEG/OJPEG core modules and the JPEG profile witness belong to the same proof
boundary. There is no codec FFI or heap-allocating dependency in this path.
The integer IDCT is specified exactly; approximation to real cosine and
standards-level numerical conformance are not proved. Color and upsampling
policies are explicit in [jpeg.md](jpeg.md). Libjpeg-turbo and the libtiff JPEG
oracles are test-only independent C executables. Their comparisons establish
observed compatibility, not a universal error bound.

CCITT Group 3/4 adds the `fax*.rs` modules to the same verification boundary.
The fixed endpoint arrays refine pure run sequences, including the documented
zero-run compatibility rules. Numeric codebook data derives from libtiff's
permissively licensed `t4.h`; its notices are retained in `fax_tables.rs`. The
production codec is Rust/Verus and does not call libtiff. Independent encoders
and the libtiff renderer remain test dependencies. Exact compiled stack usage
is not proved; see [fax.md](fax.md) for the fixed array storage and scratch limits.

Float64 transport and Predictor 2 in `float64.rs`, and the concrete
`float_witness.rs` models, belong to the same full-crate proof boundary.
Only integer operations manipulate IEEE bit patterns. The unverified CLI's
NPY header construction and JSON remain tested adapters. The test-only native
libtiff oracle emits binary64 words as canonical little-endian bytes without
performing floating-point operations. No new runtime dependency is added.

The VP8L subset adds `webp*.rs` to the same complete-core proof boundary.
No libwebp code, unsafe implementation, FFI call or runtime allocation is
introduced into the reader. Pinned libwebp 1.6.0 is a test-only independent
oracle, also enabling WebP in the libtiff test dependency. The handwritten
Python encoder is test code; its output and known pixels are checked against
that independent decoder. `webp.md` documents the restricted transforms and
entropy grouping, strict container/trailing-data policy, fixed arrays and
caller scratch. Rust stack layout/exhaustion and correctness of the prose-to-
mathematics specification translation remain outside the checked claims.

The five `logluv*.rs` modules belong to the same verified core. The SGILog32
byte-plane model and integer LogLuv16 conversion add no floating arithmetic,
unsafe code, runtime allocation, FFI, or external codec contract. Native
components have the explicit encoding in [logluv.md](logluv.md); display
rendering and HDR calibration are not claimed. The libtiff SGILog encoder and
native decoder, and the rational Python expectations, are independent test
helpers. NPY and color-model JSON remain unverified, tested CLI output.

The optional `compat/` adapter adds an unverified C varargs/ownership shim and
a Rust `unsafe` FFI bridge. Its source is outside `src/` and outside the Verus
claim. It calls the safe core API, but C pointer validity, buffer capacity,
aliasing, callback behavior, allocation, metadata presentation and rendering
adaptation are additional trusted responsibilities. The optimized core retains
its proof contracts; the adapter remains outside them. [c-api.md](c-api.md) documents exact limits, unsupported
operations and tests. ASan/UBSan check the C layer and consumers; Rust internals
are not sanitizer-instrumented, and the recorded environment could not run
LeakSanitizer. Never describe public symbol coverage as full compatibility or
end-to-end formal verification.

The seekable C reader uses new verified `container`, `native` and
`native_samples` modules for local Classic/BigTIFF records, native compression,
word transport, horizontal prediction, bit reversal and checked OJPEG header
adaptation. It no longer reconstructs palette indices, associated samples or
edge padding from a normalized raster. C directory/geometry assembly and
composition of these primitives are outside the proof. Standard allocation
limits are tested adapter accounting, not a verified allocator theorem.
Synthetic TIFF views for specialized codecs and source-orientation/RGBA
presentation remain explicit unverified transformations. See
[native-compatibility.md](native-compatibility.md) for the complete boundary.


The consumer compatibility pass adds verified local Predictor-3 and CCITT-RLE
primitives; no native codec dependency or new trusted contract is introduced.
Typed/custom metadata, EXIF/GPS selection, environment settings, temporary TIFF
metadata serialization, user-buffer dispatch and numerical XYZ/Lab/YCbCr/CMYK
presentation are ordinary Rust/C adapter work. See consumer-compatibility.md
for limits, tests, numerical comparisons and the native codec boundaries.

The native Zstandard implementation has no new runtime dependency. Zstandard
1.5.7 and a separate ZSTD-enabled libtiff 4.7.1 build are test-only oracles.
Its integer/checksum/entropy/history models are checked; C segment selection,
FFI and callback orchestration remain outside the proof. See `zstd.md`.

LERC and complete static WebP introduce no native runtime dependency. Pinned
Esri LERC/libwebp and an additional libtiff build are test-only. Checked models
cover software IEEE sample reconstruction, VP8L transforms/grouping, VP8 integer
reconstruction/filtering/color conversion and alpha filtering. Model translation
and numerical conformance still require independent review. ALPH synthetic
framing, LERC wrapper/band orchestration, allocation and TIFF/FFI composition
remain ordinary adapter code. Licenses/provenance are in `third-party/`.


The resumable reader pass adds verified codec-step refinements and native
component permutation, without a new trusted assumption or codec dependency.
Lazy metadata, window refill/continuity, validation replay, allocation accounting,
C tag hooks, buffer/client ownership and cropped RGBA placement remain ordinary
adapter code. The public numerical color helpers adapt licensed libtiff code
outside the verified core; they do not decode compressed payloads. Their numerical
behavior is independently compared with pinned libtiff. See `reader-features.md`.

The Deflate/LERC optimization adds no project-owned axiom or unchecked codec.
LERC's fast normalization uses Rust's standard `u64::leading_zeros` intrinsic
and its pinned vstd bit specification, within the existing standard-library/
compiler trust boundary; an erased proof connects it to the unchanged `top`
model. Deflate batches refine the same transitions with exact local acceptance,
output and charging. Selecting one-pass private staging for complete native
segments, input-window continuity and application-buffer commit remain tested
C-adapter responsibilities, not an end-to-end formal theorem.

The kernel pass adds no trusted primitive. Deflate match runs, LERC integer
rounding/narrowing, VP8 filter/DC kernels, VP8L and floating-predictor coordinate
loops refine their prior models. Packed-byte RGBA expansion (8 KiB fixed table),
Lab gamma caching (1,501 bytes) and direct contiguous color loads remain ordinary
Rust adapter presentation. Stack size is compiler-dependent and is not included
in heap-allocation accounting. Complete LZW private staging uses the existing
checked heap limits; it does not bypass validation before caller exposure.

LERC word arithmetic adds only checked integer refinements. FloatLib and
SoftFloat supply algorithm/proof ideas; neither is linked or trusted by the
decoder. The Verus lemmas preserve the earlier bit models, including rounding
after jamming and cancellation guards. Normalization still uses the same pinned
standard intrinsic specification. Formal conformance to real arithmetic or to
the IEEE prose standard is not newly claimed. See `lerc-arithmetic.md`.

The current [native-FP pass](native-float.md) supersedes the earlier no-new-trust
statements for LERC numerical execution only. Earlier optimization descriptions
below/above describe their historical software implementations.

## Current adapter proof boundary

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
