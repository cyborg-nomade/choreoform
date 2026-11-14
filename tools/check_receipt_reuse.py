# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Read-only factoring evidence and current native barriers, never a new language."""
from copy import deepcopy
from collections import Counter
from hashlib import sha256
import json
import subprocess
import tempfile
from pathlib import Path
import unittest
from benchmarks.receipt_reuse_evidence import ROOT,report,rebind,Budget
from check_source_capacity import fixed
from check_metadata_authoring import invoke
from check_rp01_capture_checkpoint import package as expense
from check_rp03_package import package as order
from check_rp03_factored_authoring import mutate
from check_authoring_evidence import check_links
from benchmarks.rp03_predicate_checkpoint import artifacts as source18_archive,prior_benchmark_inventory

BASE=ROOT/'docs/evaluation'


class ReceiptReuseEvidence(unittest.TestCase):
    def test_actual_native_current_metrics_reproduce_and_benchmark_tree_stays_exact(self):
        before=fixed()
        # Preserve exact captured fixture labels and bytes through a complete
        # virtual source18 current-package tree, rather than rewriting the report.
        with tempfile.TemporaryDirectory() as directory:
            base=Path(directory)/'docs/benchmarks'
            for name,raw in source18_archive().items():
                path=base/'rp03'/name;path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(raw)
            for path in (ROOT/'docs/benchmarks/rp01').rglob('*'):
                relative=path.relative_to(ROOT/'docs/benchmarks/rp01')
                if path.is_file() and 'history' not in relative.parts:
                    target=base/'rp01'/relative;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(path.read_bytes())
            result=subprocess.run([str(ROOT/'target/debug/examples/factoring_metrics'),'docs/benchmarks/rp01/source.choreo','docs/benchmarks/rp03/source.choreo'],cwd=directory,capture_output=True,timeout=30)
        self.assertEqual(result.returncode,0,result.stderr)
        self.assertEqual(result.stdout,(BASE/'0089-current-native-metrics.json').read_bytes())
        measured=json.loads(result.stdout)
        self.assertEqual([(r['sourceBytes'],r['tokens'],r['outerExpressions']['bytes'],r['outerExpressions']['repeatedBytesUpperBound']) for r in measured],[(55453,9455,22971,128),(410752,66783,204788,16268)])
        self.assertEqual(fixed(),before)
        prior=prior_benchmark_inventory()
        rows=''.join(k+'\0'+v+'\n' for k,v in sorted(prior.items()))
        self.assertEqual((len(prior),sha256(rows.encode()).hexdigest()),(778,'716f37ed07bdaeb0b05236dadb06e5e1eecf96025af58c3d7d843c9e28f19a42'))

    def test_all_actual_receipt_cases_reconstruct_exactly_with_two_complete_typed_shapes(self):
        actual=report();self.assertEqual(actual,json.loads((BASE/'0089-receipt-reuse-ast.json').read_bytes()))
        self.assertEqual((actual['capabilityActivities'],actual['distinctRequestInputs'],actual['distinctNominalResults'],actual['distinctEvidenceRepresentations'],actual['distinctResultInvariants'],actual['cases']),(25,1,23,1,23,55))
        self.assertEqual(Counter(c['comparisonLeaves'] for row in actual['rows'] for c in row['cases']),{13:26,14:29})
        self.assertEqual(len(actual['templates']),2)
        for key in ['request','activity','policy','capability']:self.assertEqual(len({r[key] for r in actual['rows']}),25)
        self.assertIn('no compiler, future byte savings',actual['interpretation'])

    def test_literal_values_are_opaque_and_measurement_work_depth_and_ops_are_closed(self):
        param={'op':'param','name':'receipt'}
        literal={'op':'literal','type':{'kind':'record','fields':{'op':{'kind':'text'},'name':{'kind':'text'}}},'value':deepcopy(param)}
        self.assertEqual(rebind(literal,[(param,{'op':'read','cell':'changed'})],Budget()),literal)
        with self.assertRaisesRegex(ValueError,'bound'):rebind(literal,[],Budget(0))
        nested={'op':'fact','name':'result'}
        for _ in range(66):nested={'op':'not','value':nested}
        with self.assertRaisesRegex(ValueError,'bound'):rebind(nested,[],Budget())
        with self.assertRaisesRegex(ValueError,'unsupported'):rebind({'op':'call','name':'unaccepted'},[],Budget())

    def test_current_native_sources_and_unbound_parameter_refusal_remain_real(self):
        for p in [expense(),order()]:
            r=invoke(p,'check');self.assertEqual(r.returncode,0,r.stderr);self.assertIs(json.loads(r.stdout)['executable'],False)
        s,l,r=expense()
        old='Let "Claim for checks" mean (the value of "Current claim") as type "Claim" from "Expense types" with parameters empty.'
        bad=mutate(s,old,old.replace('parameters empty','parameters ["unused" as text]'))
        lowered=invoke((bad,l,r));self.assertEqual(lowered.returncode,0,lowered.stderr)
        checked=invoke((bad,l,r),'check');self.assertNotEqual(checked.returncode,0);self.assertEqual(checked.stdout,b'');self.assertIn(b'parameter-binding',checked.stderr)
        refusal=invoke(order(),'check-concurrency');self.assertEqual(refusal.returncode,1);self.assertEqual(refusal.stdout,b'');self.assertIn(b'parallel-effects-unproved',refusal.stderr)
        for p in ['docs/evaluation/0089-reusable-receipt-critique-plan.md','docs/evaluation/0089-reusable-receipt-critique.md','docs/decisions/0089-reusable-receipt-direction.md']:check_links(ROOT/p)


if __name__=='__main__':unittest.main()
