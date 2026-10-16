#!/usr/bin/env python3
"""Semantic negative checks: reject valid scans, reject every image, wrong IDCT."""
import pathlib,shutil
root=pathlib.Path(__file__).resolve().parents[1]
for name,module,function,replacement in [
 ('jpeg-scan','jpeg_scan.rs','validate','Err(Error::Invalid)'),
 ('jpeg-image','jpeg_tiff.rs',"inspect_jpeg_tiff<'a>",'Err(Error::Unsupported)'),
 ('jpeg-idct','jpeg_transform.rs','idct','[0u8;64]'),
 ('jpeg-raster','reader.rs',"inspect_raster<'a>",'match inspect_extended(s,page,limits,budget) { Ok(i)=>Ok(Raster {inner:Kind::Extended(i)}), Err(e)=>Err(e) }')]:
 dest=root/'target'/('reject-'+name);dest.mkdir(exist_ok=True)
 for path in (root/'src').glob('*.rs'):
  if path.name!='main.rs':shutil.copyfile(path,dest/path.name)
 path=dest/module;source=path.read_text()
 if name=='jpeg-raster':
  source=source.replace('Ok(i)=>Ok(Raster {inner:Kind::Jpeg(i)})','Ok(_)=>Err(Error::Unsupported)');path.write_text(source);continue
 start=source.index('fn '+function);body=source.index('\n{',start);depth=0
 for end in range(body+1,len(source)):
  if source[end]=='{':depth+=1
  elif source[end]=='}':
   depth-=1
   if depth==0:break
 else:raise SystemExit('Missing function body')
 path.write_text(source[:body]+'\n{\n    '+replacement+'\n}'+source[end+1:])
