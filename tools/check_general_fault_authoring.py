# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Explicit source-0.9 declarations; no fault events, receipts or execution."""
from copy import deepcopy
import json
import unittest

import authoring_probe as grammar
from benchmarks.rp01_resources import encoded,pin
from check_fault_source_authoring import bind_source,FAULT_PROFILE,dense as retry_dense
from check_general_fault_use import CORE6
from check_late_observation_boundaries import package as current
from check_metadata_authoring import invoke
from check_static_contracts import ROOT
from check_wait_capture_authoring import STAGES,promoted_local


def promote(source,ledger):
    s=source.replace('authoring "0.7.0"','authoring "0.9.0"',1).replace('authoring "0.8.0"','authoring "0.9.0"',1)
    l=deepcopy(ledger);l.update(profile='0.9.0',version='0.8.0');l['contracts']['core']={'contract':CORE6['id'],'digest':CORE6['revision'],'dialect':'core'}
    return s,l


def recognise(source,work,name):
    start=source.index(f'Rule for work "{work}".');end=source.index('End policy.',start)
    block=source[start:end];old='Authority "Payment operator".';assert old in block
    block=block.replace(old,old+f' Recognise faults from "{name}".',1)
    return source[:start]+block+source[end:]


def package(capture=False):
    s,l,r=promoted_local() if capture else current();s,l=promote(s,l)
    s=recognise(s,'Payment work','Payment faults').replace(', fault source "Payment faults"','')
    provider=next({'id':i,'revision':h} for i,h,_ in r if i=='urn:choreoform:benchmarks:rp01:notice:0.1.0')
    artifact={'kind':'provider-fault-source','version':'0.1.0','id':'urn:test:authoring-notice-faults','profile':FAULT_PROFILE,'provider':provider,'emissions':{'notice uncertainty':{'observations':{'unknown':True},'evidence':'unresolved'}}}
    bind_source(l,r,artifact,binding='notice_faults_binding',name='Notice faults');s=recognise(s,'Notice work','Notice faults')
    return s,l,r


def dense_long_pins(calls=130):
    from check_wait_capture_authoring import package as capture_package
    s,l,r=capture_package(long_pins=True,unvisited=True);s,l=promote(s,l)
    provider=json.loads(r[0][2]);provider['observations']['unknown']={'state':'unknown','result':None,'outcome':None}
    raw=encoded(provider);cp=pin(provider['id'],raw);r[0]=(cp['id'],cp['revision'],raw)
    next(v for v in l['resources'] if v['binding']=='provider_binding')['digest']=cp['revision']
    terminal=json.loads(r[1][2]);terminal['provider']=cp;raw=encoded(terminal);tp=pin(terminal['id'],raw);r[1]=(tp['id'],tp['revision'],raw)
    next(v for v in l['resources'] if v['binding']=='terminal_binding')['digest']=tp['revision']
    fault={'kind':'provider-fault-source','version':'0.1.0','id':'f'*3900,'profile':FAULT_PROFILE,'provider':cp,'emissions':{'uncertain':{'observations':{'unknown':True},'evidence':'unresolved'}}}
    bind_source(l,r,fault,binding='general_faults_binding',name='General faults');s=recognise(s,'Request work','General faults')
    steps=[]
    for i in range(calls-1):
        identity=f'extra{i}';outcome=identity+'_outcome'
        l['declarations'].append({'id':identity,'kind':'node','scope':'expense','name':f'Extra {i}'})
        l['symbols'].append({'id':outcome,'owner':identity,'role':'outcome','name':'done','wire':'done'})
        l['generated'].append({'id':identity+'_flow','owner':identity,'role':'flow','key':outcome})
        steps.append(f'Step "Extra {i}". Request "Capture service" using "Request input" under "Request work" and record in nothing. On "done", continue with "Done". End step.')
    s=s.replace('End process.','\n'.join(steps)+'\nEnd process.')
    return s,l,r


