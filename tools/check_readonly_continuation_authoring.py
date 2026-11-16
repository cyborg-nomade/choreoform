# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Readable conditional static evidence; never actual work or host acceptance."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest
import authoring_probe as grammar
from benchmarks.rp01_resources import encoded
from check_static_contracts import ROOT
from check_metadata_authoring import invoke

FIXTURES = ROOT / 'crates/authoring-frontend/tests/capture'
STAGES = ['check', 'check-control', 'check-protection', 'check-invalidation',
          'invalidation-requirements', 'check-concurrency', 'check-timing',
          'check-fault-routes', 'check-exceptional-availability',
          'check-reconciliation-links', 'check-reconciliation-availability']


def package():
    source = (FIXTURES / 'readonly-continuation.choreo').read_text()
    ledger = json.loads((FIXTURES / 'readonly-continuation.bindings.json').read_bytes())
    raw = json.loads((ROOT / 'crates/semantic-check/tests/conformance/owned-readonly-definition.json').read_bytes())
    supplied = [(r['id'], r['revision'], r['bytes'].encode()) for r in raw['resources']]
    supplied += [(r['id'], r['revision'], (ROOT / 'docs/ir/contracts' / (r['revision'].replace(':', '-') + '.txt')).read_bytes()) for r in raw['normative'] if 'core-profile' not in r['id'] and not r['id'].startswith('urn:choreoform:semantics:')]
    return source, ledger, supplied


def budget_package(count=130):
    source, ledger, supplied = package()
    def declaration(identity, kind, scope, name):
        ledger['declarations'].append({'id':identity,'kind':kind,'scope':scope,'name':name})
    def scope_symbol(identity):
        ledger['symbols'].append({'id':identity+'_outcome_complete','owner':identity,'role':'outcome','name':'complete','wire':'complete'})
    profile={'id':'urn:choreoform:fault-history-profile:provider-observations:0.1.0','revision':'sha256:2f580f46eb6ef9b7fff42906f6b0b5cafc6c5fa71ae14365828935bfe2da070c'}
    types={'id':'urn:choreoform:types:provider-fault-history:0.1.0','revision':'sha256:a2e177ce1dd4bad5b9cd68b74e6da7749ec2b04b68a39dae1bd72729e2aae7a7'}
    for pin in [profile,types]:supplied.append((pin['id'],pin['revision'],(ROOT/'docs/ir/contracts'/(pin['revision'].replace(':','-')+'.txt')).read_bytes()))
    declaration('history_types_resource','resource','main','History types')
    ledger['resources'].append({'binding':'history_types_resource','kind':'named-type','contract':types['id'],'digest':types['revision']})
    pin=next(p for p in ledger['resources'] if p['binding']=='fault_resource')
    declaration('unused_owner','scope','main','Unused history owner');scope_symbol('unused_owner')
    declaration('unused_done','node','unused_owner','Unused done')
    declaration('unused_faults','policy','unused_owner','Unused faults')
    lines=['Scope "Unused history owner" within "Main".','Start at "Unused done".','Inputs empty. Outputs empty. Outcomes ["complete"].','Use cancellation "Cancel", faults "Unused faults", closure "Close" and races "Race".','Step "Unused done". Finish as "complete". End step.']
    routes=[]
    for i in range(count):
        identity=f'handler{i}';caption=f'History handler {i}'
        h={'kind':'provider-fault-history','version':'0.1.0','id':f'urn:test:readonly-report:{i}:'+('h'*3900),'profile':profile,'source':{'id':pin['contract'],'revision':pin['digest']}}
        raw=encoded(h);revision='sha256:'+sha256(raw).hexdigest();supplied.append((h['id'],revision,raw))
        declaration(f'history_resource{i}','resource','main',f'History resource {i}')
        ledger['resources'].append({'binding':f'history_resource{i}','kind':'provider-fault-history','contract':h['id'],'digest':revision})
        declaration(identity,'scope','unused_owner',caption);scope_symbol(identity)
        declaration(f'history{i}','data',identity,'History')
        declaration(f'handler_done{i}','node',identity,'Handler done')
        ledger['symbols'].append({'id':f'history{i}_purpose_test','owner':f'history{i}','role':'purpose','name':'test','wire':'test'})
        lines += [f'Scope "{caption}" within "Unused history owner".','Start at "Handler done".','Inputs empty. Outputs empty. Outcomes ["complete"].','Use cancellation "Cancel", faults "Faults", closure "Close" and races "Race".',f'Receive fault history from "History resource {i}" and record in "History".','Keep "History" as type "Provider fault history" from "History types" under "Protection". Sensitivity "internal". Purposes ["test"]. Participants ["Operator"]. Capabilities ["Adapter", "Adapter lookup"]. Invalidate empty. Initially unavailable. End data.','Step "Handler done". Finish as "complete". End step.','End scope.']
        routes.append(f'"fault {i}" by "{caption}"')
    lines += ['Rule for faults "Unused faults". Handle ['+', '.join(routes)+']. End policy.','End scope.']
    source=source.removesuffix('End process.\n')+'\n'.join(lines)+'\nEnd process.\n'
    return source,ledger,supplied


