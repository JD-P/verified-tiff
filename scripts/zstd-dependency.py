#!/usr/bin/env python3
"""Pinned Zstandard reference encoder/decoder, for tests only."""
import hashlib,json,pathlib,shutil,subprocess,sys,tarfile,urllib.request
root=pathlib.Path(__file__).resolve().parents[1];tools=root/'.tools';target=root/'target'
lock=json.loads((root/'toolchain-lock.json').read_text());version=lock['zstd'];archive=tools/f'zstd-{version}.tar.gz'
if not archive.exists():urllib.request.urlretrieve(f'https://codeload.github.com/facebook/zstd/tar.gz/refs/tags/v{version}',archive)
if hashlib.file_digest(archive.open('rb'),'sha256').hexdigest()!=lock['zstd_sha256']:raise SystemExit('Zstandard checksum mismatch')
source=tools/f'zstd-{version}'
if not source.exists():
    with tarfile.open(archive) as t:t.extractall(tools,filter='data')
cmake=shutil.which('cmake')
if not cmake:
    cmake=tools/'cmake/cmake/data/bin/cmake'
    if not cmake.exists():
        with (target/'cmake-install.log').open('w') as log:subprocess.run([sys.executable,'-m','pip','install','--target',str(tools/'cmake'),'cmake=='+lock['cmake']],stdout=log,stderr=subprocess.STDOUT,check=True)
cmake=str(cmake)
build=target/'zstd-reference'
if not (build/'lib/libzstd.so').exists():
    commands=[('configure',[cmake,'-S',str(source/'build/cmake'),'-B',str(build),'-DCMAKE_BUILD_TYPE=Release','-DZSTD_BUILD_PROGRAMS=OFF','-DZSTD_BUILD_TESTS=OFF','-DZSTD_BUILD_STATIC=OFF','-DZSTD_BUILD_SHARED=ON','-DZSTD_LEGACY_SUPPORT=OFF']),('build',[cmake,'--build',str(build),'-j4'])]
    for name,args in commands:
        with (target/f'zstd-reference-{name}.log').open('w') as log:subprocess.run(args,stdout=log,stderr=subprocess.STDOUT,check=True)
print(f'Zstandard test reference: {build}/lib/libzstd.so ({version}); no production linkage')
