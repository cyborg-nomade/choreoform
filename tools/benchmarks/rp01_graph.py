# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Whole RP-01 graph construction; this does not admit or execute a process."""
from .rp01_resources import (
    TEXT, INTEGER, BOOLEAN, binary, every, field, literal, negate, nominal, option, record, variant,
)
from .rp01_source import Package, Scope, expression_source, quote, type_source

PEOPLE = ['Employee', 'Manager', 'Finance reviewer', 'Payment operator', 'Clock observer']
CAPABILITIES = ['Check the claim', 'Create the payable', 'Notify the employee']
REASONS = {'kind': 'list', 'element': TEXT}


def reasons(*values):
    return {'op': 'list', 'element': TEXT, 'items': list(values)}


def read(name):
    return {'op': 'read', 'cell': name}


def fact(name):
    return {'op': 'fact', 'name': name}


def representation(value):
    return {'op': 'unwrapNamed', 'value': value}


def payload(value, tag):
    return {'op': 'payload', 'value': value, 'tag': tag}


def present(value):
    return {'op': 'unwrap', 'value': value}


def equal(left, right):
    return binary('eq', left, right)


def text(value):
    return literal(TEXT, value)


def role(name):
    return {'op': 'contains', 'list': fact('roles'), 'value': text('expense.' + name)}


def record_expression(**fields):
    return {'op': 'record', 'fields': fields}


def evidence_matches(evidence, claim, revision_field='revision'):
    return every([
        equal(field(evidence, 'claim ID'), field(claim, 'claim ID')),
        equal(field(evidence, 'claim revision'), field(claim, revision_field)),
    ])


def result_matches(tags, claim, extra=None, revision_field='revision'):
    """Branch on the real variant before extracting its nominal evidence."""
    result = literal(BOOLEAN, False)
    for tag in reversed(tags):
        evidence = representation(payload(fact('result'), tag))
        checks = [evidence_matches(evidence, claim, revision_field)]
        if extra:
            checks += extra(tag, evidence)
        result = {
            'op': 'if', 'condition': equal({'op': 'tag', 'value': fact('result')}, text(tag)),
            'then': every(checks), 'else': result,
        }
    return result


def work(scope, identity, name, authority, instructions, completion, retry=None, timeout='none'):
    attempts = 2 if retry else 1
    clause = f'For retries use clock "Expense clock", basis {quote(retry)}. ' if retry else ''
    faults = '["confirmed no effect"]' if retry else 'empty'
    scope.policy(identity, 'work', name,
        f'Instructions {quote(instructions)}. Authority {quote(authority)}. Attempts {attempts}. '
        f'Delays {"[1000 microseconds]" if retry else "empty"}. Retry faults {faults}. {clause}'
        f'Timeout {timeout}. Complete when ({expression_source(completion)}). Compensates none.')


def foundations(root):
    for key, name in zip(['employee', 'manager', 'finance', 'operator', 'clock'], PEOPLE):
        root.actor(f'{key}_actor', name, name + ' authority')
        predicate = role(key)
        if key == 'employee':
            # This is authenticated external role/relationship assignment bound
            # to the instance, not a relation inferred from an untrusted name.
            predicate = every([predicate, {'op': 'contains', 'list': fact('relationships'), 'value': text('request.submitter')}])
        root.policy(f'{key}_authority', 'actor', name + ' authority', f'Permit when ({expression_source(predicate)}).')
    root.actor('fixed_deadline_actor', 'Keep the deadline fixed', 'No deadline extension')
    root.policy('fixed_deadline_authority', 'actor', 'No deadline extension', 'Permit when (false).')
    root.policy('expense_access', 'protection', 'Expense access', 'Permit access when ((the fact "purpose") equals ("reimbursement")).')
    root.policy('settle_children', 'settlement', 'Settle children', 'Unfinished work cancel.')
    root.policy('stop_work', 'cancel', 'Stop work', 'Authority "Payment operator". For children use "Settle children".')
    root.policy('withdraw_request', 'cancel', 'Withdraw request', 'Authority "Employee". For children use "Settle children".')
    root.policy('no_handlers', 'faults', 'Unhandled faults', 'Handle empty.')
    root.policy('ordinary_close', 'closure', 'Ordinary close', 'Fully terminal.')
    root.policy('acceptance_order', 'race', 'Acceptance order', 'Accept in acceptance order.')
    root.policy('preserve_remaining', 'closure', 'Preserve remaining work', 'Reconcile in "Remaining work".')
    remaining = Scope(root.package, 'remaining', 'Remaining work', root, 'Release remaining owner')
    remaining.step('remaining_done', 'Release remaining owner', 'Finish as "complete".')
    root.child(remaining)


