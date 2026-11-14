# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Actual declaration boundary controls; no provider truth, conflicts or run traces."""
from hashlib import sha256
import json
import unittest
import authoring_probe as grammar
from benchmarks.rp01_resources import encoded
from check_metadata_authoring import invoke
from check_readonly_continuation_authoring import STAGES
from check_rp03_package import package as whole
from check_source_capacity import fixed
from check_static_contracts import ROOT
from proofs.parallel_effects import package


class ParallelEffectsAuditEvidence(unittest.TestCase):
    def checked(self,p,stage='lower'):
        r=invoke(p,stage);self.assertEqual(r.returncode,0,(stage,r.stderr));return json.loads(r.stdout)

    def test_complete_supported_controls_keep_later_limits_and_no_execution(self):
        for modes in [('read-only','read-only'),('wait','wait'),('compensatable','silent'),('irreversible','silent')]:
            p=package(*modes);grammar.recognize(p[0]);grammar.validate_ledger(p[1])
            report=self.checked(p,'check-concurrency')
            self.assertEqual(report['checkedForks'],['fork']);self.assertIs(report['executable'],False)
            self.assertIn('domain-and-irreversible-race-invariant-proofs',report['remaining'])
            for stage in STAGES[:5]:self.assertIs(self.checked(p,stage)['executable'],False)

    def test_mixed_opaque_and_effectful_work_refuses_even_with_guarded_true(self):
        for modes in [('compensatable','read-only'),('read-only','irreversible'),('irreversible','compensatable'),('compensatable','compensatable'),('human','wait'),('human','read-only')]:
            for guarded in [False,True]:
                p=package(*modes,guarded=guarded)
                body=self.checked(p)['body']
                self.assertEqual(body['policies']['race']['body']['mode'],'guarded' if guarded else 'acceptance')
                for stage in STAGES[:5]:self.assertIs(self.checked(p,stage)['executable'],False)
                for stage in ['check-concurrency','check-reconciliation-availability']:
                    r=invoke(p,stage);self.assertEqual(r.returncode,1);self.assertEqual(r.stdout,b'')
                    self.assertTrue(r.stderr.startswith(b'parallel-effects-unproved at /body/nodes/fork;'),r.stderr)

    def test_fresh_exact_provider_claims_and_source_roundtrip_are_not_host_evidence(self):
        p=package();d=self.checked(p)
        providers=[json.loads(raw) for _,_,raw in p[2]];self.assertEqual(len(providers),2)
        self.assertNotEqual(providers[0]['id'],providers[1]['id'])
        for identity,revision,raw in p[2]:
            self.assertEqual(revision,'sha256:'+sha256(raw).hexdigest())
            self.assertEqual(json.loads(raw)['id'],identity)
        f=self.checked(p,'format');q=f['source'],json.loads(f['companion']),p[2]
        self.assertEqual(self.checked(q),d)
        self.assertEqual(self.checked(q,'check-concurrency'),self.checked(p,'check-concurrency'))
        self.assertEqual(self.checked(p,'integrity'),grammar.package_digests(p[0].encode(),encoded(p[1])))

    def test_actual_whole_case_and_all_benchmark_bytes_remain_unchanged(self):
        before=fixed();p=whole()
        for stage in STAGES[:5]:self.assertIs(self.checked(p,stage)['executable'],False)
        for stage in STAGES[5:]:
            r=invoke(p,stage);self.assertEqual(r.returncode,1);self.assertEqual(r.stdout,b'')
            self.assertEqual(r.stderr.strip(),b'parallel-effects-unproved at /body/nodes/start_acceptance; source bytes 58784..58971')
        provider_fields={'kind','version','id','input','output','outcomes','resultProtection','observations','idempotency','effectClass'}
        providers=[doc for identity,_,raw in p[2] if identity.startswith('urn:choreoform:benchmarks:rp03:') for doc in [json.loads(raw)] if doc.get('kind')=='provider-contract']
        self.assertEqual(len(providers),24)
        self.assertTrue(all(set(doc)==provider_fields for doc in providers))
        self.assertEqual(fixed(),before)
        self.assertIn('tools/check_parallel_effects_audit.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__=='__main__':unittest.main()
