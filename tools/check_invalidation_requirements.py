# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Advisory ordinary requirements never replace strict checks or host authority."""
from copy import deepcopy
import json
from pathlib import Path
import tempfile
import unittest

from check_closed_provenance import package, invoke, large_package
from check_static_contracts import ROOT, run

BASE = ROOT / 'docs/benchmarks/rp01'


def missing_case():
    source, ledger = package(omit_ancestor=True)
    source = source.replace('Invalidate ["Review"].', 'Invalidate empty.')
    return source, ledger


def pairs(values):
    return {(v['site'], v['cell']) for v in values}


class InvalidationRequirementsEvidence(unittest.TestCase):
    def test_exact_required_minus_declared_set_does_not_bypass_any_strict_gate(self):
        source, ledger = missing_case()
        result = invoke(source, ledger, 'invalidation-requirements')
        self.assertEqual(result.returncode, 0, result.stderr)
        report = json.loads(result.stdout)
        self.assertEqual(report['status'], 'invalidation-requirements-derived')
        self.assertIs(report['advisory'], True)
        self.assertIs(report['executable'], False)
        self.assertEqual(pairs(report['missingInvalidationObligations']), {('review','seed'),('review','copy')})
        raw = json.loads(invoke(source, ledger, 'lower').stdout)
        declared = {(site, cell) for cell, d in raw['body']['data'].items() for site in d['invalidates']}
        self.assertEqual(pairs(report['missingInvalidationObligations']), pairs(report['requiredInvalidationObligations']) - declared)
        self.assertEqual(report['revision'], raw['revision'])
        self.assertEqual(report['requiredNextCheck'], 'check-invalidation-at-final-definition-revision')
        for stage in ['check-invalidation', 'check-concurrency', 'check-timing', 'check-reconciliation-availability']:
            result = invoke(source, ledger, stage)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b'invalidation-coverage', result.stderr)
            self.assertEqual(result.stdout, b'')

    def test_fresh_exact_links_pass_but_an_empty_advisory_is_still_not_a_passed_report(self):
        old_source, old_ledger = missing_case()
        old = json.loads(invoke(old_source, old_ledger, 'invalidation-requirements').stdout)
        source, ledger = package()
        fresh = json.loads(invoke(source, ledger, 'invalidation-requirements').stdout)
        strict = invoke(source, ledger)
        self.assertEqual(strict.returncode, 0, strict.stderr)
        self.assertNotEqual(old['revision'], fresh['revision'])
        self.assertEqual(fresh['missingInvalidationObligations'], [])
        self.assertIs(fresh['advisory'], True)
        self.assertEqual(pairs(old['requiredInvalidationObligations']), pairs(fresh['requiredInvalidationObligations']))
        self.assertEqual(fresh['historicalOrigins'], json.loads(strict.stdout)['historicalOrigins'])

    def test_unproved_private_contexts_and_bad_resources_never_return_a_partial_list(self):
        source, ledger = package(survivor='writer')
        result = invoke(source, ledger, 'invalidation-requirements')
        self.assertIn(b'invalidation-context-unproved', result.stderr)
        self.assertEqual(result.stdout, b'')
        manifest = json.loads((BASE / 'manifest.json').read_bytes())
        supplied = [(r['id'], r['revision'], ROOT / r['path']) for r in manifest['resources']]
        result = run('invalidation-requirements', BASE / 'source.choreo', BASE / 'bindings.json', supplied[1:])
        self.assertIn(b'resource-missing-or-ambiguous', result.stderr)
        self.assertEqual(result.stdout, b'')
        with tempfile.TemporaryDirectory() as directory:
            corrupt = Path(directory) / 'corrupt.json'
            corrupt.write_bytes(b'{}')
            supplied[0] = (supplied[0][0], supplied[0][1], corrupt)
            result = run('invalidation-requirements', BASE / 'source.choreo', BASE / 'bindings.json', supplied)
        self.assertIn(b'resource-digest', result.stderr)
        self.assertEqual(result.stdout, b'')

    def test_full_rp01_is_advisory_with_real_missing_links_and_unchanged_strict_refusal(self):
        manifest = json.loads((BASE / 'manifest.json').read_bytes())
        supplied = [(r['id'], r['revision'], ROOT / r['path']) for r in manifest['resources']]
        before = (BASE / 'source.choreo').read_bytes(), (BASE / 'bindings.json').read_bytes()
        result = run('invalidation-requirements', BASE / 'source.choreo', BASE / 'bindings.json', supplied)
        self.assertEqual(result.returncode, 0, result.stderr)
        report = json.loads(result.stdout)
        self.assertEqual(len(report['requiredInvalidationObligations']), 84)
        self.assertEqual(len(report['missingInvalidationObligations']), 72)
        self.assertEqual(len(report['historicalOrigins']), 88)
        candidate = json.loads((BASE / 'candidate.ir.json').read_bytes())
        declared = {(site, cell) for cell, d in candidate['body']['data'].items() for site in d['invalidates']}
        self.assertEqual(pairs(report['missingInvalidationObligations']), pairs(report['requiredInvalidationObligations']) - declared)
        self.assertIs(report['advisory'], True)
        self.assertIs(report['executable'], False)
        strict = run('check-invalidation', BASE / 'source.choreo', BASE / 'bindings.json', supplied)
        self.assertIn(b'invalidation-coverage at /body/data/disposition/invalidates', strict.stderr)
        self.assertEqual(before, ((BASE / 'source.choreo').read_bytes(), (BASE / 'bindings.json').read_bytes()))

    def test_unvisited_declarations_and_formatting_do_not_invent_requirements_or_change_identity(self):
        source, ledger = package()
        original = deepcopy(ledger)
        formatted = json.loads(invoke(source, ledger, 'format').stdout)
        a = json.loads(invoke(source, ledger, 'invalidation-requirements').stdout)
        b = json.loads(invoke(formatted['source'], json.loads(formatted['companion']), 'invalidation-requirements').stdout)
        self.assertEqual(a,b)
        self.assertEqual(ledger,original)
        ledger['declarations'] += [
            {'id':'unused_condition','kind':'expression','scope':'root','name':'Unused condition'},
            {'id':'unused','kind':'node','scope':'root','name':'Unvisited'},
        ]
        for tag in ['go','other']:
            ledger['symbols'].append({'id':'unused_'+tag,'owner':'unused','role':'outcome','name':tag,'wire':tag})
            ledger['generated'].append({'id':'unused_'+tag+'_flow','owner':'unused','role':'flow','key':'unused_'+tag})
        source = source.replace('End process.', 'Let "Unused condition" mean (true) as truth value with parameters empty. Step "Unvisited". Choose by ["go" when "Unused condition"]; otherwise "other". On "go", continue with "Done". On "other", continue with "Done". End step. End process.')
        result = invoke(source, ledger, 'invalidation-requirements')
        self.assertEqual(result.returncode, 0, result.stderr)
        r = json.loads(result.stdout)
        self.assertIn('unused',r['uncoveredInvalidationNodes'])
        self.assertNotIn('unused',r['analyzedInvalidationNodes'])
        self.assertTrue(all(o['site'] != 'unused' for o in r['requiredInvalidationObligations']))

    def test_native_advisory_report_budget_refuses_before_stdout(self):
        source, ledger = large_package()
        result = invoke(source, ledger, 'invalidation-requirements')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b'report-budget',result.stderr)
        self.assertEqual(result.stdout,b'')


if __name__ == '__main__':
    unittest.main()
