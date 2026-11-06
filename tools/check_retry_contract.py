# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Retry publication and legacy source boundaries, not retry execution evidence."""
from hashlib import sha256
import json
import re
import tempfile
import unittest

from check_authoring_evidence import check_links
from check_static_contracts import ROOT, run
from check_timing import timed_package

RETRY_ID = 'urn:choreoform:contracts:core-profile:0.2.0'
RETRY_HASH = '5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623'
RETRY_PATH = ROOT / f'docs/ir/contracts/sha256-{RETRY_HASH}.txt'


class RetryContractEvidence(unittest.TestCase):
    def test_publication_preserves_base_contract(self):
        raw = RETRY_PATH.read_bytes()
        self.assertEqual(sha256(raw).hexdigest(), RETRY_HASH)
        self.assertEqual(raw, (ROOT / 'docs/dialects/core-retry-v0.2.md').read_bytes())
        self.assertIn(RETRY_ID.encode(), raw)
        base = '26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58'
        self.assertEqual(sha256((ROOT / f'docs/ir/contracts/sha256-{base}.txt').read_bytes()).hexdigest(), base)
        self.assertIn(base.encode(), raw)

    def test_old_source_keeps_unbound_retry_refusal_and_no_new_syntax(self):
        with tempfile.TemporaryDirectory() as directory:
            source, companion, resources, *_ = timed_package(directory)
            original = source.read_text(encoding='utf-8')
            result = run('check-timing', source, companion, resources)
            self.assertEqual(result.returncode, 0, result.stderr)
            report = json.loads(result.stdout)
            self.assertEqual(report['retryUses'], [])
            self.assertIs(report['executable'], False)
            changed, count = re.subn(r'Attempts 1\.\s+Delays empty\.', 'Attempts 2. Delays [0 microseconds].', original)
            self.assertEqual(count, 1)
            source.write_text(changed, encoding='utf-8')
            result = run('check-timing', source, companion, resources)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b'retry-clock-basis-unproved', result.stderr)
            source.write_text(original.replace('Timeout ', 'For retries use clock "Elapsed clock", basis "Audit basis". Timeout '), encoding='utf-8')
            result = run('lower', source, companion, resources)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b'syntax', result.stderr)

    def test_documentation_links_and_licenses(self):
        for name in ['docs/decisions/0026-explicit-retry-clock.md',
                     'docs/evaluation/0026-retry-clock-plan.md',
                     'docs/evaluation/0026-explicit-retry-clock.md']:
            path = ROOT / name
            check_links(path)
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))


if __name__ == '__main__':
    unittest.main()
