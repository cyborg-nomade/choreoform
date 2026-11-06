# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Settlement dependency/source evidence, not receiver acceptance or ownership."""
import json
from pathlib import Path
import tempfile
import unittest

import jsonschema
import authoring_probe as grammar
import check_ir_fixtures as wire
from check_authoring_evidence import check_links
from check_static_contracts import ROOT, EXAMPLES, run
from check_retry_contract import RETRY_ID, RETRY_HASH


def settlement_package(directory, modern=False, mode='transfer to "Terminal example"'):
    text = (EXAMPLES / 'terminal.choreo').read_text(encoding='utf-8')
    text = text.replace('Permit when (true).', 'Permit when ((the value of "Receiver evidence") equals (0)).')
    text = text.replace('Finish as "complete".', 'Run "Child" with inputs empty and outputs empty. On "complete", continue with "Final".')
    text = text.replace('End process.', f'''
Require actor "Reader" under "Reader requirement".
Rule for actor "Reader requirement". Permit when (true). End policy.
Rule for protection "Protection". Permit access when (true). End policy.
Keep "Receiver evidence" as whole number under "Protection".
Sensitivity "internal". Purposes ["Test"]. Participants ["Reader"].
Capabilities empty. Invalidate empty. Initially (0). End data.
Step "Final". Finish as "complete". End step.
Scope "Child" within "Terminal example".
Start at "Child done". Inputs empty. Outputs empty. Outcomes ["complete"].
Use cancellation "Child stop", faults "Fault handling", closure "Close" and races "Commit order".
Rule for cancel "Child stop". Authority "Reader". For children use "Transfer". End policy.
Rule for settlement "Transfer". Unfinished work {mode}. End policy.
Step "Child done". Finish as "complete". End step. End scope.
End process.
''')
    ledger = json.loads((EXAMPLES / 'terminal.bindings.json').read_bytes())
    ledger['declarations'] += [
        {'id': identity, 'kind': kind, 'scope': scope, 'name': name}
        for identity, kind, scope, name in [
            ('reader_actor', 'actor', ledger['root'], 'Reader'),
            ('reader_requirement', 'policy', ledger['root'], 'Reader requirement'),
            ('protect', 'policy', ledger['root'], 'Protection'),
            ('receiver_evidence', 'data', ledger['root'], 'Receiver evidence'),
            ('final_node', 'node', ledger['root'], 'Final'),
            ('scope_child', 'scope', ledger['root'], 'Child'),
            ('child_cancel', 'policy', 'scope_child', 'Child stop'),
            ('child_transfer', 'policy', 'scope_child', 'Transfer'),
            ('child_done', 'node', 'scope_child', 'Child done'),
        ]]
    ledger['symbols'] += [
        {'id': identity, 'owner': owner, 'role': role, 'name': name, 'wire': value}
        for identity, owner, role, name, value in [
            ('evidence_purpose', 'receiver_evidence', 'purpose', 'Test', 'test'),
            ('call_complete', 'node_done', 'outcome', 'complete', 'complete'),
            ('child_complete', 'scope_child', 'outcome', 'complete', 'complete'),
        ]]
    ledger['generated'] += [
        {'id': 'evidence_initial', 'owner': 'receiver_evidence', 'role': 'initial', 'key': None},
        {'id': 'call_flow', 'owner': 'node_done', 'role': 'flow', 'key': 'call_complete'},
    ]
    if modern:
        text = text.replace('"0.2.0"', '"0.3.0"')
        ledger.update({'profile': '0.3.0', 'version': '0.2.0'})
        ledger['contracts']['core'].update({'contract': RETRY_ID, 'digest': 'sha256:' + RETRY_HASH})
    source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
    source.write_text(text, encoding='utf-8')
    companion.write_text(json.dumps(ledger), encoding='utf-8')
    return source, companion


class SettlementDependencies(unittest.TestCase):
    def test_receiver_reads_survive_both_versioned_source_and_report_cycles(self):
        for modern in [False, True]:
            with self.subTest(modern=modern), tempfile.TemporaryDirectory() as directory:
                source, companion = settlement_package(directory, modern)
                grammar.recognize(source.read_text(encoding='utf-8'))
                grammar.validate_ledger(grammar.strict_json(companion.read_bytes()))
                result = run('lower', source, companion)
                self.assertEqual(result.returncode, 0, result.stderr)
                candidate = wire.load(result.stdout)
                jsonschema.Draft202012Validator(wire.SCHEMA).validate(candidate)
                self.assertEqual(candidate['revision'], wire.revision(candidate))
                for policy in ['child_transfer', 'child_cancel']:
                    self.assertEqual(candidate['body']['policies'][policy]['body']['reads'], {'receiver_evidence': True})
                self.assertEqual(candidate['body']['nodes']['child_done']['reads'], {'receiver_evidence': True})
                checked = run('check-protection', source, companion)
                self.assertEqual(checked.returncode, 0, checked.stderr)
                report = json.loads(checked.stdout)
                self.assertIs(report['executable'], False)
                self.assertIn('child_done', report['coveredNodes'])
                self.assertEqual(report['revision'], candidate['revision'])
                package = json.loads(run('format', source, companion).stdout)
                source.write_text(package['source'], encoding='utf-8')
                companion.write_text(package['companion'], encoding='utf-8')
                self.assertEqual(json.loads(run('lower', source, companion).stdout), candidate)
                self.assertEqual(json.loads(run('check-protection', source, companion).stdout), report)

    def test_nontransfer_modes_do_not_borrow_receiver_authority(self):
        for mode in ['cancel', 'retain']:
            with self.subTest(mode=mode), tempfile.TemporaryDirectory() as directory:
                source, companion = settlement_package(directory, mode=mode)
                result = run('lower', source, companion)
                self.assertEqual(result.returncode, 0, result.stderr)
                candidate = json.loads(result.stdout)
                for policy in ['child_transfer', 'child_cancel']:
                    self.assertEqual(candidate['body']['policies'][policy]['body']['reads'], {})
                self.assertIsNone(candidate['body']['policies']['child_transfer']['body']['owner'])
                self.assertEqual(run('check-protection', source, companion).returncode, 0)

    def test_documentation_links_and_licenses(self):
        for name in ['docs/evaluation/0019-settlement-access-plan.md',
                     'docs/evaluation/0019-settlement-authority-dependencies.md']:
            path = ROOT / name
            check_links(path)
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))


if __name__ == '__main__':
    unittest.main()
