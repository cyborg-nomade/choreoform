# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Explicit bounded order fixture helpers; not an editor, exporter or runtime."""
from .rp01_source import Package, Scope, quote, names, type_source, expression_source
from .rp01_capture import CORE11, PROFILE as TERMINAL_PROFILE
from .rp01_resources import TEXT, INTEGER, BOOLEAN, AMOUNT, record, variant, option, nominal, literal, field, binary, every, negate
from .rp03_resources import PEOPLE, ROLES, SPECS, CAPABILITIES, instruction


def read(name): return {'op':'read','cell':name}
def fact(name): return {'op':'fact','name':name}
def text(value): return literal(TEXT,value)
def eq(left,right): return binary('eq',left,right)
def tag(value): return {'op':'tag','value':value}
def payload(value,case): return {'op':'payload','value':value,'tag':case}
def raw(value): return {'op':'unwrapNamed','value':value}
def rec(**fields): return {'op':'record','fields':fields}
def conditional(condition,then,otherwise): return {'op':'if','condition':condition,'then':then,'else':otherwise}
def present(value): return {'op':'unwrap','value':value}
def exists(value): return {'op':'isSome','value':value}
def zero(): return literal(AMOUNT,'0')
def expr(value): return expression_source(value,'Order types')
def ty(value): return type_source(value,'Order types')


def new_package():
    p=Package(definition='rp03_benchmark',root='order',profile='0.14.0',version='0.13.0',core=CORE11,
        type_name='Order types',purpose=('fulfillment','Order fulfillment'),protection_policy='Order access',sensitivity='order')
    p.ledger['annotations']={'benchmark':'RP-03','synthetic':True,'variant':'two explicit identified lines and warehouses; no general fan-out output aggregation'}
    return p


def foundations(root):
    for key,name in zip(ROLES,PEOPLE):
        root.actor(key+'_actor',name,name+' authority')
        checks=[{'op':'contains','list':fact('roles'),'value':text('order.'+key)}]
        if key=='customer': checks.append({'op':'contains','list':fact('relationships'),'value':text('order.submitter')})
        root.policy(key+'_authority','actor',name+' authority',f'Permit when ({expr(every(checks))}).')
    root.actor('fixed_deadline_actor','Keep order deadlines fixed','No deadline extension')
    root.policy('fixed_deadline_authority','actor','No deadline extension','Permit when (false).')
    root.policy('order_access','protection','Order access','Permit access when ((the fact "purpose") equals ("fulfillment")).')
    root.policy('settle_children','settlement','Settle children','Unfinished work cancel.')
    root.policy('stop_work','cancel','Stop work','Authority "Commerce operator". For children use "Settle children".')
    root.policy('customer_cancel','cancel','Customer cancellation','Authority "Customer". For children use "Settle children".')
    root.policy('no_handlers','faults','Unhandled faults','Handle empty.')
    root.policy('ordinary_close','closure','Ordinary close','Fully terminal.')
    root.policy('acceptance_order','race','Acceptance order','Accept in acceptance order.')
    root.policy('order_close','closure','Preserve order obligations','Reconcile in "Order settlement".')
    # Guard evidence remains typed state, not a dispatch-authorizing host Boolean.
    order=raw(read('Original order'))
    guard=every([negate(eq(field(field(order,'first line'),'line ID'),field(field(order,'second line'),'line ID'))),
        eq(field(order,'amount'),binary('add',field(field(order,'first line'),'amount'),field(field(order,'second line'),'amount')))])
    root.policy('order_commits','race','Order commitments',f'Guard commits when ({expr(guard)}).')


def work(scope,key,name,completion,compensates=None,deadline=None):
    # Compensation purpose and own effect class remain independent.
    link=compensates if compensates else 'none'
    timeout=('clock "Order clock", basis "Original order", due ('+expr(deadline)+'), calendar none, pause authority "Keep order deadlines fixed"') if deadline else 'none'
    scope.policy(key+'_work','work',name+' work',f'Instructions "Use only the exact original instruction and current authority; retain uncertainty and every accepted effect.". Authority "Commerce operator". Attempts 1. Delays empty. Retry faults empty. Timeout {timeout}. Complete when ({expr(completion)}). Compensates {link}.')


def instruction_expression(order,scope_key,scope_revision,amount,basis,request_revision,quantity,warehouse):
    return rec(**{'order':order,'scope key':scope_key,'scope revision':scope_revision,
        'request revision':text(request_revision),'amount':amount,'currency':field(raw(order),'currency'),
        'basis reference':basis,'address revision':field(field(raw(order),'address reference'),'revision'),
        'target quantity':quantity,'target warehouse':warehouse})


