# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Controlled full RP-01 fixture adoption, never a general source migration."""
from copy import deepcopy
from .rp01_resources import encoded, pin
from .rp01_source import expression_source, type_source, names, quote

CORE11 = {'contract': 'urn:choreoform:contracts:core-profile:0.11.0',
          'digest': 'sha256:246bf3835eef25f8adf76bce39e6b887715e098761e675f821bf745a8111d29e', 'dialect': 'core'}
PROFILE = {'id': 'urn:choreoform:observation-source-profile:provider-terminal-facts:0.1.0',
           'revision': 'sha256:a224fc2b8318af2ff8dfc9955b55af03f4d5b7bd0de719de0384421698d20ead'}
BINDING = 'payment_terminal_binding'
CAPTURE = 'late_payment_capture'
REQUEST = 'retained_payment_request'
CATEGORY = 'late_payment_category'
CLASSIFY = 'classify_late_payment'


def classification(completion):
    """Reuse the original receipt judgment, with only its two inputs rebound."""
    captured = {'op': 'read', 'cell': CAPTURE}
    payload = {'op': 'payload', 'value': captured, 'tag': 'confirmed'}
    def rebound(value):
        if isinstance(value, dict):
            if value.get('op') == 'literal': return deepcopy(value)
            if value.get('op') == 'fact':
                assert value == {'op': 'fact', 'name': 'result'}
                return deepcopy(payload)
            if value.get('op') == 'read':
                assert value == {'op': 'read', 'cell': 'payment_request'}
                return {'op': 'read', 'cell': REQUEST}
            return {key: rebound(child) for key, child in value.items()}
        if isinstance(value, list): return [rebound(child) for child in value]
        return value
    text = lambda value: {'op': 'literal', 'type': {'kind': 'text'}, 'value': value}
    return {'op': 'if',
            'condition': {'op': 'eq', 'left': {'op': 'tag', 'value': captured}, 'right': text('confirmed')},
            'then': {'op': 'if', 'condition': rebound(completion),
                     'then': {'op': 'tag', 'value': payload}, 'else': text('mismatched-evidence')},
            'else': {'op': 'tag', 'value': captured}}


def adopt(source, ledger, body, provider):
    assert (ledger['profile'], ledger['version'], ledger['root']) == ('0.7.0', '0.6.0', 'expense')
    assert body['dialects']['core']['id'] == 'urn:choreoform:contracts:core-profile:0.4.0'
    assert not {BINDING, CAPTURE, REQUEST, CATEGORY, CLASSIFY} & {d['id'] for d in ledger['declarations']}
    assert source.count('authoring "0.7.0"') == 1
    ledger = deepcopy(ledger)
    ledger.update(profile='0.14.0', version='0.13.0')
    ledger['contracts']['core'] = deepcopy(CORE11)
    source = source.replace('authoring "0.7.0"', 'authoring "0.14.0"', 1)
    lines = source.splitlines(keepends=True)
    work = [i for i, line in enumerate(lines) if line.startswith('Rule for work "Payment work".')]
    assert len(work) == 1
    i, = work
    old = ', fault source "Payment faults".'
    authority = 'Authority "Payment operator". Attempts'
    assert lines[i].count(old) == lines[i].count(authority) == 1
    lines[i] = lines[i].replace(old, '.').replace(authority, 'Authority "Payment operator". Recognise faults from "Payment faults". Attempts')
    source = ''.join(lines)
    descriptor = {'kind': 'provider-terminal-observation-source', 'version': '0.1.0',
                  'id': 'urn:choreoform:benchmarks:rp01:payment-terminal:0.1.0', 'profile': PROFILE.copy(), 'provider': provider.copy()}
    raw = encoded(descriptor)
    source_pin = pin(descriptor['id'], raw)
    ledger['declarations'].append({'id': BINDING, 'kind': 'resource', 'scope': 'expense', 'name': 'Payment terminal facts'})
    ledger['resources'].append({'binding': BINDING, 'kind': descriptor['kind'], 'contract': source_pin['id'], 'digest': source_pin['revision']})
    captions = {row['id']: row['name'] for row in ledger['declarations']}
    envelope = body['data']['payment_result']['protection']
    def local(identity, caption, ty):
        ledger['declarations'].append({'id': identity, 'kind': 'data', 'scope': 'payment_recovery', 'name': caption})
        ledger['symbols'].append({'id': identity+'_purpose', 'owner': identity, 'role': 'purpose', 'name': 'Reimbursement', 'wire': 'reimbursement'})
        captions[identity] = caption
        return f'Keep {quote(caption)} as {type_source(ty)} under {quote(captions[envelope["policy"]])}. Sensitivity {quote(envelope["sensitivity"])}. Purposes ["Reimbursement"]. Participants {names(captions[a] for a in envelope["participants"])}. Capabilities {names(captions[a] for a in envelope["capabilities"])}. Invalidate ["Observe Payment", "Classify late payment", "Finish Payment recovery"]. Initially unavailable. End data.\n'
    declarations = 'Retain the original request from "Request Payment" and record in "Original payment request".\n'
    declarations += local(REQUEST, 'Original payment request', body['capabilities']['payment_capability']['input']['body'])
    declarations += local(CAPTURE, 'Late payment evidence', body['policies']['payment_wait']['body']['observationType'])
    declarations += local(CATEGORY, 'Late payment category', {'kind': 'text'})
    ledger['declarations'].append({'id': CLASSIFY, 'kind': 'node', 'scope': 'payment_recovery', 'name': 'Classify late payment'})
    ledger['symbols'].append({'id': CLASSIFY+'_next', 'owner': CLASSIFY, 'role': 'outcome', 'name': 'next', 'wire': 'next'})
    ledger['generated'] += [{'id': CLASSIFY+'_assignment', 'owner': CLASSIFY, 'role': 'assignment', 'key': CATEGORY},
                            {'id': CLASSIFY+'_flow', 'owner': CLASSIFY, 'role': 'flow', 'key': CLASSIFY+'_next'}]
    expr = classification(body['policies']['payment_work']['body']['completion'])
    def readable(value):
        if isinstance(value, dict):
            if value.get('op') == 'literal': return deepcopy(value)
            if value.get('op') == 'read': return {'op': 'read', 'cell': captions[value['cell']]}
            return {key: readable(child) for key, child in value.items()}
        if isinstance(value, list): return [readable(child) for child in value]
        return value
    declarations += f'Step "Classify late payment". Set together ["Late payment category" to ({expression_source(readable(expr))})]. On "next", continue with "Finish Payment recovery". End step.\n'
    marker = 'Step "Observe Payment".'
    assert source.count(marker) == 1
    source = source.replace(marker, declarations+marker)
    old = marker+' Wait under "Payment reconciliation". On "observed", continue with "Finish Payment recovery".'
    assert source.count(old) == 1
    source = source.replace(old, marker+' Wait under "Payment reconciliation" for observations from "Payment terminal facts" about "Request Payment" and record in "Late payment evidence". On "observed", continue with "Classify late payment".')
    return source, ledger, raw, source_pin
