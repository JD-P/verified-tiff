#!/usr/bin/env python3
"""Install isolated, checksum-pinned Linux x86-64 verification tools."""
import hashlib,json,os,pathlib,platform,subprocess,urllib.request,zipfile
ROOT=pathlib.Path(__file__).resolve().parents[1]
LOCK=json.loads((ROOT/'toolchain-lock.json').read_text())
TOOLS=ROOT/'.tools';TOOLS.mkdir(exist_ok=True)
if platform.system()!='Linux' or platform.machine()!='x86_64':
    raise SystemExit('This bootstrap pins Linux x86-64; other platforms need an explicitly checked toolchain.')

def fetch(url,path,digest):
    if not path.exists():
        print('Downloading',path.name,flush=True)
        tmp=path.with_suffix(path.suffix+'.partial')
        urllib.request.urlretrieve(url,tmp)
        tmp.replace(path)
    actual=hashlib.file_digest(path.open('rb'),'sha256').hexdigest()
    if actual!=digest:raise SystemExit(f'Checksum mismatch for {path.name}: {actual}')

v=LOCK['verus'];archive=TOOLS/'verus.zip'
fetch(f'https://github.com/verus-lang/verus/releases/download/release/{v}/verus-{v}-x86-linux.zip',archive,LOCK['verus_sha256'])
if not (TOOLS/'verus-x86-linux/verus').exists():
    with zipfile.ZipFile(archive) as z:
        for item in z.infolist():
            if not (TOOLS/item.filename).resolve().is_relative_to(TOOLS.resolve()):raise SystemExit('Unsafe archive path')
        z.extractall(TOOLS)
for name in ['verus','rust_verify','z3','cargo-verus']:
    (TOOLS/'verus-x86-linux'/name).chmod(0o755)
env=dict(os.environ,CARGO_HOME=str(TOOLS/'cargo'),RUSTUP_HOME=str(TOOLS/'rustup'))
env['PATH']=str(TOOLS/'cargo/bin')+os.pathsep+env['PATH']
if not (TOOLS/'cargo/bin/rustup').exists():
    init=TOOLS/'rustup-init'
    fetch(f"https://static.rust-lang.org/rustup/archive/{LOCK['rustup']}/x86_64-unknown-linux-gnu/rustup-init",init,LOCK['rustup_sha256'])
    init.chmod(0o755)
    subprocess.run([str(init),'-y','--no-modify-path','--profile','minimal','--default-toolchain',LOCK['rust']],env=env,check=True)
else:
    subprocess.run(['rustup','toolchain','install',LOCK['rust'],'--profile','minimal'],env=env,check=True)
subprocess.run([str(TOOLS/'verus-x86-linux/verus'),'--version'],env=env,check=True)
subprocess.run(['cargo','fetch','--locked'],cwd=ROOT,env=env,check=True)
print('Bootstrap complete. Run bash scripts/verify.sh and bash scripts/test.sh.')
