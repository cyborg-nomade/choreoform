# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Generate the complete bounded RP-03 source draft and actual admission refusals."""
import argparse
from hashlib import sha256
import json
from pathlib import Path
import re

import rfc8785
from authoring_probe import lex, validate_ledger, package_digests
from benchmarks.rp01_resources import encoded,pin
from benchmarks.rp03_resources import types,providers,clock_resources,inputs,SPECS
from benchmarks.rp03_source import TERMINAL_PROFILE, matched_completion, read
from benchmarks.rp03_graph import build
from benchmarks.rp03_invalidation import apply_missing
from benchmarks.rp03_instance_checkpoint import artifacts as instance_checkpoint
from benchmarks.rp03_factoring_checkpoint import artifacts as factoring_checkpoint
from build_rp03_checkpoint import package_artifacts as checkpoint_artifacts, artifacts as checkpoint_files, PRE_INSTANCE_README
from check_metadata_authoring import invoke
from build_rp01_benchmark import STAGES

ROOT=Path(__file__).resolve().parents[1]
BASE=ROOT/'docs/benchmarks/rp03'
LICENSE=b'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n'


def package_artifacts(*,historical=False,pre_instances=False):
    if historical or pre_instances:
        return checkpoint_artifacts(historical=historical)
    recipients=[key+'_capability' for key,*_ in SPECS]+['cancel_release_a_capability','cancel_release_b_capability']
    _,raw,tp=types();files={'types.json':raw,'launch-values.json':encoded(inputs())}
    docs=providers(tp,capabilities=recipients);clocks=clock_resources(capabilities=recipients);pins={k:v[2] for k,v in docs.items()}
    registry=[]
    def resource(name,raw,p):
        files[name]=raw
        registry.append({'id':p['id'],'revision':p['revision'],'path':'docs/benchmarks/rp03/'+name})
    resource('types.json',raw,tp)
    for key,(doc,raw,p) in docs.items():
        resource(key.replace('_','-')+'.provider.json',raw,p)
        source={'kind':'provider-terminal-observation-source','version':'0.1.0',
            'id':f'urn:choreoform:benchmarks:rp03:{key.replace("_","-")}-terminal:0.1.0','profile':TERMINAL_PROFILE,'provider':p}
        raw=encoded(source);sp=pin(source['id'],raw);pins[key+'_terminal']=sp
        resource(key.replace('_','-')+'.terminal-source.json',raw,sp)
    for key,(doc,raw,p) in clocks.items():
        resource(key+'.json',raw,p);pins[key]=p
    for profile in [TERMINAL_PROFILE,clocks['clock'][0]['profile']]:
        registry.append({'id':profile['id'],'revision':profile['revision'],'path':'docs/ir/contracts/'+profile['revision'].replace(':','-')+'.txt'})
    source,package=build(tp,{k:v[0] for k,v in docs.items()},pins,historical=historical,factored=not historical)
    validate_ledger(package.ledger)
    files['source.choreo']=source.encode();files['bindings.json']=package.companion()
    files['receipt-rules.json']=encoded({'premise':'Emitted pure declaration ASTs, not compiled full source, accepted observations or process traces.', 'rules':{key:matched_completion(key,read('Instruction'),tags) for key,_,_,tags,_ in SPECS}})
    files['receipt-cases.json']=encoded(receipt_cases())
    files['guard-rules.json']=encoded({'premise':'Pure emitted guard ASTs, not parsed full source or a process trace.', 'types':package.guard_types, 'rules':package.guard_rules})
    files['guard-cases.json']=encoded(guard_cases())
    supplied=[(r['id'],r['revision'],files[r['path'].removeprefix('docs/benchmarks/rp03/')] if r['path'].startswith('docs/benchmarks/rp03/') else (ROOT/r['path']).read_bytes()) for r in registry]
    initial=invoke((source,package.ledger,supplied))
    if initial.returncode!=0:raise ValueError('instance source must lower before invalidation adoption')
    initial_candidate=json.loads(initial.stdout)
    advisory=invoke((source,package.ledger,supplied),'invalidation-requirements')
    if advisory.returncode!=0:raise ValueError('no actual advisory for invalidation adoption: '+advisory.stderr.decode())
    advice=json.loads(advisory.stdout)
    source=apply_missing(source,package.ledger,initial_candidate['body'],advice['missingInvalidationObligations'])
    files['source.choreo']=source.encode()
    result=invoke((source,package.ledger,supplied))
    manifest={'format':'choreoform-rp03-benchmark-draft','version':'0.1.0','status':'source-draft-with-actual-lowering-result','executable':False,
        'sourceProfile':package.ledger['profile'],'companionVersion':package.ledger['version'],'coreProfile':'0.11.0' if historical else '0.12.0','intendedWireVersion':'0.4.0',
        'sourceBytes':len(files['source.choreo']),'measuredTokens':sum(len(lex(line)) for line in source.splitlines()),
        'tokenMeasurement':'sum of complete-line lexing; not whole-program recognition' if historical else 'sum of complete-line lexing; whole-program native recognition verified separately',
        **{k+'Digest':v for k,v in package_digests(files['source.choreo'],files['bindings.json']).items()},
        'contracts':package.ledger['contracts'],'resources':registry,
        'inputArtifacts':[{'path':name,'digest':'sha256:'+sha256(files[name]).hexdigest(),'meaning':'synthetic input or pure declaration evidence, never an accepted event'} for name in ['launch-values.json','receipt-rules.json','receipt-cases.json','guard-rules.json','guard-cases.json']],
        'invalidationAdoption':{'basisRevision':initial_candidate['revision'],
            'requiredCount':len(advice['requiredInvalidationObligations']),
            'added':advice['missingInvalidationObligations'],
            'meaning':'actual advisory-derived declarations only; no host authority or unvisited-context proof'},
        'instanceMap':package.instances,
        'protectionClaimDelta':{'addedCapabilities':['cancel_release_a_capability','cancel_release_b_capability'],
            'meaning':'fresh fictional provider/clock recipient claims, not actual consent or authority'},
        'lowering':None,'canonicalIR':None,'semanticStageResults':[],
        'declarationCounts':{k:len(package.ledger[k]) for k in ['declarations','symbols','generated','resources']},
        'scenarios':[
            {'id':'RP-03-A','status':'Partial','declarations':['start_acceptance','start_acceptance_gather','start_fulfillment','capture_guard','capture_call','start_delivery','record_dispositions','check_exception']},
            {'id':'RP-03-B','status':'Partial','declarations':['check_stock','check_partial','partial_choice','run_release','normal_void_call','normal_release_a_call','normal_release_b_call','record_releases']},
            {'id':'RP-03-C','status':'Partial','declarations':['customer_cancel','order_commits','order_settlement','settlement_direction_ask','stop_a_call','stop_b_call','intercept_a_call','intercept_b_call','refund_guard','refund_call','surviving_exception_ask']},
            {'id':'RP-03-D','status':'Partial','declarations':['capture_activity','capture_wait','capture_observe','capture_retained','capture_capture'],'boundary':'Stable provider/effect/event/request identity and monotonicity remain real host obligations; no live deduplication ledger or suppressed transition is claimed.'},
        ],
        'hostRequirements':[
            'authenticated-current-role-assignment-input-and-business-revisions',
            'exact-stable-instance-occurrence-effect-request-and-provider-event-identities',
            'truthful-reservation-authorization-capture-void-refund-warehouse-carrier-notice-contracts',
            'registered-instance-clock-domain-expiry-evidence-current-dispatch-authority-and-atomic-timers',
            'no-overselling-and-atomic-stock-ownership-outside-the-local-definition',
            'physical-provider-cutoff-and-positive-stop-interception-evidence',
            'capture-ceiling-receipt-and-accountable-refund-business-eligibility',
            'actual-duplicate-terminal-monotonicity-and-contradictory-evidence-ledger',
            'accepted-immutable-original-input-evidence-and-provenance-storage',
            'authorized-receiver-acceptance-atomic-handoff-and-all-transitive-settlement',
            'explicit-accountable-unresolved-and-partial-authorization-balance-ownership',
        ]}
    if result.returncode==0:
        candidate=json.loads(result.stdout);files['candidate.ir.json']=rfc8785.dumps(candidate)
        manifest['lowering']={'result':'passed'}
        manifest['canonicalIR']={'path':'candidate.ir.json','revision':candidate['revision'],'digest':'sha256:'+sha256(files['candidate.ir.json']).hexdigest()}
        for stage in STAGES:
            report=invoke((source,package.ledger,supplied),stage)
            if report.returncode!=0:
                manifest['semanticStageResults'].append({'stage':stage,'result':'refused','diagnostic':report.stderr.decode().strip()})
            else:
                doc=json.loads(report.stdout)
                if doc['executable'] is not False or doc['revision']!=candidate['revision']:raise ValueError('unexpected authority/revision')
                name='reports/'+stage+'.json';files[name]=encoded(doc)
                manifest['semanticStageResults'].append({'stage':stage,'result':'passed','report':name,'reportDigest':'sha256:'+sha256(files[name]).hexdigest()})
    else:
        diagnostic=result.stderr.decode().strip()
        match=re.fullmatch(r'([a-z0-9-]+) at (\d+)\.\.(\d+)',diagnostic)
        if result.returncode!=1 or result.stdout or not match:raise ValueError('unexpected lowering failure: '+diagnostic)
        manifest['lowering']={'result':'refused','code':match[1],'sourceBytes':[int(match[2]),int(match[3])],'diagnostic':diagnostic}
    files['manifest.json']=encoded(manifest)
    for name in list(files):
        if name.endswith(('.json','.txt')):files[name+'.license']=LICENSE
    return files


