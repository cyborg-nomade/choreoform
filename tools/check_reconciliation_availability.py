# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Conditional no-inherited-value follow-up evidence, not live activation."""
import json
from pathlib import Path
import tempfile
import unittest

from check_authoring_evidence import check_links
from check_static_contracts import ROOT, EXAMPLES, run, service_resources
from check_qualified_endpoints import package


class ReconciliationAvailabilityEvidence(unittest.TestCase):
    def test_owned_source_report_keeps_zero_inherited_cells_and_stable_cycles(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion = package(directory)
            result = run('check-reconciliation-availability', source, companion, service_resources())
            self.assertEqual(result.returncode, 0, result.stderr)
            report = json.loads(result.stdout)
            self.assertEqual(report['status'], 'reconciliation-availability-checks-passed')
            self.assertIs(report['executable'], False)
            self.assertEqual(report['reconciliationEntries'][0]['inheritedCells'], [])
            self.assertEqual(report['reconciliationEntries'][0]['followup'], 'recovery')
            self.assertEqual(report['checkedReconciliationNodes'], ['recovery_done', 'service_observe'])
            self.assertNotIn('service_observe', report['ordinaryCoveredNodes'])
            self.assertEqual(report['uncoveredNodes'], [])
            self.assertTrue(report['remaining'])
            self.assertTrue(report['priorReconciliationObligations'])
            formatted = json.loads(run('format', source, companion, service_resources()).stdout)
            source.write_text(formatted['source'], encoding='utf-8')
            companion.write_text(formatted['companion'], encoding='utf-8')
            self.assertEqual(json.loads(run('check-reconciliation-availability', source, companion, service_resources()).stdout), report)

    def test_parent_initializer_is_unproved_but_fresh_local_constant_is_available(self):
        for local in [False, True]:
            with self.subTest(local=local), tempfile.TemporaryDirectory() as directory:
                source, companion = package(directory)
                text = source.read_text(encoding='utf-8')
                if local:
                    declaration = '''Keep "Local basis" as whole number under "Protection".
Sensitivity "internal". Purposes ["Expense review"]. Participants ["Reviewer"].
Capabilities empty. Invalidate ["Observe service"]. Initially (7). End data.
'''
                    text = text.replace('Step "Observe service".', declaration + 'Step "Observe service".')
                    ledger = json.loads(companion.read_bytes())
                    ledger['declarations'].append({'id': 'local_basis', 'kind': 'data', 'scope': 'recovery', 'name': 'Local basis'})
                    ledger['symbols'].append({'id': 'local_purpose', 'owner': 'local_basis', 'role': 'purpose', 'name': 'Expense review', 'wire': 'expense_review'})
                    ledger['generated'].append({'id': 'local_initial', 'owner': 'local_basis', 'role': 'initial', 'key': None})
                    companion.write_text(json.dumps(ledger), encoding='utf-8')
                else:
                    text = text.replace('Match when ((the fact "payload") equals (the fact "payload")).', 'Match when ((the value of "Request") equals (the value of "Request")).')
                source.write_text(text, encoding='utf-8')
                before = run('check-reconciliation-links', source, companion, service_resources())
                self.assertEqual(before.returncode, 0, before.stderr)
                result = run('check-reconciliation-availability', source, companion, service_resources())
                if local:
                    self.assertEqual(result.returncode, 0, result.stderr)
                else:
                    self.assertNotEqual(result.returncode, 0)
                    self.assertEqual(result.stdout, b'')
                    self.assertIn(b'reconciliation-availability-unproved', result.stderr)
                    self.assertIn(b'/body/nodes/service_observe', result.stderr)
                    self.assertNotIn(b'Synthetic request', result.stderr)

    def test_terminal_and_human_only_sources_have_no_invented_followup_entries(self):
        for name in ['terminal', 'review']:
            result = run('check-reconciliation-availability', EXAMPLES / f'{name}.choreo', EXAMPLES / f'{name}.bindings.json')
            self.assertEqual(result.returncode, 0, result.stderr)
            report = json.loads(result.stdout)
            self.assertEqual(report['reconciliationEntries'], [])
            self.assertEqual(report['checkedReconciliationNodes'], [])
            self.assertIs(report['executable'], False)

    def test_docs_and_licenses(self):
        for name in ['docs/decisions/0032-conditional-reconciliation-availability.md',
                     'docs/evaluation/0032-reconciliation-entry-plan.md']:
            path = ROOT / name
            check_links(path)
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))


if __name__ == '__main__':
    unittest.main()
