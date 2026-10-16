#!/usr/bin/env python3
"""Reproduce committed JPEG API/fuzz fixtures and independent reference pixels."""
from jpeg_fixtures import *
CASES={
 'jpeg-gray':dict(ch=1,w=9,h=7,mode='self'),
 'jpeg-rgb':dict(photo=2,w=9,h=7,mode='shared',le=False),
 'jpeg-ycbcr':dict(w=17,h=19,mode='shared',restart=1),
 'jpeg-tile':dict(w=17,h=19,tiled=True,mode='self'),
 'ojpeg-tables':dict(w=17,h=19,mode='old-tables',hs=2,vs=1),
 'ojpeg-header':dict(w=17,h=19,mode='old-header',holes=True),
}
with tempfile.TemporaryDirectory() as tmp:
 for name,opts in CASES.items():
  refs=[];data=make_jpeg_tiff(**opts,references=refs)
  (ROOT/f'tests/fixtures/{name}.tif').write_bytes(data)
  (ROOT/f'tests/fixtures/{name}.pixels').write_bytes(generated_oracle(opts,refs,pathlib.Path(tmp)))
