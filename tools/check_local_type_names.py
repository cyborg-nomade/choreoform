# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Actual closed source/CLI/type expansion evidence, not editing or user evidence."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest
import jsonschema
import authoring_probe as grammar
from check_authoring_evidence import check_links
from check_metadata_authoring import invoke
from check_named_data_protection import pair as protection_pair, checked
from check_readonly_continuation_authoring import STAGES
from check_source_capacity import fixed
from check_static_contracts import ROOT


def pair(alias=True):
    s,l,r=deepcopy(protection_pair())
    s=s.replace('authoring "0.17.0"','authoring "0.18.0"');l.update(profile='0.18.0',version='0.17.0')
    if alias:
        s=s.replace('as whole number,','as type "Count",')
        s=s.replace('End process.','Describe type "Count" as whole number. End process.')
        l['declarations'].append(dict(id='count_type',kind='type-alias',scope=l['root'],name='Count'))
    return s,l,r


def add(p,identity,name,ty):
    s,l,r=p;s=s.replace('End process.',f'Describe type "{name}" as {ty}. End process.')
    l['declarations'].append(dict(id=identity,kind='type-alias',scope=l['root'],name=name))
    return s,l,r


class LocalTypeNames(unittest.TestCase):
    def test_exact_frozen_publications_schema_delta_and_native_independent_grammar(self):
        adr=(ROOT/'docs/decisions/0086-local-type-names.md').read_text()
        for p in ['docs/authoring/candidate-a-v0.18.ebnf','docs/authoring/profile-v0.18.md','schemas/authoring/bindings-0.17.schema.json']:
            self.assertIn('sha256:'+sha256((ROOT/p).read_bytes()).hexdigest(),adr)
        for p in ['docs/decisions/0086-local-type-names.md','docs/authoring/profile-v0.18.md','docs/decisions/README.md','docs/evaluation/0086-local-type-names-plan.md']:
            check_links(ROOT/p)
        old=json.loads((ROOT/'schemas/authoring/bindings-0.16.schema.json').read_bytes());new=json.loads((ROOT/'schemas/authoring/bindings-0.17.schema.json').read_bytes())
        for key in ['$id','title']:new[key]=old[key]
        for key in ['version','profile']:new['properties'][key]=old['properties'][key]
        new['properties']['declarations']['items']['properties']['kind']['enum'].remove('type-alias')
        self.assertEqual(new,old)
        s,l,r=pair();grammar.recognize(s);grammar.validate_ledger(l)
        s=s.replace('authoring "0.18.0"','authoring "0.17.0"');l.update(profile='0.17.0',version='0.16.0')
        with self.assertRaises(ValueError):grammar.recognize(s)
        with self.assertRaises(jsonschema.ValidationError):grammar.validate_ledger(l)
        self.assertIn('tools/check_local_type_names.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())

    def test_actual_cli_all_small_views_and_format_integrity_preserve_same_meaning(self):
        before=fixed();p=pair();old=pair(False)
        self.assertEqual(checked(p),checked(old))
        for stage in STAGES:
            self.assertEqual(checked(p,stage),checked(old,stage),stage);self.assertIs(checked(p,stage)['executable'],False)
        s,l,r=p;l['annotations']['future:type-detail']={'opaque':['保護',1]}
        f=checked(p,'format');q=f['source'],json.loads(f['companion']),r
        self.assertEqual(checked(q),checked(p));self.assertNotEqual(checked(q,'integrity'),checked(p,'integrity'))
        self.assertEqual(checked(q)['annotations'],l['annotations']);self.assertEqual(fixed(),before)

    def test_unused_cycles_missing_stale_kind_and_future_refuse_without_literal_echo(self):
        for change,code in [('cycle','type-alias-cycle'),('missing','reference-missing'),('kind','reference-kind'),('pair','ledger-profile'),('future','profile'),('retired','retired-identity'),('stale','binding-mismatch')]:
            p=pair();s,l,r=p
            if change=='cycle':
                s=s.replace('Describe type "Count" as whole number.','Describe type "Count" as type "Other".')
                s,l,r=add((s,l,r),'other_type','Other','type "Count"')
            elif change=='missing':s=s.replace('as type "Count",','as type "SECRET-MUST-NOT-LEAK",')
            elif change=='kind':s=s.replace('as type "Count",','as type "Reviewer",')
            elif change=='pair':l['version']='0.16.0'
            elif change=='future':s=s.replace('"0.18.0"','"0.19.0"')
            elif change=='retired':l['retired'].append('count_type')
            else:s=s.replace('Describe type "Count"','Describe type "Renamed"')
            result=invoke((s,l,r));self.assertNotEqual(result.returncode,0,change);self.assertEqual(result.stdout,b'');self.assertIn(code.encode(),result.stderr);self.assertNotIn(b'SECRET-MUST-NOT-LEAK',result.stderr)

    def test_tiny_acyclic_type_graph_refuses_before_any_expanded_candidate(self):
        p=add(pair(False),'t0','Type 0','text')
        for i in range(1,25):p=add(p,f't{i}',f'Type {i}',f'record with ["a" as type "Type {i-1}", "b" as type "Type {i-1}"]')
        self.assertLess(len(p[0].encode()),10000)
        result=invoke(p);self.assertNotEqual(result.returncode,0);self.assertEqual(result.stdout,b'');self.assertTrue(any(c in result.stderr for c in [b'type-expansion',b'work-limit',b'size',b'depth']),result.stderr)


if __name__=='__main__':unittest.main()
