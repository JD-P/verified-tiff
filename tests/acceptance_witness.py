#!/usr/bin/env python3
"""Regenerate the five 200-byte extended acceptance proof fixtures."""
from pathlib import Path
import os
os.chdir(Path(__file__).resolve().parents[1])
import struct

for mode in range(5):
    b=bytearray(200);b[:8]=b'II*\0\x08\0\0\0';struct.pack_into('<H',b,8,11)
    planar=mode==2;df=mode>=3
    tags=[(256,3,1,1),(257,3,1,1),(258,3,3 if planar else 1,146 if planar else 1 if mode in(0,4) else 16 if mode==1 else 8),(259,3,1,8 if df else 1),(262,3,1,3 if mode==0 else 2 if planar else 1),(273,4,3 if planar else 1,152 if planar else 188),(277,3,1,3 if planar else 1),(278,4,1,1),(279,4,3 if planar else 1,164 if planar else 12 if df else 2 if mode==1 else 1),(284,3,1,2 if planar else 1),(320,3,6,176)]
    for j,t in enumerate(tags):struct.pack_into('<HHII',b,10+12*j,*t)
    struct.pack_into('<HHH',b,146,8,8,8);struct.pack_into('<III',b,152,188,189,190);struct.pack_into('<III',b,164,1,1,1)
    struct.pack_into('<HHHHHH',b,176,0x1100,0x2200,0x3300,0x4400,0x5500,0x6600)
    b[188:191]=bytes([33,66,99])
    if df:b[188:200]=bytes([120,1,1,1,0,254,255,65,0,66,0,66])
    name=['palette','gray16','planar','deflate','deflate-bilevel'][mode]
    Path(f'tests/fixtures/acceptance-{name}.tif').write_bytes(b)
    Path(f'tests/fixtures/acceptance-{name}.pixels').write_bytes(bytes([17,51,85] if mode==0 else [66,33] if mode==1 else [33,66,99] if mode==2 else [65] if mode==3 else [0]))
