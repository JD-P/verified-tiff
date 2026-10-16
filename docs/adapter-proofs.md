# Adapter proof extension

This pass moves security-relevant adapter decisions into five executable Verus
modules. They are called by the actual C-compatible reader; they are not a
standalone model or a collection of assumed contracts. No new project axioms,
trusted FP operations, SIMD operations or native decoder dependencies are added.
The existing C ABI is retained.

## Checked claims and their integration

| Module | Checked contract | Production call sites |
| --- | --- | --- |
| `adapter_meta` | Exact accepted getter shapes from a registered schema; count-width limits; registration count-convention consistency; exact integer conversion or rejection; source metadata payload bounds | `compat/fields.inc` through stateless bridges; `compat/src/metadata.rs` and `directory.rs` |
| `adapter_geometry` | Positive strip/tile grids, exact plane/segment indexes and sufficient-input acceptance; orientation aliases; exact mirrored crop spans contained in the destination raster | Directory geometry assembly; native and CMYK/Lab/YCbCr RGBA placement |
| `adapter_window` | Exact unread-suffix preservation and unchanged suffix frame; refill source-position conservation; available-bit arithmetic | The production PackBits/LZW/Deflate input-window refill and LZW end checks |
| `adapter_bounds` | Exact allocation admission and release transitions, signed-addressable allocation sizes, source offsets, predictor-row rounding, bounded directory steps; failure-atomic initialized copies and initialization of C output buffers | Adapter buffer allocation/accounting, source I/O, streamed range reads, directory advancement, and final native/RGBA/user-buffer commits |
| `adapter_color` | Bounded IEEE binary32-to-index conversion including NaN, infinities and negative values; exact CMYK integer pixels; overflow-free wide YCbCr table combination and clipping | Rust Lab/CMYK rendering and public C Lab/YCbCr helpers |

The binary32 model specifies saturated nonnegative truncation directly from
IEEE sign/exponent/fraction bits. The implementation never casts an arbitrary
floating-point result to a C integer or table index. Independent hardware
comparisons include 200,000 pseudorandom bit patterns and explicit exceptional,
subnormal and overflow boundaries. This does not prove gamma or matrix evaluation
against real-number colorimetry.

## Concrete changes

* Generic metadata getters obtain an executable checked dispatch plan. A stored
  count cannot switch a registered scalar into an array or change the output
  count width. Stored types are converted to the registered host representation
  or rejected. Scalar varargs now use the corresponding typed pointer rather
  than extracting every scalar as `void *`.
* A custom definition whose getter and setter disagree about the width of a
  passed count is rejected at registration. The standard TIFF/EXIF/GPS schema
  and anonymous-tag pointer/count convention remain available.
* All signed and unsigned integer metadata narrowing runs through the checked
  value-preservation function. Metadata payloads are bounded before iteration.
* Allocation claims and releases use checked transitions inside atomic update
  operations. A mismatched release cannot wrap the live-byte counter; it poisons
  subsequent allocation admission. Accounting includes old and replacement
  buffer capacities while resizing. It counts requested heap bytes, not allocator
  rounding, stack usage or process RSS.
* Crop spans used by fast, packed, planar and color reads share the checked
  coordinate mapping. Predictor-row expansion no longer relies on an unchecked
  `end + row - 1` expression. The input window uses checked compaction and refill
  plans, publishing cursor updates only after the I/O read succeeds.
* Native, RGBA and user-buffer output commits reject address wrapping and overlap
  at the FFI bridge. The executable proof loop accepts `MaybeUninit<u8>` output,
  so an ordinary uninitialized C `malloc` buffer remains valid. It initializes
  exactly the written interval without a separate zero-fill pass. Caller prefix
  and suffix memory is never made into a Rust byte reference.
* Public Lab lookup indexes are bounded before table access. Invalid table ranges
  are rejected, and initialization checks all three channel steps for underflow.
  YCbCr table additions use wide checked-core arithmetic even if caller-owned
  tables contain extreme signed values.

## What is still not proved

This is **not an end-to-end proof of the C library**. In particular:

* C argument extraction, actual pointer validity/alignment/capacity, external
  aliasing, callbacks, handle lifetime, descriptor ownership and matching
  allocation/release events are still adapter obligations. The budget transition
  proof does not establish that every allocation is charged or that every C
  allocation is freed exactly once.
* Choosing and composing all records, caches, codec wrappers and synthetic TIFF
  containers is still ordinary Rust/C. The new geometry and window contracts
  establish their local decisions, not a theorem about the complete public read.
* Window position conservation assumes the I/O adapter supplies the requested
  bytes. Validation and replay require an unchanged source. Arbitrary callbacks
  and concurrent source mutation are not certified by a decoder proof.
* Numerical color matrix/gamma evaluation and C lookup-table pointer ownership
  remain outside the checked core. The new index and integer-combination
  functions provide safety around those results, not complete colorimetry.
* A finite per-operation budget is not a process-wide CPU/memory or wall-clock
  guarantee. Repeated API calls, consumer allocations and scheduling remain
  application responsibilities.

The original whole-image acceptance theorems are preserved. These new contracts
must not be described as universal Classic/BigTIFF C-API acceptance or as proof
that every historical CVE is eliminated. The earlier CVE ledger was a mechanism
assessment; this pass does not silently revise its prevention percentages.

## Reproduction and evidence

The recorded build has 1,806 verified obligations and zero errors; all 125
negative proof gates reject their mutations, including ten added in this pass.
The 70 Rust tests, 97 core Python tests, and 64 C tests pass. The same 64 C tests
also pass with the C boundary instrumented by ASan/UBSan. The deterministic core
mutation smoke completes 20,000 iterations. These are separate proof, regression
and smoke-test results; sanitizing C does not instrument all Rust machine code.

Run the full proof gate (`python3 scripts/verify-parallel.py --jobs 3`), the core
suite (`bash scripts/test.sh`), the C suite (`bash scripts/test-compat.sh`), and
the C suite with `VTIFF_SANITIZE=1`, a separate `VTIFF_COMPAT_OUTPUT`,
`ASAN_OPTIONS=detect_leaks=0:abort_on_error=1` and
`UBSAN_OPTIONS=halt_on_error=1`. Run the deterministic core mutation smoke with
`bash scripts/fuzz-smoke.sh 20000`; this is not a coverage-guided fuzz campaign.

`tests/adapter_contracts.rs` checks metadata boundaries, crop/reference mapping,
stream windows, allocation/row-rounding limits, numeric edge cases and output
frames. `compat/tests/adapter_proofs.py` runs twelve malformed-schema fixtures
across both byte orders and Classic/BigTIFF through the public C API, plus borrowed
read-buffer ownership and malformed numerical helper states.

Ten additional negative proof gates deliberately corrupt schema dispatch,
integer narrowing, allocation admission/release, crop mapping, window compaction,
output initialization, numeric indexing, row rounding and source offsets. They
must fail the corresponding semantic postcondition or output loop invariant.

The current run, source hashes, trusted-boundary hashes, test logs and negative
checks are recorded under `docs/evidence/adapter-proofs/`. Historical benchmark
and proof evidence remains historical and is not relabeled as this version.
The final `verification.json` and `verification-summary.json` are authoritative
for the complete source. The parallel gate log includes an earlier positive run
that predates the finalized output-initialization loop; the later complete run
supersedes it. All ten new adapter mutations use the finalized implementations.
