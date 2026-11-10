# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Limited independent static premises, never host execution or general admission."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest

import jsonschema
import check_ir_fixtures as wire
from check_static_contracts import ROOT

DIRECTORY = ROOT / 'crates/semantic-check/tests/conformance'
CORE = ROOT / 'docs/dialects/core-owned-readonly-v0.9.md'


def model(p):
    # Only isolated mutations of the admitted two-level fixture. Resource/type /
    # visibility/capture premises remain prevalidated by the actual Rust SDK.
    if not p['captureProtection']:
        return 'information-flow'
    if not p['snapshot']:
        return 'owned-continuation-request-unproved'
    if not p['localWrite']:
        return 'owned-continuation-storage-unproved'
    if not p['generalSource']:
        return 'owned-continuation-query-policy'
    if not p['exactInput']:
        return 'owned-continuation-query-input'
    if not p['coveredSource']:
        return 'owned-continuation-source-unproved'
    if not p['recipient']:
        return 'owned-continuation-recipient'
    if not p['invalidation']:
        return 'owned-continuation-invalidation-coverage'
    return 'covered'


class OwnedReadonlyEvidence(unittest.TestCase):
    def test_exact_core_artifact_and_all_inherited_bases(self):
        raw = CORE.read_bytes()
        digest = 'sha256:' + sha256(raw).hexdigest()
        self.assertEqual((ROOT / 'docs/ir/contracts' / (digest.replace(':', '-') + '.txt')).read_bytes(), raw)
        self.assertIn(digest, (ROOT / 'crates/semantic-check/src/lib.rs').read_text())
        for name in ['core-retained-request-v0.8.md', 'core-fault-entry-v0.7.md',
                     'core-emission-source-v0.6.md', 'core-wait-capture-v0.5.md',
                     'core-fault-source-v0.4.md', 'core-metadata-v0.3.md', 'core-retry-v0.2.md']:
            self.assertIn('sha256:' + sha256((CORE.parent / name).read_bytes()).hexdigest(), raw.decode())
        self.assertIn('sha256:26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58', raw.decode())

    def test_static_fixture_wire_revision_and_every_supplied_digest(self):
        v = json.loads((DIRECTORY / 'owned-readonly-definition.json').read_bytes())
        self.assertIn('no actual events', v['purpose'])
        d = v['definition']
        self.assertEqual(d['version'], '0.4.0')
        self.assertEqual(d['revision'], wire.revision(d))
        schema = json.loads((ROOT / 'schemas/ir/definition-0.4.schema.json').read_bytes())
        jsonschema.Draft202012Validator(schema).validate(d)
        for r in v['normative'] + v['resources']:
            raw = r['bytes'].encode() if 'bytes' in r else (ROOT / 'docs/ir/contracts' / (r['revision'].replace(':', '-') + '.txt')).read_bytes()
            self.assertEqual(r['revision'], 'sha256:' + sha256(raw).hexdigest())
            if 'bytes' in r:
                self.assertEqual(json.loads(raw)['id'], r['id'])

    def test_finite_ownership_source_and_request_relations(self):
        d = json.loads((DIRECTORY / 'owned-readonly-definition.json').read_bytes())['definition']
        b = d['body']
        owner, source, seen = 'root', 'origin', set()
        while True:
            scope = b['policies'][b['scopes'][owner]['closure']]['body']['followup']
            self.assertNotIn(scope, seen)
            seen.add(scope)
            s = b['scopes'][scope]
            self.assertEqual(s['parent'], owner)
            self.assertEqual(list(s['retainedRequests'].values()), [source])
            request = next(iter(s['retainedRequests']))
            self.assertEqual(b['data'][request]['scope'], scope)
            self.assertNotIn('initial', b['data'][request])
            self.assertEqual(b['nodes'][s['entry']]['capture']['origin'], source)
            nodes = [n for n in b['nodes'].values() if n['scope'] == scope]
            self.assertTrue(all(set(n['writes']) <= {i for i, cell in b['data'].items() if cell['scope'] == scope} for n in nodes))
            closure = b['policies'][s['closure']]['body']
            if closure['mode'] == 'terminal':
                self.assertFalse(any(n['kind'] == 'activity' for n in nodes))
                break
            queries = [(i, n) for i, n in b['nodes'].items() if n['scope'] == scope and n['kind'] == 'activity']
            self.assertEqual(len(queries), 1)
            source, query = queries[0]
            cap = b['capabilities'][query['target']]
            self.assertEqual(b['policies'][cap['effects']]['body']['class'], 'read-only')
            self.assertEqual(b['expressions'][query['input']]['body'], {'op': 'read', 'cell': request})
            self.assertEqual(query['reads'], {request: True})
            owner = scope
        self.assertEqual(seen, {'recovery', 'lookup_recovery'})

    def test_shared_isolated_vectors_have_independent_limited_expectations(self):
        v = json.loads((DIRECTORY / 'owned-readonly.json').read_bytes())
        self.assertEqual(v['format'], 'choreoform-owned-readonly-vectors')
        self.assertIn('not general admission', v['premise'])
        self.assertEqual(len(v['cases']), 11)
        self.assertEqual(len({c['id'] for c in v['cases']}), 11)
        for c in v['cases']:
            self.assertEqual(model(c['premises']), c['expect'], c['id'])
            # Apply every exact mutation independently and verify the wire stays
            # structurally valid; semantic rejection belongs to the Rust SDK.
            d = deepcopy(json.loads((DIRECTORY / 'owned-readonly-definition.json').read_bytes())['definition'])
            for change in c['replace']:
                target = d['body']
                parts = change['path'].strip('/').split('/')
                for part in parts[:-1]:
                    target = target[part]
                target[parts[-1]] = change['value']
            d['revision'] = wire.revision(d)
            jsonschema.Draft202012Validator(json.loads((ROOT / 'schemas/ir/definition-0.4.schema.json').read_bytes())).validate(d)

    def test_licensing_ci_and_raw_only_boundary(self):
        notice = 'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n'
        for name in ['owned-readonly.json', 'owned-readonly-definition.json']:
            self.assertEqual((DIRECTORY / (name + '.license')).read_text(), notice)
        self.assertIn('tools/check_owned_readonly.py', (ROOT / '.github/workflows/rust-portability.yml').read_text())
        # No source upgrade is silently selected by this raw SDK increment.
        self.assertEqual(json.loads((ROOT / 'docs/benchmarks/rp01/candidate.ir.json').read_bytes())['version'], '0.1.0')


if __name__ == '__main__':
    unittest.main()
