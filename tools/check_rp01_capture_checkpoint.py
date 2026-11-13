# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Current full-case checkpoint accountability, never host execution evidence."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest

import rfc8785
import jsonschema
import authoring_probe as grammar
from benchmarks.rp01_capture import classification, CAPTURE, REQUEST, CATEGORY, CLASSIFY
from benchmarks.rp01_resources import encoded
from build_rp01_benchmark import artifacts, historical_artifacts, STAGES
from check_metadata_authoring import invoke
from check_static_contracts import ROOT
from check_rp01_guard_snapshots import leaves

BASE = ROOT/'docs/benchmarks/rp01'
HISTORY = BASE/'history/source-0.7'


def package():
    m = json.loads((BASE/'manifest.json').read_bytes())
    return ((BASE/'source.choreo').read_text(), json.loads((BASE/'bindings.json').read_bytes()),
            [(r['id'], r['revision'], (ROOT/r['path']).read_bytes()) for r in m['resources']])


def load(path): return json.loads(path.read_bytes())


class Rp01CaptureCheckpoint(unittest.TestCase):
    def test_actual_generation_and_exact_historical_checkpoint_are_reproducible(self):
        generated = artifacts()
        for name, raw in generated.items(): self.assertEqual((BASE/name).read_bytes(), raw, name)
        for name, raw in historical_artifacts().items():
            if name != 'README.md': self.assertEqual((HISTORY/name).read_bytes(), raw, name)
        self.assertEqual(sha256((HISTORY/'manifest.json').read_bytes()).hexdigest(), 'fd1ca83a022d7ccd8e3f21cf85305d662725e2ac5172cd201615ded0c7471ffb')
        old = load(HISTORY/'candidate.ir.json')
        self.assertEqual(len((HISTORY/'source.choreo').read_bytes()), 56874)
        self.assertEqual(len(grammar.lex((HISTORY/'source.choreo').read_text())), 9470)
        self.assertEqual(old['revision'], 'sha256:97e5c346442d5b051e40bc91d2c3014be1bd990688324be846739c5d35a265c4')
        self.assertEqual(sha256((HISTORY/'candidate.ir.json').read_bytes()).hexdigest(), 'c012c1493ad7b04abc5e5f81529eea6231b720a54ee6a8db867fd2e207fd546a')
        self.assertEqual(len(load(HISTORY/'manifest.json')['semanticStageResults']), 10)

    def test_complete_body_delta_preserves_every_original_guard_flow_and_data_record(self):
        old = load(HISTORY/'candidate.ir.json')['body']
        b = deepcopy(load(BASE/'candidate.ir.json')['body'])
        b['dialects'] = old['dialects']
        for scope, s in b['scopes'].items():
            self.assertIsNone(s.pop('faultInput'))
            self.assertEqual(s.pop('retainedRequests'), {REQUEST:'payment_request_step'} if scope == 'payment_recovery' else {})
        for identity, n in b['nodes'].items():
            if n['kind'] == 'wait':
                c = n.pop('capture')
                if identity == 'payment_observe':
                    self.assertEqual((c['origin'], c['result']), ('payment_request_step', CAPTURE))
                    self.assertEqual(n['writes'], {CAPTURE:True}); n['writes'] = {}
                else: self.assertIsNone(c)
        for identity, p in b['policies'].items():
            if p['body']['kind'] == 'work':
                self.assertEqual(p['body'].pop('faultSource'), old['policies'][identity]['body']['retry']['faultSource'] if identity == 'payment_work' else None)
        for cell in [CAPTURE, REQUEST, CATEGORY]:
            self.assertEqual(b['data'][cell]['scope'], 'payment_recovery')
            self.assertEqual(b['data'][cell]['protection'], old['data']['payment_result']['protection'])
            self.assertEqual(b['data'][cell]['invalidates'], {'payment_observe':True, CLASSIFY:True, 'payment_recovered':True})
            self.assertNotIn('initial', b['data'][cell]); del b['data'][cell]
        node = b['nodes'].pop(CLASSIFY)
        self.assertEqual(node['reads'], {CAPTURE:True, REQUEST:True}); self.assertEqual(node['writes'], {CATEGORY:True})
        self.assertEqual(node['assignments'], {CATEGORY:CLASSIFY+'_assignment'})
        self.assertEqual(b['expressions'].pop(CLASSIFY+'_assignment')['body'], classification(old['policies']['payment_work']['body']['completion']))
        self.assertEqual(b['flows'].pop(CLASSIFY+'_flow'), {'source':CLASSIFY, 'outcome':'next', 'target':'payment_recovered'})
        flows = [f for f in b['flows'].values() if f['source'] == 'payment_observe']
        self.assertEqual(len(flows), 1); self.assertEqual(flows[0]['target'], CLASSIFY); flows[0]['target'] = 'payment_recovered'
        self.assertEqual(b, old)

    def test_all_eleven_actual_current_views_preserve_conditional_lifetime_and_coverage(self):
        p = package(); m = load(BASE/'manifest.json')
        self.assertEqual([r['stage'] for r in m['semanticStageResults']], STAGES)
        for stage in STAGES:
            result = invoke(p, stage); self.assertEqual(result.returncode, 0, (stage, result.stderr))
            report = json.loads(result.stdout)
            self.assertEqual(report, load(BASE/('reports/'+stage+'.json')))
            self.assertIs(report['executable'], False)
            if stage in STAGES[5:]:
                self.assertEqual(len(report['isolatedFollowups']), 2)
                row, = report['ownedLocalFollowups']
                self.assertEqual(row['originalRequest'], {'origin':'payment_request_step', 'result':REQUEST})
                self.assertEqual(row['captureSites'], ['payment_observe'])
                self.assertEqual(set(row['localCells']), {CAPTURE,REQUEST,CATEGORY})
                self.assertTrue(report['conditionalOwnedLocalChecks']['conditional'])
                self.assertIn({'kind':'cell', 'owner':'payment_service', 'site':'current_claim', 'providerRole':None}, report['conditionalOwnedLocalChecks']['requiredProtectionProvenance'])
            if stage == STAGES[-1]:
                self.assertEqual(report['retainedRequestScopes'], ['payment_recovery'])
                self.assertIn('payment_observe', report['uncoveredNodes']); self.assertIn(CLASSIFY, report['uncoveredNodes'])
                self.assertNotIn('ownedContinuations', report)

    def test_original_guard_corpus_is_repinned_only_and_all_original_artifacts_stay_identical(self):
        guard = load(ROOT/'crates/semantic-check/tests/conformance/rp01-guard-snapshots.json')
        content = {k:v for k,v in guard.items() if k not in ['candidateRevision','candidateDigest']}
        self.assertEqual(sha256(rfc8785.dumps(content)).hexdigest(), 'b0becff0544aa8a5a0610c0b0c7c1ae4ddda21211d77d57566f4364adff5897d')
        raw = (BASE/'candidate.ir.json').read_bytes()
        self.assertEqual(guard['candidateRevision'], json.loads(raw)['revision'])
        self.assertEqual(guard['candidateDigest'], 'sha256:'+sha256(raw).hexdigest())
        old = load(HISTORY/'candidate.ir.json')['body']; new = json.loads(raw)['body']
        for case in guard['cases']:
            s = case['subject']; a,b = old[s['map']][s['id']],new[s['map']][s['id']]
            self.assertEqual(a[s['field']] if s['map']=='expressions' else a['body'][s['field']], b[s['field']] if s['map']=='expressions' else b['body'][s['field']])
        m = load(HISTORY/'manifest.json')
        for r in m['resources']:
            self.assertEqual('sha256:'+sha256((ROOT/r['path']).read_bytes()).hexdigest(), r['revision'])
        for entry in m['inputArtifacts']:
            self.assertEqual((BASE/entry['path']).read_bytes(), (HISTORY/entry['path']).read_bytes())
        self.assertTrue(all(s['status']=='Partial' for s in load(BASE/'manifest.json')['scenarios']))

    def test_compiled_classifier_equals_all_thirteen_pure_cases_with_explicit_official_ids(self):
        corpus = load(ROOT/'crates/semantic-check/tests/conformance/rp01-late-classification.json')
        def official(v):
            if isinstance(v,dict):
                if v.get('op')=='literal':return v
                if v.get('op')=='read':return {'op':'read','cell':{'audit_payment_capture':CAPTURE,'audit_payment_request':REQUEST}[v['cell']]}
                return {k:official(c) for k,c in v.items()}
            if isinstance(v,list):return [official(c) for c in v]
            return v
        b = load(BASE/'candidate.ir.json')['body']; expr = b['expressions'][CLASSIFY+'_assignment']['body']
        self.assertEqual(expr, official(corpus['expression'])); self.assertEqual(len(corpus['cases']), 13)
        self.assertEqual(leaves(expr), ({CAPTURE,REQUEST},set()))
        self.assertIn('not authenticated observations', corpus['premise'])
        self.assertEqual(b['scopes']['payment_recovery']['outputs'], {})
        self.assertEqual(b['nodes'][CLASSIFY]['writes'], {CATEGORY:True})

    def test_genuine_current_provider_request_and_invalidation_refusals(self):
        for change, stage, code in [('provider','check',b'wait-capture-provider'), ('request','check',b'retained-request-readonly'), ('invalidation','check-concurrency',b'owned-local-invalidation-coverage')]:
            s,l,r = deepcopy(package())
            if change=='provider':
                index = next(i for i,row in enumerate(r) if row[0]=='urn:choreoform:benchmarks:rp01:payment-terminal:0.1.0')
                d = json.loads(r[index][2]); d['provider'] = next({'id':i,'revision':rev} for i,rev,_ in r if i=='urn:choreoform:benchmarks:rp01:notice:0.1.0')
                raw = encoded(d); revision='sha256:'+sha256(raw).hexdigest(); r[index]=(d['id'],revision,raw)
                next(row for row in l['resources'] if row['binding']=='payment_terminal_binding')['digest']=revision
            elif change=='request':
                self.assertEqual(s.count('Retain the original request from "Request Payment" and record in "Original payment request".'),1)
                before=s
                s=s.replace('Retain the original request from "Request Payment" and record in "Original payment request".', 'Retain the original request from "Request Payment" and record in "Late payment evidence".')
            else:
                start=s.index('Keep "Original payment request"'); end=s.index('End data.',start)
                before=s
                self.assertEqual(s[start:end].count('Invalidate ["Observe Payment", "Classify late payment", "Finish Payment recovery"].'),1)
                s=s[:start]+s[start:end].replace('Invalidate ["Observe Payment", "Classify late payment", "Finish Payment recovery"].','Invalidate empty.')+s[end:]
            if change!='provider':self.assertNotEqual(s,before)
            result=invoke((s,l,r),stage)
            self.assertEqual(result.returncode,1,(change,result.stderr));self.assertEqual(result.stdout,b'');self.assertIn(code,result.stderr)

    def test_current_wire_profile_integrity_format_cycle_and_budgets_are_explicit(self):
        p=package(); m=load(BASE/'manifest.json'); candidate=load(BASE/'candidate.ir.json')
        self.assertEqual((p[1]['profile'],p[1]['version'],m['coreProfile'],candidate['version']), ('0.18.0','0.17.0','0.12.0','0.4.0'))
        jsonschema.Draft202012Validator(load(ROOT/'schemas/ir/definition-0.4.schema.json')).validate(candidate)
        self.assertEqual((len(p[0].encode()),len(grammar.lex(p[0]))), (55453,9455))
        self.assertLess(len(grammar.lex(p[0])),16384)
        for key,value in grammar.package_digests(p[0].encode(), (BASE/'bindings.json').read_bytes()).items(): self.assertEqual(value,m[key+'Digest'])
        result=invoke(p,'format');self.assertEqual(result.returncode,0,result.stderr)
        formatted=json.loads(result.stdout);q=(formatted['source'],json.loads(formatted['companion']),p[2])
        result=invoke(q);self.assertEqual(result.returncode,0,result.stderr);self.assertEqual(json.loads(result.stdout),candidate)
        self.assertIn('tools/check_rp01_capture_checkpoint.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())
        for path in [BASE/'payment.terminal-source.json', BASE/'reports/invalidation-requirements.json']:
            self.assertEqual(path.with_name(path.name+'.license').read_text(), 'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n')


if __name__=='__main__':unittest.main()
