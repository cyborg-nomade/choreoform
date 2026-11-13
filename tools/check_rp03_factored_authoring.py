# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Actual source adoption, exact old bytes and active negative mutations; no host proof."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest
from check_authoring_evidence import check_links
from check_metadata_authoring import invoke
from check_rp03_package import BASE,ROOT,package,pre_factoring_package
from benchmarks.rp03_factoring_checkpoint import artifacts as archive,PATHS
from benchmarks.rp03_source import Scope,new_package
from benchmarks.rp03_resources import PEOPLE,CAPABILITIES
from benchmarks.rp01_resources import INTEGER


def mutate(source,old,new):
    if source.count(old)!=1:raise AssertionError('negative mutation needs one actual source target')
    result=source.replace(old,new,1)
    if result==source:raise AssertionError('negative mutation was a no-op')
    return result


class FactoredOrderAuthoring(unittest.TestCase):
    def test_exact_archive_all_current_non_authoring_bytes_and_other_benchmark_stay_fixed(self):
        old=archive();self.assertEqual(len(old),127)
        changed={name for name in PATHS if (BASE/name).read_bytes()!=old[name]}
        self.assertEqual(changed,{'source.choreo','bindings.json','manifest.json','README.md'})
        self.assertEqual((BASE/'candidate.ir.json').read_bytes(),old['candidate.ir.json'])
        self.assertEqual(sha256(old['candidate.ir.json']).hexdigest(),'6550ac457539457b6e48a452d17d95060db1a2f36fbcf74f8b3c7c1e7c11cfb4')
        self.assertEqual(json.loads(old['manifest.json'])['sourceBytes'],576759)
        paths=sorted(p for p in (ROOT/'docs/benchmarks/rp01').rglob('*') if p.is_file())
        rows=''.join(p.relative_to(ROOT/'docs/benchmarks/rp01').as_posix()+'\0'+sha256(p.read_bytes()).hexdigest()+'\n' for p in paths)
        self.assertEqual(len(paths),113)
        self.assertEqual(sha256(rows.encode()).hexdigest(),'62ebf5abf58d73a682a3f69271ba00521065539451195f0324d357c16f677c9b')
        for path in ['docs/decisions/0087-rp03-factored-authoring.md','docs/evaluation/0087-rp03-factored-authoring-plan.md','docs/benchmarks/rp03/README.md']:
            check_links(ROOT/path)
        self.assertIn('tools/check_rp03_factored_authoring.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())

    def test_actual_format_reparse_and_historical_lowering_keep_exact_complete_document(self):
        p=package();current=invoke(p);self.assertEqual(current.returncode,0,current.stderr)
        old=invoke(pre_factoring_package());self.assertEqual(old.returncode,0,old.stderr)
        self.assertEqual(json.loads(current.stdout),json.loads(old.stdout))
        result=invoke(p,'format');self.assertEqual(result.returncode,0,result.stderr);formatted=json.loads(result.stdout)
        again=invoke((formatted['source'],json.loads(formatted['companion']),p[2]));self.assertEqual(again.returncode,0,again.stderr)
        self.assertEqual(json.loads(again.stdout),json.loads(current.stdout))
        self.assertEqual(p[0].count('protected as "Order information"'),155)
        self.assertEqual(p[0].count('Describe type "Order instruction"'),1)
        self.assertEqual(p[0].count('Protect data as "Order information"'),1)
        self.assertEqual((p[1]['profile'],p[1]['version']),('0.18.0','0.17.0'))

    def test_new_current_negative_mutations_really_change_definitions_and_refuse(self):
        source,ledger,resources=package()
        for old,new,code in [
            ('Purposes ["Order fulfillment"]','Purposes empty',b'data-purposes'),
            ('Describe type "Order instruction"','Describe type "SECRET-MUST-NOT-LEAK"',b'binding-mismatch'),
        ]:
            bad=mutate(source,old,new);r=invoke((bad,ledger,resources),'check')
            self.assertNotEqual(r.returncode,0);self.assertEqual(r.stdout,b'');self.assertIn(code,r.stderr);self.assertNotIn(b'SECRET-MUST-NOT-LEAK',r.stderr)
        bad=mutate(source,'"target warehouse" as optional text','"target warehouse" as optional whole number')
        lower=invoke((bad,ledger,resources));self.assertEqual(lower.returncode,0,lower.stderr)
        changed=json.loads(lower.stdout);original=json.loads((BASE/'candidate.ir.json').read_bytes())
        self.assertNotEqual(changed['revision'],original['revision'])
        r=invoke((bad,ledger,resources),'check');self.assertNotEqual(r.returncode,0);self.assertEqual(r.stdout,b'')
        self.assertFalse(r.stderr.startswith(b'parallel-effects-unproved'),r.stderr)
        with self.assertRaises(AssertionError):mutate(source,'missing source clause','replacement')

    def test_finite_producer_refuses_changed_complete_envelope_and_freezes_type_description(self):
        p=new_package(factored=True);root=Scope(p,'order','Order fulfillment',None,'Done')
        root.protection_definition('order_data_protection','Order information',PEOPLE,CAPABILITIES)
        ty=deepcopy(INTEGER);root.instruction_definition('order_instruction_type','Order instruction',ty)
        before=deepcopy(p.ledger)
        with self.assertRaises(AssertionError):root.data('wrong','Wrong',INTEGER,participants=PEOPLE,capabilities=CAPABILITIES[:-1])
        self.assertEqual(p.ledger,before)
        p.sensitivity='changed after the definition'
        with self.assertRaises(AssertionError):root.data('wrong','Wrong',INTEGER,participants=PEOPLE,capabilities=CAPABILITIES)
        self.assertEqual(p.ledger,before)
        ty['kind']='text';self.assertEqual(root.type_text(ty),'text');self.assertEqual(p.instruction_description,INTEGER)


if __name__=='__main__':unittest.main()
