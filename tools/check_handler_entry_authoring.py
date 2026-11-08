# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Readable declarations and conditional native proof; no actual fault events."""
from copy import deepcopy
import json
from types import SimpleNamespace
import unittest

import authoring_probe as grammar
from benchmarks.rp01_source import Package,Scope
from benchmarks.rp01_graph import PEOPLE,foundations
from benchmarks.rp01_resources import encoded,pin,BOOLEAN,TEXT,record
from check_general_fault_authoring import package as general_package
from check_metadata_authoring import invoke
from check_static_contracts import ROOT
from check_wait_capture_authoring import STAGES

CORE7={'contract':'urn:choreoform:contracts:core-profile:0.7.0','digest':'sha256:e286e54441b3d785ae823eef359c12d64fb8405829c70709227282e5af62f54b','dialect':'core'}
HISTORY_PROFILE={'id':'urn:choreoform:fault-history-profile:provider-observations:0.1.0','revision':'sha256:2f580f46eb6ef9b7fff42906f6b0b5cafc6c5fa71ae14365828935bfe2da070c'}
HISTORY_TYPES={'id':'urn:choreoform:types:provider-fault-history:0.1.0','revision':'sha256:a2e177ce1dd4bad5b9cd68b74e6da7749ec2b04b68a39dae1bd72729e2aae7a7'}
CLAUSE='Receive fault history from "Payment history" and record in "Fault history".'


def promote(source,ledger):
    source=source.replace('authoring "0.9.0"','authoring "0.10.0"',1)
    ledger=deepcopy(ledger);ledger.update(profile='0.10.0',version='0.9.0');ledger['contracts']['core']=CORE7.copy()
    return source,ledger


def package(capture=True):
    source,ledger,resources=general_package(capture);source,ledger=promote(source,ledger)
    p=Package();p.ledger=ledger;p.used={ledger['definition']}|{v['id'] for k in ['declarations','generated','symbols','retired'] for v in ledger[k]}
    for identity in [HISTORY_PROFILE,HISTORY_TYPES]:
        resources.append((identity['id'],identity['revision'],(ROOT/'docs/ir/contracts'/(identity['revision'].replace(':','-')+'.txt')).read_bytes()))
    payment=next({'id':i,'revision':r} for i,r,_ in resources if i=='urn:choreoform:benchmarks:rp01:payment-faults:0.1.0')
    descriptor={'kind':'provider-fault-history','version':'0.1.0','id':'urn:test:readable-payment-history','profile':HISTORY_PROFILE,'source':payment}
    raw=encoded(descriptor);history=pin(descriptor['id'],raw);resources.append((history['id'],history['revision'],raw))
    p.resource('payment_history_binding','Payment history','provider-fault-history',history)
    p.resource('history_types_binding','History types','named-type',HISTORY_TYPES)
    parent=SimpleNamespace(id='payment_service',name='Payment')
    handler=Scope(p,'provider_handler','Fault handling',parent,'Inspect history')
    history_type={'kind':'named','contract':HISTORY_TYPES,'name':'Provider fault history'}
    caps=['Check the claim','Create the payable','Notify the employee']
    handler.declarations.append(CLAUSE)
    handler.data('fault_history','Fault history',history_type,invalidates=['"Inspect history"'],participants=PEOPLE,capabilities=caps)
    handler.data('history_copy','History copy',history_type,initial={'op':'read','cell':'Fault history'},participants=PEOPLE,capabilities=caps)
    handler.expression('history_guard','History has expected type',BOOLEAN,{'op':'eq','left':{'op':'field','name':'type','value':{'op':'unwrapNamed','value':{'op':'read','cell':'Fault history'}}},'right':{'op':'literal','type':{'kind':'text'},'value':'provider unknown'}})
    handler.step('history_branch','Inspect history','Choose by ["expected" when "History has expected type"]; otherwise "unexpected".', [('expected','Done'),('unexpected','Done')])
    handler.step('handler_done','Done','Finish as "complete".')
    p.declaration('payment_faults','policy','payment_service','Payment fault handling')
    start=source.index('Scope "Payment" within "Expense reimbursement".')
    end=source.index('Scope "Notice" within "Expense reimbursement".',start)
    block=source[start:end];block=block.replace('faults "Unhandled faults"','faults "Payment fault handling"',1)
    offset=block.rindex('End scope.')
    addition='Rule for faults "Payment fault handling". Handle ["provider unknown" by "Fault handling"]. End policy.\n'+handler.render().replace('from "Expense types"','from "History types"')+'\n'
    block=block[:offset]+addition+block[offset:]
    return source[:start]+block+source[end:],p.ledger,resources


