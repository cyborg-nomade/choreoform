# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""RP-01 fixture assembly with explicit stable IDs, not a general IR exporter."""
import json

from .rp01_resources import TEXT, BOOLEAN, encoded

SEMANTICS = {'contract': 'urn:choreoform:semantics:adr-0008', 'digest': 'sha256:0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141'}
CORE = {'contract': 'urn:choreoform:contracts:core-profile:0.2.0', 'digest': 'sha256:5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623', 'dialect': 'core'}

CORE3 = {'contract': 'urn:choreoform:contracts:core-profile:0.3.0', 'digest': 'sha256:30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3', 'dialect': 'core'}


def quote(value):
    assert not any(ord(c) < 32 for c in value)
    return json.dumps(value, ensure_ascii=False)


def type_source(ty):
    kind = ty['kind']
    if kind == 'text':
        return 'text'
    if kind == 'boolean':
        return 'truth value'
    if kind == 'integer':
        return 'whole number'
    if kind == 'decimal':
        return f'decimal with {ty["scale"]} places'
    if kind == 'named':
        return f'type {quote(ty["name"])} from "Expense types"'
    if kind == 'option':
        return 'optional ' + type_source(ty['element'])
    if kind == 'list':
        return 'list of ' + type_source(ty['element'])
    if kind == 'record':
        fields = ', '.join(f'{quote(k)} as {type_source(v)}' for k, v in ty['fields'].items())
        return 'record with ' + (f'[{fields}]' if fields else 'empty')
    if kind == 'variant':
        return 'choice with [' + ', '.join(f'{quote(k)} as {type_source(v)}' for k, v in ty['cases'].items()) + ']'
    raise ValueError(kind)


def expression_source(expr):
    op = expr['op']
    unary = {
        'not': lambda value: f'not ({value})',
        'tag': lambda value: f'the case of ({value})',
        'unwrapNamed': lambda value: f'the representation of ({value})',
        'some': lambda value: f'some ({value})',
        'isSome': lambda value: f'a value exists in ({value})',
        'unwrap': lambda value: f'the present value of ({value})',
        'length': lambda value: f'the length of ({value})',
    }
    if op == 'literal':
        ty, value = expr['type'], expr['value']
        if ty == BOOLEAN:
            return 'true' if value else 'false'
        if ty == TEXT:
            return quote(value)
        if ty['kind'] == 'integer':
            return value
        raise ValueError('fixture literal requires an explicit supported encoding')
    if op == 'read':
        return 'the value of ' + quote(expr['cell'])
    if op == 'fact':
        return 'the fact ' + quote(expr['name'])
    if op == 'field':
        return f'field {quote(expr["name"])} of ({expression_source(expr["value"])})'
    if op in unary:
        return unary[op](expression_source(expr['value']))
    if op == 'payload':
        return f'the payload of ({expression_source(expr["value"])}) for {quote(expr["tag"])}'
    if op == 'none':
        return 'no value of ' + type_source(expr['element'])
    if op == 'record':
        pairs = ', '.join(f'{quote(k)} is ({expression_source(v)})' for k, v in expr['fields'].items())
        return 'record with ' + (f'[{pairs}]' if pairs else 'empty')
    if op == 'if':
        return f'if ({expression_source(expr["condition"])}) then ({expression_source(expr["then"])}) otherwise ({expression_source(expr["else"])})'
    if op == 'contains':
        return f'({expression_source(expr["list"])}) contains ({expression_source(expr["value"])})'
    if op == 'list':
        items = ', '.join(f'({expression_source(v)})' for v in expr['items'])
        return f'list of {type_source(expr["element"])} containing ' + (f'[{items}]' if items else 'empty')
    operators = {'and': 'and', 'or': 'or', 'eq': 'equals', 'lt': 'is less than', 'add': 'plus', 'sub': 'minus'}
    if op in operators:
        return f'({expression_source(expr["left"])}) {operators[op]} ({expression_source(expr["right"])})'
    raise ValueError(op)


def names(items):
    return '[' + ', '.join(quote(x) for x in items) + ']' if items else 'empty'


class Package:
    def __init__(self):
        self.ledger = {
            'format': 'choreoform-authoring-bindings', 'version': '0.3.0', 'profile': '0.4.0',
            'definition': 'rp01_benchmark', 'root': 'expense',
            'contracts': {'semantics': SEMANTICS, 'core': CORE},
            'declarations': [], 'generated': [], 'symbols': [], 'retired': [],
            'resources': [], 'annotations': {'benchmark': 'RP-01', 'synthetic': True},
        }
        self.used = set()
        self.actions = {}

    def identity(self, identity):
        assert identity not in self.used, identity
        assert len(identity) <= 64 and identity[0].isascii() and identity[0].isalpha()
        assert all(c.isascii() and (c.isalnum() or c in '_-') for c in identity)
        self.used.add(identity)
        return identity

    def declaration(self, identity, kind, scope, name):
        self.ledger['declarations'].append({'id': self.identity(identity), 'kind': kind, 'scope': scope, 'name': name})

    def symbol(self, owner, role, wire, name=None):
        identity = self.identity(f'{owner}_{role}_{wire}')
        self.ledger['symbols'].append({'id': identity, 'owner': owner, 'role': role, 'name': name or wire, 'wire': wire})
        return identity

    def generated(self, identity, owner, role, key=None):
        self.ledger['generated'].append({'id': self.identity(identity), 'owner': owner, 'role': role, 'key': key})

    def resource(self, identity, name, kind, pin):
        self.declaration(identity, 'resource', 'expense', name)
        self.ledger['resources'].append({'binding': identity, 'kind': kind, 'contract': pin['id'], 'digest': pin['revision']})

    def companion(self):
        return encoded(self.ledger)


