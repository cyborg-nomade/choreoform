# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Explicit source capture evidence; static declarations, never host execution."""
from copy import deepcopy
from hashlib import sha256
import json
from pathlib import Path
import unittest
import jsonschema

import authoring_probe as grammar
import check_ir_fixtures as wire
from benchmarks.rp01_graph import foundations, PEOPLE
from benchmarks.rp01_resources import record, INTEGER, TEXT, encoded, pin
from benchmarks.rp01_source import Package, Scope, type_source
from check_metadata_authoring import invoke
from check_static_contracts import ROOT
from check_terminal_observation_sources import normalized_type

CORE5={'contract':'urn:choreoform:contracts:core-profile:0.5.0','digest':'sha256:50007ac05e5e71df91bb9997a40a06c7a8993df04b1a673421e4a987014299d7','dialect':'core'}
PROFILE={'id':'urn:choreoform:observation-source-profile:provider-terminal-facts:0.1.0','revision':'sha256:a224fc2b8318af2ff8dfc9955b55af03f4d5b7bd0de719de0384421698d20ead'}
STAGES=['check','check-control','check-protection','check-invalidation','invalidation-requirements','check-concurrency','check-timing','check-fault-routes','check-exceptional-availability','check-reconciliation-links','check-reconciliation-availability']


def source_resource(ledger,resources,provider,identity='urn:test:terminal-source',binding='terminal_binding',name='Terminal facts'):
    artifact={'kind':'provider-terminal-observation-source','version':'0.1.0','id':identity,'profile':PROFILE,'provider':provider}
    raw=encoded(artifact);p=pin(identity,raw)
    ledger['declarations'].append({'id':binding,'kind':'resource','scope':ledger['root'],'name':name})
    ledger['resources'].append({'binding':binding,'kind':artifact['kind'],'contract':p['id'],'digest':p['revision']})
    resources.append((p['id'],p['revision'],raw))
    if not any(i==PROFILE['id'] for i,_,_ in resources):
        resources.append((PROFILE['id'],PROFILE['revision'],(ROOT/'docs/ir/contracts'/(PROFILE['revision'].replace(':','-')+'.txt')).read_bytes()))
    return artifact


