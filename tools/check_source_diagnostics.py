# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Original UTF-8 declaration locations, not field precision or admission."""
import json
import re
from pathlib import Path
import tempfile
import unittest

from check_authoring_evidence import check_links
from check_static_contracts import ROOT, EXAMPLES, run, service_resources
from check_qualified_endpoints import package


def location(test, result, source, code, path, starts):
    test.assertNotEqual(result.returncode, 0)
    test.assertEqual(result.stdout, b'')
    prefix = f'{code} at {path}; source bytes '
    message = result.stderr.decode('utf-8')
    test.assertTrue(message.startswith(prefix), message)
    match = re.fullmatch(re.escape(prefix) + r'(\d+)\.\.(\d+)\n', message)
    test.assertIsNotNone(match, message)
    start, end = map(int, match.groups())
    raw = source.read_bytes()
    test.assertTrue(0 <= start < end <= len(raw))
    snippet = raw[start:end].decode('utf-8')
    test.assertTrue(snippet.startswith(starts), snippet)
    return start, end, snippet


class SourceDiagnosticEvidence(unittest.TestCase):
    def test_static_control_and_utf8_source_locations_preserve_code_and_path(self):
        for control in [False, True]:
            with self.subTest(control=control), tempfile.TemporaryDirectory() as directory:
                source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
                text = '# é 日本語\n' + (EXAMPLES / 'review.choreo').read_text(encoding='utf-8')
                ledger = json.loads((EXAMPLES / 'review.bindings.json').read_bytes())
                if control:
                    text = text.replace('Initially ("Synthetic request").', 'Initially unavailable.')
                    ledger['generated'] = [r for r in ledger['generated'] if not (r['role'] == 'initial' and r['owner'] == 'review_request')]
                else:
                    text = text.replace('Complete when (true).', 'Complete when (7).')
                source.write_text(text, encoding='utf-8')
                companion.write_text(json.dumps(ledger), encoding='utf-8')
                code, path, starts = ('data-availability-unproved', '/body/nodes/review_step', 'Step "Review".') if control else ('predicate-type', '/body/policies/review_work', 'Rule for work "Review work".')
                result = run('check-control' if control else 'check', source, companion)
                start, _, _ = location(self, result, source, code, path, starts)
                self.assertEqual(start, source.read_bytes().index(starts.encode()))
                self.assertNotIn(b'Synthetic request', result.stderr)

    def test_lifecycle_and_followup_refusals_locate_scopes_and_steps(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion = package(directory)
            text = source.read_text(encoding='utf-8').replace('Match when ((the fact "payload") equals (the fact "payload")).', 'Match when ((the value of "Request") equals (the value of "Request")).')
            source.write_text(text, encoding='utf-8')
            result = run('check-reconciliation-availability', source, companion, service_resources())
            location(self, result, source, 'reconciliation-availability-unproved', '/body/nodes/service_observe', 'Step "Observe service".')
        source, companion = EXAMPLES / 'service.choreo', EXAMPLES / 'service.bindings.json'
        result = run('check-reconciliation-links', source, companion, service_resources())
        location(self, result, source, 'reconciliation-followup-unproved', '/body/scopes/scope_service/closure', 'Process "Service example".')

    def test_generated_expression_error_locates_assignment_without_echoing_value(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
            text = (EXAMPLES / 'control.choreo').read_text(encoding='utf-8').replace('"Result" to (1)', '"Result" to ("PRIVATE TEST MARKER")')
            source.write_text(text, encoding='utf-8')
            companion.write_bytes((EXAMPLES / 'control.bindings.json').read_bytes())
            ledger = json.loads(companion.read_bytes())
            owner = next(r['id'] for r in ledger['declarations'] if r['name'] == 'Write first')
            expr = next(r['id'] for r in ledger['generated'] if r['role'] == 'assignment' and r['owner'] == owner)
            result = run('check', source, companion)
            location(self, result, source, 'expression-result', f'/body/expressions/{expr}', '"Result" to (')
            self.assertNotIn(b'PRIVATE TEST MARKER', result.stderr)

    def test_reparse_after_formatting_recomputes_locations_for_the_same_ir_error(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
            source.write_text('# é prefix\n' + (EXAMPLES / 'review.choreo').read_text(encoding='utf-8').replace('Complete when (true).', 'Complete when (7).'), encoding='utf-8')
            companion.write_bytes((EXAMPLES / 'review.bindings.json').read_bytes())
            first = location(self, run('check', source, companion), source, 'predicate-type', '/body/policies/review_work', 'Rule for work "Review work".')
            formatted = json.loads(run('format', source, companion).stdout)
            source.write_text(formatted['source'], encoding='utf-8')
            companion.write_text(formatted['companion'], encoding='utf-8')
            second = location(self, run('check', source, companion), source, 'predicate-type', '/body/policies/review_work', 'Rule for work "Review work" .')
            self.assertNotEqual(first[:2], second[:2])

    def test_docs_and_licenses(self):
        for name in ['docs/authoring/diagnostics.md', 'docs/decisions/0033-semantic-source-locations.md',
                     'docs/evaluation/0033-source-diagnostics-plan.md']:
            path = ROOT / name
            check_links(path)
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))


if __name__ == '__main__':
    unittest.main()
