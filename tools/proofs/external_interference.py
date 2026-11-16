# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Inactive, pure assumed model. Never import this from a production reader.

Synthetic snapshots/tables are mathematical inputs, not accepted process events,
provider promises, nominal-invariant validation or a host authorization API.
"""
from dataclasses import dataclass
from hashlib import sha256
import json
from pathlib import Path
import re

import jsonschema
import rfc8785
from authoring_probe import strict_json

ROOT = Path(__file__).resolve().parents[2]
VERSION = '0.1.0-draft.1'
PROFILE_PATH = ROOT / 'docs/dialects/drafts/external-interference.md'
PROFILE = {'id': 'urn:choreoform:drafts:external-interference:0.1.0-draft.1',
           'revision': 'sha256:4e37d47a5c61f520dba3db704cbe19b7ad0e92b00a56c46edcf91a1e30cb0dfc'}
CORE = {'id': 'urn:choreoform:contracts:core-profile:0.12.0',
        'revision': 'sha256:73158731c33084fde009abcc86530926447902609fda9535f04daa7c768c2007'}
SCHEMAS = {kind: jsonschema.Draft202012Validator(json.loads(
    (ROOT / 'schemas/drafts' / filename).read_bytes())) for kind, filename in [
        ('descriptor', 'external-interference.schema.json'),
        ('namespace', 'external-region-namespace.schema.json')]}
P3_FIELDS = {'kind', 'version', 'id', 'input', 'output', 'outcomes',
             'resultProtection', 'observations', 'idempotency', 'effectClass'}
ASSUMPTIONS = ['validated-provider-and-input-invariants',
               'truthful-exhaustive-provider-coverage',
               'supported-common-canonical-authority',
               'synthetic-original-request-correlation']


class Unproved(ValueError):
    """Private exception deliberately carries no selected values."""


def require(condition):
    if not condition:
        raise Unproved()


def closed(value, keys):
    require(type(value) is dict and set(value) == set(keys))


def ref(value):
    closed(value, ('id', 'revision'))
    require(type(value['id']) is str and 0 < len(value['id'].encode()) <= 256)
    require(type(value['revision']) is str and
            re.fullmatch(r'sha256:[0-9a-f]{64}', value['revision']) is not None)
    return value['id'], value['revision']


def pin(identity, raw):
    return {'id': identity, 'revision': 'sha256:' + sha256(raw).hexdigest()}


def encode(value):
    return (json.dumps(value, ensure_ascii=False, indent=2) + '\n').encode()


@dataclass(frozen=True)
class ModelWork:
    provider: bytes
    descriptor: bytes
    snapshot: bytes
    channel: str = 'activity'


class Budget:
    def __init__(self, work=100000):
        require(type(work) is int and 0 <= work <= 100000)
        self.work = work
        self.bytes = 0

    def spend(self, amount=1):
        self.work -= amount
        require(self.work >= 0)

    def load(self, raw):
        require(type(raw) is bytes)
        self.bytes += len(raw)
        require(self.bytes <= 1024 * 1024)
        document = strict_json(raw)
        stack = [(document, 0)]
        while stack:
            self.spend()
            item, depth = stack.pop()
            require(depth <= 64)
            if isinstance(item, str):
                require(len(item.encode()) <= 256)
            elif isinstance(item, dict):
                stack.extend((x, depth + 1) for pair in item.items() for x in pair)
            elif isinstance(item, list):
                stack.extend((x, depth + 1) for x in item)
        return document


def integer(value):
    require(type(value) is str and re.fullmatch(r'0|-?[1-9][0-9]*', value) is not None)
    require(-2**63 <= int(value) < 2**63)


def typed_key(value):
    require(type(value) is list and 1 <= len(value) <= 4)
    result = []
    for component in value:
        closed(component, ('type', 'value'))
        kind, item = component['type'], component['value']
        require(kind in ('text', 'integer') and type(item) is str)
        if kind == 'integer':
            integer(item)
        result.append((kind, item))
    return tuple(result)


class Model:
    def __init__(self, namespace_bytes, named_bytes, table_bytes, budget):
        self.budget = budget
        require(len(namespace_bytes) <= 32 and len(named_bytes) <= 32)
        self.namespaces = self.resources(namespace_bytes, 'namespace')
        self.named = self.resources(named_bytes, 'named')
        self.selected = {}
        table = budget.load(table_bytes)
        closed(table, ('authority', 'entries'))
        self.authority = ref(table['authority'])
        require(type(table['entries']) is list and len(table['entries']) <= 64)
        self.canonical = {}
        for entry in table['entries']:
            closed(entry, ('namespace', 'key', 'canonical'))
            namespace = ref(entry['namespace'])
            require(namespace in self.namespaces)
            key = typed_key(entry['key'])
            self.namespace_key(namespace, key)
            canonical = entry['canonical']
            closed(canonical, ('domain', 'key'))
            require(type(canonical['domain']) is str and canonical['domain'])
            target = (canonical['domain'], typed_key(canonical['key']))
            source = (namespace, key)
            require(source not in self.canonical)  # No ambiguous/duplicate bindings.
            self.canonical[source] = target

    def resources(self, resources, kind):
        result = {}
        for raw in resources:
            doc = self.budget.load(raw)
            require(type(doc) is dict)
            identity = pin(doc['id'], raw)
            key = ref(identity)
            require(key not in result)
            if kind == 'namespace':
                SCHEMAS[kind].validate(doc)
                require(doc['profile'] == PROFILE)
            else:
                closed(doc, ('kind', 'version', 'id', 'definitions'))
                require(doc['kind'] == 'named-types' and doc['version'] == '0.1.0')
                require(type(doc['definitions']) is dict)
            result[key] = doc
        return result

    def namespace_key(self, namespace, key):
        doc = self.namespaces[namespace]
        require(ref(doc['authority']) == self.authority)
        require(tuple(doc['keyTypes']) == tuple(k for k, _ in key))

    def representation(self, ty):
        closed(ty, ('kind', 'contract', 'name'))
        resource = self.named[ref(ty['contract'])]
        definition = resource['definitions'][ty['name']]
        closed(definition, ('representation', 'invariant'))
        # Full V2 invariant evaluation is explicitly assumed, never bypassed in
        # production by using this model. Representation is not nominal admission.
        return definition['representation']

    def value(self, ty, value, depth=0):
        self.budget.spend()
        require(depth <= 64 and type(ty) is dict)
        kind = ty['kind']
        if kind == 'named':
            self.value(self.representation(ty), value, depth + 1)
        elif kind in ('text', 'integer', 'boolean', 'decimal'):
            closed(ty, ('kind', 'scale') if kind == 'decimal' else ('kind',))
            if kind in ('integer', 'decimal'):
                integer(value)
                if kind == 'decimal':
                    require(type(ty['scale']) is int and 0 <= ty['scale'] <= 18)
            else:
                require(type(value) is (str if kind == 'text' else bool))
        elif kind == 'record':
            closed(ty, ('kind', 'fields'))
            require(type(ty['fields']) is dict)
            closed(value, ty['fields'])
            for name, field in ty['fields'].items():
                self.value(field, value[name], depth + 1)
        elif kind == 'option':
            closed(ty, ('kind', 'element'))
            require(type(value) is dict)
            if value.get('tag') == 'none':
                closed(value, ('tag',))
            else:
                closed(value, ('tag', 'value'))
                require(value['tag'] == 'some')
                self.value(ty['element'], value['value'], depth + 1)
        elif kind == 'variant':
            closed(ty, ('kind', 'cases'))
            closed(value, ('tag', 'value'))
            require(type(ty['cases']) is dict and ty['cases'])
            self.value(ty['cases'][value['tag']], value['value'], depth + 1)
        elif kind == 'list':
            closed(ty, ('kind', 'element'))
            require(type(value) is list)
            for item in value:
                self.value(ty['element'], item, depth + 1)
        else:
            raise Unproved()

    def selector_type(self, selector, ty):
        for step in selector['path']:
            self.budget.spend()
            if step['op'] == 'representation':
                require(ty['kind'] == 'named')
                ty = self.representation(ty)
            elif step['op'] == 'field':
                require(ty['kind'] == 'record')
                ty = ty['fields'][step['name']]
            elif step['op'] == 'present':
                require(ty['kind'] == 'option')
                ty = ty['element']
            else:
                raise Unproved()
        require(ty == {'kind': selector['type']})

    def select(self, selector, ty, value):
        for step in selector['path']:
            self.budget.spend()
            op = step['op']
            if op == 'representation':
                require(ty['kind'] == 'named')
                ty = self.representation(ty)
            elif op == 'field':
                require(ty['kind'] == 'record')
                ty, value = ty['fields'][step['name']], value[step['name']]
            elif op == 'present':
                require(ty['kind'] == 'option' and value['tag'] == 'some')
                ty, value = ty['element'], value['value']
            else:
                raise Unproved()
        require(ty == {'kind': selector['type']})
        self.value(ty, value)
        return selector['type'], value

    def work(self, work):
        require(type(work) is ModelWork and work.channel in ('activity', 'terminal'))
        provider = self.budget.load(work.provider)
        closed(provider, P3_FIELDS)
        require(provider['kind'] == 'provider-contract' and provider['version'] == '0.1.0')
        require(provider['effectClass'] in ('read-only', 'compensatable', 'irreversible'))
        descriptor = self.budget.load(work.descriptor)
        SCHEMAS['descriptor'].validate(descriptor)
        require(descriptor['profile'] == PROFILE and descriptor['providerProfile'] == CORE)
        require(descriptor['provider'] == pin(provider['id'], work.provider))
        provider_pin = ref(descriptor['provider'])
        descriptor_pin = ref(pin(descriptor['id'], work.descriptor))
        if provider_pin in self.selected:
            require(self.selected[provider_pin] == descriptor_pin)
        self.selected[provider_pin] = descriptor_pin
        snapshot = self.budget.load(work.snapshot)
        closed(snapshot, ('definition', 'scopeOccurrence', 'activityOccurrence',
                          'logicalEffect', 'cohort', 'inputRevisions', 'attempt', 'request'))
        ref(snapshot['definition'])
        for key in ('scopeOccurrence', 'activityOccurrence', 'logicalEffect', 'cohort', 'attempt'):
            require(type(snapshot[key]) is str and snapshot[key])
        revisions = snapshot['inputRevisions']
        require(type(revisions) is dict and 0 < len(revisions) <= 32)
        for cell, revision in revisions.items():
            require(type(cell) is str and cell)
            ref(revision)
        self.value(provider['input'], snapshot['request'])
        # Validate both channels before selecting one. A used opaque channel is
        # unproved even if its peer is finite. No read-only relabeling.
        for channel in ('activity', 'terminal'):
            footprint = descriptor[channel]
            if footprint != 'opaque':
                if channel == 'terminal' or provider['effectClass'] == 'read-only':
                    require(not footprint['writes'])
                else:
                    require(footprint['writes'])
                for access in ('reads', 'writes'):
                    for region in footprint[access]:
                        namespace = ref(region['namespace'])
                        self.namespace_key(namespace, tuple((s['type'], '') for s in region['key']))
                        for selector in region['key']:
                            self.selector_type(selector, provider['input'])
        footprint = descriptor[work.channel]
        require(footprint != 'opaque')
        resolved = {}
        for access in ('reads', 'writes'):
            regions = set()
            for region in footprint[access]:
                self.budget.spend()
                namespace = ref(region['namespace'])
                key = tuple(self.select(s, provider['input'], snapshot['request']) for s in region['key'])
                self.namespace_key(namespace, key)
                regions.add(self.canonical[(namespace, key)])
            resolved[access] = regions
        # Correlation excludes attempt; technical retries preserve everything else.
        identity = {k: v for k, v in snapshot.items() if k != 'attempt'}
        identity.update(provider=descriptor['provider'], descriptor=pin(descriptor['id'], work.descriptor))
        identity['requestDigest'] = 'sha256:' + sha256(rfc8785.dumps(
            {'type': provider['input'], 'request': snapshot['request']})).hexdigest()
        return resolved, identity


def report(status):
    result = {'status': status, 'executable': False, 'assumptions': ASSUMPTIONS[:],
              'remaining': ['generic-static-proof', 'provider-authority-host-evidence',
                            'all-other-semantic-and-phase-gates']}
    raw = encode(result)
    require(len(raw) <= 1024)
    return raw


def compare(left, right, namespaces, table, named=(), *, work_limit=100000):
    """Fixed private result over assumed inputs; no writes or execution admission."""
    try:
        model = Model(namespaces, named, table, Budget(work_limit))
        a, ai = model.work(left)
        b, bi = model.work(right)
        occurrence = ('definition', 'scopeOccurrence', 'activityOccurrence')
        if all(ai[k] == bi[k] for k in occurrence):
            require(ai == bi)  # Same logical occurrence cannot switch its request.
        model.budget.spend(len(a['writes']) + len(b['reads']) + len(b['writes']) + len(a['reads']))
        overlap = a['writes'] & (b['reads'] | b['writes']) or b['writes'] & a['reads']
        return report('overlap-in-model' if overlap else 'disjoint-in-model')
    except (ValueError, KeyError, TypeError, RecursionError, OverflowError,
            jsonschema.ValidationError, UnicodeError):
        return report('unproved')


def same_logical_request(original, retry, namespaces, table, named=()):
    """Pure identity oracle, not an accepted retry event or dispatch permission."""
    try:
        model = Model(namespaces, named, table, Budget())
        _, old = model.work(original)
        _, new = model.work(retry)
        return old == new
    except (ValueError, KeyError, TypeError, RecursionError, OverflowError,
            jsonschema.ValidationError, UnicodeError):
        return False
