#!/usr/bin/env python3
"""One-pixel JPEG: unit quantization, DC=0 and AC=EOB, output exactly 128."""
from jpeg_fixtures import *
q=marker(219,bytes([0])+bytes([1])*64)
f=marker(192,bytes([8,0,1,0,1,1,1,17,0]))
table=bytes([1])+bytes(15)+bytes([0])
d=marker(196,bytes([0])+table+bytes([16])+table)
s=marker(218,bytes([1,1,0,0,63,0]))
jpeg=b'\xff\xd8'+q+f+d+s+b'\x3f\xff\xd9'
tags=[(256,3,1,1),(257,3,1,1),(258,3,1,8),(259,3,1,7),(262,3,1,1),(273,4,1,134),(277,3,1,1),(278,4,1,1),(279,4,1,len(jpeg)),(284,3,1,1)]
data=b'II*\0\x08\0\0\0'+struct.pack('<H',len(tags))+b''.join(struct.pack('<HHII',*t) for t in tags)+bytes(4)+jpeg
assert len(jpeg)==137 and len(data)==271
(ROOT/'tests/fixtures/jpeg-witness.tif').write_bytes(data)
(ROOT/'tests/fixtures/jpeg-witness.pixels').write_bytes(bytes([128]))
if __name__=='__main__':print('JPEG witness:',len(data),'bytes; entropy offset',data.index(b'\x3f\xff\xd9'))
