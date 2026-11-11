# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Complete generated source/IR evidence, never a host/user/settlement witness."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest

import authoring_probe as grammar
import check_ir_fixtures as wire
from build_rp03_benchmark import BASE, artifacts
from build_rp03_checkpoint import package_artifacts as old_artifacts
from check_metadata_authoring import invoke
from check_rp03_package import package, historical_package, pre_instance_package, load
from check_readonly_continuation_authoring import STAGES


class OrderInstanceEvidence(unittest.TestCase):
    def checked(self, p, stage='lower'):
        r=invoke(p,stage);self.assertEqual(r.returncode,0,(stage,r.stderr));return json.loads(r.stdout)

    def test_complete_current_ir_reports_and_every_parent_relationship(self):
        p=package();m=load(BASE/'manifest.json');d=self.checked(p);self.assertEqual(d,load(BASE/'candidate.ir.json'))
        self.assertEqual(d['revision'],wire.revision(d))
        self.assertEqual(m['canonicalIR']['digest'],'sha256:'+sha256((BASE/'candidate.ir.json').read_bytes()).hexdigest())
        b=d['body'];calls=[];splits=[]
        for identity,node in b['nodes'].items():
            if node['kind']=='invoke':calls.append((identity,node['scope'],node['body']))
            if node['kind']=='split':splits += [(identity,node['scope'],body) for body in node['children'].values()]
        self.assertEqual((len(calls),len(splits)),(26,7))
        for _,scope,body in calls+splits:self.assertEqual(b['scopes'][body]['parent'],scope)
        self.assertEqual(len(m['instanceMap']),25)
        for row in m['instanceMap']:
            self.assertEqual(b['nodes'][row['creator']]['body'],row['body'])
            self.assertEqual(b['nodes'][row['instance']+'_activity']['target'],row['capability'])
            self.assertEqual(b['policies'][row['instance']+'_effects']['body']['reconciliation'],row['instance']+'_wait')
        for stage in STAGES[:5]:
            r=self.checked(p,stage);self.assertIs(r['executable'],False);self.assertTrue(r['remaining'])
            self.assertEqual(r,load(BASE/'reports'/f'{stage}.json'))
        for stage in STAGES[5:]:
            r=invoke(p,stage);self.assertEqual(r.returncode,1);self.assertEqual(r.stdout,b'')
            self.assertTrue(r.stderr.startswith(b'parallel-effects-unproved at /body/nodes/start_acceptance;'),r.stderr)

    def test_original_and_pre_instance_archives_reproduce_exact_old_bytes_and_pins(self):
        for historical,folder,expected_digest in [(True,'source-0.14','797d01f0b1a42db5c71de9bc7a054d58e24333798b23e6cde22225dc4cc707ee'),(False,'source-0.16-pre-instances','895ff7e48499bef7cdccb070b6d072c1d8ffb21833b9232407cf25caf3492ae0')]:
            files=old_artifacts(historical=historical);h=sha256();self.assertEqual(len(files),113)
            for name,raw in sorted(files.items()):
                self.assertEqual((BASE/'history'/folder/name).read_bytes(),raw)
                h.update(name.encode()+b'\0'+len(raw).to_bytes(8,'big')+raw)
            self.assertEqual(h.hexdigest(),expected_digest)
        for p,code in [(historical_package(),b'tokens at 108626..108627'),(pre_instance_package(),b'child-template at 0..0')]:
            r=invoke(p);self.assertEqual(r.returncode,1);self.assertEqual(r.stdout,b'');self.assertEqual(r.stderr.strip(),code)
            for _,revision,raw in p[2]:self.assertEqual(revision,'sha256:'+sha256(raw).hexdigest())

    def test_only_explicit_synthetic_recipient_claims_change_resource_payloads(self):
        old=old_artifacts();current={p.name:p.read_bytes() for p in BASE.iterdir() if p.is_file()}
        added={'cancel_release_a_capability','cancel_release_b_capability'}
        for name,raw in old.items():
            if name.endswith('.provider.json') or name=='clock-source.json':
                a=json.loads(raw);b=json.loads(current[name]);caps=b['resultProtection']['capabilities']
                self.assertEqual(set(caps)-set(a['resultProtection']['capabilities']),added)
                for cap in added:del caps[cap]
                self.assertEqual(a,b,name)
        for name in ['types.json','launch-values.json','receipt-rules.json','receipt-cases.json','guard-rules.json','guard-cases.json']:
            self.assertEqual(old[name],current[name],name)
        # Exact old resources do not supply newly declared recipient claims.
        s,l,_=deepcopy(package());old_resources=pre_instance_package()[2];pins={i:v for i,v,_ in old_resources}
        for row in l['resources']:row['digest']=pins[row['contract']]
        r=invoke((s,l,old_resources),'check');self.assertEqual(r.returncode,1)
        self.assertTrue(r.stderr.startswith(b'information-flow at '),r.stderr);self.assertEqual(r.stdout,b'')

    def test_format_and_integrity_preserve_the_full_candidate_and_actual_refusal(self):
        p=package();f=self.checked(p,'format');q=f['source'],json.loads(f['companion']),p[2]
        self.assertEqual(self.checked(p),self.checked(q))
        self.assertEqual(self.checked(p,'check-protection'),self.checked(q,'check-protection'))
        self.assertEqual(self.checked(p,'integrity'),grammar.package_digests(p[0].encode(),json.dumps(p[1]).encode()))
        r=invoke(q,'check-concurrency');self.assertEqual(r.returncode,1);self.assertIn(b'parallel-effects-unproved',r.stderr)

    def test_exact_generation_inventory_no_stale_reports_or_supported_scenario_claims(self):
        generated=artifacts()
        self.assertEqual({p.relative_to(BASE).as_posix() for p in BASE.rglob('*') if p.is_file()},set(generated)|{'README.md'})
        for name,raw in generated.items():self.assertEqual((BASE/name).read_bytes(),raw,name)
        m=load(BASE/'manifest.json');self.assertIs(m['executable'],False)
        self.assertTrue(all(s['status']=='Partial' for s in m['scenarios']));self.assertEqual(len(m['scenarios']),4)
        self.assertEqual(len(list((BASE/'reports').glob('*.json'))),5)
        self.assertEqual(len(load(BASE/'receipt-cases.json')['cases']),22)
        self.assertEqual(len(load(BASE/'guard-cases.json')['cases']),8)
        self.assertIn('tools/check_rp03_instances.py',(BASE.parents[2]/'.github/workflows/rust-portability.yml').read_text())


if __name__=='__main__':unittest.main()
