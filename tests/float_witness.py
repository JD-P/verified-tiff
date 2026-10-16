#!/usr/bin/env python3
"""Regenerate the two small float64 acceptance witnesses (same fixed LZW stream)."""
from pathlib import Path
import struct
ROOT=Path(__file__).resolve().parents[1]

def make(predict):
    raw=bytes([255]*8+[2]+[0]*7)
    codes=[c for b in raw for c in (256,b)]+[257]
    bits=''.join(f'{c:09b}' for c in codes)
    encoded=int(bits.ljust((len(bits)+7)//8*8,'0'),2).to_bytes((len(bits)+7)//8,'big')
    b=bytearray(122);b[:8]=b'II*\0\x08\0\0\0';struct.pack_into('<H',b,8,9)
    tags=[(256,3,1,2),(257,3,1,1),(258,3,1,64),(259,3,1,5),(262,3,1,1),
          (273,4,1,122),(279,4,1,len(encoded)),(317,3,1,2 if predict else 1),(339,3,1,3)]
    for j,t in enumerate(tags):struct.pack_into('<HHII',b,10+12*j,*t)
    b.extend(encoded)
    out=bytes([255]*8)+(1 if predict else 2).to_bytes(8,'little')
    return bytes(b),out,codes

if __name__=='__main__':
    for predict in (False,True):
        data,out,_=make(predict);name='float64-predict2' if predict else 'float64-plain'
        (ROOT/f'tests/fixtures/acceptance-{name}.tif').write_bytes(data)
        (ROOT/f'tests/fixtures/acceptance-{name}.pixels').write_bytes(out)
