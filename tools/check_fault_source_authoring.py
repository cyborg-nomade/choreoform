# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Explicit source-0.7 dependency evidence, never a host execution."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest

import authoring_probe as grammar
import check_ir_fixtures as wire
from check_metadata_authoring import invoke
from check_static_contracts import ROOT

BASE=ROOT/'docs/benchmarks/rp01'
CORE4={'contract':'urn:choreoform:contracts:core-profile:0.4.0','digest':'sha256:0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3','dialect':'core'}
FAULT_PROFILE={'id':'urn:choreoform:fault-source-profile:provider-observations:0.1.0','revision':'sha256:f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0'}
STAGES=['check-timing','check-fault-routes','check-exceptional-availability','check-reconciliation-links','check-reconciliation-availability']


def package():
    source=(BASE/'source.choreo').read_text().replace('authoring "0.6.0"','authoring "0.7.0"',1)
    source=source.replace('basis "Payable request".','basis "Payable request", fault source "Payment faults".')
    ledger=json.loads((BASE/'bindings.json').read_bytes())
    ledger.update(profile='0.7.0',version='0.6.0');ledger['contracts']['core']=CORE4.copy()
    manifest=json.loads((BASE/'manifest.json').read_bytes())
    resources=[(r['id'],r['revision'],(ROOT/r['path']).read_bytes()) for r in manifest['resources']]
    provider=next({'id':i,'revision':r} for i,r,_ in resources if i=='urn:choreoform:benchmarks:rp01:payment:0.1.0')
    artifact={'kind':'provider-fault-source','version':'0.1.0','id':'urn:test:authoring-payment-faults','profile':FAULT_PROFILE,'provider':provider,'emissions':{'confirmed no effect':{'observations':{'no-effect':True},'evidence':'definitive-no-effect'},'provider unknown':{'observations':{'unknown':True},'evidence':'unresolved'}}}
    bind_source(ledger,resources,artifact)
    return source,ledger,resources


def bind_source(ledger,resources,artifact,binding="payment_faults_binding",name="Payment faults"):
    raw=json.dumps(artifact,separators=(',',':')).encode();revision='sha256:'+sha256(raw).hexdigest()
    ledger['declarations']=[d for d in ledger['declarations'] if d['id']!=binding]
    ledger['declarations'].append({'id':binding,'kind':'resource','scope':ledger['root'],'name':name})
    ledger['resources']=[r for r in ledger['resources'] if r['binding']!=binding]
    ledger['resources'].append({'binding':binding,'kind':'provider-fault-source','contract':artifact['id'],'digest':revision})
    resources.append((artifact['id'],revision,raw))
    if not any(i==FAULT_PROFILE['id'] for i,_,_ in resources):
        resources.append((FAULT_PROFILE['id'],FAULT_PROFILE['revision'],(ROOT/'docs/ir/contracts'/ (FAULT_PROFILE['revision'].replace(':','-')+'.txt')).read_bytes()))


