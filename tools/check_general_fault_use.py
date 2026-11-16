# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite declared-source contract checks; no accepted faults or complete validator."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest

import jsonschema
import check_ir_fixtures as wire
from check_metadata_authoring import invoke
from check_static_contracts import ROOT

PIN='27bce3fde9a0393abb6bf760a8588af9268de65069b73f4cb946144f5d30cb85'
CORE6={'id':'urn:choreoform:contracts:core-profile:0.6.0','revision':'sha256:'+PIN}
CORPUS=ROOT/'crates/semantic-check/tests/conformance/general-fault-use.json'


def expected(case):
    """Independent rules over the prevalidated metadata/graph fixture only."""
    if case['legacy']:return 'fields'
    if not case['resources'] and (case['outer']!='none' or case['retrySource']):return 'resource-missing'
    if case['attempts']>1:
        same=(case['outer']=='same' and case['retrySource']) or (case['outer']=='none' and not case['retrySource'])
        if not same:return 'fault-source-retry-consistency'
        if not case['retrySource']:return 'retry-fault-source-required'
    elif case['retrySource']:return 'retry-fault-source-unused'
    if case['outer']!='none':
        if case['mode']!='capability':return 'fault-source-context-unproved'
        if case['provider']!='same':return 'fault-source-provider'
    # The explicit unresolved-eligible case has prevalidated stable-key metadata;
    # none of these declarations establish actual key support/current eligibility.
    return 'passed'


class GeneralFaultUseEvidence(unittest.TestCase):
    def test_exact_contract_bytes_inherited_bases_and_closed_corpus(self):
        raw=(ROOT/f'docs/ir/contracts/sha256-{PIN}.txt').read_bytes();self.assertEqual(sha256(raw).hexdigest(),PIN);self.assertEqual(raw,(ROOT/'docs/dialects/core-emission-source-v0.6.md').read_bytes())
        for p in ['50007ac05e5e71df91bb9997a40a06c7a8993df04b1a673421e4a987014299d7','0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3','30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3','5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623','26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58']:
            self.assertIn(p.encode(),raw);self.assertEqual(sha256((ROOT/f'docs/ir/contracts/sha256-{p}.txt').read_bytes()).hexdigest(),p)
        self.assertEqual(json.loads(CORPUS.read_bytes())['version'],'0.1.0');self.assertEqual(CORPUS.with_suffix('.json.license').read_text(),'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n')

    def test_all_thirteen_independent_declaration_vectors(self):
        cases=json.loads(CORPUS.read_bytes())['cases'];self.assertEqual(len(cases),13);self.assertEqual(len({c['id'] for c in cases}),13)
        for c in cases:self.assertEqual(expected(c),c['expect'],c['id'])

    def test_wire_hash_promotion_adds_only_core_work_source_and_nullable_wait_fields(self):
        old=json.loads((ROOT/'docs/benchmarks/rp01/history/source-0.7/candidate.ir.json').read_bytes());new=deepcopy(old);new['version']='0.2.0';new['body']['dialects']['core']=CORE6
        for p in new['body']['policies'].values():
            if p['body']['kind']=='work':p['body']['faultSource']=deepcopy(p['body']['retry']['faultSource'])
        for n in new['body']['nodes'].values():
            if n['kind']=='wait':n['capture']=None
        new['revision']=wire.revision(new);self.assertNotEqual(old['revision'],new['revision']);jsonschema.Draft202012Validator(wire.SCHEMA_V2).validate(new)
        restored=deepcopy(new);restored['version']='0.1.0';restored['revision']=old['revision'];restored['body']['dialects']['core']=old['body']['dialects']['core']
        for p in restored['body']['policies'].values():
            if p['body']['kind']=='work':del p['body']['faultSource']
        for n in restored['body']['nodes'].values():
            if n['kind']=='wait':del n['capture']
        self.assertEqual(restored,old)

    def test_old_package_and_source_selection_do_not_implicitly_adopt_new_core(self):
        from check_late_observation_boundaries import package as current
        from check_owned_local_followups import package as v8
        p=current();result=invoke(p);self.assertEqual(result.returncode,0,result.stderr)
        self.assertEqual(json.loads(result.stdout),json.loads((ROOT/'docs/benchmarks/rp01/history/source-0.7/candidate.ir.json').read_bytes()))
        s,l,r=v8();l['contracts']['core']={'contract':CORE6['id'],'digest':CORE6['revision'],'dialect':'core'}
        result=invoke((s,l,r));self.assertEqual(result.returncode,1);self.assertEqual(result.stdout,b'');self.assertIn(b'contract-pin',result.stderr)


if __name__=='__main__':unittest.main()
