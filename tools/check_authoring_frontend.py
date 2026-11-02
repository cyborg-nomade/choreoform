# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Differential Rust syntax/package evidence, not executable admission."""

from pathlib import Path
import json
import subprocess
import tempfile
import unittest

import jsonschema

import authoring_probe as oracle
from check_authoring_profile import CASES, SOURCE, RAW_LEDGER
import check_ir_fixtures as wire
from check_authoring_evidence import check_links

BINARY = oracle.ROOT / 'target/debug/choreoform-authoring-frontend'


def run(*args, raw=b''):
    return subprocess.run([str(BINARY), *args], input=raw, capture_output=True, timeout=10)


class FrontendEvidence(unittest.TestCase):
    def test_differential_positive_grammar_catalogue(self):
        count = 0
        for rule, sources in CASES.items():
            for source in sources:
                with self.subTest(rule=rule, source=source):
                    oracle.recognize(source, rule)
                    result = run('parse-fragment', rule, raw=source.encode())
                    self.assertEqual(result.returncode, 0, result.stderr.decode())
                    count += 1
        self.assertEqual(count, 106)

    def test_differential_lexical_and_structural_refusals(self):
        for source, rule in [('true trailing', 'expression'), ('true and false', 'expression'),
                              ('01', 'literal'), ('-0', 'literal'), ('1.0', 'literal'), ('1with', 'literal'),
                              ('"a\\n"', 'literal'), ('"a\nb"', 'literal'), ('"\u2028"', 'literal'),
                              ('Set together empty.', 'compute'), ('any report empty', 'join_condition'),
                              (SOURCE.replace('"0.2.0"', '"0.3.0"'), 'program')]:
            with self.subTest(source=source), self.assertRaises(ValueError):
                oracle.recognize(source, rule)
            self.assertNotEqual(run('parse-fragment', rule, raw=source.encode()).returncode, 0)

    def test_package_hash_shape_revision_and_reparse(self):
        for relative in ('crates/authoring-frontend/README.md',
                         'docs/evaluation/0015-rust-authoring-frontend.md',
                         'docs/evaluation/0015-frontend-plan.md'):
            path = oracle.ROOT / relative
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))
            check_links(path)
        for relative in ('docs/authoring/examples/review.choreo',
                         'docs/authoring/examples/review.bindings.json.license'):
            self.assertIn('SPDX-License-Identifier: MPL-2.0',
                          (oracle.ROOT / relative).read_text(encoding='utf-8'))
        self.check_package(SOURCE.encode(), RAW_LEDGER)
        self.check_package((oracle.ROOT / 'docs/authoring/examples/review.choreo').read_bytes(),
                           (oracle.ROOT / 'docs/authoring/examples/review.bindings.json').read_bytes())

    def check_package(self, raw_source, raw_ledger):
        with tempfile.TemporaryDirectory() as directory:
            source = Path(directory) / 'source.choreo'
            companion = Path(directory) / 'source.bindings.json'
            source.write_bytes(raw_source)
            companion.write_bytes(raw_ledger)
            integrity = run('integrity', str(source), str(companion))
            self.assertEqual(integrity.returncode, 0, integrity.stderr)
            self.assertEqual(json.loads(integrity.stdout), oracle.package_digests(source.read_bytes(), companion.read_bytes()))
            result = run('lower', str(source), str(companion))
            self.assertEqual(result.returncode, 0, result.stderr)
            candidate = wire.load(result.stdout)
            jsonschema.Draft202012Validator(wire.SCHEMA).validate(candidate)
            self.assertEqual(candidate['revision'], wire.revision(candidate))
            self.assertEqual(candidate['annotations'], json.loads(raw_ledger)['annotations'])
            formatted = run('format', str(source), str(companion))
            self.assertEqual(formatted.returncode, 0, formatted.stderr)
            package = json.loads(formatted.stdout)
            source.write_text(package['source'], encoding='utf-8')
            companion.write_text(package['companion'], encoding='utf-8')
            oracle.recognize(package['source'])
            again = run('lower', str(source), str(companion))
            self.assertEqual(again.returncode, 0, again.stderr)
            self.assertEqual(json.loads(again.stdout), candidate)
            self.assertEqual(json.loads(run('format', str(source), str(companion)).stdout), package)

    def test_invalid_ledger_and_source_inventory_are_not_auto_reconciled(self):
        with tempfile.TemporaryDirectory() as directory:
            source = Path(directory) / 'source.choreo'
            companion = Path(directory) / 'source.bindings.json'
            source.write_bytes(SOURCE.encode())
            for mutation in ['duplicate', 'missing', 'retired', 'wrong-kind', 'extra-symbol']:
                ledger = json.loads(RAW_LEDGER)
                if mutation == 'duplicate':
                    ledger['declarations'].append(ledger['declarations'][-1])
                elif mutation == 'missing':
                    ledger['declarations'].pop()
                elif mutation == 'retired':
                    ledger['retired'] = ['node_done']
                elif mutation == 'wrong-kind':
                    ledger['declarations'][-1]['kind'] = 'expression'
                else:
                    ledger['symbols'].append({'id': 'unused', 'owner': 'node_done', 'role': 'outcome',
                                              'name': 'unused', 'wire': 'unused'})
                companion.write_text(json.dumps(ledger), encoding='utf-8')
                with self.subTest(mutation=mutation):
                    result = run('lower', str(source), str(companion))
                    self.assertNotEqual(result.returncode, 0)
                    self.assertEqual(result.stdout, b'')
            companion.write_bytes(RAW_LEDGER)
            source.write_text(SOURCE.replace('"Done"', '"Different"'), encoding='utf-8')
            self.assertNotEqual(run('lower', str(source), str(companion)).returncode, 0)


if __name__ == '__main__':
    unittest.main(verbosity=2)
