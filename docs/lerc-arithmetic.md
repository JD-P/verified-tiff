# LERC word arithmetic

This pass replaces the common LERC reconstruction arithmetic with narrower
integer kernels while preserving the original `scaled`, `sum`, `packed`,
`rounded`, and `narrowed` specifications. Multiplication and addition still round
separately to binary64 before the final typed conversion. There is no fused
multiply-add, host floating-point operation, native codec dependency, or new
trusted proof assumption in the core.

Across seven LERC native-read workloads, the geometric-mean speedup is **1.29×**. Plain tiled float32 improves from 45.91 ms to 33.83 ms (1.36×), and float64 from 45.45 ms to 33.46 ms (1.36×). Plain float32 still takes 15.74× libtiff 4.7.1's runtime on this host.

| Native read | Before ms | After ms | libtiff 4.7.1 ms | Speedup | After / libtiff |
| --- | ---: | ---: | ---: | ---: | ---: |
| Float32 LERC plain | 45.9108 | 33.8329 | 2.1500 | 1.36× | 15.74× |
| Float32 LERC deflate | 50.9874 | 43.0366 | 3.1223 | 1.18× | 13.78× |
| Float32 LERC zstd | 48.3344 | 39.1418 | 2.9847 | 1.23× | 13.11× |
| Float64 LERC tiled | 45.4538 | 33.4560 | 2.3921 | 1.36× | 13.99× |
| UInt16 LERC tiled | 48.5302 | 37.6262 | 2.2809 | 1.29× | 16.50× |
| Int16 LERC tiled | 47.7615 | 37.4607 | 2.3071 | 1.27× | 16.24× |
| Float32 LERC strips | 42.7737 | 32.6159 | 2.1373 | 1.31× | 15.26× |

The complete crate passed **1,704 obligations**, zero errors, and all **89 negative gates**. The checks passed 58 Rust tests, 97 core Python methods, 63 C methods both normally and under ASan/UBSan, all 31 original corpus images, 20,000 core mutations, and the independent libtiff build's 108 upstream tests.

All **96 before/after outputs match exactly**, including JPEG. Against libtiff, 94 match exactly; the two textured-JPEG cases retain their previous maximum error of one channel value. The seven LERC outputs match libtiff bit-for-bit.

Measured 2026-10-15T18:00:42.124158+00:00 on AMD EPYC 9V74 80-Core Processor, pinned CPU 0; nine randomized serial batches of at least 200 ms per engine/workload. No speedup is claimed for codecs that were only checked for identical output in this pass.

The remaining gap includes software arithmetic, entropy decoding and adapter costs. This pass does not establish an unavoidable verification overhead or hardware-float parity.

## Implementation and checked refinements

`jam64` aligns a 64-bit significand, preserving whether any discarded bit was
nonzero. `align_word` proves that aligning the 53-bit mantissa with eight extra
bits agrees with compressing the original 128-bit alignment by 56 bits.
`combine_words` proves both magnitude ordering and exact correspondence after
addition or subtraction; it includes the borrow effects of a nonzero suffix.

`packed_jam` proves rounding equivalence when at least two bits separate the
jammed suffix from the retained significand. The executable checks the remaining
magnitude after cancellation and uses the word path for normal results when
that condition holds. Deep cancellation, subnormal results, and exceptional
ranges retain the original wide calculation. The public addition contract
continues to cover every pair of input bit patterns.

`pack_normal` uses the known leading position and the fact that rounding can
produce at most one carry. Adding the rounded significand to the exponent field
handles that carry, including overflow to infinity, without another leading-bit
scan. `round_significand` makes the rounding decision using the low 64-bit word
for shifts below 64 bits. This covers the normal 53-by-32-bit LERC scaling
product: the exact product remains 128-bit, but its rounding shift is at most
32 bits. The wider rounder remains available for the original fallback.

All these functions are allocation-free and inside the full Verus crate. The
conservative decoder budgets, exact acceptance, checksums, stream endings, and
output/suffix contracts are unchanged. The C adapter is unchanged. This is a
refinement of our explicit bit models, not a new theorem equating those models
with the IEEE prose standard or Esri's implementation.

## Algorithm and proof references

The implementation and Verus proofs are written locally; external proofs are
not imported or assumed.

- [FloatLib, section 3.3 and theorem 4](https://arxiv.org/pdf/2609.19352): rounding
  after shift-with-jamming, the separation between guard and sticky bits, and
  the need to account for cancellation and subtraction borrows. Our instance
  proves equality with the existing finite-width LERC model using bit-vector
  lemmas and the checked leading-position theorem.
- [Berkeley SoftFloat binary64 magnitude addition](https://github.com/ucb-bar/berkeley-softfloat-3/blob/master/source/s_addMagsF64.c)
  and [round/pack](https://github.com/ucb-bar/berkeley-softfloat-3/blob/master/source/s_roundPackToF64.c):
  machine-word significands, jammed alignment, and exponent carry during packing.
  SoftFloat is an algorithm reference, not a trusted runtime or proof dependency.
- [Flocq](https://flocq.gitlabpages.inria.fr/) and
  [CompCert's floating-point interface](https://compcert.org/doc/html/compcert.lib.Floats.html):
  the distinction between numerical specification and executable refinement.

## Validation and reproducibility

The numeric suite retains its random hardware comparisons, normalization tests,
and narrowing ties/carries. New cases sweep every finite binary64 exponent,
both signs and operand orders, exponent gaps around 8/52/64/128 bits, close
cancellation around the word-path cutoff, and scaling by integers adjacent to
every power of two through the full uint32 range. Hardware addition and
multiplication independently supply expected bits for finite inputs.

Five new negative proof gates deliberately corrupt word jamming, significand
rounding, normal packing, addition, and scaling. The preexisting normalization,
wide-rounding, narrowing, checksum, and acceptance gates are retained.

`benchmarks/prepare-lerc-arithmetic-baseline.py` reverses the exact core patch,
checks all preceding source hashes, and builds the previous adapter separately.
The older restoration scripts also unwind this new patch before their existing
ones; all four historical source sets were checked. No host libtiff is replaced.

After bootstrap and upstream setup, reproduce the comparison with:

```bash
python3 benchmarks/prepare-lerc-arithmetic-baseline.py
python3 scripts/verify-parallel.py --jobs 3
bash scripts/test.sh
bash scripts/test-compat.sh
VTIFF_SANITIZE=1 VTIFF_COMPAT_OUTPUT="$PWD/target/compat-lerc-asan" ASAN_OPTIONS=detect_leaks=0 bash scripts/test-compat.sh
python3 benchmarks/prepare-reader.py
bash scripts/build-compat.sh
bash benchmarks/build-reader.sh
python3 benchmarks/lerc-arithmetic.py --rounds 9 --milliseconds 200
```

Run the timing command without concurrent proof, build, or test jobs. It uses
the unchanged C consumer, fresh handles, one pinned CPU, randomized serial
engine order, memory callbacks, and separately measured file-backed RSS.
Input loading, output allocation, hashing, and process startup are excluded from
timing. The reference is freshly built libtiff 4.7.1 with Esri LERC 4.0.0, zlib
and Zstandard wrappers. Only seven LERC native workloads are timed; the entire
preceding 92-workload set is also compared byte-for-byte before/after, including
JPEG. This focused run does not establish a new all-codec geometric mean.

Recorded logs, exact source/binary/input hashes, correctness hashes and raw
timing samples are in `docs/evidence/lerc-arithmetic/`. C sanitizers cover the C
shim and consumers, not Rust internals; LeakSanitizer is disabled.
