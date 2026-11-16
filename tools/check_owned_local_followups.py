# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite conditional isolation/protection evidence, never values or a host run."""
from copy import deepcopy
import json
import unittest

import authoring_probe as grammar
from benchmarks.rp01_graph import foundations, PEOPLE
from benchmarks.rp01_resources import BOOLEAN, INTEGER, TEXT, record, encoded
from benchmarks.rp01_source import Package, Scope, type_source
from check_metadata_authoring import invoke
from check_wait_capture_authoring import CORE5, STAGES, package as ordinary, promoted_local
from check_terminal_observation_sources import normalized_type


def package(cells=1,creators=1,restricted_control=False):
    _,old,resources=ordinary();provider=json.loads(resources[0][2])
    p=Package();p.ledger.update(profile='0.8.0',version='0.7.0',definition='owned_local_example',annotations={'synthetic':True,'fixture':'owned-local'});p.ledger['contracts']['core']=CORE5.copy()
    root=Scope(p,'expense','Owned capture example',None,'Gate' if restricted_control else 'Run 0')
    foundations(root)
    p.resource('provider_binding','Provider','provider',{'id':resources[0][0],'revision':resources[0][1]})
    source=next(v for v in old['resources'] if v['binding']=='terminal_binding');p.resource('terminal_binding','Terminal facts',source['kind'],{'id':source['contract'],'revision':source['digest']})
    root.capability('capture_cap','Capture service','Provider',record(),INTEGER,'Payment operator','Effects')
    root.policy('effect','effect','Effects','Effect is read-only. Idempotency is none. Reconcile under "Capture policy".')
    root.policy('capture_policy','wait','Capture policy',f'Observe {type_source(normalized_type(provider))}. Match when (true). Authority "Payment operator". Timer none. On observation outcome "observed" of "Capture" in "Recovery" in "Job" in "Owned capture example". On timer none.')
    root.policy('work','work','Request work','Instructions "Synthetic". Authority "Payment operator". Attempts 1. Delays empty. Retry faults empty. Timeout none. Complete when (true). Compensates none.')
    root.expression('input','Request input',record(),{'op':'record','fields':{}})
    if restricted_control:
        root.data('signal','Private signal',BOOLEAN,initial={'op':'literal','type':BOOLEAN,'value':True},participants=PEOPLE,capabilities=['Capture service'],invalidates=['"Gate"','"Request" in "Job" in "Owned capture example"'])
        root.declarations[-1]=root.declarations[-1].replace('Sensitivity "expense".','Sensitivity "secret".')
        root.expression('signal_read','Signal',BOOLEAN,{'op':'read','cell':'Private signal'})
        root.step('gate','Gate','Choose by ["run" when "Signal"]; otherwise "skip".',[('run','Run 0'),('skip','Done')])
    job=Scope(p,'job','Job',root,'Request',closure='Job close')
    job.policy('job_close','closure','Job close','Reconcile in "Recovery".')
    job.step('origin','Request','Request "Capture service" using "Request input" under "Request work" and record in nothing.',[('done','Finish job')])
    job.step('job_done','Finish job','Finish as "complete".')
    recovery=Scope(p,'recovery','Recovery',job,'Capture')
    recovery.data('captured','Captured result',normalized_type(provider),participants=PEOPLE,capabilities=['Capture service'],invalidates=['"Inspect"'])
    for i in range(1,cells):recovery.data(f'local{i}',f'Local {i}',INTEGER,initial={'op':'literal','type':INTEGER,'value':'0'},participants=PEOPLE,capabilities=['Capture service'])
    recovery.expression('guard','Confirmed',BOOLEAN,{'op':'eq','left':{'op':'tag','value':{'op':'read','cell':'Captured result'}},'right':{'op':'literal','type':TEXT,'value':'confirmed'}})
    recovery.step('capture','Capture','Wait under "Capture policy" for observations from "Terminal facts" about "Request" and record in "Captured result".',[('observed','Inspect')])
    recovery.step('inspect','Inspect','Choose by ["confirmed" when "Confirmed"]; otherwise "other".',[('confirmed','Finish recovery'),('other','Finish recovery')])
    recovery.step('recovered','Finish recovery','Finish as "complete".')
    job.child(recovery);root.child(job)
    for i in range(creators):root.step(f'run{i}',f'Run {i}','Run "Job" with inputs empty and outputs empty.',[('complete',f'Run {i+1}' if i+1<creators else 'Done')])
    root.step('done','Done','Finish as "complete".')
    # Long stable IDs stress shared proof output; readable names stay unchanged.
    rename={f'local{i}':'cell'+str(i)+'x'*(60-len(str(i))) for i in range(1,cells)}
    def renamed(v):
        if isinstance(v,str):return rename.get(v,v)
        if isinstance(v,list):return [renamed(x) for x in v]
        if isinstance(v,dict):return {k:renamed(x) for k,x in v.items()}
        return v
    return 'Choreoform authoring "0.8.0".\n'+root.render(),renamed(p.ledger),resources


