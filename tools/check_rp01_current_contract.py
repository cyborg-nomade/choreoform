# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Explicit experimental full-case probes; never committed package adoption."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest
from check_static_contracts import ROOT
from check_metadata_authoring import invoke
from check_readonly_continuation_authoring import STAGES
from benchmarks.rp01_resources import encoded
from benchmarks.rp01_source import type_source, expression_source, names, quote

CHECKPOINT = ROOT/'docs/benchmarks/rp01'
# Explicit historical fixture; current adoption is checked by check_rp01_capture_checkpoint.
BASE = CHECKPOINT/'history/source-0.7'
CORE = {'contract':'urn:choreoform:contracts:core-profile:0.11.0',
        'digest':'sha256:246bf3835eef25f8adf76bce39e6b887715e098761e675f821bf745a8111d29e', 'dialect':'core'}


def baseline():
    manifest = json.loads((BASE/'manifest.json').read_bytes())
    return ((BASE/'source.choreo').read_text(), json.loads((BASE/'bindings.json').read_bytes()),
            [(r['id'], r['revision'], (ROOT/r['path']).read_bytes()) for r in manifest['resources']])


def promoted(adapt_retry=True):
    s, l, r = deepcopy(baseline())
    s = s.replace('authoring "0.7.0"', 'authoring "0.14.0"', 1)
    l.update(profile='0.14.0', version='0.13.0'); l['contracts']['core'] = deepcopy(CORE)
    if adapt_retry:
        lines = s.splitlines(keepends=True)
        for i, line in enumerate(lines):
            if not line.startswith('Rule for work "Payment work".'): continue
            old = ', fault source "Payment faults".'
            assert line.count(old) == 1
            line = line.replace(old, '.')
            old = 'Authority "Payment operator". Attempts'
            assert line.count(old) == 1
            lines[i] = line.replace(old, 'Authority "Payment operator". Recognise faults from "Payment faults". Attempts')
        s = ''.join(lines)
    return s, l, r


