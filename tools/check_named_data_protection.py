# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Independent schema/CLI/package checks, not an editor or user evidence."""
from copy import deepcopy
from hashlib import sha256
import json
import jsonschema
import unittest
import authoring_probe as grammar
from check_authoring_evidence import check_links
from check_compensation_authoring import CORE
from check_metadata_authoring import invoke
from check_readonly_continuation_authoring import STAGES
from check_source_capacity import fixed
from check_static_contracts import ROOT

GROUP='Protect data as "Shared protection". Under "Access". Sensitivity "test". Purposes ["Use"]. Participants ["Reviewer"]. Capabilities empty. End protection.\n'
INLINE='Keep "Value" as whole number under "Access". Sensitivity "test". Purposes ["Use"]. Participants ["Reviewer"]. Capabilities empty. Invalidate empty. Initially unavailable. End data.\n'
NAMED='Keep "Value" as whole number, protected as "Shared protection". Invalidate empty. Initially unavailable. End data.\n'


def pair(named=True):
    s=(ROOT/'docs/authoring/examples/terminal.choreo').read_text().replace('authoring "0.2.0"','authoring "0.17.0"')
    l=json.loads((ROOT/'docs/authoring/examples/terminal.bindings.json').read_bytes())
    l.update(profile='0.17.0',version='0.16.0');l['contracts']['core']=deepcopy(CORE)
    for identity,kind,name in [('access','policy','Access'),('value','data','Value')]:
        l['declarations'].append(dict(id=identity,kind=kind,scope=l['root'],name=name))
    l['symbols'].append(dict(id='value_purpose',owner='value',role='purpose',name='Use',wire='use'))
    s=s.replace('End process.','Rule for protection "Access". Permit access when (true). End policy.\n'+(NAMED if named else INLINE)+(GROUP if named else '')+'End process.')
    if named:
        l['declarations'].append(dict(id='shared',kind='data-protection',scope=l['root'],name='Shared protection'))
        l['symbols'].append(dict(id='shared_purpose',owner='shared',role='purpose',name='Use',wire='use'))
    return s,l,[]


def checked(p,stage='lower'):
    r=invoke(p,stage)
    if r.returncode:raise AssertionError((stage,r.stderr))
    return json.loads(r.stdout)


