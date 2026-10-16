#!/usr/bin/env python3
"""Mutate the new Deflate shortcuts; their exact-value contracts must fail."""
import pathlib
import shutil

root = pathlib.Path(__file__).resolve().parents[1]
changes = {
    'huffman': ('Ok(self.symbols[self.offsets[len]+rank] as usize)', 'Ok(0)'),
    'stored': ('return Ok(byte as usize);', 'return Ok(0);'),
}
for name, (before, after) in changes.items():
    target = root / 'target' / ('reject-profile-' + name)
    shutil.copytree(root / 'src', target, dirs_exist_ok=True)
    source = target / 'deflate_bits.rs'
    text = source.read_text()
    assert text.count(before) == 1, name
    source.write_text(text.replace(before, after))
