import json,pathlib,re,subprocess,sys
p=pathlib.Path(sys.argv[1]);api=json.loads((p/'api.json').read_text())
nm=subprocess.check_output(['nm','-D','--defined-only',str(p/'libtiff.so.6')],text=True)
exports={line.split()[-1].split('@')[0] for line in nm.splitlines()}
expected={f['name'] for f in api['functions']}|{'VTIFFLastErrorCode','VTIFFLastError','VTIFFClearError','VTIFFOpenOptionsSetLimits'}
missing=expected-exports;assert not missing,sorted(missing)
assert not any(n.startswith('vt_') for n in exports),'private Rust bridge leaked'
deps=subprocess.check_output(['readelf','-d',str(p/'libtiff.so.6')],text=True)
needed=re.findall(r'\(NEEDED\).*?\[(.*?)\]',deps)
assert not any(re.search(r'lib(tiff|jpeg|webp|z|zstd|lerc|Lerc|openjp2)\.',n) for n in needed),needed
assert 'Library soname: [libtiff.so.6]' in deps
(p/'export-check.json').write_text(json.dumps(dict(public_symbols=len(api['functions']),missing=[],needed=needed,private_bridge_exported=False),indent=2)+'\n')
print('All public header symbols exported; no native TIFF/codec dependency; private bridge hidden')
