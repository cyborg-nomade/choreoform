# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite capture schema/binding/dataflow oracle, not a full validator or runtime."""
from copy import deepcopy
from hashlib import sha256
import json
from pathlib import Path
import tempfile
import unittest

import jsonschema
import rfc8785
import check_ir_fixtures as wire
from check_static_contracts import ROOT,run
from check_terminal_observation_sources import normalized_type

CORPUS=ROOT/'crates/semantic-check/tests/conformance/wait-capture.json'
CORE5='urn:choreoform:contracts:core-profile:0.5.0'
PIN='50007ac05e5e71df91bb9997a40a06c7a8993df04b1a673421e4a987014299d7'


def changed(corpus,case):
    d=deepcopy(corpus['definition']);d['version']=case['wire']
    for edit in case['changes']:
        parts=edit['path'].strip('/').split('/');value=d['body']
        for key in parts[:-1]:value=value[key]
        value[parts[-1]]=deepcopy(edit['value'])
    d['revision']=wire.revision(d)
    return d


def expected(corpus,d,stage):
    """Independent finite model atop this prevalidated noncapture fixture only."""
    schema=wire.SCHEMA_V2 if d['version']=='0.2.0' else wire.SCHEMA
    if list(jsonschema.Draft202012Validator(schema).iter_errors(d)):return {'code':'structure'}
    b=d['body'];provider=corpus['resources']['provider'];source=corpus['resources']['source']
    provider_pin={'id':provider['id'],'revision':'sha256:'+sha256(rfc8785.dumps(provider)).hexdigest()}
    source_pin={'id':source['id'],'revision':'sha256:'+sha256(rfc8785.dumps(source)).hexdigest()}
    for n in b['nodes'].values():
        if n['kind']=='wait' and n.get('capture'):
            c=n['capture']
            if c['origin'] not in b['nodes'] or c['result'] not in b['data']:return {'code':'structure'}
    for identity,n in sorted(b['nodes'].items()):
        if n['kind']=='decision':
            required=set().union(*(set(b['expressions'][e]['reads']) for e in n['guards'].values()))
            if not required<=set(n['reads']):return {'code':'node-reads'}
        c=n.get('capture')
        if n['kind']!='wait' or c is None:continue
        p=b['policies'][n['policy']]
        if b['dialects'][p['dialect']]['id']!=CORE5:return {'code':'wait-capture-dialect'}
        if p['body']['timer'] is not None:return {'code':'wait-capture-timer-unproved'}
        origin=b['nodes'][c['origin']]
        if origin['kind']!='activity' or origin['mode']!='capability':return {'code':'wait-capture-origin-unproved'}
        if c['source']!=source_pin:return {'code':'resource-missing'}
        cap=b['capabilities'][origin['target']]
        actual=b['policies'][cap['contract']]['body']['provider']
        if actual!=source['provider'] or actual!=provider_pin:return {'code':'wait-capture-provider'}
        ty=normalized_type(provider)
        if p['body']['observationType']!=ty or b['data'][c['result']]['type']['body']!=ty:return {'code':'wait-capture-type'}
        if set(n['writes'])!={c['result']}:return {'code':'wait-capture-writes'}
        s,t=provider['resultProtection'],b['data'][c['result']]['protection']
        if any(s[k]!=t[k] for k in ['policy','sensitivity']) or any(not set(t[k])<=set(s[k]) for k in ['purposes','participants','capabilities']):return {'code':'information-flow'}
    if stage=='static':return {'passed':True}
    # Definite initialization and cell influence over the finite ordinary DAG.
    # These states are static sets, not values, accepted events or engine states.
    scope=b['root'];nodes={i:n for i,n in b['nodes'].items() if n['scope']==scope}
    predecessors={i:set() for i in nodes};targets={i:set() for i in nodes}
    for flow in b['flows'].values():
        if flow['source'] in nodes and flow['target'] in nodes:
            predecessors[flow['target']].add(flow['source']);targets[flow['source']].add(flow['target'])
    pending={i:len(p) for i,p in predecessors.items()};ready=sorted(i for i,n in pending.items() if n==0);order=[]
    while ready:
        i=ready.pop();order.append(i)
        for t in targets[i]:
            pending[t]-=1
            if pending[t]==0:ready.append(t)
    after={};influence={}
    for identity in order:
        n=nodes[identity]
        if identity==b['scopes'][scope]['entry']:state=set();pc=set()
        else:
            prior=[after[p] for p in predecessors[identity] if p in after]
            if not prior:continue
            state=set.intersection(*prior);pc=set().union(*(influence[p] for p in predecessors[identity] if p in influence))
        if not set(n['reads'])<=state:return {'code':'data-availability-unproved'}
        if stage=='invalidation' and n['kind'] in ['activity','wait','decision']:
            for cell in pc|set(n['reads']):
                if identity not in b['data'][cell]['invalidates']:return {'code':'invalidation-coverage'}
        after[identity]=state|set(n['writes']);influence[identity]=pc|set(n['reads'])|set(n['writes'])
    return {'passed':True}


