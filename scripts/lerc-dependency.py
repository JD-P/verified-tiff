#!/usr/bin/env python3
"""Checksum-pinned test-only Esri LERC encoder and decoder."""
import hashlib,json,pathlib,shutil,subprocess,sys,tarfile,urllib.request
root=pathlib.Path(__file__).resolve().parents[1];tools=root/'.tools';target=root/'target'
target.mkdir(exist_ok=True)
lock=json.loads((root/'toolchain-lock.json').read_text());version=lock['lerc'];archive=tools/f'Lerc-{version}.tar.gz'
if not archive.exists():urllib.request.urlretrieve(f'https://codeload.github.com/Esri/lerc/tar.gz/refs/tags/v{version}',archive)
if hashlib.file_digest(archive.open('rb'),'sha256').hexdigest()!=lock['lerc_sha256']:raise SystemExit('LERC checksum mismatch')
source=tools/f'lerc-{version}'
if not source.exists():
    with tarfile.open(archive) as t:t.extractall(tools,filter='data')
cmake=shutil.which('cmake')
if not cmake:
    cmake=tools/'cmake/cmake/data/bin/cmake'
    if not cmake.exists():
        with (target/'cmake-install.log').open('w') as log:subprocess.run([sys.executable,'-m','pip','install','--target',str(tools/'cmake'),'cmake=='+lock['cmake']],stdout=log,stderr=subprocess.STDOUT,check=True)
build=target/'lerc-reference'
for name,args in [('configure',['-S',str(source),'-B',str(build),'-DCMAKE_BUILD_TYPE=Release','-DBUILD_SHARED_LIBS=ON']),('build',['--build',str(build),'-j4'])]:
    with (target/f'lerc-reference-{name}.log').open('w') as log:subprocess.run([str(cmake),*args],stdout=log,stderr=subprocess.STDOUT,check=True)
print(f'LERC {version} test reference: {build}/libLerc.so; no production linkage')
