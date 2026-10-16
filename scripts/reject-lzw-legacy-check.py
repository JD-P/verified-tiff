#!/usr/bin/env python3
"""Legacy selection, widths and full-table acceptance are substantive claims."""
import pathlib,shutil
root=pathlib.Path(__file__).resolve().parents[1]
mutations={
    'selection':('    count>=2 && s[start]==0 && s[start+1]%2==1','    false'),
    'width':('if legacy {if self.next<512 {9} else if self.next<1024 {10} else if self.next<2048 {11} else {12}}',
             'if legacy {if self.next<511 {9} else if self.next<1023 {10} else if self.next<2047 {11} else {12}}'),
    'full':('if self.prev!=256 && self.next==4096 && !legacy { return Err(Error::Invalid); }',
            'if self.prev!=256 && self.next==4096 { return Err(Error::Invalid); }'),
}
for name,(needle,replacement) in mutations.items():
    dest=root/'target'/('reject-lzw-legacy-'+name);dest.mkdir(exist_ok=True)
    for path in (root/'src').glob('*.rs'):
        if path.name!='main.rs':shutil.copyfile(path,dest/path.name)
    path=dest/'lib.rs';source=path.read_text();assert source.count(needle)==1
    path.write_text(source.replace(needle,replacement))