def finite_judgment(d,provider):
    """Limited independent model for the prevalidated small fixture, not all IR."""
    b=d['body'];scope='recovery';sc=b['scopes'][scope]
    local={i for i,c in b['data'].items() if c['scope']==scope};nodes={i:n for i,n in b['nodes'].items() if n['scope']==scope}
    if sc['inputs'] or sc['outputs'] or any(v['parent']==scope for v in b['scopes'].values()):return 'scope-lifetime-unproved'
    if any(n.get('body')==scope or scope in n.get('children',{}).values() for n in b['nodes'].values()):return 'scope-lifetime-unproved'
    entry=sc['entry']
    if nodes[entry]['kind']!='wait' or nodes[entry].get('capture') is None:return 'scope-lifetime-unproved'
    for n in nodes.values():
        if n['kind'] not in ['wait','compute','decision','finish'] or not set(n['reads'])|set(n['writes'])<=local:return 'scope-lifetime-unproved'
        if n['kind']=='wait' and b['policies'][n['policy']]['body']['timer'] is not None:return 'scope-lifetime-unproved'
    preds={i:set() for i in nodes}
    for f in b['flows'].values():
        if f['source'] in nodes and f['target'] in nodes:preds[f['target']].add(f['source'])
    pending=set(nodes);order=[]
    while pending:
        ready={i for i in pending if preds[i]<=set(order)}
        if not ready:return 'scope-lifetime-unproved'
        order+=sorted(ready);pending-=ready
    available={};provenance={};initial={i for i in local if 'initial' in b['data'][i]}
    def label(origin):return provider['resultProtection'] if origin=='provider' else b['data'][origin.split(':',1)[1]]['protection']
    def transfer(source,target):return source['policy']==target['policy'] and source['sensitivity']==target['sensitivity'] and all(set(target[k])<=set(source[k]) for k in ['purposes','participants','capabilities'])
    # The fixture's owner executes its provider request; an optional root signal
    # controls creation. Preserve that immutable label, never its runtime value.
    inherited={'provider'}|({'label:signal'} if 'signal' in b['data'] else set())
    for identity in order:
        n=nodes[identity]
        if identity==entry:state=set(initial);pc=set(inherited)
        else:
            previous=[p for p in preds[identity] if p in available]
            if not previous:continue
            state=set.intersection(*(available[p] for p in previous));pc=set().union(*(provenance[p] for p in previous))
        if not set(n['reads'])<=state:return 'owned-local-availability-unproved'
        reads={'cell:'+c for c in n['reads']}
        if n.get('capture'):pc.add('provider')
        if n['kind'] in ['wait','decision']:
            for origin in pc|reads:
                if origin.startswith('cell:') and identity not in b['data'][origin[5:]]['invalidates']:return 'owned-local-invalidation-coverage'
        for cell in n['writes']:
            if any(not transfer(label(o),b['data'][cell]['protection']) for o in pc|reads):return 'control-information-flow'
        available[identity]=state|set(n['writes']);provenance[identity]=pc|reads|{'cell:'+c for c in n['writes']}
    return 'passed'


