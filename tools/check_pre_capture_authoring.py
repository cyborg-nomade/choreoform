# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite source/native integration; no host events, authority or acceptance."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest
import authoring_probe as grammar
from benchmarks.rp01_resources import encoded
from check_static_contracts import ROOT
from check_readonly_continuation_authoring import (
    FIXTURES, STAGES, invoke, package as prior_package, budget_package as prior_budget,
)

CORE = {
    'contract': 'urn:choreoform:contracts:core-profile:0.10.0',
    'digest': 'sha256:74c3d5bfe2a880259bc13f521127aa1dd84977ae9e6f83b339e3a0629964e60f',
    'dialect': 'core',
}


def package():
    return ((FIXTURES/'pre-capture.choreo').read_text(),
            json.loads((FIXTURES/'pre-capture.bindings.json').read_bytes()),
            prior_package()[2])


def promote(p):
    s, l, r = deepcopy(p)
    s = s.replace('authoring "0.12.0"', 'authoring "0.13.0"')
    s = s.replace('Start at "Capture wait".', 'Start at "Lookup".')
    s = s.replace('On "done", continue with "Done".', 'On "done", continue with "Capture wait".')
    s = s.replace('On "yes", continue with "Lookup".', 'On "yes", continue with "Done".')
    l.update(profile='0.13.0', version='0.12.0')
    l['contracts']['core'] = deepcopy(CORE)
    return s, l, r