def oversized_package(count=130):
    p=Package();p.ledger.update(profile='0.10.0',version='0.9.0');p.ledger['contracts']['core']=CORE7.copy()
    root=Scope(p,'expense','Entry budget',None,'Done');foundations(root);root.step('done','Done','Finish as "complete".')
    protection={'policy':'expense_access','sensitivity':'expense','purposes':{'reimbursement':True},'participants':{k+'_actor':True for k in ['employee','manager','finance','operator','clock']},'capabilities':{}}
    provider={'kind':'provider-contract','version':'0.1.0','id':'urn:test:budget-provider','input':record(),'output':TEXT,'outcomes':{'done':True},'observations':{'done':{'state':'succeeded','result':TEXT,'outcome':'done'},'unknown':{'state':'unknown','result':None,'outcome':None}},'resultProtection':protection,'effectClass':'read-only','idempotency':'none'}
    resources=[]
    def artifact(value):
        raw=encoded(value);identity=pin(value['id'],raw);resources.append((identity['id'],identity['revision'],raw));return identity
    provider_pin=artifact(provider)
    source_profile={'id':'urn:choreoform:fault-source-profile:provider-observations:0.1.0','revision':'sha256:f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0'}
    source=artifact({'kind':'provider-fault-source','version':'0.1.0','id':'urn:test:budget-faults','profile':source_profile,'provider':provider_pin,'emissions':{'uncertain':{'observations':{'unknown':True},'evidence':'unresolved'}}})
    for identity in [source_profile,HISTORY_PROFILE,HISTORY_TYPES]:
        resources.append((identity['id'],identity['revision'],(ROOT/'docs/ir/contracts'/(identity['revision'].replace(':','-')+'.txt')).read_bytes()))
    p.resource('history_types_binding','History types','named-type',HISTORY_TYPES)
    owner=Scope(p,'unused_owner','Unused owner',root,'Owner done');owner.step('owner_done','Owner done','Finish as "complete".')
    routes=[]
    for i in range(count):
        h=artifact({'kind':'provider-fault-history','version':'0.1.0','id':f'urn:test:{i}:'+('h'*3900),'profile':HISTORY_PROFILE,'source':source})
        p.resource(f'history_binding{i}',f'History {i}','provider-fault-history',h)
        handler=Scope(p,f'handler{i}',f'Handler {i}',owner,f'Handler done {i}')
        handler.declarations.append(f'Receive fault history from "History {i}" and record in "History".')
        handler.data(f'history{i}','History',{'kind':'named','contract':HISTORY_TYPES,'name':'Provider fault history'},participants=PEOPLE)
        handler.step(f'handler_done{i}',f'Handler done {i}','Finish as "complete".')
        owner.child(handler);routes.append(f'"fault {i}" by "Handler {i}"')
    owner.policy('unused_faults','faults','Unused faults','Handle ['+', '.join(routes)+'].')
    root.declarations.append(owner.render().replace('faults "Unhandled faults"','faults "Unused faults"',1))
    return 'Choreoform authoring "0.10.0".\n'+root.render().replace('from "Expense types"','from "History types"'),p.ledger,resources


