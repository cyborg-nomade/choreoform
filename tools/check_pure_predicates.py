# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite grammar/schema/native-CLI evidence, not independent review or user evidence."""
from copy import deepcopy
from hashlib import sha256
import json
import unittest
import jsonschema
import authoring_probe as grammar
from check_authoring_evidence import check_links
from check_metadata_authoring import invoke
from check_named_data_protection import pair as inherited, checked
from check_readonly_continuation_authoring import STAGES
from check_rp01_capture_checkpoint import package as expense
from check_rp03_package import package as order
from check_source_capacity import fixed
from check_static_contracts import ROOT
from benchmarks.rp03_predicate_checkpoint import prior_benchmark_inventory

BASE = ROOT / 'docs/evaluation/0090-predicate-variants'
DEFINITION = 'Define test "Same" with inputs ["Value" as whole number]. It holds when ((the parameter "Value") equals (the parameter "Value")). End test.'


def pair(recipes=True):
    s, l, r = deepcopy(inherited())
    s = s.replace('authoring "0.17.0"', 'authoring "0.19.0"')
    l.update(profile='0.19.0', version='0.18.0')
    expression = 'test "Same" using ["Value" is (the value of "Value")]' if recipes else '(the value of "Value") equals (the value of "Value")'
    s = s.replace('End process.', f'Let "Read condition" mean ({expression}) as truth value with parameters empty. End process.')
    l['declarations'].append(dict(id='read_condition', kind='expression', scope=l['root'], name='Read condition'))
    if recipes:
        s = s.replace('End process.', DEFINITION + ' End process.')
        l['declarations'].append(dict(id='same', kind='predicate', scope=l['root'], name='Same'))
        l['symbols'].append(dict(id='same_value', owner='same', role='predicate-input', name='Value', wire='v'))
    return s, l, r


def variant(name, resources):
    base = BASE / name
    return (base.joinpath('source.choreo').read_text(), json.loads(base.joinpath('bindings.json').read_bytes()), resources)


