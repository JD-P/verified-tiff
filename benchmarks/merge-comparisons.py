#!/usr/bin/env python3
"""Combine disjoint case runs, preserving each run's exact harness provenance.

Sources, binaries, libraries, CPU affinity and measurement settings must match.
Duplicate cases are rejected: this never selects favorable batches or changes
compare.py's strict resume guard. Different harness versions remain explicit.
"""
import argparse
import json
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('inputs', nargs='+', type=Path)
parser.add_argument('--output', required=True, type=Path)
args = parser.parse_args()
runs = [json.loads(p.read_text()) for p in args.inputs]
common = [
    'cpu_affinity', 'cpu_model', 'uname', 'rounds', 'target_batch_ms',
    'shuffle_seed', 'input_output_limit_bytes', 'work_budget', 'compiler',
    'rustc', 'build', 'before_verified', 'after_verified',
    'before_core_sha256', 'after_core_sha256', 'unchanged_compat_sha256',
    'binaries', 'pinned_libtiff', 'system_libtiff', 'dependency_sha256',
    'pinned_static_library_sha256',
]
meta = runs[0]['metadata'].copy()
meta['measurement_runs'] = []
cases, names = [], set()
for i, run in enumerate(runs):
    assert 'measurement_runs' not in run['metadata'], 'Merge original run files only'
    for key in common:
        assert run['metadata'][key] == meta[key], key
    for case in run['cases']:
        assert case['name'] not in names, f"Duplicate case: {case['name']}"
        names.add(case['name'])
        assert all(len(v) == meta['rounds'] for v in case['samples'].values())
        cases.append(dict(case, measurement_run=i))
    meta['measurement_runs'].append(dict(
        case_names=[c['name'] for c in run['cases']], metadata=run['metadata']))
args.output.write_text(json.dumps(dict(metadata=meta, cases=cases), indent=2) + '\n')
print(f'Combined {len(cases)} disjoint cases from {len(runs)} runs; all run metadata retained.')
