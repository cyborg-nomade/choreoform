# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite two-site expense-model spelling, not a source compiler or general exporter."""
from copy import deepcopy
from hashlib import sha256
import rfc8785
from .rp01_source import expression_source,type_source,quote
from .rp01_capture import classification,CAPTURE,REQUEST,CATEGORY,CLASSIFY
from .rp01_factored_source import CORE12

TEMPLATE='7efefb1be344fd9cf19db15e2ba9cc84906ab1d6c467542747a962885eaa39ab'
INSTRUCTION='3317d8300293637f720c1b840ceacf6ce29a7f312a32615773f9582606d95c53'
RESULT='ee991749e845459bc0893611f5b9af467bced514ad1637d91793cec5fe4bab7e'
CHILDREN={'literal':(), 'read':(), 'fact':(), 'param':(), 'field':('value',),
          'unwrapNamed':('value',), 'payload':('value',), 'tag':('value',),
          'eq':('left','right'), 'and':('left','right'), 'if':('condition','then','else')}
ATTRS={'literal':('type','value'),'read':('cell',),'fact':('name',),'param':('name',),
       'field':('name',),'payload':('tag',)}


class Budget:
    def __init__(self):self.work=0
    def tick(self,depth):
        self.work+=1
        if depth>64 or self.work>100000:raise ValueError('finite receipt model bound')


def digest(value):return sha256(rfc8785.dumps(value)).hexdigest()


def bind(ast,pairs,budget,depth=0):
    budget.tick(depth)
    for old,new in pairs:
        if ast==old:return deepcopy(new)
    op=ast.get('op')
    if op not in CHILDREN:raise ValueError('unsupported finite receipt operation')
    if set(ast)!={'op',*CHILDREN[op],*ATTRS.get(op,())}:raise ValueError('finite receipt fields differ')
    if not CHILDREN[op]:return deepcopy(ast)
    result=dict(ast)
    for key in CHILDREN[op]:result[key]=bind(ast[key],pairs,budget,depth+1)
    return result


def sorted_descriptor(value):
    if isinstance(value,dict):return {k:sorted_descriptor(v) for k,v in sorted(value.items())}
    if isinstance(value,list):return [sorted_descriptor(v) for v in value]
    return value


def captioned(ast,captions,budget,depth=0):
    budget.tick(depth)
    op=ast.get('op')
    if op not in CHILDREN or set(ast)!={'op',*CHILDREN[op],*ATTRS.get(op,())}:
        raise ValueError('finite receipt fields differ')
    if op=='read':
        if ast['cell'] not in captions:raise ValueError('missing finite receipt caption')
        return {'op':'read','cell':captions[ast['cell']]}
    if not CHILDREN[op]:return deepcopy(ast)
    result=dict(ast)
    for key in CHILDREN[op]:result[key]=captioned(ast[key],captions,budget,depth+1)
    return result


def template_source(ast):
    op=ast['op'];child=lambda key:template_source(ast[key])
    if op=='param':return 'the parameter '+quote({'result':'Result','request':'Instruction'}[ast['name']])
    if op=='literal':return expression_source(ast)
    if op=='field':return f'field {quote(ast["name"])} of ({child("value")})'
    if op=='unwrapNamed':return f'the representation of ({child("value")})'
    if op=='tag':return f'the case of ({child("value")})'
    if op=='payload':return f'the payload of ({child("value")}) for {quote(ast["tag"])}'
    if op in ('eq','and'):
        word='equals' if op=='eq' else 'and'
        return f'({child("left")}) {word} ({child("right")})'
    if op=='if':return f'if ({child("condition")}) then ({child("then")}) otherwise ({child("else")})'
    raise ValueError('ambient or unsupported finite pure receipt operation')


