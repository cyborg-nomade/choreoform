# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Exact signed pre-factoring package, immutable inputs rather than regeneration."""
from hashlib import sha256
from pathlib import Path

BASE=Path(__file__).resolve().parents[2]/'docs/benchmarks/rp01/history/source-0.14-before-factoring'
INVENTORY='198e5c971a01299c441c0740b0dbab140467642e91725691a89026a3b184b0ac'
NOTE='feb5d09bafd3d4b6462bb354b3f8f9f51bb00ec96c22e34aa8c4d3a6101b834d'
PATHS=('README.md',
 'bindings.json',
 'bindings.json.license',
 'candidate.ir.json',
 'candidate.ir.json.license',
 'checks.provider.json',
 'checks.provider.json.license',
 'claim-v1.json',
 'claim-v1.json.license',
 'claim-v2.json',
 'claim-v2.json.license',
 'clock.json',
 'clock.json.license',
 'clock.provider.json',
 'clock.provider.json.license',
 'correction-value.json',
 'correction-value.json.license',
 'host-contract.md',
 'launch-values.json',
 'launch-values.json.license',
 'manifest.json',
 'manifest.json.license',
 'notice.provider.json',
 'notice.provider.json.license',
 'payment.fault-source.json',
 'payment.fault-source.json.license',
 'payment.provider.json',
 'payment.provider.json.license',
 'payment.terminal-source.json',
 'payment.terminal-source.json.license',
 'policy.json',
 'policy.json.license',
 'receipt.txt',
 'receipt.txt.license',
 'reports/check-concurrency.json',
 'reports/check-concurrency.json.license',
 'reports/check-control.json',
 'reports/check-control.json.license',
 'reports/check-exceptional-availability.json',
 'reports/check-exceptional-availability.json.license',
 'reports/check-fault-routes.json',
 'reports/check-fault-routes.json.license',
 'reports/check-invalidation.json',
 'reports/check-invalidation.json.license',
 'reports/check-protection.json',
 'reports/check-protection.json.license',
 'reports/check-reconciliation-availability.json',
 'reports/check-reconciliation-availability.json.license',
 'reports/check-reconciliation-links.json',
 'reports/check-reconciliation-links.json.license',
 'reports/check-timing.json',
 'reports/check-timing.json.license',
 'reports/check.json',
 'reports/check.json.license',
 'reports/invalidation-requirements.json',
 'reports/invalidation-requirements.json.license',
 'source.choreo',
 'types.json',
 'types.json.license')


def artifacts():
    expected=set(PATHS)|{'ARCHIVE.md'}
    actual={p.relative_to(BASE).as_posix() for p in BASE.rglob('*') if p.is_file()}
    if actual!=expected:raise ValueError('pre-factoring archive inventory changed')
    files={name:(BASE/name).read_bytes() for name in PATHS}
    rows=''.join(name+'\0'+sha256(raw).hexdigest()+'\n' for name,raw in files.items())
    if sha256(rows.encode()).hexdigest()!=INVENTORY:raise ValueError('pre-factoring archive bytes changed')
    note=(BASE/'ARCHIVE.md').read_bytes()
    if sha256(note).hexdigest()!=NOTE:raise ValueError('pre-factoring archive provenance changed')
    files['ARCHIVE.md']=note
    return files


def original_tree():
    """Exact original 113-file tree, without duplicating its older history."""
    files=artifacts();del files['ARCHIVE.md']
    older=BASE.parent/'source-0.7'
    files.update({'history/source-0.7/'+p.relative_to(older).as_posix():p.read_bytes() for p in older.rglob('*') if p.is_file()})
    if len(files)!=113:raise ValueError('original RP-01 tree inventory changed')
    return dict(sorted(files.items()))