def package(captures=1,long_pins=False,unvisited=False):
    p=Package();p.ledger.update(profile='0.8.0',version='0.7.0');p.ledger['contracts']['core']=CORE5.copy()
    root=Scope(p,'expense','Capture example',None,'Done' if unvisited else 'Request',closure='Preserve remaining work')
    foundations(root)
    protection={'policy':'expense_access','sensitivity':'expense','purposes':{'reimbursement':True},'participants':{k+'_actor':True for k in ['employee','manager','finance','operator','clock']},'capabilities':{'capture_cap':True}}
    provider={'kind':'provider-contract','version':'0.1.0','id':('p'*3900 if long_pins else 'urn:test:terminal-provider'),'input':record(),'output':INTEGER,'outcomes':{'done':True},'observations':{'done':{'state':'succeeded','result':INTEGER,'outcome':'done'}},'resultProtection':protection,'effectClass':'read-only','idempotency':'none'}
    raw=encoded(provider);cp=pin(provider['id'],raw);resources=[(cp['id'],cp['revision'],raw)];p.resource('provider_binding','Provider','provider',cp)
    source_resource(p.ledger,resources,cp,identity='s'*3900 if long_pins else 'urn:test:terminal-source')
    root.capability('capture_cap','Capture service','Provider',record(),INTEGER,'Payment operator','Effects')
    root.policy('effect','effect','Effects','Effect is read-only. Idempotency is none. Reconcile under "Recovery observations".')
    root.policy('recovery_wait','wait','Recovery observations','Observe text. Match when (true). Authority "Payment operator". Timer none. On observation outcome "observed" of "Observe" in "Remaining work" in "Capture example". On timer none.')
    root.policy('work','work','Request work','Instructions "Synthetic". Authority "Payment operator". Attempts 1. Delays empty. Retry faults empty. Timeout none. Complete when (true). Compensates none.')
    root.expression('request_input','Request input',record(),{'op':'record','fields':{}})
    root.data('captured','Captured result',normalized_type(provider),participants=PEOPLE,capabilities=['Capture service'],invalidates=['"Inspect"'])
    root.expression('tag','Captured tag',TEXT,{'op':'tag','value':{'op':'read','cell':'Captured result'}})
    root.step('origin','Request','Request "Capture service" using "Request input" under "Request work" and record in nothing.',[('done','Capture 0')])
    for i in range(captures):
        root.policy(f'wait{i}','wait',f'Capture policy {i}',f'Observe {type_source(normalized_type(provider))}. Match when (true). Authority "Payment operator". Timer none. On observation outcome "observed" of "Capture {i}". On timer none.')
        root.step(f'capture{i}',f'Capture {i}',f'Wait under "Capture policy {i}" for observations from "Terminal facts" about "Request" and record in "Captured result".',[('observed',f'Capture {i+1}' if i+1<captures else 'Inspect')])
    root.expression('guard','Confirmed result',{'kind':'boolean'},{'op':'eq','left':{'op':'tag','value':{'op':'read','cell':'Captured result'}},'right':{'op':'literal','type':TEXT,'value':'confirmed'}})
    root.step('inspect','Inspect','Choose by ["known" when "Confirmed result"]; otherwise "other".', [('known','Done'),('other','Done')])
    root.step('done','Done','Finish as "complete".')
    root.step('observe','Observe','Wait under "Recovery observations".',[('observed','Release remaining owner')])
    observe=root.declarations.pop();next(d for d in p.ledger['declarations'] if d['id']=='observe')['scope']='remaining'
    root.declarations=[d.replace('Start at "Release remaining owner".','Start at "Observe".').replace('Step "Release remaining owner".',observe+'\nStep "Release remaining owner".') if d.startswith('Scope "Remaining work"') else d for d in root.declarations]
    return 'Choreoform authoring "0.8.0".\n'+root.render(),p.ledger,resources


