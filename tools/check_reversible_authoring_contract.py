# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Read-only contract/native-projection oracle; NOT a native managed-edit engine."""
from copy import deepcopy
from hashlib import sha256
import json
from pathlib import Path
import tempfile
import unittest

import jsonschema
from referencing import Registry, Resource
import rfc8785

import authoring_probe as grammar
import check_ir_fixtures as wire
from check_authoring_evidence import check_links
from check_pure_predicates import pair
from check_rp01_capture_checkpoint import package as expense
from check_rp03_package import package as order
from check_source_capacity import fixed
from check_static_contracts import ROOT, run

BASE = ROOT / 'docs/authoring/reversible-0.1'
MIB = 1024 * 1024
SCHEMAS = {kind: json.loads((ROOT / f'schemas/authoring/{kind}-0.1.schema.json').read_bytes())
           for kind in ('document', 'layout', 'change')}
REGISTRY = Registry().with_resources((s['$id'], Resource.from_contents(s)) for s in SCHEMAS.values())
VALIDATORS = {kind: jsonschema.Draft202012Validator(s, registry=REGISTRY) for kind, s in SCHEMAS.items()}


class Refusal(ValueError):
    """Only a fixed code, never an untrusted payload or schema/native error."""


def refuse(code):
    raise Refusal(code)


def pin(raw):
    return 'sha256:' + sha256(raw).hexdigest()


def strict(raw, maximum, code):
    """Bound bytes/depth before JSON allocation; private scalar/duplicate checks."""
    if len(raw) > maximum or raw.startswith(b'\xef\xbb\xbf'):
        refuse(code)
    # JSON string contents do not contribute to outer nesting. The full JSON
    # parser still validates escaping, container pairing and grammar afterwards.
    quoted = escaped = False
    depth = 0
    for byte in raw:
        if quoted:
            if escaped:
                escaped = False
            elif byte == 92:
                escaped = True
            elif byte == 34:
                quoted = False
        elif byte == 34:
            quoted = True
        elif byte in (91, 123):
            depth += 1
            if depth > 64:
                refuse(code)
        elif byte in (93, 125):
            depth -= 1
    def integer(token):
        if token == '-0':
            refuse(code)
        value = int(token)
        if abs(value) > 9007199254740991:
            refuse(code)
        return value
    def pairs(rows):
        result = {}
        for key, value in rows:
            if key in result:
                refuse(code)
            result[key] = value
        return result
    try:
        value = json.loads(raw.decode('utf-8'), object_pairs_hook=pairs, parse_int=integer,
                           parse_float=lambda _: refuse(code), parse_constant=lambda _: refuse(code))
        pending = [value]
        count = 0
        while pending:
            item = pending.pop()
            count += 1
            if count > 262144:
                refuse(code)
            if isinstance(item, str) and any(0xD800 <= ord(c) <= 0xDFFF for c in item):
                refuse(code)
            if isinstance(item, dict):
                pending.extend(item.keys())
                pending.extend(item.values())
            elif isinstance(item, list):
                pending.extend(item)
        if len(rfc8785.dumps(value)) > maximum:
            refuse(code)
        return value
    except (UnicodeError, ValueError, OverflowError, RecursionError):
        refuse(code)


def shape(value, kind):
    if not VALIDATORS[kind].is_valid(value):
        refuse(kind + '-format')


def authoring(document):
    return pin(b'choreoform-authoring-document:0.1.0\0' + rfc8785.dumps(document))


def native(document, resources, stage):
    """Preserve BOTH stored strings exactly at the existing native CLI boundary."""
    with tempfile.TemporaryDirectory() as directory:
        base = Path(directory)
        source, companion = base / 'source.choreo', base / 'bindings.json'
        source.write_bytes(document['source'].encode())
        companion.write_bytes(document['bindings'].encode())
        supplied = []
        for index, (identity, revision, raw) in enumerate(resources):
            path = base / f'resource{index}'
            path.write_bytes(raw)
            supplied.append((identity, revision, path))
        result = run(stage, source, companion, [] if stage == 'integrity' else supplied)
        if result.returncode:
            # Native refusal body may contain a code/location, never needed here.
            refuse('component-native')
        return json.loads(result.stdout)