class PurePredicates(unittest.TestCase):
    def test_frozen_publications_closed_schema_delta_and_independent_small_grammar(self):
        adr = (ROOT/'docs/decisions/0090-pure-predicate-recipes.md').read_text()
        for p in ['docs/authoring/candidate-a-v0.19.ebnf', 'docs/authoring/profile-v0.19.md', 'schemas/authoring/bindings-0.18.schema.json']:
            self.assertIn(sha256((ROOT/p).read_bytes()).hexdigest(), adr)
        old = json.loads((ROOT/'schemas/authoring/bindings-0.17.schema.json').read_bytes())
        new = json.loads((ROOT/'schemas/authoring/bindings-0.18.schema.json').read_bytes())
        for key in ['$id', 'title']: new[key] = old[key]
        for key in ['profile', 'version']: new['properties'][key] = old['properties'][key]
        new['properties']['declarations']['items']['properties']['kind']['enum'].remove('predicate')
        new['properties']['symbols']['items']['properties']['role']['enum'].remove('predicate-input')
        self.assertEqual(old, new)
        s, l, _ = pair()
        grammar.recognize(s); grammar.validate_ledger(l)
        with self.assertRaises(ValueError): grammar.recognize(s.replace('"0.19.0"', '"0.18.0"'))
        with self.assertRaises(jsonschema.ValidationError): grammar.validate_ledger(dict(l, version='0.17.0'))
        for path in ['docs/authoring/README.md', 'docs/decisions/0090-pure-predicate-recipes.md', 'docs/evaluation/0090-pure-predicates.md', 'docs/evaluation/0090-predicate-variants/README.md']:
            check_links(ROOT/path)
        self.assertIn('tools/check_pure_predicates.py', (ROOT/'.github/workflows/rust-portability.yml').read_text())

    def test_actual_reads_all_small_views_and_format_integrity_match_explicit_expansion(self):
        before = fixed(); p = pair(); expanded = pair(False)
        doc = checked(p)
        self.assertEqual(doc, checked(expanded))
        self.assertEqual(doc['body']['expressions']['read_condition']['reads'], {'value': True})
        for stage in STAGES:
            self.assertEqual(checked(p, stage), checked(expanded, stage), stage)
        f = checked(p, 'format'); q = f['source'], json.loads(f['companion']), p[2]
        self.assertEqual(checked(q), doc)
        self.assertNotEqual(checked(q, 'integrity'), checked(p, 'integrity'))
        self.assertEqual(fixed(), before)

    def test_wrong_type_purity_stale_future_and_owner_refuse_without_source_echo(self):
        for change, code in [('type', b'source-type-mismatch'), ('purity', b'predicate-purity'), ('stale', b'binding-mismatch'), ('future', b'profile'), ('owner', b'symbol-owner')]:
            s, l, r = pair()
            if change == 'type': s = s.replace('is (the value of "Value")', 'is ("SECRET-MUST-NOT-LEAK")')
            elif change == 'purity': s = s.replace('(the parameter "Value") equals (the parameter "Value")', 'the fact "SECRET-MUST-NOT-LEAK"')
            elif change == 'stale': s = s.replace('Define test "Same"', 'Define test "SECRET-MUST-NOT-LEAK"')
            elif change == 'future': s = s.replace('"0.19.0"', '"0.20.0"')
            else: next(x for x in l['symbols'] if x['id']=='same_value')['owner'] = 'read_condition'
            result = invoke((s, l, r))
            self.assertNotEqual(result.returncode, 0, change)
            self.assertEqual(result.stdout, b'')
            self.assertIn(code, result.stderr)
            self.assertNotIn(b'SECRET-MUST-NOT-LEAK', result.stderr)

    def test_both_complete_variants_keep_current_wire_all_views_and_refusal(self):
        before = fixed()
        pins = json.loads((ROOT/'docs/evaluation/0090-predicate-variant-digests.json').read_bytes())
        measured = json.loads((ROOT/'docs/evaluation/0090-predicate-native-metrics.json').read_bytes())
        self.assertEqual([(r['sourceBytes'], r['tokens'], r['parserWork']) for r in measured], [(54172,9145,113796),(291309,41306,386333)])
        prior = prior_benchmark_inventory()
        inventory = ''.join(k+'\0'+v+'\n' for k,v in sorted(prior.items()))
        self.assertEqual(pins['unchangedBenchmarks'], dict(files=len(prior), inventorySha256=sha256(inventory.encode()).hexdigest()))
        for name, current, size, tokens in [('rp01', expense(), 54172, 9145), ('rp03', order(), 291309, 41306)]:
            p = variant(name, current[2])
            self.assertEqual(len(p[0].encode()), size)
            raw = (BASE/name/'bindings.json').read_bytes()
            self.assertEqual(pins[name]['sourceSha256'], sha256(p[0].encode()).hexdigest())
            self.assertEqual((pins[name]['bindingsBytes'],pins[name]['bindingsRawSha256']), (len(raw),sha256(raw).hexdigest()))
            self.assertEqual(pins[name]['integrity'], checked(p, 'integrity'))
            self.assertEqual(len(grammar.lex(p[0])), tokens)
            grammar.validate_ledger(p[1])
            self.assertEqual(checked(p), checked(current))
            for stage in STAGES:
                a, b = invoke(current, stage), invoke(p, stage)
                self.assertEqual(a.returncode, b.returncode, (name, stage, b.stderr))
                if a.returncode:
                    self.assertEqual(a.stderr.split(b'; source bytes ')[0], b.stderr.split(b'; source bytes ')[0])
                    self.assertTrue(b.stderr.startswith(b'parallel-effects-unproved at /body/nodes/start_acceptance; source bytes '), b.stderr)
                    self.assertEqual(b.stdout, b'')
                else:
                    self.assertEqual(json.loads(a.stdout), json.loads(b.stdout), (name, stage))
                    self.assertIs(json.loads(b.stdout)['executable'], False)
            f = checked(p, 'format')
            self.assertEqual(checked((f['source'], json.loads(f['companion']), p[2])), checked(current))
        self.assertEqual(fixed(), before)

    def test_source19_alias_summary_cache_does_not_admit_exponential_expansion(self):
        s, l, r = pair(False)
        for i in range(25):
            ty = 'text' if i == 0 else f'record with ["a" as type "Type {i-1}", "b" as type "Type {i-1}"]'
            s = s.replace('End process.', f'Describe type "Type {i}" as {ty}. End process.')
            l['declarations'].append(dict(id=f't{i}', kind='type-alias', scope=l['root'], name=f'Type {i}'))
        self.assertLess(len(s.encode()), 10000)
        result = invoke((s, l, r))
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(result.stdout, b'')
        self.assertTrue(any(c in result.stderr for c in [b'type-expansion', b'work-limit', b'size', b'depth']), result.stderr)


if __name__ == '__main__': unittest.main()
