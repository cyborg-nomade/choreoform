# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Clock publication integrity and refusal boundaries, not host clock support."""
from hashlib import sha256
import json
from pathlib import Path
import tempfile
import unittest

from check_authoring_evidence import check_links
from check_static_contracts import ROOT, EXAMPLES, run

PROFILE_ID = 'urn:choreoform:clock-profile:instance-microseconds:0.1.0'
PROFILE_HASH = '8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9'
PROFILE_PATH = ROOT / f'docs/ir/contracts/sha256-{PROFILE_HASH}.txt'


class ClockContractEvidence(unittest.TestCase):
    def test_exact_published_profile(self):
        raw = PROFILE_PATH.read_bytes()
        self.assertEqual(sha256(raw).hexdigest(), PROFILE_HASH)
        self.assertEqual(raw, (ROOT / 'docs/clocks/instance-microseconds-v0.1.md').read_bytes())
        self.assertIn(PROFILE_ID.encode(), raw)
        self.assertIn(b'SPDX-License-Identifier: CC-BY-4.0', raw)
        self.assertIn('sha256-*.txt -text', (ROOT / 'docs/ir/contracts/.gitattributes').read_text())

    def test_known_metadata_does_not_enable_source_lowering(self):
        with tempfile.TemporaryDirectory() as directory:
            source = json.loads((EXAMPLES / 'service.provider.json').read_bytes())
            source.update({'id': 'urn:test:clock-source', 'input': {'kind': 'record', 'fields': {}},
                           'output': {'kind': 'integer'}, 'outcomes': {'tick': True},
                           'observations': {'tick': {'state': 'succeeded', 'result': {'kind': 'integer'}, 'outcome': 'tick'},
                                            'unknown': {'state': 'unknown', 'result': None, 'outcome': None}}})
            source_bytes = json.dumps(source).encode()
            source_pin = {'id': source['id'], 'revision': 'sha256:' + sha256(source_bytes).hexdigest()}
            clock = {'kind': 'clock', 'version': '0.1.0', 'id': 'urn:test:clock',
                     'profile': {'id': PROFILE_ID, 'revision': 'sha256:' + PROFILE_HASH}, 'source': source_pin}
            clock_bytes = json.dumps(clock).encode()
            clock_revision = 'sha256:' + sha256(clock_bytes).hexdigest()
            source_path, clock_path, companion = (Path(directory) / name for name in ['provider.json', 'clock.json', 'bindings.json'])
            source_path.write_bytes(source_bytes)
            clock_path.write_bytes(clock_bytes)
            ledger = json.loads((EXAMPLES / 'terminal.bindings.json').read_bytes())
            ledger['declarations'].append({'id': 'clock', 'kind': 'resource', 'scope': ledger['root'], 'name': 'Clock'})
            ledger['resources'] = [{'binding': 'clock', 'kind': 'clock', 'contract': clock['id'], 'digest': clock_revision}]
            companion.write_text(json.dumps(ledger), encoding='utf-8')
            resources = [(clock['id'], clock_revision, clock_path),
                         (PROFILE_ID, 'sha256:' + PROFILE_HASH, PROFILE_PATH),
                         (source_pin['id'], source_pin['revision'], source_path)]
            result = run('check', EXAMPLES / 'terminal.choreo', companion, resources)
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(result.stdout, b'')
            self.assertIn(b'clock-contract-unsupported', result.stderr)

    def test_documents_and_licenses(self):
        for relative in ['docs/decisions/0023-clock-resource-contract.md',
                         'docs/clocks/README.md', 'docs/clocks/instance-microseconds-v0.1.md',
                         'docs/evaluation/0023-clock-contract-plan.md',
                         'docs/evaluation/0023-clock-resource-contract.md']:
            path = ROOT / relative
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))
            check_links(path)


if __name__ == '__main__':
    unittest.main(verbosity=2)
