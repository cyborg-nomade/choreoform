# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Abstract fault-route source cycles, not live dispatch or handler data proof."""
import json
import tempfile
import unittest

import jsonschema
import authoring_probe as grammar
import check_ir_fixtures as wire
from check_authoring_evidence import check_links
from check_static_contracts import ROOT, run
from check_settlement_dependencies import settlement_package


def fault_package(directory, modern=False, cycle=False):
    source, companion = settlement_package(directory, modern, mode='cancel')
    text = source.read_text(encoding='utf-8')
    names = [('First handler', 'first_handler')] if not cycle else [('First handler', 'first_handler'), ('Second handler', 'second_handler')]
    entries = ', '.join(f'"failure{i}" by "{name}"' for i, (name, _) in enumerate(names))
    text = text.replace('Rule for faults "Fault handling".\n    Handle empty.',
                        f'Rule for faults "Fault handling".\n    Handle [{entries}].')
    text = text.replace('Use cancellation "Child stop", faults "Fault handling"',
                        'Use cancellation "Child stop", faults "Empty faults"')
    addition = 'Rule for faults "Empty faults". Handle empty. End policy.\n'
    for name, _ in names:
        addition += f'''Scope "{name}" within "Terminal example".
Start at "Handler done". Inputs empty. Outputs empty. Outcomes ["complete"].
Use cancellation "Stop", faults "Empty faults", closure "Close" and races "Commit order".
Step "Handler done". Finish as "complete". End step. End scope.\n'''
    text = text.replace('End process.', addition + 'End process.')
    source.write_text(text, encoding='utf-8')
    ledger = json.loads(companion.read_bytes())
    ledger['declarations'].append({'id': 'empty_faults', 'kind': 'policy', 'scope': ledger['root'], 'name': 'Empty faults'})
    for name, identity in names:
        ledger['declarations'] += [
            {'id': identity, 'kind': 'scope', 'scope': ledger['root'], 'name': name},
            {'id': identity + '_done', 'kind': 'node', 'scope': identity, 'name': 'Handler done'},
        ]
        ledger['symbols'].append({'id': identity + '_complete', 'owner': identity, 'role': 'outcome', 'name': 'complete', 'wire': 'complete'})
    companion.write_text(json.dumps(ledger), encoding='utf-8')
    return source, companion


class FaultRouteEvidence(unittest.TestCase):
    def test_both_profiles_preserve_route_metadata_without_claiming_handler_data_coverage(self):
        for modern in [False, True]:
            with self.subTest(modern=modern), tempfile.TemporaryDirectory() as directory:
                source, companion = fault_package(directory, modern)
                grammar.recognize(source.read_text(encoding='utf-8'))
                grammar.validate_ledger(grammar.strict_json(companion.read_bytes()))
                result = run('lower', source, companion)
                self.assertEqual(result.returncode, 0, result.stderr)
                candidate = wire.load(result.stdout)
                jsonschema.Draft202012Validator(wire.SCHEMA).validate(candidate)
                self.assertEqual(candidate['revision'], wire.revision(candidate))
                checked = run('check-fault-routes', source, companion)
                self.assertEqual(checked.returncode, 0, checked.stderr)
                report = json.loads(checked.stdout)
                self.assertIs(report['executable'], False)
                self.assertEqual(report['revision'], candidate['revision'])
                self.assertEqual(report['analysisContexts'], 3)
                self.assertIn('first_handler', report['checkedFaultScopes'])
                self.assertIn('first_handler_done', report['uncoveredNodes'])
                self.assertNotIn('first_handler_done', report['coveredNodes'])
                self.assertEqual(report['uncoveredHandlerSelections'], [])
                self.assertEqual(len(report['checkedHandlerSelections']), 1)
                self.assertEqual({r['originScope'] for r in report['faultRoutes']}, {'scope_terminal', 'scope_child'})
                for route in report['faultRoutes']:
                    self.assertIsNone(route['from'])
                    self.assertEqual(route['selected']['handler'], 'first_handler')
                    self.assertEqual(route['selected']['fault'], 'failure0')
                self.assertIn('exceptional-handler-entry-availability-protection-and-invalidation', report['remaining'])
                self.assertTrue(report['priorTimingObligations'])
                package = json.loads(run('format', source, companion).stdout)
                source.write_text(package['source'], encoding='utf-8')
                companion.write_text(package['companion'], encoding='utf-8')
                self.assertEqual(json.loads(run('lower', source, companion).stdout), candidate)
                self.assertEqual(json.loads(run('check-fault-routes', source, companion).stdout), report)

    def test_conservative_cycle_refusal_follows_a_passing_timing_stage(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion = fault_package(directory, cycle=True)
            self.assertEqual(run('check-timing', source, companion).returncode, 0)
            result = run('check-fault-routes', source, companion)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b'fault-routing-cycle-unproved', result.stderr)
            self.assertIn(b'/body/scopes/scope_terminal/faults', result.stderr)
            self.assertEqual(result.stdout, b'')

    def test_documentation_and_licenses(self):
        for name in ['docs/decisions/0028-conservative-fault-routing.md',
                     'docs/evaluation/0028-fault-routing-plan.md',
                     'docs/evaluation/0028-conservative-fault-routing.md']:
            path = ROOT / name
            check_links(path)
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))


if __name__ == '__main__':
    unittest.main()