class PreCaptureAuthoringEvidence(unittest.TestCase):
    def test_all_eleven_views_keep_distinct_proof_and_optional_live_host_premises(self):
        for stage in STAGES:
            result = invoke(package(), stage)
            self.assertEqual(result.returncode, 0, (stage, result.stderr))
            view = json.loads(result.stdout)
            self.assertIs(view['executable'], False)
            self.assertEqual(len(view['declaredRetainedRequests']), 2)
            if stage in STAGES[5:]:
                scopes = {r['scope']: r for r in view['ownedContinuationScopes']}
                relation = scopes['recovery']['preCaptureEntry']
                self.assertEqual(relation['entry'], 'lookup')
                self.assertEqual(relation['captureWait'], 'capture_wait')
                self.assertIn('actual-accepted-occurrence-bound-P6-registration-and-atomic-handoff-before-entry', relation['hostRequirements'])
                self.assertNotIn('preCaptureEntry', scopes['lookup_recovery'])
                checks = view['conditionalOwnedContinuationChecks']
                self.assertTrue(checks['conditional'])
                self.assertEqual(checks['revision'], view['revision'])
                self.assertEqual(len(checks['gates']), 7)
            else:
                self.assertNotIn('ownedContinuationScopes', view)
            if stage == STAGES[-1]:
                self.assertEqual(set(view['continuationScopes']), {'recovery', 'lookup_recovery'})
                self.assertNotIn('lookup', view['checkedReconciliationNodes'])
                self.assertIn('lookup', view['uncoveredNodes'])

    def test_unchanged_productions_closed_schema_delta_and_package_identity(self):
        p = package()
        grammar.recognize(p[0]); grammar.validate_ledger(p[1])
        self.assertEqual(grammar.load_grammar(ROOT/'docs/authoring/candidate-a-v0.13.ebnf'), grammar.load_grammar(ROOT/'docs/authoring/candidate-a-v0.12.ebnf'))
        old = json.loads((ROOT/'schemas/authoring/bindings-0.11.schema.json').read_bytes())
        new = json.loads((ROOT/'schemas/authoring/bindings-0.12.schema.json').read_bytes())
        new['$id'] = old['$id']; new['title'] = old['title']
        new['properties']['profile']['const'] = '0.12.0'
        new['properties']['version']['const'] = '0.11.0'
        new['properties']['contracts']['properties']['core']['properties'] = old['properties']['contracts']['properties']['core']['properties']
        self.assertEqual(new, old)
        self.assertEqual(json.loads(invoke(p, 'integrity').stdout), grammar.package_digests(p[0].encode(), encoded(p[1])))
        before = json.loads(invoke(p).stdout)
        formatted = json.loads(invoke(p, 'format').stdout)
        self.assertEqual(json.loads(invoke((formatted['source'], json.loads(formatted['companion']), p[2])).stdout), before)
        s, l, r = deepcopy(p)
        s = s.replace('"Lookup recovery"', '"Fólio de recuperação"')
        next(row for row in l['declarations'] if row['id'] == 'lookup_recovery')['name'] = 'Fólio de recuperação'
        self.assertEqual(json.loads(invoke((s, l, r)).stdout), before)

    def test_real_refusals_and_original_utf8_clause_location(self):
        cases = [('request', 'check', b'retained-request-readonly'),
                 ('wait', 'check', b'wait-endpoint-use-unproved'),
                 ('unreachable', 'check-concurrency', b'owned-continuation-capture-unreachable'),
                 ('initializer', 'check', b'retained-request-source-result'),
                 ('early-read', 'check-concurrency', b'owned-continuation-availability-unproved'),
                 ('input', 'check-concurrency', b'owned-continuation-query-input'),
                 ('source', 'check-concurrency', b'owned-continuation-query-policy'),
                 ('effect', 'check', b'capability-effect'),
                 ('invalidation', 'check-concurrency', b'owned-continuation-invalidation-coverage')]
        for change, stage, code in cases:
            s, l, r = deepcopy(package())
            if change == 'request':
                s = s.replace('"Lookup recovery"', '"Fólio de recuperação"')
                next(row for row in l['declarations'] if row['id'] == 'lookup_recovery')['name'] = 'Fólio de recuperação'
                s = s.replace('and record in "Lookup request".', 'and record in "Lookup captured".')
            elif change == 'wait':
                s = s.replace('Reconcile under "Ordinary wait".', 'Reconcile under "Recovery wait".')
            elif change == 'unreachable':
                s = s.replace('On "done", continue with "Capture wait".', 'On "done", continue with "Done".')
            elif change == 'initializer':
                start = s.index('Keep "Captured"'); end = s.index('End data.', start)
                s = s[:start]+s[start:end].replace('Initially unavailable.', 'Initially (case "confirmed" of choice with ["confirmed" as whole number] containing (0)).')+s[end:]
                l['generated'].append({'id':'invented_capture', 'owner':'captured', 'role':'initial', 'key':None})
            elif change == 'early-read':
                s = s.replace('Start at "Lookup".', 'Start at "Read capture".').replace('On "observed", continue with "Read capture".', 'On "observed", continue with "Done".').replace('On "yes", continue with "Done".', 'On "yes", continue with "Lookup".')
            elif change == 'input':
                s = s.replace('Let "Lookup input" mean (the value of "Original request")', 'Let "Lookup input" mean (if (true) then (the value of "Original request") otherwise (the value of "Original request"))')
            elif change == 'source':
                s = s.replace('Recognise faults from "Query faults". ', '')
            elif change == 'effect':
                s = s.replace('Rule for effect "Lookup effects". Effect is read-only. Idempotency is none.', 'Rule for effect "Lookup effects". Effect is irreversible. Idempotency is stable-key.')
            else:
                start = s.index('Keep "Original request"'); end = s.index('End data.', start)
                block = s[start:end]; block = block[:block.index('Invalidate ')]+'Invalidate empty. Initially unavailable. '
                s = s[:start]+block+s[end:]
            result = invoke((s, l, r), stage)
            self.assertEqual(result.returncode, 1, (change, result.stderr))
            self.assertEqual(result.stdout, b'')
            self.assertIn(code, result.stderr, change)
            if change == 'request':
                clause = 'Retain the original request from "Lookup" and record in "Lookup captured".'
                offset = s.encode().index(clause.encode())
                self.assertIn(f'source bytes {offset}..{offset+len(clause.encode())}'.encode(), result.stderr)

    def test_old_selection_is_fixed_and_capturing_first_new_profile_omits_relation(self):
        p = package(); s, l, r = deepcopy(p)
        s = s.replace('authoring "0.13.0"', 'authoring "0.12.0"')
        l.update(profile='0.12.0', version='0.11.0')
        l['contracts']['core'] = deepcopy(prior_package()[1]['contracts']['core'])
        result = invoke((s, l, r), 'check')
        self.assertEqual(result.returncode, 1); self.assertEqual(result.stdout, b'')
        self.assertIn(b'retained-request-capture', result.stderr)
        old = prior_package(); s, l, r = deepcopy(old)
        s = s.replace('authoring "0.12.0"', 'authoring "0.13.0"')
        l.update(profile='0.13.0', version='0.12.0'); l['contracts']['core'] = deepcopy(CORE)
        for candidate in [old, (s, l, r)]:
            for stage in STAGES:
                result = invoke(candidate, stage)
                self.assertEqual(result.returncode, 0, (stage, result.stderr))
                view = json.loads(result.stdout)
                for row in view.get('ownedContinuationScopes', []): self.assertNotIn('preCaptureEntry', row)

    def test_valid_combined_small_proof_and_large_empty_stdout_budget_refusal(self):
        small = promote(prior_budget(1))
        grammar.recognize(small[0]); grammar.validate_ledger(small[1])
        for stage in STAGES:
            result = invoke(small, stage)
            self.assertEqual(result.returncode, 0, (stage, result.stderr))
            if stage in STAGES[5:]:
                self.assertTrue(any(row.get('preCaptureEntry') for row in json.loads(result.stdout)['ownedContinuationScopes']))
        large = promote(prior_budget())
        grammar.recognize(large[0]); grammar.validate_ledger(large[1])
        self.assertLess(len(grammar.lex(large[0])), 16384)
        result = invoke(large); self.assertEqual(result.returncode, 0, result.stderr)
        self.assertLess(len(result.stdout), 1024*1024)
        for stage in STAGES:
            result = invoke(large, stage)
            self.assertEqual(result.returncode, 1, (stage, result.stderr))
            self.assertEqual(result.stdout, b''); self.assertIn(b'report-budget', result.stderr)

    def test_pair_pins_unknown_header_licensing_and_required_ci(self):
        for change in ['pair', 'pin', 'unknown']:
            s, l, r = deepcopy(package())
            if change == 'pair': l['version'] = '0.11.0'
            elif change == 'pin': l['contracts']['core']['digest'] = 'sha256:'+'0'*64
            else: s = s.replace('authoring "0.13.0"', 'authoring "0.14.0"')
            result = invoke((s, l, r)); self.assertEqual(result.returncode, 1); self.assertEqual(result.stdout, b'')
        artifact = (ROOT/'docs/dialects/core-owned-pre-capture-v0.10.md').read_bytes()
        self.assertEqual(CORE['digest'], 'sha256:'+sha256(artifact).hexdigest())
        notice = 'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n'
        for path in [FIXTURES/'pre-capture.choreo', FIXTURES/'pre-capture.bindings.json', ROOT/'schemas/authoring/bindings-0.12.schema.json']:
            self.assertEqual(path.with_name(path.name+'.license').read_text(), notice)
        self.assertIn('tools/check_pre_capture_authoring.py', (ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__ == '__main__':
    unittest.main()