def captured(retain=True, record_status=False):
    # Existing exact provider and recipients; only new static source binding and
    # fresh local declarations. No actual event or accepted input is constructed.
    s, l, r = promoted()
    b = json.loads((BASE/'candidate.ir.json').read_bytes())['body']
    provider = next({'id':i, 'revision':revision} for i, revision, _ in r if i == 'urn:choreoform:benchmarks:rp01:payment:0.1.0')
    profile = {'id':'urn:choreoform:observation-source-profile:provider-terminal-facts:0.1.0', 'revision':'sha256:a224fc2b8318af2ff8dfc9955b55af03f4d5b7bd0de719de0384421698d20ead'}
    descriptor = {'kind':'provider-terminal-observation-source', 'version':'0.1.0', 'id':'urn:test:rp01-current-payment-terminal', 'profile':profile, 'provider':provider}
    raw = encoded(descriptor); revision = 'sha256:'+sha256(raw).hexdigest()
    r += [(descriptor['id'], revision, raw), (profile['id'], profile['revision'], (ROOT/'docs/ir/contracts'/(profile['revision'].replace(':', '-')+'.txt')).read_bytes())]
    l['declarations'].append({'id':'audit_payment_terminal_binding', 'kind':'resource', 'scope':'expense', 'name':'Audit payment facts'})
    l['resources'].append({'binding':'audit_payment_terminal_binding', 'kind':'provider-terminal-observation-source', 'contract':descriptor['id'], 'digest':revision})
    captions = {row['id']:row['name'] for row in l['declarations']}
    envelope = b['data']['payment_result']['protection']
    def local(identity, caption, ty):
        l['declarations'].append({'id':identity, 'kind':'data', 'scope':'payment_recovery', 'name':caption})
        l['symbols'].append({'id':identity+'_purpose', 'owner':identity, 'role':'purpose', 'name':'Reimbursement', 'wire':'reimbursement'})
        captions[identity] = caption
        invalidates = ['Observe Payment', 'Finish Payment recovery']+(['Audit classify payment'] if record_status else [])
        return f'Keep {quote(caption)} as {type_source(ty)} under {quote(captions[envelope["policy"]])}. Sensitivity {quote(envelope["sensitivity"])}. Purposes ["Reimbursement"]. Participants {names(captions[a] for a in envelope["participants"])}. Capabilities {names(captions[a] for a in envelope["capabilities"])}. Invalidate {names(invalidates)}. Initially unavailable. End data.\n'
    declarations = ''
    if retain:
        declarations += 'Retain the original request from "Request Payment" and record in "Audit original payment request".\n'
        declarations += local('audit_payment_request', 'Audit original payment request', b['capabilities']['payment_capability']['input']['body'])
    declarations += local('audit_payment_capture', 'Audit payment capture', b['policies']['payment_wait']['body']['observationType'])
    if record_status:
        assert retain
        declarations += local('audit_late_category', 'Audit late category', {'kind':'text'})
        l['declarations'].append({'id':'audit_classify_payment', 'kind':'node', 'scope':'payment_recovery', 'name':'Audit classify payment'})
        l['symbols'].append({'id':'audit_classify_next', 'owner':'audit_classify_payment', 'role':'outcome', 'name':'next', 'wire':'next'})
        l['generated'] += [{'id':'audit_classify_assignment', 'owner':'audit_classify_payment', 'role':'assignment', 'key':'audit_late_category'}, {'id':'audit_classify_flow', 'owner':'audit_classify_payment', 'role':'flow', 'key':'audit_classify_next'}]
        ast = json.loads((ROOT/'crates/semantic-check/tests/conformance/rp01-late-classification.json').read_bytes())['expression']
        def readable(value):
            if isinstance(value, dict):
                if value.get('op') == 'read': return {'op':'read', 'cell':captions[value['cell']]}
                return {key:readable(child) for key, child in value.items()}
            if isinstance(value, list): return [readable(child) for child in value]
            return value
        expr = expression_source(readable(ast))
        declarations += f'Step "Audit classify payment". Set together ["Audit late category" to ({expr})]. On "next", continue with "Finish Payment recovery". End step.\n'
    marker = 'Step "Observe Payment".'; assert s.count(marker) == 1
    s = s.replace(marker, declarations+marker)
    old = 'Step "Observe Payment". Wait under "Payment reconciliation".'
    assert s.count(old) == 1
    s = s.replace(old, 'Step "Observe Payment". Wait under "Payment reconciliation" for observations from "Audit payment facts" about "Request Payment" and record in "Audit payment capture".')
    if record_status:
        old = 'and record in "Audit payment capture". On "observed", continue with "Finish Payment recovery".'
        assert s.count(old) == 1
        s = s.replace(old, 'and record in "Audit payment capture". On "observed", continue with "Audit classify payment".')
    return s, l, r


def fixed_bytes():
    paths = [p for p in CHECKPOINT.rglob('*') if p.is_file()]
    paths += [ROOT/'crates/semantic-check/tests/conformance/rp01-guard-snapshots.json']
    return {str(p.relative_to(ROOT)):sha256(p.read_bytes()).hexdigest() for p in paths}


