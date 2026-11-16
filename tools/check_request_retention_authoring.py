# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Readable original-input declarations; no actual requests or owned storage."""
from copy import deepcopy
import json
from types import SimpleNamespace
import unittest
import authoring_probe as grammar
from benchmarks.rp01_source import Package,Scope,type_source
from benchmarks.rp01_graph import PEOPLE
from benchmarks.rp01_resources import BOOLEAN,encoded,record,TEXT
from check_handler_entry_authoring import package as history_package,oversized_package
from check_metadata_authoring import invoke
from check_wait_capture_authoring import STAGES,source_resource
from check_terminal_observation_sources import normalized_type
from check_static_contracts import ROOT

CORE8={'contract':'urn:choreoform:contracts:core-profile:0.8.0','digest':'sha256:edc8195ba0c31536ab2cdc95089e8aecc3ff402489639b4435277f0a38e7b603','dialect':'core'}
CLAUSE='Retain the original request from "Request Payment" and record in "Original request".'

def promote(s,l):
    s=s.replace('authoring "0.10.0"','authoring "0.11.0"',1);l=deepcopy(l);l.update(profile='0.11.0',version='0.10.0');l['contracts']['core']=CORE8.copy();return s,l

def package():
    s,l,r=history_package();s,l=promote(s,l)
    p=Package();p.ledger=l;p.used={l['definition']}|{v['id'] for k in ['declarations','symbols','generated','retired'] for v in l[k]}
    scope=SimpleNamespace(package=p,id='payment_recovery',declarations=[])
    provider=json.loads(next(raw for i,_,raw in r if i=='urn:choreoform:benchmarks:rp01:payment:0.1.0'))
    caps=['Check the claim','Create the payable','Notify the employee'];invalidates=['"Observe Payment"','"Inspect original request"','"Finish Payment recovery"']
    Scope.data(scope,'original_request','Original request',provider['input'],invalidates=invalidates,participants=PEOPLE,capabilities=caps)
    Scope.data(scope,'request_copy','Request copy',provider['input'],initial={'op':'read','cell':'Original request'},participants=PEOPLE,capabilities=caps)
    revision={'op':'field','name':'revision','value':{'op':'unwrapNamed','value':{'op':'field','name':'claim','value':{'op':'read','cell':'Original request'}}}}
    Scope.expression(scope,'request_guard','Original revision is stable',BOOLEAN,{'op':'eq','left':revision,'right':revision})
    Scope.step(scope,'request_branch','Inspect original request','Choose by ["same" when "Original revision is stable"]; otherwise "other".',[('same','Finish Payment recovery'),('other','Finish Payment recovery')])
    start=s.index('Scope "Payment recovery" within "Payment".');end=s.index('End scope.',start)+len('End scope.')
    block=s[start:end];interface=block.index(' and races "Acceptance order".')+len(' and races "Acceptance order".')
    block=block[:interface]+'\n'+CLAUSE+'\n'+'\n'.join(scope.declarations)+'\n'+block[interface:]
    block=block.replace('On "observed", continue with "Finish Payment recovery".','On "observed", continue with "Inspect original request".')
    block=block.replace('Capabilities ["Create the payable"]. Invalidate empty.','Capabilities ["Create the payable"]. Invalidate ["Observe Payment", "Inspect original request", "Finish Payment recovery"].')
    return s[:start]+block+s[end:],p.ledger,r

