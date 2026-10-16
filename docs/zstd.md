# Native Zstandard decoding

The separate `native_zstd` Rust API and C reader support TIFF Compression
50000 without linking a native Zstandard library. Predictor 1, integer
Predictor 2 and floating Predictor 3 compose with the existing verified sample
transforms. The original whole-image Rust `Raster`/CLI acceptance union is
unchanged; this is a native segment API with local proofs, used by the
unverified C/BigTIFF adapter.

## Stream profile

One complete, dictionary-free Zstandard frame is required per strip or tile.
The model follows the [Zstandard 1.5.7 format specification](https://github.com/facebook/zstd/blob/v1.5.7/doc/zstd_compression_format.md),
format revision 0.4.3. It implements:

- Single-segment and explicit-window headers, all content-size encodings,
  optional/unknown content size, zero dictionary IDs and the optional XXH64
  checksum. The reserved header bit is rejected; the unused bit is ignored.
- Raw, RLE and compressed blocks, bounded by the declared window and 128 KiB.
- Raw, RLE, Huffman-compressed and treeless literals. Both one-stream and
  four-stream layouts, direct and FSE-compressed weight descriptions, implicit
  final weights and exact bitstream termination are checked.
- Predefined, RLE, normalized FSE and repeated tables for each sequence alphabet.
  Literal/match extra bits, offsets through code 31, initial and subsequent
  FSE states, repeated-offset history and zero-literal offset rules are modeled.
- Overlapping copies and history across blocks. Tables and repeated offsets
  persist across compressed blocks; raw/RLE blocks do not reset them.
- Declared content size, checksum and exact frame ending. Missing payload,
  invalid state/table references, zero offsets, output overruns and trailing
  bytes are rejected.

Nonzero/external dictionaries, skippable frames and concatenated frames are
outside this segment profile. A zero dictionary ID does not provide an implicit
external dictionary. There is no permissive checksum bypass or native fallback.

The C adapter retains the bounded short-final-strip policy: a frame may contain
padding up to a complete declared strip. The entire frame, including padding
and checksum, is validated before the requested native prefix is exposed.
Tiles require their full native storage rectangle. Sample transformations run
after decompression and strip-prefix selection. Both TIFF byte orders,
Classic/BigTIFF, separate planes and both FillOrders are covered.

## Resource and failure contracts

The core borrows encoded input and uses caller-provided output and literal
scratch. `scratch_len(cap)` is `min(cap, 131072)`. FSE and Huffman tables use
fixed arrays; there is no heap allocation, FFI or runtime recursion in the
core. Exact machine stack placement/exhaustion is not proved.

For encoded byte count `N`, output ceiling `C` and structurally counted
compressed blocks `B`, the reserved abstract work is:

`8*N + 64*C + 50000*B + 128`.

All arithmetic is checked. The input-scanning allowance is charged before
framing inspection; the remaining allowance is charged before pixel writes.
`work_limit` validates framing and computes this bound, not entropy or checksum
validity. These are conservative work units, not CPU instructions or a wall
clock deadline. Per-handle C allocation limits also charge full output capacity
and literal scratch; truncating a cached final-strip prefix does not release
the retained allocation's charge.

`decode` has exact success/acceptance conditions relative to its byte-string
model and available buffers/work. Successful output equals that model and work
is conserved. On failure, only output below `cap` and scratch below
`scratch_len(cap)` may change. Insufficient buffers or the full work allowance
leave both buffers unchanged. The C adapter uses private staging buffers and
therefore retains caller-output failure atomicity.

## Proof boundary

The checked modules separate bit fields, normalized distributions, modular FSE
spreading, weight streams, canonical literal trees, literal sections, sequence
symbols, history execution, framing and XXH64. Pure models use immutable byte
strings and tables; executable code refines them using fixed arrays and caller
buffers. The history model states overlapping copies as periodic slices of the
previous byte string, separately from the incremental copy loop.

A concrete raw frame witnesses nonempty acceptance. Six negative checks alter
reverse bit extraction, FSE spreading, repeated-offset resolution, overlapping
copy output, checksum rejection and all successful decoding. Each must fail
its semantic/acceptance contract. Existing proof gates remain required.

This does not verify C varargs, foreign pointers, I/O callbacks, resource-account
atomics, TIFF orchestration or the end-to-end BigTIFF/native pipeline. The XXH64
proof concerns exact specified integer operations; XXH64 is not an
adversarial-integrity or cryptographic authentication mechanism.

## Independent tests

`tests/zstd.py` compares 260 reference-encoded frames across five compression
levels, varied alphabets, checksums, unknown sizes and multiple blocks. Five
independent byte-built streams exercise direct/four-stream literal trees,
treeless reuse, RLE literals and repeated RLE sequence tables. Observed feature
counts cover all block/literal/table modes and both tree-description encodings.
All truncations of selected streams, strict trailers, dictionaries, checksums,
size limits and 1,500 deterministic mutations are checked; accepted mutations
must match the independent decoder. These are not coverage-guided fuzzing.

`tests/zstd_api.rs` checks exact/one-short work, output/scratch suffixes,
undersized buffers, raw/RLE/mixed/empty frames, unknown size and an independently
built overlapping sequence stream.

`compat/tests/zstd_compat.py` adds 256 independently specified TIFF files; 224
also compare with libtiff 4.7.1 built with Zstandard 1.5.7. The remaining 32 use
64-bit Predictor 2, which libtiff rejects, and compare with independent native
sample bytes. Additional checks cover RGBA, callback temporary contexts and
bounded final-strip padding. Reference libraries are checksum-pinned test
only dependencies. A separate libtiff build preserves the earlier oracle.
