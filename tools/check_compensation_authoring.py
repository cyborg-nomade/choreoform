# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Exact readable prior identity; no eligible effects or full-case admission."""
from copy import deepcopy
import json
import unittest
import authoring_probe as grammar
from check_static_contracts import ROOT
from check_finite_work_authoring import package as small
from check_compensation_references import package as sibling, refusal
from check_rp03_package import historical_package as order
from check_source_capacity import promote, fixed
from check_metadata_authoring import invoke
from check_readonly_continuation_authoring import STAGES
from benchmarks.rp01_resources import encoded

CORE={'contract':'urn:choreoform:contracts:core-profile:0.12.0', 'digest':'sha256:73158731c33084fde009abcc86530926447902609fda9535f04daa7c768c2007', 'dialect':'core'}
PRIOR='"Create payable" in "Root" in "Main"'


def current(p):
    p=deepcopy(p)
    s,l,r=promote(p) if p[1]['profile']=='0.14.0' else p
    s=s.replace('authoring "0.15.0"','authoring "0.16.0"',1)
    l.update(profile='0.16.0',version='0.15.0');l['contracts']['core']=deepcopy(CORE)
    return s,l,r


def human(p):
    s,l,r=deepcopy(p)
    s=s.replace('Scope "Recovery" within "Root".', 'Step "Human target". Ask "Operator" to decide using "Request" under "Work" and record in nothing. On "done", continue with "Owner done". End step.\nScope "Recovery" within "Root".')
    l['declarations'].append(dict(id='human_target',kind='node',scope='root',name='Human target'))
    l['symbols'].append(dict(id='human_target_outcome_done',owner='human_target',role='outcome',name='done',wire='done'))
    l['generated'].append(dict(id='human_next',owner='human_target',role='flow',key='human_target_outcome_done'))
    return s,l,r


def checked(p,stage='check'):
    result=invoke(p,stage)
    if result.returncode:raise AssertionError((stage,result.stderr))
    return json.loads(result.stdout)


