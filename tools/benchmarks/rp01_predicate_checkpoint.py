# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Exact signed source18 expense epoch and complete prior-tree proofs."""
from hashlib import sha256
from pathlib import Path
from .rp01_factoring_checkpoint import PATHS

ROOT=Path(__file__).resolve().parents[2]
PACKAGE=ROOT/'docs/benchmarks/rp01'
BASE=PACKAGE/'history/source-0.18-before-predicates'
INVENTORY='9539d35227a57c5b4d2ada0cc9b05d64c7a7d7bfb85aa3852f2d6dd4f48112b4'
NOTE='1a7040754d938b7dd2e70cc82038a9422a68b73e8972108b675435c5a3088395'


def artifacts():
    actual={p.relative_to(BASE).as_posix() for p in BASE.rglob('*') if p.is_file()}
    if actual!=set(PATHS)|{'ARCHIVE.md'}:raise ValueError('expense pre-predicate archive inventory changed')
    files={name:(BASE/name).read_bytes() for name in PATHS}
    rows=''.join(name+'\0'+sha256(raw).hexdigest()+'\n' for name,raw in files.items())
    if sha256(rows.encode()).hexdigest()!=INVENTORY:raise ValueError('expense pre-predicate archive bytes changed')
    note=(BASE/'ARCHIVE.md').read_bytes()
    if sha256(note).hexdigest()!=NOTE:raise ValueError('expense pre-predicate archive provenance changed')
    return files|{'ARCHIVE.md':note}


def prior_rp01_tree():
    files={name:raw for name,raw in artifacts().items() if name!='ARCHIVE.md'}
    for history in ['source-0.7','source-0.14-before-factoring']:
        for path in sorted((PACKAGE/'history'/history).rglob('*')):
            if path.is_file():files[path.relative_to(PACKAGE).as_posix()]=path.read_bytes()
    rows=''.join(name+'\0'+sha256(raw).hexdigest()+'\n' for name,raw in sorted(files.items()))
    if len(files)!=173 or sha256(rows.encode()).hexdigest()!='fbed694a081590cf8bf53b954939b70d5e145d3751d388d9048a9845c4522b17':
        raise ValueError('complete prior RP01 tree changed')
    return files


def unchanged_rp03_tree():
    base=ROOT/'docs/benchmarks/rp03'
    files={p.relative_to(base).as_posix():p.read_bytes() for p in sorted(base.rglob('*')) if p.is_file()}
    rows=''.join(name+'\0'+sha256(raw).hexdigest()+'\n' for name,raw in files.items())
    if len(files)!=732 or sha256(rows.encode()).hexdigest()!='72718f58e2d6e58865d7eef88d06ef9249ec8d070dd8005b33d3e1d998f17e7f':
        raise ValueError('complete post106 RP03 tree changed')
    return files


def prior_benchmark_inventory():
    files={'docs/benchmarks/rp01/'+name:sha256(raw).hexdigest() for name,raw in prior_rp01_tree().items()}
    files.update({'docs/benchmarks/rp03/'+name:sha256(raw).hexdigest() for name,raw in unchanged_rp03_tree().items()})
    rows=''.join(name+'\0'+pin+'\n' for name,pin in sorted(files.items()))
    if len(files)!=905 or sha256(rows.encode()).hexdigest()!='d9dab2e4d1a421f07ba0cc3d5a280527dba2cc8d09e9515fbf6bfafc18fa1d29':
        raise ValueError('complete prior post106 benchmark tree changed')
    return files