class OwnedLocalEvidence(unittest.TestCase):
    def test_local_capture_branch_and_all_composed_declarations(self):
        p=package();grammar.recognize(p[0]);grammar.validate_ledger(p[1])
        for stage in STAGES:
            result=invoke(p,stage);self.assertEqual(result.returncode,0,(stage,result.stderr))
            report=json.loads(result.stdout);self.assertIs(report['executable'],False)
            if stage in STAGES[5:]:
                row,=report['ownedLocalFollowups'];self.assertEqual(row['creator'],'run0');self.assertEqual(row['localCells'],['captured']);self.assertTrue(row['hostRequirements'])
                checks=report['conditionalOwnedLocalChecks'];self.assertIs(checks['conditional'],True);self.assertIn('inspect',checks['checkedNodes']);self.assertEqual(checks['invalidationObligations'],[{'cell':'captured','site':'inspect'}])

    def test_creator_control_provenance_cannot_be_erased_by_fresh_storage(self):
        p=package(restricted_control=True)
        result=invoke(p,'check-invalidation');self.assertEqual(result.returncode,0,result.stderr)
        result=invoke(p,'check-concurrency');self.assertEqual(result.returncode,1);self.assertEqual(result.stdout,b'');self.assertIn(b'control-information-flow at /body/nodes/capture',result.stderr)


    def test_finite_isolation_initialization_protection_and_invalidation_vectors(self):
        vectors=[('plain','passed'),('restricted','control-information-flow'),('missing_links','owned-local-invalidation-coverage'),('read_before','owned-local-availability-unproved'),('ancestor','scope-lifetime-unproved'),('no_capture','scope-lifetime-unproved')]
        for name,expected in vectors:
            s,l,r=package(restricted_control=name in ['restricted','ancestor'])
            if name=='missing_links':s=s.replace('Invalidate ["Inspect"].','Invalidate empty.')
            elif name=='read_before':s=s.replace('and record in "Captured result".','and record in "Captured result". Also read ["Captured result"].')
            elif name=='ancestor':s=s.replace('Sensitivity "secret".','Sensitivity "expense".').replace('and record in "Captured result".','and record in "Captured result". Also read ["Private signal"].')
            elif name=='no_capture':s=s.replace(' for observations from "Terminal facts" about "Request" and record in "Captured result"','')
            p=s,l,r;raw=invoke(p);self.assertEqual(raw.returncode,0,(name,raw.stderr));d=json.loads(raw.stdout)
            self.assertEqual(finite_judgment(d,json.loads(r[0][2])),expected,name)
            result=invoke(p,'check-concurrency')
            if expected=='passed':self.assertEqual(result.returncode,0,result.stderr)
            else:self.assertEqual(result.stdout,b'');self.assertEqual(result.returncode,1);self.assertIn(expected.encode(),result.stderr,(name,result.stderr))

    def test_shared_template_contexts_and_bounded_disclosure(self):
        p=package(cells=3,creators=3);result=invoke(p,'check-reconciliation-availability');self.assertEqual(result.returncode,0,result.stderr)
        report=json.loads(result.stdout);self.assertEqual(len(report['ownedLocalFollowups']),3);self.assertEqual(report['conditionalOwnedLocalChecks']['checkedScopes'],['recovery'])
        self.assertTrue(all(row['localCells']==report['ownedLocalFollowups'][0]['localCells'] for row in report['ownedLocalFollowups']))
        dense=package(cells=130,creators=130);self.assertEqual(len(grammar.lex(dense[0],program=True)),9451)
        for stage in ['lower','check-invalidation']:
            result=invoke(dense,stage);self.assertEqual(result.returncode,0,(stage,result.stderr))
        for stage in STAGES[5:]:
            result=invoke(dense,stage);self.assertEqual(result.returncode,1,(stage,result.stderr));self.assertEqual(result.stdout,b'');self.assertEqual(result.stderr.strip(),b'report-budget')

    def test_full_promoted_package_and_unchanged_old_report_evidence_remain_conditional(self):
        p=promoted_local()
        for stage in STAGES[5:]:
            result=invoke(p,stage);self.assertEqual(result.returncode,0,(stage,result.stderr));report=json.loads(result.stdout)
            row,=report['ownedLocalFollowups'];self.assertEqual(row['creator'],'run_payment');self.assertIn('authenticated-occurrence-bound-source-control-protection-provenance-retained-through-handoff',row['hostRequirements'])
            self.assertIs(report['conditionalOwnedLocalChecks']['conditional'],True);self.assertTrue(report['conditionalOwnedLocalChecks']['remaining'])
        from check_late_observation_boundaries import package as current
        old=current()
        for stage in STAGES:
            result=invoke(old,stage);self.assertEqual(result.returncode,0,(stage,result.stderr));report=json.loads(result.stdout);self.assertNotIn('ownedLocalFollowups',report);self.assertNotIn('conditionalOwnedLocalChecks',report)


if __name__=='__main__':unittest.main()
