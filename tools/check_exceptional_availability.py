# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Conditional input-contract/body evidence, never live fault snapshots."""
import json
import tempfile
import unittest

import jsonschema
import authoring_probe as grammar
import check_ir_fixtures as wire
from check_authoring_evidence import check_links
from check_static_contracts import ROOT, run
from check_fault_routes import fault_package


def input_bound_package(directory, modern=False):
    source, companion = fault_package(directory, modern)
    text = source.read_text(encoding='utf-8')
    text = text.replace('Inputs empty.', 'Inputs ["Evidence" uses "Receiver evidence"].', 1)
    text = text.replace('Initially (0).', 'Initially unavailable.')
    source.write_text(text, encoding='utf-8')
    ledger = json.loads(companion.read_bytes())
    ledger['symbols'].append({'id': 'evidence_input', 'owner': ledger['root'], 'role': 'input', 'name': 'Evidence', 'wire': 'evidence'})
    ledger['generated'] = [row for row in ledger['generated'] if row['id'] != 'evidence_initial']
    ledger['retired'].append('evidence_initial')
    companion.write_text(json.dumps(ledger), encoding='utf-8')
    return source, companion


class ExceptionalAvailabilityEvidence(unittest.TestCase):
    def test_both_profiles_preserve_conditional_body_coverage_and_explicit_input_assumptions(self):
        for modern in [False, True]:
            with self.subTest(modern=modern), tempfile.TemporaryDirectory() as directory:
                source, companion = input_bound_package(directory, modern)
                grammar.recognize(source.read_text(encoding='utf-8'))
                grammar.validate_ledger(grammar.strict_json(companion.read_bytes()))
                result = run('lower', source, companion)
                self.assertEqual(result.returncode, 0, result.stderr)
                candidate = wire.load(result.stdout)
                jsonschema.Draft202012Validator(wire.SCHEMA).validate(candidate)
                self.assertEqual(candidate['revision'], wire.revision(candidate))
                checked = run('check-exceptional-availability', source, companion)
                self.assertEqual(checked.returncode, 0, checked.stderr)
                report = json.loads(checked.stdout)
                self.assertIs(report['executable'], False)
                self.assertEqual(report['revision'], candidate['revision'])
                self.assertEqual(report['handlerEntries'], [{'handler': 'first_handler', 'requiredAncestorInputs': ['receiver_evidence']}])
                self.assertEqual(report['checkedExceptionalNodes'], ['first_handler_done'])
                self.assertEqual(report['checkedExceptionalScopes'], ['first_handler'])
                self.assertIn('first_handler_done', report['ordinaryUncoveredNodes'])
                self.assertNotIn('first_handler_done', report['ordinaryCoveredNodes'])
                self.assertNotIn('first_handler_done', report['uncoveredNodes'])
                self.assertIn('live-ancestor-input-revisions-and-successful-activation-premises', report['remaining'])
                self.assertIn('exceptional-fault-control-protection-and-invalidation', report['remaining'])
                self.assertTrue(report['priorFaultRouteObligations'])
                self.assertTrue(report['priorTimingObligations'])
                package = json.loads(run('format', source, companion).stdout)
                source.write_text(package['source'], encoding='utf-8')
                companion.write_text(package['companion'], encoding='utf-8')
                self.assertIn('evidence_initial', json.loads(companion.read_bytes())['retired'])
                self.assertEqual(json.loads(run('lower', source, companion).stdout), candidate)
                self.assertEqual(json.loads(run('check-exceptional-availability', source, companion).stdout), report)

    def test_prior_routing_proof_and_owner_initializer_are_not_an_actual_fault_snapshot(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion = fault_package(directory)
            self.assertEqual(run('check-fault-routes', source, companion).returncode, 0)
            result = run('check-exceptional-availability', source, companion)
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(result.stdout, b'')
            self.assertIn(b'exceptional-availability-unproved', result.stderr)
            self.assertIn(b'/body/nodes/first_handler_done', result.stderr)

    def test_documentation_links_and_licenses(self):
        for name in ['docs/decisions/0029-conditional-handler-availability.md',
                     'docs/evaluation/0029-exceptional-availability-plan.md',
                     'docs/evaluation/0029-conditional-handler-availability.md']:
            path = ROOT / name
            check_links(path)
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))


if __name__ == '__main__':
    unittest.main()
