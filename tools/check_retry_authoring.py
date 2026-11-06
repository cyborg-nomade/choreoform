# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Versioned retry source/schema/hash cycles; no user or host observations."""
from copy import deepcopy
from hashlib import sha256
import json
from pathlib import Path
import re
import tempfile
import unittest

import jsonschema
import authoring_probe as grammar
import check_ir_fixtures as wire
from check_authoring_evidence import check_links
from check_static_contracts import ROOT, run
from check_timing import timed_package
from check_retry_contract import RETRY_ID, RETRY_HASH

CLAUSE = 'For retries use clock "Elapsed clock", basis "Audit basis". '


def retry_package(directory, multiple=True, timeout=False):
    source, companion, resources, owner, prefix, clock_revision, source_pin = timed_package(directory)
    text = source.read_text(encoding='utf-8').replace('"0.2.0"', '"0.3.0"')
    if not timeout:
        text, count = re.subn(r'Timeout clock .*?pause authority "Reviewer"\.', 'Timeout none.', text)
        assert count == 1
    if multiple:
        text = text.replace('Attempts 1.', 'Attempts 2.').replace('Delays empty.', 'Delays [0 microseconds].')
        text = text.replace('Timeout ', CLAUSE + 'Timeout ')
    source.write_text(text, encoding='utf-8')
    ledger = json.loads(companion.read_bytes())
    ledger.update({'version': '0.2.0', 'profile': '0.3.0'})
    ledger['contracts']['core'].update({'contract': RETRY_ID, 'digest': 'sha256:' + RETRY_HASH})
    companion.write_text(json.dumps(ledger), encoding='utf-8')
    return source, companion, resources, owner, prefix, clock_revision, source_pin


