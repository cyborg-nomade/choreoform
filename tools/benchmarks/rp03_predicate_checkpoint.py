# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Exact signed source18 epoch; no version-dependent current-fixture fallback."""
from hashlib import sha256
from pathlib import Path
from .rp03_factoring_checkpoint import PATHS

ROOT=Path(__file__).resolve().parents[2]
PACKAGE=ROOT/'docs/benchmarks/rp03'
BASE=PACKAGE/'history/source-0.18-before-predicates'
INVENTORY='09cbdebdb11ffb50e8644985927b9864a34cdad3ba1d5c7ecf6b38eadae76283'
NOTE='58dcbc1e58476d32ef29279442cd01cbe973cf2d634997b2259b161ca5997cd9'
HISTORIES=('source-0.14','source-0.16-pre-instances','source-0.16-instances','source-0.16-before-factoring')


def artifacts():
    actual={p.relative_to(BASE).as_posix() for p in BASE.rglob('*') if p.is_file()}
    if actual!=set(PATHS)|{'ARCHIVE.md'}:raise ValueError('pre-predicate archive inventory changed')
    files={name:(BASE/name).read_bytes() for name in PATHS}
    rows=''.join(name+'\0'+sha256(raw).hexdigest()+'\n' for name,raw in files.items())
    if sha256(rows.encode()).hexdigest()!=INVENTORY:raise ValueError('pre-predicate archive bytes changed')
    note=(BASE/'ARCHIVE.md').read_bytes()
    if sha256(note).hexdigest()!=NOTE:raise ValueError('pre-predicate archive provenance changed')
    return files|{'ARCHIVE.md':note}


def prior_rp03_tree():
    files={name:raw for name,raw in artifacts().items() if name!='ARCHIVE.md'}
    for name in HISTORIES:
        for path in sorted((PACKAGE/'history'/name).rglob('*')):
            if path.is_file():files[path.relative_to(PACKAGE).as_posix()]=path.read_bytes()
    rows=''.join(name+'\0'+sha256(raw).hexdigest()+'\n' for name,raw in sorted(files.items()))
    if len(files)!=605 or sha256(rows.encode()).hexdigest()!='c68b965e389f3dc36e731b4a70448d23e4f4d580d9b9141199cdc1617d3a536c':
        raise ValueError('complete prior RP03 tree changed')
    return files


def prior_benchmark_inventory():
    files={'docs/benchmarks/rp03/'+name:sha256(raw).hexdigest() for name,raw in prior_rp03_tree().items()}
    other=ROOT/'docs/benchmarks/rp01'
    for path in sorted(other.rglob('*')):
        if path.is_file():files[path.relative_to(ROOT).as_posix()]=sha256(path.read_bytes()).hexdigest()
    rows=''.join(name+'\0'+pin+'\n' for name,pin in sorted(files.items()))
    if len(files)!=778 or sha256(rows.encode()).hexdigest()!='716f37ed07bdaeb0b05236dadb06e5e1eecf96025af58c3d7d843c9e28f19a42':
        raise ValueError('complete prior combined benchmark tree changed')
    return files
