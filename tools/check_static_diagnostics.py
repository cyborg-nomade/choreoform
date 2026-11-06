# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Located static refusals preserve codes, not admission or token precision."""
import json
from pathlib import Path
import tempfile
import unittest

from check_authoring_evidence import check_links
from check_source_diagnostics import location
from check_static_contracts import ROOT, EXAMPLES, run, service_resources
from check_retry_contract import RETRY_ID, RETRY_HASH


class StaticDiagnosticEvidence(unittest.TestCase):
    def test_initialized_input_has_its_scope_port_path_and_original_source_span(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
            text = (EXAMPLES / 'review.choreo').read_text(encoding='utf-8').replace('Inputs empty.', 'Inputs ["Launch request" uses "Request"].', 1)
            ledger = json.loads((EXAMPLES / 'review.bindings.json').read_bytes())
            ledger['symbols'].append({'id': 'launch_request', 'owner': ledger['root'], 'role': 'input', 'name': 'Launch request', 'wire': 'launch_request'})
            source.write_text(text, encoding='utf-8')
            companion.write_text(json.dumps(ledger), encoding='utf-8')
            result = run('check', source, companion)
            location(self, result, source, 'input-initializer', '/body/scopes/scope_review/inputs/launch_request', 'Process "Review example".')
            self.assertNotIn(b'Synthetic request', result.stderr)

    def test_capability_input_and_output_paths_work_in_all_source_profiles(self):
        for profile, version in [('0.2.0', '0.1.0'), ('0.3.0', '0.2.0'), ('0.4.0', '0.3.0')]:
            for field in ['input', 'output']:
                with self.subTest(profile=profile, field=field), tempfile.TemporaryDirectory() as directory:
                    source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
                    text = '# é 日本語\n' + (EXAMPLES / 'service.choreo').read_text(encoding='utf-8')
                    ledger = json.loads((EXAMPLES / 'service.bindings.json').read_bytes())
                    if field == 'input':
                        text = text.replace('with input text', 'with input whole number')
                    else:
                        # This clause occurs only in the capability declaration.
                        start = text.index('  Use capability "Decision service"')
                        end = text.index('\n', start)
                        clause = text[start:end]
                        prefix = clause[:clause.index(' and output ')]
                        suffix = clause[clause.index(', authority '):]
                        text = text[:start] + prefix + ' and output whole number' + suffix + text[end:]
                    text = text.replace('"0.2.0"', f'"{profile}"')
                    ledger.update(profile=profile, version=version)
                    if profile != '0.2.0':
                        ledger['contracts']['core'].update(contract=RETRY_ID, digest='sha256:' + RETRY_HASH)
                    source.write_text(text, encoding='utf-8')
                    companion.write_text(json.dumps(ledger), encoding='utf-8')
                    result = run('check', source, companion, service_resources())
                    location(self, result, source, 'capability-type', f'/body/capabilities/service_capability/{field}', 'Use capability "Decision service"')

    def test_documentation_and_license(self):
        path = ROOT / 'docs/evaluation/0033-static-paths-plan.md'
        check_links(path)
        self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))


if __name__ == '__main__':
    unittest.main()