class RetryAuthoringEvidence(unittest.TestCase):
    def test_source_schema_revision_report_integrity_and_format_cycles(self):
        for multiple, timeout in [(False, False), (True, False), (True, True)]:
            with self.subTest(multiple=multiple, timeout=timeout), tempfile.TemporaryDirectory() as directory:
                source, companion, resources, owner, prefix, clock_revision, pin = retry_package(directory, multiple, timeout)
                text = source.read_text(encoding='utf-8')
                grammar.recognize(text)
                ledger = grammar.validate_ledger(grammar.strict_json(companion.read_bytes()))
                lowered = run('lower', source, companion, resources)
                self.assertEqual(lowered.returncode, 0, lowered.stderr)
                candidate = wire.load(lowered.stdout)
                jsonschema.Draft202012Validator(wire.SCHEMA).validate(candidate)
                self.assertEqual(candidate['revision'], wire.revision(candidate))
                r = candidate['body']['policies'][f'{prefix}_work']['body']['retry']
                self.assertEqual(r['maxAttempts'], 2 if multiple else 1)
                self.assertEqual(r['basis'], f'{prefix}_audit' if multiple else None)
                self.assertEqual(r['clock'], {'id': 'urn:test:clock', 'revision': clock_revision} if multiple else None)
                checked = run('check-timing', source, companion, resources)
                self.assertEqual(checked.returncode, 0, checked.stderr)
                report = json.loads(checked.stdout)
                self.assertIs(report['executable'], False)
                self.assertEqual(report['revision'], candidate['revision'])
                self.assertEqual(len(report['retryUses']), int(multiple))
                self.assertEqual(len(report['timerUses']), int(timeout))
                self.assertEqual(report['checkedTimingNodes'], [owner] if multiple or timeout else [])
                if multiple:
                    use = report['retryUses'][0]
                    self.assertEqual(use['clock'], r['clock'])
                    self.assertEqual(use['source'], pin)
                    self.assertEqual(use['delays'], ['0'])
                    self.assertIn('accepted-eligible-fault-attempt-correlation-and-domain-stamp', use['hostRequirements'])
                    self.assertNotIn('authorized', use)
                self.assertEqual(json.loads(run('integrity', source, companion).stdout), grammar.package_digests(source.read_bytes(), companion.read_bytes()))
                formatted = json.loads(run('format', source, companion, resources).stdout)
                source.write_text(formatted['source'], encoding='utf-8')
                companion.write_text(formatted['companion'], encoding='utf-8')
                grammar.recognize(formatted['source'])
                self.assertEqual(json.loads(companion.read_bytes()), ledger)
                self.assertEqual(json.loads(run('lower', source, companion, resources).stdout), candidate)
                self.assertEqual(json.loads(run('check-timing', source, companion, resources).stdout), report)

    def test_version_and_schema_pairs_refuse_mixed_or_unknown_contracts(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion, resources, *_ = retry_package(directory)
            ledger = json.loads(companion.read_bytes())
            for field, value in [('version', '0.1.0'), ('profile', '0.2.0'), ('extra', True)]:
                changed = deepcopy(ledger)
                changed[field] = value
                with self.assertRaises(jsonschema.ValidationError):
                    grammar.validate_ledger(changed)
                companion.write_text(json.dumps(changed), encoding='utf-8')
                self.assertNotEqual(run('lower', source, companion, resources).returncode, 0)
            changed = deepcopy(ledger)
            changed['contracts']['core']['digest'] = 'sha256:' + 'a' * 64
            with self.assertRaises(jsonschema.ValidationError):
                grammar.validate_ledger(changed)
            companion.write_text(json.dumps(changed), encoding='utf-8')
            self.assertIn(b'contract-pin', run('lower', source, companion, resources).stderr)

    def test_legacy_grammar_rejects_new_sentence_and_new_lowering_requires_binding(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion, resources, *_ = retry_package(directory)
            text = source.read_text(encoding='utf-8')
            with self.assertRaises(ValueError):
                grammar.recognize(text.replace('"0.3.0"', '"0.2.0"'))
            for changed in [text.replace(CLAUSE, ''), text.replace('Attempts 2.', 'Attempts 1.').replace('Delays [0 microseconds].', 'Delays empty.')]:
                grammar.recognize(changed)
                source.write_text(changed, encoding='utf-8')
                self.assertIn(b'retry-binding', run('lower', source, companion, resources).stderr)

    def test_independent_timeout_and_retry_clocks_keep_exact_references(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion, resources, *_ = retry_package(directory, timeout=True)
            old_clock = next(path for identity, _, path in resources if identity == 'urn:test:clock')
            clock = json.loads(old_clock.read_bytes())
            clock['id'] = 'urn:test:retry-clock'
            raw = json.dumps(clock).encode()
            pin = {'id': clock['id'], 'revision': 'sha256:' + sha256(raw).hexdigest()}
            path = Path(directory) / 'retry-clock.json'
            path.write_bytes(raw)
            resources.append((pin['id'], pin['revision'], path))
            source.write_text(source.read_text(encoding='utf-8').replace(CLAUSE, CLAUSE.replace('Elapsed clock', 'Retry clock')), encoding='utf-8')
            ledger = json.loads(companion.read_bytes())
            ledger['declarations'].append({'id': 'retry_clock_binding', 'kind': 'resource', 'scope': ledger['root'], 'name': 'Retry clock'})
            ledger['resources'].append({'binding': 'retry_clock_binding', 'kind': 'clock', 'contract': pin['id'], 'digest': pin['revision']})
            companion.write_text(json.dumps(ledger), encoding='utf-8')
            result = run('check-timing', source, companion, resources)
            self.assertEqual(result.returncode, 0, result.stderr)
            report = json.loads(result.stdout)
            self.assertEqual(report['retryUses'][0]['clock'], pin)
            self.assertEqual(report['timerUses'][0]['clock']['id'], 'urn:test:clock')

    def test_alias_edit_preserves_ir_and_all_permanent_identity_rows(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion, resources, *_ = retry_package(directory)
            before = json.loads(run('lower', source, companion, resources).stdout)
            ledger = json.loads(companion.read_bytes())
            renamed = grammar.rename_ledger(ledger, 'clock_binding', 'Time since registration')
            source.write_text(source.read_text(encoding='utf-8').replace('"Elapsed clock"', '"Time since registration"'), encoding='utf-8')
            companion.write_text(json.dumps(renamed), encoding='utf-8')
            after = run('lower', source, companion, resources)
            self.assertEqual(after.returncode, 0, after.stderr)
            self.assertEqual(json.loads(after.stdout), before)
            for key in ['generated', 'symbols', 'retired', 'resources']:
                self.assertEqual(renamed[key], ledger[key])

    def test_documents_licenses_and_only_intended_grammar_delta(self):
        old = grammar.load_grammar()
        new = grammar.load_grammar(ROOT / 'docs/authoring/candidate-a-v0.3.ebnf')
        self.assertEqual(set(new) - set(old), {'retry_binding'})
        self.assertEqual({rule for rule in old if old[rule] != new[rule]}, {'work_policy'})
        for name in ['docs/authoring/profile-v0.3.md', 'docs/decisions/0027-explicit-retry-authoring.md',
                     'docs/evaluation/0027-retry-authoring-plan.md', 'docs/evaluation/0027-explicit-retry-authoring.md']:
            path = ROOT / name
            check_links(path)
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))


if __name__ == '__main__':
    unittest.main()
