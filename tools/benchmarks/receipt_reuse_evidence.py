# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite read-only RP-03 AST experiment, not a source compiler or evaluator."""
from copy import deepcopy
from hashlib import sha256
import json
from pathlib import Path
import rfc8785

ROOT=Path(__file__).resolve().parents[2]
CHILDREN={'literal':(), 'read':(), 'fact':(), 'param':(), 'field':('value',),
          'unwrapNamed':('value',), 'payload':('value',), 'not':('value',),
          'eq':('left','right'), 'and':('left','right')}


def bounded(path):
    with path.open('rb') as f:raw=f.read(1048577)
    if len(raw)>1048576:raise ValueError('measurement input bound')
    return raw


def digest(value):return sha256(rfc8785.dumps(value)).hexdigest()


class Budget:
    def __init__(self, work=100000):self.remaining=work
    def tick(self, depth):
        self.remaining-=1
        if depth>64 or self.remaining<0:raise ValueError('measurement AST bound')


def rebind(ast, pairs, budget, depth=0):
    budget.tick(depth)
    for original,replacement in pairs:
        if ast==original:return deepcopy(replacement)
    op=ast['op']
    if op not in CHILDREN:raise ValueError('unsupported measurement expression')
    # In particular, literal.value/type and field/payload labels are opaque.
    if not CHILDREN[op]:return deepcopy(ast)
    result=dict(ast)
    for key in CHILDREN[op]:result[key]=rebind(ast[key],pairs,budget,depth+1)
    return result


def conjunction_leaves(ast,budget,depth=0):
    budget.tick(depth)
    if ast['op']=='and':return conjunction_leaves(ast['left'],budget,depth+1)+conjunction_leaves(ast['right'],budget,depth+1)
    return [ast]


def report():
    base=ROOT/'docs/benchmarks/rp03';raw=bounded(base/'candidate.ir.json');doc=json.loads(raw);b=doc['body']
    types_raw=bounded(base/'types.json');type_doc=json.loads(types_raw);types=type_doc['definitions']
    type_pin={'id':type_doc['id'],'revision':'sha256:'+sha256(types_raw).hexdigest()}
    manifest=json.loads(bounded(base/'manifest.json'))
    if 'sha256:'+sha256(raw).hexdigest()!=manifest['canonicalIR']['digest']:raise ValueError('candidate digest')
    budget=Budget();rows=[];templates={};inputs=set();representations=set();nominals=set();invariants=set()
    for activity,n in sorted(b['nodes'].items()):
        if n['kind']!='activity' or n['mode']!='capability':continue
        policy=n['policy'];completion=b['policies'][policy]['body']['completion'];case=completion
        capability=b['capabilities'][n['target']]
        request=b['expressions'][n['input']]['body'];assert request['op']=='read'
        inputs.add(digest(capability['input']))
        cases=[]
        while case['op']=='if':
            guard=case['condition'];assert guard['op']=='eq' and guard['left']=={'op':'tag','value':{'op':'fact','name':'result'}}
            phase=guard['right'];assert phase['op']=='literal' and phase['type']=={'kind':'text'}
            tag=phase['value'];assert tag in n['outcomes']
            named=capability['output']['body']['cases'][tag];assert named['kind']=='named'
            assert named['contract']==type_pin
            nominals.add(named['name']);invariants.add(digest(types[named['name']]['invariant']));representation=types[named['name']]['representation'];representations.add(digest(representation))
            evidence={'op':'unwrapNamed','value':{'op':'payload','value':{'op':'fact','name':'result'},'tag':tag}}
            leaves=conjunction_leaves(case['then'],budget);first=leaves[0]
            assert first['op']=='eq' and first['left']=={'op':'field','name':'operation','value':evidence}
            operation=first['right'];assert operation['op']=='literal' and operation['type']=={'kind':'text'}
            assert len({digest(v) for v in [evidence,request,operation,phase]})==4
            pairs=[(value,{'op':'param','name':name}) for name,value in [('receipt',evidence),('instruction',request),('operation',operation),('phase',phase)]]
            template=rebind(case['then'],pairs,budget)
            restored=rebind(template,[(param,value) for value,param in pairs],budget)
            if restored!=case['then']:raise ValueError('exact AST reconstruction failed')
            signature=digest(template);templates[signature]=template
            cases.append({'tag':tag,'operation':operation['value'],'nominal':named,'representationDigest':digest(representation),'template':signature,'comparisonLeaves':len(leaves),'originalAstBytes':len(rfc8785.dumps(case['then']))})
            case=case['else']
        assert case=={'op':'literal','type':{'kind':'boolean'},'value':False}
        assert {c['tag'] for c in cases}==set(n['outcomes'])
        rows.append({'activity':activity,'policy':policy,'capability':n['target'],'request':request['cell'],
                     'completionDigest':digest(completion),'cases':cases})
    return {'interpretation':'finite exact AST reconstruction only; no compiler, future byte savings, runtime or user evidence',
            'candidateDigest':sha256(raw).hexdigest(),'revision':doc['revision'],'typesDigest':sha256(types_raw).hexdigest(),
            'capabilityActivities':len(rows),'distinctRequestInputs':len(inputs),'distinctNominalResults':len(nominals),
            'distinctEvidenceRepresentations':len(representations),'distinctResultInvariants':len(invariants),'cases':sum(len(r['cases']) for r in rows),
            'templates':{key:{'digest':digest(value),'astBytes':len(rfc8785.dumps(value))} for key,value in sorted(templates.items())},'rows':rows}


if __name__=='__main__':
    raw=(json.dumps(report(),ensure_ascii=False,indent=2)+'\n').encode()
    if len(raw)>65536:raise ValueError('measurement output bound')
    print(raw.decode(),end='')
