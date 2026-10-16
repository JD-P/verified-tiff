#!/usr/bin/env python3
"""Require optimized copies and address calculations to retain pixel semantics."""
import pathlib
import shutil

root = pathlib.Path(__file__).resolve().parents[1]
changes = {
    'copy': ('target.copy_from_slice(source);', '/* deliberately omit the copy */'),
    'target': ('return Some((base_y+row)*stride+i%stride);', 'return Some(0);'),
}
for name, (before, after) in changes.items():
    target = root / 'target' / ('reject-fast-' + name)
    shutil.copytree(root / 'src', target, dirs_exist_ok=True)
    source = target / 'lib.rs'
    text = source.read_text()
    assert text.count(before) == 1, name
    source.write_text(text.replace(before, after))
