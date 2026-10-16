#!/usr/bin/env python3
"""Regenerate compact known-pixel fax API/mutation fixtures."""
from fax_fixtures import ROOT,make_fax
for name,opts in [('fax-g3',dict(group=3)),('fax-mixed',dict(group=3,mixed=True,aligned=True)),('fax-g4-tile',dict(group=4,tiled=True,le=False)),('fax-lsb',dict(group=4,fill=2,white=True)),('fax-noeol',dict(group=3,missing=True,terminate=False))]:
    data,expected=make_fax(**opts)
    (ROOT/f'tests/fixtures/{name}.tif').write_bytes(data)
    (ROOT/f'tests/fixtures/{name}.pixels').write_bytes(expected[0])