def matched_completion(key,request,tags):
    result=literal(BOOLEAN,False)
    for case in reversed(tags):
        evidence=raw(payload(fact('result'),case))
        checks=[eq(field(evidence,'operation'),text(key)),eq(field(evidence,'phase'),text(case)),
            eq(field(evidence,'order ID'),field(raw(field(request,'order')),'order ID')),
            eq(field(evidence,'order revision'),field(raw(field(request,'order')),'revision'))]
        checks += [eq(field(evidence,k),field(request,k)) for k in ['scope key','scope revision','request revision','amount','currency','basis reference','address revision','target quantity','target warehouse']]
        if case not in ['reserved','authorized','packed','labelled','handed','delivered','released','voided','expired','captured','refunded','stopped','intercepted','notified']:
            checks.append(negate(eq(field(evidence,'reason'),text(''))))
        result=conditional(eq(tag(fact('result')),text(case)),every(checks),result)
    return result


def service(root,parent,key,provider,*,compensates=None,deadline=None):
    """One request/effect per occurrence, explicit typed local follow-up capture.

    The graph caller supplies original request construction and explicit result
    publication. Unknown is never an ordinary successful port value.
    """
    _,name,_,tags,_=next(row for row in SPECS if row[0]==key)
    package=root.package
    owner=Scope(package,key+'_service',name+' occurrence',parent,'Request '+name,
        inputs=[('request','Request',name+' request')],outputs=[('result','Result',name+' result')],
        closure=name+' close',race='Order commitments')
    owner.policy(key+'_close','closure',name+' close',f'Reconcile in {quote(name+" recovery")}.')
    owner.data(key+'_request',name+' request',provider['input'],participants=PEOPLE,capabilities=CAPABILITIES)
    owner.data(key+'_result',name+' result',provider['output'],participants=PEOPLE,capabilities=CAPABILITIES)
    owner.expression(key+'_input',name+' input',provider['input'],read(name+' request'))
    work(owner,key,name,matched_completion(key,read(name+' request'),tags),compensates,deadline)
    owner.step(key+'_activity','Request '+name,f'Request {quote(name)} using {quote(name+" input")} under {quote(name+" work")} and record in {quote(name+" result")}.',[(tag,'Finish '+name) for tag in tags])
    owner.step(key+'_done','Finish '+name,'Finish as "complete".')
    recovery=Scope(package,key+'_recovery',name+' recovery',owner,'Observe '+name)
    correlation=record(**{'logical effect':TEXT,'request digest':TEXT,'observation':TEXT})
    observation=variant({'confirmed':provider['output'],'noeffect':correlation,'rejected':correlation,'cancelled':correlation})
    recovery.data(key+'_capture',name+' late evidence',observation,participants=PEOPLE,capabilities=CAPABILITIES,
        invalidates=[quote('Observe '+name),quote('Finish '+name+' recovery')])
    recovery.declarations.insert(0,f'Retain the original request from {quote("Request "+name)} and record in {quote(name+" original request")}.')
    recovery.data(key+'_retained',name+' original request',provider['input'],participants=PEOPLE,capabilities=CAPABILITIES,
        invalidates=[quote('Observe '+name),quote('Finish '+name+' recovery')])
    recovery.step(key+'_observe','Observe '+name,f'Wait under {quote(name+" reconciliation")} for observations from {quote(name+" terminal facts")} about {quote("Request "+name)} and record in {quote(name+" late evidence")}.',[('observed','Finish '+name+' recovery')])
    recovery.step(key+'_recovered','Finish '+name+' recovery','Finish as "complete".')
    owner.child(recovery)
    root.capability(key+'_capability',name,name+' provider',provider['input'],provider['output'],'Commerce operator',name+' effects')
    root.policy(key+'_effects','effect',name+' effects',f'Effect is {provider["effectClass"]}. Idempotency is {provider["idempotency"]}. Reconcile under {quote(name+" reconciliation")}.')
    chain=[];cursor=recovery
    while cursor:chain.append(cursor.name);cursor=cursor.parent
    endpoint=quote('Observe '+name)+''.join(' in '+quote(part) for part in chain)
    root.policy(key+'_wait','wait',name+' reconciliation',f'Observe {ty(observation)}. Match when (true). Authority "Commerce operator". Timer none. On observation outcome "observed" of {endpoint}. On timer none.')
    parent.child(owner)
    return owner
