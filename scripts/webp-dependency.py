#!/usr/bin/env python3
"""Checksum-pinned libwebp for independent test oracles; never linked to Rust."""
import hashlib,json,pathlib,subprocess,tarfile,urllib.request
root=pathlib.Path(__file__).resolve().parents[1];tools=root/'.tools';target=root/'target'
target.mkdir(exist_ok=True)
lock=json.loads((root/'toolchain-lock.json').read_text());version=lock['libwebp'];archive=tools/f'libwebp-{version}.tar.gz'
if not archive.exists():urllib.request.urlretrieve(f'https://storage.googleapis.com/downloads.webmproject.org/releases/webp/libwebp-{version}.tar.gz',archive)
if hashlib.file_digest(archive.open('rb'),'sha256').hexdigest()!=lock['libwebp_sha256']:raise SystemExit('libwebp checksum mismatch')
source=tools/f'libwebp-{version}'
if not source.exists():
    with tarfile.open(archive) as t:t.extractall(tools,filter='data')
prefix=tools/'webp'
if not (prefix/'lib/libwebp.a').exists():
    for name,args in [('configure',['./configure','--prefix='+str(prefix),'--disable-shared','--enable-static','--disable-sdl','--disable-png','--disable-jpeg','--disable-tiff','--disable-gif','--disable-libwebpmux','--disable-libwebpdemux','--disable-threading']),('build',['make','-j4']),('install',['make','install'])]:
        with (target/f'webp-{name}.log').open('w') as log:subprocess.run(args,cwd=source,stdout=log,stderr=subprocess.STDOUT,check=True)
subprocess.run(['cc','-O2','-I',str(prefix/'include'),str(root/'tests/webp_oracle.c'),str(prefix/'lib/libwebp.a'),'-lm','-o',str(target/'webp-oracle')],check=True)
(target/'webp-oracle').chmod((target/'webp-oracle').stat().st_mode | 0o111)
print(f'WebP test oracle: libwebp {version}; independent RGBA decode')

for name,filename,internal,encoder in [('webp-encode','webp_encode.c',False,True),('vp8-primitives-oracle','vp8_primitives.c',True,False),('vp8-yuv-oracle','vp8_yuv_oracle.c',False,False)]:
    args=['cc','-O2','-I',str(prefix/'include')]
    if internal:args+=['-I',str(source)]
    args+=[str(root/'tests'/filename),str(prefix/'lib/libwebp.a')]
    if encoder:args+=[str(prefix/'lib/libsharpyuv.a')]
    subprocess.run(args+['-lm','-o',str(target/name)],check=True)
    (target/name).chmod((target/name).stat().st_mode | 0o111)
