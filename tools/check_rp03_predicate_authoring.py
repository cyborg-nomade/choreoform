# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Real current adoption, exact complete epochs and native refusals; no host proof."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest
from check_authoring_evidence import check_links
from check_metadata_authoring import invoke
from check_named_data_protection import checked
from check_readonly_continuation_authoring import STAGES
from check_rp03_package import BASE,ROOT,package,pre_predicate_package
from benchmarks.rp03_predicate_checkpoint import artifacts as archive,PATHS,prior_benchmark_inventory,prior_rp03_tree
from benchmarks.rp03_predicate_source import PureReceiptSource,template_source
from benchmarks.rp03_source import matched_completion,read
from benchmarks.rp03_resources import types,providers
from build_rp03_benchmark import package_artifacts


class PureTestOrderAdoption(unittest.TestCase):
    def test_archive_full_prior_tree_only_four_changed_current_files_and_legacy_producer(self):
        old=archive();self.assertEqual(len(old),127)
        self.assertEqual(len(prior_benchmark_inventory()),778)
        changed={name for name in PATHS if (BASE/name).read_bytes()!=old[name]}
        self.assertEqual(changed,{'source.choreo','bindings.json','manifest.json','README.md'})
        current={p.relative_to(BASE).as_posix() for p in BASE.rglob('*') if p.is_file() and not p.relative_to(BASE).as_posix().startswith('history/')}
        self.assertEqual(current,set(PATHS))
        complete={p.relative_to(BASE).as_posix() for p in BASE.rglob('*') if p.is_file()}
        expected_tree=set(prior_rp03_tree())|{'history/source-0.18-before-predicates/'+name for name in old}
        self.assertEqual(complete,expected_tree)
        self.assertEqual(len(complete),732)
        for name,raw in package_artifacts().items():self.assertEqual(raw,old[name],name)
        expected=json.loads((ROOT/'docs/evaluation/0091-before-predicates-inventory.json').read_bytes())
        self.assertEqual(set(expected),set(PATHS))
        for name,pins in expected.items():self.assertEqual(pins,dict(bytes=len(old[name]),sha256=sha256(old[name]).hexdigest()))
        changes=json.loads((ROOT/'docs/evaluation/0091-rp03-adoption-digests.json').read_bytes())
        self.assertEqual(set(changes),changed)
        for name,pins in changes.items():self.assertEqual(pins,dict(old=sha256(old[name]).hexdigest(),new=sha256((BASE/name).read_bytes()).hexdigest()))
        for path in ['docs/decisions/0091-rp03-pure-test-adoption.md','docs/evaluation/0091-rp03-predicate-adoption-plan.md','docs/evaluation/0091-rp03-predicate-adoption.md','docs/benchmarks/rp03/README.md']:check_links(ROOT/path)

    def test_new_finite_production_matches_experiment_and_changes_no_semantic_or_resource_byte(self):
        actual=package_artifacts(predicates=True)
        for name,raw in actual.items():self.assertEqual((BASE/name).read_bytes(),raw,name)
        experiment=ROOT/'docs/evaluation/0090-predicate-variants/rp03'
        self.assertEqual(actual['source.choreo'],(experiment/'source.choreo').read_bytes())
        self.assertEqual(json.loads(actual['bindings.json']),json.loads((experiment/'bindings.json').read_bytes()))
        old=archive()
        for name in set(PATHS)-{'source.choreo','bindings.json','manifest.json','README.md'}:self.assertEqual((BASE/name).read_bytes(),old[name],name)
        s,l,_=package();_,previous,_=pre_predicate_package()
        for table in ['declarations','symbols']:
            new={r['id']:r for r in l[table]}
            for row in previous[table]:self.assertEqual(new[row['id']],row)
        for key in set(l)-{'profile','version','declarations','symbols'}:self.assertEqual(l[key],previous[key],key)
        self.assertEqual(s.count('test "Receipt agrees" using'),26)
        self.assertEqual(s.count('test "Reasoned receipt agrees" using'),29)

    def test_actual_all_eleven_views_format_integrity_and_private_refusal_keep_old_results(self):
        p=package();old=pre_predicate_package()
        self.assertEqual(checked(p),checked(old))
        for stage in STAGES:
            a,b=invoke(old,stage),invoke(p,stage)
            self.assertEqual(a.returncode,b.returncode,(stage,a.stderr,b.stderr))
            if a.returncode:
                self.assertEqual(b.stdout,b'')
                self.assertEqual(a.stderr.split(b'; source bytes ')[0],b.stderr.split(b'; source bytes ')[0])
                self.assertEqual(b.stderr,b'parallel-effects-unproved at /body/nodes/start_acceptance; source bytes 58784..58971\n')
            else:self.assertEqual(json.loads(a.stdout),json.loads(b.stdout),stage)
        f=checked(p,'format');q=f['source'],json.loads(f['companion']),p[2]
        self.assertEqual(checked(q),checked(p))
        self.assertNotEqual(checked(q,'integrity'),checked(p,'integrity'))

    def test_actual_new_calls_unused_definitions_and_bindings_have_private_native_refusals(self):
        source,l,r=package()
        # All targets are unique complete declarations or the first real call.
        for old,new,code in [
            ('"Operation" as text, "Phase" as text','"Operation" as whole number, "Phase" as text',b'expression-type'),
            ('Describe type "Receipt fields"','Describe type "SECRET-MUST-NOT-LEAK"',b'binding-mismatch'),
        ]:
            # Both tests share this signature; mutate the first exact definition.
            s=source.replace(old,new,1);self.assertNotEqual(s,source)
            result=invoke((s,l,r));self.assertNotEqual(result.returncode,0);self.assertEqual(result.stdout,b'')
            self.assertIn(code,result.stderr);self.assertNotIn(b'SECRET-MUST-NOT-LEAK',result.stderr)
        needle='"Operation" is ("reserve_a")'
        self.assertIn(needle,source)
        s=source.replace(needle,'"Operation" is (42)',1)
        result=invoke((s,l,r));self.assertNotEqual(result.returncode,0);self.assertEqual(result.stdout,b'');self.assertIn(b'source-type-mismatch',result.stderr)
        stale=deepcopy(l);stale['retired'].append('receipt_test_receipt')
        result=invoke((source,stale,r));self.assertNotEqual(result.returncode,0);self.assertEqual(result.stdout,b'');self.assertIn(b'retired-identity',result.stderr)

    def test_finite_producer_refuses_changed_whole_shapes_instruction_and_nominal_representations(self):
        _,_,pin=types();docs=providers(pin)
        original=docs['reserve_a'][0]
        tags=list(original['output']['cases'])
        request=read('Request')
        completion=matched_completion('reserve_a',request,tags)
        def attempt(ast=completion,provider=original):
            return PureReceiptSource(pin).completion('one',ast,'reserve_a',request,tags,provider)
        self.assertIn('test "Receipt agrees"',attempt())
        changed=deepcopy(original);changed['input']['fields']['scope key']={'kind':'integer'}
        with self.assertRaisesRegex(AssertionError,'complete instruction'):attempt(provider=changed)
        changed=deepcopy(completion);changed['then']['op']='or'
        with self.assertRaises((AssertionError,ValueError)):attempt(ast=changed)
        recipes=PureReceiptSource(pin);recipes.receipt['fields']['operation']={'kind':'integer'}
        with self.assertRaisesRegex(AssertionError,'complete receipt'):recipes.completion('one',completion,'reserve_a',request,tags,original)
        with self.assertRaisesRegex(ValueError,'finite pure'):template_source({'op':'fact','name':'SECRET'})


if __name__=='__main__':unittest.main()
