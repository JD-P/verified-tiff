#!/usr/bin/env python3
"""Prepare a fresh pinned libtiff 4.7.1 benchmark reference (test-only).
Run bootstrap/upstream first for the pinned source archives and codec oracles.
The production decoder is never edited by this script.
"""
import hashlib,json,pathlib,subprocess,tarfile
ROOT=pathlib.Path(__file__).resolve().parents[1];OUT=ROOT/'target/reader-benchmark';OUT.mkdir(exist_ok=True)
lock=json.loads((ROOT/'toolchain-lock.json').read_text());tools=ROOT/'.tools'
for script in ['jpeg-dependency.py','webp-dependency.py','zstd-dependency.py','lerc-dependency.py']:
 subprocess.run(['python3',str(ROOT/'scripts'/script)],check=True)
archive=tools/('tiff-'+lock['libtiff']+'.tar.gz');assert hashlib.sha256(archive.read_bytes()).hexdigest()==lock['libtiff_sha256']
source=OUT/('tiff-'+lock['libtiff'])
if not source.exists():
 with tarfile.open(archive) as t:t.extractall(OUT,filter='data')
args=['./configure','--disable-shared','--disable-docs','--disable-tools','--disable-tests','--disable-libdeflate',
 '--with-lerc-include-dir='+str(tools/('lerc-'+lock['lerc'])/'src/LercLib/include'),'--with-lerc-lib-dir='+str(ROOT/'target/lerc-reference'),
 '--with-zstd-include-dir='+str(tools/('zstd-'+lock['zstd'])/'lib'),'--with-zstd-lib-dir='+str(ROOT/'target/zstd-reference/lib'),
 '--with-jpeg-include-dir='+str(tools/'jpeg/include'),'--with-jpeg-lib-dir='+str(tools/'jpeg/lib'),
 '--with-webp-include-dir='+str(tools/'webp/include'),'--with-webp-lib-dir='+str(tools/'webp/lib')]
with (OUT/'configure.log').open('w') as f:subprocess.run(args,cwd=source,stdout=f,stderr=subprocess.STDOUT,check=True)
with (OUT/'build-reference.log').open('w') as f:
 subprocess.run(['make','-C','libtiff','clean'],cwd=source,stdout=f,stderr=subprocess.STDOUT,check=True)
 subprocess.run(['make','-C','libtiff','-j4'],cwd=source,stdout=f,stderr=subprocess.STDOUT,check=True)
config=(source/'libtiff/tif_config.h').read_text()+(source/'libtiff/tiffconf.h').read_text()
for feature in ['LERC','ZSTD','WEBP','JPEG','ZIP']:
 assert '#define '+feature+'_SUPPORT 1' in config,feature
print('Fresh pinned libtiff reference built:',source)
