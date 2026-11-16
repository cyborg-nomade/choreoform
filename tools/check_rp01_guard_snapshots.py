# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Independent catalogue accountability, not predicate evaluation or execution."""
from collections import Counter
from hashlib import sha256
import json
from pathlib import Path
import unittest

import check_ir_fixtures as wire

ROOT=Path(__file__).resolve().parents[1]
PATH=ROOT/'crates/semantic-check/tests/conformance/rp01-guard-snapshots.json'
BASE=ROOT/'docs/benchmarks/rp01'


def leaves(ast):
    """Collect only expression read/fact leaves; opaque literals are not ASTs."""
    op=ast['op']
    if op in ['literal','none']: return set(),set()
    if op=='read': return {ast['cell']},set()
    if op=='fact': return set(),{ast['name']}
    if op=='record': children=list(ast['fields'].values())
    elif op=='list': children=ast['items']
    else: children=[v for v in ast.values() if isinstance(v,dict) and 'op' in v]
    cells,facts=set(),set()
    for child in children:
        c,f=leaves(child);cells|=c;facts|=f
    return cells,facts


class Rp01GuardCatalogue(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.doc=wire.load(PATH.read_bytes())
        cls.candidate=wire.load((BASE/'candidate.ir.json').read_bytes())

    def test_closed_transport_revision_identity_and_exact_finite_counts(self):
        d=self.doc
        self.assertEqual(set(d),{'format','version','candidateRevision','candidateDigest','values','cases'})
        self.assertEqual((d['format'],d['version']),('choreoform-rp01-guard-snapshots','0.1.0'))
        self.assertEqual(d['candidateRevision'],self.candidate['revision'])
        self.assertEqual(d['candidateDigest'],'sha256:'+sha256((BASE/'candidate.ir.json').read_bytes()).hexdigest())
        self.assertEqual(len(d['values']),46)
        self.assertEqual(len(d['cases']),44)
        self.assertEqual(len({c['id'] for c in d['cases']}),44)
        self.assertEqual(Counter(c['scenario'] for c in d['cases']),{'RP-01-A':12,'RP-01-B':16,'RP-01-C':6,'RP-01-D':10})
        self.assertEqual(Counter(c['expect']['phase'] for c in d['cases']),{'value':41,'binding':2,'evaluation':1})
        self.assertEqual(Counter(c['expect']['value'] for c in d['cases'] if c['expect']['phase']=='value'),{True:12,False:29})

    def test_subjects_values_and_dependency_bindings_reference_actual_ast_records(self):
        b=self.candidate['body'];used=set()
        for c in self.doc['cases']:
            self.assertEqual(set(c),{'id','scenario','subject','cells','facts','expect'})
            s=c['subject'];self.assertEqual(set(s),{'map','id','field','site'})
            self.assertIn(s['map'],['expressions','policies'])
            record=b[s['map']][s['id']]
            if s['map']=='expressions':
                self.assertEqual(s['field'],'body');self.assertIsNone(s['site']);ast=record['body']
            else:
                kind=record['body']['kind']
                self.assertEqual(s['field'],{'actor':'permit','work':'completion','race':'invariant'}[kind])
                ast=record['body'][s['field']]
                if kind=='work': self.assertEqual(b['nodes'][s['site']]['policy'],s['id'])
                else: self.assertIsNone(s['site'])
            required_cells,required_facts=leaves(ast)
            if c['expect'].get('code')!='snapshot-binding':
                self.assertEqual(set(c['cells']),required_cells,c['id'])
                self.assertEqual(set(c['facts']),required_facts,c['id'])
            for values in [c['cells'],c['facts']]:
                self.assertTrue(set(values.values())<=self.doc['values'].keys())
                used|=set(values.values())
        self.assertEqual(used,self.doc['values'].keys())

    def test_expected_faults_and_policy_revision_negatives_cannot_be_relabeled_as_success(self):
        cases={c['id']:c for c in self.doc['cases']}
        for id in ['commit_wrong_policy','commit_wrong_finance_policy','commit_changed_current_policy','old_manager_cannot_approve_v2','old_finance_cannot_approve_v2','wrong_approval_policy','payment_wrong_amount','payment_wrong_currency','manager_self','finance_same_manager','withdraw_wrong_relationship']:
            self.assertEqual(cases[id]['expect'],{'phase':'value','value':False})
        for id in ['fresh_v2_approvals','commit_fresh_v2','manager_approved','finance_approved']:
            self.assertEqual(cases[id]['expect'],{'phase':'value','value':True})
        self.assertEqual(cases['finance_no_manager']['expect'],{'phase':'evaluation','code':'evaluation-tag'})
        self.assertEqual(cases['missing_claim_binding']['expect'],{'phase':'binding','code':'snapshot-binding'})
        self.assertEqual(cases['invalid_nominal_claim']['expect'],{'phase':'binding','code':'invalid-nominal-value'})
        # These are explicit synthetic values, not accepted host observations.
        self.assertEqual(self.doc['values']['claim_v1'],json.loads((BASE/'claim-v1.json').read_bytes()))
        self.assertEqual(self.doc['values']['claim_v2'],json.loads((BASE/'claim-v2.json').read_bytes()))
        self.assertTrue(all(s['status']=='Partial' for s in json.loads((BASE/'manifest.json').read_bytes())['scenarios']))

    def test_spdx_and_test_only_harness_boundary(self):
        self.assertEqual(Path(str(PATH)+'.license').read_text(),'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n')
        source=(ROOT/'crates/semantic-check/src/expression.rs').read_text()
        self.assertIn('#[cfg(test)]\nmod rp01_snapshots;',source)
        ci=(ROOT/'.github/workflows/rust-portability.yml').read_text()
        self.assertIn('tools/check_rp01_guard_snapshots.py',ci)


if __name__=='__main__': unittest.main()
