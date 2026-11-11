# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Explicit lexical capacity; no wider proof, private binding or benchmark adoption."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest
import authoring_probe as grammar
from check_static_contracts import ROOT
from check_finite_work_authoring import package as small
from check_rp03_package import historical_package as order
from check_metadata_authoring import invoke
from check_readonly_continuation_authoring import STAGES


def promote(p):
    s,l,r=deepcopy(p);s=s.replace('authoring "0.14.0"','authoring "0.15.0"',1)
    l.update(profile='0.15.0',version='0.14.0');return s,l,r


def fixed():
    paths=[]
    for folder in ['rp01','rp03']:paths += [p for p in (ROOT/'docs/benchmarks'/folder).rglob('*') if p.is_file()]
    return {str(p.relative_to(ROOT)):sha256(p.read_bytes()).hexdigest() for p in paths}


class SourceCapacityEvidence(unittest.TestCase):
    def test_exact_grammar_and_closed_schema_only_change_profile_metadata(self):
        old=(ROOT/'docs/authoring/candidate-a-v0.14.ebnf').read_text();new=(ROOT/'docs/authoring/candidate-a-v0.15.ebnf').read_text()
        self.assertEqual([l for l in old.splitlines() if not l.startswith('#')],[l for l in new.splitlines() if not l.startswith('#')])
        old=json.loads((ROOT/'schemas/authoring/bindings-0.13.schema.json').read_bytes());new=json.loads((ROOT/'schemas/authoring/bindings-0.14.schema.json').read_bytes())
        new.update({'$id':old['$id'],'title':old['title']});new['properties']['version']['const']='0.13.0';new['properties']['profile']['const']='0.14.0'
        self.assertEqual(new,old)

    def test_independent_lexer_has_exact_new_old_and_fragment_limits(self):
        for profile,limit in [('0.4.0',4096),('0.14.0',16384),('0.15.0',131072)]:
            s=f'Choreoform authoring "{profile}". '+'x '*(limit-4)
            self.assertEqual(len(grammar.lex(s)),limit)
            with self.assertRaisesRegex(ValueError,'oracle token limit'):grammar.lex(s+'x')
        self.assertEqual(len(grammar.lex('x '*4096,program=False)),4096)
        with self.assertRaisesRegex(ValueError,'oracle token limit'):grammar.lex('x '*4097,program=False)
        with self.assertRaisesRegex(ValueError,'delimiter depth limit'):grammar.lex('('*65)

    def test_all_eleven_small_native_views_and_semantic_identity_are_preserved(self):
        a=small();b=promote(a);grammar.recognize(b[0]);grammar.validate_ledger(b[1])
        x=invoke(a);y=invoke(b);self.assertEqual(x.returncode,0,x.stderr);self.assertEqual(y.returncode,0,y.stderr)
        self.assertEqual(json.loads(x.stdout),json.loads(y.stdout))
        for stage in STAGES:
            x=invoke(a,stage);y=invoke(b,stage);self.assertEqual((x.returncode,y.returncode),(0,0),(stage,x.stderr,y.stderr))
            self.assertEqual(json.loads(x.stdout),json.loads(y.stdout));self.assertIs(json.loads(y.stdout)['executable'],False)
        self.assertNotEqual(grammar.package_digests(a[0].encode(),json.dumps(a[1]).encode())['package'],grammar.package_digests(b[0].encode(),json.dumps(b[1]).encode())['package'])

    def test_actual_whole_order_moves_from_tokens_to_a_preserved_private_reference_refusal(self):
        before=fixed();a=order();b=promote(a)
        x=invoke(a);y=invoke(b)
        self.assertEqual(x.returncode,1);self.assertEqual(x.stdout,b'');self.assertEqual(x.stderr.strip(),b'tokens at 108626..108627')
        self.assertEqual(y.returncode,1);self.assertEqual(y.stdout,b'');self.assertEqual(y.stderr.strip(),b'reference-missing at 60937..61023')
        self.assertEqual(b[0].encode()[60937:61023],b'"Request Reserve first line" in "Reserve first line occurrence" in "Order fulfillment"')
        self.assertEqual(len(grammar.lex(b[0])),77135)
        # Different algorithms have different work metrics; keep both bounds.
        with self.assertRaisesRegex(ValueError,'oracle state limit'):grammar.recognize(b[0])
        self.assertEqual(fixed(),before)

    def test_wrong_pair_core_future_version_and_declared_resource_do_not_become_valid(self):
        for change in ['pair','core','future','resource']:
            s,l,r=promote(small())
            if change=='pair':l['version']='0.13.0'
            elif change=='core':l['contracts']['core']['digest']='sha256:'+'0'*64
            elif change=='future':s=s.replace('authoring "0.15.0"','authoring "0.17.0"')
            else:l['resources'][0]['digest']='sha256:'+'0'*64
            result=invoke((s,l,r));self.assertEqual(result.returncode,1,(change,result.stderr));self.assertEqual(result.stdout,b'')

    def test_profile_files_licensing_and_required_ci_are_explicit(self):
        path=ROOT/'schemas/authoring/bindings-0.14.schema.json'
        self.assertEqual(path.with_name(path.name+'.license').read_text(),'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n')
        self.assertIn('SPDX-License-Identifier: MPL-2.0',(ROOT/'docs/authoring/candidate-a-v0.15.ebnf').read_text())
        self.assertIn('tools/check_source_capacity.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__=='__main__':unittest.main()
