# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite expense fixture spellings; Rust remains the source compiler."""
from copy import deepcopy
from .rp01_source import Scope as InlineScope, names, quote, type_source, expression_source

CORE12={'contract':'urn:choreoform:contracts:core-profile:0.12.0','digest':'sha256:73158731c33084fde009abcc86530926447902609fda9535f04daa7c768c2007','dialect':'core'}


def recipient_set(values):
    values=tuple(values)
    unique=frozenset(values)
    if len(unique)!=len(values):raise ValueError('duplicate fixture protection recipient')
    return unique


class Names:
    def __init__(self, package, participants, capabilities, instruction):
        self.package=package
        self.envelope=(package.protection_policy,package.sensitivity,tuple(package.purpose),recipient_set(participants),recipient_set(capabilities))
        self.instruction=deepcopy(instruction)

    def type_text(self, ty):
        return 'type "Payable instruction"' if ty==self.instruction else type_source(ty,self.package.type_name)

    def data_text(self, name, ty, initial=None, invalidates=(), participants=(), capabilities=(), policy=None):
        p=self.package
        requested=(p.protection_policy if policy is None else policy,p.sensitivity,tuple(p.purpose),recipient_set(participants),recipient_set(capabilities))
        if requested!=self.envelope:raise ValueError('fixture protection differs from its explicit definition')
        value='('+expression_source(initial,p.type_name)+')' if initial is not None else 'unavailable'
        invalidation='['+', '.join(invalidates)+']' if invalidates else 'empty'
        return f'Keep {quote(name)} as {self.type_text(ty)}, protected as "Expense information". Invalidate {invalidation}. Initially {value}. End data.'


class Scope(InlineScope):
    def define(self, participants, capabilities, instruction):
        p=self.package
        assert self.parent is None and not hasattr(p,'factoring')
        p.factoring=Names(p,participants,capabilities,instruction)
        p.declaration('expense_data_protection','data-protection',self.id,'Expense information')
        p.symbol('expense_data_protection','purpose',*p.purpose)
        self.declarations.append(f'Protect data as "Expense information". Under {quote(p.protection_policy)}. Sensitivity {quote(p.sensitivity)}. Purposes [{quote(p.purpose[1])}]. Participants {names(participants)}. Capabilities {names(capabilities)}. End protection.')
        p.declaration('payable_instruction_type','type-alias',self.id,'Payable instruction')
        self.declarations.append(f'Describe type "Payable instruction" as {type_source(instruction,p.type_name)}.')

    def data(self, identity, name, ty, initial=None, invalidates=(), participants=(), capabilities=(), policy=None):
        p=self.package
        text=p.factoring.data_text(name,ty,initial,invalidates,participants,capabilities,policy)
        p.declaration(identity,'data',self.id,name);p.symbol(identity,'purpose',*p.purpose)
        if initial is not None:p.generated(f'{identity}_initial',identity,'initial')
        self.declarations.append(text)

    def expression(self, identity, name, ty, expr):
        p=self.package;p.declaration(identity,'expression',self.id,name)
        self.declarations.append(f'Let {quote(name)} mean ({expression_source(expr,p.type_name)}) as {p.factoring.type_text(ty)} with parameters empty.')

    def capability(self, identity, name, provider, input_type, output_type, authority, effects):
        p=self.package;p.declaration(identity,'capability',self.id,name);p.generated(f'{identity}_provider',identity,'provider-policy')
        self.declarations.append(f'Use capability {quote(name)} from {quote(provider)} with input {p.factoring.type_text(input_type)} and output {p.factoring.type_text(output_type)}, authority {quote(authority)} and effects {quote(effects)}.')
