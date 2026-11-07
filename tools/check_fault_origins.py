# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite lexical-context model; no actual faults, history or entry evidence."""
import json
import unittest

from check_static_contracts import ROOT
from check_late_observation_boundaries import package
from check_metadata_authoring import invoke

CORPUS = ROOT/'crates/semantic-check/tests/conformance/fault-origin-contexts.json'


def select(scopes, origin, fault, active):
    # Independent declarative model: at each candidate owner retain precisely
    # the active handler boundaries containing that owner; escaped selections
    # are excluded. It does not model activation or authenticate an emitter.
    selections = {owner+':'+name: target for owner,s in scopes.items() for name,target in s['handlers'].items()}
    def ancestors(scope):
        chain=[]
        while scope is not None:
            chain.append(scope);scope=scopes[scope]['parent']
        return chain
    for owner in ancestors(origin):
        kept=[s for s in active if selections[s] in ancestors(owner)]
        escaped=set(active)-set(kept)
        candidate=owner+':'+fault
        if fault in scopes[owner]['handlers'] and candidate not in escaped:
            return candidate,kept+[candidate]
    return None,[]


class FaultOriginEvidence(unittest.TestCase):
    def test_independent_full_boundary_model_matches_all_rust_selection_vectors(self):
        v=json.loads(CORPUS.read_bytes())
        self.assertEqual((v['format'],v['version']),('choreoform-fault-origin-context-vectors','0.1.0'))
        self.assertEqual(len(v['cases']),9);self.assertEqual(len({c['id'] for c in v['cases']}),9)
        for c in v['cases']:
            self.assertEqual(select(v['scopes'],c['origin'],c['fault'],c['active']),(c['selected'],c['after']),c['id'])
        deep=next(c for c in v['cases'] if c['id']=='retain-full-three-deep-context')
        self.assertEqual(len(deep['after']),3)
        escaped=next(c for c in v['cases'] if c['id']=='exclude-three-then-root')
        self.assertEqual(escaped['after'],['root:f'])

    def test_legacy_native_routes_and_all_committed_package_bytes_remain_unchanged(self):
        p=package();result=invoke(p,'check-fault-routes');self.assertEqual(result.returncode,0,result.stderr)
        expected=json.loads((ROOT/'docs/benchmarks/rp01/reports/check-fault-routes.json').read_bytes())
        self.assertEqual(json.loads(result.stdout),expected)
        result=invoke(p);self.assertEqual(result.returncode,0,result.stderr)
        self.assertEqual(json.loads(result.stdout),json.loads((ROOT/'docs/benchmarks/rp01/candidate.ir.json').read_bytes()))

    def test_licensing_and_required_ci_keep_this_finite_model_separate(self):
        self.assertEqual((CORPUS.parent/(CORPUS.name+'.license')).read_text(),'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n')
        self.assertIn('tools/check_fault_origins.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())
        self.assertIn('no fault events',json.loads(CORPUS.read_bytes())['premise'])


if __name__=='__main__': unittest.main()
