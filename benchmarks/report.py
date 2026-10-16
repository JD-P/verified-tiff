#!/usr/bin/env python3
"""Summarize recorded measurements; no estimated or fabricated timings."""
import hashlib,json,math,pathlib,shutil,statistics,subprocess
ROOT=pathlib.Path(__file__).resolve().parents[1];OUT=ROOT/'target/benchmark';EVIDENCE=ROOT/'docs/evidence'
j=json.loads((OUT/'results.json').read_text());cases=j['cases'];meta=j['metadata']
for c in cases:
 assert all(len(v)==meta['rounds'] for v in c['samples'].values())
 assert all(s['seconds']*1000>=meta['target_batch_ms']*.999 for samples in c['samples'].values() for s in samples)
 assert all(v['exact'] or (c['jpeg'] and v['max_abs_error']<=3) for v in c['correctness'].values())
 assert hashlib.sha256((ROOT/c['file']).read_bytes()).hexdigest()==c['input_sha256']
for path,digest in meta['core_source_sha256'].items():assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==digest
meta['system_dependencies']=subprocess.check_output(['ldd',str(OUT/'system')],text=True)
meta['distribution_packages']=subprocess.check_output(['dpkg-query','-W','libtiff6','libjpeg-turbo8','libwebp7','libdeflate0','zlib1g'],text=True)
meta['dependency_sha256']={}
for line in meta['system_dependencies'].splitlines():
 parts=line.split()
 if '=>' in parts and len(parts)>2 and pathlib.Path(parts[2]).is_file():meta['dependency_sha256'][parts[2]]=hashlib.sha256(pathlib.Path(parts[2]).read_bytes()).hexdigest()
meta['harness_sha256']={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [ROOT/'benchmarks/run.py',ROOT/'benchmarks/report.py',ROOT/'benchmarks/libtiff.c',ROOT/'benchmarks/encode.c',ROOT/'examples/benchmark.rs',ROOT/'scripts/build-benchmarks.sh']}
meta['pinned_static_library_sha256']=hashlib.sha256((ROOT/'.tools/tiff-4.7.1/libtiff/.libs/libtiff.a').read_bytes()).hexdigest()
meta['compat_source_sha256']=json.loads((ROOT/'docs/evidence/c-api-source-sha256.json').read_text())
for path,digest in meta['compat_source_sha256'].items():assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==digest,path
(OUT/'results.json').write_text(json.dumps(j,indent=2)+'\n')
def ms(c,e):return c['statistics'][e]['median_ms']
def ratio(c,a,b):return ms(c,a)/ms(c,b)
def gmean(v):return math.exp(statistics.mean(map(math.log,v)))
def interval(c,e):
 s=c['statistics'][e];return f"{s['median_ms']:.3f} [{s['p25_ms']:.3f}, {s['p75_ms']:.3f}]"
core_ratios=[ratio(c,'rust_total','libtiff_total') for c in cases]
rgba_cases=[c for c in cases if c['rgba']]
rgba_ratios=[ratio(c,'adapter_rgba','libtiff_rgba') for c in rgba_cases]
system_ratios=[ratio(c,'adapter_rgba','system_rgba') for c in rgba_cases]
all_exact=all(v['exact'] for c in cases for v in c['correctness'].values())
differences=' '.join(f"{c['label']}: {e} maximum absolute difference {v['max_abs_error']}/255, mean {v['mean_abs_error']:.6f}." for c in cases for e,v in c['correctness'].items() if not v['exact'])
lines=['# Decoder benchmark against libtiff','',
 f"Measured on {meta['cpu_model']} under KVM/Linux, pinned to logical CPU {meta['cpu_affinity'][0]}. The runtime timestamp is {meta['timestamp_utc']}. These are medians of {meta['rounds']} separately warmed process batches, each at least {meta['target_batch_ms']:g} ms or one full operation. Inputs and outputs are resident in memory; disk I/O and process startup are excluded.",'',
 f"**The current implementation is substantially slower.** Across these {len(cases)} chosen workloads, the geometric mean of the Rust core's complete-read time divided by pinned libtiff's is **{gmean(core_ratios):.1f}×** (range {min(core_ratios):.1f}–{max(core_ratios):.1f}×). The complete C API RGBA path is **{gmean(rgba_ratios):.1f}×** slower than pinned libtiff across {len(rgba_cases)} applicable cases, and **{gmean(system_ratios):.1f}×** slower than Ubuntu's system libtiff. These suite summaries do not estimate an application workload distribution.",'',
 '## Complete native-image read','',
 'Rust includes inspection and decoding into reusable buffers. Libtiff includes in-memory open, metadata setup, decoding/canonical output conversion, and close. Both produce the same raster representation. Caller allocation and input loading are excluded; library-internal allocations are included. Higher ratios mean verified-tiff is slower.','',
 '| Workload | MP | Input KiB | Rust ms | libtiff 4.7.1 ms | Ratio |','| --- | ---: | ---: | ---: | ---: | ---: |']
