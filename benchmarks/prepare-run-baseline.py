#!/usr/bin/env python3
"""Restore and build the pre-pass reader and each intermediate commit in separate trees.

Each `--step NAME=REF` is exported with git archive, its core/C source hashes
and tree id are recorded, and its compatibility library is freshly built.
The default builds only `before`, the parent commit of this pass. Trees are
restored beside the checkout so that Cargo reads only their own `.cargo`.
"""
import argparse, hashlib, io, json, os, pathlib, subprocess, tarfile

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUT = ROOT/'target/run-optimization'
SOURCES = ROOT.parent/(ROOT.name+'-run-steps')
PATHS = ['.cargo','src','compat','examples','trusted-float','trusted-simd','scripts','tests','fuzz',
         'Cargo.toml','Cargo.lock','rust-toolchain.toml','toolchain-lock.json']

def sha(p): return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def git(*args): return subprocess.run(['git',*args],cwd=ROOT,capture_output=True,check=True).stdout

def build(name, ref):
    dest = SOURCES/name
    assert not dest.exists(), f'Refusing to overwrite {dest}'
    assert not any((d/'.cargo').exists() for d in dest.parents), 'An enclosing .cargo would change the build'
    tree = git('rev-parse',ref+'^{tree}').decode().strip()
    present = set(git('ls-tree','--name-only',tree).decode().split())
    archive = git('archive','--format=tar',tree,*(p for p in PATHS if p in present))
    dest.mkdir(parents=True)
    with tarfile.open(fileobj=io.BytesIO(archive)) as t: t.extractall(dest,filter='data')
    (dest/'.tools').symlink_to(ROOT/'.tools',target_is_directory=True)
    source = {str(p.relative_to(dest)):sha(p) for d in ['src','compat','trusted-float','trusted-simd']
              for p in sorted((dest/d).rglob('*')) if p.is_file() and p.suffix in ['.rs','.c','.h','.inc']}
    folder = OUT/name; folder.mkdir(exist_ok=True)
    env = dict(os.environ,VTIFF_COMPAT_OUTPUT=str(folder/'lib'))
    log = OUT/f'build-{name}.log'
    with log.open('w') as f:
        subprocess.run(['bash','scripts/build-compat.sh'],cwd=dest,env=env,stdout=f,stderr=subprocess.STDOUT,check=True)
    receipt = dict(label=name,ref=ref,tree=tree,archive_sha256=hashlib.sha256(archive).hexdigest(),source_sha256=source,
        library_sha256=sha(folder/'lib/libtiff.so.6'),consumer_source_sha256=sha(ROOT/'benchmarks/reader.c'),
        build=f'VTIFF_COMPAT_OUTPUT=target/run-optimization/{name}/lib bash scripts/build-compat.sh (restored tree)',
        build_log_sha256=sha(log))
    (folder/'build-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print('Restored and built',name,ref,tree,flush=True)

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--step',action='append',default=None,help='NAME=REF, in commit order')
    steps = parser.parse_args().step or ['before=9a64643']
    for item in steps:
        name, ref = item.split('=',1)
        build(name, ref)

if __name__=='__main__': main()
