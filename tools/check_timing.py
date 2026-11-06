# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Typed timer/source evidence; never actual clock facts, host authority or execution."""
from hashlib import sha256
import json
from pathlib import Path
import tempfile
import unittest

import jsonschema
import authoring_probe as grammar
import check_ir_fixtures as wire
from check_authoring_evidence import check_links
from check_clock_contract import PROFILE_ID, PROFILE_HASH, PROFILE_PATH
from check_static_contracts import ROOT, EXAMPLES, run, service_resources


def timed_package(directory, wait=False, private=False):
    prefix = 'service' if wait else 'review'
    text = (EXAMPLES / f'{prefix}.choreo').read_text(encoding='utf-8')
    ledger = json.loads((EXAMPLES / f'{prefix}.bindings.json').read_bytes())
    provider = json.loads((EXAMPLES / 'service.provider.json').read_bytes())
    provider.update({'id': 'urn:test:clock-source', 'input': {'kind': 'record', 'fields': {}},
                     'output': {'kind': 'integer'}, 'outcomes': {'tick': True},
                     'observations': {'tick': {'state': 'succeeded', 'result': {'kind': 'integer'}, 'outcome': 'tick'},
                                      'unknown': {'state': 'unknown', 'result': None, 'outcome': None}}})
    provider['resultProtection']['policy'] = f'{prefix}_protection'
    provider['resultProtection']['participants'] = {f'{prefix}_actor': True}
    if private:
        provider['resultProtection']['purposes'] = {'private': True}
    raw = json.dumps(provider).encode()
    source_pin = {'id': provider['id'], 'revision': 'sha256:' + sha256(raw).hexdigest()}
    provider_path = Path(directory) / 'clock-provider.json'
    provider_path.write_bytes(raw)
    clock = {'kind': 'clock', 'version': '0.1.0', 'id': 'urn:test:clock',
             'profile': {'id': PROFILE_ID, 'revision': 'sha256:' + PROFILE_HASH}, 'source': source_pin}
    raw = json.dumps(clock).encode()
    clock_revision = 'sha256:' + sha256(raw).hexdigest()
    clock_path = Path(directory) / 'clock.json'
    clock_path.write_bytes(raw)
    timer = 'clock "Elapsed clock", basis "Audit basis", due (10), calendar none, pause authority "Reviewer"'
    text = text.replace('    Also read ["Audit basis"].\n', '')
    if wait:
        text = text.replace('Timer none.', f'Timer {timer}.').replace('On timer none.', 'On timer outcome "expired" of "Observe service".')
        text = text.replace('On "observed", continue with "Done".', 'On "observed", continue with "Done".\n    On "expired", continue with "Done".')
        for outcome in ['approved', 'correction needed']:
            text = text.replace(f'On "{outcome}", continue with "Done".', f'On "{outcome}", continue with "Observe service".')
        text = text.replace('Invalidate ["Review"].', 'Invalidate ["Review", "Observe service"].').replace('Invalidate empty.', 'Invalidate ["Observe service"].')
        owner = next(d['id'] for d in ledger['declarations'] if d['kind'] == 'node' and d['name'] == 'Observe service')
        ledger['symbols'].append({'id': 'observe_expired', 'owner': owner, 'role': 'outcome', 'name': 'expired', 'wire': 'expired'})
        ledger['generated'].append({'id': 'observe_expired_flow', 'owner': owner, 'role': 'flow', 'key': 'observe_expired'})
    else:
        text = text.replace('Timeout none.', f'Timeout {timer}.')
        owner = 'review_step'
    ledger['declarations'].append({'id': 'clock_binding', 'kind': 'resource', 'scope': ledger['root'], 'name': 'Elapsed clock'})
    ledger['resources'].append({'binding': 'clock_binding', 'kind': 'clock', 'contract': clock['id'], 'digest': clock_revision})
    source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
    source.write_text(text, encoding='utf-8')
    companion.write_text(json.dumps(ledger), encoding='utf-8')
    resources = service_resources() if wait else []
    resources += [(clock['id'], clock_revision, clock_path), (PROFILE_ID, 'sha256:' + PROFILE_HASH, PROFILE_PATH),
                  (source_pin['id'], source_pin['revision'], provider_path)]
    return source, companion, resources, owner, prefix, clock_revision, source_pin