def service(root, parent, provider, key, name, capability_name, input_name, output_name, permitted=None, completion_override=None):
    """One capability activity per owner occurrence, with an explicit follow-up.

    Capabilities stay visible at root so its data can name their permitted access.
    The ancestor wait policy needs an actual-use endpoint binding beyond profile
    0.4's policy-own-scope restriction; measure and record that dependency honestly.
    """
    package = root.package
    permitted = CAPABILITIES if permitted is None else permitted
    input_type, output_type = provider['input'], provider['output']
    inputs = [('request', 'Request', input_name)]
    if key == 'payment':
        inputs.append(('deadline', 'Deadline', 'Payment deadline'))
    owner = Scope(package, key + '_service', name, parent, 'Request ' + name,
        inputs=inputs, outputs=[('result', 'Result', output_name)], closure=name + ' close',
        race=name + ' commits' if key == 'payment' else 'Acceptance order')
    owner.policy(key + '_close', 'closure', name + ' close', f'Reconcile in {quote(name + " recovery")}.')
    owner.data(key + '_request', input_name, input_type, participants=PEOPLE, capabilities=permitted)
    owner.data(key + '_result', output_name, output_type, participants=PEOPLE, capabilities=permitted)
    owner.expression(key + '_input', name + ' input', input_type, read(input_name))
    claim = representation(read(input_name)) if key == 'checks' else representation(field(read(input_name), 'claim')) if key == 'payment' else read(input_name)
    completion = result_matches(list(output_type['cases']), claim)
    if key == 'checks':
        completion = result_matches(list(output_type['cases']), claim,
            lambda tag, e: [equal(field(e, 'policy revision'), field(claim, 'policy revision'))]
                + ([binary('lt', literal(INTEGER, '0'), {'op': 'length', 'value': field(e, 'reasons')})] if tag != 'clear' else [])
                + ([{'op': 'isSome', 'value': field(e, 'duplicate claim')}] if tag == 'duplicate' else []))
    if key == 'notice':
        completion = result_matches(list(output_type['cases']), claim,
            lambda _, e: [equal(field(e, 'recipient'), field(read(input_name), 'recipient'))], revision_field='claim revision')
    if key == 'payment':
        owner.data('payment_deadline', 'Payment deadline', INTEGER, participants=PEOPLE, capabilities=CAPABILITIES)
        witnesses = [representation(field(read(input_name), person)) for person in ['manager', 'finance']]
        guard = every([evidence_matches(w, claim) for w in witnesses] + [
            equal(field(claim, 'revision'), field(representation(read('Current claim')), 'revision')),
            equal(field(claim, 'claim ID'), field(representation(read('Current claim')), 'claim ID')),
        ])
        owner.policy('payment_commits', 'race', name + ' commits', f'Guard commits when ({expression_source(guard)}).')
        completion = result_matches(list(output_type['cases']), claim,
            lambda tag, e: [equal(field(e, 'amount'), field(claim, 'amount')), equal(field(e, 'currency'), field(claim, 'currency'))] if tag == 'paid' else [])
    timeout = 'clock "Expense clock", basis "Payment deadline", due (the value of "Payment deadline"), calendar none, pause authority "Keep the deadline fixed"' if key == 'payment' else 'none'
    work(owner, key + '_work', name + ' work', 'Payment operator',
        'Process only this exact request revision. Retain unknown effects; never infer payment or delivery from a request.',
        completion if completion_override is None else completion_override, retry=input_name if key == 'payment' else None, timeout=timeout)
    owner.step(key + '_request_step', 'Request ' + name,
        f'Request {quote(capability_name)} using {quote(name + " input")} under {quote(name + " work")} and record in {quote(output_name)}.',
        [(tag, 'Finish ' + name) for tag in output_type['cases']])
    owner.step(key + '_done', 'Finish ' + name, 'Finish as "complete".')
    recovery = Scope(package, key + '_recovery', name + ' recovery', owner, 'Observe ' + name)
    recovery.step(key + '_observe', 'Observe ' + name, f'Wait under {quote(name + " reconciliation")}.', [('observed', 'Finish ' + name + ' recovery')])
    recovery.step(key + '_recovered', 'Finish ' + name + ' recovery', 'Finish as "complete".')
    owner.child(recovery)
    root.capability(key + '_capability', capability_name, name + ' provider', input_type, output_type, 'Payment operator', name + ' effects')
    root.policy(key + '_effects', 'effect', name + ' effects',
        f'Effect is {provider["effectClass"]}. Idempotency is {provider["idempotency"]}. Reconcile under {quote(name + " reconciliation")}.')
    correlation = record(**{'logical effect': TEXT, 'request digest': TEXT, 'observation': TEXT})
    observation_type = variant({'confirmed': output_type, 'noeffect': correlation, 'rejected': correlation, 'cancelled': correlation})
    # This is an explicit terminal-fact envelope contract for the benchmark host,
    # not an ordinary provider result or proof of adapter support/authenticity.
    chain = []
    cursor = recovery
    while cursor:
        chain.append(cursor.name)
        cursor = cursor.parent
    qualified = quote('Observe ' + name) + ''.join(' in ' + quote(part) for part in chain)
    root.policy(key + '_wait', 'wait', name + ' reconciliation',
        f'Observe {type_source(observation_type)}. Match when (true). Authority "Payment operator". Timer none. '
        f'On observation outcome "observed" of {qualified}. On timer none.')
    parent.child(owner)
    return input_type, output_type


