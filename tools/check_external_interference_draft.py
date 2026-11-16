# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Inactive assumed-model evidence. No provider registration, input acceptance or run."""
from copy import deepcopy
from dataclasses import replace
from hashlib import sha256
import json
from pathlib import Path
import unittest

import jsonschema
from check_authoring_evidence import check_links
from check_metadata_authoring import invoke
from check_source_capacity import fixed
from proofs.external_interference import (
    CORE, PROFILE, PROFILE_PATH, SCHEMAS, ModelWork, compare, encode, pin,
    same_logical_request,
)
from proofs.external_interference_examples import (
    AUTHORITY, bundle, descriptor, namespace, snapshot, table,
)
from proofs.parallel_effects import package

ROOT = Path(__file__).resolve().parents[1]
EXAMPLES = ROOT / 'docs/dialects/drafts/external-interference-examples'


def edit(work, field, change):
    doc = json.loads(getattr(work, field))
    change(doc)
    return replace(work, **{field: encode(doc)})


def rebind_provider(work, doc):
    raw = encode(doc)
    return replace(edit(work, 'descriptor', lambda d: d.update(provider=pin(doc['id'], raw))), provider=raw)


class ExternalInterferenceDraft(unittest.TestCase):
    def status(self, a, b, ns, mapping, expected, named=(), **kw):
        result = compare(a, b, ns, mapping, named, **kw)
        self.assertLessEqual(len(result), 1024)
        d = json.loads(result)
        self.assertEqual(d['status'], expected)
        self.assertIs(d['executable'], False)
        self.assertIn('validated-provider-and-input-invariants', d['assumptions'])
        self.assertIn('generic-static-proof', d['remaining'])
        return result

    def test_pins_closed_schemas_examples_and_profile_identity(self):
        self.assertEqual(pin(PROFILE['id'], PROFILE_PATH.read_bytes()), PROFILE)
        for filename, schema in [('descriptor-a.json', 'descriptor'), ('descriptor-b.json', 'descriptor'), ('namespace.json', 'namespace')]:
            SCHEMAS[schema].validate(json.loads((EXAMPLES / filename).read_bytes()))
        a, b, ns, mapping = bundle()
        expected = {'provider-a.json': a.provider, 'provider-b.json': b.provider,
                    'descriptor-a.json': a.descriptor, 'descriptor-b.json': b.descriptor,
                    'snapshot-a.json': a.snapshot, 'snapshot-b.json': b.snapshot,
                    'namespace.json': ns[0], 'canonical-model.json': mapping}
        self.assertEqual({p.name for p in EXAMPLES.glob('*.json')}, set(expected))
        for name, raw in expected.items():
            self.assertEqual((EXAMPLES / name).read_bytes(), raw)
            self.assertIn('SPDX-License-Identifier: MPL-2.0', (EXAMPLES / (name + '.license')).read_text())
        for schema in SCHEMAS.values():
            jsonschema.Draft202012Validator.check_schema(schema.schema)
        self.status(a, b, ns, mapping, 'disjoint-in-model')

    def test_read_write_conflicts_and_read_read_overlap(self):
        a, b, ns, mapping = bundle()
        b = edit(b, 'snapshot', lambda d: d.update(request='1'))
        self.status(a, b, ns, mapping, 'overlap-in-model')
        a, b, ns, mapping = bundle(readonly=True)
        b = edit(b, 'snapshot', lambda d: d.update(request='1'))
        self.status(a, b, ns, mapping, 'disjoint-in-model')
        a, b, ns, mapping = bundle()
        self.status(a, replace(b, channel='terminal'), ns, mapping, 'disjoint-in-model')
        b = edit(b, 'snapshot', lambda d: d.update(request='1'))
        self.status(a, replace(b, channel='terminal'), ns, mapping, 'overlap-in-model')
        self.status(replace(a, channel='terminal'), replace(b, channel='terminal'), ns, mapping, 'disjoint-in-model')

    def test_aliases_across_namespaces_and_keys_need_common_authority(self):
        a, b, ns, mapping = bundle()
        alias_raw, alias_pin = namespace('urn:test:different-name')
        b = edit(b, 'descriptor', lambda d: [r.update(namespace=alias_pin) for c in ['activity', 'terminal'] for v in d[c].values() for r in v])
        entries = json.loads(mapping)
        entries['entries'].append({'namespace': alias_pin, 'key': [{'type': 'integer', 'value': '2'}],
                                   'canonical': entries['entries'][0]['canonical']})
        self.status(a, b, ns + [alias_raw], encode(entries), 'overlap-in-model')
        entries['entries'].pop()
        self.status(a, b, ns + [alias_raw], encode(entries), 'unproved')
        entries = json.loads(mapping)
        entries['entries'][1]['canonical'] = entries['entries'][0]['canonical']
        self.status(*bundle()[:3], encode(entries), 'overlap-in-model')
        entries['authority'] = pin('urn:test:unregistered', b'other authority')
        self.status(*bundle()[:3], encode(entries), 'unproved')
        entries = json.loads(mapping)
        entries['entries'].append(deepcopy(entries['entries'][0]))
        self.status(*bundle()[:3], encode(entries), 'unproved')

    def test_record_option_integer_selection_and_missing_optional_refusal(self):
        a, b, ns, mapping = bundle()
        selector = {'path': [{'op': 'field', 'name': 'item'}, {'op': 'present'}], 'type': 'integer'}
        works = []
        for work, key in [(a, '1'), (b, '2')]:
            doc = json.loads(work.provider)
            doc['input'] = {'kind': 'record', 'fields': {'item': {'kind': 'option', 'element': {'kind': 'integer'}}}}
            work = rebind_provider(work, doc)
            np = pin(json.loads(ns[0])['id'], ns[0])
            work = replace(work, descriptor=descriptor(work.provider, np, selector=selector))
            work = edit(work, 'snapshot', lambda d: d.update(request={'item': {'tag': 'some', 'value': key}}))
            works.append(work)
        self.status(*works, ns, mapping, 'disjoint-in-model')
        for request in [{}, {'item': {'tag': 'none'}}, {'item': None}, {'item': {'tag': 'some', 'value': '01'}}, {'item': {'tag': 'some', 'value': '9223372036854775808'}}]:
            changed = edit(works[1], 'snapshot', lambda d: d.update(request=request))
            self.status(works[0], changed, ns, mapping, 'unproved')

    def test_real_pinned_named_representation_is_assumed_not_invariant_validation(self):
        from benchmarks.rp03_resources import types, providers, inputs
        _, named_raw, named_pin = types()
        provider = providers(named_pin)['reserve_a'][1]
        nsraw, nspin = namespace(kinds=('text',))
        selector = {'path': [{'op': 'field', 'name': 'order'}, {'op': 'representation'},
                             {'op': 'field', 'name': 'first line'}, {'op': 'field', 'name': 'warehouse'}], 'type': 'text'}
        request = {'order': inputs()['order'], 'scope key': 'line-a', 'scope revision': 'v1',
                   'request revision': 'v1', 'amount': '12000', 'currency': 'EUR',
                   'basis reference': 'synthetic', 'address revision': 'v1',
                   'target quantity': {'tag': 'some', 'value': '1'},
                   'target warehouse': {'tag': 'some', 'value': 'fictional-warehouse-a'}}
        a = ModelWork(provider, descriptor(provider, nspin, selector=selector), snapshot('a', request))
        brequest = deepcopy(request);brequest['order']['first line']['warehouse'] = 'another-synthetic-warehouse'
        b = replace(a, snapshot=snapshot('b', brequest))
        mapping = table(nspin, ('fictional-warehouse-a', 'another-synthetic-warehouse'), kind='text')
        self.status(a, b, [nsraw], mapping, 'disjoint-in-model', [named_raw])
        self.status(a, b, [nsraw], mapping, 'unproved')
        self.status(a, b, [nsraw], mapping, 'unproved', [named_raw + b' '])
        # Deliberately violate a nominal invariant while keeping representation.
        # The prototype reports its assumption; this is NOT nominal acceptance.
        bad = edit(a, 'snapshot', lambda d: d['request']['order'].update(amount='-1'))
        result = self.status(bad, b, [nsraw], mapping, 'disjoint-in-model', [named_raw])
        self.assertIn(b'validated-provider-and-input-invariants', result)
        bad_selector = deepcopy(selector);bad_selector['path'].pop(1)
        wrong = replace(a, descriptor=descriptor(provider, nspin, selector=bad_selector))
        self.status(wrong, b, [nsraw], mapping, 'unproved', [named_raw])

    def test_original_request_terminal_and_retry_identity_across_revisions(self):
        a, b, ns, mapping = bundle()
        retry = edit(a, 'snapshot', lambda d: d.update(attempt='attempt-2'))
        self.assertTrue(same_logical_request(a, retry, ns, mapping))
        self.assertTrue(same_logical_request(a, replace(retry, channel='terminal'), ns, mapping))
        for change in [lambda d: d.update(request='2'), lambda d: d.update(cohort='v2'),
                       lambda d: d.update(request='2', logicalEffect='renamed-effect'),
                       lambda d: d.update(inputRevisions={'request': pin('urn:test:revision', b'v2')})]:
            changed = edit(retry, 'snapshot', change)
            self.assertFalse(same_logical_request(a, changed, ns, mapping))
            self.status(a, changed, ns, mapping, 'unproved')
        # New logical work in another cohort is compared to the OLD original key.
        revised = edit(b, 'snapshot', lambda d: d.update(request='1', cohort='v2'))
        self.status(a, revised, ns, mapping, 'overlap-in-model')
        replaced = edit(revised, 'snapshot', lambda d: d.update(currentRequest='2'))
        self.status(a, replaced, ns, mapping, 'unproved')
        self.status(a, replace(revised, channel='terminal'), ns, mapping, 'overlap-in-model')
        compensation = edit(b, 'snapshot', lambda d: d.update(logicalEffect='new-compensation', request='1'))
        self.assertFalse(same_logical_request(a, compensation, ns, mapping))
        self.status(a, compensation, ns, mapping, 'overlap-in-model')

    def test_closed_unknown_versions_profiles_pins_and_opaque_class_rules(self):
        a, b, ns, mapping = bundle()
        changes = [lambda d: d.update(independent=True), lambda d: d.update(version='0.1.0'),
                   lambda d: d.update(profile=CORE), lambda d: d.update(providerProfile=PROFILE),
                   lambda d: d.update(coverage='partial'), lambda d: d.update(activity='opaque'),
                   lambda d: d.update(activity={'reads': [], 'writes': []}),
                   lambda d: d['terminal']['writes'].append(d['terminal']['reads'][0]),
                   lambda d: d['activity']['reads'][0]['key'][0].update(path=[{'op': 'current'}]),
                   lambda d: d['terminal']['reads'][0]['key'][0].update(type='text')]
        for change in changes:
            self.status(a, edit(b, 'descriptor', change), ns, mapping, 'unproved')
        self.status(a, replace(b, provider=b.provider + b' '), ns, mapping, 'unproved')
        raw = b.descriptor.replace(b'"version":', b'"id":"duplicate","version":')
        self.status(a, replace(b, descriptor=raw), ns, mapping, 'unproved')
        self.status(a, b, [ns[0] + b' '], mapping, 'unproved')
        doc = json.loads(b.provider);doc['effectClass'] = 'read-only'
        self.status(a, rebind_provider(b, doc), ns, mapping, 'unproved')
        for kind in ['boolean', 'decimal', 'list', 'named']:
            changed = edit(b, 'descriptor', lambda d: d['activity']['reads'][0]['key'][0].update(type=kind))
            self.status(a, changed, ns, mapping, 'unproved')

    def test_shared_work_bytes_depth_cardinality_and_private_fixed_output(self):
        a, b, ns, mapping = bundle()
        self.status(a, b, ns, mapping, 'unproved', work_limit=0)
        self.status(a, b, ns, mapping, 'unproved', work_limit=100001)
        self.status(a, b, ns * 33, mapping, 'unproved')
        self.status(a, replace(b, snapshot=b' ' * 1048577), ns, mapping, 'unproved')
        self.status(a, replace(b, snapshot=b'[' * 70 + b'0' + b']' * 70), ns, mapping, 'unproved')
        d = json.loads(mapping);d['entries'] *= 33
        self.status(a, b, ns, encode(d), 'unproved')
        secret = 'SECRET-MUST-NOT-APPEAR'
        leaked = edit(b, 'snapshot', lambda d: d.update(request=secret))
        result = self.status(a, leaked, ns, mapping, 'unproved')
        self.assertNotIn(secret.encode(), result)
        for token in [b'synthetic-physical-domain', b'urn:test:', sha256(a.snapshot).hexdigest().encode()]:
            self.assertNotIn(token, compare(a, b, ns, mapping))
        self.assertEqual(result, compare(a, b, ns, mapping, work_limit=0))

    def test_composite_typed_keys_and_unicode_aliases_are_not_implicit(self):
        a, b, _, _ = bundle()
        nsraw, nspin = namespace(kinds=('text', 'integer'))
        works = []
        keys = [{'tenant': 'Café', 'slot': '1'}, {'tenant': 'Cafe\u0301', 'slot': '1'}]
        selectors = [{'path': [{'op': 'field', 'name': k}], 'type': ty}
                     for k, ty in [('tenant', 'text'), ('slot', 'integer')]]
        for work, request in zip([a, b], keys):
            provider = json.loads(work.provider)
            provider['input'] = {'kind': 'record', 'fields': {'tenant': {'kind': 'text'}, 'slot': {'kind': 'integer'}}}
            work = rebind_provider(work, provider)
            work = edit(work, 'snapshot', lambda d: d.update(request=request))
            work = edit(work, 'descriptor', lambda d: [r.update(namespace=nspin, key=selectors)
                         for c in ['activity', 'terminal'] for v in d[c].values() for r in v])
            works.append(work)
        entries = [{'namespace': nspin, 'key': [{'type': 'text', 'value': k['tenant']}, {'type': 'integer', 'value': k['slot']}],
                    'canonical': {'domain': 'synthetic', 'key': [{'type': 'integer', 'value': str(i)}]}}
                   for i, k in enumerate(keys)]
        mapping = {'authority': AUTHORITY, 'entries': entries}
        self.status(*works, [nsraw], encode(mapping), 'disjoint-in-model')
        entries[1]['canonical'] = entries[0]['canonical']
        self.status(*works, [nsraw], encode(mapping), 'overlap-in-model')
        entries[1]['key'].pop()
        self.status(*works, [nsraw], encode(mapping), 'unproved')

    def test_namespace_type_profile_and_unselected_channel_validation(self):
        a, b, ns, mapping = bundle()
        for field, value in [('version', '0.2.0'), ('profile', CORE), ('unknown', True), ('keyTypes', ['text'])]:
            changed = json.loads(ns[0]);changed[field] = value
            raw = encode(changed);newpin = pin(changed['id'], raw)
            def rebind(d):
                for channel in ['activity', 'terminal']:
                    for regions in d[channel].values():
                        for region in regions:
                            region['namespace'] = newpin
            works = [edit(w, 'descriptor', rebind) for w in [a, b]]
            m = json.loads(mapping)
            for entry in m['entries']:
                entry['namespace'] = newpin
            self.status(*works, [raw], encode(m), 'unproved')
        invalid = edit(b, 'descriptor', lambda d: d['terminal']['reads'][0]['key'][0].update(path=[{'op': 'field', 'name': 'absent'}]))
        self.status(a, invalid, ns, mapping, 'unproved')
        opaque = edit(b, 'descriptor', lambda d: d.update(terminal='opaque'))
        self.status(a, opaque, ns, mapping, 'disjoint-in-model')
        self.status(a, replace(opaque, channel='terminal'), ns, mapping, 'unproved')

    def test_one_shared_work_budget_fails_after_partial_success_without_output(self):
        from proofs.external_interference import Budget, Model
        a, b, ns, mapping = bundle()
        budget = Budget();model = Model(ns, (), mapping, budget)
        model.work(a)
        spent = 100000 - budget.work
        self.assertGreater(spent, 0)
        # Enough for complete resource setup and first work, not the second work.
        self.status(a, b, ns, mapping, 'unproved', work_limit=spent)
        self.status(a, b, ns, mapping, 'disjoint-in-model', work_limit=100000)
        long_selector = edit(b, 'descriptor', lambda d: d['activity']['reads'][0]['key'][0].update(path=[{'op': 'present'}] * 9))
        self.status(a, long_selector, ns, mapping, 'unproved')
        too_many = edit(b, 'descriptor', lambda d: d['activity'].update(reads=d['activity']['reads'] * 17))
        self.status(a, too_many, ns, mapping, 'unproved')

    def test_one_exact_provider_cannot_select_conflicting_descriptor_revisions(self):
        a, b, ns, mapping = bundle()
        # Reusing a provider at a different occurrence is fine with ONE selected
        # descriptor; changing descriptor identity/bytes for it is unproved.
        b = replace(b, provider=a.provider, descriptor=a.descriptor)
        self.status(a, b, ns, mapping, 'disjoint-in-model')
        conflict = edit(b, 'descriptor', lambda d: d.update(id=d['id'] + ':other'))
        self.status(a, conflict, ns, mapping, 'unproved')

    def test_existing_source_refusal_benchmark_bytes_and_document_links(self):
        before = fixed()
        p = package('compensatable', 'compensatable')
        r = invoke(p, 'check-concurrency')
        self.assertEqual(r.returncode, 1)
        self.assertEqual(r.stdout, b'')
        self.assertTrue(r.stderr.startswith(b'parallel-effects-unproved at /body/nodes/fork;'))
        self.assertEqual(fixed(), before)
        for path in [PROFILE_PATH, ROOT/'docs/decisions/0079-external-interference-draft.md',
                     ROOT/'docs/evaluation/0079-external-interference-design-critique.md']:
            check_links(path)
        self.assertIn('tools/check_external_interference_draft.py', (ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__ == '__main__':
    unittest.main()
