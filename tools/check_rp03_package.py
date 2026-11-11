# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Whole RP-03 draft accountability; no semantic admission or process traces."""
from hashlib import sha256
import json
from pathlib import Path
import unittest

from authoring_probe import lex,validate_ledger,package_digests
from build_rp03_benchmark import artifacts,BASE
from check_metadata_authoring import invoke
from check_static_contracts import ROOT


def load(path):return json.loads(path.read_bytes())


def load_package(base):
    m=load(base/'manifest.json')
    def resource(path):
        prefix='docs/benchmarks/rp03/'
        return (base/path.removeprefix(prefix)).read_bytes() if path.startswith(prefix) else (ROOT/path).read_bytes()
    return ((base/'source.choreo').read_text(),load(base/'bindings.json'),
        [(r['id'],r['revision'],resource(r['path'])) for r in m['resources']])


def package():return load_package(BASE)
def historical_package():return load_package(BASE/'history/source-0.14')
def pre_instance_package():return load_package(BASE/'history/source-0.16-pre-instances')


class OrderPackageEvidence(unittest.TestCase):
    def test_actual_generation_keeps_exact_candidate_and_only_three_passed_partial_reports(self):
        generated=artifacts()
        for name,raw in generated.items():self.assertEqual((BASE/name).read_bytes(),raw,name)
        m=load(BASE/'manifest.json');p=package();r=invoke(p)
        self.assertEqual(r.returncode,0,r.stderr)
        self.assertEqual(json.loads(r.stdout),load(BASE/'candidate.ir.json'))
        self.assertEqual(m['lowering'],{'result':'passed'})
        self.assertEqual([row['result'] for row in m['semanticStageResults'][:3]],['passed']*3)
        self.assertTrue(all(row['result']=='refused' for row in m['semanticStageResults'][3:]))
        self.assertIs(m['executable'],False)

    def test_measured_current_files_and_companion_keep_candidate_admission_separate(self):
        s,l,_=package();m=load(BASE/'manifest.json');validate_ledger(l)
        self.assertEqual((len(s.encode()),sum(len(lex(line)) for line in s.splitlines())),(554331,82373))
        self.assertEqual(m['sourceBytes'],len(s.encode()));self.assertEqual(m['measuredTokens'],sum(len(lex(line)) for line in s.splitlines()))
        self.assertEqual((l['profile'],l['version'],m['coreProfile'],m['intendedWireVersion']),('0.16.0','0.15.0','0.12.0','0.4.0'))
        for key,value in package_digests(s.encode(),(BASE/'bindings.json').read_bytes()).items():self.assertEqual(m[key+'Digest'],value)
        self.assertGreater(m['measuredTokens'],16384);self.assertLess(m['sourceBytes'],1024*1024)

    def test_exact_resources_preserve_separate_effects_and_exclude_unknown_from_terminals(self):
        m=load(BASE/'manifest.json');providers={};sources=[]
        for entry in m['inputArtifacts']:self.assertEqual('sha256:'+sha256((BASE/entry['path']).read_bytes()).hexdigest(),entry['digest'])
        for r in m['resources']:
            raw=(ROOT/r['path']).read_bytes();self.assertEqual('sha256:'+sha256(raw).hexdigest(),r['revision'])
            if r['path'].endswith('.provider.json'):providers[r['id']]=json.loads(raw)
            elif r['path'].endswith('.terminal-source.json'):sources.append(json.loads(raw))
        self.assertEqual(len(providers),23);self.assertEqual(len(sources),23)
        for source in sources:
            provider=providers[source['provider']['id']]
            self.assertEqual(source['profile']['id'],'urn:choreoform:observation-source-profile:provider-terminal-facts:0.1.0')
            self.assertEqual(provider['observations']['unknown']['state'],'unknown')
            self.assertIsNone(provider['observations']['unknown']['result'])
            self.assertEqual(provider['idempotency'],'none' if provider['effectClass']=='read-only' else 'stable-key')
        self.assertEqual({p['id'].split(':')[-2] for p in providers.values() if p['effectClass']=='read-only'},{'deliver-a','deliver-b'})
        self.assertEqual(load(BASE/'clock.json')['source']['id'],'urn:choreoform:benchmarks:rp03:clock-source:0.1.0')

    def test_all_original_scenarios_and_real_declarations_stay_partial_and_visible(self):
        m=load(BASE/'manifest.json');l=load(BASE/'bindings.json');ids={d['id'] for d in l['declarations']}
        self.assertEqual([s['id'] for s in m['scenarios']],['RP-03-A','RP-03-B','RP-03-C','RP-03-D'])
        for scenario in m['scenarios']:
            self.assertEqual(scenario['status'],'Partial');self.assertTrue(set(scenario['declarations'])<=ids)
        for key in ['reserve_a','reserve_b','authorize','void','capture','refund','handover_a','handover_b','deliver_a','deliver_b','stop_a','stop_b','intercept_a','intercept_b']:
            self.assertIn(key+'_activity',ids);self.assertIn(key+'_observe',ids);self.assertIn(key+'_retained',ids)
        self.assertIn('settlement_direction_ask',ids);self.assertIn('surviving_exception_ask',ids)
        self.assertTrue(l['annotations']['synthetic']);self.assertIn('two explicit',l['annotations']['variant'])

    def test_emitted_receipt_rules_keep_every_request_field_and_typed_negative_case(self):
        rules=load(BASE/'receipt-rules.json');cases=load(BASE/'receipt-cases.json')
        self.assertIn('not compiled full source',rules['premise']);self.assertIn('never accepted observations',cases['premise'])
        self.assertEqual(len(rules['rules']),23);self.assertEqual(len(cases['cases']),22)
        self.assertEqual(sum(c['expected'] is True for c in cases['cases']),3)
        required={'operation','phase','order ID','order revision','scope key','scope revision','request revision','amount','currency','basis reference','address revision','target quantity','target warehouse'}
        def fields(v):
            if isinstance(v,dict):return ({v['name']} if v.get('op')=='field' else set())|set().union(*(fields(c) for c in v.values()))
            if isinstance(v,list):return set().union(*(fields(c) for c in v))
            return set()
        for rule in rules['rules'].values():self.assertTrue(required<=fields(rule))
        guards=load(BASE/'guard-cases.json');self.assertEqual(len(guards['cases']),8)
        for identity in ['default-stock-failure-does-not-offer-partial','unknown-stop-does-not-authorize-second-release','capture-alone-proves-no-refund-eligibility','unknown-interception-does-not-authorize-refund','wrong-capture-total-does-not-authorize-refund']:
            self.assertIs(next(c for c in guards['cases'] if c['id']==identity)['expected'],False)
        for id in ['reserve_a-wrong-target-quantity','deliver_a-wrong-target-warehouse','capture-wrong-amount','capture-wrong-request-revision','capture-wrong-currency']:
            self.assertIs(next(c for c in cases['cases'] if c['id']==id)['expected'],False)

    def test_rp01_current_history_and_guard_bytes_stay_frozen(self):
        base=ROOT/'docs/benchmarks/rp01';paths=sorted(p for p in base.rglob('*') if p.is_file());h=sha256()
        for p in paths:
            raw=p.read_bytes();h.update(p.relative_to(base).as_posix().encode()+b'\0'+len(raw).to_bytes(8,'big')+raw)
        self.assertEqual(len(paths),113);self.assertEqual(h.hexdigest(),'d3aad696ec8ec3bc325a42b7467c8725c73bb15f1c8620b58fc76531d2f1e592')
        guard=load(ROOT/'crates/semantic-check/tests/conformance/rp01-guard-snapshots.json')
        self.assertEqual((len(guard['cases']),len(guard['values'])),(44,46))
        self.assertEqual(guard['candidateRevision'],'sha256:55413bc5be721c103f56bd61a30bbf829e3cc97b1f867157e554baec4470e285')

    def test_new_shared_surface_encodings_lower_in_a_bounded_native_witness(self):
        # Pure source-encoding witness, not a truncated order or an admission.
        from benchmarks.rp03_source import new_package, Scope, literal, AMOUNT, record, variant, read
        from benchmarks.rp01_resources import nominal
        p=new_package(historical=True);root=Scope(p,'order','Formatter witness',None,'Done')
        tp=next({'id':r['id'],'revision':r['revision']} for r in load(BASE/'manifest.json')['resources'] if r['path'].endswith('/types.json'))
        p.resource('types_binding','Order types','named-type',tp)
        root.actor('author','Author','No operational authority')
        root.policy('no_authority','actor','No operational authority','Permit when (false).')
        root.policy('access','protection','Order access','Permit access when ((the fact "purpose") equals ("fulfillment")).')
        root.policy('settle','settlement','Settle','Unfinished work cancel.')
        root.policy('cancel','cancel','Stop work','Authority "Author". For children use "Settle".')
        root.policy('faults','faults','Unhandled faults','Handle empty.')
        root.policy('close','closure','Ordinary close','Fully terminal.')
        root.policy('race','race','Acceptance order','Accept in acceptance order.')
        choice=variant({'ready':record(amount=AMOUNT)})
        root.data('scaled','Scaled',AMOUNT,literal(AMOUNT,'-1'))
        root.data('choice','Choice',choice,{'op':'variant','type':choice,'tag':'ready','value':{'op':'record','fields':{'amount':literal(AMOUNT,'0')}}})
        order_type=nominal(tp,'Order');representation=load(BASE/'types.json')['definitions']['Order']['representation']
        root.data('raw','Raw order',representation)
        root.data('wrapped','Wrapped order',order_type,{'op':'wrap','type':order_type,'value':read('Raw order')})
        root.step('done','Done','Finish as "complete".')
        source='Choreoform authoring "0.14.0".\n'+root.render()
        supplied=[(tp['id'],tp['revision'],(BASE/'types.json').read_bytes())]
        result=invoke((source,p.ledger,supplied));self.assertEqual(result.returncode,0,result.stderr)
        b=json.loads(result.stdout)['body']
        self.assertEqual(b['expressions']['scaled_initial']['body'],{'op':'literal','type':AMOUNT,'value':'-1'})
        self.assertEqual(b['expressions']['choice_initial']['body']['op'],'variant')
        self.assertEqual(b['expressions']['choice_initial']['body']['value']['fields']['amount']['value'],'0')
        self.assertEqual(b['expressions']['wrapped_initial']['body']['op'],'wrap')

    def test_licensing_ci_and_private_pure_harness_keep_evidence_boundaries(self):
        for path in BASE.rglob('*.json'):
            self.assertEqual(path.with_name(path.name+'.license').read_text(),'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n')
        ci=(ROOT/'.github/workflows/rust-portability.yml').read_text()
        self.assertIn('tools/build_rp03_benchmark.py --check',ci);self.assertIn('tools/check_rp03_package.py',ci)
        self.assertIn('#[cfg(test)]\nmod rp03_receipts;',(ROOT/'crates/semantic-check/src/expression.rs').read_text())


if __name__=='__main__':unittest.main()
