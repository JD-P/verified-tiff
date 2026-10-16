#!/usr/bin/env python3
"""Both rejecting valid LZW tails and accepting incomplete pixels must fail."""
import pathlib,shutil
root=pathlib.Path(__file__).resolve().parents[1]
needle='        if left==0 && table.prev!=256 && bits.at_tail() { return Ok(()); }'
for name,replacement in [('acceptance',''),
                         ('soundness','        if table.prev!=256 && bits.at_tail() { return Ok(()); }')]:
    dest=root/'target'/('reject-lzw-tail-'+name);dest.mkdir(exist_ok=True)
    for path in (root/'src').glob('*.rs'):
        if path.name!='main.rs':shutil.copyfile(path,dest/path.name)
    path=dest/'lib.rs';source=path.read_text();assert source.count(needle)==1
    path.write_text(source.replace(needle,replacement))
