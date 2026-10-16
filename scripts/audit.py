#!/usr/bin/env python3
"""Source audit: decoder has no proof escapes; five FP primitives are explicit."""
import pathlib,re,sys
root=pathlib.Path(__file__).resolve().parents[1]
for f in (root/(sys.argv[1] if len(sys.argv)>1 else 'src')).rglob('*.rs'):
    if f.name=='main.rs':continue # Explicitly unverified OS adapter.
    text=f.read_text()
    for pattern in [r'\baxiom\s+fn\b',r'\binline_air_stmt\b',r'\bassume\s*\(',r'\badmit\s*\(',r'\bexternal_body\b',r'\bassume_specification\b',
                    r'\bexternal_fn_specification\b',r'verifier::external\b',r'\bunsafe\s*[\{\(]',r'#\[cfg\(']:
        if re.search(pattern,text):raise SystemExit(f'Forbidden proof bypass/configuration: {f}: {pattern}')
print('Core source audit passed (supplements, does not replace, verification).')

# A named, reviewed platform boundary is the only project-owned exception.
import hashlib,json
trusted=json.loads((root/'trusted-float/audit.json').read_text())
assert set(trusted['files'])=={'trusted-float/src/lib.rs','trusted-float/src/model.rs'}
for name,digest in trusted['files'].items():
    assert hashlib.sha256((root/name).read_bytes()).hexdigest()==digest, 'Review changed trusted FP source: '+name
body=(root/'trusted-float/src/lib.rs').read_text()
assert body.count('#[verifier::external_body]')==5
assert re.findall(r'pub fn (\w+)\(',body)==trusted['primitives']
for f in (root/'trusted-float/src').glob('*.rs'):
    assert str(f.relative_to(root)) in trusted['files']
    for pattern in [r'\baxiom\s+fn\b',r'\binline_air_stmt\b',r'\bassume\s*\(',r'\badmit\s*\(',r'\bassume_specification\b',r'\bunsafe\s*[\{\(]',r'#\[cfg\(']:
        assert not re.search(pattern,f.read_text()),(f,pattern)
assert 'external_body' not in (root/'trusted-float/src/model.rs').read_text()
print('Trusted FP boundary: five explicitly assumed primitives; source hashes match the review ledger.')

# User-authorized fixed-value SSE2 boundary. Hash every implementation file;
# unsafe intrinsics and architecture selection are confined to cpu.rs.
ledger=json.loads((root/'trusted-simd/audit.json').read_text())
assert set(ledger['files'])=={'trusted-simd/src/lib.rs','trusted-simd/src/cpu.rs'}
for name,digest in ledger['files'].items():
    assert hashlib.sha256((root/name).read_bytes()).hexdigest()==digest,'Review changed SIMD source: '+name
assert {str(p.relative_to(root)) for p in (root/'trusted-simd/src').rglob('*.rs')}==set(ledger['files'])
body=(root/'trusted-simd/src/lib.rs').read_text()
assert body.count('#[verifier::external_body]')==12
assert re.findall(r'pub fn (\w+)\(',body)==ledger['primitives']
for name in ledger['files']:
    for pattern in [r'\baxiom\s+fn\b',r'\binline_air_stmt\b',r'\bassume\s*\(',r'\badmit\s*\(',r'\bassume_specification\b']:
        assert not re.search(pattern,(root/name).read_text()),(name,pattern)
assert 'external_body' not in (root/'trusted-simd/src/cpu.rs').read_text()
print('Trusted SIMD boundary: eleven integer lane operations and feature dispatch; reviewed source hashes match.')
