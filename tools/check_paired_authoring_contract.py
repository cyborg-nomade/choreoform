# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Read-only publication/coverage oracle over real native views; no edit engine."""
from copy import deepcopy
from hashlib import sha256
import importlib.util
import json
import tempfile
import unittest

import jsonschema
from referencing import Registry, Resource
from check_authoring_evidence import check_links
from check_metadata_authoring import invoke
from check_readonly_continuation_authoring import STAGES
from check_source_capacity import fixed
from check_static_contracts import ROOT

spec = importlib.util.spec_from_file_location('paired_contract_server', ROOT/'tools/authoring-prototype/server.py')
server = importlib.util.module_from_spec(spec); spec.loader.exec_module(server)
BASE = ROOT/'docs/authoring/paired-0.1'
SCHEMAS = {kind: json.loads((ROOT/f'schemas/authoring/{kind}-0.1.schema.json').read_bytes())
           for kind in ['view', 'resource-registry', 'prototype-catalog', 'prototype-error']}
IR_SCHEMA = json.loads((ROOT/'schemas/ir/definition-0.4.schema.json').read_bytes())
REGISTRY = Registry().with_resources([(IR_SCHEMA['$id'], Resource.from_contents(IR_SCHEMA))])
VALIDATORS = {kind: jsonschema.Draft202012Validator(schema, registry=REGISTRY) for kind, schema in SCHEMAS.items()}
MAPS = ['scopes', 'data', 'expressions', 'actors', 'capabilities', 'policies', 'nodes', 'flows']


def view_refusal(value, expected):
    """Check output, not import it. Expected is the real native snapshot, never inferred."""
    if not VALIDATORS['view'].is_valid(value): return 'view-format'
    if value['binding'] != expected['binding']: return 'view-binding'
    if {e['id'] for e in value['entities']} != {e['id'] for e in expected['entities']}: return 'view-coverage'
    locations = {e['id']: e['location'] for e in expected['entities']}
    if any(e['location'] != locations[e['id']] for e in value['entities']): return 'view-location'
    return None


