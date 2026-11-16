# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Exact synthetic RP-01 resources; hashes and types confer no live authority."""
from hashlib import sha256
import json

TYPES_ID = 'urn:choreoform:benchmarks:rp01:types:0.1.0'
TEXT = {'kind': 'text'}
INTEGER = {'kind': 'integer'}
BOOLEAN = {'kind': 'boolean'}
AMOUNT = {'kind': 'decimal', 'scale': 2}


def record(**fields):
    return {'kind': 'record', 'fields': fields}


def option(element):
    return {'kind': 'option', 'element': element}


def variant(cases):
    return {'kind': 'variant', 'cases': cases}


def literal(ty, value):
    return {'op': 'literal', 'type': ty, 'value': value}


def field(value, name):
    return {'op': 'field', 'value': value, 'name': name}


def binary(op, left, right):
    return {'op': op, 'left': left, 'right': right}


def negate(value):
    return {'op': 'not', 'value': value}


def every(items):
    """Balanced conjunction keeps generated invariant depth explicit and small."""
    items = list(items)
    if not items:
        return literal(BOOLEAN, True)
    if len(items) == 1:
        return items[0]
    middle = len(items) // 2
    return binary('and', every(items[:middle]), every(items[middle:]))


def nonempty(value):
    return negate(binary('eq', value, literal(TEXT, '')))


def encoded(document):
    return (json.dumps(document, indent=2, ensure_ascii=False) + '\n').encode('utf-8')


def pin(identity, raw):
    return {'id': identity, 'revision': 'sha256:' + sha256(raw).hexdigest()}


def types():
    artifact = record(identity=TEXT, revision=TEXT, digest=TEXT, provenance=TEXT)
    receipt = variant({'provided': artifact, 'missing': record(reason=TEXT)})
    claim = record(**{
        'claim ID': TEXT, 'revision': TEXT, 'policy revision': TEXT, 'employee': TEXT,
        'date': TEXT, 'amount': AMOUNT, 'currency': TEXT, 'category': TEXT,
        'business purpose': TEXT, 'cost allocation': TEXT, 'receipt': receipt,
    })
    check = record(**{
        'claim ID': TEXT, 'claim revision': TEXT, 'policy revision': TEXT,
        'reasons': {'kind': 'list', 'element': TEXT}, 'duplicate claim': option(TEXT),
    })
    review = record(**{
        'claim ID': TEXT, 'claim revision': TEXT, 'policy revision': TEXT,
        'principal': TEXT, 'assignment': TEXT, 'reason': TEXT,
    })
    paid = record(**{
        'claim ID': TEXT, 'claim revision': TEXT, 'command reference': TEXT,
        'payable reference': TEXT, 'amount': AMOUNT, 'currency': TEXT,
    })
    problem = record(**{
        'claim ID': TEXT, 'claim revision': TEXT, 'command reference': TEXT, 'reason': TEXT,
    })
    notice = record(**{'claim ID': TEXT, 'claim revision': TEXT, 'recipient': TEXT, 'message reference': TEXT})
    value = {'op': 'param', 'name': 'value'}
    definitions = {}
    for name, representation, required in [
        ('Claim', claim, ['claim ID', 'revision', 'policy revision', 'employee', 'date', 'category', 'business purpose', 'cost allocation']),
        ('Check evidence', check, ['claim ID', 'claim revision', 'policy revision']),
        ('Review evidence', review, ['claim ID', 'claim revision', 'policy revision', 'principal', 'assignment', 'reason']),
        ('Paid evidence', paid, ['claim ID', 'claim revision', 'command reference', 'payable reference']),
        ('Problem evidence', problem, ['claim ID', 'claim revision', 'command reference', 'reason']),
        ('Notice evidence', notice, ['claim ID', 'claim revision', 'recipient', 'message reference']),
    ]:
        predicates = [nonempty(field(value, key)) for key in required]
        if name in ['Claim', 'Paid evidence']:
            predicates += [
                negate(binary('lt', field(value, 'amount'), literal(AMOUNT, '0'))),
                {'op': 'contains', 'list': literal({'kind': 'list', 'element': TEXT}, ['EUR', 'USD']), 'value': field(value, 'currency')},
            ]
        if name == 'Claim':
            receipt_value = field(value, 'receipt')
            supplied = {'op': 'payload', 'value': receipt_value, 'tag': 'provided'}
            missing = {'op': 'payload', 'value': receipt_value, 'tag': 'missing'}
            predicates.append({
                'op': 'if',
                'condition': binary('eq', {'op': 'tag', 'value': receipt_value}, literal(TEXT, 'provided')),
                'then': every(nonempty(field(supplied, key)) for key in artifact['fields']),
                'else': nonempty(field(missing, 'reason')),
            })
        definitions[name] = {'representation': representation, 'invariant': every(predicates)}
    document = {'kind': 'named-types', 'version': '0.1.0', 'id': TYPES_ID, 'definitions': definitions}
    raw = encoded(document)
    return document, raw, pin(TYPES_ID, raw)