def replace_clause(source,prefix,old,new):
    lines=source.splitlines(keepends=True)
    indices=[i for i,line in enumerate(lines) if line.startswith(prefix)]
    if len(indices)!=1 or lines[indices[0]].count(old)!=1:raise ValueError('finite receipt source clause differs')
    lines[indices[0]]=lines[indices[0]].replace(old,new,1)
    return ''.join(lines)


def adopt(source,ledger,body,*,factoring):
    assert (ledger['profile'],ledger['version'],ledger['root'])==('0.18.0','0.17.0','expense')
    assert ledger['contracts']['core']==CORE12 and factoring is not None
    capabilities=body['capabilities']['payment_capability']
    instruction=capabilities['input']['body'];result_type=capabilities['output']['body']
    assert digest(instruction)==INSTRUCTION and digest(result_type)==RESULT, 'complete receipt input descriptors differ'
    assert instruction==factoring.instruction, 'complete payable instruction differs'
    completion=body['policies']['payment_work']['body']['completion']
    pairs=[({'op':'fact','name':'result'},{'op':'param','name':'result'}),
           ({'op':'read','cell':'payment_request'},{'op':'param','name':'request'})]
    budget=Budget();template=bind(completion,pairs,budget)
    assert digest(template)==TEMPLATE, 'complete receipt template differs'
    assert bind(template,[(new,old) for old,new in pairs],budget)==completion
    captions={r['id']:r['name'] for r in ledger['declarations']}
    readable=lambda ast:expression_source(captioned(ast,captions,budget))
    work_use='test "Payment receipt agrees" using ["Result" is (the fact "result"), "Instruction" is (the value of '+quote(captions['payment_request'])+')]'
    source=replace_clause(source,'Rule for work '+quote(captions['payment_work'])+'.',
        'Complete when ('+readable(completion)+'). Compensates none.',
        'Complete when ('+work_use+'). Compensates none.')
    classified=classification(completion)
    inner=classified['then']
    result_argument={'op':'payload','value':{'op':'read','cell':CAPTURE},'tag':'confirmed'}
    request_argument={'op':'read','cell':REQUEST}
    assert bind(template,[({'op':'param','name':'result'},result_argument),({'op':'param','name':'request'},request_argument)],budget)==inner['condition']
    late_use='test "Payment receipt agrees" using ["Result" is ('+readable(result_argument)+'), "Instruction" is ('+readable(request_argument)+')]'
    classified_source=f'if ({readable(classified["condition"])}) then (if ({late_use}) then ({readable(inner["then"])}) otherwise ({readable(inner["else"])})) otherwise ({readable(classified["else"])})'
    old='Set together ['+quote(captions[CATEGORY])+' to ('+readable(classified)+')].'
    new='Set together ['+quote(captions[CATEGORY])+' to ('+classified_source+')].'
    source=replace_clause(source,'Step '+quote(captions[CLASSIFY])+'.',old,new)
    identity='payment_receipt_test'
    ids={identity,identity+'_result',identity+'_request'}
    used={r['id'] for key in ['declarations','symbols','generated'] for r in ledger[key]}|set(ledger['retired'])|{ledger['definition']}
    assert not ids & used, 'finite receipt managed identity collision'
    ledger=deepcopy(ledger)
    ledger['declarations'].append(dict(id=identity,kind='predicate',scope=ledger['root'],name='Payment receipt agrees'))
    for wire,caption in [('result','Result'),('request','Instruction')]:
        ledger['symbols'].append(dict(id=identity+'_'+wire,owner=identity,role='predicate-input',name=caption,wire=wire))
    definition='Define test "Payment receipt agrees" with inputs ["Result" as '+type_source(sorted_descriptor(result_type))+', "Instruction" as type "Payable instruction"]. It holds when ('+template_source(template)+'). End test.\n'
    assert source.count('End process.')==source.count('authoring "0.18.0"')==1
    source=source.replace('End process.',definition+'End process.',1).replace('authoring "0.18.0"','authoring "0.19.0"',1)
    ledger.update(profile='0.19.0',version='0.18.0')
    return source,ledger