def dense(calls=130):
    from benchmarks.rp01_source import Package, Scope, quote
    from benchmarks.rp01_graph import foundations, PEOPLE
    from benchmarks.rp01_resources import clock_resources, record, INTEGER, encoded, pin
    p=Package();p.ledger.update(profile='0.7.0',version='0.6.0',root='expense')
    p.ledger['contracts']['core']=CORE4.copy()
    root=Scope(p,'expense','Retry disclosure budget',None,'Call 0',inputs=[('basis','Basis','Basis')],closure='Preserve remaining work')
    foundations(root);root.data('basis_value','Basis',INTEGER,participants=PEOPLE,invalidates=[f'"Call {i}"' for i in range(calls)])
    clock=clock_resources();resources=[]
    for _,raw,cp in clock.values():resources.append((cp['id'],cp['revision'],raw))
    profile=clock['clock'][0]['profile'];resources.append((profile['id'],profile['revision'],(ROOT/'docs/ir/contracts'/(profile['revision'].replace(':','-')+'.txt')).read_bytes()))
    p.resource('clock_binding','Expense clock','clock',clock['clock'][2])
    protection=clock['clock-source'][0]['resultProtection']
    observation='o'*4000;fault='f'*4000
    provider={'kind':'provider-contract','version':'0.1.0','id':'urn:test:wide-fault-provider','input':record(),'output':record(),'outcomes':{'done':True},'observations':{'done':{'state':'succeeded','result':record(),'outcome':'done'},observation:{'state':'unknown','result':None,'outcome':None}},'resultProtection':protection,'effectClass':'read-only','idempotency':'stable-key'}
    raw=encoded(provider);cp=pin(provider['id'],raw);resources.append((cp['id'],cp['revision'],raw));p.resource('provider_binding','Provider','provider',cp)
    root.capability('retry_cap','Retry capability','Provider',record(),record(),'Payment operator','Effects')
    root.policy('effect','effect','Effects','Effect is read-only. Idempotency is stable-key. Reconcile under "Observation".')
    root.policy('observation','wait','Observation','Observe record with empty. Match when (true). Authority "Payment operator". Timer none. On observation outcome "observed" of "Observe" in "Remaining work" in "Retry disclosure budget". On timer none.')
    root.policy('retry_work','work','Retry work',f'Instructions "Synthetic". Authority "Payment operator". Attempts 2. Delays [0 microseconds]. Retry faults [{quote(fault)}]. For retries use clock "Expense clock", basis "Basis", fault source "Payment faults". Timeout none. Complete when (true). Compensates none.')
    root.expression('request_input','Request input',record(),{'op':'record','fields':{}})
    for i in range(calls):root.step(f'call{i}',f'Call {i}','Request "Retry capability" using "Request input" under "Retry work" and record in nothing.', [('done',f'Call {i+1}' if i+1<calls else 'Done')])
    root.step('done','Done','Finish as "complete".')
    root.step('observe','Observe','Wait under "Observation".', [('observed','Release remaining owner')])
    observe=root.declarations.pop()
    next(d for d in p.ledger['declarations'] if d['id']=='observe')['scope']='remaining'
    root.declarations=[d.replace('Start at "Release remaining owner".','Start at "Observe".').replace('Step "Release remaining owner".',observe+'\nStep "Release remaining owner".') if d.startswith('Scope "Remaining work"') else d for d in root.declarations]
    artifact={'kind':'provider-fault-source','version':'0.1.0','id':'urn:test:wide-fault-source','profile':FAULT_PROFILE,'provider':cp,'emissions':{fault:{'observations':{observation:True},'evidence':'unresolved'}}}
    bind_source(p.ledger,resources,artifact)
    return 'Choreoform authoring "0.7.0".\n'+root.render(),p.ledger,resources


