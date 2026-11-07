# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Actual whole-source candidate/refusal evidence, not benchmark admission."""
from copy import deepcopy
from hashlib import sha256
import json
from pathlib import Path
import tempfile
import unittest

import rfc8785

import authoring_probe as grammar
import check_ir_fixtures as wire
from build_rp01_benchmark import artifacts, STAGES
from benchmarks.rp01_resources import types, providers
from check_static_contracts import ROOT, run
from check_metadata_authoring import full as unpatched_rp01, invoke as invoke_unpatched
from benchmarks.rp01_invalidation import MISSING_LINKS, declare_links
from benchmarks.rp01_faults import historical_v6, adopt

BASE = ROOT / 'docs/benchmarks/rp01'


def registry(manifest):
    return [(r['id'], r['revision'], ROOT / r['path']) for r in manifest['resources']]


class Rp01PackageEvidence(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.manifest = json.loads((BASE / 'manifest.json').read_bytes())
        cls.source = (BASE / 'source.choreo').read_bytes()
        cls.bindings = (BASE / 'bindings.json').read_bytes()
        cls.candidate = wire.load((BASE / 'candidate.ir.json').read_bytes())

    def test_only_exact_missing_links_change_the_body_and_all_identities_are_preserved(self):
        source, ledger, supplied = baseline = unpatched_rp01(payment_policy_revisions=True)
        hs,hl=historical_v6(self.source.decode(),json.loads(self.bindings))
        self.assertEqual(hl, ledger)
        old = json.loads(invoke_unpatched(baseline, 'lower').stdout)
        advice = json.loads(invoke_unpatched(baseline, 'invalidation-requirements').stdout)
        missing = {(o['site'], o['cell']) for o in advice['missingInvalidationObligations']}
        self.assertEqual(missing, {(site,cell) for cell, sites in MISSING_LINKS.items() for site in sites})
        self.assertEqual(len(missing),72)
        historical=json.loads(invoke_unpatched((hs,hl,supplied),'lower').stdout)
        before, after = deepcopy(old['body']), deepcopy(historical['body'])
        added = set()
        for cell, data in before['data'].items():
            old_links = set(data['invalidates'])
            new_links = set(after['data'][cell]['invalidates'])
            self.assertTrue(old_links <= new_links)
            added |= {(site,cell) for site in new_links - old_links}
            after['data'][cell]['invalidates'] = data['invalidates']
        self.assertEqual(added,missing)
        self.assertEqual(before,after)
        self.assertNotEqual(old['revision'],historical['revision'])
        self.assertEqual(declare_links(source,ledger),hs)
        self.assertEqual(len(grammar.lex(self.source.decode())),self.manifest['measuredTokens'])
        # The fixture assembler refuses a changed/already-patched baseline.
        with self.assertRaises(AssertionError):
            declare_links(hs,ledger)

    def test_explicit_fault_source_adoption_changes_only_accepted_versioned_fields(self):
        current=json.loads(self.bindings)
        hs,hl=historical_v6(self.source.decode(),current)
        self.assertEqual((len(hs.encode()),len(grammar.lex(hs))),(56843,9466))
        supplied=[(r['id'],r['revision'],(ROOT/r['path']).read_bytes()) for r in self.manifest['resources']]
        prior=json.loads(invoke_unpatched((hs,hl,supplied),'lower').stdout)
        self.assertEqual(prior['revision'],'sha256:d0bca38fb470cd0485b9a1cc22767182e6276669ca9ca131ac45d7af268269f3')
        before,after=deepcopy(prior['body']),deepcopy(self.candidate['body'])
        after['dialects']=before['dialects']
        source_pin=None
        for identity,p in after['policies'].items():
            if p['body']['kind']=='work':
                binding=p['body']['retry'].pop('faultSource')
                if identity=='payment_work':self.assertIsNotNone(binding);source_pin=binding
                else:self.assertIsNone(binding)
        self.assertEqual(after,before)
        provider=next({'id':i,'revision':rev} for i,rev,_ in supplied if i=='urn:choreoform:benchmarks:rp01:payment:0.1.0')
        s,l,raw,pin=adopt(hs,hl,provider)
        self.assertEqual((s,l),(self.source.decode(),current))
        self.assertEqual(raw,(BASE/'payment.fault-source.json').read_bytes());self.assertEqual(pin,source_pin)
        artifact=json.loads(raw);self.assertEqual(artifact['provider'],provider)
        self.assertEqual(artifact['emissions'],{'confirmed no effect':{'observations':{'no-effect':True},'evidence':'definitive-no-effect'},'provider unknown':{'observations':{'unknown':True},'evidence':'unresolved'}})
        for stage in STAGES[5:]:
            report=json.loads((BASE/('reports/'+stage+'.json')).read_bytes());row,=report['retryFaultBindings']
            self.assertEqual(row['source'],source_pin);self.assertEqual(row['provider'],provider);self.assertEqual(row['revision'],self.candidate['revision'])
            self.assertEqual(set(row['faults']),{'confirmed no effect'});self.assertTrue(row['hostRequirements'])
        fault=json.loads((BASE/'reports/check-fault-routes.json').read_bytes())
        self.assertEqual(fault['unhandledFaultCases'],2*len(fault['checkedFaultScopes']))
        with self.assertRaises(AssertionError):adopt(s,l,provider)

    def test_policy_guard_correction_preserves_all_six_old_clauses_and_only_adds_three(self):
        weak=json.loads(invoke_unpatched(unpatched_rp01(),'lower').stdout)['body']
        strong=json.loads(invoke_unpatched(unpatched_rp01(payment_policy_revisions=True),'lower').stdout)['body']
        before=weak['policies']['payment_commits']['body']['invariant']
        after=strong['policies']['payment_commits']['body']['invariant']
        def conjunction(ast):
            return conjunction(ast['left'])+conjunction(ast['right']) if ast['op']=='and' else [ast]
        old,new=conjunction(before),conjunction(after)
        self.assertEqual(len(old),6)
        self.assertEqual(new[:6],old)
        self.assertEqual(len(new),9)
        self.assertTrue(all(c['op']=='eq' and c['left']['name']==c['right']['name']=='policy revision' for c in new[6:]))
        strong['policies']['payment_commits']['body']['invariant']=before
        self.assertEqual(strong,weak)

    def test_generation_repeats_native_candidate_and_all_ten_actual_stage_results(self):
        generated = artifacts()
        for name, raw in generated.items():
            self.assertEqual((BASE / name).read_bytes(), raw, name)
        manifest = json.loads(generated['manifest.json'])
        self.assertIs(manifest['executable'], False)
        self.assertEqual([s['stage'] for s in manifest['semanticStageResults']], STAGES)
        self.assertEqual([s['result'] for s in manifest['semanticStageResults']], ['passed'] * 10)
        for stage in manifest['semanticStageResults']:
            if stage['result'] == 'passed':
                report = json.loads(generated[stage['report']])
                self.assertIs(report['executable'], False)
                self.assertEqual(report['revision'], self.candidate['revision'])
                self.assertEqual(stage['reportDigest'], 'sha256:' + sha256(generated[stage['report']]).hexdigest())
                continue
            self.fail('unexpected stage refusal: '+repr(stage))
        self.assertEqual(manifest['sourceDigest'], 'sha256:' + sha256(self.source).hexdigest())
        self.assertEqual(manifest['canonicalIR']['digest'], 'sha256:' + sha256(generated['candidate.ir.json']).hexdigest())

    def test_wire_revision_package_integrity_and_format_cycle_match_real_frontend(self):
        m, candidate = self.manifest, self.candidate
        # The wire oracle accepts only its historical dialect by default. Schema
        # and semantic revision checks here do not claim dialect admission.
        wire.jsonschema.Draft202012Validator(wire.SCHEMA).validate(candidate)
        self.assertEqual(candidate['revision'], wire.revision(candidate))
        self.assertEqual((BASE / 'candidate.ir.json').read_bytes(), rfc8785.dumps(candidate))
        grammar.recognize(self.source.decode('utf-8'))
        grammar.validate_ledger(json.loads(self.bindings))
        digests = grammar.package_digests(self.source, self.bindings)
        for key in ['source', 'bindings', 'package']:
            self.assertEqual(digests[key], m[key + 'Digest'])
        formatted = run('format', BASE / 'source.choreo', BASE / 'bindings.json', registry(m))
        self.assertEqual(formatted.returncode, 0, formatted.stderr)
        with tempfile.TemporaryDirectory() as directory:
            s, b = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
            pair = json.loads(formatted.stdout)
            s.write_text(pair['source'], encoding='utf-8')
            b.write_text(pair['companion'], encoding='utf-8')
            result = run('lower', s, b, registry(m))
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(json.loads(result.stdout), candidate)

    def test_shared_result_capability_envelope_reproduces_old_core_policy_cycle(self):
        _, _, type_pin = types()
        _, raw, pin = providers(type_pin, shared=True)['checks']
        historical_source,ledger = historical_v6(self.source.decode(),json.loads(self.bindings))
        ledger.update(profile='0.5.0', version='0.4.0')
        ledger['contracts']['core'].update(contract='urn:choreoform:contracts:core-profile:0.2.0', digest='sha256:5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623')
        next(r for r in ledger['resources'] if r['binding'] == 'checks_provider_binding')['digest'] = pin['revision']
        with tempfile.TemporaryDirectory() as directory:
            base = Path(directory)
            s, b, p = base / 'source.choreo', base / 'bindings.json', base / 'provider.json'
            s.write_text(historical_source.replace('authoring "0.6.0"','authoring "0.5.0"',1))
            b.write_text(json.dumps(ledger), encoding='utf-8')
            p.write_bytes(raw)
            supplied = [(i, pin['revision'], p) if i == pin['id'] else (i, digest, path) for i, digest, path in registry(self.manifest)]
            result = run('lower', s, b, supplied)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(result.stdout, b'')
        self.assertIn(b'policy-cycle', result.stderr)

    def test_mismatched_profile_resource_and_role_fail_before_any_authority_claim(self):
        ledger = json.loads(self.bindings)
        manager = next(d for d in ledger['declarations'] if d['id'] == 'manager_actor')
        with tempfile.TemporaryDirectory() as directory:
            s, b = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
            s.write_bytes(self.source.replace(b'authoring "0.7.0"', b'authoring "0.9.0"', 1))
            b.write_bytes(self.bindings)
            result = run('lower', s, b, registry(self.manifest))
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(result.stdout, b'')
            s.write_bytes(self.source)
            changed = deepcopy(ledger)
            changed['resources'][0]['digest'] = 'sha256:' + '0' * 64
            b.write_text(json.dumps(changed), encoding='utf-8')
            result = run('lower', s, b, registry(self.manifest))
            self.assertIn(b'resource-missing-or-ambiguous', result.stderr)
            b.write_bytes(self.bindings)
            s.write_bytes(self.source.replace(('Ask "' + manager['name'] + '" to decide').encode(), b'Ask "Expense types" to decide'))
            result = run('lower', s, b, registry(self.manifest))
            self.assertIn(b'reference-kind', result.stderr)
            self.assertEqual(result.stdout, b'')
        # A readable authority name is a reference, never a tested live principal.

    def test_all_four_scenarios_retain_real_graph_and_revision_guard_boundaries(self):
        b = self.candidate['body']
        ids = set().union(*(set(b[k]) for k in wire.MAPS))
        self.assertEqual([s['id'] for s in self.manifest['scenarios']], ['RP-01-A', 'RP-01-B', 'RP-01-C', 'RP-01-D'])
        for scenario in self.manifest['scenarios']:
            self.assertEqual(scenario['status'], 'Partial')
            self.assertTrue(set(scenario['declarations']) <= ids)
        self.assertEqual(b['nodes']['review_again']['kind'], 'repeat')
        self.assertEqual(b['nodes']['review_again']['body'], 'review_cycle')
        self.assertEqual(b['nodes']['correct_claim']['target'], 'employee_actor')
        self.assertEqual(b['nodes']['manager_review']['target'], 'manager_actor')
        self.assertEqual(b['nodes']['finance_review']['target'], 'finance_actor')
        self.assertEqual(b['scopes']['expense']['cancellation'], 'withdraw_request')
        guard = json.dumps(b['expressions']['may_pay']['body'])
        self.assertIn('policy revision', guard)
        self.assertIn('revision', guard)
        retry = b['policies']['payment_work']['body']['retry']
        self.assertEqual(retry['faults'], {'confirmed no effect': True})
        self.assertEqual(retry['maxAttempts'], 2)
        self.assertEqual(b['policies']['payment_close']['body']['mode'], 'reconcile')
        self.assertEqual(b['nodes']['payment_observe']['policy'], 'payment_wait')


if __name__ == '__main__':
    unittest.main()
