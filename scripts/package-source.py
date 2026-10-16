#!/usr/bin/env python3
"""Package current source and matching metadata security evidence; exclude tools."""
import argparse,datetime,hashlib,json,os,pathlib,tarfile
ROOT=pathlib.Path(__file__).resolve().parents[1]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 p=argparse.ArgumentParser(description=__doc__)
 p.add_argument('--output',type=pathlib.Path,default=ROOT.parent/'output/verified-tiff-0.1.0.tar')
 args=p.parse_args();output=args.output.resolve();assert not output.is_relative_to(ROOT)
 evidence=ROOT/'docs/evidence/metadata-contracts'
 summary=json.loads((evidence/'summary.json').read_text())
 assert summary['core_proof']['errors']==0 and summary['core_proof']['entire_crate'] and not summary['core_proof']['reused_unchanged_sources']
 assert summary['coverage']['lines']==[1351,1351] and summary['coverage']['functions']==[242,242] and summary['coverage']['branches']==[1455,1455]
 assert summary['coverage']['exclusions']==[] and summary['optimized_python_tests']==70 and summary['sanitized_python_tests']==70
 assert summary['negative_proof_gates']==128 and summary['rust_tests']==71 and summary['core_python_tests']==97
 for n,h in summary['evidence_sha256'].items():assert sha(evidence/n)==h,n
 for n,h in summary['source_sha256'].items():assert sha(ROOT/n)==h,n
 files=[]
 for base,dirs,names in os.walk(ROOT):
  dirs[:]=sorted(d for d in dirs if d not in ('.tools','.git','target','__pycache__'))
  for name in sorted(names):
   path=pathlib.Path(base)/name
   if path.is_symlink() or path.suffix=='.pyc' or name=='SOURCE-MANIFEST.json':continue
   files.append(path)
 files.sort(key=lambda p:str(p.relative_to(ROOT)))
 manifest={'algorithm':'sha256','files':{str(p.relative_to(ROOT)):sha(p) for p in files}}
 (ROOT/'SOURCE-MANIFEST.json').write_text(json.dumps(manifest,indent=2)+'\n')
 files.append(ROOT/'SOURCE-MANIFEST.json');files.sort(key=lambda p:str(p.relative_to(ROOT)))
 output.parent.mkdir(exist_ok=True,parents=True);temp=output.with_suffix('.tar.tmp')
 epoch=int(datetime.datetime(2026,10,15,tzinfo=datetime.timezone.utc).timestamp())
 with tarfile.open(temp,'w',format=tarfile.PAX_FORMAT) as tar:
  for path in files:
   info=tar.gettarinfo(str(path),arcname='verified-tiff/'+str(path.relative_to(ROOT)))
   info.uid=info.gid=0;info.uname=info.gname='';info.mtime=epoch
   info.mode=0o755 if path.stat().st_mode&0o111 else 0o644
   with path.open('rb') as f:tar.addfile(info,f)
 os.replace(temp,output)
 with tarfile.open(output) as tar:
  assert len(tar.getnames())==len(files)
  for name,h in manifest['files'].items():assert hashlib.sha256(tar.extractfile('verified-tiff/'+name).read()).hexdigest()==h,name
 print(json.dumps(dict(archive=str(output),files=len(files),bytes=output.stat().st_size,sha256=sha(output)),indent=2))
if __name__=='__main__':main()
