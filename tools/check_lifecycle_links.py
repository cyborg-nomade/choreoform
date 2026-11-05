# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Lifecycle definition-link evidence, not live ownership or runtime settlement."""
import json
from pathlib import Path
import tempfile
import unittest

import jsonschema
import authoring_probe as grammar
import check_ir_fixtures as wire
from check_authoring_evidence import check_links
from check_static_contracts import ROOT, EXAMPLES, run, service_resources


class LifecycleLinks(unittest.TestCase):
    def test_service_reconciliation_site_survives_source_cycles(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
            source.write_bytes((EXAMPLES / 'service.choreo').read_bytes())
            companion.write_bytes((EXAMPLES / 'service.bindings.json').read_bytes())
            resources = service_resources()
            candidate = wire.load(run('lower', source, companion, resources).stdout)
            jsonschema.Draft202012Validator(wire.SCHEMA).validate(candidate)
            self.assertEqual(candidate['revision'], wire.revision(candidate))
            owners = {key for key, node in candidate['body']['nodes'].items() if node['kind'] == 'wait'}
            self.assertEqual(len(owners), 1)
            reports = {}
            for stage in ['check', 'check-control', 'check-protection']:
                checked = run(stage, source, companion, resources)
                self.assertEqual(checked.returncode, 0, checked.stderr)
                reports[stage] = json.loads(checked.stdout)
                self.assertIs(reports[stage]['executable'], False)
                self.assertEqual(reports[stage]['revision'], candidate['revision'])
                if stage != 'check':
                    self.assertTrue(owners.issubset(reports[stage]['uncoveredNodes']))
            formatted = run('format', source, companion, resources)
            self.assertEqual(formatted.returncode, 0, formatted.stderr)
            package = json.loads(formatted.stdout)
            source.write_text(package['source'], encoding='utf-8')
            companion.write_text(package['companion'], encoding='utf-8')
            self.assertEqual(json.loads(run('lower', source, companion, resources).stdout), candidate)
            for stage, report in reports.items():
                self.assertEqual(json.loads(run(stage, source, companion, resources).stdout), report)

    def test_named_wait_policy_without_a_wait_site_does_not_suffice(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
            text = (EXAMPLES / 'service.choreo').read_text(encoding='utf-8').replace(
                'Wait under "Service reconciliation".',
                'Set together ["Request" to ("Reconciliation owner placeholder")].')
            ledger = json.loads((EXAMPLES / 'service.bindings.json').read_bytes())
            owner = next(d['id'] for d in ledger['declarations'] if d['kind'] == 'node' and d['name'] == 'Observe service')
            ledger['generated'].append({'id': 'observer_assignment', 'owner': owner, 'role': 'assignment', 'key': 'service_request'})
            source.write_text(text, encoding='utf-8')
            companion.write_text(json.dumps(ledger), encoding='utf-8')
            grammar.recognize(text)
            resources = service_resources()
            lowered = run('lower', source, companion, resources)
            self.assertEqual(lowered.returncode, 0, lowered.stderr)
            self.assertEqual(json.loads(lowered.stdout)['body']['nodes'][owner]['kind'], 'compute')
            for stage in ['check', 'check-control', 'check-protection']:
                checked = run(stage, source, companion, resources)
                self.assertNotEqual(checked.returncode, 0)
                self.assertEqual(checked.stdout, b'')
                self.assertIn(b'reconciliation-owner-missing', checked.stderr)
                self.assertNotIn(b'Reconciliation owner placeholder', checked.stderr)

    def test_documents_and_licenses(self):
        for relative in ['docs/decisions/0020-lifecycle-policy-links.md',
                         'docs/evaluation/0020-lifecycle-link-plan.md',
                         'docs/evaluation/0020-lifecycle-policy-links.md']:
            path = ROOT / relative
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))
            check_links(path)


if __name__ == '__main__':
    unittest.main(verbosity=2)
