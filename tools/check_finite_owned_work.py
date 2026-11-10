# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Limited independent finite premises, never general admission or actual work."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest
import jsonschema
import check_ir_fixtures as wire
from check_static_contracts import ROOT

DIRECTORY = ROOT/'crates/semantic-check/tests/conformance'
CORPUS = DIRECTORY/'finite-owned-work.json'
CONTRACT = ROOT/'docs/dialects/core-owned-work-v0.11.md'


def model(case):
    # Setup selectors promise prevalidated matching metadata, not wire fields.
    # Only the isolated mutations in this corpus belong to this model.
    s = case['setup']; changes = {p['path']: p['value'] for p in case['put']}
    if changes.get('/scopes/recovery/retainedRequests') == {}:
        return 'owned-continuation-request-unproved'
    if s['class'] != 'read-only' and s['idempotency'] != 'stable-key':
        return 'owned-continuation-work-idempotency'
    if '/policies/lookup_work/body/faultSource' in changes and changes['/policies/lookup_work/body/faultSource'] is None:
        return 'owned-continuation-work-policy'
    if changes.get('/policies/lookup_work/body/compensates', 'origin') != 'origin':
        return 'owned-continuation-compensation-source'
    if not s['capturing'] and (s['class'] != 'read-only' or s['compensates']):
        return 'owned-continuation-work-entry'
    if changes.get('/scopes/root/entry') == 'owner_done':
        return 'owned-continuation-source-unproved'
    if s['input'] == 'captured' and not s['capturing']:
        return 'owned-continuation-availability-unproved'
    recipients = changes.get('/data/original_request/protection/capabilities')
    if recipients is not None and 'adapter_lookup' not in recipients:
        target = changes.get('/data/captured/protection/capabilities', {'adapter': True, 'adapter_lookup': True})
        if s['capturing'] and not set(target).issubset(recipients):
            return 'control-information-flow'
        return 'owned-continuation-recipient'
    if changes.get('/data/original_request/invalidates') == {}:
        return 'owned-continuation-invalidation-coverage'
    return 'covered'


class FiniteOwnedWorkEvidence(unittest.TestCase):
    def test_exact_artifact_ten_inherited_bases_and_library_pin(self):
        corpus = json.loads(CORPUS.read_bytes()); raw = CONTRACT.read_bytes()
        pin = 'sha256:'+sha256(raw).hexdigest()
        self.assertEqual(corpus['core']['revision'], pin)
        self.assertEqual((ROOT/'docs/ir/contracts'/(pin.replace(':', '-')+'.txt')).read_bytes(), raw)
        self.assertIn(pin, (ROOT/'crates/semantic-check/src/lib.rs').read_text())
        base = json.loads((DIRECTORY/corpus['base']).read_bytes())
        cores = [p for p in base['normative'] if 'core-profile:' in p['id']]+[corpus['additionalBase']]
        self.assertEqual(len(cores), 10)
        for inherited in cores:
            artifact = (ROOT/'docs/ir/contracts'/(inherited['revision'].replace(':', '-')+'.txt')).read_bytes()
            self.assertEqual(inherited['revision'], 'sha256:'+sha256(artifact).hexdigest())
            self.assertIn(inherited['id'], raw.decode()); self.assertIn(inherited['revision'], raw.decode())

    def test_shared_finite_cases_have_explicit_premises_and_independent_results(self):
        corpus = json.loads(CORPUS.read_bytes())
        self.assertIn('not serialized IR or general admission', corpus['premise'])
        self.assertIn('no actual observations', corpus['premise'])
        self.assertEqual(len(corpus['cases']), 21)
        self.assertEqual(len({c['id'] for c in corpus['cases']}), 21)
        for case in corpus['cases']:
            s = case['setup']
            self.assertEqual(set(s), {'class', 'compensates', 'capturing', 'input', 'idempotency'})
            self.assertIn(s['class'], ['read-only', 'compensatable', 'irreversible'])
            self.assertIn(s['input'], ['original', 'constant', 'captured'])
            self.assertIn(s['idempotency'], ['none', 'stable-key'])
            self.assertIs(type(s['compensates']), bool); self.assertIs(type(s['capturing']), bool)
            self.assertEqual(model(case), case['expect'], case['id'])

    def test_existing_wire_represents_new_core_without_a_schema_delta(self):
        corpus = json.loads(CORPUS.read_bytes())
        definition = deepcopy(json.loads((DIRECTORY/corpus['base']).read_bytes())['definition'])
        definition['body']['dialects']['core'] = corpus['core']
        definition['revision'] = wire.revision(definition)
        schema = json.loads((ROOT/'schemas/ir/definition-0.4.schema.json').read_bytes())
        jsonschema.Draft202012Validator(schema).validate(definition)
        self.assertEqual(definition['version'], '0.4.0')

    def test_readable_selection_and_old_native_rows_stay_at_prior_core(self):
        from check_pre_capture_authoring import package, invoke, STAGES
        p = package(); self.assertEqual(p[1]['profile'], '0.13.0')
        d = json.loads(invoke(p).stdout)
        self.assertEqual(d['body']['dialects']['core']['id'], 'urn:choreoform:contracts:core-profile:0.10.0')
        for stage in STAGES[5:]:
            result = invoke(p, stage); self.assertEqual(result.returncode, 0, result.stderr)
            view = json.loads(result.stdout); self.assertIs(view['executable'], False)
            for row in view['ownedContinuationScopes']: self.assertNotIn('work', row)

    def test_licensing_and_required_ci(self):
        self.assertEqual(CORPUS.with_name(CORPUS.name+'.license').read_text(), 'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n')
        self.assertIn('SPDX-License-Identifier: CC-BY-4.0', CONTRACT.read_text())
        self.assertIn('tools/check_finite_owned_work.py', (ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__ == '__main__':
    unittest.main()