class ReadonlyContinuationAuthoringEvidence(unittest.TestCase):
    def test_all_native_views_keep_declarations_and_separate_conditional_proof(self):
        p = package()
        for stage in STAGES:
            r = invoke(p, stage)
            self.assertEqual(r.returncode, 0, (stage, r.stderr))
            d = json.loads(r.stdout)
            self.assertIs(d['executable'], False)
            self.assertEqual(len(d['declaredRetainedRequests']), 2)
            if stage in STAGES[5:]:
                self.assertEqual(d['ownedContinuations'], [{'creator': 'run_owner', 'owner': 'root'}])
                self.assertEqual(len(d['ownedContinuationScopes']), 2)
                c = d['conditionalOwnedContinuationChecks']
                self.assertTrue(c['conditional'])
                self.assertIn('lookup', c['checkedNodes'])
                self.assertEqual(len(c['gates']), 7)
                self.assertTrue(all(g['hostRequirements'] for g in c['gates']))
            else:
                self.assertNotIn('conditionalOwnedContinuationChecks', d)
            if stage == STAGES[-1]:
                self.assertEqual(set(d['continuationScopes']), {'recovery', 'lookup_recovery'})
                self.assertNotIn('lookup', d['checkedReconciliationNodes'])
                self.assertIn('lookup', d['uncoveredNodes'])
                self.assertNotIn('retainedRequestScopes', d)

    def test_unchanged_grammar_schema_delta_and_independent_package_identity(self):
        p = package()
        grammar.recognize(p[0]); grammar.validate_ledger(p[1])
        self.assertEqual(grammar.load_grammar(ROOT/'docs/authoring/candidate-a-v0.12.ebnf'), grammar.load_grammar(ROOT/'docs/authoring/candidate-a-v0.11.ebnf'))
        old = json.loads((ROOT/'schemas/authoring/bindings-0.10.schema.json').read_bytes())
        new = json.loads((ROOT/'schemas/authoring/bindings-0.11.schema.json').read_bytes())
        new['$id'] = old['$id']; new['title'] = old['title']
        new['properties']['profile']['const'] = '0.11.0'; new['properties']['version']['const'] = '0.10.0'
        new['properties']['contracts']['properties']['core']['properties'] = old['properties']['contracts']['properties']['core']['properties']
        self.assertEqual(new, old)
        self.assertEqual(json.loads(invoke(p,'integrity').stdout), grammar.package_digests(p[0].encode(), encoded(p[1])))
        formatted = json.loads(invoke(p,'format').stdout)
        self.assertEqual(json.loads(invoke((formatted['source'], json.loads(formatted['companion']),p[2])).stdout), json.loads(invoke(p).stdout))
        s,l,r = deepcopy(p)
        s = s.replace('"Lookup recovery"', '"Fólio de recuperação"')
        next(v for v in l['declarations'] if v['id']=='lookup_recovery')['name']='Fólio de recuperação'
        self.assertEqual(json.loads(invoke((s,l,r)).stdout), json.loads(invoke(p).stdout))

    def test_raw_refusals_keep_actual_stage_and_utf8_clause_location(self):
        p = package()
        for change, stage, code in [('capture','check',b'retained-request-readonly'), ('input','check-concurrency',b'owned-continuation-query-input'), ('source','check-concurrency',b'owned-continuation-query-policy'), ('recipient','check-concurrency',b'owned-continuation-recipient'), ('invalidation','check-concurrency',b'owned-continuation-invalidation-coverage')]:
            s,l,r = deepcopy(p)
            if change=='capture': s=s.replace('and record in "Lookup request".', 'and record in "Lookup captured".')
            elif change=='input': s=s.replace('Let "Lookup input" mean (the value of "Original request")','Let "Lookup input" mean (record with empty)')
            elif change=='source': s=s.replace('Recognise faults from "Query faults". ','')
            else:
                start=s.index('Keep "Captured"' if change=='recipient' else 'Keep "Original request"')
                end=s.index('End data.',start)
                block=s[start:end]
                if change=='recipient': block=block.replace('Capabilities ["Adapter", "Adapter lookup"].','Capabilities ["Adapter"].')
                else: block=block[:block.index('Invalidate ')]+ 'Invalidate empty. Initially unavailable. '
                s=s[:start]+block+s[end:]
            result=invoke((s,l,r),stage)
            self.assertEqual(result.returncode,1,(change,result.stderr));self.assertEqual(result.stdout,b'');self.assertIn(code,result.stderr)
            if change=='capture':
                clause='Retain the original request from "Lookup" and record in "Lookup captured".'
                offset=s.encode().index(clause.encode())
                self.assertIn(f'source bytes {offset}..{offset+len(clause.encode())}'.encode(),result.stderr)

    def test_old_profile_is_not_upgraded_and_new_fields_stay_omitted(self):
        s,l,r = package()
        old=deepcopy(l);old.update(profile='0.11.0',version='0.10.0')
        core=ROOT/'docs/dialects/core-retained-request-v0.8.md'
        old['contracts']['core']={'contract':'urn:choreoform:contracts:core-profile:0.8.0','digest':'sha256:'+sha256(core.read_bytes()).hexdigest(),'dialect':'core'}
        result=invoke((s.replace('authoring "0.12.0"','authoring "0.11.0"'),old,r),'check')
        self.assertIn(b'retained-request-creator',result.stderr);self.assertEqual(result.stdout,b'')
        from check_request_retention_authoring import package as old_package
        older=old_package()
        source,ledger,resources=deepcopy(older)
        source=source.replace('authoring "0.11.0"','authoring "0.12.0"')
        ledger.update(profile='0.12.0',version='0.11.0');ledger['contracts']['core']=deepcopy(l['contracts']['core'])
        for candidate in [older,(source,ledger,resources)]:
            for stage in STAGES:
                result=invoke(candidate,stage);self.assertEqual(result.returncode,0,result.stderr)
                view=json.loads(result.stdout)
                for field in ['ownedContinuations','ownedContinuationScopes','conditionalOwnedContinuationChecks','continuationScopes']:self.assertNotIn(field,view)

    def test_valid_unvisited_history_composes_with_real_conditional_proof_and_report_budget(self):
        small=budget_package(1);grammar.recognize(small[0]);grammar.validate_ledger(small[1])
        for stage in STAGES:
            result=invoke(small,stage);self.assertEqual(result.returncode,0,(stage,result.stderr))
            if stage in STAGES[5:]:self.assertEqual(len(json.loads(result.stdout)['conditionalOwnedContinuationChecks']['gates']),7)
        p=budget_package();grammar.recognize(p[0]);grammar.validate_ledger(p[1]);self.assertLess(len(grammar.lex(p[0])),16384)
        result=invoke(p);self.assertEqual(result.returncode,0,result.stderr)
        for stage in STAGES:
            result=invoke(p,stage);self.assertEqual(result.returncode,1,(stage,result.stderr));self.assertEqual(result.stdout,b'');self.assertIn(b'report-budget',result.stderr)

    def test_explicit_pair_licensing_and_ci(self):
        p=package()
        for mutation in ['pair','pin']:
            s,l,r=deepcopy(p)
            if mutation=='pair':l['version']='0.10.0'
            else:l['contracts']['core']['digest']='sha256:'+'0'*64
            result=invoke((s,l,r));self.assertEqual(result.returncode,1);self.assertEqual(result.stdout,b'')
        notice='SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n'
        for path in [FIXTURES/'readonly-continuation.bindings.json',FIXTURES/'readonly-continuation.choreo',ROOT/'schemas/authoring/bindings-0.11.schema.json']:
            self.assertEqual(path.with_name(path.name+'.license').read_text(),notice)
        self.assertIn('tools/check_readonly_continuation_authoring.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__ == '__main__':
    unittest.main()
