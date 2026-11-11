# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite readable/static evidence, never actual provider or host observations."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest
import authoring_probe as grammar
from benchmarks.rp01_resources import encoded
from check_static_contracts import ROOT
from check_readonly_continuation_authoring import FIXTURES, STAGES, invoke, budget_package as prior_budget
from check_pre_capture_authoring import package as prior_package

CORE = {'contract':'urn:choreoform:contracts:core-profile:0.11.0',
        'digest':'sha256:246bf3835eef25f8adf76bce39e6b887715e098761e675f821bf745a8111d29e', 'dialect':'core'}


def package():
    bundle = json.loads((FIXTURES/'finite-work.resources.json').read_bytes())
    resources = [(r['id'], r['revision'], r['bytes'].encode()) for r in bundle['resources']]
    resources += [(r['id'], r['revision'], (ROOT/'docs/ir/contracts'/(r['revision'].replace(':', '-')+'.txt')).read_bytes()) for r in bundle['normative'] if 'core-profile' not in r['id'] and not r['id'].startswith('urn:choreoform:semantics:')]
    return ((FIXTURES/'finite-work.choreo').read_text(), json.loads((FIXTURES/'finite-work.bindings.json').read_bytes()), resources)


def promote(p, old_profile):
    s, l, r = deepcopy(p)
    s = s.replace(f'authoring "{old_profile}"', 'authoring "0.14.0"')
    l.update(profile='0.14.0', version='0.13.0'); l['contracts']['core'] = deepcopy(CORE)
    return s, l, r


def command_metadata(p, effect='irreversible', linked=True, key=None):
    # Fresh test resource revisions. Published fixture/benchmark bytes stay fixed.
    s, l, r = deepcopy(p); key = key or ('none' if effect == 'read-only' else 'stable-key')
    s = s.replace('Rule for effect "Adjustment effects". Effect is irreversible. Idempotency is stable-key.', f'Rule for effect "Adjustment effects". Effect is {effect}. Idempotency is {key}.')
    if not linked: s = s.replace('Compensates "Create payable".', 'Compensates none.')
    identity = 'urn:test:readable-owned-adjustment'
    i = next(i for i, row in enumerate(r) if row[0] == identity)
    old = {'id':identity, 'revision':r[i][1]}; provider = json.loads(r[i][2])
    provider.update(effectClass=effect, idempotency=key)
    payload = encoded(provider); pin = {'id':identity, 'revision':'sha256:'+sha256(payload).hexdigest()}
    r[i] = (identity, pin['revision'], payload)
    changes = {identity:pin['revision']}
    for i, (name, revision, raw) in enumerate(r):
        if name not in ['urn:test:readable-adjustment-terminal', 'urn:test:readable-adjustment-faults']: continue
        resource = json.loads(raw); assert resource['provider'] == old
        resource['provider'] = pin; raw = encoded(resource); revision = 'sha256:'+sha256(raw).hexdigest()
        r[i] = (name, revision, raw); changes[name] = revision
    for resource in l['resources']:
        if resource['contract'] in changes: resource['digest'] = changes[resource['contract']]
    return s, l, r