class TimingEvidence(unittest.TestCase):
    def test_work_and_wait_source_report_cycles(self):
        for wait in [False, True]:
            with self.subTest(wait=wait), tempfile.TemporaryDirectory() as directory:
                source, companion, resources, owner, prefix, clock_revision, source_pin = timed_package(directory, wait)
                grammar.recognize(source.read_text(encoding='utf-8'))
                lowered = run('lower', source, companion, resources)
                self.assertEqual(lowered.returncode, 0, lowered.stderr)
                candidate = wire.load(lowered.stdout)
                jsonschema.Draft202012Validator(wire.SCHEMA).validate(candidate)
                self.assertEqual(candidate['revision'], wire.revision(candidate))
                policy = candidate['body']['nodes'][owner]['policy']
                self.assertIn(f'{prefix}_audit', candidate['body']['policies'][policy]['body']['reads'])
                self.assertIn(f'{prefix}_audit', candidate['body']['nodes'][owner]['reads'])
                checked = run('check-timing', source, companion, resources)
                self.assertEqual(checked.returncode, 0, checked.stderr)
                report = json.loads(checked.stdout)
                self.assertEqual(report['status'], 'timing-checks-passed')
                self.assertIs(report['executable'], False)
                self.assertEqual(report['revision'], candidate['revision'])
                self.assertEqual(report['checkedTimingNodes'], [owner])
                self.assertEqual(report['uncoveredTimingNodes'], [])
                self.assertEqual(len(report['timerUses']), 1)
                use = report['timerUses'][0]
                self.assertEqual(use['site'], owner)
                self.assertEqual(use['clock'], {'id': 'urn:test:clock', 'revision': clock_revision})
                self.assertEqual(use['source'], source_pin)
                self.assertEqual(use['basis'], f'{prefix}_audit')
                self.assertEqual(use['pauseAuthority'], f'{prefix}_actor')
                self.assertEqual(use['role'], 'timer' if wait else 'timeout')
                self.assertIn('current-clock-source-result-protection-and-authority', use['hostRequirements'])
                if wait:
                    self.assertEqual(set(candidate['body']['nodes'][owner]['outcomes']), {'observed', 'expired'})
                formatted = run('format', source, companion, resources)
                self.assertEqual(formatted.returncode, 0, formatted.stderr)
                package = json.loads(formatted.stdout)
                source.write_text(package['source'], encoding='utf-8')
                companion.write_text(package['companion'], encoding='utf-8')
                self.assertEqual(json.loads(run('lower', source, companion, resources).stdout), candidate)
                self.assertEqual(json.loads(run('check-timing', source, companion, resources).stdout), report)

    def test_clock_protection_is_not_bypassed_by_typed_timer_metadata(self):
        for wait in [False, True]:
            with self.subTest(wait=wait), tempfile.TemporaryDirectory() as directory:
                source, companion, resources, *_ = timed_package(directory, wait, private=True)
                for stage in ['check', 'check-control']:
                    checked = run(stage, source, companion, resources)
                    self.assertEqual(checked.returncode, 0, checked.stderr)
                checked = run('check-timing', source, companion, resources)
                self.assertNotEqual(checked.returncode, 0)
                self.assertEqual(checked.stdout, b'')
                self.assertIn(b'control-information-flow', checked.stderr)
                self.assertNotIn(b'Synthetic request', checked.stderr)

    def test_retry_clock_is_unproved_even_with_a_work_timeout(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion, resources, *_ = timed_package(directory)
            text = source.read_text(encoding='utf-8').replace('Attempts 1.', 'Attempts 2.').replace('Delays empty.', 'Delays [0 microseconds].')
            source.write_text(text, encoding='utf-8')
            earlier = run('check-concurrency', source, companion, resources)
            self.assertEqual(earlier.returncode, 0, earlier.stderr)
            checked = run('check-timing', source, companion, resources)
            self.assertNotEqual(checked.returncode, 0)
            self.assertEqual(checked.stdout, b'')
            self.assertIn(b'retry-clock-basis-unproved', checked.stderr)

    def test_no_timer_packages_keep_empty_timing_evidence(self):
        for name in ['terminal', 'review', 'service', 'control']:
            checked = run('check-timing', EXAMPLES / f'{name}.choreo', EXAMPLES / f'{name}.bindings.json', service_resources() if name == 'service' else [])
            self.assertEqual(checked.returncode, 0, checked.stderr)
            report = json.loads(checked.stdout)
            self.assertIs(report['executable'], False)
            self.assertEqual(report['timerUses'], [])
            self.assertEqual(report['checkedTimingNodes'], [])
            self.assertEqual(report['uncoveredTimingNodes'], [])

    def test_documents_and_licenses(self):
        for relative in ['docs/decisions/0024-timer-contract-checks.md',
                         'docs/evaluation/0024-timer-integration-plan.md',
                         'docs/evaluation/0024-timer-contract-checks.md']:
            path = ROOT / relative
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))
            check_links(path)


if __name__ == '__main__':
    unittest.main(verbosity=2)
