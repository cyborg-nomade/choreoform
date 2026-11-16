# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite declared creator-parent audit; not complete validation or live ownership."""
from collections import Counter
import unittest
from unittest.mock import patch
import authoring_probe as grammar
import build_rp03_checkpoint as builder
from benchmarks.rp01_source import Package,Scope
from check_compensation_authoring import CORE,checked
from check_metadata_authoring import invoke
from check_readonly_continuation_authoring import STAGES
from check_rp03_package import pre_instance_package as package
from check_source_capacity import fixed
from check_static_contracts import ROOT


def relationships():
    captured=[];original=builder.build
    def capture(*args,**kwargs):
        source,p=original(*args,**kwargs);captured.append(p);return source,p
    with patch.object(builder,'build',side_effect=capture):files=builder.package_artifacts()
    assert len(captured)==1
    p=captured[0];scopes={r['id']:r for r in p.ledger['declarations'] if r['kind']=='scope'}
    names={(r['scope'],r['name']):r['id'] for r in scopes.values()}
    def resolve(scope,name):
        while scope is not None:
            if (scope,name) in names:return names[scope,name]
            scope=scopes[scope]['scope']
        raise ValueError('fixture scope not found')
    calls=[];splits=[];other=[]
    for node,(scope,_,action) in p.actions.items():
        tokens=grammar.lex(action)
        if tokens[0]==('WORD','Run'):
            assert tokens[2]==('WORD','with')  # This finite fixture uses unqualified bodies.
            body=resolve(scope,tokens[1][1]);calls.append((node,scope,body,scopes[body]['scope']))
        elif tokens[:2]==(('WORD','Start'),('WORD','together')):
            for i,token in enumerate(tokens):
                if token==('WORD','runs'):
                    body=resolve(scope,tokens[i+1][1]);splits.append((node,scope,body,scopes[body]['scope']))
        elif tokens[0] in [('WORD','While'),('WORD','For')]:other.append(node)
    return files,calls,splits,other


def bounded(body='C',again=False):
    p=Package(definition='bounded_template_audit',root='main',profile='0.16.0',version='0.15.0',core=CORE)
    p.ledger['annotations']={'synthetic':True,'fixture':'template-audit'}
    def scope(identity,name,parent,entry):return Scope(p,identity,name,parent,entry,cancellation='Cancel',closure='Close',race='Race')
    root=scope('main','Main',None,'Use A')
    root.actor('operator','Operator','Authority')
    root.policy('authority','actor','Authority','Permit when (true).')
    root.policy('cancel','cancel','Cancel','Authority "Operator". For children use "Settlement".')
    root.policy('settlement','settlement','Settlement','Unfinished work cancel.')
    root.policy('close','closure','Close','Fully terminal.')
    root.policy('faults','faults','Unhandled faults','Handle empty.')
    root.policy('race','race','Race','Accept in acceptance order.')
    root.step('use_a','Use A','Run "A" with inputs empty and outputs empty.',[('complete','Use A again' if again else 'Use B')])
    if again:root.step('again','Use A again','Run "A" with inputs empty and outputs empty.',[('complete','Use B')])
    root.step('use_b','Use B','Run "B" with inputs empty and outputs empty.',[('complete','Main done')])
    root.step('main_done','Main done','Finish as "complete".')
    a=scope('a','A',root,'A done');a.step('a_done','A done','Finish as "complete".');root.child(a)
    b=scope('b','B',root,'Use C');b.step('use_c','Use C',f'Run {body} with inputs empty and outputs empty.' if body.startswith('"') else f'Run "{body}" with inputs empty and outputs empty.',[('complete','B done')]);b.step('b_done','B done','Finish as "complete".')
    c=scope('c','C',b,'C done');c.step('c_done','C done','Finish as "complete".');b.child(c);root.child(b)
    return 'Choreoform authoring "0.16.0".\n'+root.render()+'\n',p.ledger,[]


class OrderTemplateBindingEvidence(unittest.TestCase):
    def test_all_26_direct_invocations_and_seven_split_children_have_explicit_parent_evidence(self):
        before=fixed();files,calls,splits,other=relationships()
        for path,raw in files.items():self.assertEqual(raw,(builder.BASE/'history/source-0.16-pre-instances'/path).read_bytes(),path)
        self.assertEqual(len(calls),26);self.assertEqual(len(splits),7);self.assertEqual(other,[])
        wrong=[r for r in calls if r[1]!=r[3]];self.assertEqual(len(wrong),23)
        self.assertIn(('authorize_call','accept_payment','authorize_service','order'),wrong)
        self.assertEqual({r[0] for r in calls if r[1]==r[3]},{'capture_call','notify_call','run_release'})
        self.assertTrue(all(r[1]==r[3] for r in splits))
        counts=Counter(r[2] for r in wrong);self.assertEqual({k:v for k,v in counts.items() if v>1},{'release_a_service':2,'release_b_service':2})
        self.assertEqual(fixed(),before)

    def test_direct_children_and_repeated_same_parent_creators_keep_all_partial_views(self):
        for again in [False,True]:
            p=bounded(again=again);grammar.recognize(p[0]);grammar.validate_ledger(p[1])
            for stage in STAGES:self.assertIs(checked(p,stage)['executable'],False)

    def test_visible_sibling_scope_and_missing_wrong_kind_body_have_distinct_refusals(self):
        for body,code in [('A','child-template'),('Missing','reference-missing'),('Operator','reference-kind')]:
            p=bounded(body);x=invoke(p);self.assertEqual(x.returncode,1);self.assertEqual(x.stdout,b'')
            self.assertTrue(x.stderr.startswith(code.encode()+b' at '),x.stderr)
        self.assertEqual(invoke(bounded('"C" in "B" in "Main"')).returncode,0)
        s,l,r=bounded();s=s.replace('Step "Use A". Run "A"','Step "Use A". Run "C" in "B" in "Main"')
        x=invoke((s,l,r));self.assertEqual(x.returncode,1);self.assertTrue(x.stderr.startswith(b'reference-missing at '),x.stderr)

    def test_real_current_source_and_history_keep_the_same_actual_refusal(self):
        before=fixed();x=invoke(package());self.assertEqual((x.returncode,x.stdout,x.stderr),(1,b'',b'child-template at 0..0\n'))
        self.assertEqual(fixed(),before)
        self.assertIn('tools/check_rp03_template_binding.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__=='__main__':unittest.main()