class GeneralFaultAuthoringEvidence(unittest.TestCase):
    def test_full_source_and_all_eleven_use_and_catalogue_disclosures(self):
        p=package();grammar.recognize(p[0]);grammar.validate_ledger(p[1]);result=invoke(p);self.assertEqual(result.returncode,0,result.stderr);candidate=json.loads(result.stdout)
        self.assertEqual(candidate['version'],'0.2.0');self.assertEqual(candidate['body']['dialects']['core'],CORE6)
        self.assertIsNone(candidate['body']['policies']['notice_work']['body']['retry']['faultSource'])
        self.assertEqual(candidate['body']['policies']['payment_work']['body']['faultSource'],candidate['body']['policies']['payment_work']['body']['retry']['faultSource'])
        for stage in STAGES:
            result=invoke(p,stage);self.assertEqual(result.returncode,0,(stage,result.stderr));report=json.loads(result.stdout);self.assertIs(report['executable'],False)
            uses=report['declaredFaultSourceUses'];self.assertEqual({u['site'] for u in uses},{'payment_request_step','notice_request_step'});self.assertTrue(all(u['revision']==candidate['revision'] and u['hostRequirements'] for u in uses));self.assertEqual(len(report['declaredFaultSourceCatalogues']),2)
            notice=next(c for c in report['declaredFaultSourceCatalogues'] if c['source']['id']=='urn:test:authoring-notice-faults');self.assertEqual(notice['emissions'],{'notice uncertainty':{'evidence':'unresolved','observations':['unknown']}})
            if stage in STAGES[6:]:self.assertEqual(len(report['retryFaultBindings']),1)


    def test_format_alias_package_hashes_and_inherited_capture_checks(self):
        p=package(capture=True);result=invoke(p,'check-reconciliation-availability');self.assertEqual(result.returncode,0,result.stderr);report=json.loads(result.stdout)
        self.assertEqual(len(report['declaredWaitCaptures']),1);self.assertEqual(len(report['ownedLocalFollowups']),1)
        candidate=json.loads(invoke(p).stdout);formatted=json.loads(invoke(p,'format').stdout)
        self.assertEqual(json.loads(invoke((formatted['source'],json.loads(formatted['companion']),p[2])).stdout),candidate)
        s,l,r=deepcopy(p);s=s.replace('"Notice faults"','"Notice failures"');next(v for v in l['declarations'] if v['id']=='notice_faults_binding')['name']='Notice failures'
        self.assertEqual(json.loads(invoke((s,l,r)).stdout),candidate)
        self.assertEqual(json.loads(invoke(p,'integrity').stdout),grammar.package_digests(p[0].encode(),encoded(p[1])))

    def test_grammar_schema_pair_kind_and_no_implicit_source_rebinding(self):
        p=package();old=grammar.load_grammar(ROOT/'docs/authoring/candidate-a-v0.8.ebnf');new=grammar.load_grammar(ROOT/'docs/authoring/candidate-a-v0.9.ebnf')
        self.assertEqual(set(new)-set(old),{'fault_binding'});self.assertEqual({k:v for k,v in old.items() if k not in ['work_policy','retry_binding']},{k:v for k,v in new.items() if k not in ['work_policy','retry_binding','fault_binding']})
        for change,expected in [('missing',b'fault-source-required'),('old_retry',b'syntax'),('pair',b'ledger-profile'),('kind',b'resource-format'),('wrong_provider',b'fault-source-provider')]:
            s,l,r=deepcopy(p)
            if change=='missing':s=s.replace(' Recognise faults from "Payment faults".','')
            elif change=='old_retry':s=s.replace('basis "Payable request".','basis "Payable request", fault source "Payment faults".')
            elif change=='pair':l['version']='0.7.0'
            elif change=='kind':next(v for v in l['resources'] if v['binding']=='notice_faults_binding')['kind']='provider'
            else:s=s.replace('Recognise faults from "Notice faults".','Recognise faults from "Payment faults".')
            result=invoke((s,l,r),'check');self.assertEqual(result.returncode,1);self.assertEqual(result.stdout,b'');self.assertIn(expected,result.stderr,(change,result.stderr))
        for profile in ['0.7.0','0.8.0']:
            with self.assertRaises(ValueError):grammar.recognize(p[0].replace('authoring "0.9.0"',f'authoring "{profile}"'))
        with self.assertRaises(ValueError):grammar.recognize('Choreoform authoring "1.0.0". '+'bad '*5000)

    def test_catalogue_is_shared_once_for_many_uses_and_distinct_from_retry_evidence(self):
        s,l,r=retry_dense();s,l=promote(s,l);s=recognise(s,'Retry work','Payment faults').replace(', fault source "Payment faults"','')
        result=invoke((s,l,r),'check');self.assertEqual(result.returncode,0,result.stderr);report=json.loads(result.stdout)
        self.assertEqual(len(report['declaredFaultSourceUses']),130);self.assertEqual(len(report['declaredFaultSourceCatalogues']),1)
        self.assertLess(len(result.stdout),250000);self.assertEqual(result.stdout.count(('f'*4000).encode()),1);self.assertEqual(result.stdout.count(('o'*4000).encode()),1)
        self.assertNotIn('retryFaultBindings',report)

    def test_long_pin_use_rows_refuse_before_stdout_under_unchanged_source_quota(self):
        p=dense_long_pins();self.assertLess(len(grammar.lex(p[0],program=True)),16384)
        result=invoke(p);self.assertEqual(result.returncode,0,result.stderr)
        for stage in STAGES:
            result=invoke(p,stage);self.assertEqual(result.returncode,1,(stage,result.stderr));self.assertEqual(result.stdout,b'');self.assertEqual(result.stderr.strip(),b'report-budget')

    def test_current_package_and_all_old_native_views_regenerate_unchanged(self):
        p=current();result=invoke(p);self.assertEqual(result.returncode,0,result.stderr);self.assertEqual(json.loads(result.stdout),json.loads((ROOT/'docs/benchmarks/rp01/history/source-0.7/candidate.ir.json').read_bytes()))
        for stage in STAGES:
            result=invoke(p,stage);self.assertEqual(result.returncode,0,(stage,result.stderr));report=json.loads(result.stdout)
            self.assertNotIn('declaredFaultSourceUses',report);self.assertNotIn('declaredFaultSourceCatalogues',report)


if __name__=='__main__':unittest.main()
