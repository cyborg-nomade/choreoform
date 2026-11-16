# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite untrusted history claim oracle; never actual fault/entry evidence."""
from copy import deepcopy
from hashlib import sha256
import json
import re
import unittest

from check_static_contracts import ROOT
from check_late_observation_boundaries import package
from check_metadata_authoring import invoke

BASE = ROOT / 'docs/benchmarks/rp01'
CORPUS = ROOT / 'crates/semantic-check/tests/conformance/provider-fault-history.json'
PROFILE = ROOT / 'docs/dialects/provider-fault-history.md'
TYPES = ROOT / 'docs/dialects/provider-fault-history.types.json'
PROFILE_ID = 'urn:choreoform:fault-history-profile:provider-observations:0.1.0'


def pin(identity, raw):
    return {'id': identity, 'revision': 'sha256:' + sha256(raw).hexdigest()}


def model(c, source):
    # Deliberately limited model over the published fixed representation and
    # prevalidated source metadata; no events, identity lookups or policy checks.
    shapes = [(c, {'type','sourceOccurrence','cause','inputRevisions'}),
              (c.get('sourceOccurrence'), {'definitionRevision','instance','scopeOccurrence','site','occurrence','attempt'}),
              (c.get('cause'), {'observation','record'})]
    for row, keys in shapes:
        if not isinstance(row, dict): return 'shape'
        if set(row) != keys: return 'record-value'
    inputs = c['inputRevisions']
    if not isinstance(inputs, list) or len(inputs) > 4096: return 'collection'
    for row in inputs:
        if not isinstance(row, dict): return 'shape'
        if set(row) != {'cell','revision'}: return 'record-value'
    origin, cause = c['sourceOccurrence'], c['cause']
    strings = [c['type'], *origin.values(), *cause.values(), *(v for row in inputs for v in row.values())]
    if any(not isinstance(s, str) for s in strings): return 'shape'
    attempt = origin['attempt']
    if not re.fullmatch(r'0|-?[1-9][0-9]*', attempt): return 'integer-encoding'
    if not -(2**63) <= int(attempt) < 2**63: return 'integer-range'
    emission = source['emissions'].get(c['type'])
    if emission is None or cause['observation'] not in emission['observations']: return 'fault-history-source'
    if not 0 < len(cause['record']) <= 4096: return 'name'
    if not re.fullmatch(r'sha256:[0-9a-f]{64}', origin['definitionRevision']): return 'artifact-pin'
    if any(not 0 < len(origin[k]) <= 4096 for k in ['instance','scopeOccurrence','site','occurrence']): return 'name'
    if not 1 <= int(attempt) <= 1000: return 'fault-history-attempt'
    seen = set()
    for row in inputs:
        if any(not 0 < len(v) <= 4096 for v in row.values()): return 'name'
        if row['cell'] in seen: return 'fault-history-inputs'
        seen.add(row['cell'])
    return 'passed'


def apply(base, case):
    c = deepcopy(base)
    for path, value in case['patch'].items():
        parts = path.split('/')[1:]; target = c
        for part in parts[:-1]: target = target[part]
        target[parts[-1]] = deepcopy(value)
    for path in case['remove']:
        parts = path.split('/')[1:]; target = c
        for part in parts[:-1]: target = target[part]
        del target[parts[-1]]
    return c


class FaultHistoryEvidence(unittest.TestCase):
    def test_exact_profile_fixed_nominal_and_snapshot_bytes(self):
        reader = (ROOT/'crates/semantic-check/src/resources/fault_history.rs').read_text()
        for path in [PROFILE, TYPES]:
            raw = path.read_bytes(); revision = 'sha256:' + sha256(raw).hexdigest()
            self.assertIn(revision, reader)
            self.assertEqual((ROOT/'docs/ir/contracts'/(revision.replace(':','-')+'.txt')).read_bytes(), raw)
        types = json.loads(TYPES.read_bytes()); d = types['definitions']['Provider fault history']
        text = {'kind':'text'}
        expected = {'kind':'record','fields':{
            'type': text,
            'sourceOccurrence': {'kind':'record','fields':dict({k:text for k in ['definitionRevision','instance','scopeOccurrence','site','occurrence']},attempt={'kind':'integer'})},
            'cause': {'kind':'record','fields':{k:text for k in ['observation','record']}},
            'inputRevisions': {'kind':'list','element':{'kind':'record','fields':{k:text for k in ['cell','revision']}}}}}
        self.assertEqual(d, {'representation':expected,'invariant':{'op':'literal','type':{'kind':'boolean'},'value':True}})
        self.assertEqual(set(types['definitions']), {'Provider fault history'})

    def test_independent_finite_claim_model_matches_all_rust_consumed_vectors(self):
        v = json.loads(CORPUS.read_bytes()); source = json.loads((BASE/'payment.fault-source.json').read_bytes())
        self.assertEqual((v['format'],v['version']), ('choreoform-provider-fault-history-claims','0.1.0'))
        self.assertEqual(len(v['cases']),20); self.assertEqual(len({c['id'] for c in v['cases']}),20)
        for case in v['cases']:
            self.assertEqual(model(apply(v['base'],case),source), case['expect'], case['id'])
        invented = next(c for c in v['cases'] if c['id']=='invented-site-is-still-only-a-claim')
        self.assertEqual(invented['expect'],'passed')
        self.assertEqual(source['emissions']['provider unknown']['evidence'],'unresolved')
        self.assertEqual(source['emissions']['confirmed no effect']['observations'],{'no-effect':True})

    def test_unused_exact_metadata_does_not_initialize_or_change_any_current_stage(self):
        s,l,r = package(); source = pin(json.loads((BASE/'payment.fault-source.json').read_bytes())['id'],(BASE/'payment.fault-source.json').read_bytes())
        descriptor = {'kind':'provider-fault-history','version':'0.1.0','id':'urn:test:unused-history','profile':pin(PROFILE_ID,PROFILE.read_bytes()),'source':source}
        raw = (json.dumps(descriptor)+'\n').encode()
        extra = [(descriptor['id'],'sha256:'+sha256(raw).hexdigest(),raw)]
        for identity,path in [(PROFILE_ID,PROFILE),(json.loads(TYPES.read_bytes())['id'],TYPES)]:
            p = pin(identity,path.read_bytes());extra.append((identity,p['revision'],path.read_bytes()))
        for stage in ['lower','check','check-control','check-fault-routes','check-exceptional-availability','check-reconciliation-availability']:
            before = invoke((s,l,r),stage);after = invoke((s,l,r+extra),stage)
            self.assertEqual(before.returncode,0,(stage,before.stderr));self.assertEqual((after.returncode,after.stdout),(before.returncode,before.stdout),(stage,after.stderr))

    def test_licensing_ci_and_explicit_remaining_protection_boundary(self):
        notice = 'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n'
        for path in [CORPUS,TYPES]:self.assertEqual((path.parent/(path.name+'.license')).read_text(),notice)
        self.assertIn('SPDX-License-Identifier: CC-BY-4.0',PROFILE.read_text())
        self.assertIn('tools/check_fault_history.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())
        self.assertIn('provider-fault-history',(ROOT/'docs/ir/contracts/README.md').read_text())
        for phrase in ['lower bound only','Zero inherited values never implies zero provenance','always true','no ordinary ports']:
            self.assertIn(phrase,PROFILE.read_text())


if __name__ == '__main__': unittest.main()
