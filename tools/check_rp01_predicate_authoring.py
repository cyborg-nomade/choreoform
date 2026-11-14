# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Actual expense adoption, exact archived epochs and private native refusals."""
from copy import deepcopy
from hashlib import sha256
import json
from types import SimpleNamespace
import unittest
from check_authoring_evidence import check_links
from check_metadata_authoring import invoke
from check_named_data_protection import checked
from check_readonly_continuation_authoring import STAGES
from check_rp01_capture_checkpoint import BASE,ROOT,package,pre_predicate_package
from benchmarks.rp01_predicate_checkpoint import artifacts as archive,PATHS,prior_rp01_tree,prior_benchmark_inventory,unchanged_rp03_tree
from benchmarks.rp01_predicate_source import adopt,bind,Budget,template_source
from build_rp01_benchmark import capture_artifacts


class PureTestExpenseAdoption(unittest.TestCase):
    def test_archive_exact_prior_trees_four_current_changes_and_legacy_generation(self):
        old=archive();self.assertEqual(len(old),60)
        self.assertEqual((len(prior_rp01_tree()),len(unchanged_rp03_tree()),len(prior_benchmark_inventory())),(173,732,905))
        changed={name for name in PATHS if (BASE/name).read_bytes()!=old[name]}
        self.assertEqual(changed,{'source.choreo','bindings.json','manifest.json','README.md'})
        current={p.relative_to(BASE).as_posix() for p in BASE.rglob('*') if p.is_file() and not p.relative_to(BASE).as_posix().startswith('history/')}
        self.assertEqual(current,set(PATHS))
        complete={p.relative_to(BASE).as_posix() for p in BASE.rglob('*') if p.is_file()}
        self.assertEqual(complete,set(prior_rp01_tree())|{'history/source-0.18-before-predicates/'+name for name in old})
        self.assertEqual(len(complete),233)
        for name,raw in capture_artifacts(factored=True).items():
            if not name.startswith('history/'):self.assertEqual(old[name],raw,name)
        inventory=json.loads((ROOT/'docs/evaluation/0092-before-predicates-inventory.json').read_bytes())
        self.assertEqual(set(inventory),set(PATHS))
        for name,pins in inventory.items():self.assertEqual(pins,dict(bytes=len(old[name]),sha256=sha256(old[name]).hexdigest()))
        table=json.loads((ROOT/'docs/evaluation/0092-rp01-adoption-digests.json').read_bytes())
        self.assertEqual(set(table),changed)
        for name,pins in table.items():self.assertEqual(pins,dict(old=sha256(old[name]).hexdigest(),new=sha256((BASE/name).read_bytes()).hexdigest()))
        for path in ['docs/decisions/0092-rp01-pure-test-adoption.md','docs/evaluation/0092-rp01-predicate-adoption-plan.md','docs/evaluation/0092-rp01-predicate-adoption.md','docs/benchmarks/rp01/README.md']:check_links(ROOT/path)

    def test_explicit_new_generation_matches_experiment_and_preserves_all_semantic_bytes(self):
        generated=capture_artifacts(factored=True,predicates=True)
        for name,raw in generated.items():self.assertEqual((BASE/name).read_bytes(),raw,name)
        experiment=ROOT/'docs/evaluation/0090-predicate-variants/rp01'
        self.assertEqual(generated['source.choreo'],(experiment/'source.choreo').read_bytes())
        self.assertEqual(json.loads(generated['bindings.json']),json.loads((experiment/'bindings.json').read_bytes()))
        old=archive()
        for name in set(PATHS)-{'source.choreo','bindings.json','manifest.json','README.md'}:self.assertEqual((BASE/name).read_bytes(),old[name],name)
        s,l,_=package();_,previous,_=pre_predicate_package()
        self.assertEqual(s.count('test "Payment receipt agrees" using'),2)
        for table in ['declarations','symbols']:
            current={row['id']:row for row in l[table]}
            for row in previous[table]:self.assertEqual(current[row['id']],row)
        for key in set(l)-{'profile','version','declarations','symbols'}:self.assertEqual(l[key],previous[key],key)

    def test_all_eleven_current_views_exact_format_reparse_and_integrity(self):
        current=package();old=pre_predicate_package()
        self.assertEqual(checked(current),checked(old))
        for stage in STAGES:
            a,b=invoke(old,stage),invoke(current,stage)
            self.assertEqual((a.returncode,b.returncode),(0,0),(stage,a.stderr,b.stderr))
            self.assertEqual(json.loads(a.stdout),json.loads(b.stdout),stage)
            self.assertIs(json.loads(b.stdout)['executable'],False)
        f=checked(current,'format');again=f['source'],json.loads(f['companion']),current[2]
        self.assertEqual(checked(again),checked(current))
        self.assertNotEqual(checked(again,'integrity'),checked(current,'integrity'))

    def test_actual_wrong_arguments_stale_names_retirement_and_unbound_params_emit_no_candidate(self):
        s,l,r=package()
        for target,replacement,code in [
            ('"Result" is (the fact "result")','"Result" is (1)',b'source-type-mismatch'),
            ('"Instruction" is (the value of "Payable request")','"Instruction" is ("SECRET-MUST-NOT-LEAK")',b'expression-type'),
            ('Define test "Payment receipt agrees"','Define test "SECRET-MUST-NOT-LEAK"',b'binding-mismatch'),
        ]:
            self.assertEqual(s.count(target),1)
            result=invoke((s.replace(target,replacement,1),l,r))
            self.assertNotEqual(result.returncode,0);self.assertEqual(result.stdout,b'');self.assertIn(code,result.stderr)
            self.assertNotIn(b'SECRET-MUST-NOT-LEAK',result.stderr)
        stale=deepcopy(l);stale['retired'].append('payment_receipt_test_result')
        result=invoke((s,stale,r));self.assertNotEqual(result.returncode,0);self.assertEqual(result.stdout,b'');self.assertIn(b'retired-identity',result.stderr)
        clause='Let "Claim for checks" mean (the value of "Current claim") as type "Claim" from "Expense types" with parameters empty.'
        self.assertEqual(s.count(clause),1)
        bad=s.replace(clause,clause.replace('parameters empty','parameters ["unused" as text]'),1)
        result=invoke((bad,l,r));self.assertNotEqual(result.returncode,0);self.assertEqual(result.stdout,b'');self.assertIn(b'parameter-binding',result.stderr)

    def test_finite_complete_boundaries_literal_opacity_collision_and_cumulative_bounds(self):
        s,l,_=pre_predicate_package()
        body=json.loads(archive()['candidate.ir.json'])['body']
        factoring=SimpleNamespace(instruction=deepcopy(body['capabilities']['payment_capability']['input']['body']))
        out,ledger=adopt(s,l,body,factoring=factoring)
        self.assertEqual(out,package()[0]);self.assertEqual(ledger,package()[1])
        changed=deepcopy(body);changed['policies']['payment_work']['body']['completion']['else']={'op':'literal','type':{'kind':'boolean'},'value':True}
        with self.assertRaisesRegex(AssertionError,'complete receipt template'):adopt(s,l,changed,factoring=factoring)
        changed=deepcopy(body);changed['capabilities']['payment_capability']['input']['body']['fields']['claim']={'kind':'text'}
        with self.assertRaisesRegex(AssertionError,'input descriptors'):adopt(s,l,changed,factoring=factoring)
        retired=deepcopy(l);retired['retired'].append('payment_receipt_test_result')
        with self.assertRaisesRegex(AssertionError,'identity collision'):adopt(s,retired,body,factoring=factoring)
        literal={'op':'literal','type':{'kind':'record','fields':{}},'value':{'op':'read','cell':'payment_request'}}
        replaced=bind(literal,[({'op':'read','cell':'payment_request'},{'op':'param','name':'request'})],Budget())
        self.assertEqual(replaced,literal)
        with self.assertRaisesRegex(ValueError,'ambient'):template_source({'op':'fact','name':'SECRET'})
        deep={'op':'fact','name':'result'}
        for _ in range(65):deep={'op':'unwrapNamed','value':deep}
        with self.assertRaisesRegex(ValueError,'bound'):bind(deep,[],Budget())
        exhausted=Budget();exhausted.work=100000
        with self.assertRaisesRegex(ValueError,'bound'):bind(literal,[],exhausted)


if __name__=='__main__':unittest.main()