class FaultSourceAuthoringEvidence(unittest.TestCase):
    def test_full_source_grammar_schema_derived_body_and_all_native_disclosures(self):
        p=package();s,l,_=p
        grammar.recognize(s);grammar.validate_ledger(l)
        raw=invoke(p);self.assertEqual(raw.returncode,0,raw.stderr)
        candidate=wire.load(raw.stdout);self.assertEqual(candidate['revision'],wire.revision(candidate))
        self.assertEqual(candidate['body']['dialects']['core']['id'],CORE4['contract'])
        for name,pol in candidate['body']['policies'].items():
            if pol['body']['kind']=='work':
                self.assertEqual(pol['body']['retry']['faultSource'] is None,name!='payment_work')
        for stage in STAGES:
            r=invoke(p,stage);self.assertEqual(r.returncode,0,(stage,r.stderr))
            report=json.loads(r.stdout);self.assertIs(report['executable'],False)
            row,=report['retryFaultBindings']
            self.assertEqual(row['revision'],candidate['revision']);self.assertEqual(row['site'],'payment_request_step')
            self.assertEqual(row['profile'],FAULT_PROFILE)
            self.assertEqual(row['faults'],{'confirmed no effect':{'observations':['no-effect'],'evidence':'definitive-no-effect'}})
            self.assertTrue(row['hostRequirements'])
        f=json.loads(invoke(p,'check-fault-routes').stdout)
        self.assertEqual(f['unhandledFaultCases'],len(['confirmed no effect','provider unknown'])*len(f['checkedFaultScopes']))

    def test_format_alias_hash_cycles_and_explicit_revision_change(self):
        p=package();raw=json.loads(invoke(p).stdout)
        old=json.loads((BASE/'candidate.ir.json').read_bytes());self.assertNotEqual(raw['revision'],old['revision'])
        actual=json.loads(invoke(p,'integrity').stdout)
        self.assertEqual(actual,grammar.package_digests(p[0].encode(),json.dumps(p[1],separators=(',',':')).encode()))
        formatted=json.loads(invoke(p,'format').stdout)
        again=(formatted['source'],json.loads(formatted['companion']),p[2])
        self.assertEqual(json.loads(invoke(again).stdout),raw)
        s,l,r=deepcopy(p);s=s.replace('"Payment faults"','"Payment observations"')
        next(d for d in l['declarations'] if d['id']=='payment_faults_binding')['name']='Payment observations'
        self.assertEqual(json.loads(invoke((s,l,r)).stdout),raw)

    def test_old_versions_wrong_pairs_and_closed_source_resource_bindings(self):
        p=package()
        for change,expected in [('pair',b'ledger-profile'),('pin',b'contract-pin'),('kind',b'resource-format'),('missing_clause',b'syntax'),('missing_source',b'reference-missing'),('unknown_fault',b'retry-fault-source-name')]:
            s,l,r=deepcopy(p)
            if change=='pair': l['version']='0.5.0'
            elif change=='pin': l['contracts']['core']['digest']='sha256:'+'0'*64
            elif change=='kind': next(x for x in l['resources'] if x['kind']=='provider-fault-source')['kind']='provider'
            elif change=='missing_clause': s=s.replace(', fault source "Payment faults"','')
            elif change=='missing_source': s=s.replace('fault source "Payment faults"','fault source "SECRET-MISSING-SOURCE"')
            else:s=s.replace('Retry faults ["confirmed no effect"]','Retry faults ["SECRET-UNMAPPED"]')
            result=invoke((s,l,r),'check');self.assertNotEqual(result.returncode,0,change);self.assertEqual(result.stdout,b'');self.assertIn(expected,result.stderr)
            self.assertNotIn(b'SECRET-',result.stderr)
        for old in ['0.3.0','0.4.0','0.5.0','0.6.0']:
            with self.assertRaises(ValueError): grammar.recognize(p[0].replace('authoring "0.7.0"',f'authoring "{old}"',1))
        # Ordinary old package reports gain no new field from unrelated metadata.
        s=(BASE/'source.choreo').read_text();l=json.loads((BASE/'bindings.json').read_bytes())
        for stage in STAGES:
            result=invoke((s,l,p[2]),stage);self.assertEqual(result.returncode,0,result.stderr)
            self.assertNotIn('retryFaultBindings',json.loads(result.stdout))
        with self.assertRaises(ValueError):grammar.recognize('Choreoform authoring "0.8.0". '+'bad '*5000)

    def test_source_only_metadata_reads_are_derived_in_unused_work(self):
        s,l,r=package()
        provider=json.loads((BASE/'payment.provider.json').read_bytes())
        provider['id']='urn:test:source-only-provider';provider['resultProtection']['policy']='source_protect'
        raw=json.dumps(provider,separators=(',',':')).encode();cp={'id':provider['id'],'revision':'sha256:'+sha256(raw).hexdigest()};r.append((cp['id'],cp['revision'],raw))
        artifact={'kind':'provider-fault-source','version':'0.1.0','id':'urn:test:source-only-faults','profile':FAULT_PROFILE,'provider':cp,'emissions':{'confirmed no effect':{'observations':{'no-effect':True},'evidence':'definitive-no-effect'}}}
        bind_source(l,r,artifact,'source_only_binding','Other faults')
        for identity,name in [('source_protect','Source protection'),('unused_source_work','Unused source work')]:l['declarations'].append({'id':identity,'kind':'policy','scope':'expense','name':name})
        insert='Rule for protection "Source protection". Permit access when (the value of "Review needed"). End policy.\nRule for work "Unused source work". Instructions "Unused declaration". Authority "Payment operator". Attempts 2. Delays [0 microseconds]. Retry faults ["confirmed no effect"]. For retries use clock "Expense clock", basis "Payment due", fault source "Other faults". Timeout none. Complete when (true). Compensates none. End policy.\n'
        s=s.replace('Rule for settlement "Settle children".',insert+'Rule for settlement "Settle children".',1)
        result=invoke((s,l,r),'check');self.assertEqual(result.returncode,0,result.stderr)
        candidate=json.loads(invoke((s,l,r)).stdout)
        reads=candidate['body']['policies']['unused_source_work']['body']['reads']
        self.assertIn('review_needed',reads);self.assertIn('payment_due',reads)
        # This extra declaration is not a real use or ordinary entry.
        report=json.loads(invoke((s,l,r),'check-timing').stdout)
        self.assertEqual([x['site'] for x in report['retryFaultBindings']],['payment_request_step'])

    def test_actual_source_context_and_provider_identity_refuse_even_unvisited(self):
        s,l,r=package()
        old=next(line for line in s.splitlines() if line.startswith('Rule for work "Manager review rules".'))
        new=old.replace('Attempts 1. Delays empty. Retry faults empty.','Attempts 2. Delays [0 microseconds]. Retry faults ["confirmed no effect"]. For retries use clock "Expense clock", basis "Current claim", fault source "Payment faults".')
        result=invoke((s.replace(old,new),l,r),'check');self.assertIn(b'retry-fault-source-context-unproved',result.stderr);self.assertEqual(result.stdout,b'')
        for unvisited in [False,True]:
            s,l,r=package();provider=json.loads((BASE/'payment.provider.json').read_bytes())
            raw=json.dumps(provider,separators=(',',':')).encode();cp={'id':provider['id'],'revision':'sha256:'+sha256(raw).hexdigest()};r.append((cp['id'],cp['revision'],raw))
            artifact=json.loads(next(raw for i,_,raw in r if i=='urn:test:authoring-payment-faults'));artifact['provider']=cp;bind_source(l,r,artifact)
            if unvisited:s=s.replace('Start at "Review again".','Start at "Finish paid".',1)
            result=invoke((s,l,r),'check');self.assertIn(b'retry-fault-source-provider',result.stderr);self.assertEqual(result.stdout,b'')

    def test_actual_bounded_native_disclosure_refuses_without_stdout(self):
        p=dense()
        lowered=invoke(p);self.assertEqual(lowered.returncode,0,lowered.stderr)
        for stage in STAGES:
            result=invoke(p,stage);self.assertNotEqual(result.returncode,0,stage)
            self.assertEqual(result.stdout,b'');self.assertEqual(result.stderr.strip(),b'report-budget',stage)

    def test_missing_exact_source_profile_and_inherited_provider_bytes_refuse(self):
        p=package()
        for identity in ['urn:test:authoring-payment-faults',FAULT_PROFILE['id'],'urn:choreoform:benchmarks:rp01:payment:0.1.0','urn:choreoform:benchmarks:rp01:types:0.1.0']:
            s,l,r=deepcopy(p);r=[x for x in r if x[0]!=identity]
            result=invoke((s,l,r));self.assertNotEqual(result.returncode,0);self.assertEqual(result.stdout,b'');self.assertIn(b'resource-missing',result.stderr)
        s,l,r=deepcopy(p);r=[(i,rev,b'corrupt' if i==FAULT_PROFILE['id'] else raw) for i,rev,raw in r]
        result=invoke((s,l,r));self.assertNotEqual(result.returncode,0);self.assertEqual(result.stdout,b'');self.assertIn(b'resource-digest',result.stderr)

if __name__=='__main__':unittest.main()
