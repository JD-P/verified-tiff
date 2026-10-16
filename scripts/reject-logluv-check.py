#!/usr/bin/env python3
"""Require logluv acceptance, pixel semantics and public dispatch to be meaningful."""
import pathlib,shutil
root=pathlib.Path(__file__).resolve().parents[1]
for name,module,function,replacement in [
 ('stream','logluv.rs','sg_validate','Err(Error::Invalid)'),
 ('image','logluv_tiff.rs',"inspect_logluv_tiff<'a>",'Err(Error::Unsupported)'),
 ('pixels','logluv_segment.rs','convert',''),
 ('chroma','logluv_segment.rs','expand_uv','0'),
 ('raster','reader.rs',"inspect_raster<'a>",'')]:
 dest=root/'target'/('reject-logluv-'+name);dest.mkdir(exist_ok=True)
 for path in (root/'src').glob('*.rs'):
  if path.name!='main.rs':shutil.copyfile(path,dest/path.name)
 path=dest/module;source=path.read_text()
 if name=='chroma':
  old='(((2*(v as u32)+1)*8192)/205) as u16';assert source.count(old)==1
  path.write_text(source.replace(old,'0'));continue
 if name=='raster':
  old='Ok(i)=>Ok(Raster {inner:Kind::Logluv(i)})';assert source.count(old)==1
  path.write_text(source.replace(old,'Ok(_)=>Err(Error::Unsupported)'));continue
 start=source.index('fn '+function);body=source.index('\n{',start);depth=0
 for end in range(body+1,len(source)):
  if source[end]=='{':depth+=1
  elif source[end]=='}':
   depth-=1
   if depth==0:break
 else:raise SystemExit('Missing function body')
 path.write_text(source[:body]+'\n{\n    '+replacement+'\n}'+source[end+1:])