def historical_artifacts():
    return package_artifacts(historical=True)


def artifacts():
    files=package_artifacts()
    for name,raw in factoring_checkpoint().items():files['history/source-0.16-before-factoring/'+name]=raw
    # Both historical packages reproduce independently with their exact old pins.
    for name,raw in checkpoint_files().items():
        if name.startswith('history/'):
            files[name]=raw
        else:
            files['history/source-0.16-pre-instances/'+name]=raw
    for name,raw in instance_checkpoint().items():files['history/source-0.16-instances/'+name]=raw
    files['history/source-0.16-instances/README.md']=b'''<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Full instance checkpoint before all-join history

Exact generated artifacts and previous README from signed PR #91 head
8e5d1382a0d6964b77a3696046df637544756901, verified as immutable inputs against
recorded SHA-256 pins. Its three passing reports and capture inherited-context
refusal are then-current observations; reproduce that checker at the signed commit.
The current checker can prove a narrower closed all-join subset, so it is not
claimed to reproduce the old semantic refusal. This archive is preserved evidence,
not executable admission, a new provider claim or a re-created process trace.
Historical benchmark-local resources resolve inside this archive.
'''
    files['history/source-0.16-pre-instances/README.previous.md']=PRE_INSTANCE_README
    files['history/source-0.16-pre-instances/README.md']=b'''<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Source-0.16 pre-instance checkpoint

All generated artifacts here preserve the exact PR #90 checkpoint before ADR-0076
caller-local instantiation and explicit synthetic recipient revisions. The source
recognizes then refuses child-template; no candidate or semantic-stage report.
Original manifest paths name the then-current resources. Historical loaders resolve
benchmark-local paths inside their own archive so old pins still select old bytes.
The older source-0.14 archive is also unchanged; its README describes the historical
moment before current recipient revisions. No live effect or authorization follows.
'''
    return files