class PairedAuthoring(unittest.TestCase):
    def test_frozen_publication_schemas_and_positive_negative_vectors(self):
        vectors = json.loads((BASE/'vectors.json').read_bytes())
        adr = (ROOT/'docs/decisions/0096-experimental-paired-prototype.md').read_text()
        for name in ['docs/authoring/paired-prototype-v0.1.md', 'docs/authoring/paired-0.1/vectors.json', *[f'schemas/authoring/{k}-0.1.schema.json' for k in SCHEMAS]]:
            self.assertIn('sha256:' + sha256((ROOT/name).read_bytes()).hexdigest(), adr)
        for name, digest in vectors['rawPins'].items(): self.assertEqual('sha256:' + sha256((ROOT/name).read_bytes()).hexdigest(), digest, name)
        for schema in SCHEMAS.values(): jsonschema.Draft202012Validator.check_schema(schema)
        for kind, name in [('view','view.json'), ('resource-registry','registry.json'), ('prototype-catalog','catalog-exporter.json'), ('prototype-error','error-access.json')]:
            VALIDATORS[kind].validate(json.loads((BASE/name).read_bytes()))
        expected = json.loads((BASE/'view.json').read_bytes())
        for name, code in vectors['refusals'].items():
            if name.startswith('view-'): self.assertEqual(view_refusal(json.loads((BASE/name).read_bytes()), expected), code)
        for name in ['docs/decisions/0096-experimental-paired-prototype.md', 'docs/authoring/paired-prototype-v0.1.md', 'docs/evaluation/0096-paired-authoring-prototype-plan.md', 'docs/evaluation/0096-paired-authoring-prototype-design.md', 'tools/authoring-prototype/README.md']:
            check_links(ROOT/name)

    def check_view(self, state):
        view = json.loads(state.view); doc = json.loads(state.document); ledger = json.loads(doc['bindings'])
        VALIDATORS['view'].validate(view)
        VALIDATORS['resource-registry'].validate(json.loads(state.registry))
        rows = {row['id']: row for group in ['declarations','symbols','generated'] for row in ledger[group]}
        self.assertEqual([e['id'] for e in view['entities']], sorted(rows))
        self.assertEqual(view['source'], doc['source'])
        semantic = {(kind, identity) for kind in MAPS for identity in view['ir']['body'][kind]}
        self.assertEqual({(e['semanticMap'], e['id']) for e in view['entities'] if e['semanticMap']}, semantic)
        self.assertFalse(view['executable']); self.assertEqual(view['status'], 'projection-matched')
        for e in view['entities']:
            row = rows[e['id']]
            self.assertEqual(e['owner'], row.get('scope', row.get('owner')))
            group, index = e['location']['companion'].strip('/').split('/')
            self.assertEqual(ledger[group][int(index)], row)
            if e['role'] == 'resource':
                self.assertEqual(e['location']['basis'], 'companion'); self.assertIsNone(e['location']['start'])
                pin = next(r for r in ledger['resources'] if r['binding'] == e['id'])
                self.assertEqual(e['pin'], {k:pin[k] for k in ['kind','contract','digest']})
            else:
                start, end = e['location']['start'], e['location']['end']
                self.assertTrue(0 <= start < end <= len(doc['source'].encode()))
                doc['source'].encode()[start:end].decode('utf-8')
            if 'name' in row: self.assertEqual(e['caption'], row['name'])
        slots = {r['id']: r for r in ledger['generated']}; symbols = {r['id']:r for r in ledger['symbols']}
        for flow in view['flows']:
            native = view['ir']['body']['flows'][flow['id']]
            self.assertEqual({k:flow[k] for k in ['source','target','outcome']}, native)
            self.assertEqual(flow['caption'], symbols[slots[flow['id']]['key']]['name'])
        self.assertEqual({f['id'] for f in view['flows']}, set(view['ir']['body']['flows']))
        return view, doc, ledger

    def test_whole_native_views_edits_layout_exports_all_stages_and_unchanged_benchmarks(self):
        before = fixed(); kinds = set(); roles = set(); measurements = []
        with tempfile.TemporaryDirectory() as directory:
            catalog = server.load_catalog(directory)
            for key, state in catalog.items():
                view, doc, ledger = self.check_view(state)
                original = deepcopy(view['ir']); raw_original = state.document
                kinds.update(e['nodeKind'] for e in view['entities'] if e['nodeKind']); roles.update(e['role'] for e in view['entities'])
                if key == 'example': self.assertEqual(view, json.loads((BASE/'view.json').read_bytes())); continue
                self.assertEqual(original, json.loads((ROOT/f'docs/benchmarks/{key}/candidate.ir.json').read_bytes()))
                measurements.append((key,len(view['entities']),len(view['flows']),len(state.view)))
                target = next(e for e in view['entities'] if e['role'] == 'predicate')
                for operation in [dict(kind='rename',target=target['id'],name='Paired renamed β'), dict(kind='replace-layout',layout=dict(format='choreoform-authoring-layout',version='0.1.0',entries=[dict(entity=target['id'],x=-10,y=20,collapsed=True)])), dict(kind='format')]:
                    request = server.encoded(dict(format='choreoform-authoring-change',version='0.1.0',base=state.digest,operation=operation))
                    state.apply(request); next_view, final, _ = self.check_view(state)
                    self.assertEqual(next_view['ir'], original)
                    self.assertEqual(state.bridge('canonical', state.document), state.document)
                placed = next(e for e in next_view['entities'] if e['id'] == target['id'])
                self.assertEqual(placed['caption'], 'Paired renamed β'); self.assertEqual(placed['placement'], dict(x=-10,y=20,collapsed=True,explicit=True))
                registry = json.loads(state.registry)
                supplied = [(r['contract'],r['digest'],state.blob[r['start']:r['start']+r['length']]) for r in registry['resources']]
                original_doc = json.loads(raw_original)
                for stage in STAGES:
                    a = invoke((original_doc['source'],json.loads(original_doc['bindings']),supplied),stage)
                    b = invoke((final['source'],json.loads(final['bindings']),supplied),stage)
                    self.assertEqual(a.returncode,b.returncode,(key,stage,b.stderr))
                    if a.returncode:
                        self.assertEqual(a.stderr.split(b'; source bytes ')[0],b.stderr.split(b'; source bytes ')[0]); self.assertEqual(b.stdout,b'')
                        self.assertTrue(b.stderr.startswith(b'parallel-effects-unproved at /body/nodes/start_acceptance; source bytes '))
                    else: self.assertEqual(json.loads(a.stdout),json.loads(b.stdout),(key,stage))
        self.assertEqual(kinds, set(['activity','compute','invoke','decision','split','join','wait','repeat','finish']))
        self.assertTrue({'scope','resource','type-alias','data-protection','predicate','predicate-input','provider-policy','initial','assignment','flow','purpose'} <= roles)
        # Fanout is exercised as an actual current-profile document in package.rs.
        self.assertEqual(measurements,[('rp01',340,53,296968),('rp03',1442,192,1760748)])
        self.assertEqual(fixed(),before)
        inventory = ''.join(name+'\0'+digest+'\n' for name,digest in sorted(before.items()))
        self.assertEqual((len(before),sha256(inventory.encode()).hexdigest()),(965,'a25c6de850fdaa51436023fffe011c8f96695e9d8660191e7598cd4f4bf2e9f7'))
        print('Observed complete native views:', measurements)


if __name__ == '__main__': unittest.main()