class HandlerEntryAuthoringEvidence(unittest.TestCase):
    def test_readable_full_package_and_all_stages_keep_conditional_evidence_separate(self):
        p=package();grammar.recognize(p[0]);grammar.validate_ledger(p[1])
        lowered=invoke(p);self.assertEqual(lowered.returncode,0,lowered.stderr);doc=json.loads(lowered.stdout)
        self.assertEqual(doc['version'],'0.3.0');self.assertEqual(doc['body']['scopes']['provider_handler']['faultInput']['result'],'fault_history')
        for stage in STAGES:
            result=invoke(p,stage);self.assertEqual(result.returncode,0,(stage,result.stderr));report=json.loads(result.stdout)
            self.assertEqual(len(report['declaredFaultEntries']),1);self.assertEqual(len(report['declaredFaultHistories']),1)
            if stage in ['check-exceptional-availability','check-reconciliation-links','check-reconciliation-availability']:
                typed=report['conditionalFaultEntries'];self.assertEqual(typed['coveredEntries'],['provider_handler']);self.assertIn('history_branch',typed['checkedNodes']);self.assertIn('history_branch',report['uncoveredNodes'])
                self.assertNotIn('history_branch',report.get('checkedExceptionalNodes',report.get('conditionalExceptionalNodes',[])))
                self.assertEqual(typed['incomingSources'][0]['sourceUseSite'],'payment_request_step')
            else:self.assertNotIn('conditionalFaultEntries',report)

    def test_format_aliases_integrity_and_nullable_legacy_boundaries(self):
        p=package(False);result=invoke(p);self.assertEqual(result.returncode,0,result.stderr);doc=json.loads(result.stdout)
        qualified=p[0].replace(CLAUSE,'Receive fault history from "Payment history" in "Expense reimbursement" and record in "Fault history" in "Fault handling" in "Payment" in "Expense reimbursement".')
        self.assertEqual(json.loads(invoke((qualified,p[1],p[2])).stdout),doc)
        formatted=json.loads(invoke(p,'format').stdout)
        self.assertEqual(json.loads(invoke((formatted['source'],json.loads(formatted['companion']),p[2])).stdout),doc)
        s,l,r=deepcopy(p);s=s.replace('"Payment history"','"Historique été"')
        next(v for v in l['declarations'] if v['id']=='payment_history_binding')['name']='Historique été'
        self.assertEqual(json.loads(invoke((s,l,r)).stdout),doc)
        self.assertEqual(json.loads(invoke(p,'integrity').stdout),grammar.package_digests(p[0].encode(),encoded(p[1])))
        s,l,r=general_package(False);s,l=promote(s,l);d=json.loads(invoke((s,l,r)).stdout)
        self.assertTrue(all(v['faultInput'] is None for v in d['body']['scopes'].values()))
        for stage in STAGES:
            for item in [(s,l,r),general_package(False)]:
                result=invoke(item,stage);self.assertEqual(result.returncode,0,result.stderr);out=json.loads(result.stdout)
                self.assertNotIn('declaredFaultEntries',out);self.assertNotIn('conditionalFaultEntries',out)

    def test_pair_grammar_and_explicit_resource_refusals_including_unused(self):
        p=package(False)
        old=grammar.load_grammar(ROOT/'docs/authoring/candidate-a-v0.9.ebnf');new=grammar.load_grammar(ROOT/'docs/authoring/candidate-a-v0.10.ebnf')
        self.assertEqual({k:v for k,v in old.items() if k!='scope'},{k:v for k,v in new.items() if k not in ['scope','fault_entry']})
        for version in ['0.7.0','0.8.0','0.9.0','1.0.0']:
            with self.assertRaises(ValueError):grammar.recognize(p[0].replace('authoring "0.10.0"',f'authoring "{version}"'))
        for mutation,code in [('pair',b'ledger-profile'),('pin',b'contract-pin'),('kind',b'resource-format'),('missing',b'resource-missing'),('corrupt',b'resource-digest'),('unused',b'resource-digest')]:
            s,l,r=deepcopy(p)
            if mutation=='pair':l['version']='0.8.0'
            elif mutation=='pin':l['contracts']['core']['digest']='sha256:'+'0'*64
            elif mutation=='kind':next(v for v in l['resources'] if v['binding']=='payment_history_binding')['kind']='provider'
            elif mutation=='missing':r=[v for v in r if v[0]!=HISTORY_PROFILE['id']]
            else:
                r=[(i,h,raw+b' ' if i=='urn:test:readable-payment-history' else raw) for i,h,raw in r]
                if mutation=='unused':s=s.replace(CLAUSE,'')
            result=invoke((s,l,r),'check');self.assertEqual(result.returncode,1,(mutation,result.stderr));self.assertEqual(result.stdout,b'');self.assertIn(code,result.stderr,(mutation,result.stderr))

    def test_source_owner_type_initialization_and_invalidation_refusals(self):
        p=package(False)
        for mutation,stage,code in [('owner','lower',b'fault-entry-owner'),('type','check',b'fault-entry-type'),('initial','check',b'fault-entry-readonly'),('source','check-exceptional-availability',b'fault-entry-source'),('invalidation','check-exceptional-availability',b'fault-entry-invalidation-coverage')]:
            s,l,r=deepcopy(p)
            if mutation=='owner':s=s.replace('and record in "Fault history".','and record in "Current claim".')
            elif mutation=='type':
                s=s.replace('as type "Provider fault history" from "History types"','as text')
                a=s.index('Let "History has expected type" mean ');b=s.index(' as truth value',a)
                s=s[:a]+'Let "History has expected type" mean (true)'+s[b:]
            elif mutation=='initial':
                s=s.replace('Invalidate ["Inspect history"]. Initially unavailable.','Invalidate ["Inspect history"]. Initially (the value of "History copy").')
                l['generated'].append({'id':'forged_initial','owner':'fault_history','role':'initial','key':None})
            elif mutation=='source':
                index=next(i for i,v in enumerate(r) if v[0]=='urn:test:readable-payment-history')
                history=json.loads(r[index][2]);history['source']=next({'id':i,'revision':h} for i,h,_ in r if i=='urn:test:authoring-notice-faults')
                raw=encoded(history);new_pin=pin(history['id'],raw);r[index]=(new_pin['id'],new_pin['revision'],raw)
                next(v for v in l['resources'] if v['binding']=='payment_history_binding')['digest']=new_pin['revision']
            else:s=s.replace('Invalidate ["Inspect history"].','Invalidate empty.')
            result=invoke((s,l,r),stage);self.assertEqual(result.returncode,1,(mutation,result.stderr));self.assertEqual(result.stdout,b'');self.assertIn(code,result.stderr,(mutation,result.stderr))

    def test_unexplored_entries_remain_declared_and_uncovered_with_shared_catalogue(self):
        p=oversized_package(2)
        first=next(v for v in p[1]['resources'] if v['binding']=='history_binding0')
        second=next(v for v in p[1]['resources'] if v['binding']=='history_binding1')
        second.update(contract=first['contract'],digest=first['digest'])
        for stage in STAGES:
            result=invoke(p,stage);self.assertEqual(result.returncode,0,(stage,result.stderr));out=json.loads(result.stdout)
            self.assertEqual(len(out['declaredFaultEntries']),2);self.assertEqual(len(out['declaredFaultHistories']),1)
            if 'conditionalFaultEntries' in out:
                self.assertEqual(out['conditionalFaultEntries']['coveredEntries'],[])
                self.assertEqual(out['conditionalFaultEntries']['uncoveredEntries'],['handler0','handler1'])

    def test_access_before_entry_and_source_control_protection_are_not_bypassed(self):
        p=package(False)
        for mutation,code in [('access',b'fault-entry-access-precheck'),('protection',b'control-information-flow')]:
            s,l,r=deepcopy(p)
            if mutation=='access':
                s=s.replace('Keep "Fault history" as type "Provider fault history" from "History types" under "Expense access".','Keep "Fault history" as type "Provider fault history" from "History types" under "Entry access".')
                rule='Rule for protection "Entry access". Permit access when ((the value of "Fault history") equals (the value of "Fault history")). End policy.'
                s=s.replace(CLAUSE,CLAUSE+' '+rule)
                l['declarations'].append({'id':'entry_access','kind':'policy','scope':'provider_handler','name':'Entry access'})
            else:
                a=s.index('Keep "History copy"');b=s.index('End data.',a)
                s=s[:a]+s[a:b].replace('Sensitivity "expense".','Sensitivity "public".')+s[b:]
            result=invoke((s,l,r),'check-exceptional-availability');self.assertEqual(result.returncode,1,(mutation,result.stderr));self.assertEqual(result.stdout,b'');self.assertIn(code,result.stderr,(mutation,result.stderr))

    def test_bounded_source_refuses_oversized_report_before_stdout_in_all_views(self):
        p=oversized_package();grammar.recognize(p[0]);grammar.validate_ledger(p[1]);self.assertLess(len(grammar.lex(p[0])),16384)
        lowered=invoke(p);self.assertEqual(lowered.returncode,0,lowered.stderr)
        for stage in STAGES:
            result=invoke(p,stage);self.assertEqual(result.returncode,1,(stage,result.stderr));self.assertEqual(result.stdout,b'');self.assertIn(b'report-budget',result.stderr)


if __name__=='__main__':unittest.main()
