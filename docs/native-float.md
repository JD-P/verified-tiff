# LERC hardware floating point

LERC now uses the CPU's floating-point instructions for finite multiplication,
addition, binary64/binary32 conversion, and integer-to-binary64 conversion.
This replaces integer emulation on the ordinary sample reconstruction path.
The original software implementation remains verified and serves as a runtime
test oracle and fallback for non-finite bit patterns whose historical model is
not ordinary IEEE arithmetic. No native codec library is linked.

## Exact trusted boundary

`trusted-float/src/lib.rs` contains five small scalar primitives. Their bodies
are **trusted, not proved by Verus**. They accept integer bit patterns or a
32-bit integer magnitude, have no buffers, allocation, file access, or unsafe
Rust, and return integer bit patterns. Their contracts refer to the existing
pure bit specifications, moved without semantic changes to
`trusted-float/src/model.rs`:

| Primitive | Contract and executable operation |
| --- | --- |
| `add` | Finite binary64 operands; native addition equals `sum`. |
| `mul_twice_u32` | Finite binary64 operand; native multiplication by exact `2*q` equals `scaled`. |
| `narrow` | Finite binary64 operand; native conversion to binary32 equals `narrowed`. |
| `widen` | Finite binary32 operand; native conversion to binary64 equals `value(_,6)`. |
| `from_magnitude` | Exact uint32-to-binary64 conversion plus optional sign equals `packed`. |

The correspondence between those hardware operations and the bit specifications
is assumed explicitly. Differential tests are evidence for that correspondence,
not a proof of it. This is not a claim that the bit specifications themselves
have a separately proved real-number IEEE-754 semantics.

`mul_twice_u32` forms `2*q` in uint64 (at most 33 significant bits, hence exactly
representable in binary64), then performs one rounded multiply. For admitted
LERC error headers, this is identical to `q * (2 * max_error)`: doubling the
error is exact and finite. It also preserves the old helper's result for finite
arguments outside that header restriction. Multiplication and subsequent
addition remain separate; no fused multiply-add or approximate arithmetic is
used.

`src/lerc_numeric.rs` verifies the finite-input dispatch and every fallback
against the same unrestricted postconditions as before. Non-finite addition
and scaling retain the old software model; narrowing/widening retain its exact
NaN sign and payload conventions. Signed zeros, subnormals, cancellation,
rounding ties, carry, and overflow remain covered by comparisons against the
software routines. Input acceptance, checksum/end checks, work accounting,
buffer bounds and output/suffix contracts have not been weakened.

## Execution assumptions

The tested configuration is the pinned Rust/LLVM toolchain on Linux x86-64
with IEEE binary32/binary64 instructions. The calling thread must use round to
nearest, ties to even, gradual underflow (flush-to-zero and denormals-are-zero
disabled), and masked floating-point traps. The normal process/thread default
satisfies these conditions. C callers that alter the floating-point environment
must restore it before entering this Rust library. The library does not change
or restore the caller's floating-point environment. Hardware operations may
set ordinary floating-point status flags. Fast-math/reassociation/FMA contraction
flags or custom compilation replacing those semantics are outside this contract.
Other architectures require validating these assumptions before claiming the
same assurance. Parsing and integer-only codecs do not use this new boundary.

## Verification and audit

The user authorized this trust-boundary change on 2026-10-15. The former blanket
ban on new trusted primitives is narrowed to permit these five operations.
`--no-cheating` also rejects assumptions in imported crates, so the current
full-crate run **does not use that flag**. No bypass flag is silently stripped
by the launcher. The source audit rejects proof escapes in decoder sources and
pins the two boundary source files and five function names in
`trusted-float/audit.json`. Changes to that ledger require reviewing the actual
contracts and bodies. This audit is a supplementary source guard, not a proof.

