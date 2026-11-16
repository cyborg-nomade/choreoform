# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Complete fictional declaration fixtures; no real work or provider consent."""
from hashlib import sha256
from benchmarks.rp01_resources import encoded, INTEGER, literal, variant
from benchmarks.rp01_source import Package, Scope, quote
from check_compensation_authoring import CORE


def package(left='read-only', right='read-only', *, guarded=False):
    p=Package(definition='parallel_effects_audit',root='main',profile='0.16.0',version='0.15.0',core=CORE,
              purpose=('test','Test'),protection_policy='Protection',sensitivity='test')
    p.ledger['annotations']={'synthetic':True,'meaning':'external declaration boundary, no actual effects'}
    def scope(identity,name,parent,entry,outcomes=('complete',)):
        return Scope(p,identity,name,parent,entry,outcomes=outcomes,cancellation='Cancel',closure='Close',race='Race')
    root=scope('main','Main',None,'Fork');supplied=[]
    root.actor('operator','Operator','Authority')
    root.policy('authority','actor','Authority','Permit when (true).')
    root.policy('protection','protection','Protection','Permit access when ((the fact "purpose") equals ("test")).')
    root.policy('cancel','cancel','Cancel','Authority "Operator". For children use "Settlement".')
    root.policy('settle','settlement','Settlement','Unfinished work cancel.')
    root.policy('close','closure','Close','Fully terminal.')
    root.policy('faults','faults','Unhandled faults','Handle empty.')
    root.policy('race','race','Race','Guard commits when (true).' if guarded else 'Accept in acceptance order.')
    for key,mode in [('a',left),('b',right)]:
        done='Done '+key.upper()
        child=scope(key,key.upper(),root,done if mode=='silent' else 'Observe' if mode=='wait' else 'Work',[key])
        cap=key+'_cap'
        if mode not in ['human','wait','silent']:
            provider={'kind':'provider-contract','version':'0.1.0','id':'urn:test:parallel-effects:'+key+':'+mode,
                'input':INTEGER,'output':INTEGER,'outcomes':{'done':True},
                'resultProtection':{'policy':'protection','sensitivity':'test','purposes':{'test':True},'participants':{'operator':True},'capabilities':{cap:True}},
                'observations':{'done':{'state':'succeeded','result':INTEGER,'outcome':'done'},'unknown':{'state':'unknown','result':None,'outcome':None}},
                'idempotency':'none' if mode=='read-only' else 'stable-key','effectClass':mode}
            raw=encoded(provider);pin={'id':provider['id'],'revision':'sha256:'+sha256(raw).hexdigest()}
            supplied.append((pin['id'],pin['revision'],raw));p.resource(key+'_provider',key.upper()+' provider','provider',pin)
            child.capability(cap,'Adapter',key.upper()+' provider',INTEGER,INTEGER,'Operator','Effects')
            child.policy(key+'_effects','effect','Effects',f'Effect is {mode}. Idempotency is {provider["idempotency"]}. Reconcile under "Observations".')
        if mode not in ['human','silent']:
            child.policy(key+'_wait','wait','Observations','Observe whole number. Match when ((the fact "payload") equals (0)). Authority "Operator". Timer none. On observation outcome "done" of "Observe". On timer none.')
            child.step(key+'_observe','Observe','Wait under "Observations".',[('done',done)])
        if mode not in ['silent','wait']:
            child.data(key+'_result','Result',variant({'done':INTEGER}) if mode=='human' else INTEGER,participants=['Operator'],capabilities=[] if mode=='human' else ['Adapter'])
            child.expression(key+'_input','Input',INTEGER,literal(INTEGER,'0'))
            completion='(the payload of (the fact "result") for "done")' if mode=='human' else '(the fact "result")'
            child.policy(key+'_work','work','Work rules',f'Instructions "Synthetic integer receipt, no dispatch grant". Authority "Operator". Attempts 1. Delays empty. Retry faults empty. Timeout none. Complete when ({completion} equals (0)). Compensates none.')
            action='Ask "Operator" to decide using "Input" under "Work rules" and record in "Result".' if mode=='human' else 'Request "Adapter" using "Input" under "Work rules" and record in "Result".'
            child.step(key+'_activity','Work',action,[('done',done)])
        child.step(key+'_done',done,'Finish as '+quote(key)+'.');root.child(child);p.symbol('fork','branch',key)
    root.step('fork','Fork','Start together ["a" runs "A", "b" runs "B"] and gather at "Gather".')
    root.step('gather','Gather','Gather from "Fork" when all report [outcome "a" of "A", outcome "b" of "B"]; for unfinished work use "Settlement".',[('gathered','Done')])
    root.step('done','Done','Finish as "complete".')
    return 'Choreoform authoring "0.16.0".\n'+root.render()+'\n',p.ledger,supplied