def endpoint_probe(type_pin, provider, provider_pin):
    """Isolate the ancestor-policy endpoint relation below old parser quotas.

    This is a declaration witness, not reimbursement or domain-policy evidence.
    """
    package = Package()
    root = Scope(package, 'expense', 'Expense reimbursement', None, 'Run worker',
        inputs=[('claim', 'Claim', 'Current claim')])
    package.resource('expense_types', 'Expense types', 'named-type', type_pin)
    package.resource('checks_provider_binding', 'Claim checks provider', 'provider', provider_pin)
    foundations(root)
    service(root, root, provider, 'checks', 'Claim checks', 'Check the claim', 'Check request', 'Check outcome',
        permitted=['Check the claim'], completion_override=literal(BOOLEAN, True))
    root.expression('probe_input', 'Claim input', provider['input'], read('Current claim'))
    root.data('current_claim', 'Current claim', provider['input'], participants=PEOPLE, capabilities=['Check the claim'])
    root.data('probe_result', 'Root check finding', provider['output'], participants=PEOPLE, capabilities=['Check the claim'])
    root.step('probe_run', 'Run worker', 'Run "Claim checks" with inputs ["Request" uses "Claim input"] and outputs ["Result" uses "Root check finding"].', [('complete', 'Finish witness')])
    root.step('probe_done', 'Finish witness', 'Finish as "complete".')
    return 'Choreoform authoring "0.4.0".\n' + root.render(), package


