# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Declaration-link source boundaries, not ownership or provider observations."""
import json
from pathlib import Path
import tempfile
import unittest

import authoring_probe as grammar
from check_authoring_evidence import check_links
from check_static_contracts import ROOT, EXAMPLES, run, service_resources
from check_retry_contract import RETRY_ID, RETRY_HASH


def service_package(directory, modern=False, owned=False):
    source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
    text = (EXAMPLES / 'service.choreo').read_text(encoding='utf-8')
    ledger = json.loads((EXAMPLES / 'service.bindings.json').read_bytes())
    if modern:
        text = text.replace('"0.2.0"', '"0.3.0"')
        ledger.update({'profile': '0.3.0', 'version': '0.2.0'})
        ledger['contracts']['core'].update({'contract': RETRY_ID, 'digest': 'sha256:' + RETRY_HASH})
    if owned:
        block = '  Step "Observe service".\n    Wait under "Service reconciliation".\n    On "observed", continue with "Done".\n  End step.\n'
        assert block in text
        text = text.replace(block, '').replace('Fully terminal.', 'Reconcile in "Recovery".')
        text = text.replace('outcome "observed" of "Observe service"', 'outcome "observed" of "Observe service" in "Recovery"')
        text = text.replace('End process.', '''Rule for closure "Recovery close". Fully terminal. End policy.
Scope "Recovery" within "Service review example".
Start at "Observe service". Inputs empty. Outputs empty. Outcomes ["complete"].
Use cancellation "Stop", faults "Fault handling", closure "Recovery close" and races "Commit order".
Step "Observe service". Wait under "Service reconciliation". On "observed", continue with "Recovery done". End step.
Step "Recovery done". Finish as "complete". End step. End scope.
End process.''')
        root_name = next(d['name'] for d in ledger['declarations'] if d['id'] == ledger['root'])
        text = text.replace('Scope "Recovery" within "Service review example".', f'Scope "Recovery" within "{root_name}".')
        next(d for d in ledger['declarations'] if d['id'] == 'service_observe')['scope'] = 'recovery'
        ledger['declarations'] += [
            {'id': 'recovery', 'kind': 'scope', 'scope': ledger['root'], 'name': 'Recovery'},
            {'id': 'recovery_close', 'kind': 'policy', 'scope': ledger['root'], 'name': 'Recovery close'},
            {'id': 'recovery_done', 'kind': 'node', 'scope': 'recovery', 'name': 'Recovery done'},
        ]
        ledger['symbols'].append({'id': 'recovery_complete', 'owner': 'recovery', 'role': 'outcome', 'name': 'complete', 'wire': 'complete'})
    source.write_text(text, encoding='utf-8')
    companion.write_text(json.dumps(ledger), encoding='utf-8')
    return source, companion


class ReconciliationLinksEvidence(unittest.TestCase):
    def test_unique_wait_in_old_service_package_is_not_a_surviving_followup(self):
        for modern in [False, True]:
            with self.subTest(modern=modern), tempfile.TemporaryDirectory() as directory:
                source, companion = service_package(directory, modern)
                resources = service_resources()
                self.assertEqual(run('check-exceptional-availability', source, companion, resources).returncode, 0)
                result = run('check-reconciliation-links', source, companion, resources)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(b'reconciliation-followup-unproved', result.stderr)
                self.assertIn(b'/body/scopes/scope_service/closure', result.stderr)
                self.assertEqual(result.stdout, b'')

    def test_human_and_terminal_source_reports_remain_nonexecutable_and_reparse_stable(self):
        for name in ['review', 'terminal']:
            with self.subTest(name=name), tempfile.TemporaryDirectory() as directory:
                source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
                source.write_bytes((EXAMPLES / f'{name}.choreo').read_bytes())
                companion.write_bytes((EXAMPLES / f'{name}.bindings.json').read_bytes())
                result = run('check-reconciliation-links', source, companion)
                self.assertEqual(result.returncode, 0, result.stderr)
                report = json.loads(result.stdout)
                self.assertIs(report['executable'], False)
                self.assertEqual(report['reconciliationUses'], [])
                self.assertEqual(report['unusedEffectContexts'], [])
                self.assertTrue(report['priorExceptionalObligations'])
                package = json.loads(run('format', source, companion).stdout)
                source.write_text(package['source'], encoding='utf-8')
                companion.write_text(package['companion'], encoding='utf-8')
                self.assertEqual(json.loads(run('check-reconciliation-links', source, companion).stdout), report)

    def test_old_source_profiles_do_not_silently_resolve_private_child_wait_endpoints(self):
        for modern in [False, True]:
            with self.subTest(modern=modern), tempfile.TemporaryDirectory() as directory:
                source, companion = service_package(directory, modern, owned=True)
                grammar.recognize(source.read_text(encoding='utf-8'))
                grammar.validate_ledger(grammar.strict_json(companion.read_bytes()))
                result = run('lower', source, companion, service_resources())
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(b'reference-missing', result.stderr)
                self.assertEqual(result.stdout, b'')

    def test_documentation_links_and_licenses(self):
        for name in ['docs/decisions/0030-reconciliation-declaration-links.md',
                     'docs/evaluation/0030-reconciliation-links-plan.md',
                     'docs/evaluation/0030-reconciliation-declaration-links.md']:
            path = ROOT / name
            check_links(path)
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))


if __name__ == '__main__':
    unittest.main()
