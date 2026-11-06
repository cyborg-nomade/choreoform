# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Independent corpus catalogue/integrity checks, not a semantic evaluator."""
from collections import Counter
from hashlib import sha256
import re
import unittest

import authoring_probe as transport
from check_authoring_evidence import check_links
from check_static_contracts import ROOT

PATH = ROOT / 'crates/semantic-check/tests/conformance/value-expressions.json'
OPERATORS = {'literal', 'read', 'param', 'fact', 'field', 'record', 'list', 'variant',
             'none', 'some', 'isSome', 'unwrap', 'tag', 'payload', 'if', 'not',
             'and', 'or', 'eq', 'lt', 'add', 'sub', 'mul', 'length', 'contains',
             'index', 'rescale', 'wrap', 'unwrapNamed'}


def operations(value):
    if isinstance(value, dict):
        found = {value['op']} if 'op' in value else set()
        return found.union(*(operations(v) for v in value.values()))
    if isinstance(value, list):
        return set().union(*(operations(v) for v in value))
    return set()


class ValueConformanceCatalogue(unittest.TestCase):
    def setUp(self):
        self.corpus = transport.strict_json(PATH.read_bytes())

    def test_exact_contract_and_named_resource_pins(self):
        self.assertEqual(set(self.corpus), {'format', 'version', 'contract', 'base', 'namedResource', 'cases'})
        self.assertEqual(self.corpus['format'], 'choreoform-value-conformance')
        self.assertEqual(self.corpus['version'], '0.1.0')
        for key, identity, path in [
            ('contract', 'urn:choreoform:contracts:core-profile:0.2.0', 'docs/dialects/core-retry-v0.2.md'),
            ('base', 'urn:choreoform:contracts:core-profile:0.1.0', 'docs/ir/contracts/sha256-26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58.txt'),
            ('namedResource', 'urn:choreoform:examples:number-types:0.1.0', 'docs/authoring/examples/service.named-types.json'),
        ]:
            pin = self.corpus[key]
            self.assertEqual(set(pin), {'id', 'revision'})
            self.assertEqual(pin['id'], identity)
            self.assertEqual(pin['revision'], 'sha256:' + sha256((ROOT / path).read_bytes()).hexdigest())

    def test_case_shape_phase_catalogue_and_operator_accountability(self):
        cases = self.corpus['cases']
        self.assertEqual(len(cases), 69)
        self.assertEqual(len({c['id'] for c in cases}), len(cases))
        self.assertEqual(Counter(c['expect']['phase'] for c in cases), {'value': 42, 'static': 16, 'evaluation': 11})
        found = set()
        for case in cases:
            self.assertEqual(set(case), {'id', 'rule', 'binding', 'expression', 'expect'})
            self.assertRegex(case['id'], r'^[a-z][a-z0-9-]+$')
            self.assertRegex(case['rule'], r'^(V[12]|E[123])(?:/[A-Za-z-]+)?$')
            if case['binding'] is not None:
                self.assertEqual(set(case['binding']), {'type', 'value'})
            expected = case['expect']
            self.assertEqual(set(expected), {'phase', 'type', 'value'} if expected['phase'] == 'value'
                             else {'phase', 'code', 'type'} if expected['phase'] == 'evaluation'
                             else {'phase', 'code'})
            found |= operations(case['expression'])
        self.assertEqual(found, OPERATORS | {'notAnOperator'})
        # E1 lists every accepted operation in the normative source. This check
        # catches a missing catalogue entry; it does not prove its evaluation.
        normative = (ROOT / 'docs/dialects/values-expressions.md').read_text(encoding='utf-8')
        listed = set(re.findall(r'op:"([A-Za-z]+)"', normative))
        listed |= {'or', 'add', 'sub', 'mul'}  # quoted alternatives in table rows
        self.assertEqual(listed, OPERATORS)

    def test_documentation_and_corpus_license(self):
        self.assertIn('SPDX-License-Identifier: MPL-2.0', PATH.with_name(PATH.name + '.license').read_text())
        for name in ['docs/conformance/README.md', 'docs/evaluation/0028-validator-gap-audit.md',
                     'docs/evaluation/0028-value-conformance-plan.md', 'docs/evaluation/0028-value-conformance.md']:
            path = ROOT / name
            check_links(path)
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))


if __name__ == '__main__':
    unittest.main()
