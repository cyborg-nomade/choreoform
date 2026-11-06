# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Qualified endpoint source/schema/hash evidence, not live reconciliation."""
import json
from pathlib import Path
import tempfile
import unittest

import jsonschema
import authoring_probe as grammar
import check_ir_fixtures as wire
from check_authoring_evidence import check_links
from check_static_contracts import ROOT, EXAMPLES, run, service_resources
from check_retry_authoring import retry_package


def package(directory):
    source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
    source.write_bytes((EXAMPLES / 'reconciliation.choreo').read_bytes())
    companion.write_bytes((EXAMPLES / 'reconciliation.bindings.json').read_bytes())
    return source, companion


class QualifiedEndpointEvidence(unittest.TestCase):
    def test_owned_source_grammar_schema_revision_integrity_and_report_cycles(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion = package(directory)
            grammar.recognize(source.read_text(encoding='utf-8'))
            grammar.validate_ledger(grammar.strict_json(companion.read_bytes()))
            result = run('lower', source, companion, service_resources())
            self.assertEqual(result.returncode, 0, result.stderr)
            candidate = wire.load(result.stdout)
            jsonschema.Draft202012Validator(wire.SCHEMA).validate(candidate)
            self.assertEqual(candidate['revision'], wire.revision(candidate))
            integrity = json.loads(run('integrity', source, companion).stdout)
            self.assertEqual(integrity, grammar.package_digests(source.read_bytes(), companion.read_bytes()))
            checked = run('check-reconciliation-links', source, companion, service_resources())
            self.assertEqual(checked.returncode, 0, checked.stderr)
            report = json.loads(checked.stdout)
            self.assertIs(report['executable'], False)
            self.assertEqual(report['reconciliationUses'][0]['waitNode'], 'service_observe')
            self.assertEqual(report['reconciliationUses'][0]['followup'], 'recovery')
            self.assertIn('live-followup-entry-snapshot-availability-and-current-authority', report['reconciliationUses'][0]['hostRequirements'])
            self.assertNotIn('service_observe', report['ordinaryCoveredNodes'])
            formatted_result = run('format', source, companion, service_resources())
            self.assertEqual(formatted_result.returncode, 0, formatted_result.stderr)
            formatted = json.loads(formatted_result.stdout)
            source.write_text(formatted['source'], encoding='utf-8')
            companion.write_text(formatted['companion'], encoding='utf-8')
            self.assertEqual(json.loads(run('lower', source, companion, service_resources()).stdout), candidate)
            self.assertEqual(json.loads(run('check-reconciliation-links', source, companion, service_resources()).stdout), report)

    def test_source_03_promotion_preserves_retry_timeout_ir_but_changes_package_domain(self):
        for multiple, timeout in [(False, False), (True, False), (True, True)]:
            with self.subTest(multiple=multiple, timeout=timeout), tempfile.TemporaryDirectory() as directory:
                source, companion, resources, *_ = retry_package(directory, multiple, timeout)
                old_candidate = json.loads(run('lower', source, companion, resources).stdout)
                old_report = json.loads(run('check-timing', source, companion, resources).stdout)
                old_integrity = json.loads(run('integrity', source, companion).stdout)
                source.write_text(source.read_text(encoding='utf-8').replace('"0.3.0"', '"0.4.0"'), encoding='utf-8')
                ledger = json.loads(companion.read_bytes())
                ledger.update(profile='0.4.0', version='0.3.0')
                companion.write_text(json.dumps(ledger), encoding='utf-8')
                grammar.recognize(source.read_text(encoding='utf-8'))
                grammar.validate_ledger(ledger)
                self.assertEqual(json.loads(run('lower', source, companion, resources).stdout), old_candidate)
                self.assertEqual(json.loads(run('check-timing', source, companion, resources).stdout), old_report)
                integrity = json.loads(run('integrity', source, companion).stdout)
                self.assertNotEqual(integrity, old_integrity)
                self.assertEqual(integrity, grammar.package_digests(source.read_bytes(), companion.read_bytes()))

    def test_qualified_scope_must_be_the_actual_designated_child(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion = package(directory)
            text = source.read_text(encoding='utf-8')
            ledger = json.loads(companion.read_bytes())
            ledger['declarations'] += [
                {'id': 'other', 'kind': 'scope', 'scope': ledger['root'], 'name': 'Other'},
                {'id': 'other_done', 'kind': 'node', 'scope': 'other', 'name': 'Other done'}]
            ledger['symbols'].append({'id': 'other_complete', 'owner': 'other', 'role': 'outcome', 'name': 'complete', 'wire': 'complete'})
            other = '''Scope "Other" within "Service example".
Start at "Other done". Inputs empty. Outputs empty. Outcomes ["complete"].
Use cancellation "Stop", faults "Fault handling", closure "Recovery close" and races "Commit order".
Step "Other done". Finish as "complete". End step. End scope.
'''
            text = text.replace('Reconcile in "Recovery".', 'Reconcile in "Other".').replace('End process.', other + 'End process.')
            source.write_text(text, encoding='utf-8')
            companion.write_text(json.dumps(ledger), encoding='utf-8')
            result = run('lower', source, companion, service_resources())
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b'wait-endpoint-boundary', result.stderr)
            self.assertEqual(result.stdout, b'')
            # Moving the endpoint farther down does not export a descendant namespace.
            base = (EXAMPLES / 'reconciliation.choreo').read_text(encoding='utf-8')
            recovery = base[base.index('Scope "Recovery" within'):base.index('End process.')]
            recovery = recovery.replace('within "Service example"', 'within "Other"')
            base = base[:base.index('Scope "Recovery" within')] + 'End process.'
            text = base.replace('Reconcile in "Recovery".', 'Reconcile in "Other".').replace('in "Recovery".', 'in "Recovery" in "Other".').replace('End process.', other.replace('End scope.', recovery + 'End scope.') + 'End process.')
            next(d for d in ledger['declarations'] if d['id'] == 'recovery')['scope'] = 'other'
            source.write_text(text, encoding='utf-8')
            companion.write_text(json.dumps(ledger), encoding='utf-8')
            result = run('lower', source, companion, service_resources())
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b'reference-missing', result.stderr)

    def test_schema_pair_pins_and_unknown_profiles_refuse(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion = package(directory)
            ledger = json.loads(companion.read_bytes())
            for field, value in [('version', '0.2.0'), ('profile', '0.3.0'), ('extra', True)]:
                changed = dict(ledger, **{field: value})
                with self.assertRaises((ValueError, jsonschema.ValidationError)):
                    grammar.validate_ledger(changed)
                companion.write_text(json.dumps(changed), encoding='utf-8')
                self.assertNotEqual(run('lower', source, companion, service_resources()).returncode, 0)
            ledger['contracts']['core']['digest'] = 'sha256:' + '0' * 64
            with self.assertRaises(jsonschema.ValidationError):
                grammar.validate_ledger(ledger)
            companion.write_text(json.dumps(ledger), encoding='utf-8')
            result = run('lower', source, companion, service_resources())
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b'contract-pin', result.stderr)
            with self.assertRaises(ValueError):
                grammar.recognize(source.read_text(encoding='utf-8').replace('"0.4.0"', '"0.6.0"'))

    def test_documentation_links_and_licenses(self):
        for name in ['docs/authoring/profile-v0.4.md', 'docs/decisions/0031-qualified-wait-endpoints.md',
                     'docs/evaluation/0031-qualified-endpoint-plan.md']:
            path = ROOT / name
            check_links(path)
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))


if __name__ == '__main__':
    unittest.main()