def budget_package(count=130):
    s,l,r=oversized_package(count);s,l=promote(s,l)
    p=Package();p.ledger=l;p.used={l['definition']}|{v['id'] for k in ['declarations','symbols','generated','retired'] for v in l[k]}
    identity,revision,raw=next(v for v in r if v[0]=='urn:test:budget-provider');provider=json.loads(raw);provider_pin={'id':identity,'revision':revision}
    p.resource('retained_provider_binding','Retained provider','provider',provider_pin)
    source_resource(l,r,provider_pin,identity='urn:test:retained-terminal',binding='retained_terminal_binding',name='Retained terminal facts')
    root=SimpleNamespace(id='expense',name='Entry budget')
    owner=Scope(p,'retention_owner','Retention owner',root,'Retained request',closure='Retained close')
    owner.capability('retained_cap','Retained service','Retained provider',record(),TEXT,'Payment operator','Retained effects')
    owner.policy('retained_effects','effect','Retained effects','Effect is read-only. Idempotency is none. Reconcile under "Retained observation".')
    owner.policy('retained_work','work','Retained work','Instructions "Retained request". Authority "Payment operator". Attempts 1. Delays empty. Retry faults empty. Timeout none. Complete when (true). Compensates none.')
    owner.policy('retained_close','closure','Retained close','Reconcile in "Retained follow-up".')
    owner.policy('retained_wait','wait','Retained observation','Observe '+type_source(normalized_type(provider))+'. Match when (true). Authority "Payment operator". Timer none. On observation outcome "observed" of "Retain terminal" in "Retained follow-up". On timer none.')
    owner.data('retained_output','Retained output',TEXT,participants=PEOPLE)
    owner.expression('retained_input','Retained input',record(),{'op':'record','fields':{}})
    owner.step('retained_origin','Retained request','Request "Retained service" using "Retained input" under "Retained work" and record in "Retained output".',[('done','Owner complete')])
    owner.step('retained_done','Owner complete','Finish as "complete".')
    followup=Scope(p,'retained_followup','Retained follow-up',owner,'Retain terminal')
    followup.declarations.append('Retain the original request from "Retained request" and record in "Retained original".')
    followup.data('retained_original','Retained original',record(),participants=PEOPLE)
    followup.data('retained_result','Retained result',normalized_type(provider),participants=PEOPLE)
    followup.step('retained_observe','Retain terminal','Wait under "Retained observation" for observations from "Retained terminal facts" about "Retained request" and record in "Retained result".',[('observed','Retained complete')])
    followup.step('retained_finish','Retained complete','Finish as "complete".')
    owner.child(followup)
    return s.replace('End process.',owner.render()+'\nEnd process.'),p.ledger,r