def layout(value, ledger):
    shape(value, 'layout')
    if len(rfc8785.dumps(value)) > 256 * 1024:
        refuse('layout-limit')
    live = {row['id'] for group in ('declarations', 'symbols', 'generated') for row in ledger[group]}
    ids = [row['entity'] for row in value['entries']]
    if ids != sorted(set(ids)) or not set(ids) <= live:
        refuse('layout-reference')


def admit(raw, resources=()):
    """Finite read-only consistency oracle; success is only projection-matched."""
    document = strict(raw, 16 * MIB, 'document-format')
    shape(document, 'document')
    source, bindings = document['source'].encode(), document['bindings'].encode()
    if len(source) > MIB or len(bindings) > MIB:
        refuse('component-limit')
    ledger = strict(bindings, MIB, 'bindings-format')
    if ledger.get('profile') != '0.19.0' or ledger.get('version') != '0.18.0':
        refuse('component-profile')
    if len(resources) > 128 or sum(len(row[2]) for row in resources) > 8 * MIB:
        refuse('registry-limit')
    pins = [(identity, digest) for identity, digest, _ in resources]
    if len(pins) != len(set(pins)) or any(pin(raw) != digest for _, digest, raw in resources):
        refuse('resource-integrity')
    actual = native(document, resources, 'integrity')
    if document['integrity'] != actual:
        refuse('integrity-mismatch')
    layout(document['layout'], ledger)
    ir = native(document, resources, 'lower')
    projected = dict(format=ir['format'], version=ir['version'], definition=ir['body']['id'], revision=wire.revision(ir))
    if document['projection'] != projected or ir['revision'] != projected['revision']:
        refuse('projection-mismatch')
    return document, ledger, ir


def preflight(request, state):
    """Request structure/base/target checks ONLY; does not perform any operation."""
    request = strict(request, MIB, 'change-format')
    shape(request, 'change')
    document, ledger, _ = state
    if request['base'] != authoring(document):
        refuse('stale-base')
    operation = request['operation']
    if operation['kind'] == 'replace-layout':
        layout(operation['layout'], ledger)
    elif operation['kind'] == 'rename':
        identity = operation['target']
        if identity in ledger['retired']:
            refuse('retired-target')
        changed = deepcopy(ledger)
        rows = [row for group in ('declarations', 'symbols') for row in changed[group]
                if row['id'] == identity and (row.get('kind') in ('predicate', 'type-alias', 'data-protection')
                                            or row.get('role') == 'predicate-input')]
        if len(rows) != 1:
            refuse('unsupported-target')
        rows[0]['name'] = operation['name']
        try:
            grammar.validate_ledger(changed)
        except (ValueError, jsonschema.ValidationError):
            refuse('name-conflict')
    return request


def encoded(value):
    return json.dumps(value, ensure_ascii=False).encode()


def from_package(package):
    source, ledger, resources = package
    skeleton = json.loads((BASE / 'base.json').read_bytes())
    skeleton.update(source=source, bindings=json.dumps(ledger, ensure_ascii=False))
    skeleton['layout']['entries'] = []
    skeleton['integrity'] = native(skeleton, resources, 'integrity')
    ir = native(skeleton, resources, 'lower')
    skeleton['projection'] = dict(format=ir['format'], version=ir['version'], definition=ir['body']['id'], revision=ir['revision'])
    return skeleton, resources