class Scope:
    def __init__(self, package, identity, name, parent, entry, inputs=(), outputs=(), outcomes=('complete',), closure='Ordinary close', race='Acceptance order', cancellation='Stop work'):
        self.package, self.id, self.name, self.parent = package, identity, name, parent
        self.entry, self.inputs, self.outputs, self.outcomes = entry, inputs, outputs, outcomes
        self.closure, self.race, self.cancellation = closure, race, cancellation
        self.declarations = []
        package.declaration(identity, 'scope', parent.id if parent else None, name)
        for role, ports in [('input', inputs), ('output', outputs)]:
            for wire, display, _ in ports:
                package.symbol(identity, role, wire, display)
        for outcome in outcomes:
            package.symbol(identity, 'outcome', outcome)

    def ports(self, ports):
        return '[' + ', '.join(f'{quote(display)} uses {quote(cell)}' for _, display, cell in ports) + ']' if ports else 'empty'

    def render(self):
        start = f'Scope {quote(self.name)} within {quote(self.parent.name)}.' if self.parent else f'Process {quote(self.name)}.'
        body = '\n'.join(self.declarations)
        return f'''{start}
Start at {quote(self.entry)}.
Inputs {self.ports(self.inputs)}. Outputs {self.ports(self.outputs)}. Outcomes {names(self.outcomes)}.
Use cancellation {quote(self.cancellation)}, faults "Unhandled faults", closure {quote(self.closure)} and races {quote(self.race)}.
{body}
End {'scope' if self.parent else 'process'}.'''

    def policy(self, identity, kind, name, sentences):
        self.package.declaration(identity, 'policy', self.id, name)
        self.declarations.append(f'Rule for {kind} {quote(name)}. {sentences} End policy.')

    def actor(self, identity, name, requirement):
        self.package.declaration(identity, 'actor', self.id, name)
        self.declarations.append(f'Require actor {quote(name)} under {quote(requirement)}.')

    def data(self, identity, name, ty, initial=None, invalidates=(), participants=(), capabilities=(), policy='Expense access'):
        self.package.declaration(identity, 'data', self.id, name)
        self.package.symbol(identity, 'purpose', 'reimbursement', 'Reimbursement')
        if initial is not None:
            self.package.generated(f'{identity}_initial', identity, 'initial')
        value = '(' + expression_source(initial) + ')' if initial is not None else 'unavailable'
        invalidation = '[' + ', '.join(invalidates) + ']' if invalidates else 'empty'
        self.declarations.append(f'''Keep {quote(name)} as {type_source(ty)} under {quote(policy)}.
Sensitivity "expense". Purposes ["Reimbursement"]. Participants {names(participants)}.
Capabilities {names(capabilities)}. Invalidate {invalidation}. Initially {value}. End data.''')

    def expression(self, identity, name, ty, expr):
        self.package.declaration(identity, 'expression', self.id, name)
        self.declarations.append(f'Let {quote(name)} mean ({expression_source(expr)}) as {type_source(ty)} with parameters empty.')

    def capability(self, identity, name, provider, input_type, output_type, authority, effects):
        self.package.declaration(identity, 'capability', self.id, name)
        self.package.generated(f'{identity}_provider', identity, 'provider-policy')
        self.declarations.append(f'Use capability {quote(name)} from {quote(provider)} with input {type_source(input_type)} and output {type_source(output_type)}, authority {quote(authority)} and effects {quote(effects)}.')

    def step(self, identity, name, action, successors=(), assignments=()):
        self.package.declaration(identity, 'node', self.id, name)
        self.package.actions[identity] = (self.id, name, action)
        for cell_id in assignments:
            self.package.generated(f'{identity}_set_{cell_id}', identity, 'assignment', cell_id)
        flows = []
        for outcome, target in successors:
            symbol = self.package.symbol(identity, 'outcome', outcome)
            self.package.generated(f'{identity}_flow_{outcome}', identity, 'flow', symbol)
            flows.append(f'On {quote(outcome)}, continue with {quote(target)}.')
        self.declarations.append(f'Step {quote(name)}. {action} ' + ' '.join(flows) + ' End step.')

    def compute(self, identity, name, assignments, target):
        # Caller supplies stable cell IDs as well as readable names explicitly.
        pairs = ', '.join(f'{quote(display)} to ({expression_source(expr)})' for _, display, expr in assignments)
        self.step(identity, name, f'Set together [{pairs}].', [('next', target)], [cell for cell, _, _ in assignments])

    def child(self, scope):
        self.declarations.append(scope.render())
