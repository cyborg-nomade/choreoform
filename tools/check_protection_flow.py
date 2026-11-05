# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Artifact and source-cycle evidence; semantic results come from the Rust stage."""
import json
from pathlib import Path
import tempfile
import unittest

import jsonschema
import authoring_probe as grammar
import check_ir_fixtures as wire
from check_authoring_evidence import check_links
from check_static_contracts import ROOT, EXAMPLES, run, service_resources


def branch_input(private):
    source = (EXAMPLES / 'control.choreo').read_text(encoding='utf-8')
    ledger = json.loads((EXAMPLES / 'control.bindings.json').read_bytes())
    purpose = 'Private' if private else 'Illustration'
    declaration = f'''Keep "Secret choice" as truth value under "Protection".
    Sensitivity "internal". Purposes ["{purpose}"]. Participants ["Reviewer"].
    Capabilities empty. Invalidate empty. Initially unavailable. End data.
    '''
    source = source.replace('Inputs empty.', 'Inputs ["Secret" uses "Secret choice"].')
    source = source.replace('Let "First route applies"', declaration + 'Let "First route applies"')
    source = source.replace('mean (true)', 'mean (the value of "Secret choice")')
    ledger['declarations'].append({'id': 'secret_choice', 'kind': 'data', 'scope': 'scope_branch', 'name': 'Secret choice'})
    ledger['symbols'] += [
        {'id': 'secret_input', 'owner': 'scope_branch', 'role': 'input', 'name': 'Secret', 'wire': 'secret_argument'},
        {'id': 'secret_purpose', 'owner': 'secret_choice', 'role': 'purpose', 'name': purpose, 'wire': purpose.lower()},
    ]
    return source, json.dumps(ledger)


class ProtectionEvidence(unittest.TestCase):
    def test_reports_and_original_package_cycles(self):
        for name in ['terminal', 'review', 'service', 'control', 'root-input']:
            with self.subTest(package=name), tempfile.TemporaryDirectory() as directory:
                source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
                if name == 'root-input':
                    text, bindings = branch_input(False)
                    source.write_text(text, encoding='utf-8')
                    companion.write_text(bindings, encoding='utf-8')
                else:
                    source.write_bytes((EXAMPLES / f'{name}.choreo').read_bytes())
                    companion.write_bytes((EXAMPLES / f'{name}.bindings.json').read_bytes())
                resources = service_resources() if name == 'service' else []
                grammar.recognize(source.read_text(encoding='utf-8'))
                lowered = run('lower', source, companion, resources)
                self.assertEqual(lowered.returncode, 0, lowered.stderr)
                candidate = wire.load(lowered.stdout)
                jsonschema.Draft202012Validator(wire.SCHEMA).validate(candidate)
                self.assertEqual(candidate['revision'], wire.revision(candidate))
                checked = run('check-protection', source, companion, resources)
                self.assertEqual(checked.returncode, 0, checked.stderr)
                report = json.loads(checked.stdout)
                self.assertEqual(report['status'], 'protection-checks-passed')
                self.assertIs(report['executable'], False)
                self.assertEqual(report['revision'], candidate['revision'])
                self.assertIn('current-principal-purpose-capability-and-policy-enforcement', report['remaining'])
                self.assertEqual(report['requiredInputs'], ['secret_choice'] if name == 'root-input' else [])
                body = candidate['body']
                self.assertEqual(set(report['coveredNodes']) | set(report['uncoveredNodes']), set(body['nodes']))
                sites = set(body['nodes']) | set(body['scopes']) | set(body['data'])
                self.assertEqual(bool(report['accessObligations']), name != 'terminal')
                for access in report['accessObligations']:
                    self.assertIn(access['site'], sites)
                    self.assertIn(access['cell'], body['data'])
                    self.assertIn(access['operation'], ['read', 'write', 'transfer'])
                formatted = run('format', source, companion, resources)
                self.assertEqual(formatted.returncode, 0, formatted.stderr)
                package = json.loads(formatted.stdout)
                source.write_text(package['source'], encoding='utf-8')
                companion.write_text(package['companion'], encoding='utf-8')
                self.assertEqual(json.loads(run('lower', source, companion, resources).stdout), candidate)
                self.assertEqual(json.loads(run('check-protection', source, companion, resources).stdout), report)

    def test_control_success_is_not_protection_success(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
            for text, bindings, code in [
                (*branch_input(True), 'control-information-flow'),
                ((EXAMPLES / 'control.choreo').read_text().replace('Participants ["Reviewer"].', 'Participants empty.'),
                 (EXAMPLES / 'control.bindings.json').read_text(), 'access-context-empty'),
            ]:
                with self.subTest(code=code):
                    source.write_text(text, encoding='utf-8')
                    companion.write_text(bindings, encoding='utf-8')
                    grammar.recognize(text)
                    checked = run('check-control', source, companion)
                    self.assertEqual(checked.returncode, 0, checked.stderr)
                    refused = run('check-protection', source, companion)
                    self.assertNotEqual(refused.returncode, 0)
                    self.assertEqual(refused.stdout, b'')
                    self.assertIn(code, refused.stderr.decode())
                    self.assertNotIn('Secret choice', refused.stderr.decode())

    def test_documentation_and_licenses(self):
        for relative in ['docs/decisions/0019-protected-ordinary-transfers.md',
                         'docs/evaluation/0019-protection-plan.md',
                         'docs/evaluation/0019-protected-ordinary-transfers.md']:
            path = ROOT / relative
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))
            check_links(path)


if __name__ == '__main__':
    unittest.main(verbosity=2)
