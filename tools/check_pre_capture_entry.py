# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Limited independent static premises, not general admission or host execution."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest
import jsonschema
import check_ir_fixtures as wire
from check_static_contracts import ROOT

DIRECTORY=ROOT/'crates/semantic-check/tests/conformance'
CORE=ROOT/'docs/dialects/core-owned-pre-capture-v0.10.md'


def put(body, change):
    parts=change['path'].strip('/').split('/')
    target=body
    for part in parts[:-1]:target=target[part]
    target[parts[-1]]=deepcopy(change['value'])


def reachable(body, scope):
    # Finite local DAG reachability, no values or enabled runtime obligations.
    reached={body['scopes'][scope]['entry']}
    pending=list(reached)
    while pending:
        prior=pending.pop()
        for flow in body['flows'].values():
            if flow['source']==prior and flow['target'] not in reached:
                reached.add(flow['target']);pending.append(flow['target'])
    return reached


def model(b):
    # Only the twelve isolated, prevalidated fixture mutations. Provider/type /
    # visibility/policy/resource premises are checked by the actual Rust SDK.
    aliases={(p['id'],p['revision']) for p in b['dialects'].values()}
    if len(aliases)!=1:return 'retained-request-wire-version'
    if next(iter(aliases))[0].endswith(':0.9.0'):return 'retained-request-capture'
    s=b['scopes']['recovery']
    if not s['retainedRequests']:return 'owned-continuation-request-unproved'
    if s['entry']!='capture_wait':
        policy=b['policies'][b['capabilities']['adapter']['effects']]['body']['reconciliation']
        waits=[n for n in b['nodes'].values() if n['kind']=='wait' and n['policy']==policy]
        if len(waits)!=1 or waits[0]['scope']!='recovery' or not waits[0]['capture'] or waits[0]['capture']['origin']!='origin':return 'retained-request-source-wait'
    if b['expressions']['lookup_input']['body']!={'op':'read','cell':'original_request'}:return 'owned-continuation-query-input'
    reach=reachable(b,'recovery')
    if s['entry']!='capture_wait' and 'capture_wait' not in reach:return 'owned-continuation-capture-unreachable'
    if 'origin' not in reachable(b,'root'):return 'owned-continuation-source-unproved'
    if 'captured' in b['nodes'][s['entry']]['reads']:return 'owned-continuation-availability-unproved'
    if 'lookup' not in reach:return 'owned-continuation-source-unproved'
    if not b['data']['original_request']['invalidates']:return 'owned-continuation-invalidation-coverage'
    return 'covered'


class PreCaptureEvidence(unittest.TestCase):
    def test_exact_new_artifact_all_nine_bases_and_selected_pin(self):
        corpus=json.loads((DIRECTORY/'pre-capture-entry.json').read_bytes())
        raw=CORE.read_bytes();digest='sha256:'+sha256(raw).hexdigest()
        self.assertEqual(corpus['core']['revision'],digest)
        self.assertEqual((ROOT/'docs/ir/contracts'/(digest.replace(':','-')+'.txt')).read_bytes(),raw)
        self.assertIn(digest,(ROOT/'crates/semantic-check/src/lib.rs').read_text())
        base=json.loads((DIRECTORY/corpus['base']).read_bytes())
        cores=[p for p in base['normative'] if 'core-profile:' in p['id']]
        self.assertEqual(len(cores),9)
        for p in cores:
            self.assertIn(p['revision'],raw.decode())
            artifact=(ROOT/'docs/ir/contracts'/(p['revision'].replace(':','-')+'.txt')).read_bytes()
            self.assertEqual(p['revision'],'sha256:'+sha256(artifact).hexdigest())

    def test_shared_isolated_vectors_have_independent_limited_results_and_existing_wire(self):
        corpus=json.loads((DIRECTORY/'pre-capture-entry.json').read_bytes())
        base=json.loads((DIRECTORY/corpus['base']).read_bytes())['definition']
        self.assertEqual(len(corpus['cases']),12);self.assertEqual(len({c['id'] for c in corpus['cases']}),12)
        self.assertIn('not general admission',corpus['premise']);self.assertIn('no actual observations',corpus['premise'])
        schema=json.loads((ROOT/'schemas/ir/definition-0.4.schema.json').read_bytes())
        for case in corpus['cases']:
            d=deepcopy(base);b=d['body'];b['dialects']['core']=deepcopy(corpus['core'])
            for change in corpus['layout']+case['put']:put(b,change)
            d['revision']=wire.revision(d)
            jsonschema.Draft202012Validator(schema).validate(d)
            self.assertEqual(d['version'],'0.4.0')
            self.assertEqual(model(b),case['expect'],case['id'])

    def test_readable_selection_stays_at_previous_core_without_new_output_fields(self):
        from check_readonly_continuation_authoring import package,invoke
        p=package();d=json.loads(invoke(p).stdout)
        self.assertEqual(p[1]['profile'],'0.12.0')
        self.assertEqual(d['body']['dialects']['core']['id'],'urn:choreoform:contracts:core-profile:0.9.0')
        result=invoke(p,'check-reconciliation-availability');self.assertEqual(result.returncode,0,result.stderr)
        view=json.loads(result.stdout);self.assertFalse(view['executable']);self.assertEqual(len(view['ownedContinuationScopes']),2)
        self.assertTrue(all('preCaptureEntry' not in row for row in view['ownedContinuationScopes']))

    def test_licensing_and_required_ci(self):
        path=DIRECTORY/'pre-capture-entry.json'
        self.assertEqual(path.with_name(path.name+'.license').read_text(),'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n')
        self.assertIn('SPDX-License-Identifier: CC-BY-4.0',CORE.read_text())
        self.assertIn('tools/check_pre_capture_entry.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__=='__main__':unittest.main()
