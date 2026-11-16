# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Synthetic local instance evidence; no live effect, host grant or full-case proof."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest

import authoring_probe as grammar
from benchmarks.rp01_resources import encoded
from check_metadata_authoring import invoke
from check_readonly_continuation_authoring import STAGES
from check_source_capacity import fixed
from check_static_contracts import ROOT
from proofs.effectful_template_instances import package

FIXTURES = ROOT / 'crates/authoring-frontend/tests/capture'


class EffectfulInstanceEvidence(unittest.TestCase):
    def checked(self, p, stage='lower'):
        result = invoke(p, stage)
        self.assertEqual(result.returncode, 0, (stage, result.stderr))
        return json.loads(result.stdout)

    def test_distinct_complete_instances_all_classes_and_compensation_identity(self):
        for effect in ['read-only', 'compensatable', 'irreversible']:
            for linked in [False, True]:
                p = package(effect=effect, compensates=linked)
                body = self.checked(p)['body']
                for key in ['a', 'b']:
                    self.assertEqual(body['scopes'][key+'_service']['parent'], key+'_caller')
                    self.assertEqual(body['nodes'][key+'_activity']['target'], key+'_adapter')
                    self.assertEqual(body['policies'][key+'_effects']['body']['class'], effect)
                    self.assertEqual(body['policies'][key+'_effects']['body']['reconciliation'], key+'_wait')
                    self.assertEqual(body['policies'][key+'_work']['body']['completion']['op'], 'eq')
                self.assertEqual(body['policies']['b_work']['body']['compensates'], 'a_activity' if linked else None)
                for stage in STAGES:
                    report = self.checked(p, stage)
                    self.assertIs(report['executable'], False)
                    self.assertTrue(report['remaining'])
                    if stage == 'invalidation-requirements':
                        self.assertEqual(len(report['requiredInvalidationObligations']), 7)
                        self.assertIn({'cell':'first_result', 'site':'b_activity'}, report['requiredInvalidationObligations'])
                        self.assertEqual(report['missingInvalidationObligations'], [])
                self.assertEqual(report['checkedReconciliationNodes'], [])
                self.assertEqual(report['uncoveredNodes'], ['a_observe', 'a_recovered', 'b_observe', 'b_recovered'])
                self.assertTrue(report['conditionalOwnedLocalChecks']['conditional'])
                for family in ['declaredWaitCaptures', 'declaredRetainedRequests', 'declaredFaultSourceUses']:
                    self.assertEqual(len(report[family]), 2)
                    self.assertTrue(all(row['hostRequirements'] for row in report[family]))

    def test_isolated_source_refusals_are_preserved_at_the_actual_stage(self):
        cases = [
            ('Request "B capability" using "Service input"', 'Request "A capability" using "Service input"', 'lower', 'wait-endpoint-use-unproved'),
            ('Reconcile under "B observations".', 'Reconcile under "Authority".', 'lower', 'wait-endpoint-use-unproved'),
            ('Reconcile under "B observations".', 'Reconcile under "A observations".', 'lower', 'wait-endpoint-use-unproved'),
            ('"Observe work" in "Recovery" in "B service" in "B caller" in "Main"', '"Observe work" in "Recovery" in "A service" in "A caller" in "Main"', 'lower', 'wait-endpoint-owner'),
            ('Let "Service input" mean (the value of "Service request")', 'Let "Service input" mean (the value of "Service request" in "A service" in "A caller" in "Main")', 'lower', 'reference-qualification'),
            ('Invalidate ["Request work" in "A service" in "A caller" in "Main", "Request work" in "B service" in "B caller" in "Main"].', 'Invalidate empty.', 'check-invalidation', 'invalidation-coverage'),
            ('Retain the original request from "Request work" and record in "Original request".', 'Retain the original request from "Request work" and record in "Captured".', 'check', 'retained-request-readonly'),
            ('for observations from "Terminal facts" about "Request work"', 'for observations from "Fault facts" about "Request work"', 'lower', 'resource-kind'),
            ('about "Request work" and record', 'about "Service done" and record', 'check', 'wait-capture-origin-unproved'),
        ]
        for old, new, stage, code in cases:
            s, l, r = package()
            self.assertIn(old, s)
            result = invoke((s.replace(old, new, -1 if code == 'reference-qualification' else 1), l, r), stage)
            self.assertEqual(result.returncode, 1, (code, result.stderr))
            self.assertEqual(result.stdout, b'')
            self.assertEqual(result.stderr.split(b' at ')[0], code.encode())

    def test_shared_provider_requires_exact_recipient_claims_and_rebound_sources(self):
        good = package()
        bad = package(recipients=('a_adapter',))
        # A's result flows into storage readable by both logical capabilities.
        result = invoke(bad, 'check')
        self.assertEqual(result.returncode, 1)
        self.assertIn(b'information-flow at /body/nodes/a_activity;', result.stderr)
        self.assertEqual(result.stdout, b'')
        self.checked(good, STAGES[-1])
        g, b = [json.loads(p[2][0][2]) for p in [good, bad]]
        self.assertEqual(g['id'], b['id'])
        self.assertNotEqual(good[2][0][1], bad[2][0][1])
        for p in [good, bad]:
            provider_pin = {'id':p[2][0][0], 'revision':p[2][0][1]}
            for _, _, raw in p[2][1:3]:
                self.assertEqual(json.loads(raw)['provider'], provider_pin)
        # Reusing an old pin for new bytes cannot express the new claim.
        s, l, r = deepcopy(good)
        r[0] = (r[0][0], r[0][1], bad[2][0][2])
        result = invoke((s, l, r))
        self.assertEqual(result.returncode, 1)
        self.assertIn(b'resource-digest', result.stderr)
        self.assertEqual(result.stdout, b'')

    def test_full_fixture_roundtrip_and_independent_integrity(self):
        p = package()
        grammar.recognize(p[0]); grammar.validate_ledger(p[1])
        formatted = self.checked(p, 'format')
        q = (formatted['source'], json.loads(formatted['companion']), p[2])
        self.assertEqual(self.checked(p), self.checked(q))
        self.assertEqual(self.checked(p, STAGES[-1]), self.checked(q, STAGES[-1]))
        self.assertEqual(self.checked(p, 'integrity'), grammar.package_digests(p[0].encode(), encoded(p[1])))
        for identity, revision, raw in p[2]:
            self.assertEqual(revision, 'sha256:'+sha256(raw).hexdigest())
            self.assertIn(identity.encode(), raw)

    def test_persisted_source_companion_resources_are_exact_generated_evidence(self):
        s, l, r = package()
        expected = {'choreo':s.encode(), 'bindings.json':encoded(l), 'resources.json':encoded([{'id':i, 'revision':v, 'bytes':b.decode()} for i,v,b in r])}
        for suffix, raw in expected.items():
            path = FIXTURES / ('effectful-instances.'+suffix)
            self.assertEqual(path.read_bytes(), raw)
            self.assertEqual(path.with_name(path.name+'.license').read_text(), 'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n')
        self.assertIn('tools/check_effectful_instances.py', (ROOT/'.github/workflows/rust-portability.yml').read_text())

    def test_no_benchmark_current_or_historical_artifact_is_mutated(self):
        before = fixed()
        for linked in [False, True]:
            self.checked(package(compensates=linked), STAGES[-1])
        self.assertEqual(fixed(), before)


if __name__ == '__main__':
    unittest.main()