`scripts/verus-core.py` builds/imports the boundary and runs the pinned verifier
with the caller's flags intact. The exact same native bodies are compiled by
Cargo for release execution. Boundary build receipts explicitly label the five
assumed contracts. The full decoder proof checks all executable core functions
against those imported contracts. A proof of the trusted bodies is not claimed.
The pure model's termination obligation is checked when exporting the boundary.
All preceding negative semantic gates remain; six new gates corrupt each
hardware dispatch result or omit the finite-operand check.

Reproduce:

```sh
python3 scripts/bootstrap.py
python3 scripts/verify-parallel.py --jobs 3
bash scripts/test.sh
bash scripts/test-compat.sh
VTIFF_SANITIZE=1 bash scripts/test-compat.sh
bash scripts/fuzz-smoke.sh 20000
python3 benchmarks/prepare-native-float-baseline.py
python3 benchmarks/native-float.py --rounds 9 --milliseconds 200
python3 scripts/record-native-float.py
python3 scripts/package-source.py
```

The `trusted-float/examples/instructions.rs` witness can be compiled with
`cargo rustc --locked --release -p verified-tiff-float --example instructions -- --emit asm`.
On the tested target its five entry points contain `addsd`, `mulsd`,
`cvtsd2ss`, `cvtss2sd`, and `cvtsi2sd`, respectively, with no helper calls.
The recorded assembly is `docs/evidence/native-float/instructions.txt`.

See `docs/evidence/native-float/` for current proof, test and timing evidence.

## Measured results

Nine randomized serial batches per engine and workload, each at least 200 ms,
on the recorded x86-64 host and pinned CPU. The unchanged C consumer opens a
fresh reader for each iteration, with memory callbacks and input loading,
allocation and output hashing outside the timer. The previous verified word
arithmetic build is restored from its exact archived source hashes.

| LERC workload | Previous ms | Hardware FP ms | libtiff 4.7.1 ms | Speedup | Current / libtiff time |
| --- | ---: | ---: | ---: | ---: | ---: |
| Float32 LERC plain | 35.31 | 28.09 | 2.14 | 1.26× | 13.14× |
| Float32 LERC deflate | 42.79 | 32.86 | 3.33 | 1.30× | 9.86× |
| Float32 LERC zstd | 37.35 | 27.90 | 2.95 | 1.34× | 9.45× |
| Float64 LERC tiled | 36.14 | 29.32 | 2.47 | 1.23× | 11.87× |
| UInt16 LERC tiled | 37.57 | 30.52 | 2.28 | 1.23× | 13.38× |
| Int16 LERC tiled | 37.48 | 29.47 | 2.24 | 1.27× | 13.13× |
| Float32 LERC strips | 33.35 | 23.57 | 2.05 | 1.42× | 11.50× |

Geometric-mean speedup across these seven LERC workloads: **1.29×**.
This is a LERC-only timing result, not a new all-codec average. Arithmetic
emulation was only part of the total cost; substantial decoding overhead remains.

All 96 before/after workload outputs agree byte-for-byte. Ninety-four also
match libtiff exactly; the two existing JPEG color differences remain unchanged.
The current decoder proof checks 1,707 obligations relative to the five trusted
FP contracts. All 95 negative semantic/precondition gates, 59 Rust tests,
97 core Python test methods, 63 normal and 63 sanitized C methods, 31 adopted
corpus images and 20,000 core mutation cases passed. The pinned upstream
libtiff suite also passes 108 tests; that last suite tests the oracle itself.
The software-reference arithmetic test covers exponent/gap/cancellation
boundaries, exceptional bit patterns and 200,000 deterministic random pairs.

C sanitizers cover the shim and C consumers, not Rust; LeakSanitizer is disabled.
The normal C log combines a completed 56-method prefix with separately captured
successful reruns of the seven final LERC/WebP/malformed methods after the
initial log ended prematurely despite a zero exit status. Both the original
logs and completed continuation are preserved. The sanitizer run is complete.