class FiniteWorkAuthoringEvidence(unittest.TestCase):
    def test_all_native_views_expose_typed_work_without_false_query_or_activation(self):
        p = package(); lowered = json.loads(invoke(p).stdout)
        body = lowered['body']
        self.assertEqual(body['data']['lookup_request']['type'], body['capabilities']['adapter_lookup']['input'])
        self.assertNotEqual(body['data']['original_request']['type'], body['data']['lookup_request']['type'])
        self.assertEqual(body['expressions']['lookup_input']['body'], {'op':'payload', 'value':{'op':'read', 'cell':'captured'}, 'tag':'confirmed'})
        for stage in STAGES:
            result = invoke(p, stage); self.assertEqual(result.returncode, 0, (stage, result.stderr))
            view = json.loads(result.stdout); self.assertIs(view['executable'], False)
            self.assertEqual(len(view['declaredRetainedRequests']), 2)
            if stage in STAGES[5:]:
                scopes = {row['scope']:row for row in view['ownedContinuationScopes']}
                work = scopes['recovery']['work']
                self.assertIsNone(scopes['recovery']['query']); self.assertNotIn('preCaptureEntry', scopes['recovery'])
                self.assertEqual({k:work[k] for k in ['site', 'capability', 'input', 'effectClass', 'compensates']}, {'site':'lookup', 'capability':'adapter_lookup', 'input':'lookup_input', 'effectClass':'irreversible', 'compensates':'origin'})
                self.assertTrue(work['hostRequirements']); self.assertNotIn('work', scopes['lookup_recovery'])
                checks = view['conditionalOwnedContinuationChecks']
                self.assertTrue(checks['conditional']); self.assertEqual(checks['revision'], view['revision']); self.assertEqual(len(checks['gates']), 7)
            else: self.assertNotIn('ownedContinuationScopes', view)
            if stage == STAGES[-1]:
                self.assertNotIn('lookup', view['checkedReconciliationNodes']); self.assertIn('lookup', view['uncoveredNodes'])
                self.assertEqual(set(view['continuationScopes']), {'recovery', 'lookup_recovery'})

    def test_closed_grammar_schema_identity_format_unicode_and_exact_metadata(self):
        p = package(); grammar.recognize(p[0]); grammar.validate_ledger(p[1])
        self.assertEqual(grammar.load_grammar(ROOT/'docs/authoring/candidate-a-v0.14.ebnf'), grammar.load_grammar(ROOT/'docs/authoring/candidate-a-v0.13.ebnf'))
        old = json.loads((ROOT/'schemas/authoring/bindings-0.12.schema.json').read_bytes())
        new = json.loads((ROOT/'schemas/authoring/bindings-0.13.schema.json').read_bytes())
        new['$id'] = old['$id']; new['title'] = old['title']
        new['properties']['profile']['const'] = '0.13.0'; new['properties']['version']['const'] = '0.12.0'
        new['properties']['contracts']['properties']['core']['properties'] = old['properties']['contracts']['properties']['core']['properties']
        self.assertEqual(new, old)
        self.assertEqual(json.loads(invoke(p, 'integrity').stdout), grammar.package_digests(p[0].encode(), encoded(p[1])))
        before = json.loads(invoke(p).stdout); formatted = json.loads(invoke(p, 'format').stdout)
        self.assertEqual(json.loads(invoke((formatted['source'], json.loads(formatted['companion']), p[2])).stdout), before)
        s, l, r = deepcopy(p); s = s.replace('"Adjustment recovery"', '"Fólio de recuperação"')
        next(row for row in l['declarations'] if row['id'] == 'lookup_recovery')['name'] = 'Fólio de recuperação'
        self.assertEqual(json.loads(invoke((s, l, r)).stdout), before)
        bundle = json.loads((FIXTURES/'finite-work.resources.json').read_bytes())
        self.assertIn('no actual observations', bundle['premise'])
        for resource in bundle['resources']:
            self.assertEqual(resource['revision'], 'sha256:'+sha256(resource['bytes'].encode()).hexdigest())

    def test_effect_class_and_compensation_link_are_independent_native_relations(self):
        for effect in ['read-only', 'compensatable', 'irreversible']:
            for linked in [False, True]:
                p = command_metadata(package(), effect, linked)
                result = invoke(p, STAGES[-1]); self.assertEqual(result.returncode, 0, (effect, linked, result.stderr))
                row = next(row for row in json.loads(result.stdout)['ownedContinuationScopes'] if row['scope'] == 'recovery')
                self.assertEqual(row['work']['effectClass'], effect); self.assertEqual(row['work']['compensates'], 'origin' if linked else None)
                self.assertEqual(row['query'], 'lookup' if effect == 'read-only' and not linked else None)
        p = promote(prior_package(), '0.13.0'); s, l, r = p
        s = s.replace('Let "Lookup input" mean (the value of "Original request")', 'Let "Lookup input" mean (record with empty)')
        result = invoke((s, l, r), STAGES[-1]); self.assertEqual(result.returncode, 0, result.stderr)
        row = next(row for row in json.loads(result.stdout)['ownedContinuationScopes'] if row['scope'] == 'recovery')
        self.assertEqual(row['query'], 'lookup'); self.assertEqual(row['work']['effectClass'], 'read-only'); self.assertIn('preCaptureEntry', row)

    def test_actual_refusals_and_original_utf8_request_clause(self):
        for change, stage, code in [('early', 'check-concurrency', b'owned-continuation-work-entry'), ('self', 'check-concurrency', b'owned-continuation-compensation-source'), ('request', 'check', b'retained-request-readonly'), ('key', 'check-concurrency', b'owned-continuation-work-idempotency'), ('source', 'check-concurrency', b'owned-continuation-work-policy'), ('type', 'check', b'activity-input'), ('invalidation', 'check-concurrency', b'owned-continuation-invalidation-coverage')]:
            s, l, r = deepcopy(package())
            if change == 'early': s = s.replace('Start at "Capture wait".', 'Start at "Adjust account".')
            elif change == 'self': s = s.replace('Compensates "Create payable".', 'Compensates "Adjust account".')
            elif change == 'key': s, l, r = command_metadata((s, l, r), key='none')
            elif change == 'source': s = s.replace('Recognise faults from "Adjustment faults". ', '')
            elif change == 'type': s = s.replace('Let "Adjustment input" mean (the payload of (the value of "Captured") for "confirmed") as whole number', 'Let "Adjustment input" mean (record with empty) as record with empty')
            elif change == 'invalidation':
                start = s.index('Keep "Original request"'); end = s.index('End data.', start)
                block = s[start:end]; block = block[:block.index('Invalidate ')]+'Invalidate empty. Initially unavailable. '
                s = s[:start]+block+s[end:]
            else:
                s = s.replace('"Adjustment recovery"', '"Fólio de recuperação"')
                next(row for row in l['declarations'] if row['id'] == 'lookup_recovery')['name'] = 'Fólio de recuperação'
                s = s.replace('and record in "Adjustment request".', 'and record in "Adjustment captured".')
            result = invoke((s, l, r), stage)
            self.assertEqual(result.returncode, 1, (change, result.stderr)); self.assertEqual(result.stdout, b''); self.assertIn(code, result.stderr, change)
            if change == 'request':
                clause = 'Retain the original request from "Adjust account" and record in "Adjustment captured".'
                offset = s.encode().index(clause.encode()); self.assertIn(f'source bytes {offset}..{offset+len(clause.encode())}'.encode(), result.stderr)

    def test_valid_combined_work_and_history_preserves_whole_report_budget(self):
        def stress(count):
            s, l, r = promote(prior_budget(count), '0.12.0')
            s = s.replace('Let "Lookup input" mean (the value of "Original request")', 'Let "Lookup input" mean (record with empty)')
            return s, l, r
        small = stress(1); grammar.recognize(small[0]); grammar.validate_ledger(small[1])
        for stage in STAGES:
            result = invoke(small, stage); self.assertEqual(result.returncode, 0, (stage, result.stderr))
            if stage in STAGES[5:]: self.assertTrue(any(row.get('work') for row in json.loads(result.stdout)['ownedContinuationScopes']))
        large = stress(130); grammar.recognize(large[0]); grammar.validate_ledger(large[1]); self.assertLess(len(grammar.lex(large[0])), 16384)
        result = invoke(large); self.assertEqual(result.returncode, 0, result.stderr); self.assertLess(len(result.stdout), 1024*1024)
        for stage in STAGES:
            result = invoke(large, stage); self.assertEqual(result.returncode, 1, (stage, result.stderr)); self.assertEqual(result.stdout, b''); self.assertIn(b'report-budget', result.stderr)

    def test_old_selection_unknown_pair_pins_licensing_and_ci_remain_explicit(self):
        s, l, r = deepcopy(package()); s = s.replace('authoring "0.14.0"', 'authoring "0.13.0"')
        l.update(profile='0.13.0', version='0.12.0'); l['contracts']['core'] = deepcopy(prior_package()[1]['contracts']['core'])
        result = invoke((s, l, r), 'check-concurrency'); self.assertEqual(result.returncode, 1); self.assertIn(b'owned-continuation-query-effect', result.stderr)
        for stage in STAGES[5:]:
            result = invoke(prior_package(), stage); self.assertEqual(result.returncode, 0, result.stderr)
            for row in json.loads(result.stdout)['ownedContinuationScopes']: self.assertNotIn('work', row)
        for change in ['pair', 'unknown', 'pin']:
            s, l, r = deepcopy(package())
            if change == 'pair': l['version'] = '0.12.0'
            elif change == 'unknown': s = s.replace('authoring "0.14.0"', 'authoring "0.16.0"')
            else: l['contracts']['core']['digest'] = 'sha256:'+'0'*64
            result = invoke((s, l, r)); self.assertEqual(result.returncode, 1); self.assertEqual(result.stdout, b'')
        notice = 'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n'
        for path in [FIXTURES/'finite-work.choreo', FIXTURES/'finite-work.bindings.json', FIXTURES/'finite-work.resources.json', ROOT/'schemas/authoring/bindings-0.13.schema.json']:
            self.assertEqual(path.with_name(path.name+'.license').read_text(), notice)
        self.assertIn('tools/check_finite_work_authoring.py', (ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__ == '__main__':
    unittest.main()
