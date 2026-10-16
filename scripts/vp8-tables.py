#!/usr/bin/env python3
"""Regenerate format probability/quantization constants from pinned libwebp."""
import pathlib,re
root=pathlib.Path(__file__).resolve().parents[1]
src=root/'.tools/libwebp-1.6.0/src/dec'
def table(file,name,n):
 s=(src/file).read_text();s=s[s.index('=',s.index(name))+1:];s=s[:s.index(';')]
 s=re.sub(r'//[^\n]*','',s);values=[int(v) for v in re.findall(r'\d+',s)]
 assert len(values)==n,(name,len(values),n)
 return values
text='''//! VP8 format constants from libwebp 1.6.0 (Google Inc.).
//! BSD license and patent grant: third-party/libwebp/{COPYING,PATENTS,AUTHORS}.
//! Regenerate using scripts/vp8-tables.py; production uses no native decoder.
use super::*;
verus! {
'''
for file,name,label,n,ty in [('tree_dec.c','CoeffsProba0','COEFF_DEFAULT',1056,'u8'),('tree_dec.c','CoeffsUpdateProba','COEFF_UPDATE',1056,'u8'),('tree_dec.c','kBModesProba','BMODE',900,'u8'),('quant_dec.c','kDcTable','DC_QUANT',128,'u16'),('quant_dec.c','kAcTable','AC_QUANT',128,'u16')]:
 values=table(file,name,n);text+=f'pub(super) const {label}:[{ty};{n}]=[\n'
 text+='\n'.join('    '+','.join(map(str,values[i:i+22]))+',' for i in range(0,n,22))+'\n];\n'
text+='}\n';(root/'src/vp8_tables.rs').write_text(text)
