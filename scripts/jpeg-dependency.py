#!/usr/bin/env python3
"""Pinned, test-only libjpeg-turbo oracle dependency. Never linked to Rust."""
import hashlib,json,pathlib,shutil,subprocess,sys,tarfile,urllib.request
root=pathlib.Path(__file__).resolve().parents[1];tools=root/'.tools';target=root/'target';target.mkdir(exist_ok=True)
lock=json.loads((root/'toolchain-lock.json').read_text());version=lock['libjpeg_turbo'];archive=tools/f'libjpeg-turbo-{version}.tar.gz'
if not archive.exists():urllib.request.urlretrieve(f'https://github.com/libjpeg-turbo/libjpeg-turbo/archive/refs/tags/{version}.tar.gz',archive)
if hashlib.file_digest(archive.open('rb'),'sha256').hexdigest()!=lock['libjpeg_turbo_sha256']:raise SystemExit('libjpeg-turbo checksum mismatch')
source=tools/f'libjpeg-turbo-{version}'
if not source.exists():
    with tarfile.open(archive) as t:t.extractall(tools,filter='data')
prefix=tools/'jpeg';build=tools/'libjpeg-build'
cmake=shutil.which('cmake')
if not cmake:
    cmake=tools/'cmake/cmake/data/bin/cmake'
    if not cmake.exists():
        with (target/'cmake-install.log').open('w') as log:subprocess.run([sys.executable,'-m','pip','install','--target',str(tools/'cmake'),'cmake=='+lock['cmake']],stdout=log,stderr=subprocess.STDOUT,check=True)
if not (prefix/'lib/libjpeg.a').exists():
    for name,args in [('configure',['-S',str(source),'-B',str(build),'-DCMAKE_INSTALL_PREFIX='+str(prefix),'-DENABLE_SHARED=OFF','-DENABLE_STATIC=ON','-DWITH_SIMD=OFF']),('build',['--build',str(build),'-j4']),('install',['--install',str(build)])]:
        with (target/f'jpeg-{name}.log').open('w') as log:subprocess.run([str(cmake),*args],stdout=log,stderr=subprocess.STDOUT,check=True)
subprocess.run(['cc','-O2','-I',str(prefix/'include'),str(root/'tests/jpeg_oracle.c'),str(prefix/'lib/libjpeg.a'),'-o',str(target/'jpeg-oracle')],check=True)
print(f'JPEG test oracle: libjpeg-turbo {version}; static, scalar C, integer IDCT, nearest chroma upsampling')
