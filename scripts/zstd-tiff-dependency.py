#!/usr/bin/env python3
"""Additional pinned libtiff oracle with ZSTD enabled; leaves prior oracle intact."""
import hashlib,json,pathlib,subprocess,tarfile
root=pathlib.Path(__file__).resolve().parents[1];tools=root/'.tools';target=root/'target'
subprocess.run(['python3',str(root/'scripts/zstd-dependency.py')],check=True)
lock=json.loads((root/'toolchain-lock.json').read_text());version=lock['libtiff'];archive=tools/f'tiff-{version}.tar.gz'
if hashlib.file_digest(archive.open('rb'),'sha256').hexdigest()!=lock['libtiff_sha256']:raise SystemExit('libtiff checksum mismatch')
parent=tools/'zstd-tiff-source';source=parent/f'tiff-{version}';parent.mkdir(exist_ok=True)
if not source.exists():
    with tarfile.open(archive) as t:t.extractall(parent,filter='data')
if not (source/'libtiff/.libs/libtiff.a').exists():
    args=['./configure','--disable-shared','--disable-docs','--disable-lerc','--with-zstd-include-dir='+str(tools/('zstd-'+lock['zstd'])/'lib'),'--with-zstd-lib-dir='+str(target/'zstd-reference/lib'),'--with-jpeg-include-dir='+str(tools/'jpeg/include'),'--with-jpeg-lib-dir='+str(tools/'jpeg/lib'),'--with-webp-include-dir='+str(tools/'webp/include'),'--with-webp-lib-dir='+str(tools/'webp/lib')]
    for name,cmd in [('configure',args),('build',['make','-C','libtiff','-j4'])]:
        with (target/f'zstd-tiff-{name}.log').open('w') as log:subprocess.run(cmd,cwd=source,stdout=log,stderr=subprocess.STDOUT,check=True)
if '#define ZSTD_SUPPORT 1' not in (source/'libtiff/tif_config.h').read_text():raise SystemExit('ZSTD oracle was not enabled')
print(f'Additional libtiff {version} / Zstandard {lock["zstd"]} oracle built')
