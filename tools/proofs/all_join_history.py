# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Finite complete source witnesses; no host outcomes, values or permissions."""
from benchmarks.rp01_source import Package, Scope, quote
from benchmarks.rp01_resources import INTEGER, BOOLEAN, literal
from check_compensation_authoring import CORE


def package(predicate='all', *, writer=False):
    p = Package(definition='fixed_join_history', root='main', profile='0.16.0',
                version='0.15.0', core=CORE, purpose=('test', 'Test'),
                protection_policy='Protection', sensitivity='test')
    p.ledger['annotations'] = {'synthetic':True, 'meaning':'conditional history declarations only'}
    def scope(identity, name, parent, entry, outcomes=('complete',)):
        return Scope(p, identity, name, parent, entry, outcomes=outcomes,
                     cancellation='Cancel', closure='Close', race='Race')
    root = scope('main', 'Main', None, 'Start')
    root.actor('operator', 'Operator', 'Authority')
    root.policy('authority', 'actor', 'Authority', 'Permit when (true).')
    root.policy('protection', 'protection', 'Protection', 'Permit access when ((the fact "purpose") equals ("test")).')
    root.policy('cancel', 'cancel', 'Cancel', 'Authority "Operator". For children use "Settlement".')
    root.policy('settlement', 'settlement', 'Settlement', 'Unfinished work cancel.')
    root.policy('close', 'closure', 'Close', 'Fully terminal.')
    root.policy('faults', 'faults', 'Unhandled faults', 'Handle empty.')
    root.policy('race', 'race', 'Race', 'Accept in acceptance order.')
    root.data('basis', 'Basis', INTEGER, literal(INTEGER, '0'), participants=['Operator'], invalidates=[quote('After')])
    root.expression('ready', 'After ready', BOOLEAN, {'op':'eq', 'left':{'op':'read','cell':'Basis'}, 'right':literal(INTEGER,'0')})
    for key in ['a', 'b']:
        child = scope(key, key.upper(), root, 'Check local' if key=='a' else 'Done B', [key])
        if key=='a':
            child.data('local', 'Local', INTEGER, literal(INTEGER,'0'), participants=['Operator'], invalidates=[quote('Check local')])
            child.expression('local_ready', 'Local ready', BOOLEAN, {'op':'eq', 'left':{'op':'read','cell':'Local'}, 'right':literal(INTEGER,'0')})
            child.step('check_local', 'Check local', 'Choose by ["yes" when "Local ready"]; otherwise "no".', [('yes','Done A'), ('no','Done A')])
            if writer:
                unused = scope('writer', 'Writer', child, 'Write local')
                unused.compute('mutate', 'Write local', [('local','Local',literal(INTEGER,'0'))], 'Writer done')
                unused.step('writer_done', 'Writer done', 'Finish as "complete".')
                child.child(unused)
        child.step(key+'_done', 'Done '+key.upper(), 'Finish as '+quote(key)+'.')
        root.child(child)
        p.symbol('start', 'branch', key)
    root.step('start', 'Start', 'Start together ["a" runs "A", "b" runs "B"] and gather at "Gather".')
    outcomes='[outcome "a" of "A", outcome "b" of "B"]'
    condition=predicate+' report '+outcomes
    if predicate=='composite':condition='both (all report '+outcomes+') and (all report '+outcomes+')'
    root.step('gather', 'Gather', 'Gather from "Start" when '+condition+'; for unfinished work use "Settlement".', [('gathered','After')])
    root.step('after', 'After', 'Choose by ["yes" when "After ready"]; otherwise "no".', [('yes','Done'), ('no','Done')])
    root.step('done', 'Done', 'Finish as "complete".')
    return 'Choreoform authoring "0.16.0".\n'+root.render()+'\n', p.ledger, []
