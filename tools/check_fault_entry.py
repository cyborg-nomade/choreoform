# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite conditional declarations and wire/type boundaries, never live entry."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest

import jsonschema
import check_ir_fixtures as wire
from check_static_contracts import ROOT
from check_late_observation_boundaries import package
from check_metadata_authoring import invoke

CORPUS=ROOT/'crates/semantic-check/tests/conformance/fault-entry.json'
CORE=ROOT/'docs/dialects/core-fault-entry-v0.7.md'
SCHEMA=ROOT/'schemas/ir/definition-0.3.schema.json'


def model(p):
    # Limited finite model under the corpus's explicit prevalidated type,
    # resource, source-context and equal-policy/purpose/participant premises.
    if p['wire']!='0.3.0':return 'fault-entry-wire-version'
    if not p['incoming']:return 'uncovered'
    if p['source']=='unsupported':return 'fault-entry-origin-unproved'
    if p['source']!='same':return 'fault-entry-source'
    if p['body']!='flat':return 'fault-entry-body-unproved'
    if p['ancestorRead']:return 'fault-entry-storage-unproved'
    if p['entryAccessReads']:return 'fault-entry-access-precheck'
    if not p['localValueAvailable']:return 'fault-entry-availability-unproved'
    if p['historySensitivity']!='expense' or p['initializerSensitivity']!='expense':return 'control-information-flow'
    if not p['invalidates']:return 'fault-entry-invalidation-coverage'
    return 'passed'


def candidate():
    v=wire.load((ROOT/'docs/benchmarks/rp01/candidate.ir.json').read_bytes())
    v['version']='0.3.0'
    for s in v['body']['scopes'].values():s['faultInput']=None
    for n in v['body']['nodes'].values():
        if n['kind']=='wait':n['capture']=None
    v['body']['dialects']['core']={'id':'urn:choreoform:contracts:core-profile:0.7.0','revision':'sha256:'+sha256(CORE.read_bytes()).hexdigest()}
    v['revision']=wire.revision(v)
    return v


class FaultEntryEvidence(unittest.TestCase):
    def test_exact_core_bases_history_nominal_and_snapshot(self):
        raw=CORE.read_bytes();revision='sha256:'+sha256(raw).hexdigest()
        self.assertEqual((ROOT/'docs/ir/contracts'/(revision.replace(':','-')+'.txt')).read_bytes(),raw)
        self.assertIn(revision,(ROOT/'crates/semantic-check/src/lib.rs').read_text())
        # Every inherited version is still explicit and its immutable bytes exist.
        for source in ['core-emission-source-v0.6.md','core-wait-capture-v0.5.md','core-fault-source-v0.4.md','core-metadata-v0.3.md','core-retry-v0.2.md']:
            r='sha256:'+sha256((ROOT/'docs/dialects'/source).read_bytes()).hexdigest()
            self.assertIn(r,raw.decode());self.assertTrue((ROOT/'docs/ir/contracts'/(r.replace(':','-')+'.txt')).is_file())
        for source in ['provider-fault-history.md','provider-fault-history.types.json']:
            self.assertIn('sha256:'+sha256((ROOT/'docs/dialects'/source).read_bytes()).hexdigest(),raw.decode())
        self.assertIn('nominal Provider fault history',raw.decode())

    def test_new_schema_delta_is_only_required_nullable_scope_history_binding(self):
        old=json.loads((ROOT/'schemas/ir/definition-0.2.schema.json').read_bytes());new=json.loads(SCHEMA.read_bytes());delta=deepcopy(new)
        delta['$id']=old['$id'];delta['$comment']=old['$comment'];delta['properties']['version']['const']='0.2.0'
        binding=delta['$defs']['scope']['properties'].pop('faultInput');delta['$defs']['scope']['required'].remove('faultInput')
        self.assertEqual(delta,old);self.assertEqual(binding['anyOf'][0],{'type':'null'})
        self.assertEqual(binding['anyOf'][1]['required'],['history','result']);self.assertFalse(binding['anyOf'][1]['additionalProperties'])
        v=candidate();jsonschema.Draft202012Validator(new).validate(v);self.assertEqual(v['revision'],wire.revision(v))
        self.assertNotEqual(v['revision'],wire.revision(dict(v,version='0.2.0')))
        for mutation in ['missing','extra','bad_ref','unknown','old']:
            d=deepcopy(v);s=d['body']['scopes']['notice_service']
            if mutation=='missing':del s['faultInput']
            elif mutation=='extra':s['faultInput']={'history':{'id':'urn:test:history','revision':'sha256:'+'1'*64},'result':'notice_result','approved':True}
            elif mutation=='bad_ref':s['faultInput']={'history':{'id':'urn:test:history','revision':'wrong'},'result':'notice_result'}
            elif mutation=='unknown':d['version']='0.4.0'
            else:d['version']='0.2.0'
            schema=old if mutation=='old' else new
            with self.assertRaises(jsonschema.ValidationError):jsonschema.Draft202012Validator(schema).validate(d)

    def test_independent_conditional_model_matches_all_actual_sdk_vectors(self):
        v=json.loads(CORPUS.read_bytes());self.assertEqual((v['format'],v['version']),('choreoform-conditional-fault-entry-vectors','0.1.0'))
        self.assertEqual(len(v['cases']),13);self.assertEqual(len({c['id'] for c in v['cases']}),13)
        for c in v['cases']:self.assertEqual(model(c['premises']),c['expect'],c['id'])
        self.assertEqual(next(c for c in v['cases'] if c['id']=='no-incoming')['expect'],'uncovered')

    def test_current_source_and_all_default_reports_keep_the_old_wire_and_bytes(self):
        p=package()
        for stage in ['lower','check','check-control','check-protection','check-fault-routes','check-exceptional-availability','check-reconciliation-availability']:
            r=invoke(p,stage);self.assertEqual(r.returncode,0,(stage,r.stderr))
            path=ROOT/'docs/benchmarks/rp01'/('candidate.ir.json' if stage=='lower' else 'reports/'+stage+'.json')
            self.assertEqual(json.loads(r.stdout),json.loads(path.read_bytes()),stage)
        self.assertEqual(json.loads((ROOT/'docs/benchmarks/rp01/candidate.ir.json').read_bytes())['version'],'0.1.0')

    def test_licensing_ci_and_non_execution_scope(self):
        notice='SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n'
        for path in [CORPUS,SCHEMA]:self.assertEqual((path.parent/(path.name+'.license')).read_text(),notice)
        self.assertIn('SPDX-License-Identifier: CC-BY-4.0',CORE.read_text())
        self.assertIn('tools/check_fault_entry.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())
        self.assertIn('no actual history or host entry',json.loads(CORPUS.read_bytes())['premise'])


if __name__=='__main__':unittest.main()