for c in cases:lines.append(f"| {c['label']} | {c['width']*c['height']/1e6:.3f} | {c['input_bytes']/1024:.1f} | {ms(c,'rust_total'):.3f} | {ms(c,'libtiff_total'):.3f} | {ratio(c,'rust_total','libtiff_total'):.1f}× |")
lines+=['','## Inspection versus prepared decoding','','The descriptor/handle and caller output buffers are prepared before the decode-only measurements. Every measured iteration performs decoding again. Rust inspection validates the compressed stream or sample addresses, not just the TIFF directory. Separate medians need not add exactly because they come from different batches.','','| Workload | Rust inspection ms | Rust decode ms | libtiff decode ms | Decode ratio |','| --- | ---: | ---: | ---: | ---: |']
for c in cases:lines.append(f"| {c['label']} | {ms(c,'rust_inspect'):.3f} | {ms(c,'rust_decode'):.3f} | {ms(c,'libtiff_decode'):.3f} | {ratio(c,'rust_decode','libtiff_decode'):.1f}× |")
lines+=['','## Complete RGBA read through the C API','','The identical C consumer opens a fresh in-memory handle, renders with `TIFFReadRGBAImageOriented`, and closes it. The adapter therefore cannot answer timed iterations from its previous decoded-raster cache. Float64 and LogLuv have no supported adapter RGBA conversion and are omitted.','','| Workload | Adapter ms | Pinned libtiff ms | System libtiff ms | Adapter / pinned | Adapter / system |','| --- | ---: | ---: | ---: | ---: | ---: |']
for c in rgba_cases:lines.append(f"| {c['label']} | {ms(c,'adapter_rgba'):.3f} | {ms(c,'libtiff_rgba'):.3f} | {ms(c,'system_rgba'):.3f} | {ratio(c,'adapter_rgba','libtiff_rgba'):.1f}× | {ratio(c,'adapter_rgba','system_rgba'):.1f}× |")
lines+=['','## What explains the gap','','Source inspection shows several concrete costs. This is an explanation consistent with the timing breakdown, not a sampled CPU profile or a measured speedup from an optimization:','',
 '- The basic uncompressed path calls `sample` for every output byte during inspection and again during decoding. Each call recomputes geometry using divisions/remainders, reads offset/count metadata, and checks ranges. Ordinary uncompressed libtiff reads can largely copy whole runs of bytes.',
 '- The C adapter inspects at open and reinspects before its first decode, then allocates canonical/scratch/render buffers, converts the canonical image to RGBA, and copies it to the caller. The extra inspection is visible in cases where validation is expensive.',
 '- Compressed core paths use conservative scalar implementations and per-byte/per-pixel work and scatter operations. The measured JPEG and other codec gaps remain even after removing initial inspection.',
 '- Proofs and ghost code are erased. These numbers measure this implementation and adapter, not an intrinsic runtime cost of formal verification or Rust. A verified segment-wise copy path and reuse of the inspected descriptor are concrete optimization candidates, but no such changes or projected speedups were made in this task.','',
 '## Correctness and scope','',
 f"All timed batches were checked against their engine's validated output hash. {'Every native and RGBA comparison was byte-exact, including the JPEG cases; no JPEG tolerance was used.' if all_exact else 'Lossless comparisons were byte-exact; the only non-exact comparisons were: '+differences} Independent libtiff writers generated the main TIFF fixtures; the existing independent Python writers generated the supported WebP and LogLuv subsets. Four upstream fixtures were used unchanged.",'',
 'The measured core source hashes are checked against the recorded full-crate proof summary. See validation.md for proof scope and current regression evidence. This report measures the current binary against libtiff; use optimization.md for controlled before/after comparisons.','',
 'The primary reference is real libtiff 4.7.1, optimized at `-O2`, with pinned JPEG/WebP dependencies and system zlib. The pinned libjpeg-turbo build intentionally disables SIMD for numerical regression tests. The system baseline is Ubuntu libtiff 4.5.1 plus distribution codecs, including libjpeg-turbo and libdeflate. Different versions and codec configurations confound any SIMD-only attribution. Rust uses Cargo release (`opt-level=3`), and the C benchmark consumers use GCC `-O3`. No native-CPU specialization or sanitizer is enabled.','',
 'All measurements are single-threaded, warm-cache and in a shared VM. Small cached uncompressed images particularly favor highly optimized memory copies; cold storage, larger working sets, concurrency, peak RSS, and allocation counts are not measured. The selected synthetic patterns and four real regressions are not representative of every TIFF workload. The generous finite benchmark limits are 256 MiB per buffer and 10^12 core work units; checks remain enabled. Default limits would reject some of these large workloads.','',
 '## Variation: median [25th, 75th percentile] in milliseconds','','| Workload | Rust complete read | libtiff complete read | Adapter RGBA |','| --- | ---: | ---: | ---: |']
for c in cases:lines.append(f"| {c['label']} | {interval(c,'rust_total')} | {interval(c,'libtiff_total')} | {interval(c,'adapter_rgba') if c['rgba'] else '—'} |")
lines+=['','## Reproduce and inspect','','See [the harness instructions](../benchmarks/README.md). Raw samples and source/input/binary/dependency hashes are in [benchmark-results.json](evidence/benchmark-results.json). Generated inputs are reproducible; the benchmark does not require shipping their uncompressed pixel buffers. All numbers above are computed from the recorded measurements.','','Compiler: `'+meta['compiler']+'`. System package versions:','','```',meta['distribution_packages'].strip(),'```','']
(ROOT/'docs/benchmark.md').write_text('\n'.join(lines))
for src,dst in [('results.json','benchmark-results.json'),('inputs.json','benchmark-inputs.json'),('build.log','benchmark-build.log'),('run.log','benchmark-run.log')]:shutil.copyfile(OUT/src,EVIDENCE/dst)
print(json.dumps({'cases':len(cases),'rgba_cases':len(rgba_cases),'all_exact':all_exact,'core_geomean_ratio':gmean(core_ratios),'rgba_pinned_geomean_ratio':gmean(rgba_ratios),'rgba_system_geomean_ratio':gmean(system_ratios)},indent=2))