class Rp01CurrentContractEvidence(unittest.TestCase):
    def test_frozen_baseline_still_passes_all_eleven_views_without_adoption(self):
        before = fixed_bytes(); p = baseline()
        self.assertEqual((len(p[0].encode()), p[1]['profile'], p[1]['version']), (56874, '0.7.0', '0.6.0'))
        for stage in STAGES:
            result = invoke(p, stage); self.assertEqual(result.returncode, 0, (stage, result.stderr))
            self.assertIs(json.loads(result.stdout)['executable'], False)
        self.assertEqual(fixed_bytes(), before)

    def test_explicit_promotion_needs_retry_syntax_and_only_changes_versioned_fields(self):
        result = invoke(promoted(False)); self.assertEqual(result.returncode, 1)
        self.assertEqual(result.stdout, b''); self.assertIn(b'syntax at ', result.stderr)
        old = json.loads((BASE/'candidate.ir.json').read_bytes())
        result = invoke(promoted()); self.assertEqual(result.returncode, 0, result.stderr)
        new = json.loads(result.stdout); self.assertEqual(new['version'], '0.4.0')
        body = deepcopy(new['body']); body['dialects'] = old['body']['dialects']
        for scope in body['scopes'].values():
            self.assertIsNone(scope.pop('faultInput')); self.assertEqual(scope.pop('retainedRequests'), {})
        for node in body['nodes'].values():
            if node['kind'] == 'wait': self.assertIsNone(node.pop('capture'))
        for identity, policy in body['policies'].items():
            if policy['body']['kind'] == 'work':
                pin = policy['body'].pop('faultSource')
                self.assertEqual(pin, old['body']['policies'][identity]['body']['retry']['faultSource'] if identity == 'payment_work' else None)
        self.assertEqual(body, old['body'])
        for stage in STAGES:
            result = invoke(promoted(), stage); self.assertEqual(result.returncode, 0, (stage, result.stderr))
            self.assertIs(json.loads(result.stdout)['executable'], False)

    def test_whole_capture_retention_and_classification_have_real_separate_owned_proof(self):
        before = fixed_bytes()
        for retain, classify in [(False, False), (True, False), (True, True)]:
            p = captured(retain, classify)
            self.assertEqual(p[2][:len(baseline()[2])], baseline()[2])
            for stage in STAGES:
                result = invoke(p, stage); self.assertEqual(result.returncode, 0, (retain, classify, stage, result.stderr))
                view = json.loads(result.stdout); self.assertIs(view['executable'], False)
                if stage in STAGES[5:]:
                    row, = view['ownedLocalFollowups']; self.assertEqual(row['body'], 'payment_service')
                    self.assertEqual(row['followup'], 'payment_recovery'); self.assertEqual(row['captureSites'], ['payment_observe'])
                    self.assertNotIn('ownedContinuations', view)
                    if retain: self.assertEqual(row['originalRequest'], {'origin':'payment_request_step', 'result':'audit_payment_request'})
                    checks = view['conditionalOwnedLocalChecks']; self.assertTrue(checks['conditional'])
                    self.assertIn({'kind':'cell', 'owner':'payment_service', 'providerRole':None, 'site':'current_claim'}, checks['requiredProtectionProvenance'])
                if stage == STAGES[-1]:
                    if retain:
                        self.assertIn('payment_observe', view['uncoveredNodes'])
                        self.assertNotIn('payment_observe', view['checkedReconciliationNodes'])
                        self.assertEqual(view['retainedRequestScopes'], ['payment_recovery'])
                    else:
                        # Fresh capture needs no retained live seed, so legacy
                        # zero-inherited-value availability is still applicable.
                        self.assertIn('payment_observe', view['checkedReconciliationNodes'])
                    if classify: self.assertIn('audit_classify_payment', view['conditionalOwnedLocalChecks']['checkedNodes'])
        self.assertEqual(fixed_bytes(), before)

    def test_late_classification_matches_shared_pure_expression_and_never_writes_parent(self):
        p = captured(record_status=True); result = invoke(p); self.assertEqual(result.returncode, 0, result.stderr)
        b = json.loads(result.stdout)['body']
        corpus = json.loads((ROOT/'crates/semantic-check/tests/conformance/rp01-late-classification.json').read_bytes())
        self.assertIn('not authenticated observations', corpus['premise'])
        self.assertEqual(len(corpus['cases']), 13)
        self.assertEqual(b['expressions']['audit_classify_assignment']['body'], corpus['expression'])
        self.assertEqual(b['nodes']['audit_classify_payment']['writes'], {'audit_late_category':True})
        self.assertEqual(b['data']['audit_late_category']['scope'], 'payment_recovery')
        self.assertEqual(b['scopes']['payment_recovery']['outputs'], {})
        self.assertEqual(b['nodes']['payment_request_step']['reads']['current_claim'], True)
        self.assertEqual(b['data']['current_claim']['scope'], 'expense')
        old = json.loads((BASE/'candidate.ir.json').read_bytes())['body']
        def rebound(value):
            if isinstance(value, dict):
                if value.get('op') == 'fact':
                    self.assertEqual(value['name'], 'result')
                    return {'op':'payload', 'value':{'op':'read', 'cell':'audit_payment_capture'}, 'tag':'confirmed'}
                if value.get('op') == 'read':
                    self.assertEqual(value['cell'], 'payment_request')
                    return {'op':'read', 'cell':'audit_payment_request'}
                return {key:rebound(child) for key, child in value.items()}
            if isinstance(value, list): return [rebound(child) for child in value]
            return value
        self.assertEqual(corpus['expression']['then']['condition'], rebound(old['policies']['payment_work']['body']['completion']))
        self.assertEqual(b['nodes']['audit_classify_payment']['reads'], {'audit_payment_capture':True, 'audit_payment_request':True})
        self.assertEqual(b['scopes']['payment_service']['outputs'], old['scopes']['payment_service']['outputs'])
        self.assertEqual(b['data']['payment_result'], old['data']['payment_result'])

    def test_real_capture_request_and_invalidation_refusals_keep_original_protections(self):
        for change, stage, code in [('provider', 'check', b'wait-capture-provider'), ('request', 'check', b'retained-request-readonly'), ('initial', 'check', b'retained-request-readonly'), ('invalidation', 'check-concurrency', b'owned-local-invalidation-coverage')]:
            s, l, r = deepcopy(captured())
            if change == 'provider':
                provider = next({'id':i, 'revision':rev} for i, rev, _ in r if i == 'urn:choreoform:benchmarks:rp01:notice:0.1.0')
                i = next(i for i, row in enumerate(r) if row[0] == 'urn:test:rp01-current-payment-terminal')
                descriptor = json.loads(r[i][2]); descriptor['provider'] = provider
                raw = encoded(descriptor); revision = 'sha256:'+sha256(raw).hexdigest()
                r[i] = (descriptor['id'], revision, raw)
                next(row for row in l['resources'] if row['binding'] == 'audit_payment_terminal_binding')['digest'] = revision
            elif change == 'request':
                s = s.replace('Retain the original request from "Request Payment" and record in "Audit original payment request".', 'Retain the original request from "Request Payment" and record in "Audit payment capture".')
            else:
                start = s.index('Keep "Audit original payment request"'); end = s.index('End data.', start)
                block = s[start:end]
                if change == 'initial':
                    block = block.replace('Initially unavailable.', 'Initially (the value of "Payable request").')
                    l['generated'].append({'id':'audit_bad_seed_initial', 'owner':'audit_payment_request', 'role':'initial', 'key':None})
                else:
                    block = block[:block.index('Invalidate ')]+ 'Invalidate empty. Initially unavailable. '
                s = s[:start]+block+s[end:]
            result = invoke((s, l, r), stage)
            self.assertEqual(result.returncode, 1, (change, result.stderr)); self.assertEqual(result.stdout, b''); self.assertIn(code, result.stderr, change)

    def test_real_sizes_stay_within_limits_licensing_and_ci(self):
        import authoring_probe as grammar
        for p in [promoted(), captured(), captured(record_status=True)]:
            grammar.recognize(p[0]); grammar.validate_ledger(p[1])
            self.assertLess(len(grammar.lex(p[0])), 16384)
            result = invoke(p); self.assertEqual(result.returncode, 0, result.stderr); self.assertLess(len(result.stdout), 1024*1024)
        corpus = ROOT/'crates/semantic-check/tests/conformance/rp01-late-classification.json'
        self.assertEqual(corpus.with_name(corpus.name+'.license').read_text(), 'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n')
        self.assertIn('tools/check_rp01_current_contract.py', (ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__ == '__main__':
    unittest.main()
