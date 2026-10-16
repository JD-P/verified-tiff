#!/usr/bin/env python3
"""Ensure decoder-side proof escapes are still rejected by the source gate."""
import pathlib,subprocess,tempfile
root=pathlib.Path(__file__).resolve().parents[1]
with tempfile.TemporaryDirectory(dir=root/'target',prefix='audit-negative-') as temp:
 p=pathlib.Path(temp)/'injected.rs'
 for text in ['proof fn bad() { assume(false); }','#[verifier::external_body] fn bad() {}','pub broadcast axiom fn bad();']:
  p.write_text(text)
  r=subprocess.run(['python3','scripts/audit.py',temp],cwd=root,capture_output=True,text=True)
  assert r.returncode and 'Forbidden proof bypass' in r.stdout+r.stderr
print('Decoder source audit rejected injected assume, external_body, and axiom.')