class ReversibleAuthoringContract(unittest.TestCase):
    def setUp(self):
        self.vectors = json.loads((BASE / 'vectors.json').read_bytes())
        self.base = admit((BASE / 'base.json').read_bytes())

    def test_signed_publication_pins_schemas_and_all_worked_vectors(self):
        adr = (ROOT / 'docs/decisions/0094-reversible-authoring-documents.md').read_text()
        for path in ['docs/authoring/reversible-v0.1.md', 'docs/authoring/reversible-0.1/vectors.json'] + [f'schemas/authoring/{kind}-0.1.schema.json' for kind in SCHEMAS]:
            self.assertIn(pin((ROOT / path).read_bytes()), adr)
        actual = {str(path.relative_to(ROOT)) for path in BASE.glob('*.json') if path.name != 'vectors.json'}
        actual.update(f'schemas/authoring/{kind}-0.1.schema.json' for kind in SCHEMAS)
        self.assertEqual(set(self.vectors['rawPins']), actual)
        for name, expected in self.vectors['rawPins'].items():
            self.assertEqual(pin((ROOT / name).read_bytes()), expected, name)
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', (ROOT / (name + '.license')).read_text())
        for schema in SCHEMAS.values():
            jsonschema.Draft202012Validator.check_schema(schema)
        for path in ['docs/authoring/reversible-v0.1.md', 'docs/authoring/reversible-0.1/README.md', 'docs/decisions/0094-reversible-authoring-documents.md', 'docs/evaluation/0094-reversible-authoring-design.md', 'docs/authoring/README.md', 'docs/decisions/README.md']:
            check_links(ROOT / path)
        self.assertIn('tools/check_reversible_authoring_contract.py', (ROOT / '.github/workflows/rust-portability.yml').read_text())

    def test_authored_after_states_use_real_native_projection_without_a_rename_engine(self):
        original = self.base[2]
        for name, digest in self.vectors['authoring'].items():
            state = admit((BASE / (name + '.json')).read_bytes())
            self.assertEqual(authoring(state[0]), digest, name)
            self.assertEqual(state[2], original, name)
            self.assertEqual(state[2]['revision'], self.vectors['semanticRevision'])
        for row in self.vectors['pairs']:
            request = preflight((BASE / row['request']).read_bytes(), self.base)
            after = admit((BASE / row['after']).read_bytes())
            self.assertNotEqual(authoring(after[0]), authoring(self.base[0]))
            self.assertEqual(after[2], original)
            if request['operation']['kind'] == 'format':
                formatted = native(self.base[0], (), 'format')
                self.assertEqual((after[0]['source'], after[0]['bindings']), (formatted['source'], formatted['companion']))
            elif request['operation']['kind'] == 'replace-layout':
                self.assertEqual(after[0]['layout'], request['operation']['layout'])
                for key in ('source', 'bindings', 'integrity', 'projection'):
                    self.assertEqual(after[0][key], self.base[0][key])
            else:
                target = request['operation']['target']
                before_ledger = deepcopy(self.base[1])
                next(x for group in ('declarations', 'symbols') for x in before_ledger[group] if x['id'] == target)['name'] = request['operation']['name']
                self.assertEqual(after[1], before_ledger)
                self.assertEqual(after[0]['bindings'].encode(), rfc8785.dumps(before_ledger))
                self.assertIn('mean ("Same") as text', after[0]['source'])
                self.assertEqual(after[1]['annotations'], self.base[1]['annotations'])
        # A raw companion whitespace edit changes full authoring integrity only.
        changed = deepcopy(self.base[0]); changed['bindings'] += '\n'
        state = admit(encoded(changed))
        self.assertNotEqual(authoring(state[0]), authoring(self.base[0]))
        self.assertEqual(state[0]['integrity'], self.base[0]['integrity'])
        self.assertEqual(state[2], original)

    def test_worked_refusals_and_same_semantics_stale_base_are_private(self):
        for name, code in self.vectors['refusals'].items():
            with self.subTest(name=name), self.assertRaisesRegex(Refusal, '^' + code + '$'):
                if name.endswith('.change.json'):
                    preflight((BASE / name).read_bytes(), self.base)
                else:
                    admit((BASE / name).read_bytes())
        stale = self.vectors['stalePair']
        changed = admit((BASE / stale['base']).read_bytes())
        self.assertEqual(changed[2], self.base[2])
        with self.assertRaisesRegex(Refusal, '^stale-base$'):
            preflight((BASE / stale['request']).read_bytes(), changed)
        for raw in [b'{"SECRET":0,"SECRET":1}', b'{"a":-0}', b'{"a":1.0}', b'{"a":NaN}', b'"\\ud800"', b'\xef\xbb\xbf{}', b'{} trailing']:
            with self.assertRaisesRegex(Refusal, '^document-format$'):
                admit(raw)
        for change in ('future', 'unknown', 'invalid-source', 'collision', 'retired', 'owner'):
            document = deepcopy(self.base[0]); ledger = json.loads(document['bindings'])
            if change == 'future': ledger['profile'] = '0.20.0'
            elif change == 'unknown': document['layout']['secret'] = 'SECRET'
            elif change == 'invalid-source': document['source'] += '\nSECRET invalid source'
            elif change == 'collision': ledger['declarations'][1]['id'] = ledger['root']
            elif change == 'retired': ledger['retired'].append('same')
            else: next(x for x in ledger['symbols'] if x['id'] == 'same_value')['owner'] = 'node_done'
            document['bindings'] = json.dumps(ledger)
            with self.assertRaises(Refusal) as error:
                admit(encoded(document))
            self.assertNotIn('SECRET', str(error.exception))

    def test_real_full_rp01_rp03_fit_and_exact_resource_bytes_are_required(self):
        before = fixed(); sizes = []
        for package in (expense(), order()):
            document, resources = from_package(package)
            raw = encoded(document)
            state = admit(raw, resources)
            sizes.append((len(raw), len(resources), sum(len(row[2]) for row in resources)))
            self.assertEqual(state[2]['revision'], document['projection']['revision'])
            self.assertLessEqual(len(raw), 16 * MIB)
            self.assertTrue(resources)
            wrong = list(resources); identity, digest, blob = wrong[0]; wrong[0] = identity, digest, blob + b'\n'
            with self.assertRaisesRegex(Refusal, '^resource-integrity$'):
                admit(raw, wrong)
            with self.assertRaisesRegex(Refusal, '^component-native$'):
                admit(raw, resources[1:])
        print('Observed document bytes / resource pins / registry bytes:', sizes)
        self.assertEqual(fixed(), before)
        tree = ''.join(name + '\0' + digest + '\n' for name, digest in sorted(before.items()))
        self.assertEqual((len(before), sha256(tree.encode()).hexdigest()), (965, 'a25c6de850fdaa51436023fffe011c8f96695e9d8660191e7598cd4f4bf2e9f7'))

    def test_qualified_ancestor_use_and_private_descendant_refusal_use_real_native_binding(self):
        source, ledger, resources = pair()
        child = 'Scope "Child" within "Terminal example". Start at "Child done". Inputs empty. Outputs empty. Outcomes ["complete"]. Use cancellation "Stop", faults "Fault handling", closure "Close" and races "Commit order". Step "Child done". Finish as "complete". End step. End scope.'
        ledger['declarations'] += [dict(id='child', kind='scope', scope=ledger['root'], name='Child'), dict(id='child_done', kind='node', scope='child', name='Child done')]
        ledger['symbols'].append(dict(id='child_complete', owner='child', role='outcome', name='complete', wire='complete'))
        source = source.replace('End process.', child + ' End process.')
        expression = 'Let "Read condition" mean (test "Same" using ["Value" is (the value of "Value")]) as truth value with parameters empty.'
        self.assertEqual(source.count(expression), 1)
        qualified = expression.replace('test "Same" using', 'test "Same" in "Terminal example" using')
        nested = source.replace(expression, '').replace('Step "Child done"', qualified + ' Step "Child done"')
        owned = deepcopy(ledger)
        next(x for x in owned['declarations'] if x['id'] == 'read_condition')['scope'] = 'child'
        document, registry = from_package((nested, owned, resources))
        self.assertEqual(admit(encoded(document), registry)[2]['revision'], document['projection']['revision'])
        # Hiding a declaration below its caller cannot be bypassed by qualification.
        from check_pure_predicates import DEFINITION
        self.assertEqual(source.count(DEFINITION), 1)
        hidden = source.replace(DEFINITION, '').replace('Step "Child done"', DEFINITION + ' Step "Child done"')
        hidden = hidden.replace('test "Same" using', 'test "Same" in "Child" in "Terminal example" using')
        next(x for x in ledger['declarations'] if x['id'] == 'same')['scope'] = 'child'
        # No projection can be emitted from this private-reference source.
        with self.assertRaisesRegex(Refusal, '^component-native$'):
            from_package((hidden, ledger, resources))

    def test_transport_component_layout_depth_items_registry_and_output_bounds(self):
        with self.assertRaisesRegex(Refusal, '^limit$'):
            strict(b' ' * (16 * MIB + 1), 16 * MIB, 'limit')
        with self.assertRaisesRegex(Refusal, '^change-format$'):
            preflight(b' ' * (MIB + 1), self.base)
        # Exact byte/depth/item boundaries independent of component semantics.
        self.assertEqual(strict(b'"x"' + b' ' * (MIB - 3), MIB, 'limit'), 'x')
        self.assertEqual(strict(b'[' * 64 + b'0' + b']' * 64, MIB, 'limit'), json.loads(b'[' * 64 + b'0' + b']' * 64))
        for raw in (b'[' * 65 + b'0' + b']' * 65, b'[' + b'0,' * 262144 + b'0]'):
            with self.assertRaisesRegex(Refusal, '^limit$'): strict(raw, 16 * MIB, 'limit')
        self.assertEqual(len(strict(b'[' + b'0,' * 262142 + b'0]', MIB, 'limit')), 262143)
        doc = deepcopy(self.base[0]); doc['source'] = 'é' * (MIB // 2 + 1)
        with self.assertRaisesRegex(Refusal, '^component-limit$'): admit(encoded(doc))
        for coordinate in (-1000001, 1000001):
            doc = deepcopy(self.base[0]); doc['layout']['entries'] = [dict(entity='same', x=coordinate, y=0, collapsed=False)]
            with self.assertRaisesRegex(Refusal, '^document-format$'): admit(encoded(doc))
        doc = deepcopy(self.base[0]); doc['layout']['entries'] = [dict(entity='same_value', x=0, y=0, collapsed=False), dict(entity='same', x=0, y=0, collapsed=False)]
        with self.assertRaisesRegex(Refusal, '^layout-reference$'): admit(encoded(doc))
        for count, blob in [(129, b'x'), (1, b'x' * (8 * MIB + 1))]:
            with self.assertRaisesRegex(Refusal, '^registry-limit$'):
                admit(encoded(self.base[0]), [('urn:test:' + str(i), pin(blob), blob) for i in range(count)])
        oversized_layout = dict(format='choreoform-authoring-layout', version='0.1.0', entries=[dict(entity='x' + str(i).zfill(63), x=1000000, y=-1000000, collapsed=False) for i in range(10000)])
        shape(oversized_layout, 'layout')
        with self.assertRaisesRegex(Refusal, '^layout-limit$'):
            layout(oversized_layout, self.base[1])
        oversized_layout['entries'].append(oversized_layout['entries'][-1])
        with self.assertRaisesRegex(Refusal, '^layout-format$'):
            layout(oversized_layout, self.base[1])
        for coordinate in (-1000000, 1000000):
            doc = deepcopy(self.base[0]); doc['layout']['entries'] = [dict(entity='same', x=coordinate, y=0, collapsed=False)]
            self.assertEqual(admit(encoded(doc))[2], self.base[2])
        # Source-only same-name no-op is structurally admissible; no edit executes.
        req = json.loads((BASE / 'rename.change.json').read_bytes()); req['operation']['name'] = 'Same'
        self.assertEqual(preflight(encoded(req), self.base), req)


if __name__ == '__main__':
    unittest.main()