class WaitCaptureAuthoringEvidence(unittest.TestCase):
    def test_readable_capture_and_all_native_stage_declarations(self):
        p=package();s,l,r=p;grammar.recognize(s);grammar.validate_ledger(l)
        result=invoke(p);self.assertEqual(result.returncode,0,result.stderr)
        d=wire.load(result.stdout);jsonschema.Draft202012Validator(wire.SCHEMA_V2).validate(d);self.assertEqual(d['version'],'0.2.0');self.assertEqual(d['revision'],wire.revision(d))
        self.assertEqual(d['body']['nodes']['capture0']['writes'],{'captured':True})
        self.assertIsNone(d['body']['nodes']['observe']['capture'])
        for stage in STAGES:
            result=invoke(p,stage);self.assertEqual(result.returncode,0,(stage,result.stderr))
            report=json.loads(result.stdout);self.assertIs(report['executable'],False)
            row,=report['declaredWaitCaptures'];self.assertEqual(row['revision'],d['revision']);self.assertEqual(row['site'],'capture0');self.assertEqual(row['origin'],'origin');self.assertEqual(row['result'],'captured');self.assertEqual(row['profile'],PROFILE);self.assertTrue(row['hostRequirements'])
            self.assertNotIn('value',row)


    def test_format_alias_integrity_and_old_profiles_keep_distinct_wire_shapes(self):
        p=package();d=json.loads(invoke(p).stdout)
        formatted=json.loads(invoke(p,'format').stdout)
        self.assertEqual(json.loads(invoke((formatted['source'],json.loads(formatted['companion']),p[2])).stdout),d)
        s,l,r=deepcopy(p);s=s.replace('"Terminal facts"','"Provider facts"').replace('"Captured result"','"Stored terminal result"')
        for row in l['declarations']:
            if row['id']=='terminal_binding':row['name']='Provider facts'
            if row['id']=='captured':row['name']='Stored terminal result'
        self.assertEqual(json.loads(invoke((s,l,r)).stdout),d)
        self.assertEqual(json.loads(invoke(p,'integrity').stdout),grammar.package_digests(p[0].encode(),encoded(p[1])))
        from check_late_observation_boundaries import package as current
        old=current();old_d=json.loads(invoke(old).stdout)
        self.assertEqual(old_d['version'],'0.1.0');self.assertTrue(all('capture' not in n for n in old_d['body']['nodes'].values() if n['kind']=='wait'))
        for stage in STAGES:
            result=invoke(old,stage);self.assertEqual(result.returncode,0,(stage,result.stderr));self.assertNotIn('declaredWaitCaptures',json.loads(result.stdout))

    def test_strict_pair_kind_grammar_delta_and_explicit_metadata_pins(self):
        p=package()
        old=(ROOT/'docs/authoring/candidate-a-v0.7.ebnf').read_text();new=(ROOT/'docs/authoring/candidate-a-v0.8.ebnf').read_text()
        old_rules=grammar.load_grammar(ROOT/'docs/authoring/candidate-a-v0.7.ebnf');new_rules=grammar.load_grammar(ROOT/'docs/authoring/candidate-a-v0.8.ebnf')
        self.assertEqual(set(old_rules),set(new_rules));self.assertEqual({k:v for k,v in old_rules.items() if k!='wait'},{k:v for k,v in new_rules.items() if k!='wait'})
        for profile in ['0.2.0','0.3.0','0.4.0','0.5.0','0.6.0','0.7.0']:
            with self.assertRaises(ValueError):grammar.recognize(p[0].replace('authoring "0.8.0"',f'authoring "{profile}"'))
        with self.assertRaises(ValueError):grammar.recognize('Choreoform authoring "0.9.0". '+'bad '*5000)
        for mutation,code in [('pair',b'ledger-profile'),('pin',b'contract-pin'),('kind',b'resource-format'),('missing_source',b'resource-missing-or-ambiguous'),('missing_profile',b'resource-missing'),('corrupt_profile',b'resource-digest')]:
            s,l,r=deepcopy(p)
            if mutation=='pair':l['version']='0.6.0'
            elif mutation=='pin':l['contracts']['core']['digest']='sha256:'+'0'*64
            elif mutation=='kind':next(v for v in l['resources'] if v['binding']=='terminal_binding')['kind']='provider'
            elif mutation=='missing_source':r=[v for v in r if v[0]!='urn:test:terminal-source']
            elif mutation=='missing_profile':r=[v for v in r if v[0]!=PROFILE['id']]
            else:r=[(i,h,raw+b' ' if i==PROFILE['id'] else raw) for i,h,raw in r]
            result=invoke((s,l,r),'check');self.assertEqual(result.stdout,b'');self.assertEqual(result.returncode,1);self.assertIn(code,result.stderr,(mutation,result.stderr))
        schema=json.loads((ROOT/'schemas/authoring/bindings-0.7.schema.json').read_bytes());jsonschema.Draft202012Validator(schema).validate(p[1])
        wrong=deepcopy(p[1]);wrong['profile']='0.7.0'
        with self.assertRaises(jsonschema.ValidationError):jsonschema.Draft202012Validator(schema).validate(wrong)

    def test_unvisited_declarations_and_source_only_access_reads_remain_explicit(self):
        p=package(unvisited=True)
        for stage in STAGES:
            result=invoke(p,stage);self.assertEqual(result.returncode,0,(stage,result.stderr));row,=json.loads(result.stdout)['declaredWaitCaptures'];self.assertEqual(row['site'],'capture0')
        s,l,r=package();provider=json.loads(r[0][2]);provider['resultProtection']['participants']['source_actor']=True
        raw=encoded(provider);provider_pin=pin(provider['id'],raw);r[0]=(provider_pin['id'],provider_pin['revision'],raw)
        next(v for v in l['resources'] if v['binding']=='provider_binding')['digest']=provider_pin['revision']
        source=json.loads(r[1][2]);source['provider']=provider_pin;raw=encoded(source);source_pin=pin(source['id'],raw);r[1]=(source_pin['id'],source_pin['revision'],raw)
        next(v for v in l['resources'] if v['binding']=='terminal_binding')['digest']=source_pin['revision']
        l['declarations'] += [{'id':'source_actor','kind':'actor','scope':'expense','name':'Source witness'},{'id':'source_authority','kind':'policy','scope':'expense','name':'Source authority'},{'id':'policy_value','kind':'data','scope':'expense','name':'Policy value'}]
        l['generated'].append({'id':'policy_initial','owner':'policy_value','role':'initial','key':None});l['symbols'].append({'id':'policy_purpose','owner':'policy_value','role':'purpose','name':'Reimbursement','wire':'reimbursement'})
        extra='Require actor "Source witness" under "Source authority". Rule for actor "Source authority". Permit when ((false) and ((the value of "Policy value") equals (0))). End policy. Keep "Policy value" as whole number under "Expense access". Sensitivity "expense". Purposes ["Reimbursement"]. Participants ["Employee", "Manager", "Finance reviewer", "Payment operator", "Clock observer"]. Capabilities empty. Invalidate empty. Initially (0). End data. '
        s=s.replace('Capabilities ["Capture service"].','Capabilities empty.').replace('End process.',extra+'End process.')
        result=invoke((s,l,r),'check');self.assertEqual(result.returncode,0,result.stderr)
        body=json.loads(invoke((s,l,r)).stdout)['body'];self.assertEqual(body['nodes']['capture0']['reads'],{'policy_value':True})
        self.assertEqual(body['policies']['wait0']['body']['reads'],{})

    def test_bounded_disclosure_refuses_before_any_stdout(self):
        p=package(captures=130,long_pins=True,unvisited=True)
        self.assertLess(len(grammar.lex(p[0],program=True)),16384)
        result=invoke(p);self.assertEqual(result.returncode,0,result.stderr)
        for stage in STAGES:
            result=invoke(p,stage);self.assertEqual(result.returncode,1,(stage,result.stderr));self.assertEqual(result.stdout,b'');self.assertEqual(result.stderr.strip(),b'report-budget')

    def test_full_rp01_promoted_local_capture_still_refuses_storage_lifetime(self):
        from check_late_observation_boundaries import package as current
        s,l,r=current();s=s.replace('authoring "0.7.0"','authoring "0.8.0"',1);l.update(profile='0.8.0',version='0.7.0');l['contracts']['core']=CORE5.copy()
        provider_pin=next({'id':i,'revision':rev} for i,rev,_ in r if i=='urn:choreoform:benchmarks:rp01:payment:0.1.0')
        source_resource(l,r,provider_pin)
        l['declarations'].append({'id':'recovery_local','kind':'data','scope':'payment_recovery','name':'Local terminal result'})
        l['symbols'].append({'id':'recovery_purpose','owner':'recovery_local','role':'purpose','name':'Reimbursement','wire':'reimbursement'})
        provider=json.loads(next(raw for i,_,raw in r if i==provider_pin['id']))
        data=f'Keep "Local terminal result" as {type_source(normalized_type(provider))} under "Expense access". Sensitivity "expense". Purposes ["Reimbursement"]. Participants ["Employee", "Manager", "Finance reviewer", "Payment operator", "Clock observer"]. Capabilities ["Create the payable"]. Invalidate empty. Initially unavailable. End data. '
        s=s.replace('Step "Observe Payment".',data+'Step "Observe Payment".')
        self.assertEqual(s.count('Keep "Local terminal result"'),1)
        s=s.replace('Wait under "Payment reconciliation".','Wait under "Payment reconciliation" for observations from "Terminal facts" about "Request Payment" and record in "Local terminal result".')
        p=s,l,r
        for stage in ['check','check-invalidation']:
            result=invoke(p,stage);self.assertEqual(result.returncode,0,(stage,result.stderr))
        result=invoke(p,'check-concurrency');self.assertEqual(result.returncode,1);self.assertEqual(result.stdout,b'');self.assertIn(b'scope-lifetime-unproved at /body/nodes/run_payment',result.stderr)


if __name__=='__main__':unittest.main()
