# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite raw snapshot proof boundaries; no actual request or host execution."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest
import jsonschema
import check_ir_fixtures as wire
from check_static_contracts import ROOT
from check_late_observation_boundaries import package
from check_metadata_authoring import invoke

CORE=ROOT/'docs/dialects/core-retained-request-v0.8.md'
SCHEMA=ROOT/'schemas/ir/definition-0.4.schema.json'
CORPUS=ROOT/'crates/semantic-check/tests/conformance/retained-request.json'

def model(p):
    # Limited under the corpus's exact type/resource/visibility/creator premises.
    if p['wire']!='0.4.0':return 'retained-request-wire-version'
    if p['cardinality']!=1:return 'retained-request-cardinality'
    if p['handler']:return 'retained-request-handler'
    if p['source']=='other':return 'retained-request-origin'
    if not p['entryCapture']:return 'retained-request-capture'
    if p['storage']=='initialized':return 'retained-request-readonly'
    if p['source']!='ordinary':return 'retained-request-source-control-unproved'
    if p['access']!='independent':return 'retained-request-access-precheck'
    if p['storage']=='public' or p['copyProtection']!='same':return 'control-information-flow'
    if not p['invalidation']:return 'owned-local-invalidation-coverage'
    return 'covered'

class RetainedRequestEvidence(unittest.TestCase):
    def test_exact_new_core_and_all_inherited_artifacts(self):
        raw=CORE.read_bytes();h=sha256(raw).hexdigest();self.assertEqual((ROOT/f'docs/ir/contracts/sha256-{h}.txt').read_bytes(),raw)
        for name in ['core-fault-entry-v0.7.md','core-emission-source-v0.6.md','core-wait-capture-v0.5.md','core-fault-source-v0.4.md','core-metadata-v0.3.md','core-retry-v0.2.md']:
            self.assertIn('sha256:'+sha256((ROOT/'docs/dialects'/name).read_bytes()).hexdigest(),raw.decode())
        self.assertIn('sha256:'+h,(ROOT/'crates/semantic-check/src/lib.rs').read_text())

    def test_only_required_scope_map_changes_structural_schema(self):
        old=json.loads((ROOT/'schemas/ir/definition-0.3.schema.json').read_bytes());new=json.loads(SCHEMA.read_bytes());d=deepcopy(new)
        d['$id']=old['$id'];d['$comment']=old['$comment'];d['properties']['version']['const']='0.3.0'
        m=d['$defs']['scope']['properties'].pop('retainedRequests');d['$defs']['scope']['required'].remove('retainedRequests');self.assertEqual(d,old)
        self.assertEqual(m,{'type':'object','propertyNames':{'$ref':'#/$defs/id'},'additionalProperties':{'$ref':'#/$defs/id'}})
        v=json.loads((ROOT/'docs/benchmarks/rp01/history/source-0.7/candidate.ir.json').read_bytes());v['version']='0.4.0'
        for s in v['body']['scopes'].values():s.update(faultInput=None,retainedRequests={})
        for n in v['body']['nodes'].values():
            if n['kind']=='wait':n['capture']=None
        v['revision']=wire.revision(v);jsonschema.Draft202012Validator(new).validate(v)
        self.assertNotEqual(v['revision'],wire.revision(dict(v,version='0.3.0')))
        for value in [None,{'wrong id':'node'},{'data':True}]:
            bad=deepcopy(v);bad['body']['scopes']['expense']['retainedRequests']=value
            with self.assertRaises(jsonschema.ValidationError):jsonschema.Draft202012Validator(new).validate(bad)

    def test_finite_model_matches_vectors_consumed_by_the_real_sdk(self):
        v=json.loads(CORPUS.read_bytes());self.assertEqual(v['format'],'choreoform-retained-request-vectors');self.assertEqual(len(v['cases']),12)
        self.assertIn('no actual request',v['premise']);self.assertEqual(len({c['id'] for c in v['cases']}),12)
        for c in v['cases']:self.assertEqual(model(c['premises']),c['expect'],c['id'])

    def test_committed_source_and_default_reports_stay_fixed(self):
        p=package()
        for stage in ['lower','check','check-control','check-protection','check-fault-routes','check-exceptional-availability','check-reconciliation-availability']:
            r=invoke(p,stage);self.assertEqual(r.returncode,0,(stage,r.stderr))
            path=ROOT/'docs/benchmarks/rp01/history/source-0.7'/('candidate.ir.json' if stage=='lower' else f'reports/{stage}.json')
            self.assertEqual(json.loads(r.stdout),json.loads(path.read_bytes()))
        self.assertEqual(p[1]['profile'],'0.7.0')

    def test_licensing_and_ci_are_explicit(self):
        notice='SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n'
        for p in [SCHEMA,CORPUS]:self.assertEqual((p.parent/(p.name+'.license')).read_text(),notice)
        self.assertIn('SPDX-License-Identifier: CC-BY-4.0',CORE.read_text());self.assertIn('tools/check_retained_request.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())

if __name__=='__main__':unittest.main()
