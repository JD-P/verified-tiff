# Proof ledger

The [C shim coverage pass](c-shim-hardening.md) leaves all checked core sources and trusted-boundary sources unchanged. It adds C hardening and complete measured C function/line/branch coverage; the 1,806-obligation core proof is reused by exact source-hash match.


The current [adapter proof extension](adapter-proofs.md) checks executable
metadata schema decisions, integer conversion, geometry, source-window
conservation, allocation transitions, output initialization and numerical table
indexing. Its final full-crate run verifies 1,806 obligations with zero errors.
Current source hashes and validation receipts are in `evidence/adapter-proofs/`.
The full C pipeline and foreign-pointer obligations remain outside the proof.

The [WebP SSE2 pass](webp-simd.md) verifies span-selected VP8L prediction and color transforms, four-lane butterfly composition, transposition, rounding, color conversion and row-bounded four-pixel stores against unchanged models. Eleven integer lane operations and CPU detection are explicitly trusted. Fifteen new negative gates preserve semantic and precondition coverage. That historical full-crate run had 1,764 verified obligations and zero errors; all 115 then-existing negative gates passed. See `evidence/webp-simd/` for that pass's proof and test receipts.


The [LERC throughput pass](lerc-throughput.md) proves word extraction against the
unchanged per-bit model, in-place block unpacking, coordinate traversal, typed
float reconstruction and uniform-mask shortcuts. Five new negative gates cover
these paths. The five-primitive FP trust boundary is unchanged.

The current [native floating-point pass](native-float.md) verifies LERC dispatch
and all surrounding decoder logic relative to five explicitly trusted native
arithmetic/conversion contracts. The primitive bodies are not proved. The
software reference/fallback and all prior semantic gates remain verified.


The LERC word-arithmetic pass proves word alignment, sticky suffix composition
through addition/subtraction, guarded rounding equivalence, significand rounding
and normal packing against the unchanged numerical models. Five additional
negative gates corrupt those operations. Whole-crate evidence and independent
hardware/decoder comparisons are described in [lerc-arithmetic.md](lerc-arithmetic.md).

The current kernel pass preserves the original codec models while checking
Deflate match-run transition, history, output, frame and work equivalence; LERC
rounding/narrowing; VP8 filter and DC transforms; and VP8L/Predictor 3 coordinate
traversal. Eight additional semantic mutations exercise these shortcuts. Fresh
full-crate evidence is in `evidence/kernel-optimization/`; see
[kernel-optimization.md](kernel-optimization.md). Packed RGBA and Lab tables
remain tested C-adapter presentation, outside the formal core boundary.

The preceding RGBA optimization adds exact 8/16-bit row acceptance, component
rounding and alpha presentation, native byte order, horizontal reversal, error
atomicity and output-frame contracts in `native_rgba`. Four negative gates
break pixels, alpha, acceptance and suffix preservation. Full-crate evidence
is in `evidence/rgba-optimization/`; see [rgba-optimization.md](rgba-optimization.md).
TIFF geometry, allocation admission and C caller-buffer commit remain tested
adapter code. This does not extend the original Raster acceptance profile.

The earlier Deflate/LERC optimization adds checked top-bit equivalence and
Deflate batch acceptance/state/output/work/frame contracts. All previous scalar,
whole-stream and image contracts remain. Four new negative gates break top-bit
selection, batch output, charging and acceptance. Fresh whole-crate and gate
results are in `evidence/reader-optimization/`; see
[reader-optimization.md](reader-optimization.md) for the exact adapter boundary.

