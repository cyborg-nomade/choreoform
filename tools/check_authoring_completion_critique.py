# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Real native measurements/erasure/refusal evidence; no UI, user or host proof."""
from hashlib import sha256
import json
import subprocess
import unittest
from check_authoring_evidence import check_links
from check_metadata_authoring import invoke
from check_rp01_capture_checkpoint import package,pre_predicate_package
from check_source_capacity import fixed
from check_static_contracts import ROOT


class AuthoringCompletionCritique(unittest.TestCase):
    def test_actual_current_native_capture_and_complete_benchmark_inventory_are_exact(self):
        before=fixed()
        r=subprocess.run([str(ROOT/'target/debug/examples/factoring_metrics'),'docs/benchmarks/rp01/source.choreo','docs/benchmarks/rp03/source.choreo'],cwd=ROOT,capture_output=True,timeout=30)
        self.assertEqual(r.returncode,0,r.stderr)
        self.assertEqual(r.stdout,(ROOT/'docs/evaluation/0093-current-native-metrics.json').read_bytes())
        rows=json.loads(r.stdout)
        self.assertEqual([(x['sourceBytes'],x['tokens'],x['parserWork'],x['ruleCounts']['scope'],x['ruleCounts']['step'],x['outerExpressions']['bytes'],x['outerExpressions']['repeatedBytesUpperBound']) for x in rows],[(54172,9145,113796,8,46,21428,128),(291309,41306,386333,59,198,84431,7156)])
        tree=''.join(name+'\0'+pin+'\n' for name,pin in sorted(before.items()))
        self.assertEqual((len(before),sha256(tree.encode()).hexdigest()),(965,'a25c6de850fdaa51436023fffe011c8f96695e9d8660191e7598cd4f4bf2e9f7'))
        self.assertEqual(fixed(),before)

    def test_actual_equal_ir_does_not_uniquely_recover_pure_authoring_structure(self):
        before=fixed();old,current=pre_predicate_package(),package()
        a,b=invoke(old),invoke(current)
        self.assertEqual((a.returncode,b.returncode),(0,0),(a.stderr,b.stderr))
        self.assertEqual(a.stdout,b.stdout)
        fa,fb=invoke(old,'format'),invoke(current,'format')
        self.assertEqual((fa.returncode,fb.returncode),(0,0),(fa.stderr,fb.stderr))
        xa,xb=json.loads(fa.stdout),json.loads(fb.stdout)
        self.assertNotEqual(xa['source'],xb['source'])
        self.assertNotIn('Define test',xa['source']);self.assertIn('Define test',xb['source'])
        self.assertEqual(invoke((xa['source'],json.loads(xa['companion']),old[2])).stdout,a.stdout)
        self.assertEqual(invoke((xb['source'],json.loads(xb['companion']),current[2])).stdout,b.stdout)
        self.assertEqual(fixed(),before)

    def test_actual_private_invalid_argument_has_the_measured_declaration_span(self):
        s,l,r=package();target='"Instruction" is (the value of "Payable request")'
        self.assertEqual(s.count(target),1)
        bad=s.replace(target,'"Instruction" is ("SECRET-MUST-NOT-LEAK")',1)
        result=invoke((bad,l,r));self.assertEqual(result.returncode,1)
        self.assertEqual(result.stdout,b'')
        self.assertEqual(result.stderr,b'expression-type at 8900..9585\n')
        self.assertNotIn(b'SECRET-MUST-NOT-LEAK',result.stderr)
        self.assertEqual(9585-8900,685)

    def test_direction_links_explicit_remaining_phase_gates_and_ci_registration(self):
        for name in ['docs/evaluation/0093-authoring-completion-critique-plan.md','docs/evaluation/0093-authoring-completion-critique.md','docs/decisions/0093-authoring-completion-sequence.md','ROADMAP.md','docs/evaluation/phase-1-progress.md']:
            check_links(ROOT/name)
        roadmap=(ROOT/'ROADMAP.md').read_text()
        for text in ['- [ ] Design an initial visual notation','- [ ] Implement complete structural/link/semantic validation','- [ ] Build text → IR → text','- [ ] Publish small executable examples','- [ ] Complete the executable IR contract']:
            self.assertIn(text,roadmap)
        for text in ['RP-08','representative users','0093-authoring-completion-sequence.md']:
            self.assertIn(text,roadmap)
        self.assertIn('tools/check_authoring_completion_critique.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())
        self.assertIn('SPDX-License-Identifier: CC-BY-4.0',(ROOT/'docs/evaluation/0093-current-native-metrics.json.license').read_text())


if __name__=='__main__':unittest.main()
