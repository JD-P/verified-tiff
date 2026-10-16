#!/usr/bin/env python3
"""Restore a measured v17/v18/v19/v20 core in a separate build tree; never edit this tree.

Uses the shipped reverse patch and checks all 51 original core source hashes.
The v20 reverse patch also restores the old C adapter and removes the new native modules.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--destination', type=Path, default=ROOT / 'target/optimization-baseline')
    parser.add_argument('--version', type=int, choices=[17, 18, 19, 20], default=17)
    args = parser.parse_args()
    dest = args.destination.resolve()
    if dest.exists():
        parser.error(f'destination already exists: {dest}')
    dirs = ['src', 'compat', 'examples', 'benchmarks', 'scripts', 'tests', 'fuzz']
    if dest == ROOT or any((ROOT / d) == dest or (ROOT / d) in dest.parents for d in dirs + ['docs']):
        parser.error('destination must be outside the copied source directories')
    current = json.loads((ROOT / 'docs/evidence/verification-summary.json').read_text())
    for path, sha in current['core_sources_sha256'].items():
        assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == sha, path
    original = ROOT / {17:'docs/evidence/optimization-baseline-verification.json',18:'docs/evidence/profile-baseline-verification.json',19:'docs/evidence/throughput-baseline-verification.json',20:'docs/evidence/api-five-baseline-verification.json'}[args.version]
    old = json.loads(original.read_text())
    dest.mkdir(parents=True)
    for d in dirs:
        shutil.copytree(ROOT / d, dest / d, ignore=shutil.ignore_patterns('__pycache__', '*.pyc'))
    for f in ['Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', 'toolchain-lock.json', 'AGENTS.md', 'LICENSE', 'README.md']:
        shutil.copyfile(ROOT / f, dest / f)
    for version in range(20, args.version-1, -1):
        subprocess.run(['patch', '--batch', '--fuzz=0', '--reverse', '-p1', '-i',
                        str(ROOT / f'benchmarks/changes-from-v{version}.patch')], cwd=dest, check=True)
    for path, sha in old['core_sources_sha256'].items():
        assert hashlib.sha256((dest / path).read_bytes()).hexdigest() == sha, path
    for path, sha in json.loads((ROOT / 'docs/evidence/api-five-baseline-c-api-source-sha256.json').read_text()).items():
        assert hashlib.sha256((dest / path).read_bytes()).hexdigest() == sha, path
    (dest / 'docs/evidence').mkdir(parents=True)
    shutil.copyfile(original, dest / 'docs/evidence/verification-summary.json')
    shutil.copyfile(ROOT / 'docs/evidence/api-five-baseline-c-api-source-sha256.json', dest / 'docs/evidence/c-api-source-sha256.json')
    (dest / 'BASELINE-README.txt').write_text(
        f'Benchmark-only build tree with exact v{args.version} core sources and archived C adapter.\n'
        f'The proof summary is archived v{args.version} evidence, not a new verification run.\n'
        'Development scripts and documentation are copied from the current project;\n'
        'do not run its new optimized-path negative checks against the old core.\n')
    if (ROOT / '.tools').exists():
        (dest / '.tools').symlink_to(os.path.relpath(ROOT / '.tools', dest), target_is_directory=True)
    print(f'Restored all 51 original core files and archived adapter in {dest}')
    print('This is a benchmark build tree, not a separate deliverable or a fresh verification run.')


if __name__ == '__main__':
    main()
