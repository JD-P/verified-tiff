#!/usr/bin/env python3
"""Restore and build the exact pre-LERC-arithmetic reader in a separate tree.

Archived hashes cover every core/C source, including public headers and C test
consumers. Current development scripts/tests are not historical proof evidence.
The archived whole-crate proof remains in kernel-optimization/verification-summary.
"""
import hashlib, json, os, pathlib, shutil, subprocess

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUT = ROOT/'target/lerc-arithmetic'
DEST = OUT/'restored-baseline-source'

def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()

def main():
    assert not DEST.exists(), f'Refusing to overwrite {DEST}'
    old = json.loads((ROOT/'docs/evidence/kernel-optimization/results.json').read_text())['metadata']
    DEST.mkdir(parents=True)
    for d in ['src','compat','examples','scripts','tests','fuzz']:
        shutil.copytree(ROOT/d,DEST/d,ignore=shutil.ignore_patterns('__pycache__','*.pyc'))
    for f in ['Cargo.toml','Cargo.lock','rust-toolchain.toml','toolchain-lock.json']:
        shutil.copyfile(ROOT/f,DEST/f)
    subprocess.run(['patch','--batch','--fuzz=0','--reverse','-p1','-i',
                    str(ROOT/'benchmarks/changes-from-native-float-baseline.patch')],cwd=DEST,check=True)
    subprocess.run(['patch','--batch','--fuzz=0','--reverse','-p1','-i',
                    str(ROOT/'benchmarks/changes-from-lerc-arithmetic-baseline.patch')],cwd=DEST,check=True)
    for name,digest in old['source_sha256'].items():
        assert sha(DEST/name)==digest,name
    (DEST/'.tools').symlink_to(ROOT/'.tools',target_is_directory=True)
    before = OUT/'before'; before.mkdir(exist_ok=True)
    env = dict(os.environ,VTIFF_COMPAT_OUTPUT=str(before/'lib'))
    log = OUT/'build-before.log'
    with log.open('w') as f:
        subprocess.run(['bash','scripts/build-compat.sh'],cwd=DEST,env=env,
                       stdout=f,stderr=subprocess.STDOUT,check=True)
    receipt = dict(label='reader baseline before LERC word arithmetic',
        source_archive_sha256='aeb27b0e212831b98f497ce5b5c97ff3865a8ac4975eabe1c59eb9181ef00659',source_sha256=old['source_sha256'],
        library_sha256=sha(before/'lib/libtiff.so.6'),
        consumer_source_sha256=sha(ROOT/'benchmarks/reader.c'),
        build='VTIFF_COMPAT_OUTPUT=target/lerc-arithmetic/before/lib bash scripts/build-compat.sh (restored tree)',
        build_log_sha256=sha(log))
    assert receipt['consumer_source_sha256']==old['benchmark_sha256']['benchmarks/reader.c']
    (before/'build-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print('Restored and built baseline; all archived core/C source hashes matched.')

if __name__=='__main__': main()
