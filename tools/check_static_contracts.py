# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Independent artifact/hash/schema/source-cycle checks, not runtime admission."""
from hashlib import sha256
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

import jsonschema
import authoring_probe as grammar
import check_ir_fixtures as wire
from check_authoring_evidence import check_links

ROOT = Path(__file__).resolve().parents[1]
EXAMPLES = ROOT / 'docs/authoring/examples'
BINARY = ROOT / 'target/debug/choreoform-authoring-frontend'


def run(command, source, bindings, resources=()):
    args = [str(BINARY), command, str(source), str(bindings)]
    for identity, revision, path in resources:
        args += ['--resource', identity, revision, str(path)]
    return subprocess.run(args, capture_output=True, timeout=15)


def service_resources():
    result = []
    ledger = json.loads((EXAMPLES / 'service.bindings.json').read_bytes())
    for pin, filename in zip(ledger['resources'], ['service.provider.json', 'service.named-types.json']):
        path = EXAMPLES / filename
        assert pin['digest'] == 'sha256:' + sha256(path.read_bytes()).hexdigest()
        assert json.loads(path.read_bytes())['id'] == pin['contract']
        result.append((pin['contract'], pin['digest'], path))
    return result


class StaticEvidence(unittest.TestCase):
    def test_source_candidate_static_stage_and_reparse_are_distinct(self):
        for name in ['terminal', 'review', 'service']:
            with self.subTest(package=name), tempfile.TemporaryDirectory() as directory:
                source = Path(directory) / 'process.choreo'
                companion = Path(directory) / 'process.bindings.json'
                source.write_bytes((EXAMPLES / f'{name}.choreo').read_bytes())
                companion.write_bytes((EXAMPLES / f'{name}.bindings.json').read_bytes())
                resources = service_resources() if name == 'service' else []
                grammar.recognize(source.read_text(encoding='utf-8'))
                lowered = run('lower', source, companion, resources)
                self.assertEqual(lowered.returncode, 0, lowered.stderr)
                candidate = wire.load(lowered.stdout)
                jsonschema.Draft202012Validator(wire.SCHEMA).validate(candidate)
                self.assertEqual(candidate['revision'], wire.revision(candidate))
                self.assertEqual(candidate['annotations'], json.loads(companion.read_bytes())['annotations'])
                checked = run('check', source, companion, resources)
                self.assertEqual(checked.returncode, 0, checked.stderr)
                report = json.loads(checked.stdout)
                self.assertEqual(report['status'], 'static-checks-passed')
                self.assertIs(report['executable'], False)
                self.assertTrue(report['remaining'])
                self.assertEqual(report['revision'], candidate['revision'])
                formatted = run('format', source, companion, resources)
                self.assertEqual(formatted.returncode, 0, formatted.stderr)
                package = json.loads(formatted.stdout)
                source.write_text(package['source'], encoding='utf-8')
                companion.write_text(package['companion'], encoding='utf-8')
                self.assertEqual(json.loads(run('lower', source, companion, resources).stdout), candidate)
                self.assertEqual(json.loads(run('check', source, companion, resources).stdout), report)
                self.assertEqual(json.loads(run('integrity', source, companion).stdout), grammar.package_digests(source.read_bytes(), companion.read_bytes()))

    def test_lowering_does_not_implicitly_claim_static_success(self):
        source_text = (EXAMPLES / 'review.choreo').read_text(encoding='utf-8')
        for old, new, code in [
            ('Initially (0).', 'Initially (true).', 'expression-result'),
            ('with parameters empty.', 'with parameters ["x" as text].', 'parameter-binding'),
            ('Permit when (true).', 'Permit when (the fact "purpose").', 'policy-fact'),
            ('Permit when (true).', 'Permit when ("SECRET-MUST-NOT-APPEAR").', 'predicate-type'),
        ]:
            with self.subTest(code=code), tempfile.TemporaryDirectory() as directory:
                source = Path(directory) / 'process.choreo'
                source.write_text(source_text.replace(old, new), encoding='utf-8')
                grammar.recognize(source.read_text(encoding='utf-8'))
                companion = EXAMPLES / 'review.bindings.json'
                self.assertEqual(run('lower', source, companion).returncode, 0)
                checked = run('check', source, companion)
                self.assertNotEqual(checked.returncode, 0)
                self.assertEqual(checked.stdout, b'')
                self.assertIn(code, checked.stderr.decode())
                self.assertNotIn('SECRET-MUST-NOT-APPEAR', checked.stderr.decode())

    def test_external_byte_pins_and_nominal_invariants(self):
        with tempfile.TemporaryDirectory() as directory:
            source = Path(directory) / 'process.choreo'
            companion = EXAMPLES / 'service.bindings.json'
            source.write_bytes((EXAMPLES / 'service.choreo').read_bytes())
            resources = service_resources()
            self.assertNotEqual(run('lower', source, companion).returncode, 0)
            self.assertNotEqual(run('lower', source, companion, resources + resources[:1]).returncode, 0)
            provider = Path(directory) / 'provider.json'
            # Same decoded JSON, different stored bytes: the old pin must fail.
            provider.write_text(json.dumps(json.loads(resources[0][2].read_bytes())), encoding='utf-8')
            corrupted = [(resources[0][0], resources[0][1], provider), resources[1]]
            checked = run('lower', source, companion, corrupted)
            self.assertNotEqual(checked.returncode, 0)
            self.assertIn(b'resource-digest', checked.stderr)
            source.write_text(source.read_text().replace('with value 0', 'with value -1'), encoding='utf-8')
            self.assertEqual(run('lower', source, companion, resources).returncode, 0)
            checked = run('check', source, companion, resources)
            self.assertIn(b'invalid-nominal-value', checked.stderr)
            self.assertEqual(checked.stdout, b'')

    def test_unknown_provider_shapes_are_not_treated_as_opaque_empty_reads(self):
        with tempfile.TemporaryDirectory() as directory:
            provider = Path(directory) / 'provider.json'
            companion = Path(directory) / 'bindings.json'
            doc = json.loads((EXAMPLES / 'service.provider.json').read_bytes())
            doc['callback'] = 'SECRET-MUST-NOT-APPEAR'
            raw = json.dumps(doc).encode()
            provider.write_bytes(raw)
            ledger = json.loads((EXAMPLES / 'service.bindings.json').read_bytes())
            ledger['resources'][0]['digest'] = 'sha256:' + sha256(raw).hexdigest()
            companion.write_text(json.dumps(ledger), encoding='utf-8')
            resources = service_resources()
            resources[0] = (doc['id'], ledger['resources'][0]['digest'], provider)
            lowered = run('lower', EXAMPLES / 'service.choreo', companion, resources)
            self.assertNotEqual(lowered.returncode, 0)
            self.assertIn(b'fields', lowered.stderr)
            self.assertNotIn(b'SECRET-MUST-NOT-APPEAR', lowered.stderr)

    def test_clock_hashes_and_malformed_resource_arguments_cannot_enable_support(self):
        with tempfile.TemporaryDirectory() as directory:
            clock = Path(directory) / 'clock'
            clock.write_bytes(b'opaque clock bytes')
            revision = 'sha256:' + sha256(clock.read_bytes()).hexdigest()
            ledger = json.loads((EXAMPLES / 'terminal.bindings.json').read_bytes())
            ledger['declarations'].append({'id': 'clock', 'kind': 'resource', 'scope': ledger['root'], 'name': 'Clock'})
            ledger['resources'] = [{'binding': 'clock', 'kind': 'clock', 'contract': 'urn:test:clock', 'digest': revision}]
            companion = Path(directory) / 'bindings.json'
            companion.write_text(json.dumps(ledger), encoding='utf-8')
            checked = run('check', EXAMPLES / 'terminal.choreo', companion, [('urn:test:clock', revision, clock)])
            self.assertNotEqual(checked.returncode, 0)
            self.assertIn(b'resource-transport', checked.stderr)
        bad = subprocess.run([str(BINARY), 'check', str(EXAMPLES / 'review.choreo'), str(EXAMPLES / 'review.bindings.json'), '--resource'], capture_output=True, timeout=15)
        self.assertNotEqual(bad.returncode, 0)
        self.assertEqual(bad.stdout, b'')

    def test_review_documents_and_example_licenses(self):
        for path in [ROOT / 'docs/decisions/0016-static-contract-checks.md',
                     ROOT / 'docs/evaluation/0016-static-check-plan.md',
                     ROOT / 'docs/evaluation/0016-static-contract-checks.md',
                     ROOT / 'crates/semantic-check/README.md']:
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))
            check_links(path)
        for relative in ['service.choreo', 'service.bindings.json.license',
                         'service.provider.json.license', 'service.named-types.json.license']:
            self.assertIn('SPDX-License-Identifier: MPL-2.0', (EXAMPLES / relative).read_text(encoding='utf-8'))


if __name__ == '__main__':
    unittest.main(verbosity=2)
