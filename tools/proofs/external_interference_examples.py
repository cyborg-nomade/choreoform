# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Synthetic mathematical requests/maps, never process events or provider truth."""
from .external_interference import CORE, PROFILE, VERSION, ModelWork, encode, pin
from .parallel_effects import package

AUTHORITY = pin('urn:test:assumed-canonical-authority', b'Synthetic common authority assumption, not host registration.\n')


def namespace(identity='urn:test:namespace', kinds=('integer',)):
    document = dict(kind='external-region-namespace', version=VERSION, id=identity,
                    profile=PROFILE, authority=AUTHORITY, keyTypes=list(kinds))
    raw = encode(document)
    return raw, pin(identity, raw)


def descriptor(provider_raw, ns, *, selector=None, readonly=False):
    import json
    provider = json.loads(provider_raw)
    region = {'namespace': ns, 'key': [selector or {'path': [], 'type': 'integer'}]}
    document = dict(kind='external-interference', version=VERSION,
                    id=provider['id'] + ':interference-draft', profile=PROFILE,
                    providerProfile=CORE, provider=pin(provider['id'], provider_raw),
                    coverage='exhaustive', activity={'reads': [region], 'writes': [] if readonly else [region]},
                    terminal={'reads': [region], 'writes': []})
    return encode(document)


def snapshot(key, value, *, cohort='cohort-v1'):
    return encode(dict(definition=pin('urn:test:definition', b'fictional definition'),
                       scopeOccurrence='scope-' + key, activityOccurrence='activity-' + key,
                       logicalEffect='effect-' + key, cohort=cohort,
                       inputRevisions={'request': pin('urn:test:request:' + key, encode(value))},
                       attempt='attempt-1', request=value))


def table(ns, keys=('1', '2'), *, kind='integer'):
    return encode({'authority': AUTHORITY, 'entries': [
        {'namespace': ns, 'key': [{'type': kind, 'value': k}],
         'canonical': {'domain': 'synthetic-physical-domain', 'key': [{'type': kind, 'value': k}]}}
        for k in keys]})


def bundle(*, readonly=False):
    mode = 'read-only' if readonly else 'compensatable'
    resources = package(mode, mode)[2]
    a, b = resources[0][2], resources[1][2]
    raw, ns = namespace()
    return (ModelWork(a, descriptor(a, ns, readonly=readonly), snapshot('a', '1')),
            ModelWork(b, descriptor(b, ns, readonly=readonly), snapshot('b', '2')),
            [raw], table(ns))
