#!/usr/bin/env python3
"""Reject all extended inputs, or remove the public wrapper's extended fallback."""
import pathlib, shutil
root=pathlib.Path(__file__).resolve().parents[1]
for name,module,function in [("extended","extended_image.rs","inspect_extended"),("raster","reader.rs","inspect_raster")]:
    dest=root/"target"/("reject-"+name);dest.mkdir(exist_ok=True)
    for path in (root/"src").glob("*.rs"):
        if path.name!="main.rs":shutil.copyfile(path,dest/path.name)
    path=dest/module;source=path.read_text()
    start=source.index("pub fn "+function+"<'a>");body=source.index("\n{\n",start)
    depth=0
    for end in range(body+1,len(source)):
        if source[end]=="{":depth+=1
        elif source[end]=="}":
            depth-=1
            if depth==0:break
    else:raise SystemExit("Could not locate inspector body")
    replacement="Err(Error::Unsupported)" if name=="extended" else "match inspect(s,page,limits,budget) { Ok(i)=>Ok(Raster {inner:Kind::Basic(i)}), Err(e)=>Err(e) }"
    path.write_text(source[:body]+"\n{\n    "+replacement+"\n}"+source[end+1:])