def build(type_pin, provider_documents, resource_pins):
    package = Package()
    package.ledger.update(profile="0.5.0", version="0.4.0")
    root = Scope(package, 'expense', 'Expense reimbursement', None, 'Review again',
        inputs=[('claim', 'Claim', 'Current claim'), ('deadline', 'Payment due', 'Payment due')],
        outputs=[('disposition', 'Disposition', 'Disposition'), ('payable', 'Payable', 'Payable reference')],
        outcomes=['paid', 'rejected', 'problem'], closure='Preserve remaining work', cancellation='Withdraw request')
    package.resource('expense_types', 'Expense types', 'named-type', type_pin)
    for key, name in [('checks', 'Claim checks'), ('payment', 'Payment'), ('notice', 'Notice')]:
        package.resource(key + '_provider_binding', name + ' provider', 'provider', resource_pins[key])
    package.resource('expense_clock', 'Expense clock', 'clock', resource_pins['clock'])
    foundations(root)
    claim_type = nominal(type_pin, 'Claim')
    review_type = nominal(type_pin, 'Review evidence')
    approval_type = variant({tag: review_type for tag in ['approved', 'correction', 'rejected']})
    correction_type = variant({'resubmitted': claim_type})
    duplicate_type = variant({tag: review_type for tag in ['clear', 'correction', 'rejected']})
    cycle = Scope(package, 'review_cycle', 'Review cycle', root, 'Run checks')
    _, checks_type = service(root, cycle, provider_documents['checks'], 'checks', 'Claim checks', 'Check the claim', 'Check request', 'Check outcome')
    service(root, root, provider_documents['payment'], 'payment', 'Payment', 'Create the payable', 'Payable request', 'Payment outcome')
    service(root, root, provider_documents['notice'], 'notice', 'Notice', 'Notify the employee', 'Notice request', 'Notice outcome')
    cycle.data('cycle_checks', 'Check finding', checks_type, participants=PEOPLE, capabilities=CAPABILITIES)
    cycle.data('manager_result', 'Manager result', approval_type, participants=PEOPLE, capabilities=CAPABILITIES)
    cycle.data('finance_result', 'Finance result', approval_type, participants=PEOPLE, capabilities=CAPABILITIES)
    cycle.data('duplicate_result', 'Duplicate result', duplicate_type, participants=PEOPLE, capabilities=CAPABILITIES)
    cycle.data('corrected_result', 'Corrected result', correction_type, participants=PEOPLE, capabilities=CAPABILITIES)
    cycle.data('correction_reasons', 'Correction reasons', REASONS, reasons(), participants=PEOPLE, capabilities=CAPABILITIES)
    cycle.expression('claim_for_checks', 'Claim for checks', claim_type, read('Current claim'))
    material_type = record(claim=claim_type, findings=checks_type)
    cycle.expression('review_material', 'Review material', material_type,
        record_expression(claim=read('Current claim'), findings=read('Check finding')))
    correction_input = record(claim=claim_type, reasons=REASONS)
    cycle.expression('claim_for_correction', 'Claim for correction', correction_input,
        record_expression(claim=read('Current claim'), reasons=read('Correction reasons')))
    for tag in ['clear', 'correction', 'duplicate']:
        cycle.expression('check_is_' + tag, 'Check is ' + tag, BOOLEAN,
            equal({'op': 'tag', 'value': read('Check finding')}, text(tag)))
    current_claim = representation(read('Current claim'))
    for key, name, authority, result_name in [
        ('manager', 'Manager review', 'Manager', 'Manager result'),
        ('finance', 'Finance review', 'Finance reviewer', 'Finance result'),
        ('duplicate', 'Duplicate resolution', 'Finance reviewer', 'Duplicate result'),
    ]:
        tags = list(duplicate_type['cases']) if key == 'duplicate' else list(approval_type['cases'])
        def checks(tag, evidence, key=key):
            predicates = [
                equal(field(evidence, 'policy revision'), field(current_claim, 'policy revision')),
                equal(field(evidence, 'principal'), fact('principal')),
            ]
            if key == 'manager':
                predicates.append(negate(equal(fact('principal'), field(current_claim, 'employee'))))
            if key == 'finance':
                prior = representation(present(read('Manager evidence')))
                predicates.append(negate(equal(fact('principal'), field(prior, 'principal'))))
            return predicates
        work(cycle, key + '_review_work', name + ' rules', authority,
            'Decide this exact claim and policy revision; record attributable reasons and current assignment. A deadline never supplies approval.',
            result_matches(tags, current_claim, checks))
        input_expression = 'Review material'
        if key == 'manager':
            successors = [('approved', 'Record manager approval'), ('correction', 'Record manager correction'), ('rejected', 'Record manager rejection')]
        elif key == 'finance':
            successors = [('approved', 'Record finance approval'), ('correction', 'Record finance correction'), ('rejected', 'Record finance rejection')]
        else:
            successors = [('clear', 'Manager review'), ('correction', 'Record duplicate correction'), ('rejected', 'Record duplicate rejection')]
        cycle.step(key + '_review', name,
            f'Ask {quote(authority)} to decide using {quote(input_expression)} under {quote(name + " rules")} and record in {quote(result_name)}.', successors)
    corrected = representation(payload(fact('result'), 'resubmitted'))
    correction_guard = every([
        equal(field(corrected, 'claim ID'), field(current_claim, 'claim ID')),
        equal(field(corrected, 'employee'), field(current_claim, 'employee')),
        negate(equal(field(corrected, 'revision'), field(current_claim, 'revision'))),
        equal(fact('principal'), field(current_claim, 'employee')),
    ])
    work(cycle, 'correction_work', 'Correction rules', 'Employee',
        'Supply a new claim revision for the same employee and claim identity. Prior approvals remain history and do not approve this revision.', correction_guard)
    cycle.step('correct_claim', 'Correct the claim',
        'Ask "Employee" to decide using "Claim for correction" under "Correction rules" and record in "Corrected result".', [('resubmitted', 'Publish corrected claim')])
    cycle.compute('publish_correction', 'Publish corrected claim', [
        ('current_claim', 'Current claim', payload(read('Corrected result'), 'resubmitted')),
        ('review_needed', 'Review needed', literal(BOOLEAN, True)),
        ('disposition', 'Disposition', text('pending')),
    ], 'Finish review cycle')
    cycle.compute('record_manager', 'Record manager approval', [
        ('manager_evidence', 'Manager evidence', {'op': 'some', 'value': payload(read('Manager result'), 'approved')}),
    ], 'Finance review')
    cycle.compute('record_finance', 'Record finance approval', [
        ('finance_evidence', 'Finance evidence', {'op': 'some', 'value': payload(read('Finance result'), 'approved')}),
        ('review_needed', 'Review needed', literal(BOOLEAN, False)),
        ('disposition', 'Disposition', text('approved')),
    ], 'Finish review cycle')
    cycle.compute('reject_claim', 'Reject the claim', [
        ('review_needed', 'Review needed', literal(BOOLEAN, False)),
        ('disposition', 'Disposition', text('rejected')),
        ('disposition_reasons', 'Disposition reasons', read('Correction reasons')),
    ], 'Finish review cycle')
    for key, result_name in [('manager', 'Manager result'), ('finance', 'Finance result'), ('duplicate', 'Duplicate result')]:
        for tag, suffix in [('correction', 'correction'), ('rejected', 'rejection')]:
            cause = field(representation(payload(read(result_name), tag)), 'reason')
            cycle.compute(f'{key}_{suffix}_reason', f'Record {key} {suffix}',
                [('correction_reasons', 'Correction reasons', reasons(cause))],
                'Correct the claim' if tag == 'correction' else 'Reject the claim')
    for tag, suffix in [('correction', 'correction'), ('rejected', 'rejection')]:
        cause = field(representation(payload(read('Check finding'), tag)), 'reasons')
        cycle.compute(f'checks_{suffix}_reason', f'Record check {suffix}',
            [('correction_reasons', 'Correction reasons', cause)],
            'Correct the claim' if tag == 'correction' else 'Reject the claim')
    cycle.step('run_checks', 'Run checks',
        'Run "Claim checks" with inputs ["Request" uses "Claim for checks"] and outputs ["Result" uses "Check finding"].', [('complete', 'Choose check result')])
    cycle.step('choose_check', 'Choose check result',
        'Choose by ["clear" when "Check is clear", "correction" when "Check is correction", "duplicate" when "Check is duplicate"]; otherwise "rejected".',
        [('clear', 'Manager review'), ('correction', 'Record check correction'), ('duplicate', 'Duplicate resolution'), ('rejected', 'Record check rejection')])
    cycle.step('cycle_done', 'Finish review cycle', 'Finish as "complete".')
    root.child(cycle)
    root.expression('needs_review', 'Another review is needed', BOOLEAN, read('Review needed'))
    manager = representation(present(read('Manager evidence')))
    finance = representation(present(read('Finance evidence')))
    ready = every([
        equal(read('Disposition'), text('approved')),
        {'op': 'isSome', 'value': read('Manager evidence')},
        {'op': 'isSome', 'value': read('Finance evidence')},
        evidence_matches(manager, current_claim), evidence_matches(finance, current_claim),
        equal(field(manager, 'policy revision'), field(current_claim, 'policy revision')),
        equal(field(finance, 'policy revision'), field(current_claim, 'policy revision')),
    ])
    root.expression('may_pay', 'Current approvals permit payment', BOOLEAN, ready)
    root.expression('pay_input', 'Approved payable instruction', provider_documents['payment']['input'],
        record_expression(claim=read('Current claim'), manager=present(read('Manager evidence')), finance=present(read('Finance evidence'))))
    root.expression('pay_due', 'Due for payment', INTEGER, read('Payment due'))
    payment_type = provider_documents['payment']['output']
    root.expression('was_paid', 'Payment is confirmed', BOOLEAN,
        equal({'op': 'tag', 'value': read('Recorded payment result')}, text('paid')))
    root.expression('status_notice_input', 'Status notice', provider_documents['notice']['input'], record_expression(**{
        'claim ID': field(current_claim, 'claim ID'), 'claim revision': field(current_claim, 'revision'),
        'recipient': field(current_claim, 'employee'), 'disposition': read('Disposition'),
        'reasons': read('Disposition reasons'), 'payable reference': read('Payable reference'),
    }))
    root.expression('is_paid', 'Disposition is paid', BOOLEAN, equal(read('Disposition'), text('paid')))
    root.expression('is_rejected', 'Disposition is rejected', BOOLEAN, equal(read('Disposition'), text('rejected')))
    root.step('review_again', 'Review again', 'While "Another review is needed" is true, repeat "Review cycle".', [('next', 'Choose payment')])
    root.step('choose_payment', 'Choose payment', 'Choose by ["pay" when "Current approvals permit payment"]; otherwise "check".', [('pay', 'Run payment'), ('check', 'Check rejection')])
    root.step('check_rejection', 'Check rejection', 'Choose by ["rejected" when "Disposition is rejected"]; otherwise "stale".',
        [('rejected', 'Run notice'), ('stale', 'Record stale payment evidence')])
    root.compute('stale_payment', 'Record stale payment evidence', [
        ('disposition', 'Disposition', text('problem')),
        ('disposition_reasons', 'Disposition reasons', reasons(text('No payable was requested because current approval evidence was unavailable or stale.'))),
    ], 'Run notice')
    root.step('run_payment', 'Run payment',
        'Run "Payment" with inputs ["Request" uses "Approved payable instruction", "Deadline" uses "Due for payment"] and outputs ["Result" uses "Recorded payment result"].', [('complete', 'Choose payment result')])
    root.step('choose_paid', 'Choose payment result', 'Choose by ["paid" when "Payment is confirmed"]; otherwise "problem".', [('paid', 'Record paid status'), ('problem', 'Record payment problem')])
    paid_evidence = representation(payload(read('Recorded payment result'), 'paid'))
    root.compute('record_paid', 'Record paid status', [
        ('disposition', 'Disposition', text('paid')),
        ('payable_reference', 'Payable reference', {'op': 'some', 'value': field(paid_evidence, 'payable reference')}),
    ], 'Run notice')
    root.compute('record_problem', 'Record payment problem', [
        ('disposition', 'Disposition', text('problem')),
        ('disposition_reasons', 'Disposition reasons', reasons(field(representation(payload(read('Recorded payment result'), 'problem')), 'reason'))),
    ], 'Run notice')
    root.step('run_notice', 'Run notice',
        'Run "Notice" with inputs ["Request" uses "Status notice"] and outputs ["Result" uses "Recorded notice result"].', [('complete', 'Choose published outcome')])
    root.step('publish_outcome', 'Choose published outcome',
        'Choose by ["paid" when "Disposition is paid", "rejected" when "Disposition is rejected"]; otherwise "problem".',
        [('paid', 'Finish paid'), ('rejected', 'Finish rejected'), ('problem', 'Finish payment problem')])
    for identity, name, outcome in [('finish_paid', 'Finish paid', 'paid'), ('finish_rejected', 'Finish rejected', 'rejected'), ('finish_problem', 'Finish payment problem', 'problem')]:
        root.step(identity, name, f'Finish as {quote(outcome)}.')
    # Late declaration is legal: all inventories are present before lowering.
    # Explicit conservative invalidation covers ordinary work, preserving late
    # reconciliation facts rather than invalidating surviving wait definitions.
    declarations = {d['id']: d for d in package.ledger['declarations']}
    invalidates = []
    for identity, (scope_id, name, action) in package.actions.items():
        if scope_id.endswith('_recovery') or scope_id == 'remaining' or action.startswith('Finish'):
            continue
        chain = []
        cursor = scope_id
        while cursor:
            chain.append(declarations[cursor]['name'])
            cursor = declarations[cursor]['scope']
        invalidates.append(quote(name) + ''.join(' in ' + quote(part) for part in chain))
    root.data('current_claim', 'Current claim', claim_type, invalidates=invalidates, participants=PEOPLE, capabilities=CAPABILITIES)
    root.data('payment_due', 'Payment due', INTEGER, participants=PEOPLE, capabilities=CAPABILITIES)
    root.data('review_needed', 'Review needed', BOOLEAN, literal(BOOLEAN, True), participants=PEOPLE, capabilities=CAPABILITIES)
    root.data('disposition', 'Disposition', TEXT, text('pending'), participants=PEOPLE, capabilities=CAPABILITIES)
    root.data('disposition_reasons', 'Disposition reasons', REASONS, reasons(), participants=PEOPLE, capabilities=CAPABILITIES)
    for identity, name in [('manager_evidence', 'Manager evidence'), ('finance_evidence', 'Finance evidence')]:
        root.data(identity, name, option(review_type), {'op': 'none', 'element': review_type}, participants=PEOPLE, capabilities=CAPABILITIES)
    root.data('payable_reference', 'Payable reference', option(TEXT), {'op': 'none', 'element': TEXT}, participants=PEOPLE, capabilities=CAPABILITIES)
    root.data('recorded_payment', 'Recorded payment result', payment_type, participants=PEOPLE, capabilities=CAPABILITIES)
    root.data('recorded_notice', 'Recorded notice result', provider_documents['notice']['output'], participants=PEOPLE, capabilities=CAPABILITIES)
    source = '# SPDX-FileCopyrightText: 2026 Choreoform contributors\n# SPDX-License-Identifier: MPL-2.0\n# Synthetic whole-case draft; not admitted or executable.\nChoreoform authoring "0.5.0".\n' + root.render() + '\n'
    return source, package
