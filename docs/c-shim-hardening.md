# C shim hardening and complete measured coverage

The Linux LP64 C shim now has 100% measured function, executable-line and
branch-outcome coverage, combining real-backend integration tests with boundary
unit tests. No source lines or emitted branches are excluded from the report.

| GCC coverage metric | Before this pass | Final clean run |
| --- | ---: | ---: |
| Pinned public API functions called | 159 / 198 | 198 / 198 |
| All emitted C functions, including internal helpers and stubs | 201 / 242 | 242 / 242 |
| Executable line records | 1,118 / 1,352 | 1,352 / 1,352 |
| Branch outcomes taken | 889 / 1,485 | 1,459 / 1,459 |

The changed branch count reflects code changes, including removal of redundant
checks after existing validation. It is not an adjusted denominator or an
exclusion list. Source hashes, complete counters and separate per-binary raw
coverage data are in `evidence/c-shim-hardening/`.

## Fixes

* Lab initialization now calculates green and blue table steps from their own
  channel ranges. Previously both used the red range. An asymmetric display
  regression checks the independent steps and expected midpoint colors.
* `TIFFMergeFieldInfo` rolls back fields added by the current call on validation,
  allocation or quota failure. All registration failures set the extender error
  flag, so failed initialization cannot silently publish a partially configured
  handle. Existing fields and previously successful calls remain intact.
* Handle names are bounded to 4,095 bytes before allocation and size arithmetic.
  This also bounds names supplied with file descriptors and client callbacks.
* `_TIFFcalloc` treats a zero product as a one-byte allocation, rather than
  replacing each zero factor with one and accidentally requesting the other,
  potentially huge factor. Allocation failure and failed-realloc preservation
  are exercised explicitly.
* Byte swapping rejects extents exceeding `PTRDIFF_MAX` before pointer traversal.
  Default transfer-table generation rejects zero bit depth before evaluating a
  formula whose denominator would be zero.
* Redundant output clamps already guaranteed by the verified index function,
  duplicated null checks and impossible LP64/range checks are removed. YCbCr
  coefficient clamping uses the established finite luma range and positive
  green denominator; overflow to positive infinity remains upper-clamped.
* Sanitized builds now enable floating-point-to-integer overflow checking as
  well as address and undefined-behavior checking.

## What the tests exercise

The ordinary 64-test C integration suite still uses the actual Rust backend,
independent fixture expectations and pinned libtiff oracles. It now also runs
`compat/tests/shim_hardening.c` at the normal optimized build setting.

The boundary harness compiles the production C source directly, so it can test
internal failure states without adding test hooks or exports to the library.
Calls normally use the real Rust implementation. Linker wrappers inject explicit
bridge failures and boundary return values. Allocator wrappers fail selected C
allocation attempts and track all C-owned allocations; the final outstanding
count must be zero. Real callback implementations exercise short writes, failed
seeks, negative/zero/oversized write returns and close ownership. A few tests
construct inconsistent internal metadata to exercise defensive rejection paths;
those are unit tests, not claims that a valid TIFF can produce such a state.

Assertions cover scalar varargs widths and signedness, output guards, failure
preservation, field-registration rollback, bounded counts, allocation and
reallocation behavior, callback ownership, directory transitions, buffer reads,
color limits, numerical edge cases, diagnostics, utilities and every public
entry point. Raw reports preserve the distinction between the shared-library
integration run and this boundary harness; their counters are merged only for
the combined C coverage metric.

The optimized integration suite and boundary harness pass ASan, UBSan and
`float-cast-overflow`. LeakSanitizer cannot run in this execution environment
because its process/thread inspection fails; no LeakSanitizer result is claimed.
The explicit C allocation ledger is checked independently. Rust machine code
is not instrumented by these C sanitizer flags.

## Reproduction

```bash
bash scripts/test-compat.sh
bash scripts/test-c-coverage.sh
VTIFF_COMPAT_OUTPUT="$PWD/target/compat-asan" VTIFF_SANITIZE=1 \
  ASAN_OPTIONS=detect_leaks=0:abort_on_error=1 UBSAN_OPTIONS=halt_on_error=1 \
  bash scripts/test-compat.sh
```

The coverage script starts with clean counters in its dedicated output directory
and compiles C with GCC `-O0 --coverage -fprofile-update=atomic`. The report merges
matching source/function/line/branch records, checks their branch structure,
requires the complete production source inventory and every public symbol, and
fails unless all three measured coverage ratios are 100%. No `gcov` exclusion
annotations or branch waivers are used. Test harness code is not in the
production-code denominator.

The Verus core and the FP/SIMD trusted boundaries are unchanged from the recorded
1,806-obligation proof. Their hashes are checked against that existing proof
receipt. This pass adds tests and C hardening; it does not claim new formal
verification of the C shim, complete path/condition coverage, or absence of all
bugs. Caller pointer validity and external callback contracts still apply.