class CompensationAuthoringEvidence(unittest.TestCase):
    def test_closed_grammar_schema_and_all_small_reports_preserve_inherited_meanings(self):
        old=promote(small());new=current(old)
        grammar.recognize(new[0]);grammar.validate_ledger(new[1])
        a=checked(old,'lower');b=checked(new,'lower')
        expected=deepcopy(a);expected['revision']=b['revision'];expected['body']['dialects']['core']={'id':CORE['contract'],'revision':CORE['digest']}
        self.assertEqual(expected,b)
        def replace(v):
            if isinstance(v,str):return {a['revision']:b['revision'],old[1]['contracts']['core']['contract']:CORE['contract'],old[1]['contracts']['core']['digest']:CORE['digest']}.get(v,v)
            if isinstance(v,list):return [replace(x) for x in v]
            if isinstance(v,dict):return {k:replace(x) for k,x in v.items()}
            return v
        for stage in STAGES:
            x=checked(old,stage);y=checked(new,stage);self.assertEqual(replace(x),y,stage);self.assertIs(y['executable'],False)
        old_schema=json.loads((ROOT/'schemas/authoring/bindings-0.14.schema.json').read_bytes());new_schema=json.loads((ROOT/'schemas/authoring/bindings-0.15.schema.json').read_bytes())
        for key in ['$id','title']:new_schema[key]=old_schema[key]
        for key in ['version','profile']:new_schema['properties'][key]=old_schema['properties'][key]
        new_schema['properties']['contracts']['properties']['core']=old_schema['properties']['contracts']['properties']['core']
        self.assertEqual(new_schema,old_schema)
        self.assertEqual(grammar.load_grammar(ROOT/'docs/authoring/candidate-a-v0.15.ebnf'),grammar.load_grammar(ROOT/'docs/authoring/candidate-a-v0.16.ebnf'))
        self.assertEqual((ROOT/'schemas/authoring/bindings-0.15.schema.json.license').read_bytes(),(ROOT/'schemas/authoring/bindings-0.14.schema.json.license').read_bytes())

    def test_exact_private_sibling_identity_formats_reparses_and_has_all_conditional_views(self):
        old=sibling(PRIOR);refusal(old,'reference-missing',PRIOR)
        p=current(old);grammar.recognize(p[0]);grammar.validate_ledger(p[1]);doc=checked(p,'lower')
        self.assertEqual(doc['body']['policies']['audit_work']['body']['compensates'],'origin')
        formatted=checked(p,'format');q=formatted['source'],json.loads(formatted['companion']),p[2]
        self.assertEqual(checked(q,'lower'),doc)
        self.assertNotEqual(checked(p,'integrity')['source'],checked(q,'integrity')['source'])
        for stage in STAGES:self.assertIs(checked(p,stage)['executable'],False)

    def test_full_paths_unqualified_privacy_kind_and_human_mode_have_real_refusals(self):
        for reference,code in [
            ('"Create payable"','reference-missing'),
            ('"Create payable" in "Root"','reference-qualification'),
            ('"Create payable" in "Foreign"','reference-qualification'),
            ('"Create payable" in "Main"','reference-missing'),
            ('"Create payable" in "Missing" in "Main"','reference-missing'),
            ('"Create payable" in "Operator" in "Main"','reference-kind'),
            ('"Origin value" in "Root" in "Main"','reference-kind'),
            ('"Main done" in "Main"','compensation-target'),
        ]:refusal(current(sibling(reference)),code,reference)
        self.assertEqual(invoke(human(current(sibling())),'check').returncode,0)
        ref='"Human target" in "Root" in "Main"'
        refusal(human(current(sibling(ref))),'compensation-target',ref)

    def test_qualified_names_disambiguate_shadows_without_caption_search_or_normalization(self):
        s,l,r=current(sibling(PRIOR))
        s=s.replace('"Audit done"','"Create payable"')
        next(row for row in l['declarations'] if row['id']=='audit_done')['name']='Create payable'
        self.assertEqual(checked((s,l,r),'lower')['body']['policies']['audit_work']['body']['compensates'],'origin')
        ref='"Create payable"';s=s.replace('Compensates '+PRIOR+'.','Compensates '+ref+'.')
        refusal((s,l,r),'compensation-target',ref)
        s,l,r=current(sibling(PRIOR));s=s.replace('"Root"','"Róót"');next(row for row in l['declarations'] if row['id']=='root')['name']='Róót'
        self.assertEqual(checked((s,l,r),'lower')['body']['policies']['audit_work']['body']['compensates'],'origin')
        bad='"Create payable" in "Róót" in "Main"'
        s=s.replace('Compensates "Create payable" in "Róót" in "Main".','Compensates '+bad+'.')
        refusal((s,l,r),'reference-missing',bad)

    def test_other_private_slots_and_owned_immediate_source_stay_closed(self):
        s,l,r=current(sibling(PRIOR))
        old='Complete when (true). Compensates '+PRIOR+'. End policy.\nStep "Audit done"'
        for expression,reference in [
            ('(the value of "Origin value" in "Root" in "Main") equals (0)','"Origin value" in "Root" in "Main"'),
            ('(outcome "done" of '+PRIOR+') equals (outcome "done" of '+PRIOR+')',PRIOR),
        ]:
            source=s.replace(old,'Complete when ('+expression+'). Compensates '+PRIOR+'. End policy.\nStep "Audit done"');grammar.recognize(source)
            refusal((source,l,r),'reference-missing',reference)
        source=s.replace('closure "Close" and races "Race".\nRule for work "Audit work"','closure "Keep" in "Root" in "Main" and races "Race".\nRule for work "Audit work"')
        refusal((source,l,r),'reference-missing','"Keep" in "Root" in "Main"')
        s,l,r=current(small());s=s.replace('Compensates "Create payable".','Compensates "Adjust account".')
        self.assertEqual(invoke((s,l,r),'check').returncode,0)
        result=invoke((s,l,r),'check-concurrency');self.assertEqual(result.returncode,1)
        self.assertIn(b'owned-continuation-compensation-source',result.stderr)

    def test_pair_core_future_and_new_inherited_token_quota_are_explicit(self):
        for change in ['pair','core','future','resource']:
            s,l,r=current(small())
            if change=='pair':l['version']='0.14.0'
            elif change=='core':l['contracts']['core']['digest']='sha256:'+'0'*64
            elif change=='future':s=s.replace('authoring "0.16.0"','authoring "0.18.0"')
            else:l['resources'][0]['digest']='sha256:'+'0'*64
            x=invoke((s,l,r));self.assertEqual(x.returncode,1,(change,x.stderr));self.assertEqual(x.stdout,b'')
        source='Choreoform authoring "0.16.0". '+'x '*(131072-4)
        self.assertEqual(len(grammar.lex(source)),131072)
        with self.assertRaisesRegex(ValueError,'oracle token limit'):grammar.lex(source+'x')

    def test_whole_unchanged_order_records_later_real_binding_failure_without_candidate(self):
        before=fixed();p=current(order());grammar.validate_ledger(p[1]);self.assertEqual(len(grammar.lex(p[0])),77135)
        integrity=checked(p,'integrity');self.assertEqual(integrity,grammar.package_digests(p[0].encode(),encoded(p[1])))
        self.assertEqual(integrity['package'],'sha256:8bd1388d2d1a5a13a3aa3dd0012c6a3989455f794076d136d7a6e56aef5d9800')
        x=invoke(p);self.assertEqual(x.returncode,1);self.assertEqual(x.stdout,b'');self.assertEqual(x.stderr,b'symbol-mismatch at 0..0\n')
        with self.assertRaisesRegex(ValueError,'oracle state limit'):grammar.recognize(p[0])
        self.assertEqual(fixed(),before)


if __name__=='__main__':unittest.main()
