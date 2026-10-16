#!/usr/bin/env python3
"""An always-error codec must fail its stream acceptance equivalence."""
import pathlib,shutil
root=pathlib.Path(__file__).resolve().parents[1]
dest=root/'target/reject-deflate';dest.mkdir(exist_ok=True)
for path in (root/'src').glob('*.rs'):
    if path.name!='main.rs':shutil.copyfile(path,dest/path.name)
p=dest/'deflate.rs';s=p.read_text();start=s.index('pub(super) fn validate(');body=s.index('\n{\n',start)
depth=0
for i in range(body+1,len(s)):
    if s[i]=='{':depth+=1
    elif s[i]=='}':
        depth-=1
        if depth==0:break
else:raise SystemExit('Could not locate codec validator body')
p.write_text(s[:body]+'\n{\n    Err(Error::Invalid)\n}'+s[i+1:])
