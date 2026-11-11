# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Real bounded source joins, not runtime completion or ownership evidence."""
import json
from copy import deepcopy
from hashlib import sha256
from pathlib import Path
import tempfile
import unittest
import authoring_probe as grammar
from benchmarks.rp01_resources import encoded
from check_metadata_authoring import invoke
from check_readonly_continuation_authoring import STAGES
from proofs.all_join_history import package


class AllJoinHistoryEvidence(unittest.TestCase):
    def checked(self, p, stage='lower'):
        r=invoke(p,stage);self.assertEqual(r.returncode,0,(stage,r.stderr));return json.loads(r.stdout)

    def test_all_native_views_expose_conditional_exact_history_and_live_parent_coverage(self):
        p=package();grammar.recognize(p[0]);grammar.validate_ledger(p[1])
        for stage in STAGES:
            report=self.checked(p,stage);self.assertIs(report['executable'],False);self.assertTrue(report['remaining'])
            if stage in STAGES[3:]:
                history=report['historicalOrigins']
                self.assertIn({'site':'after','cell':'local','boundary':'gather','scope':'a','closure':'close','condition':'ordinary-completion-with-no-declared-descendant-writer'},history)
        f=self.checked(p,'format');q=f['source'],json.loads(f['companion']),p[2]
        self.assertEqual(self.checked(p),self.checked(q))
        self.assertEqual(self.checked(p,STAGES[-1]),self.checked(q,STAGES[-1]))
        self.assertEqual(self.checked(p,'integrity'),grammar.package_digests(p[0].encode(),encoded(p[1])))

    def test_partial_threshold_composite_and_unvisited_writer_keep_actual_refusals(self):
        for p in [package('any'),package('at least 2'),package('composite'),package(writer=True)]:
            self.checked(p,'check-protection')
            for stage in ['check-invalidation','invalidation-requirements',STAGES[-1]]:
                r=invoke(p,stage);self.assertEqual(r.returncode,1);self.assertEqual(r.stdout,b'')
                self.assertTrue(r.stderr.startswith(b'invalidation-context-unproved at /body/nodes/after;'),r.stderr)

    def test_current_parent_inputs_and_private_read_namespace_are_not_upgraded(self):
        s,l,r=package();s=s.replace('Invalidate ["After"].','Invalidate empty.')
        result=invoke((s,l,r),'check-invalidation');self.assertEqual(result.returncode,1)
        self.assertTrue(result.stderr.startswith(b'invalidation-coverage at /body/data/basis/invalidates;'),result.stderr)
        s,l,r=package();s=s.replace('the value of "Basis"','the value of "Local" in "A" in "Main"')
        result=invoke((s,l,r));self.assertEqual(result.returncode,1);self.assertEqual(result.stdout,b'')
        self.assertTrue(result.stderr.startswith(b'reference-missing at '),result.stderr)


    def test_complete_adoption_adds_only_the_real_advised_links_and_preserves_old_claims(self):
        from check_rp03_package import package as current,instance_checkpoint_package,load,BASE
        from benchmarks.rp03_instance_checkpoint import artifacts
        old=instance_checkpoint_package();new=current()
        before=self.checked(old);after=self.checked(new)
        advice=self.checked(old,'invalidation-requirements')
        self.assertEqual(len(advice['requiredInvalidationObligations']),312)
        self.assertEqual(len(advice['missingInvalidationObligations']),278)
        manifest=load(BASE/'manifest.json')
        self.assertEqual(manifest['invalidationAdoption']['basisRevision'],before['revision'])
        self.assertEqual(manifest['invalidationAdoption']['added'],advice['missingInvalidationObligations'])
        expected=deepcopy(before)
        for row in advice['missingInvalidationObligations']:
            expected['body']['data'][row['cell']]['invalidates'][row['site']]=True
        expected['revision']=after['revision']
        self.assertEqual(expected,after)
        self.assertEqual(old[1:],new[1:])
        # Protection-only output is identical apart from the new definition revision,
        # including nested declaration rows bound to that revision.
        old_protection=self.checked(old,'check-protection')
        new_protection=self.checked(new,'check-protection')
        self.assertEqual(json.loads(json.dumps(old_protection).replace(before['revision'],after['revision'])),new_protection)
        for stage in STAGES[:5]:self.assertIs(self.checked(new,stage)['executable'],False)
        final_advice=self.checked(new,'invalidation-requirements')
        self.assertEqual(final_advice['missingInvalidationObligations'],[])
        self.assertEqual(final_advice['requiredInvalidationObligations'],advice['requiredInvalidationObligations'])
        self.assertEqual(len(final_advice['historicalOrigins']),849)
        self.assertEqual(len(final_advice['uncoveredInvalidationNodes']),45)
        old_manifest=load(BASE/'history/source-0.16-instances/manifest.json')
        self.assertTrue(old_manifest['semanticStageResults'][3]['diagnostic'].startswith('invalidation-context-unproved'))
        # Historical observed refusal is preserved, not attributed to this checker.
        actual=invoke(old,'check-invalidation');self.assertEqual(actual.returncode,1)
        self.assertTrue(actual.stderr.startswith(b'invalidation-coverage at /body/data/authorize_record/invalidates;'),actual.stderr)
        h=sha256();frozen=artifacts();self.assertEqual(len(frozen),122)
        for name,raw in sorted(frozen.items()):
            if name=='README.previous.md':continue
            h.update(name.encode()+b'\0'+len(raw).to_bytes(8,'big')+raw)
        self.assertEqual(h.hexdigest(),'c3da997a63a63ef4c19631b5765c7077e32fab288f24f072808f334451e465b5')

    def test_finite_assembler_link_patch_rejects_existing_or_unknown_targets(self):
        from check_rp03_package import instance_checkpoint_package
        from benchmarks.rp03_invalidation import apply_missing
        p=instance_checkpoint_package();b=self.checked(p)['body']
        for row in [{'cell':'original_order','site':'authorize_activity'},{'cell':'original_order','site':'missing'}]:
            with self.assertRaises((AssertionError,KeyError)):
                apply_missing(p[0],p[1],b,[row])


    def test_generation_removes_only_known_stale_outputs_and_preserves_unknown_files(self):
        from build_rp03_benchmark import write_artifacts
        with tempfile.TemporaryDirectory() as folder:
            base=Path(folder);(base/'reports').mkdir()
            for name in ['candidate.ir.json','candidate.ir.json.license','reports/check-concurrency.json','reports/check-concurrency.json.license','reports/notes.md']:
                (base/name).write_bytes(b'filesystem test marker only')
            write_artifacts({'reports/check.json':b'new filesystem test marker'},base)
            self.assertEqual((base/'reports/notes.md').read_bytes(),b'filesystem test marker only')
            self.assertEqual((base/'reports/check.json').read_bytes(),b'new filesystem test marker')
            self.assertFalse((base/'candidate.ir.json').exists())
            self.assertFalse((base/'reports/check-concurrency.json').exists())
            self.assertFalse((base/'reports/check-concurrency.json.license').exists())


if __name__=='__main__':unittest.main()