class RequestRetentionAuthoringEvidence(unittest.TestCase):
    def test_readable_full_composition_and_all_native_views(self):
        p=package();grammar.recognize(p[0]);grammar.validate_ledger(p[1]);lowered=invoke(p);self.assertEqual(lowered.returncode,0,lowered.stderr);doc=json.loads(lowered.stdout)
        self.assertEqual(doc['version'],'0.4.0');self.assertEqual(doc['body']['scopes']['payment_recovery']['retainedRequests'],{'original_request':'payment_request_step'})
        for stage in STAGES:
            result=invoke(p,stage);self.assertEqual(result.returncode,0,(stage,result.stderr));view=json.loads(result.stdout);row,=view['declaredRetainedRequests'];self.assertEqual(row['revision'],doc['revision']);self.assertEqual((row['scope'],row['origin'],row['result']),('payment_recovery','payment_request_step','original_request'));self.assertTrue(row['hostRequirements'])
            self.assertEqual(len(view['declaredFaultEntries']),1);self.assertEqual(len(view['declaredWaitCaptures']),1)
            if 'ownedLocalFollowups' in view:
                owned,=view['ownedLocalFollowups'];self.assertEqual(owned['originalRequest'],{'origin':'payment_request_step','result':'original_request'});self.assertIn('request_branch',view['conditionalOwnedLocalChecks']['checkedNodes'])
            if stage=='check-reconciliation-availability':
                self.assertEqual(view['retainedRequestScopes'],['payment_recovery']);self.assertNotIn('request_branch',view['checkedReconciliationNodes']);self.assertIn('request_branch',view['uncoveredNodes'])

    def test_format_qualified_aliases_and_independent_package_hashing(self):
        p=package();doc=json.loads(invoke(p).stdout);formatted=json.loads(invoke(p,'format').stdout)
        self.assertEqual(json.loads(invoke((formatted['source'],json.loads(formatted['companion']),p[2])).stdout),doc)
        s,l,r=deepcopy(p);s=s.replace('"Original request"','"Requête d’origine"');next(v for v in l['declarations'] if v['id']=='original_request')['name']='Requête d’origine'
        s=s.replace('from "Request Payment" and','from "Request Payment" in "Payment" in "Expense reimbursement" and')
        self.assertEqual(json.loads(invoke((s,l,r)).stdout),doc);self.assertEqual(json.loads(invoke(p,'integrity').stdout),grammar.package_digests(p[0].encode(),encoded(p[1])))

    def test_empty_new_maps_and_older_profiles_keep_default_reports(self):
        old=history_package();s,l=promote(old[0],old[1]);new=(s,l,old[2]);doc=json.loads(invoke(new).stdout)
        self.assertTrue(all(v['retainedRequests']=={} for v in doc['body']['scopes'].values()))
        for stage in STAGES:
            for p in [old,new]:
                result=invoke(p,stage);self.assertEqual(result.returncode,0,result.stderr);view=json.loads(result.stdout);self.assertNotIn('declaredRetainedRequests',view);self.assertNotIn('retainedRequestScopes',view)
                for row in view.get('ownedLocalFollowups',[]):self.assertNotIn('originalRequest',row)

    def test_only_scope_grammar_changes_and_strict_version_pairing(self):
        old=grammar.load_grammar(ROOT/'docs/authoring/candidate-a-v0.10.ebnf');new=grammar.load_grammar(ROOT/'docs/authoring/candidate-a-v0.11.ebnf')
        self.assertEqual({k:v for k,v in old.items() if k!='scope'},{k:v for k,v in new.items() if k not in ['scope','retained_request']})
        p=package()
        for version in ['0.9.0','0.10.0','1.0.0']:
            with self.assertRaises(ValueError):grammar.recognize(p[0].replace('authoring "0.11.0"',f'authoring "{version}"'))
        for mutation,code in [('pair',b'ledger-profile'),('core',b'contract-pin')]:
            s,l,r=deepcopy(p)
            if mutation=='pair':l['version']='0.9.0'
            else:l['contracts']['core']['digest']='sha256:'+'0'*64
            result=invoke((s,l,r));self.assertEqual(result.returncode,1);self.assertEqual(result.stdout,b'');self.assertIn(code,result.stderr)

    def test_owner_capture_readonly_access_publication_and_invalidation_refusals(self):
        p=package()
        for mutation,stage,code in [('owner','lower',b'retained-request-owner'),('origin','check',b'retained-request-origin'),('initial','check',b'retained-request-readonly'),('access','check-concurrency',b'retained-request-access-precheck'),('public','check-concurrency',b'control-information-flow'),('invalidation','check-concurrency',b'owned-local-invalidation-coverage')]:
            s,l,r=deepcopy(p)
            if mutation=='owner':s=s.replace('and record in "Original request".','and record in "Current claim".')
            elif mutation=='origin':s=s.replace(CLAUSE,'Retain the original request from "Finish Payment" and record in "Original request".')
            elif mutation=='initial':
                a=s.index('Keep "Original request"');b=s.index('End data.',a)
                s=s[:a]+s[a:b].replace('Initially unavailable.','Initially (the value of "Request copy").')+s[b:];l['generated'].append({'id':'forged_original','owner':'original_request','role':'initial','key':None})
            elif mutation=='access':
                s=s.replace('from "Request Payment" and record in "Original request".','from "Request Payment" and record in "Original request". Rule for protection "Request access". Permit access when ((the value of "Original request") equals (the value of "Original request")). End policy.')
                a=s.index('Keep "Original request"');b=s.index('End data.',a);s=s[:a]+s[a:b].replace('under "Expense access"','under "Request access"')+s[b:];l['declarations'].append({'id':'request_access','kind':'policy','scope':'payment_recovery','name':'Request access'})
            elif mutation=='public':
                a=s.index('Keep "Request copy"');b=s.index('End data.',a);s=s[:a]+s[a:b].replace('Sensitivity "expense"','Sensitivity "public"')+s[b:]
            else:
                a=s.index('Keep "Original request"');b=s.index('End data.',a);s=s[:a]+s[a:b].replace('Invalidate ["Observe Payment", "Inspect original request", "Finish Payment recovery"].','Invalidate empty.')+s[b:]
            result=invoke((s,l,r),stage);self.assertEqual(result.returncode,1,(mutation,result.stderr));self.assertEqual(result.stdout,b'');self.assertIn(code,result.stderr,(mutation,result.stderr))

    def test_unvisited_declaration_and_shared_combined_report_budget(self):
        small=budget_package(1);grammar.recognize(small[0]);grammar.validate_ledger(small[1])
        for stage in STAGES:
            result=invoke(small,stage);self.assertEqual(result.returncode,0,(stage,result.stderr));view=json.loads(result.stdout);self.assertEqual(view['declaredRetainedRequests'][0]['origin'],'retained_origin');self.assertNotIn('retainedRequestScopes',view)
        p=budget_package();grammar.recognize(p[0]);grammar.validate_ledger(p[1]);self.assertLess(len(grammar.lex(p[0])),16384)
        result=invoke(p);self.assertEqual(result.returncode,0,result.stderr)
        for stage in STAGES:
            result=invoke(p,stage);self.assertEqual(result.returncode,1,(stage,result.stderr));self.assertEqual(result.stdout,b'');self.assertIn(b'report-budget',result.stderr)

if __name__=='__main__':unittest.main()
