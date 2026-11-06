# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Calendar publication/source evidence, not live binding or clock observations."""
from hashlib import sha256
import json
from pathlib import Path
import tempfile
import unittest

import jsonschema
import authoring_probe as grammar
import check_ir_fixtures as wire
from check_authoring_evidence import check_links
from check_static_contracts import ROOT, run
from check_timing import timed_package

CALENDAR_ID = 'urn:choreoform:calendar-profile:instance-intervals:0.1.0'
CALENDAR_HASH = '2ab8a6a78f2829ababa2d023f6743ac82d7a31acbfca1fb7a03340b7c929f35f'
CALENDAR_PATH = ROOT / f'docs/ir/contracts/sha256-{CALENDAR_HASH}.txt'


def calendar_package(directory, wait=False, empty=False, wrong_clock=False, opaque=False):
    source, companion, resources, owner, prefix, clock_revision, _ = timed_package(directory, wait)
    clock = {'id': 'urn:test:clock', 'revision': clock_revision}
    if wrong_clock:
        clock_path = next(path for identity, _, path in resources if identity == clock['id'])
        other = json.loads(clock_path.read_bytes())
        other['id'] = 'urn:test:other-clock'
        raw = json.dumps(other).encode()
        path = Path(directory) / 'other-clock.json'
        path.write_bytes(raw)
        clock = {'id': other['id'], 'revision': 'sha256:' + sha256(raw).hexdigest()}
        resources.append((clock['id'], clock['revision'], path))
    rules = {'kind': 'calendar-rules', 'version': '0.1.0', 'id': 'urn:test:rules',
             'profile': {'id': CALENDAR_ID, 'revision': 'sha256:' + CALENDAR_HASH}, 'clock': clock}
    raw = b'opaque provenance bytes' if opaque else json.dumps(rules).encode()
    rules_pin = {'id': rules['id'], 'revision': 'sha256:' + sha256(raw).hexdigest()}
    rules_path = Path(directory) / 'rules.json'
    rules_path.write_bytes(raw)
    calendar = {'kind': 'calendar', 'version': '0.1.0', 'id': 'urn:test:calendar',
                'clock': clock, 'zone': 'instance-relative', 'zoneRules': rules_pin,
                'intervals': [] if empty else [{'start': '0', 'end': '100'}, {'start': '200', 'end': '300'}]}
    raw = json.dumps(calendar).encode()
    calendar_pin = {'id': calendar['id'], 'revision': 'sha256:' + sha256(raw).hexdigest()}
    path = Path(directory) / 'calendar.json'
    path.write_bytes(raw)
    source.write_text(source.read_text(encoding='utf-8').replace('calendar none', 'calendar "Relative windows"'), encoding='utf-8')
    ledger = json.loads(companion.read_bytes())
    ledger['declarations'].append({'id': 'calendar_binding', 'kind': 'resource', 'scope': ledger['root'], 'name': 'Relative windows'})
    ledger['resources'].append({'binding': 'calendar_binding', 'kind': 'calendar', 'contract': calendar_pin['id'], 'digest': calendar_pin['revision']})
    companion.write_text(json.dumps(ledger), encoding='utf-8')
    resources += [(rules_pin['id'], rules_pin['revision'], rules_path), (calendar_pin['id'], calendar_pin['revision'], path),
                  (CALENDAR_ID, 'sha256:' + CALENDAR_HASH, CALENDAR_PATH)]
    return source, companion, resources, owner, prefix, calendar_pin, rules_pin


class InstanceCalendarEvidence(unittest.TestCase):
    def test_exact_publication(self):
        raw = CALENDAR_PATH.read_bytes()
        self.assertEqual(sha256(raw).hexdigest(), CALENDAR_HASH)
        self.assertEqual(raw, (ROOT / 'docs/clocks/instance-calendar-v0.1.md').read_bytes())
        self.assertIn(CALENDAR_ID.encode(), raw)

    def test_work_wait_and_empty_calendar_source_cycles(self):
        for wait, empty in [(False, False), (True, False), (False, True)]:
            with self.subTest(wait=wait, empty=empty), tempfile.TemporaryDirectory() as directory:
                source, companion, resources, owner, prefix, pin, rules = calendar_package(directory, wait, empty)
                grammar.recognize(source.read_text(encoding='utf-8'))
                lowered = run('lower', source, companion, resources)
                self.assertEqual(lowered.returncode, 0, lowered.stderr)
                candidate = wire.load(lowered.stdout)
                jsonschema.Draft202012Validator(wire.SCHEMA).validate(candidate)
                self.assertEqual(candidate['revision'], wire.revision(candidate))
                checked = run('check-timing', source, companion, resources)
                self.assertEqual(checked.returncode, 0, checked.stderr)
                report = json.loads(checked.stdout)
                self.assertIs(report['executable'], False)
                self.assertEqual(report['revision'], candidate['revision'])
                self.assertEqual(report['checkedTimingNodes'], [owner])
                use = report['timerUses'][0]['calendar']
                self.assertEqual({key: use[key] for key in ['id', 'revision']}, pin)
                self.assertEqual(use['rules'], rules)
                self.assertEqual(use['profile'], {'id': CALENDAR_ID, 'revision': 'sha256:' + CALENDAR_HASH})
                self.assertIn('trusted-host-calendar-construction-approval', use['hostRequirements'])
                self.assertIn('exact-calendar-rules-profile-clock-and-instance-origin-binding', use['hostRequirements'])
                self.assertNotIn('originEvent', use)
                self.assertNotIn('approved', use)
                formatted = run('format', source, companion, resources)
                self.assertEqual(formatted.returncode, 0, formatted.stderr)
                package = json.loads(formatted.stdout)
                source.write_text(package['source'], encoding='utf-8')
                companion.write_text(package['companion'], encoding='utf-8')
                self.assertEqual(json.loads(run('lower', source, companion, resources).stdout), candidate)
                self.assertEqual(json.loads(run('check-timing', source, companion, resources).stdout), report)

    def test_unknown_construction_and_mismatched_timer_clock_refuse(self):
        for wrong_clock, opaque, code in [(True, False, b'calendar-clock'), (False, True, b'resource-transport')]:
            with self.subTest(code=code), tempfile.TemporaryDirectory() as directory:
                source, companion, resources, *_ = calendar_package(directory, wrong_clock=wrong_clock, opaque=opaque)
                checked = run('check-timing', source, companion, resources)
                self.assertNotEqual(checked.returncode, 0)
                self.assertEqual(checked.stdout, b'')
                self.assertIn(code, checked.stderr)
                self.assertNotIn(b'Synthetic request', checked.stderr)

    def test_documents_and_licenses(self):
        for relative in ['docs/decisions/0025-instance-calendars.md',
                         'docs/clocks/instance-calendar-v0.1.md',
                         'docs/evaluation/0025-calendar-plan.md',
                         'docs/evaluation/0025-instance-calendars.md']:
            path = ROOT / relative
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))
            check_links(path)


if __name__ == '__main__':
    unittest.main(verbosity=2)