class WaitCaptureEvidence(unittest.TestCase):
    def test_versioned_schema_delta_is_only_explicit_wait_capture_and_header(self):
        old=wire.SCHEMA;new=deepcopy(wire.SCHEMA_V2)
        self.assertEqual(new['properties']['version']['const'],'0.2.0')
        self.assertEqual(old['properties']['version']['const'],'0.1.0')
        wait=next(n for n in new['$defs']['node']['oneOf'] if n['properties']['kind']['const']=='wait')
        self.assertIn('capture',wait['required']);wait['required'].remove('capture');del wait['properties']['capture'];del new['$defs']['waitCapture']
        for key in ['$id','$comment']:new[key]=old[key]
        new['properties']['version']['const']='0.1.0'
        self.assertEqual(new,old)
        raw=(ROOT/f'docs/ir/contracts/sha256-{PIN}.txt').read_bytes()
        self.assertEqual(sha256(raw).hexdigest(),PIN);self.assertEqual(raw,(ROOT/'docs/dialects/core-wait-capture-v0.5.md').read_bytes())
        self.assertIn(PIN,(ROOT/'crates/semantic-check/src/lib.rs').read_text())

    def test_exact_revision_resources_and_all_seventeen_finite_vectors(self):
        corpus=json.loads(CORPUS.read_bytes());d=corpus['definition']
        self.assertEqual(d['revision'],wire.revision(d));self.assertEqual(d['revision'],'sha256:1e56e06815769c00f8fac99a463712bdf3544bbec115c57c4d48dc9acf5f9296')
        self.assertEqual(len(corpus['cases']),17)
        for c in corpus['cases']:
            self.assertEqual(expected(corpus,changed(corpus,c),c['stage']),c['expect'],c['id'])
        alternate=deepcopy(d);alternate['version']='0.1.0';self.assertNotEqual(wire.revision(alternate),d['revision'])

    def test_closed_shapes_and_unknown_versions_cannot_select_a_capture(self):
        corpus=json.loads(CORPUS.read_bytes());original=corpus['definition']
        for mutation in ['missing','extra','incomplete','bad_ref','old_wire','unknown']:
            d=deepcopy(original);c=d['body']['nodes']['capture_wait']
            if mutation=='missing':del c['capture']
            elif mutation=='extra':c['capture']['trusted']=True
            elif mutation=='incomplete':del c['capture']['origin']
            elif mutation=='bad_ref':c['capture']['source']['revision']='wrong'
            elif mutation=='old_wire':d['version']='0.1.0'
            else:d['version']='0.4.0'
            schema=wire.SCHEMA_V2 if d['version']=='0.2.0' else wire.SCHEMA
            with self.assertRaises(jsonschema.ValidationError):jsonschema.Draft202012Validator(schema).validate(d)
        self.assertEqual(Path(str(CORPUS)+'.license').read_text(),'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n')

    def test_current_source_package_stays_on_old_wire_and_rejects_new_capture_syntax(self):
        base=ROOT/'docs/benchmarks/rp01';m=json.loads((base/'manifest.json').read_bytes());candidate=json.loads((base/'candidate.ir.json').read_bytes())
        self.assertEqual(candidate['version'],'0.1.0');self.assertEqual(m['sourceProfile'],'0.7.0')
        self.assertTrue(all('capture' not in n for n in candidate['body']['nodes'].values() if n['kind']=='wait'))
        supplied=[(r['id'],r['revision'],ROOT/r['path']) for r in m['resources']]
        result=run('lower',base/'source.choreo',base/'bindings.json',supplied)
        self.assertEqual(result.returncode,0,result.stderr);self.assertEqual(json.loads(result.stdout),candidate)
        source=(base/'source.choreo').read_text().replace('Wait under "Payment reconciliation".','Wait under "Payment reconciliation" and record in "Recorded payment result".')
        with tempfile.TemporaryDirectory() as directory:
            p=Path(directory)/'unsupported.choreo';p.write_text(source);result=run('lower',p,base/'bindings.json',supplied)
        self.assertEqual(result.stdout,b'');self.assertIn(b'syntax at ',result.stderr)


if __name__=='__main__':unittest.main()
