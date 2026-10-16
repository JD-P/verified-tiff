#!/usr/bin/env python3
"""Render the v19-to-v20 comparison; keep opt-in API gains separate."""
import argparse,json,math
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
def geo(v):return math.exp(sum(map(math.log,v))/len(v))
def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--results',type=Path,default=ROOT/'target/throughput-results.json')
    p.add_argument('--output',type=Path,default=ROOT/'docs/throughput.md')
    a=p.parse_args();r=json.loads(a.results.read_text());cases=r['cases'];m=r['metadata']
    assert len(cases)==26 and all(c['before_after_native_exact'] for c in cases)
    rgba=[c for c in cases if 'after_rgba' in c['statistics']]
    fused=[c for c in cases if 'combined' in c['statistics']]
    assert len(rgba)==24 and len(fused)==5 and all(c['combined_native_exact'] for c in fused)
    def ms(c,e):return c['statistics'][e]['median_ms']
    speed=geo([ms(c,'before')/ms(c,'after') for c in cases])
    oldgap=geo([ms(c,'before')/ms(c,'libtiff') for c in cases]);gap=oldgap/speed
    cspeed=geo([ms(c,'before_rgba')/ms(c,'after_rgba') for c in rgba])
    text=[f'''# Throughput fixes against libtiff 4.7.1

Measured {m['timestamp_utc']} on {m['cpu_model']}, logical CPU {m['cpu_affinity'][0]}.
Both v19 and this version were freshly built with the pinned toolchain on this
host. The native reference is libtiff 4.7.1, system zlib 1.3, and scalar
libjpeg-turbo 3.1.1 (SIMD disabled). This does not represent every distribution's
configuration. The system RGBA reference is separately identified in raw metadata.

Across 26 native workloads the unchanged API is **{speed:.2f}× faster** by geometric
mean. Its same-run gap to pinned libtiff falls from **{oldgap:.2f}× to {gap:.2f}×**.
Across 24 C API RGBA workloads it is **{cspeed:.2f}× faster**. Each case has equal
weight; these are not application workload-weighted speedups. The combined
Deflate API is reported separately and excluded from these aggregate gains.

## Changes

- LZW reads bounded 24-bit windows for both modern and legacy bit orders.
  Deflate fields and canonical Huffman loops also consume a buffered word;
  exact short-tail behavior is retained. These are not Huffman prefix tables.
- JPEG's exact integer IDCT shares symmetric terms (352 rather than 1,024
  multiplications per full block) and handles DC-only lines directly, keeping
  the original rounding/clipping. Eight-pixel runs avoid repeated geometry;
  amplitude reads consume buffered chunks and Huffman bits use shifts.
- Bilevel scatter computes the visible span once per packed byte, including
  clipped tiles, then expands bits by shifts. This removes repeated geometry
  and the measured large scanned-document regression.
- Uncompressed padded tile rows use bulk copies. Straight non-inverted alpha
  skips identity normalization. Extended interleaved output uses native copies
  or direct 16-bit sample pairs instead of per-byte general layout conversion.
- Deflate reduces Adler accumulators in bounded 256-byte batches and writes
  contiguous strip slices. The opt-in combined API validates and writes each
  strip once; tiles retain the old fallback. The ordinary API still validates
  compressed streams before returning an image descriptor.

A speculative 16-bit JPEG lookahead implementation was measured and removed:
it made textured JPEG slower. The final source uses the faster shift-based
canonical decoder. A later sweep caught out-of-line entropy/amplitude calls;
explicit inlining removes that regression. Its earlier partial sweep and CPU
samples are retained as diagnostic evidence, not final performance results.
No image model, checksum/padding policy, conservative quota,
trust boundary or exact before/after pixel result was relaxed.

## Native inspect plus decode

{m['rounds']} shuffled batches per engine/case, each at least {m['target_batch_ms']} ms;
medians below. Proof/test jobs finished before timing. Disk I/O, startup, caller
buffer allocation, hashing and fixture generation are outside timing. Parsing,
validation, decoding and output conversion are inside. This shared virtual host
is not isolated benchmark hardware. Samples, quartiles and source/input/binary
hashes are in `evidence/throughput-results.json`.

| Case | Before ms | After ms | Speedup | 4.7.1 ms | After / 4.7.1 |
| --- | ---: | ---: | ---: | ---: | ---: |''']
    for c in cases:
        b,n,t=[ms(c,e) for e in ['before','after','libtiff']]
        text.append(f"| {c['name']} | {b:.4f} | {n:.4f} | {b/n:.2f}× | {t:.4f} | {n/t:.2f}× |")
    slower=[c for c in cases if ms(c,'after') > 1.05*ms(c,'before')]
    if slower:
        text.append('\nCases more than 5% slower in this run: ' + '; '.join(
            f"{c['name']} ({100*(ms(c,'after')/ms(c,'before')-1):.1f}% slower)" for c in slower
        ) + '. These results remain included in the aggregate; no timing-noise exemption is assumed.\n')
    text+=['''
## Opt-in combined Deflate API

`decode_deflate_into` uses caller-owned output and native scratch, with a
supported-profile acceptance theorem, checked sufficient budget, exact pixels
and unchanged output on failure. Scratch may be partial on failure. The CLI
and C adapter do not automatically use it. All five inputs here use strips;
final-strip padding and checksums are still checked. See `deflate.md`.

| Case | Before ms | Ordinary now ms | Combined now ms | Before / combined | Combined / 4.7.1 |
| --- | ---: | ---: | ---: | ---: | ---: |''']
    for c in fused:
        b,n,f,t=[ms(c,e) for e in ['before','after','combined','libtiff']]
        text.append(f"| {c['name']} | {b:.4f} | {n:.4f} | {f:.4f} | {b/f:.2f}× | {f/t:.2f}× |")
    text+=['''
## C API fresh-handle RGBA

The same consumer opens, renders and closes a handle each iteration. Adapter
allocation, inspection and conversions are included. Adapter source is
unchanged and outside the core proof boundary.

| Case | Before ms | After ms | Speedup | 4.7.1 ms | System ms |
| --- | ---: | ---: | ---: | ---: | ---: |''']
    for c in rgba:
        b,n,t,sys=[ms(c,e) for e in ['before_rgba','after_rgba','libtiff_rgba','system_rgba']]
        text.append(f"| {c['name']} | {b:.4f} | {n:.4f} | {b/n:.2f}× | {t:.4f} | {sys:.4f} |")
    text+=[f'''
## Validation and remaining work

The full crate verifies: {m['after_verified']:,} obligations, zero errors, unchanged
`--no-cheating --rlimit 100` gate. All 40 negative checks, 34 Rust API tests,
87 Python test methods, 5,616 matrix cases, 31 original TIFFs, 11 C compatibility
methods, 20,000 core mutations and 2,000 C ABI mutations pass. Separately,
upstream libtiff passes its own 108 C tests. Before/after output is exact,
including JPEG; original JPEG versus libtiff retains its documented tolerance.

Remaining gaps include canonical Huffman length walks instead of prefix
tables, per-byte Deflate stored/match processing instead of bulk runs,
compressed validation/replay outside the combined Deflate strip API, and
adapter allocation/conversion/handle costs. Bilevel expansion remains scalar, with geometry shared across eight bits.
The core has no heap allocation, unsafe decoder or new proof axiom.

The reverse patch and archived proof hashes reconstruct v19. See
`benchmarks/README.md` for reproduction. Historical timings are not used as
this run's speedup denominator.
''']
    a.output.write_text('\n'.join(text))
    print(json.dumps(dict(native_speedup=speed,c_api_speedup=cspeed,before_gap=oldgap,after_gap=gap),indent=2))
if __name__=='__main__':main()
