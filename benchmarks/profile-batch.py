#!/usr/bin/env python3
"""Collect the four principal workloads for baseline, current and libtiff builds."""
import argparse
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline-root', type=Path, required=True)
    parser.add_argument('--milliseconds', type=int, default=3000)
    parser.add_argument('--output', type=Path, default=ROOT / 'target/profile')
    args = parser.parse_args()
    assert args.milliseconds > 0
    cases = {c['name']: c for c in json.loads((ROOT / 'target/benchmark/inputs.json').read_text())}
    engines = {
        'before': args.baseline_root.resolve() / 'target/release/examples/benchmark',
        'after': ROOT / 'target/release/examples/benchmark',
        'libtiff': ROOT / 'target/benchmark/libtiff',
    }
    for name in ('rgb-jpeg', 'rgb-jpeg-textured', 'gray16-deflate', 'noise-deflate'):
        for phase, exe in engines.items():
            subprocess.run([sys.executable, ROOT / 'benchmarks/profile.py', '--output',
                            args.output.resolve() / f'{phase}-{name}.json', exe,
                            ROOT / cases[name]['file'], 'total', str(args.milliseconds)],
                           cwd=ROOT, check=True)


if __name__ == '__main__':
    main()
