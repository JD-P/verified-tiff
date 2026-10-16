#!/usr/bin/env python3
"""Build and test a pinned libtiff oracle; never linked into the Rust reader."""
import hashlib,json,pathlib,subprocess,tarfile,urllib.request,sys
root=pathlib.Path(__file__).resolve().parents[1];tools=root/'.tools';tools.mkdir(exist_ok=True)
lock=json.loads((root/'toolchain-lock.json').read_text());version=lock['libtiff']
archive=tools/f'tiff-{version}.tar.gz'
if not archive.exists():urllib.request.urlretrieve(f'https://download.osgeo.org/libtiff/tiff-{version}.tar.gz',archive)
if hashlib.file_digest(archive.open('rb'),'sha256').hexdigest()!=lock['libtiff_sha256']:raise SystemExit('libtiff checksum mismatch')
source=tools/f'tiff-{version}'
if not source.exists():
    with tarfile.open(archive) as t:t.extractall(tools,filter='data')
subprocess.run([sys.executable,str(root/'scripts/jpeg-dependency.py')],check=True)
subprocess.run([sys.executable,str(root/'scripts/webp-dependency.py')],check=True)
for name,args in [('configure',['./configure','--disable-shared','--disable-docs','--with-jpeg-include-dir='+str(tools/'jpeg/include'),'--with-jpeg-lib-dir='+str(tools/'jpeg/lib'),'--with-webp-include-dir='+str(tools/'webp/include'),'--with-webp-lib-dir='+str(tools/'webp/lib')]),
                  ('build',['make','-j4']),('tests',['make','check','-j4'])]:
    with (tools/f'libtiff-{name}.log').open('w') as log:
        subprocess.run(args,cwd=source,stdout=log,stderr=subprocess.STDOUT,check=True)
log=(tools/'libtiff-tests.log').read_text()
summary='\n'.join(line for line in log.splitlines() if line.startswith('# '))
print(summary)
if '# TOTAL: 0' in summary or '# TOTAL:' not in summary:raise SystemExit('No upstream tests executed')
# The generated libtool file records exactly the dependencies found by
# configure. It links this build's static archive and optional codec libraries.
(root/'target').mkdir(exist_ok=True)
for name,output in [('oracle.c','libtiff-oracle'),('alpha_oracle.c','alpha-oracle'),('native_oracle.c','native-oracle'),('jpeg_tiff_oracle.c','jpeg-tiff-oracle'),('logluv_encoder.c','logluv-encoder')]:
    subprocess.run([str(source/'libtool'),'--mode=link','--tag=CC',
                    'cc','-O2','-I',str(source/'libtiff'),str(root/'tests'/name),
                    str(source/'libtiff/libtiff.la'),
                    '-o',str(root/'target'/output)],check=True)
    (root/'target'/output).chmod(0o755)
(root/'target/upstream-baseline.txt').write_text(f'libtiff {version}\n'+summary+'\n')
print('Oracle built; run bash scripts/test.sh to compare the Rust reader.')
