# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Artifact/schema/source-cycle evidence; not an independent semantic prover."""
import json
from pathlib import Path
import tempfile
import unittest

import jsonschema
import authoring_probe as grammar
import check_ir_fixtures as wire
from check_authoring_evidence import check_links
from check_static_contracts import ROOT, EXAMPLES, run, service_resources


class ControlEvidence(unittest.TestCase):
    def test_source_cycles_bind_reports_to_unchanged_revisions(self):
        cases = [(name, False) for name in ['terminal', 'review', 'service', 'control']]
        cases.append(('control', True))
        for name, root_input in cases:
            with self.subTest(package=name, root_input=root_input), tempfile.TemporaryDirectory() as directory:
                source = Path(directory) / 'process.choreo'
                companion = Path(directory) / 'process.bindings.json'
                source.write_bytes((EXAMPLES / f'{name}.choreo').read_bytes())
                companion.write_bytes((EXAMPLES / f'{name}.bindings.json').read_bytes())
                if root_input:
                    source.write_text(source.read_text(encoding='utf-8').replace(
                        'Inputs empty.', 'Inputs ["Starting result" uses "Result"].'), encoding='utf-8')
                    ledger = json.loads(companion.read_bytes())
                    ledger['symbols'].append({
                        'id': 'sym_branch_input', 'owner': 'scope_branch', 'role': 'input',
                        'name': 'Starting result', 'wire': 'argument',
                    })
                    companion.write_text(json.dumps(ledger), encoding='utf-8')
                resources = service_resources() if name == 'service' else []
                grammar.recognize(source.read_text(encoding='utf-8'))
                lowered = run('lower', source, companion, resources)
                self.assertEqual(lowered.returncode, 0, lowered.stderr)
                candidate = wire.load(lowered.stdout)
                jsonschema.Draft202012Validator(wire.SCHEMA).validate(candidate)
                self.assertEqual(candidate['revision'], wire.revision(candidate))
                checked = run('check-control', source, companion, resources)
                self.assertEqual(checked.returncode, 0, checked.stderr)
                report = json.loads(checked.stdout)
                self.assertEqual(report['status'], 'control-checks-passed')
                self.assertIs(report['executable'], False)
                self.assertEqual(report['revision'], candidate['revision'])
                self.assertTrue(report['remaining'])
                self.assertEqual(report['requiredInputs'], ['branch_result'] if root_input else [])
                body = candidate['body']
                if root_input:
                    self.assertEqual(body['scopes'][body['root']]['inputs'], {'argument': 'branch_result'})
                covered, uncovered = set(report['coveredNodes']), set(report['uncoveredNodes'])
                self.assertFalse(covered & uncovered)
                self.assertEqual(covered | uncovered, set(body['nodes']))
                self.assertIn(body['scopes'][body['root']]['entry'], covered)
                self.assertEqual(set(report['coveredScopes']), {body['root']})
                self.assertEqual(len(uncovered), 1 if name == 'service' else 0)
                formatted = run('format', source, companion, resources)
                self.assertEqual(formatted.returncode, 0, formatted.stderr)
                package = json.loads(formatted.stdout)
                source.write_text(package['source'], encoding='utf-8')
                companion.write_text(package['companion'], encoding='utf-8')
                self.assertEqual(json.loads(run('lower', source, companion, resources).stdout), candidate)
                self.assertEqual(json.loads(run('check-control', source, companion, resources).stdout), report)
                self.assertEqual(json.loads(run('integrity', source, companion).stdout), grammar.package_digests(source.read_bytes(), companion.read_bytes()))

    def test_static_success_does_not_imply_control_or_initialization_success(self):
        mutations = [
            ('control', 'On "second", continue with "Write second".', 'On "second", continue with "Done".', 'output-availability-unproved'),
            ('control', 'otherwise "second".', 'otherwise "second". Also read ["Result"].', 'data-availability-unproved'),
            ('review', 'Initially (0).', 'Initially (the value of "Audit basis").', 'initializer-availability-unproved'),
        ]
        for name, old, new, code in mutations:
            with self.subTest(code=code), tempfile.TemporaryDirectory() as directory:
                source = Path(directory) / 'process.choreo'
                original = (EXAMPLES / f'{name}.choreo').read_text(encoding='utf-8')
                self.assertIn(old, original)
                source.write_text(original.replace(old, new), encoding='utf-8')
                grammar.recognize(source.read_text(encoding='utf-8'))
                companion = EXAMPLES / f'{name}.bindings.json'
                self.assertEqual(run('lower', source, companion).returncode, 0)
                static = run('check', source, companion)
                self.assertEqual(static.returncode, 0, static.stderr)
                checked = run('check-control', source, companion)
                self.assertNotEqual(checked.returncode, 0)
                self.assertEqual(checked.stdout, b'')
                self.assertIn(code, checked.stderr.decode())
                self.assertNotIn('Synthetic request', checked.stderr.decode())

    def test_missing_resources_and_unknown_command_refuse(self):
        checked = run('check-control', EXAMPLES / 'service.choreo', EXAMPLES / 'service.bindings.json')
        self.assertNotEqual(checked.returncode, 0)
        self.assertEqual(checked.stdout, b'')
        checked = run('execute', EXAMPLES / 'control.choreo', EXAMPLES / 'control.bindings.json')
        self.assertNotEqual(checked.returncode, 0)
        self.assertEqual(checked.stdout, b'')

    def test_review_documents_and_licenses(self):
        for relative in ['docs/decisions/0017-control-availability-checks.md',
                         'docs/evaluation/0017-control-plan.md',
                         'docs/evaluation/0017-control-availability-checks.md']:
            path = ROOT / relative
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))
            check_links(path)
        for relative in ['control.choreo', 'control.bindings.json.license']:
            self.assertIn('SPDX-License-Identifier: MPL-2.0', (EXAMPLES / relative).read_text(encoding='utf-8'))


if __name__ == '__main__':
    unittest.main(verbosity=2)