def receipt_cases():
    from copy import deepcopy
    order=inputs()['order'];cases=[]
    for op,phase in [('reserve_a','reserved'),('capture','captured'),('deliver_a','delivered')]:
        line=order['first line'] if op.endswith('_a') else None
        quantity={'tag':'some','value':line['quantity']} if line else {'tag':'none'}
        warehouse={'tag':'some','value':line['warehouse']} if line else {'tag':'none'}
        request={'order':order,'scope key':line['line ID'] if line else order['order ID'],
            'scope revision':line['revision'] if line else order['revision'],'request revision':op+'-pure-instruction-v1',
            'amount':line['amount'] if line else order['amount'],'currency':order['currency'],
            'basis reference':'fictional-prior-resource','address revision':order['address reference']['revision'],
            'target quantity':quantity,'target warehouse':warehouse}
        value={key:deepcopy(v) for key,v in request.items() if key!='order'}
        value.update(operation=op,phase=phase,**{'order ID':order['order ID'],'order revision':order['revision'],
            'reference':'fictional-command-receipt','subject reference':{'tag':'some','value':'fictional-subject-reference'},
            'provider event':'synthetic-pure-event-not-accepted','expiry':'10000000','reason':'Pure synthetic case'})
        original={'id':op+'-matched','operation':op,'request':request,'result':{'tag':phase,'value':value},'expected':True}
        cases.append(original)
        for key,changed in [('order revision','other-revision'),('request revision','other-instruction'),('amount','11999'),('currency','USD'),('basis reference','other-resource')]+([('target quantity',{'tag':'some','value':'2'}),('target warehouse',{'tag':'some','value':'other-warehouse'})] if line else []):
            case=deepcopy(original)
            case['id']=op+'-wrong-'+key.replace(' ','-');case['result']['value'][key]=changed;case['expected']=False;cases.append(case)
    return {'premise':'Synthetic pure inputs only, never accepted observations, authenticated provider facts or a running order.', 'cases':cases}