class NamedDataProtection(unittest.TestCase):
    def test_frozen_contract_hashes_closed_schema_and_actual_grammar(self):
        adr=(ROOT/'docs/decisions/0084-named-data-protection.md').read_text()
        for path in ['docs/authoring/candidate-a-v0.17.ebnf','docs/authoring/profile-v0.17.md','schemas/authoring/bindings-0.16.schema.json']:
            self.assertIn('sha256:'+sha256((ROOT/path).read_bytes()).hexdigest(),adr)
        for path in ['docs/authoring/profile-v0.17.md','docs/decisions/0084-named-data-protection.md','docs/evaluation/0084-named-data-protection-plan.md','docs/decisions/README.md']:
            check_links(ROOT/path)
        old=json.loads((ROOT/'schemas/authoring/bindings-0.15.schema.json').read_bytes())
        new=json.loads((ROOT/'schemas/authoring/bindings-0.16.schema.json').read_bytes())
        new['$id']=old['$id'];new['title']=old['title']
        for key in ['profile','version']:new['properties'][key]=old['properties'][key]
        new['properties']['declarations']['items']['properties']['kind']['enum'].remove('data-protection')
        self.assertEqual(old,new)
        s,l,_=pair();grammar.recognize(s);grammar.validate_ledger(l)
        for bad in [s.replace('Capabilities empty.','',1),s.replace('Purposes ["Use"].','Purposes ["Use"]. Purposes ["Use"].',1)]:
            with self.assertRaises(ValueError):grammar.recognize(bad)
            self.assertNotEqual(invoke((bad,l,[])).returncode,0)
        self.assertIn('tools/check_named_data_protection.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())

    def test_real_lowering_all_views_and_format_integrity_preserve_expanded_meaning(self):
        p=pair();old=pair(False);before=fixed()
        self.assertEqual(checked(p),checked(old))
        for stage in STAGES:
            self.assertEqual(checked(p,stage),checked(old,stage),stage)
            self.assertIs(checked(p,stage)['executable'],False)
        s,l,r=p;l['annotations']['future:opaque']={'unknown':['value',1]}
        f=checked(p,'format');q=f['source'],json.loads(f['companion']),r
        self.assertEqual(checked(q),checked(p));self.assertNotEqual(checked(q,'integrity'),checked(p,'integrity'))
        self.assertEqual(checked(q)['annotations'],l['annotations']);self.assertEqual(fixed(),before)

    def test_old_profiles_stale_missing_kind_and_exact_purpose_inventory_refuse_privately(self):
        for change,code in [('old','binding-kind'),('pair','ledger-profile'),('purpose','protection-purpose-bindings'),('retired','retired-identity'),('unknown','ledger-fields'),('reference','reference-missing'),('future','profile')]:
            s,l,r=pair()
            if change=='old':s=s.replace('"0.17.0"','"0.16.0"');l.update(profile='0.16.0',version='0.15.0')
            elif change=='pair':l['version']='0.15.0'
            elif change=='purpose':next(x for x in l['symbols'] if x['id']=='value_purpose')['wire']='different'
            elif change=='retired':l['retired'].append('shared')
            elif change=='unknown':l['implicitDefault']='shared'
            elif change=='reference':s=s.replace('protected as "Shared protection"','protected as "SECRET-MUST-NOT-LEAK"')
            elif change=='future':s=s.replace('"0.17.0"','"0.19.0"')
            result=invoke((s,l,r));self.assertNotEqual(result.returncode,0,change);self.assertEqual(result.stdout,b'')
            # Old syntax refuses during parse before its ledger can be inspected.
            if change!='old':self.assertIn(code.encode(),result.stderr)
            self.assertNotIn(b'SECRET-MUST-NOT-LEAK',result.stderr)
        s,l,r=pair();l['profile']='0.16.0';l['version']='0.15.0'
        with self.assertRaises(jsonschema.ValidationError):grammar.validate_ledger(l)

        # Source 0.17 also validates the inherited sensitivity-name bound in
        # unused definitions, which have no wire record for later validation.
        s,l,r=pair();s=s.replace(NAMED,INLINE).replace(GROUP,GROUP.replace('Sensitivity "test"','Sensitivity "'+'x'*4097+'"'))
        result=invoke((s,l,r));self.assertNotEqual(result.returncode,0);self.assertEqual(result.stdout,b'');self.assertIn(b'protection-sensitivity',result.stderr);self.assertNotIn(b'x'*100,result.stderr)

    def test_short_alias_source_hits_output_bound_without_partial_stdout_or_literal_echo(self):
        s,l,r=pair();s=s.replace('Sensitivity "test"','Sensitivity "'+'x'*4000+'"')
        additions=[]
        for i in range(300):
            additions.append(NAMED.replace('"Value"',f'"Value {i}"'))
            l['declarations'].append(dict(id=f'v{i}',kind='data',scope=l['root'],name=f'Value {i}'))
            l['symbols'].append(dict(id=f'vp{i}',owner=f'v{i}',role='purpose',name='Use',wire='use'))
        s=s.replace('End process.',''.join(additions)+'End process.');self.assertLess(len(s.encode()),100000)
        result=invoke((s,l,r));self.assertNotEqual(result.returncode,0);self.assertEqual(result.stdout,b'');self.assertIn(b'size at',result.stderr);self.assertNotIn(b'x'*100,result.stderr)
        s,l,r=pair();s=s.replace('as whole number,','as '+'optional '*65+'text,')
        result=invoke((s,l,r));self.assertNotEqual(result.returncode,0);self.assertEqual(result.stdout,b'')


if __name__=='__main__':unittest.main()
