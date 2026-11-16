# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Synthetic bounded source construction, never an editor, adapter or host grant."""
from hashlib import sha256
from benchmarks.rp01_source import Package,Scope,quote,type_source
from benchmarks.rp01_resources import encoded,record,variant,TEXT,INTEGER
from check_compensation_authoring import CORE
from check_static_contracts import ROOT

TERMINAL=('urn:choreoform:observation-source-profile:provider-terminal-facts:0.1.0','sha256:a224fc2b8318af2ff8dfc9955b55af03f4d5b7bd0de719de0384421698d20ead')
FAULTS=('urn:choreoform:fault-source-profile:provider-observations:0.1.0','sha256:f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0')
REQUEST=record(**{'request revision':TEXT,'amount':INTEGER})


def pin(document):
    raw=encoded(document);return {'id':document['id'],'revision':'sha256:'+sha256(raw).hexdigest()},raw


def package(*,effect='compensatable',compensates=False,recipients=('a_adapter','b_adapter')):
    p=Package(definition='effectful_instance_proof',root='main',profile='0.16.0',version='0.15.0',core=CORE,type_name='Unused types',purpose=('test','Test'),protection_policy='Protection',sensitivity='test')
    p.ledger['annotations']={'synthetic':True,'meaning':'bounded static/conditional declarations; no accepted effects or authority'}
    caps=['A capability','B capability'];people=['Operator']
    provider={'kind':'provider-contract','version':'0.1.0','id':'urn:test:instance-proof:'+effect,'input':REQUEST,'output':REQUEST,'outcomes':{'done':True},
        'resultProtection':{'policy':'protection','sensitivity':'test','purposes':{'test':True},'participants':{'operator':True},'capabilities':{key:True for key in recipients}},
        'observations':{'done':{'state':'succeeded','result':REQUEST,'outcome':'done'},'unknown':{'state':'unknown','result':None,'outcome':None}},
        'idempotency':'none' if effect=='read-only' else 'stable-key','effectClass':effect}
    pp,pr=pin(provider)
    terminal={'kind':'provider-terminal-observation-source','version':'0.1.0','id':'urn:test:instance-proof-terminal:'+effect,'profile':{'id':TERMINAL[0],'revision':TERMINAL[1]},'provider':pp}
    tp,tr=pin(terminal)
    faults={'kind':'provider-fault-source','version':'0.1.0','id':'urn:test:instance-proof-faults:'+effect,'profile':{'id':FAULTS[0],'revision':FAULTS[1]},'provider':pp,'emissions':{'uncertainty':{'observations':{'unknown':True},'evidence':'unresolved'}}}
    fp,fr=pin(faults)
    for identity,name,kind,binding in [('provider_binding','Provider','provider',pp),('terminal_binding','Terminal facts','provider-terminal-observation-source',tp),('fault_binding','Fault facts','provider-fault-source',fp)]:p.resource(identity,name,kind,binding)
    def scope(identity,name,parent,entry,inputs=(),outputs=(),closure='Close'):
        return Scope(p,identity,name,parent,entry,inputs=inputs,outputs=outputs,cancellation='Cancel',closure=closure,race='Race')
    def data(s,identity,name,ty,invalidates=()):s.data(identity,name,ty,participants=people,capabilities=caps,invalidates=[' in '.join(quote(part) for part in n) if isinstance(n,tuple) else quote(n) for n in invalidates])
    work_paths=[('Request work',c+' service',c+' caller','Main') for c in ['A','B']]
    root=scope('main','Main',None,'Use A',inputs=[('request','Request','Launch request')],outputs=[('first','First','First result'),('second','Second','Second result')])
    root.actor('operator','Operator','Authority')
    root.policy('authority','actor','Authority','Permit when ((the fact "principal") equals ("synthetic-operator")).')
    root.policy('protection','protection','Protection','Permit access when ((the fact "purpose") equals ("test")).')
    root.policy('cancel','cancel','Cancel','Authority "Operator". For children use "Settlement".')
    root.policy('settlement','settlement','Settlement','Unfinished work cancel.')
    root.policy('close','closure','Close','Fully terminal.')
    root.policy('faults','faults','Unhandled faults','Handle empty.')
    root.policy('race','race','Race','Accept in acceptance order.')
    for identity,name in [('launch','Launch request'),('first_result','First result'),('second_result','Second result')]:data(root,identity,name,REQUEST,work_paths if identity=='launch' else [work_paths[1]] if identity=='first_result' else [])
    root.expression('launch_input','Launch input',REQUEST,{'op':'read','cell':'Launch request'})
    for key,caption,next_node in [('a','A','Use B'),('b','B','Main done')]:
        caller=scope(key+'_caller',caption+' caller',root,'Run service',inputs=[('request','Request','Caller request')],outputs=[('result','Result','Caller result')])
        data(caller,key+'_caller_request','Caller request',REQUEST,[work_paths[0 if key=='a' else 1]]);data(caller,key+'_caller_result','Caller result',REQUEST)
        caller.expression(key+'_caller_input','Caller input',REQUEST,{'op':'read','cell':'Caller request'})
        service=scope(key+'_service',caption+' service',caller,'Request work',inputs=[('request','Request','Service request')],outputs=[('result','Result','Service result')],closure='Service close')
        data(service,key+'_request','Service request',REQUEST,['Request work']);data(service,key+'_result','Service result',REQUEST)
        service.expression(key+'_input','Service input',REQUEST,{'op':'read','cell':'Service request'})
        service.policy(key+'_close','closure','Service close','Reconcile in "Recovery".')
        link='"Request work" in "A service" in "A caller" in "Main"' if key=='b' and compensates else 'none'
        service.policy(key+'_work','work','Work','Instructions "Synthetic receipt matching; no dispatch authorization". Authority "Operator". Recognise faults from "Fault facts". Attempts 1. Delays empty. Retry faults empty. Timeout none. Complete when ((the fact "result") equals (the value of "Service request")). Compensates '+link+'.')
        service.step(key+'_activity','Request work',f'Request "{caption} capability" using "Service input" under "Work" and record in "Service result".',[('done','Service done')])
        service.step(key+'_done','Service done','Finish as "complete".')
        recovery=scope(key+'_recovery','Recovery',service,'Observe work')
        observation=variant({'confirmed':REQUEST})
        recovery.declarations.append('Retain the original request from "Request work" and record in "Original request".')
        for identity,name,ty in [(key+'_retained','Original request',REQUEST),(key+'_captured','Captured',observation)]:data(recovery,identity,name,ty,['Observe work','Recovery done'])
        recovery.step(key+'_observe','Observe work',f'Wait under "{caption} observations" for observations from "Terminal facts" about "Request work" and record in "Captured".',[('observed','Recovery done')])
        recovery.step(key+'_recovered','Recovery done','Finish as "complete".')
        service.child(recovery);caller.child(service)
        caller.step(key+'_run','Run service',f'Run "{caption} service" with inputs ["Request" uses "Caller input"] and outputs ["Result" uses "Caller result"].',[('complete','Caller done')])
        caller.step(key+'_caller_done','Caller done','Finish as "complete".');root.child(caller)
        root.capability(key+'_adapter',caption+' capability','Provider',REQUEST,REQUEST,'Operator',caption+' effects')
        root.policy(key+'_effects','effect',caption+' effects',f'Effect is {effect}. Idempotency is {provider["idempotency"]}. Reconcile under "{caption} observations".')
        endpoint=f'"Observe work" in "Recovery" in "{caption} service" in "{caption} caller" in "Main"'
        root.policy(key+'_wait','wait',caption+' observations',f'Observe {type_source(observation)}. Match when ((the case of (the fact "payload")) equals ("confirmed")). Authority "Operator". Timer none. On observation outcome "observed" of {endpoint}. On timer none.')
        root.step('use_'+key,'Use '+caption,f'Run "{caption} caller" with inputs ["Request" uses "Launch input"] and outputs ["Result" uses "{("First" if key=="a" else "Second")} result"].',[('complete',next_node)])
    root.step('main_done','Main done','Finish as "complete".')
    source='Choreoform authoring "0.16.0".\n'+root.render()+'\n'
    supplied=[(v['id'],v['revision'],raw) for v,raw in [(pp,pr),(tp,tr),(fp,fr)]]
    supplied += [(identity,revision,(ROOT/'docs/ir/contracts'/(revision.replace(':','-')+'.txt')).read_bytes()) for identity,revision in [TERMINAL,FAULTS]]
    return source,p.ledger,supplied