def nominal(type_pin, name):
    return {'kind': 'named', 'contract': type_pin, 'name': name}


def protection(capabilities=()):
    return {
        'policy': 'expense_access', 'sensitivity': 'expense',
        'purposes': {'reimbursement': True},
        'participants': {f'{role}_actor': True for role in ['employee', 'manager', 'finance', 'operator', 'clock']},
        'capabilities': {identity: True for identity in capabilities},
    }


def providers(type_pin, shared=False):
    claim = nominal(type_pin, 'Claim')
    check_output = variant({tag: nominal(type_pin, 'Check evidence') for tag in ['clear', 'correction', 'duplicate', 'rejected']})
    payment_input = record(claim=claim, manager=nominal(type_pin, 'Review evidence'), finance=nominal(type_pin, 'Review evidence'))
    payment_output = variant({'paid': nominal(type_pin, 'Paid evidence'), 'problem': nominal(type_pin, 'Problem evidence')})
    notice_input = record(**{'claim ID': TEXT, 'claim revision': TEXT, 'recipient': TEXT, 'disposition': TEXT, 'reasons': {'kind': 'list', 'element': TEXT}, 'payable reference': option(TEXT)})
    notice_output = variant({'notified': nominal(type_pin, 'Notice evidence')})
    specifications = [
        ('checks', claim, check_output, 'read-only', 'none'),
        ('payment', payment_input, payment_output, 'irreversible', 'stable-key'),
        ('notice', notice_input, notice_output, 'irreversible', 'stable-key'),
    ]
    result = {}
    for name, input_type, output_type, effect, idempotency in specifications:
        identity = f'urn:choreoform:benchmarks:rp01:{name}:0.1.0'
        observations = {'result-' + tag: {'state': 'succeeded', 'result': output_type, 'outcome': tag} for tag in output_type['cases']}
        observations.update({state: {'state': state, 'result': None, 'outcome': None} for state in ['acknowledged', 'rejected', 'cancelled', 'unknown', 'no-effect']})
        document = {
            'kind': 'provider-contract', 'version': '0.1.0', 'id': identity,
            'input': input_type, 'output': output_type,
            'outcomes': {tag: True for tag in output_type['cases']},
            'resultProtection': protection(['checks_capability', 'payment_capability', 'notice_capability'] if shared else ()), 'observations': observations,
            'idempotency': idempotency, 'effectClass': effect,
        }
        raw = encoded(document)
        result[name] = (document, raw, pin(identity, raw))
    return result


def clock_resources(shared=False):
    source_id = 'urn:choreoform:benchmarks:rp01:clock-source:0.1.0'
    source = {
        'kind': 'provider-contract', 'version': '0.1.0', 'id': source_id,
        'input': record(), 'output': INTEGER, 'outcomes': {'tick': True},
        'resultProtection': protection(['checks_capability', 'payment_capability', 'notice_capability'] if shared else ()),
        'observations': {
            'tick': {'state': 'succeeded', 'result': INTEGER, 'outcome': 'tick'},
            'unknown': {'state': 'unknown', 'result': None, 'outcome': None},
        },
        'idempotency': 'none', 'effectClass': 'read-only',
    }
    source_raw = encoded(source)
    source_pin = pin(source_id, source_raw)
    clock_id = 'urn:choreoform:benchmarks:rp01:clock:0.1.0'
    clock = {
        'kind': 'clock', 'version': '0.1.0', 'id': clock_id,
        'profile': {
            'id': 'urn:choreoform:clock-profile:instance-microseconds:0.1.0',
            'revision': 'sha256:8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9',
        },
        'source': source_pin,
    }
    clock_raw = encoded(clock)
    return {'clock-source': (source, source_raw, source_pin), 'clock': (clock, clock_raw, pin(clock_id, clock_raw))}
