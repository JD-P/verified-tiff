#!/usr/bin/env python3
"""Regenerate the two 101-byte TIFFs modeled by src/fax_witness.rs."""
import struct
from fax_fixtures import ROOT,encode
for group in (3,4):
    payload=encode([[0]],group=group,terminate=False)
    tags=[(256,3,1),(257,3,1),(259,3,group),(262,3,0),(273,4,98),(278,4,1),(279,4,len(payload))]
    data=b'II'+struct.pack('<HIH',42,8,7)+b''.join(struct.pack('<HHII',k,t,1,v) for k,t,v in tags)+bytes(4)+payload
    data+=bytes(101-len(data))
    (ROOT/f'tests/fixtures/fax-g{group}-witness.tif').write_bytes(data)
    (ROOT/f'tests/fixtures/fax-g{group}-witness.pixels').write_bytes(bytes([255]))
    print(f'Group {group} witness: {len(data)} bytes, white pixel, payload {payload.hex()}')
