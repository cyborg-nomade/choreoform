# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite RP-03 model spelling; native Rust supplies all binding/type/IR proofs."""
from collections import Counter
from copy import deepcopy
from .receipt_reuse_evidence import Budget, rebind, conjunction_leaves
from .rp01_source import expression_source, type_source, quote
from .rp03_resources import types, instruction


def sorted_descriptor(value):
    if isinstance(value, dict): return {k: sorted_descriptor(v) for k,v in sorted(value.items())}
    if isinstance(value, list): return [sorted_descriptor(v) for v in value]
    return value


def template_source(ast):
    """The finite pure subset, with no ambient read/fact fallback."""
    op=ast['op']
    if op=='param': return 'the parameter '+quote(ast['name'])
    if op=='literal': return expression_source(ast,'Order types')
    if op=='field': return f'field {quote(ast["name"])} of ({template_source(ast["value"])})'
    if op=='unwrapNamed': return f'the representation of ({template_source(ast["value"])})'
    if op=='not': return f'not ({template_source(ast["value"])})'
    if op in ('and','eq'):
        word='and' if op=='and' else 'equals'
        return f'({template_source(ast["left"])}) {word} ({template_source(ast["right"])})'
    raise ValueError('unsupported finite pure template operation')


class PureReceiptSource:
    def __init__(self, type_pin):
        doc, _, pin = types()
        assert pin == type_pin, 'finite type publication differs'
        self.pin = deepcopy(pin)
        self.types = doc['definitions']
        self.receipt = deepcopy(self.types['Reservation A evidence']['representation'])
        self.instruction = instruction(pin)
        self.templates = {}
        self.cases = Counter()
        self.activities = set()
        self.nominals = set()
        self.budget = Budget()

    def completion(self, identity, ast, operation, request, tags, provider):
        assert identity not in self.activities, 'duplicate finite occurrence'
        assert provider['input'] == self.instruction, 'complete instruction differs'
        assert set(provider['output']['cases']) == set(tags), 'provider outcomes differ'
        self.activities.add(identity)
        uses = []
        case = ast
        seen = set()
        while case['op'] == 'if':
            guard = case['condition']
            assert guard['op'] == 'eq' and guard['left'] == {'op':'tag','value':{'op':'fact','name':'result'}}
            phase = guard['right']
            assert phase['op']=='literal' and phase['type']=={'kind':'text'}
            tag = phase['value']
            assert tag in tags and tag not in seen
            seen.add(tag)
            nominal = provider['output']['cases'][tag]
            assert nominal['kind']=='named' and nominal['contract']==self.pin
            assert self.types[nominal['name']]['representation']==self.receipt, 'complete receipt representation differs'
            self.nominals.add(nominal['name'])
            evidence = {'op':'unwrapNamed','value':{'op':'payload','value':{'op':'fact','name':'result'},'tag':tag}}
            literal = {'op':'literal','type':{'kind':'text'},'value':operation}
            terms = [('receipt',evidence),('instruction',request),('operation',literal),('phase',phase)]
            assert all(a!=b for i,(_,a) in enumerate(terms) for _,b in terms[i+1:]), 'distinct input boundaries collide'
            pairs = [(value,{'op':'param','name':name}) for name,value in terms]
            template = rebind(case['then'],pairs,self.budget)
            assert rebind(template,[(p,v) for v,p in pairs],self.budget)==case['then'], 'complete case reconstruction differs'
            count = len(conjunction_leaves(case['then'],self.budget))
            assert count in (13,14), 'unknown complete balanced shape'
            if count in self.templates: assert self.templates[count]==template, 'complete balanced template differs'
            else: self.templates[count]=template
            self.cases[count] += 1
            name = 'Receipt agrees' if count==13 else 'Reasoned receipt agrees'
            use = f'test {quote(name)} using ["Evidence" is ({expression_source(evidence,"Order types")}), "Instruction" is ({expression_source(request,"Order types")}), "Operation" is ({quote(operation)}), "Phase" is ({quote(tag)})]'
            uses.append((expression_source(guard,'Order types'),use))
            case=case['else']
        assert seen==set(tags)
        assert case=={'op':'literal','type':{'kind':'boolean'},'value':False}, 'false fallback differs'
        result=expression_source(case,'Order types')
        for guard,use in reversed(uses): result=f'if ({guard}) then ({use}) otherwise ({result})'
        return result

    def finish(self, root):
        assert len(self.activities)==25 and len(self.nominals)==23
        assert self.cases==Counter({13:26,14:29}) and set(self.templates)=={13,14}
        p=root.package
        p.declaration('receipt_fields_type','type-alias',root.id,'Receipt fields')
        root.declarations.append('Describe type "Receipt fields" as '+type_source(sorted_descriptor(self.receipt),'Order types')+'.')
        inputs=[('receipt','Evidence'),('instruction','Instruction'),('operation','Operation'),('phase','Phase')]
        for count,identity,name in [(13,'receipt_test','Receipt agrees'),(14,'reasoned_receipt_test','Reasoned receipt agrees')]:
            p.declaration(identity,'predicate',root.id,name)
            for wire,caption in inputs:
                p.ledger['symbols'].append(dict(id=p.identity(identity+'_'+wire),owner=identity,role='predicate-input',name=caption,wire=wire))
            body=rebind(self.templates[count],[({'op':'param','name':wire},{'op':'param','name':caption}) for wire,caption in inputs],self.budget)
            root.declarations.append(f'Define test {quote(name)} with inputs ["Evidence" as type "Receipt fields", "Instruction" as type "Order instruction", "Operation" as text, "Phase" as text]. It holds when ({template_source(body)}). End test.')
        p.ledger.update(profile='0.19.0',version='0.18.0')
