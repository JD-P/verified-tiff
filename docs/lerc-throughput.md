# LERC unpacking and reconstruction throughput

This pass changes execution strategy while preserving the existing Lerc2 v4
bitstream, numerical, acceptance, checksum, strict-ending, work and suffix
models. It adds no trusted operation; the five native floating-point primitive
contracts and their reviewed source hashes are unchanged.

## Changes

- `lerc_word::read` assembles a bounded five-byte little-endian window and
  extracts an entire 0–32-bit field with shifts and a mask. A proved induction
  connects it to the original `zstd_bits::value` per-bit relation. Near an ending,
  the window starts earlier and uses a larger shift, so ordinary LERC block
  tails also avoid per-bit execution. Only ranges ending before byte five use
  the old scalar fallback. No read crosses the supplied end.
- Bit unpacking writes into a caller-owned 256-word block buffer, reused across
  tiles. `read_mode` returns metadata rather than copying that 1 KiB array.
  The original array-returning helper remains for the byte-Huffman setup path.
- Reconstruction advances row/column counters instead of dividing the pixel
  index every iteration. Separate float32/float64 quantized loops avoid the
  generic encoding/type dispatch. A conservative all-valid mask test selects
  a loop without per-pixel mask tests. Its implication is proved; padding bits
  can only prevent the shortcut, never admit a wrong pixel.
- Uniform masks are filled and their count established by an erased induction,
  removing the redundant runtime per-pixel counting pass. Mixed/reused masks
  retain their validation scan.
- Exceptional-value software arithmetic remains available with identical
  semantics, but is no longer forcibly inlined into the ordinary pixel loop.
- The C adapter bulk-initializes accounted word storage and uses fixed-width
  output stores, with a contiguous all-valid packing path. The release x86-64
  assembly uses vector loads/shuffles/stores for float32 packing and contains
  no `memcpy` calls in that helper or the block unpacker. This removes tiny
  variable-size copies per sample and allows compiler vectorization. Band
  interleaving, invalid-float NaNs, alpha masks and host byte order retain their
  original behavior. All bands and endings validate before caller exposure.

The public native API still returns `u64` sample slots. Its scratch storage and
one adapter packing pass remain; this pass does not claim direct decoding into
the caller's float32 buffer. Allocation limits and conservative work fees are
unchanged. C adapter allocation, packing and TIFF composition remain tested
ordinary Rust, outside the core proof boundary.

## Verification and tests

The full decoder proof includes the new word-model induction, in-place buffer
contract, coordinate mapping, typed reconstruction and mask shortcuts. Five
new negative gates corrupt extraction, unpacked values, float output, coordinates
and the mask shortcut. All earlier gates remain required.

`tests/lerc_word.rs` compares every width 0–32 at every bit position for all
endings 0–16, across 64 independent pseudorandom buffers, against a simple
independent bit oracle. This includes zero-width reads, truncated fields,
unaligned reads and backward tail windows. Existing Esri matrices cover all
eight sample types, masks, depths, constants, raw blocks, lookup tables and
Huffman modes; C matrices cover wrappers, endian/layout, bands and alpha.

The complete run checked 1,727 decoder obligations, all 100 negative gates,
60 Rust tests, 97 Python methods, 63 normal and 63 sanitized C methods,
31 original fixtures and 20,000 deterministic mutation cases. All passed.
The separate upstream libtiff self-test passed its 108 tests.

The initial sanitizer/reference self-test attempts encountered missing execute
bits on restored reference executables. After restoring those permissions, the
complete suites passed. Both initial failures and successful reruns are recorded;
no decoder change or test exclusion was needed.

Evidence is recorded in `docs/evidence/lerc-throughput/`. C sanitizer runs
instrument the shim and consumers, not Rust; LeakSanitizer is disabled.

## Reproduction

```sh
python3 scripts/bootstrap.py
python3 scripts/verify-parallel.py --jobs 3
python3 scripts/upstream.py
bash scripts/test.sh
bash scripts/test-compat.sh
VTIFF_COMPAT_OUTPUT="$PWD/target/compat-asan" VTIFF_SANITIZE=1 \
  ASAN_OPTIONS=detect_leaks=0:abort_on_error=1 UBSAN_OPTIONS=halt_on_error=1 \
  bash scripts/test-compat.sh
bash scripts/fuzz-smoke.sh 20000
python3 benchmarks/prepare-lerc-throughput-baseline.py
python3 benchmarks/lerc-throughput.py --rounds 9 --milliseconds 200
```

The baseline is the preceding native-FP build, restored and freshly compiled
from its archived source hashes. `reader.c` is unchanged. Each iteration opens
a fresh handle using memory callbacks. Input loading, caller output allocation
and output hashing are outside timing; decoder allocations remain inside.
Measurements run serially on one pinned CPU without the sampler or concurrent
validation. Seven LERC workloads are timed; all 96 workloads require exact
before/after output, including JPEG. This is not an all-codec speedup average.

## Measured results

Nine randomized serial batches per engine and workload, each at least 200 ms.
The table reports medians from the same-host comparison.

| LERC workload | Previous ms | Current ms | libtiff 4.7.1 ms | Speedup | Current / libtiff |
| --- | ---: | ---: | ---: | ---: | ---: |
| Float32 LERC plain | 26.40 | 9.98 | 2.16 | 2.65× | 4.62× |
| Float32 LERC deflate | 32.96 | 17.61 | 3.21 | 1.87× | 5.49× |
| Float32 LERC zstd | 29.00 | 12.82 | 3.12 | 2.26× | 4.11× |
| Float64 LERC tiled | 28.66 | 12.97 | 2.43 | 2.21× | 5.34× |
| UInt16 LERC tiled | 30.99 | 14.66 | 2.43 | 2.11× | 6.03× |
| Int16 LERC tiled | 31.24 | 13.30 | 2.31 | 2.35× | 5.75× |
| Float32 LERC strips | 24.47 | 8.66 | 2.14 | 2.83× | 4.04× |

Geometric-mean speedup across these seven LERC workloads: **2.31×**.
All **96 before/after outputs match exactly**; 94 also match libtiff exactly.
The two pre-existing JPEG color differences are unchanged.

The remaining ratio to libtiff is reported explicitly. The public `u64` staging
buffer, initialization and adapter packing pass remain, and generic integer
reconstruction is still more expensive than Esri's typed loops.
