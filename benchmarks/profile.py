#!/usr/bin/env python3
"""Flat SIGPROF samples of unchanged release binaries (Linux x86-64).

Usage: profile.py --output target/profile/name.json EXECUTABLE INPUT MODE MS
Timing under the sampler is diagnostic only, never a benchmark comparison.
Samples include startup, warmup and preflight as well as the timed batch.
"""
import argparse, bisect, collections, hashlib, json, os, pathlib, subprocess

ROOT = pathlib.Path(__file__).resolve().parents[1]

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def symbols(path):
    if not pathlib.Path(path).is_file():
        return [], []
    # Static symbols for our release builds, dynamic symbols as a fallback for
    # stripped runtime libraries. A sample outside a symbol's size is unknown.
    found = {}
    for options in ([], ['-D']):
        p = subprocess.run(['nm', '-n', '-S', '-C', '--defined-only', *options, path], capture_output=True, text=True)
        for line in p.stdout.splitlines():
            a = line.split(None, 3)
            if len(a) == 4 and a[2] in ('T', 't', 'W', 'w', 'i', 'I'):
                try:
                    start, size = int(a[0], 16), int(a[1], 16)
                except ValueError:
                    continue
                if size: found[start] = (start, size, a[3])
    entries = sorted(found.values())
    return [s[0] for s in entries], entries

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=pathlib.Path, required=True)
    parser.add_argument('command', nargs='+')
    args = parser.parse_args()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    shared = ROOT / 'target/profile/sample.so'
    shared.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(['cc', '-O2', '-Wall', '-Wextra', '-Werror', '-shared', '-fPIC', ROOT / 'benchmarks/sample.c', '-ldl', '-o', shared], check=True)
    raw = args.output.with_suffix('.pcs').resolve()
    cpu = min(os.sched_getaffinity(0))
    os.sched_setaffinity(0, {cpu})
    command = [str(pathlib.Path(args.command[0]).resolve()), *args.command[1:]]
    env = dict(os.environ, LD_PRELOAD=str(shared), TIFF_PROFILE_OUTPUT=str(raw))
    p = subprocess.run(command, env=env, capture_output=True, text=True, check=True, timeout=300)
    lines = raw.read_text().splitlines()
    modules, counts, pc_counts = [], collections.Counter(), collections.Counter()
    for line in lines[1:]:
        if line.startswith('module '):
            _, base, start, end, name = line.split(' ', 4)
            modules.append((int(base, 16), int(start, 16), int(end, 16), name, symbols(name)))
        else:
            pc_counts[int(line, 16)] += 1
    for pc, count in pc_counts.items():
        name = '[unknown mapping]'
        for base, start, end, module, (starts, entries) in modules:
            if start <= pc < end:
                offset = pc - base
                i = bisect.bisect_right(starts, offset) - 1
                name = pathlib.Path(module).name + '::[unresolved]'
                if i >= 0 and offset < entries[i][0] + entries[i][1]:
                    name = pathlib.Path(module).name + '::' + entries[i][2]
                break
        counts[name] += count
    total = sum(counts.values())
    assert total >= 100, 'too few CPU samples'
    result = dict(method='SIGPROF interrupted instruction; flat self CPU samples',
                  includes='process startup, preflight, warmup, timed batch; no call graph',
                  header=lines[0], samples=total, cpu=cpu, command=command,
                  binary_sha256=sha(pathlib.Path(command[0])), input_sha256=sha(pathlib.Path(command[1])),
                  sampler_sha256=sha(ROOT / 'benchmarks/sample.c'), resolver_sha256=sha(pathlib.Path(__file__)),
                  diagnostic_run=json.loads(p.stdout),
                  functions=[dict(function=n, samples=c, percent=100*c/total) for n,c in counts.most_common()])
    args.output.write_text(json.dumps(result, indent=2)+'\n')
    print(args.output, total, 'samples')
    for n,c in counts.most_common(15): print(f'{100*c/total:6.2f}% {n}')

if __name__ == '__main__': main()