The palette/planar/16-bit/Deflate whole-inspection acceptance theorem and
resource bounds are listed in [extended-formats.md](extended-formats.md#checked-proof-scope).
The original complete inspection-acceptance theorem below is preserved.
`inspect_raster` composes the original, extended, JPEG, fax, WebP and LogLuv profiles and accepts their union with a checked
sufficient budget.
Deflate has additional checked stream semantics, acceptance, circular-history,
checksum, and output proofs described in [deflate.md](deflate.md).

Verification is performed on the entire crate rooted at `src/lib.rs`, including all imported core modules with the pinned Verus
release, five audited FP primitive contracts, and eleven audited SIMD integer
operation contracts plus CPU dispatch. `--no-cheating` is no longer
used because it rejects those imported contracts. The solver resource limit is 100; timeouts fail
the gate. All executable core functions are inside
`verus!` blocks. The CLI, fixture builders, test oracle, bootstrap, and fuzz
harness are outside that boundary. No runtime code is duplicated to supply a
different implementation to the verifier.

| Obligation | Status | Checked implementation and specification |
| --- | --- | --- |
| Checked integer addition/multiplication and byte ranges | Proved | `add`, `mul`, `checked_range` have success equivalences and exact-result postconditions. |
| Endian reads | Proved | `read16`, `read32` succeed exactly for in-range reads and agree with mathematical byte-weighted `word16`/`word32`. |
| Header interpretation and acceptance | Proved | `header` succeeds iff `header_valid` holds and ties byte order, magic, and first IFD offset to the input bytes. |
| Entry parsing and local acceptance | Proved | `tag_at` succeeds iff `tag_valid` holds; returned type/count/payload location match the bytes. |
| Directory safety and local acceptance | Proved | `directory` establishes readable entries and raw count/next offset; `directory_valid` plus enough work implies success. |
| Tag lookup and scalar interpretation | Proved | `find` agrees with declarative first-match `lookup`; `scalar` agrees with `scalar_model`. Readable directories, valid scalar encodings, and enough work imply local success. |
| Scalar/channel metadata and local acceptance | Proved | `read_format` establishes `profile_fields`, exact source-value correspondence, and output size. Every supported field combination is accepted with at most `12 * directory_tag_count + 4` work units. This separates channel/alpha validation from traversal and segment proofs. |
| Selected page provenance | Proved | `inspect` relates its directory to `nth_ifd` starting at the actual header offset. |
| Accepted metadata matches the file | Proved | `metadata_matches_source` connects geometry, photometric interpretation, bit depths, codec/transform exclusions, and data arrays to raw tag lookup. |
| Core panic/bounds/arithmetic safety | Proved under the stated API invariants | All reachable executable core functions verify their operations; public inspection has no TIFF-validity precondition. `decode_into` requires the invariant established by `inspect`; private fields preserve it for safe Rust callers. |
| Successful pixel output | Proved | `decode_into` writes the mathematical `decoded_byte` value at every output index. The model computes strip/tile coordinates and byte addresses directly from the original metadata. Bilevel output selects the correct bit for FillOrder 1/2, expands to 0/255, and applies photometric inversion. |
| Decode acceptance after inspection | Proved | `inspect` establishes sample validity and, for PackBits/LZW, valid segment streams. `decode_into` succeeds iff the output is large enough and its initial work budget is at least `decode_work_limit()`. |
| Segment storage covers every logical sample | Proved | `segment_covers_sample` derives coordinate bounds, in-range segment indices, checked intermediate arithmetic, and sample addressability from whole-strip/tile storage. Handles byte-padded bilevel rows, padded edge tiles, and short final strips. |
| PackBits packet interpretation and validation | Proved | `pb_packet` agrees with the mathematical packet sizes. `pb_validate` succeeds iff `pb_valid` holds and work covers the encoded byte count. The model requires exact expanded size, complete payloads, and no row-crossing packets. |
| Streaming PackBits output | Proved | `PbCursor::next` returns the corresponding `pb_byte` value, advances the output position exactly once, and preserves the remaining-stream relation. Source position is monotone; no-op processing has a decreasing source-length measure. |
| LZW bit order and code widths | Proved | `LzBits::read` agrees with the mathematical `lz_wire_bits`, selecting MSB-first `lz_bits` or LSB-first `lz_lsb_bits`, succeeds exactly for complete codes, and advances the bit position exactly. `LzTable::width` implements modern early-change and legacy late-change thresholds. `lz_is_legacy` agrees exactly with the segment-prefix predicate. |
| LZW dictionary and phrase expansion | Proved | `LzTable::data` refines a byte-string dictionary model, including next-entry self-reference and a frozen full table for legacy streams. Modern full-table data remains rejected. Prefixes point to smaller codes; `expand` fills the fixed buffer with exactly the modeled phrase. Clear restores the initial dictionary without admitting stale references. |
| LZW validation and streaming output | Proved | `lz_validate` succeeds iff `lzw_valid` and sufficient encoded-byte work hold. The stream predicate requires initial Clear, exact expansion, and complete EOI with at most final-byte pad bits, or an exact-output sub-byte ending immediately after a data code. `LzCursor::next` returns exactly `lzw_byte` at the next output position and preserves stream/table invariants. |
| LZW sub-byte ending compatibility | Proved | `LzBits::at_tail` holds iff 0–7 bits remain. `lz_validate` additionally requires zero bytes left to expand and a previous data code. `lzw_tail_has_witness` proves acceptance and the exact byte for a concrete three-byte stream, and rejection if two output bytes or a whole extra encoded byte are requested. Two negative mutations must fail the validator's equivalence contract: removing acceptance and permitting incomplete output. |
| Storage-to-output mapping and padding | Proved | `sample_maps_to_target` and `target_maps_to_sample` establish the correspondence between logical samples and storage byte/bit positions. `scatter_byte` writes all and only corresponding output samples; `decode_pb_segment`, `decode_lz_segment`, and `decode_compressed` compose this across streams and segments. |
| Alpha metadata and canonical pixels | Proved | `read_alpha` accepts exactly the supported ExtraSamples encodings with sufficient lookup work. `convert_color` refines `straight_color`, including zero, rounding, clipping, and grayscale polarity. `alpha_index_bounds` and `convert_alpha` prove in-place normalization, alpha preservation, and the output suffix frame. `decoded_byte` specifies the final straight-alpha result. |
| Output frame and exact decode quota | Proved | Output length and the suffix beyond the decoded image are preserved. Successful decode spends exactly `decode_work_limit()` units: output bytes, another output-length pass for alpha when present, and encoded/expanded segment storage for PackBits/LZW. This is a conservative work quota, not an instruction count. |
| Termination and work conservation | Proved for core loops and counters; cost interpretation audited | Every executable loop has a decreasing measure. Loops charge finite budget; the verifier establishes conservation and local lookup cost bounds. Fixed-width helper operations have constant abstract cost. |
| Core heap allocation | Established by source architecture | Core parsing/decoding performs no heap allocation; caller-owned buffers and fixed-size state are used. No input-driven runtime recursion. No claim about exact compiled stack bytes or total process RSS. |
| Every file satisfying the formal supported profile is accepted by `inspect` | Proved | `supported_profile(input, page, limits)` and initial work at least `inspection_work_bound(page, limits)` imply success. Inspection has no validity precondition for safety. |
| Sufficient inspection budget and checked calculator | Proved | The mathematical bound is `(page + 1) * (max_tags + 1) + 19 * max_tags + 4 + max_segments * (max_input + 1) + max_output`. `inspection_work_limit` succeeds iff this bound fits `usize` and returns exactly it. |
| The formal profile is inhabited at both depths and all three compressions | Proved | `supported_profile_has_witness(bilevel)` establishes concrete 87-byte uncompressed TIFFs (inspection bound 214). `packbits_profile_has_witness(bilevel)` establishes concrete 100-byte PackBits TIFFs (inspection bound 247, decode quota 4). `lzw_profile_has_witness(bilevel)` establishes 102-byte LZW TIFFs (inspection bound 249, decode quota 6). `alpha_profile_has_witness(rgb, associated)` establishes four 122-byte Gray+A/RGBA alpha witnesses (inspection bound 292). `lzw_legacy_profile_has_witness(bilevel)` adds two 102-byte old-style LZW witnesses with the same bounds. All twelve are runtime regression cases. |
| Extended whole-inspection acceptance | Proved | `supported_extended_profile` implies successful `inspect_extended` with `extended_inspection_work_bound`. Layout, palette, array-window, plane validation and quota-sum helpers compose without per-pixel preflight assumptions. |
| Deflate plane acceptance and work | Proved | `deflate_plane::profile` and sufficient work imply validation succeeds. Returned work equals the mathematical sum of per-segment transition quotas plus segment charges. |
| Public raster acceptance | Proved | `supported_raster_profile` is the union of the original, extended, JPEG, fax, WebP and LogLuv profiles. `raster_inspection_work_bound` suffices for the bounded original, extended, JPEG, fax and WebP probes plus independent LogLuv inspection; the original smaller-budget guarantee is retained. |
| Extended bound calculators | Proved | Both extended and raster calculators succeed iff their mathematical bound fits `usize` and return exactly that bound. |
| Extended profile non-vacuity | Proved | Five 200-byte TIFF witnesses cover palette, Gray16, separate RGB, Deflate Gray8 and Deflate bilevel; each has an extended sufficient bound of 20,288 units. |
| Fidelity of the mathematical TIFF model to the prose standard | Reviewed and independently tested | Specifications themselves need human review. Differential tests are evidence against specification errors, not a formal proof of the prose standard. |
| CLI allocation/I/O behavior and process-wide resource consumption | Tested/source-reviewed | OS calls and allocation remain outside the core proof. |

The public pipeline claim is: **for every input satisfying the formal supported
profile, inspection succeeds with the proved sufficient work budget; decoding
then succeeds with enough output space and `decode_work_limit()` additional work
units, producing the specified pixels without core panics, invalid accesses,
or unchecked arithmetic errors.** The successful-output guarantee also holds
when inspection happens to succeed with less than the conservative budget.
Malformed input inspection retains its unconditional safety and termination
contract and may return a structured error.

`supported_profile` describes the byte-order marker and magic, visited main
directories, sorted valid entries, selected image's scalar and array fields,
supported transformations, dimensions, resource limits, declared segment
storage, and valid PackBits/LZW streams for compressed segments. It is a pure
specification over the original byte sequence. It does
not invoke `inspect`, `sample`, or another executable decoder, and it does not
contain a per-output-byte `sample_valid` assumption. The latter is derived
from segment storage by a separate verified coordinate/address lemma.

The bound allocates at most `max_tags + 1` units per visited directory,
19 directory scans for the selected image, 4 channel-format checks,
`max_segments` segment checks, at most `max_segments * max_input` encoded-byte
validation quota, and a retained `max_output` uncompressed inspection quota.
Uncompressed inspection derives sample addressability from complete segment
coverage; it no longer executes a per-output-sample preflight loop.
It is intentionally conservative, not a necessary acceptance threshold.
If it exceeds `usize`, the calculator reports `Limit`; that does not imply
that every particular file needs that much work.

For bilevel data, the model maps output pixel `(x,y)` to its segment's
`row * ceil(block_width/8) + floor(local_x/8)` byte, selects bit
`7 - local_x % 8` (FillOrder 1) or `local_x % 8` (FillOrder 2), and expands it
to 0 or 255 before photometric inversion. The executable uses division by
a verified power-of-two divisor and remainder modulo 2. Row padding and
edge-tile pixels have no output indices. The acceptance predicate describes
packed segment storage directly; it still contains no per-pixel validity
assumption. The conservative uncompressed inspection quota still counts
expanded output pixels, preserving the existing sufficient-budget contract.

`pb_valid` and `pb_byte` are recursive mathematical definitions over source
packets, not calls to the executable codec. Validation reduces the remaining
encoded range on every packet, checks literal/repeat payloads, subtracts their
expanded lengths, and respects the remaining bytes in each row. The streaming
cursor maintains equality between the original byte model and its current
packet/unread suffix. Its proof-only source/stride/size metadata is ghost state.

For PackBits/LZW, inspection derives sample validity from the validated streams
and segment geometry without per-pixel preflight. Decode reserves the quota
`output_len + alpha_pass + sum(encoded_segment_bytes + expanded_segment_bytes)`
where `alpha_pass` is `output_len` when alpha is present, otherwise zero before
writing. A segment is visited once; packet/code parsing advances through its
encoded range, and rendering visits each expanded storage byte once with at
most eight bit slots. Exact quota subtraction and terminating loops are
checked. Interpreting that quota as a constant-factor CPU-work bound also
depends on the fixed-cost helper-operation audit; this is not a proof of
machine instruction counts, elapsed time, or compiler complexity.

The LZW mathematical dictionary is a sequence of byte strings. `lz_after`
specifies dictionary insertion, and `lz_phrase` specifies literal, existing,
and next-entry phrases. These are independent of the executable prefix-table
representation. Recursive `lz_valid` and `lz_byte` consume one encoded code per
step, with remaining encoded bits as their decreasing measure. `lzw_valid`
adds initial-Clear and segment-boundary requirements. The predicate never
calls the executable validator or decoder.

The executable table's ghost dictionary relates every active entry to its
prefix plus last byte, cached length, and first byte. Prefix indices are smaller
than the referring entry; expanding a phrase therefore terminates without
cycles or runtime recursion. The fixed buffer contains the modeled phrase in
forward order. A cursor invariant relates pending buffer bytes and future
stream bytes to the original `lzw_byte`. Segment and image proofs then compose
that relation with the existing bilevel/Gray8/RGB8 scatter mapping.

LZW inspection follows codes and cached lengths without expanding each phrase.
Decode expands each phrase once. Fixed table/buffer initialization is part of
the abstract per-segment constant cost, code reads take at most 12 iterations,
and phrase expansion is linear in expanded storage. Resets do not reinitialize
the arrays. As with PackBits, interpreting the proved quota as a CPU-work
bound includes this source-level constant-factor audit; exact compiled stack
usage, elapsed time, and machine-instruction costs are outside the proof.

For alpha, `storage_pixel` models the native color/alpha samples and
`output_pixel` applies the specified straight-alpha normalization. The public
`decoded_byte` contract uses the latter. During `convert_alpha`, earlier bytes
already equal final pixels and all later bytes still equal native samples.
The last channel of each pixel therefore remains available unchanged while
its colors are normalized. Bounds, arithmetic, exact color conversion, alpha
preservation, and output framing are verified. No image allocation or new
trusted codec is introduced. The alpha conversion policy, including clipping
extended associated colors, is specified explicitly in `profile.md`.

The parser delegates scalar/channel checks to `read_format` and seven layout
lookups to `read_geometry`, preserving source correspondence and the original
sufficient inspection budget; its verified
contract provides source-field correspondence and a sufficient local budget.
The large remaining parser proof uses a separate prover context for scheduling
only, with the same contracts and resource limit.

Opaque specification functions control solver unfolding; they do not assume
results. Their definitions are revealed in the relevant verified helper proofs.
Upstream standard-library specifications are trusted dependencies, not new
project axioms. `scripts/verify.sh` also requires a deliberately false contract
and always-error replacements for `inspect`, `inspect_extended`, and the Deflate
validator, plus removal of `inspect_raster`'s extended fallback, to be rejected, and records the
verified source hash. These negative checks test the proof gate; they add no
assumptions to the production implementation.

## Remaining trust and future proof work

Parser completeness is checked relative to the mathematical profile. Fidelity
of that profile to the prose TIFF standard still needs independent review,
just as the original pixel model did. This is not a proof of every TIFF
extension, compiler correctness, OS behavior, or the unverified CLI.

The segment-coverage lemma avoids sample-by-sample inspection preflight for
uncompressed images and PackBits/LZW. New codecs require their own semantics
and acceptance/resource proofs.

The optimized uncompressed 8-bit strip path uses `strip_row_mapping` to prove
each row's contiguous input range and `copy_byte_run` to copy or invert it.
The helper proves exact destination bytes and preservation outside that range;
the row loop establishes the existing `storage_pixel` contract. Normal copies
use the upstream `vstd` specification of safe slice `copy_from_slice`.
Padded tile rows now use a corresponding contiguous mapping proof. Packed
bilevel expansion shares one proved visible span across each byte's eight
bits. Insufficient-budget paths retain their scalar fallback. Alpha conversion
uses the existing verified pass unless normalization is proved to be identity.
The full-width 8-bit `storage_target` shortcut
proves the same valid-target and exact-index relation as the general mapping.
Two additional negative gates remove the copy or corrupt that index and must
fail their semantic postconditions. No new assumptions or external bodies are
introduced. Inspection and decode quotas remain conservative and unchanged;
plain uncompressed decoding retains its insufficient-budget partial-prefix
behavior, including untouched output beyond that prefix.

JPEG/OJPEG adds exact byte-stream, integer IDCT, YCbCr conversion, scatter,
whole-image acceptance, and resource proofs described in [jpeg.md](jpeg.md).
The complete crate also verifies a concrete JPEG TIFF profile witness and
checks JPEG semantic mutations through the negative gate.

The profiling pass retains those exact transform and entropy models. Immutable
JPEG/Deflate constant arrays are borrowed instead of constructed for each lookup;
JPEG bit extraction uses the existing proved power-of-two divisor. Borrowed
JPEG geometry and Huffman tables avoid value copies. Deflate's fixed symbol-order
cache is constructed with a proof of equality to the original `ranked` function,
and an aligned stored byte refines the existing eight-bit `field` model. There
is no changed acceptance predicate, weakened contract, new axiom or trusted
external body. Two additional negative gates corrupt these Deflate return values
and must fail the existing exact-value postconditions. The existing scatter
helper is marked `inline(always)` to avoid a measured compiler inlining regression;
its body and contracts are unchanged. See [profiling.md](profiling.md).

CCITT Group 3/4 adds the [fax profile](fax.md): byte/code/run acceptance,
reference-row transitions, framing, exact lossless pixels, TIFF inspection
acceptance, scratch/scatter frames, and checked sufficient budgets. Two
concrete TIFF witnesses establish profile non-vacuity. Four additional negative
checks cover stream/image rejection, removed pixel writes and removed fax
dispatch. Existing acceptance and semantic contracts remain checked.

## Float64 and Predictor 2

The existing extended acceptance theorem now covers the float64 combinations
in [float64.md](float64.md), with the same sufficient inspection bound.
`float64.rs` proves byte-weighted 64-bit sample loading, modular addition,
per-channel predecessor tracking with row/tile resets, little-endian output
and suffix frames. The pipeline composes these semantics with native scratch
and segment/scatter proofs. No floating-point arithmetic or new trusted codec
is introduced. Two concrete 160-byte LZW TIFF witnesses establish acceptance
for Predictor 1/2. Three negative gates reject zero loads, removed addition and
float64-only rejection in the format inspector. Existing thirteen negative
gates and every previous acceptance/output contract remain checked.

## Lossless WebP subset

The nine `webp*.rs` modules add the complete supported-profile acceptance and
pixel/resource proofs in [webp.md](webp.md). The single-group VP8L model includes
canonical Huffman descriptions, zero-bit leaves, code-length repeats, every
length/distance prefix, plane-distance remapping, overlapping copies, cache
hashing/update order, exact raster length and inverse subtract-green.
Validation and decoding refine the same pixel/cache model. Erased history
allows inspection without a real decoded raster. Conversion/scatter proofs
preserve alpha semantics, exact RGB/RGBA output and both buffer suffixes.

The checked sufficient-budget calculator composes with the public profile
union and preserves all previous acceptance contracts. A concrete 160-byte
TIFF witness is paired with a runtime regression. Five new semantic mutations
cover stream acceptance, TIFF acceptance, transform writes, cache hashing and
public dispatch; all sixteen earlier negative gates remain required.

## Old-style libtiff LZW

The shared byte-string dictionary and stream/pixel predicates now include the
segment's explicit legacy selection. `LzBits::read` preserves that selection and
refines both bit orders; width and dictionary transitions use the same flag.
`lz_after` leaves a full table unchanged, while `lz_data_ok` permits that case
only for legacy data. The cursor links every returned byte to `lzw_byte`, with
the original image and extended/public acceptance, exact pixels, budgets and
buffer frames preserved. No external decoder or proof assumption is added.
The shared profile witness proves acceptance for both wire variants and both
original sample depths. Three negative mutations must fail postconditions:
removing legacy detection, using modern width thresholds for legacy, and
rejecting legacy data at dictionary saturation. All 23 earlier gates remain.

## Native SGILog32 / LogLuv16

The five `logluv*.rs` modules implement the [LogLuv profile](logluv.md) in the
same full-crate run. The SGILog packet validator has an acceptance equivalence
against a pure byte-plane model; its constant-size cursor refines exact
expanded bytes. Integer native-component conversion, geometry/scatter, work
sums, scratch maxima, and buffer suffixes compose into whole-image output and
acceptance contracts. The public six-profile union and checked budget include
LogLuv while preserving every earlier guarantee. A concrete 166-byte TIFF
witness establishes non-vacuity with a 327-unit dedicated inspection bound.
Five new negative gates cover stream/TIFF acceptance, conversion writes,
chroma arithmetic and public dispatch; all 26 earlier gates remain required.
The proof concerns native encoded components, not RGB/XYZ or tone mapping.

## C compatibility adapter (outside proof boundary)

`compat/` is a separate ordinary Rust crate plus C shim. It reuses the verified
core through its safe API. None of the C varargs,
foreign pointer ownership, metadata adaptation, allocation or output
conversion is part of the core proof. The adapter has public
symbol audits, C contract tests, independent-source and libtiff comparisons,
C-layer sanitizer runs and a separate mutation smoke target. See
[c-api.md](c-api.md) and [recorded validation](validation.md).

## Throughput refinements

The second throughput pass preserves all original stream and pixel models:

- `bit_window` proves safe 24-bit loads and bounded extraction equivalent to
  both LZW bit orders and the Deflate field model. The original short-tail
  readers remain the fallback.
- JPEG consumes buffered amplitude bits in chunks and extracts Huffman bits
  by shifts. The exact integer IDCT uses a symmetric factorization and DC-only
  shortcut; output rounding/clipping is unchanged. Eight-pixel render runs
  refine the existing component index model, including subsampling factor 3.
- Uncompressed padded-tile rows use proved contiguous mapping and bulk copies.
  Packed bilevel scatter proves one visible span per byte, then uses contiguous
  output offsets and bit shifts, including clipped tiles and both bit orders.
  Straight alpha skips a redundant pass only when normalization is identity.
  Extended interleaved 8-bit output uses a native copy; interleaved 16-bit
  output uses direct sample pairs with the same endian semantics.
- Deflate batches checksum reduction with bounded accumulators. Direct strip
  output and the combined API refine the original decompression model and
  preserve checksum, padding, resource and frame claims. The combined API has
  a supported-profile acceptance theorem and a checked sufficient budget.

Five new negative checks require rejection of broken word extraction, the
IDCT DC shortcut, the native identity copy, bilevel span truncation, and an always-error combined
Deflate decoder. Existing acceptance and shortcut checks remain required.
The transform proof explicitly instantiates coefficient bounds and isolates
its solver context with `spinoff_prover`; it uses the same solver resource limit
100 and verifies the same runtime body as the rest of the crate.

## Native segment and BigTIFF record additions

`container.rs`, `native.rs` and `native_samples.rs` are part of the same
full-crate proof. Their exact acceptance, source-value, native-output,
row/channel prediction, buffer-frame and resource contracts are listed in
[native-compatibility.md](native-compatibility.md#verification-scope).
They add 26 verified obligations and four negative semantic/acceptance checks.
All earlier obligations and 40 negative checks remain. The current evidence
records 1,064 obligations, zero errors, at the unchanged resource limit 100.

These are local record and supplied-buffer contracts. They do not establish
an end-to-end BigTIFF parser acceptance or C-handle theorem. File I/O, selecting
IFDs/segments, assembling metadata/geometry, allocating buffers, presenting
RGBA and constructing synthetic one-segment containers remain unverified
adapter responsibilities. Existing original whole-image profile theorems are
unchanged. Never extend their scope to all files accepted by the C adapter.

## Consumer compatibility primitives

The C reader adds `native_fp::decode` (now IEEE16/24/32/64 Predictor 3) and
`native_rle::decode` (CCITT Modified Huffman with byte-aligned rows). Both have
exact local acceptance/output relations and suffix frames; RLE also has checked
work conservation. Predictor 3 composes the prior per-channel accumulator with
a proved byte permutation. Two additional negative semantic gates corrupt that
permutation and disable row alignment. See [consumer-compatibility.md](consumer-compatibility.md).
Metadata/custom-directory handling, resource configuration, temporary writing,
user-buffer orchestration, LogLuv float arithmetic and CMYK/Lab/YCbCr presentation
remain unverified adapter code. This does not extend the public whole-image
TIFF profile theorem or establish end-to-end C/BigTIFF verification.

## Native Zstandard addition

`native_zstd` and the `zstd_*` modules refine explicit frame, entropy, history
and checksum models. They preserve exact acceptance, decoded bytes, caller
buffer frames and checked work allowances, with a concrete raw witness and
six semantic/acceptance negative mutations. See [zstd.md](zstd.md). This does
not broaden the existing whole-image `Raster` theorem or verify the C adapter.

## Native LERC and full static WebP

The new `native_lerc`, `native_webp`, `native_vp8` and supporting modules refine
explicit header, entropy, transform and sample models. Public decoders prove
exact acceptance with sufficient storage/work, output correspondence and suffix
frames. LERC includes Fletcher32 and software IEEE bit arithmetic; WebP includes
all VP8L transforms/grouping, VP8 binary arithmetic, prediction, integer
transforms, filtering, YUV/RGB conversion and ALPH unfiltering. See [LERC](lerc.md)
and [WebP](webp-native.md) for their numerical and integration boundaries.

Twelve additional negative gates cover LERC checksum/rounding/acceptance, VP8L
group/predictor/palette/cross-color/acceptance, VP8 arithmetic/RGBA/acceptance,
and ALPH prediction. All 52 prior gates and original whole-image witnesses are
preserved. No proof bypass, runtime FFI codec or new trusted assumption is used.
The C adapter and its TIFF composition remain outside these theorems.


## Resumable native reader primitives

`native_stream` refines the existing PackBits packet, LZW dictionary/bit-reader
and Deflate transition models. Contracts cover exact packet/code acceptance,
expanded phrases and dictionary updates, cursor positions, Deflate byte emission,
checksum completion and one-step budget conservation. The adapter supplies
consecutive bounded input windows; window continuity, validation/replay and I/O
composition remain unverified. These are local refinement claims, not a new
whole-file streaming theorem. Existing whole-stream/whole-image claims remain.

`native_samples::swap_components` proves exact byte reversal within components,
layout acceptance and suffix preservation. Predictor 3's existing permutation
and output proofs now include two- and three-byte floating components.
Four new negative gates corrupt component reversal, the Deflate transition,
LZW phrase output and PackBits packet length. All 64 previous gates remain.
See [reader-features.md](reader-features.md) for integration evidence and limits.

## Adapter proof extension

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
