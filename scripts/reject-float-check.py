#!/usr/bin/env python3
"""Require float64 bit transport, modular prediction, and format acceptance."""
import pathlib,shutil
root=pathlib.Path(__file__).resolve().parents[1]
for name,function,replacement in [('bits','load','0'),('prediction','accumulate','value'),('format','','')]:
    dest=root/'target'/('reject-float-'+name);dest.mkdir(exist_ok=True)
    for path in (root/'src').glob('*.rs'):
        if path.name!='main.rs':shutil.copyfile(path,dest/path.name)
    path=dest/('extended.rs' if name=='format' else 'float64.rs');source=path.read_text()
    if name=='format':
        needle='    let sample_format=if depth==64 {3usize} else {1usize};'
        assert source.count(needle)==1
        source=source.replace(needle,'    if depth==64 {return Err(Error::Unsupported);}\n'+needle)
    else:
        start=source.index('fn '+function+'(');body=source.index('\n{',start);depth=0
        for end in range(body+1,len(source)):
            if source[end]=='{':depth+=1
            elif source[end]=='}':
                depth-=1
                if depth==0:break
        else:raise SystemExit('Missing function body')
        source=source[:body]+'\n{\n    '+replacement+'\n}'+source[end+1:]
    path.write_text(source)
