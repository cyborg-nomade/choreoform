# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite generated finish-name audit, not full symbol or semantic validation."""
from copy import deepcopy
import json
import unittest
from unittest.mock import patch

import authoring_probe as grammar
import build_rp03_benchmark as builder
from check_compensation_authoring import CORE,current,order,checked
from check_metadata_authoring import invoke
from check_readonly_continuation_authoring import STAGES
from check_source_capacity import fixed
from check_static_contracts import ROOT


def finish_audit(*,historical=True):
    captured=[];original=builder.build
    def capture(*args,**kwargs):
        source,package=original(*args,**kwargs);captured.append(package);return source,package
    with patch.object(builder,'build',side_effect=capture):files=builder.historical_artifacts() if historical else builder.package_artifacts()
    assert len(captured)==1
    p=captured[0];names={(r['owner'],r['role'],r['name']) for r in p.ledger['symbols']}
    total=0;failures=[]
    for node,(scope,caption,action) in p.actions.items():
        if not action.startswith('Finish as '):continue
        total+=1;name=json.loads(action[len('Finish as '):-1])
        if (scope,'outcome',name) not in names:
            failures.append(dict(node=node,scope=scope,name=name,declared=[(r['name'],r['wire']) for r in p.ledger['symbols'] if r['owner']==scope and r['role']=='outcome']))
    return files,total,failures


def bounded(interface,finish,symbol):
    source=f'''Choreoform authoring "0.16.0".
Process "Audit".
Start at "Done".
Inputs empty. Outputs empty. Outcomes [{json.dumps(interface)}].
Use cancellation "Cancel", faults "Faults", closure "Close" and races "Race".
Rule for actor "Authority". Permit when (true). End policy.
Require actor "Operator" under "Authority".
Rule for cancel "Cancel". Authority "Operator". For children use "Settlement". End policy.
Rule for settlement "Settlement". Unfinished work cancel. End policy.
Rule for closure "Close". Fully terminal. End policy.
Rule for faults "Faults". Handle empty. End policy.
Rule for race "Race". Accept in acceptance order. End policy.
Step "Done". Finish as {json.dumps(finish)}. End step.
End process.
'''
    ledger=dict(format='choreoform-authoring-bindings',profile='0.16.0',version='0.15.0',definition='bounded_symbol_audit',root='root',contracts={'semantics':{'contract':'urn:choreoform:semantics:adr-0008','digest':'sha256:0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141'},'core':deepcopy(CORE)},declarations=[dict(id='root',kind='scope',scope=None,name='Audit'),dict(id='done',kind='node',scope='root',name='Done'),dict(id='operator',kind='actor',scope='root',name='Operator')],symbols=[dict(id='root_outcome',owner='root',role='outcome',name=symbol,wire='authorization-recorded')],generated=[],resources=[],retired=[],annotations={})
    for identity,name in [('authority','Authority'),('cancel','Cancel'),('settlement','Settlement'),('close','Close'),('faults','Faults'),('race','Race')]:
        ledger['declarations'].append(dict(id=identity,kind='policy',scope='root',name=name))
    return source,ledger,[]


class OrderSymbolBindingEvidence(unittest.TestCase):
    def test_all_56_generated_finishes_have_one_exact_original_name_mismatch(self):
        before=fixed();files,total,failures=finish_audit()
        for path,raw in files.items():self.assertEqual(raw,(builder.BASE/'history/source-0.14'/path).read_bytes(),path)
        self.assertEqual(total,56)
        self.assertEqual(failures,[dict(node='accept_payment_done',scope='accept_payment',name='authorization recorded',declared=[('authorization-recorded','authorization-recorded')])])
        self.assertEqual(fixed(),before)

    def test_bounded_matching_caption_variants_keep_identical_wire_and_all_views(self):
        for caption in ['authorization-recorded','authorization recorded']:
            p=bounded(caption,caption,caption);grammar.recognize(p[0]);grammar.validate_ledger(p[1])
            self.assertEqual(checked(p,'lower')['body']['nodes']['done']['outcome'],'authorization-recorded')
            for stage in STAGES:self.assertIs(checked(p,stage)['executable'],False)

    def test_companion_rename_alias_extra_wire_or_wrong_role_does_not_repair_source(self):
        for symbol in ['authorization-recorded','authorization recorded']:
            p=bounded('authorization-recorded','authorization recorded',symbol)
            x=invoke(p);self.assertEqual(x.returncode,1);self.assertEqual(x.stdout,b'');self.assertEqual(x.stderr,b'symbol-mismatch at 0..0\n')
        s,l,r=bounded('authorization-recorded','authorization recorded','authorization-recorded')
        alias=deepcopy(l['symbols'][0]);alias.update(id='alias',name='authorization recorded');l['symbols'].append(alias)
        self.assertEqual(invoke((s,l,r)).stderr,b'symbol-duplicate at 0..0\n')
        l['symbols'][1]['wire']='alternate_wire'
        self.assertEqual(invoke((s,l,r)).stderr,f'source-ledger-inventory at 0..{len(s.encode())}\n'.encode())
        l['symbols'].pop();l['symbols'][0]['role']='branch'
        self.assertEqual(invoke((s,l,r)).stderr,b'symbol-owner at 0..0\n')

    def test_full_original_source_and_runtime_refusal_stay_fixed(self):
        before=fixed();p=current(order());x=invoke(p)
        self.assertEqual(x.returncode,1);self.assertEqual(x.stdout,b'');self.assertEqual(x.stderr,b'symbol-mismatch at 0..0\n')
        s,l,r=deepcopy(p);next(row for row in l['symbols'] if row['owner']=='accept_payment' and row['role']=='outcome')['name']='authorization recorded'
        x=invoke((s,l,r));self.assertEqual(x.returncode,1);self.assertEqual(x.stdout,b'');self.assertEqual(x.stderr,b'symbol-mismatch at 0..0\n')
        self.assertEqual(fixed(),before)
        self.assertIn('tools/check_rp03_symbol_binding.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__=='__main__':unittest.main()
