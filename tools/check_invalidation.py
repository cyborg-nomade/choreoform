# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Source/artifact evidence; no runtime withdrawal, cancellation or race proof."""
import json
from pathlib import Path
import tempfile
import unittest

import jsonschema
import check_ir_fixtures as wire
from check_authoring_evidence import check_links
from check_static_contracts import ROOT, EXAMPLES, run, service_resources


class InvalidationEvidence(unittest.TestCase):
    def test_reports_and_source_cycles(self):
        for name in ['terminal', 'review', 'service', 'control']:
            with self.subTest(package=name), tempfile.TemporaryDirectory() as directory:
                source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
                source.write_bytes((EXAMPLES / f'{name}.choreo').read_bytes())
                companion.write_bytes((EXAMPLES / f'{name}.bindings.json').read_bytes())
                resources = service_resources() if name == 'service' else []
                lowered = run('lower', source, companion, resources)
                self.assertEqual(lowered.returncode, 0, lowered.stderr)
                candidate = wire.load(lowered.stdout)
                jsonschema.Draft202012Validator(wire.SCHEMA).validate(candidate)
                self.assertEqual(candidate['revision'], wire.revision(candidate))
                checked = run('check-invalidation', source, companion, resources)
                self.assertEqual(checked.returncode, 0, checked.stderr)
                report = json.loads(checked.stdout)
                self.assertEqual(report['status'], 'invalidation-checks-passed')
                self.assertIs(report['executable'], False)
                self.assertEqual(report['revision'], candidate['revision'])
                self.assertIn('concurrency-and-irreversible-race-safety-proofs', report['remaining'])
                consequential = {key for key, n in candidate['body']['nodes'].items() if n['kind'] in ['activity', 'decision', 'wait']}
                self.assertEqual(set(report['checkedInvalidationNodes']) | set(report['uncoveredInvalidationNodes']), consequential)
                self.assertFalse(set(report['checkedInvalidationNodes']) & set(report['uncoveredInvalidationNodes']))
                for obligation in report['invalidationObligations']:
                    self.assertIn(obligation['site'], candidate['body']['data'][obligation['cell']]['invalidates'])
                if name in ['review', 'service']:
                    self.assertEqual(len(report['invalidationObligations']), 2)
                    self.assertNotIn(f'{name}_decision', {o['cell'] for o in report['invalidationObligations']})
                if name == 'service':
                    waits = {key for key, n in candidate['body']['nodes'].items() if n['kind'] == 'wait'}
                    self.assertEqual(set(report['uncoveredInvalidationNodes']), waits)
                formatted = run('format', source, companion, resources)
                self.assertEqual(formatted.returncode, 0, formatted.stderr)
                package = json.loads(formatted.stdout)
                source.write_text(package['source'], encoding='utf-8')
                companion.write_text(package['companion'], encoding='utf-8')
                self.assertEqual(json.loads(run('lower', source, companion, resources).stdout), candidate)
                self.assertEqual(json.loads(run('check-invalidation', source, companion, resources).stdout), report)

    def test_missing_links_pass_earlier_stages_then_refuse(self):
        with tempfile.TemporaryDirectory() as directory:
            source = Path(directory) / 'source.choreo'
            source.write_text((EXAMPLES / 'review.choreo').read_text(encoding='utf-8').replace('Invalidate ["Review"].', 'Invalidate empty.'), encoding='utf-8')
            companion = EXAMPLES / 'review.bindings.json'
            for stage in ['check', 'check-control', 'check-protection']:
                result = run(stage, source, companion)
                self.assertEqual(result.returncode, 0, result.stderr)
            result = run('check-invalidation', source, companion)
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(result.stdout, b'')
            self.assertIn(b'invalidation-coverage at /body/data/', result.stderr)
            self.assertNotIn(b'Synthetic request', result.stderr)

    def test_wait_after_work_requires_prior_result_and_control_dependencies(self):
        with tempfile.TemporaryDirectory() as directory:
            source = Path(directory) / 'source.choreo'
            text = (EXAMPLES / 'service.choreo').read_text(encoding='utf-8')
            for outcome in ['approved', 'correction needed']:
                text = text.replace(f'On "{outcome}", continue with "Done".', f'On "{outcome}", continue with "Observe service".')
            text = text.replace('Invalidate ["Review"].', 'Invalidate ["Review", "Observe service"].')
            text = text.replace('Invalidate empty.', 'Invalidate ["Observe service"].')
            source.write_text(text, encoding='utf-8')
            companion = EXAMPLES / 'service.bindings.json'
            result = run('check-invalidation', source, companion, service_resources())
            self.assertEqual(result.returncode, 0, result.stderr)
            report = json.loads(result.stdout)
            self.assertEqual(len(report['invalidationObligations']), 5)
            self.assertEqual(report['uncoveredInvalidationNodes'], [])
            self.assertTrue(any(o['cell'] == 'service_decision' for o in report['invalidationObligations']))
            self.assertIn('exceptional-and-non-data-invalidation-and-lifecycle-enforcement', report['remaining'])

    def test_documents_and_licenses(self):
        for relative in ['docs/decisions/0021-ordinary-invalidation.md',
                         'docs/evaluation/0021-invalidation-plan.md',
                         'docs/evaluation/0021-ordinary-invalidation.md']:
            path = ROOT / relative
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))
            check_links(path)


if __name__ == '__main__':
    unittest.main(verbosity=2)
