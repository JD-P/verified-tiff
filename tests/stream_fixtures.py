#!/usr/bin/env python3
"""Independent zlib fixtures for resumable batch/window regression tests."""
import hashlib,json,pathlib,zlib
root=pathlib.Path(__file__).resolve().parent/'fixtures'
raw=bytes((i*73+(i//97)*19+i//4093)&255 for i in range(130001))
files={}
for name,level,strategy in [('stored',0,zlib.Z_DEFAULT_STRATEGY),('fixed',6,zlib.Z_FIXED),('dynamic',6,zlib.Z_DEFAULT_STRATEGY)]:
 encoder=zlib.compressobj(level,zlib.DEFLATED,15,8,strategy);wire=encoder.compress(raw)+encoder.flush()
 assert zlib.decompress(wire)==raw
 p=root/('stream-batch-'+name+'.zlib');p.write_bytes(wire);files[p.name]={'bytes':len(wire),'sha256':hashlib.sha256(wire).hexdigest()}
(root/'stream-batch.json').write_text(json.dumps({'generator':'tests/stream_fixtures.py','zlib':zlib.ZLIB_RUNTIME_VERSION,'raw_bytes':len(raw),'raw_sha256':hashlib.sha256(raw).hexdigest(),'files':files},indent=2)+'\n')
print(files)
