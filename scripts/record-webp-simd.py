#!/usr/bin/env python3
"""Record matching whole-crate proof, WebP SIMD regressions and benchmark evidence."""
import datetime, hashlib, json, pathlib, re, shutil, statistics

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUT = ROOT / 'target/webp-simd'
E = ROOT / 'docs/evidence/webp-simd'

def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def read(p): return json.loads(p.read_text())
def rust(s): return sum(map(int, re.findall(r'test result: ok\. (\d+) passed; 0 failed', s)))
def python(s): return sum(map(int, re.findall(r'(?m)^Ran (\d+) tests? in ', s)))

def main():
    proof = read(ROOT / 'target/verification-summary.json')
    assert proof['success'] and proof['errors'] == 0 and proof['is-verifying-entire-crate']
    for source in [proof['core_sources_sha256'], proof['trusted_float']['source_sha256'], proof['trusted_simd']['source_sha256']]:
        for name, digest in source.items(): assert sha(ROOT / name) == digest, name
    for boundary in ['float', 'simd']:
        folder = ROOT / ('target/trusted-' + boundary)
        recorded = proof['trusted_' + boundary]
        assert sha(folder / ('libverified_tiff_' + boundary + '.rlib')) == recorded['rlib_sha256']
        assert sha(folder / (boundary + '.vir')) == recorded['vir_sha256']
    gate = read(ROOT / 'target/parallel-verification/receipt.json')
    assert len(gate['results']) == 25 and all(r['exit_code'] == 0 for r in gate['results'])
    assert gate['verify_sh_sha256'] == sha(ROOT / 'scripts/verify.sh')
    gates = (ROOT / 'target/parallel-proof-gate.log').read_text()
    assert len(re.findall(r'(?m)^.*(?:check|gate):.*rejected.*$', gates)) == 115

    logs = ['core-final.log', 'compat-final.log', 'sanitizers-final.log', 'scalar-final.log', 'lanes-full.log', 'fuzz-final.log', 'corpus-final.log']
    texts = {n: (OUT / n).read_text() for n in logs}
    for name, body in texts.items():
        assert not re.search(r'(?m)^(FAILED|ERROR:|FAIL:|Traceback)', body), name
    counts = (rust(texts['core-final.log']), python(texts['core-final.log']), python(texts['compat-final.log']), python(texts['sanitizers-final.log']))
    assert counts == (62, 97, 63, 63), counts
    assert (rust(texts['scalar-final.log']), python(texts['scalar-final.log'])) == (7, 3)
    assert rust(texts['lanes-full.log']) == 4
    for name in ['core-final.log', 'scalar-final.log']:
        for expected in ['192 encoder/reference', '50 independent transform/group', '140 complete images']:
            assert expected in texts[name], (name, expected)
    assert 'iterations=20000 ' in texts['fuzz-final.log']
    corpus = read(ROOT / 'target/original-corpus.json')
    assert corpus['accepted'] == corpus['total'] == 31
    upstream = ROOT.parent / 'upstream-setup.log'
    assert re.search(r'# PASS:\s+108', upstream.read_text())

    results = read(OUT / 'results.json')
    assert len(results['records']) == 116
    for name, digest in results['metadata']['source_sha256'].items(): assert sha(ROOT / name) == digest, name
    for name, digest in results['metadata']['benchmark_sha256'].items(): assert sha(ROOT / name) == digest, name
    assert results['metadata']['proof_sha256'] == sha(ROOT / 'target/verification-summary.json')
    assert results['metadata']['binary_sha256']['after-library'] == sha(ROOT / 'target/compat/libtiff.so.6')
    assert results['metadata']['rounds'] == 9 and results['metadata']['milliseconds'] == 200
    for r in results['records']: assert r['correctness']['before'] == r['correctness']['after']
    timed = [r for r in results['records'] if 'statistics' in r]
    assert len(timed) == 24 and all('webp' in r['case']['name'] for r in timed)
    assert all(r['difference']['max_channel_error'] == 0 for r in timed)

    for name in ['lossless', 'lossy', 'lossless-texture']:
        profile = read(OUT / ('profile-' + name + '-final.json'))
        row = next(r for r in timed if r['case']['name'] == 'webp-' + name and r['mode'] == 'native')
        assert profile['diagnostic_run']['hash'] == row['correctness']['after']['hash']
        assert profile['input_sha256'] == row['case']['sha256']
        assert profile['binary_sha256'] == results['metadata']['binary_sha256']['after']
    kernels = [json.loads(s) for s in (OUT / 'kernel-timings.jsonl').read_text().splitlines()]
    assert len(kernels) == 54 and all(r['ns'] > 0 for r in kernels)

    E.mkdir(exist_ok=True)
    for name in logs + ['proof-final.log', 'benchmark-final.log', 'results.json', 'baseline-patch-check.log', 'kernel-timings.jsonl', 'profile-lossless-final.json', 'profile-lossless-final.pcs', 'profile-lossy-final.json', 'profile-lossy-final.pcs', 'profile-lossless-texture-final.json', 'profile-lossless-texture-final.pcs', 'kernels-final.asm', 'sanitizer-permission-failure.log', 'sanitizers-rerun.log', 'scalar-import-failure.log', 'gate-first-attempt.json', 'gate-first-attempt.log', 'gate-rerun-receipt.json', 'span-gates-rerun.log', 'simd-gates-final.log', 'span-gates-final.log', 'simd-gates-assertion-with-limit.log', 'color-position-assertion-with-limit.log', 'benchmark-fixture-first-attempt.log']:
        shutil.copy2(OUT / name, E / name)
    shutil.copy2(upstream, E / 'upstream.log')
    for source, dest in [
        ('target/verification.json', 'verification.json'),
        ('target/verification-summary.json', 'verification-summary.json'),
        ('target/parallel-proof-gate.log', 'full-proof-gate.log'),
        ('target/parallel-verification/receipt.json', 'parallel-receipt.json'),
        ('target/verus-version.txt', 'verus-version.txt'),
        ('target/false-contract.log', 'false-contract.log'),
        ('target/original-corpus.json', 'original-corpus.json'),
        ('target/webp-simd/before/build-receipt.json', 'baseline-build-receipt.json'),
        ('docs/evidence/lerc-throughput/verification-summary.json', 'baseline-verification-summary.json')]:
        shutil.copy2(ROOT / source, E / dest)
    for pattern in ['reject-webp-simd-*.log', 'reject-webp-spans-*.log']:
        for p in (ROOT / 'target').glob(pattern): shutil.copy2(p, E / p.name)
    sources = {str(p.relative_to(ROOT)): sha(p)
        for d in ['src', 'compat', 'tests', 'scripts', 'benchmarks', 'examples', 'trusted-float', 'trusted-simd']
        for p in sorted((ROOT / d).rglob('*'))
        if p.is_file() and '__pycache__' not in p.parts and p.suffix != '.pyc'}
    sources.update({n: sha(ROOT / n) for n in ['Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', 'toolchain-lock.json']})
    categories = {}
    for kind in ['lossless', 'lossy']:
        rows = [r for r in timed if kind in r['case']['name']]
        assert len(rows) == 12
        categories[kind] = dict(cases=len(rows), geomean_speedup=statistics.geometric_mean(r['speedup'] for r in rows), geomean_runtime_vs_libtiff=statistics.geometric_mean(r['vs_libtiff'] for r in rows))
    summary = dict(
        utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
        proof={k: proof[k] for k in ['verified', 'errors', 'success', 'is-verifying-entire-crate']},
        negative_gates=115, rust_tests=62, core_python_tests=97, c_tests=63, sanitized_c_tests=63,
        forced_scalar_rust_tests=7, forced_scalar_python_matrices=3, simd_primitive_tests=4,
        upstream_tests=108, corpus_images=31, core_fuzz_mutations=20000,
        exact_before_after_cases=116,
        exact_libtiff_cases=sum(r['difference']['max_channel_error'] == 0 for r in results['records']),
        timed_webp_cases=24, webp_geomean_speedup=statistics.geometric_mean(r['speedup'] for r in timed),
        categories=categories, source_sha256=sources,
        evidence_sha256={p.name: sha(p) for p in sorted(E.iterdir()) if p.is_file() and p.name != 'summary.json'},
        sanitizer_note='C shim and consumers are instrumented; Rust is not. LeakSanitizer is disabled.',
        validation_note='The core proof is conditional on five trusted FP operations and eleven trusted SIMD integer operations plus CPU dispatch. Intrinsic bodies are not proved. Full image and cropped outputs are exact before/after, including JPEG. Forced-scalar testing ran on x86-64; no ARM or 32-bit execution claim. Initial C sanitizer attempts failed on missing execute permission for generated consumers; the complete final suite passed after the harness restored permission. An initial scalar matrix import failure was corrected and the complete scalar suite rerun. Initial failure logs are preserved. The new negative-test harness was refined to recognize the tile-span precondition failure and to use concrete color counterexamples and isolated solver contexts; solver-limit failures are not counted as valid rejections. The first benchmark fixture attempt passed a full 64-row buffer for an odd-height final WebP strip to the reference writer; libtiff rejected it. The odd-size fixture now uses one exact-height strip, and the complete comparison was restarted.',
        trusted_float=proof['trusted_float'], trusted_simd=proof['trusted_simd'])
    (E / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps({k: v for k, v in summary.items() if k not in ['source_sha256', 'evidence_sha256', 'trusted_float', 'trusted_simd']}, indent=2))

if __name__ == '__main__': main()