def guard_cases():
    from copy import deepcopy
    order=inputs()['order']
    contexts={'Original order':order}
    for _,name,*_ in SPECS:contexts[name+' record']={'tag':'none'}
    def event(op,phase):
        line=order['first line'] if op.endswith('_a') else order['second line'] if op.endswith('_b') else None
        value={'operation':op,'phase':phase,'order ID':order['order ID'],'order revision':order['revision'],
            'scope key':line['line ID'] if line else order['order ID'],'scope revision':line['revision'] if line else order['revision'],
            'request revision':op+'-pure-instruction-v1','amount':line['amount'] if line else order['amount'],'currency':order['currency'],
            'basis reference':'synthetic-prior-resource','reference':'synthetic-command-receipt','provider event':'synthetic-event-not-accepted',
            'address revision':order['address reference']['revision'],'expiry':'10000000','reason':'Pure synthetic guard input',
            'subject reference':{'tag':'some','value':'synthetic-subject'},
            'target quantity':{'tag':'some','value':line['quantity']} if line else {'tag':'none'},
            'target warehouse':{'tag':'some','value':line['warehouse']} if line else {'tag':'none'}}
        return {'tag':'some','value':{'tag':phase,'value':value}}
    cases=[]
    def case(identity,rule,values,expect):cases.append({'id':identity,'rule':rule,'cells':deepcopy(values),'expected':expect})
    values=deepcopy(contexts);values['Reserve first line record']=event('reserve_a','reserved');values['Reserve second line record']=event('reserve_b','unavailable')
    case('default-stock-failure-does-not-offer-partial','check_partial',values,False)
    values['Original order']['delivery choice']='offer partial'
    case('explicit-opt-in-offers-new-customer-choice','check_partial',values,True)
    values=deepcopy(contexts);values['Reserve first line record']=event('reserve_a','reserved')
    case('unknown-stop-does-not-authorize-second-release','cancel_release_a_guard',values,False)
    values['Stop first warehouse work record']=event('stop_a','stopped')
    case('positive-stop-permits-release-proposal','cancel_release_a_guard',values,True)
    values=deepcopy(contexts);values['Capture committed amount record']=event('capture','captured')
    case('capture-alone-proves-no-refund-eligibility','refund_guard',values,False)
    values['Hand over first parcel record']=event('handover_a','handed');values['Hand over second parcel record']=event('handover_b','handed')
    case('unknown-interception-does-not-authorize-refund','refund_guard',values,False)
    values['Request first intercept record']=event('intercept_a','intercepted');values['Request second intercept record']=event('intercept_b','intercepted')
    case('matched-capture-and-positive-intercepts-permit-proposal','refund_guard',values,True)
    values['Capture committed amount record']['value']['value']['amount']='19999'
    case('wrong-capture-total-does-not-authorize-refund','refund_guard',values,False)
    return {'premise':'Synthetic pure values only; None means no confirmed local record, never proof that an effect is absent. No accepted event, dispatch, handoff or process execution.', 'cases':cases}


def write_artifacts(files,base=BASE):
    # Only closed generated output names are cleanup candidates. Preserve unknown
    # files so exact-inventory verification can report them without deleting work.
    stale=[base/'candidate.ir.json',base/'candidate.ir.json.license']
    for stage in STAGES:
        stale.extend([base/'reports'/(stage+'.json'),base/'reports'/(stage+'.json.license')])
    for path in stale:
        if path.exists() and path.relative_to(base).as_posix() not in files:path.unlink()
    for name,raw in files.items():
        target=base/name;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(raw)


def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--check',action='store_true');args=parser.parse_args()
    files=artifacts()
    if args.check:
        changed=[n for n,b in files.items() if not (BASE/n).is_file() or (BASE/n).read_bytes()!=b]
        if changed:raise SystemExit('generated RP-03 files differ: '+', '.join(changed))
    else:write_artifacts(files)
    m=json.loads(files['manifest.json'])
    print(f'RP-03: {m["sourceBytes"]} bytes / {m["measuredTokens"]} complete-line tokens; actual lowering {m["lowering"]}; no admission or execution.')


if __name__=='__main__':main()
