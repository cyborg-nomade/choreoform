# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Closed ordinary provenance witnesses; no execution or host observations."""
import json
from pathlib import Path
import tempfile
import unittest

from benchmarks.rp01_graph import foundations, work, read, PEOPLE
from benchmarks.rp01_source import Package, Scope, CORE3
from benchmarks.rp01_resources import INTEGER, BOOLEAN, record, variant, literal, binary
from check_static_contracts import run


def package(mode='invoke', survivor=None, omit_ancestor=False):
    p = Package()
    p.ledger.update(profile='0.6.0', version='0.5.0', root='root', definition='closed_provenance', annotations={'synthetic': True, 'case': 'closed-provenance'})
    p.ledger['contracts']['core'] = CORE3.copy()
    root = Scope(p, 'root', 'Snapshot example', None, 'Run worker')
    foundations(root)
    root.expression('seed_input', 'Seed input', INTEGER, read('Seed'))
    root.expression('review_input', 'Review input', INTEGER, read('Copy' if mode == 'invoke' else 'Seed'))
    result = variant({'done': record()})
    work(root, 'review_work', 'Review work', 'Payment operator', 'Synthetic declaration review.', literal(BOOLEAN, True))
    root.step('review', 'Review', 'Ask "Payment operator" to decide using "Review input" under "Review work" and record in "Answer".', [('done', 'Done')])
    root.step('done', 'Done', 'Finish as "complete".')
    if mode == 'invoke':
        root.step('run', 'Run worker', 'Run "Worker" with inputs ["Seed" uses "Seed input"] and outputs ["Result" uses "Copy"].', [('complete', 'Review')])
    else:
        root.expression('again_test', 'Again test', BOOLEAN, read('Again'))
        root.step('run', 'Run worker', 'While "Again test" is true, repeat "Worker".', [('next', 'Review')])
        root.data('again', 'Again', BOOLEAN, literal(BOOLEAN, True), invalidates=['"Child choice" in "Worker" in "Snapshot example"', '"Review"'], participants=PEOPLE)
    child = Scope(p, 'worker', 'Worker', root, 'Child choice',
        inputs=[('seed', 'Seed', 'Local')] if mode == 'invoke' else [],
        outputs=[('result', 'Result', 'Local')] if mode == 'invoke' else [],
        closure='Keep worker' if survivor else 'Ordinary close')
    child.data('local', 'Local', INTEGER, read('Seed') if mode == 'repeat' else None,
        invalidates=['"Child choice"'], participants=PEOPLE)
    child.expression('zero_test', 'Zero test', BOOLEAN, binary('eq', read('Local'), literal(INTEGER, '0')))
    next_step = 'Stop repeating' if mode == 'repeat' else 'Worker done'
    child.step('choose', 'Child choice', 'Choose by ["zero" when "Zero test"]; otherwise "other".', [('zero', next_step), ('other', next_step)])
    if mode == 'repeat':
        child.compute('stop_repeat', 'Stop repeating', [('again', 'Again', literal(BOOLEAN, False))], 'Worker done')
    child.step('worker_done', 'Worker done', 'Finish as "complete".')
    if survivor:
        child.policy('keep_worker', 'closure', 'Keep worker', 'Reconcile in "Recovery".')
        recovery = Scope(p, 'recovery', 'Recovery', child, 'Change local' if survivor == 'writer' else 'Observe')
        if survivor == 'writer':
            recovery.compute('change_local', 'Change local', [('local', 'Local', literal(INTEGER, '7'))], 'Recovery done')
        else:
            child.policy('observe_policy', 'wait', 'Observe policy', 'Observe text. Match when (true). Authority "Payment operator". Timer none. On observation outcome "observed" of "Observe" in "Recovery" in "Worker" in "Snapshot example". On timer none.')
            recovery.step('observe', 'Observe', 'Wait under "Observe policy".', [('observed', 'Recovery done')])
        recovery.step('recovery_done', 'Recovery done', 'Finish as "complete".')
        child.child(recovery)
    root.child(child)
    invalidates = ['"Child choice" in "Worker" in "Snapshot example"'] + ([] if omit_ancestor else ['"Review"'])
    root.data('seed', 'Seed', INTEGER, literal(INTEGER, '0'), invalidates=invalidates, participants=PEOPLE)
    root.data('copy', 'Copy', INTEGER, invalidates=['"Review"'], participants=PEOPLE)
    root.data('answer', 'Answer', result, participants=PEOPLE)
    return 'Choreoform authoring "0.6.0".\n' + root.render(), p.ledger


def invoke(source, ledger, stage='check-invalidation'):
    with tempfile.TemporaryDirectory() as directory:
        s, b = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
        s.write_text(source, encoding='utf-8')
        b.write_text(json.dumps(ledger), encoding='utf-8')
        return run(stage, s, b)


def large_package(cells=100, sites=80):
    p = Package()
    p.ledger.update(profile='0.6.0', version='0.5.0', root='root', definition='historical_report_budget', annotations={'synthetic': True})
    p.ledger['contracts']['core'] = CORE3.copy()
    root = Scope(p, 'root', 'Historical report budget', None, 'Run')
    foundations(root)
    root.expression('yes', 'Yes', BOOLEAN, literal(BOOLEAN, True))
    root.step('run', 'Run', 'Run "Worker" with inputs empty and outputs empty.', [('complete', 'After 0')])
    for i in range(sites):
        target = f'After {i+1}' if i+1 < sites else 'Done'
        root.step(f'after{i}', f'After {i}', 'Choose by ["go" when "Yes"]; otherwise "other".', [('go', target), ('other', target)])
    root.step('done', 'Done', 'Finish as "complete".')
    child = Scope(p, 'worker', 'Worker', root, 'Choose')
    terms = []
    for i in range(cells):
        child.data(f'local{i}', f'Local {i}', INTEGER, literal(INTEGER, '0'), invalidates=['"Choose"'], participants=PEOPLE)
        terms.append(binary('eq', read(f'Local {i}'), literal(INTEGER, '0')))
    def balanced(values):
        if len(values) == 1:
            return values[0]
        middle = len(values) // 2
        return binary('and', balanced(values[:middle]), balanced(values[middle:]))
    child.expression('condition', 'Condition', BOOLEAN, balanced(terms))
    child.step('choose', 'Choose', 'Choose by ["go" when "Condition"]; otherwise "other".', [('go','Worker done'),('other','Worker done')])
    child.step('worker_done', 'Worker done', 'Finish as "complete".')
    root.child(child)
    return 'Choreoform authoring "0.6.0".\n' + root.render(), p.ledger


class ClosedProvenanceEvidence(unittest.TestCase):
    def test_synchronous_invoke_and_repeat_keep_ancestor_obligations_and_report_conditions(self):
        for mode in ['invoke', 'repeat']:
            source, ledger = package(mode)
            result = invoke(source, ledger)
            self.assertEqual(result.returncode, 0, result.stderr)
            report = json.loads(result.stdout)
            self.assertIs(report['executable'], False)
            self.assertEqual(report['historicalOrigins'], [{'site': 'review', 'cell': 'local', 'boundary': 'run', 'scope': 'worker', 'closure': 'ordinary_close', 'condition': 'ordinary-completion-with-no-declared-descendant-writer'}])
            self.assertIn({'site':'review','cell':'seed'}, report['invalidationObligations'])
            self.assertNotIn('Synthetic declaration review.', result.stdout.decode())
            self.assertIn('occurrence-bound-ordinary-closure-and-snapshot-enforcement', report['remaining'])

    def test_mutable_ancestor_and_surviving_writer_are_still_refused_independently(self):
        for survivor, omit, code, path in [('writer',False,'invalidation-context-unproved','/body/nodes/review'), (None,True,'invalidation-coverage','/body/data/seed/invalidates')]:
            source, ledger = package(survivor=survivor, omit_ancestor=omit)
            result = invoke(source, ledger)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(f'{code} at {path}; source bytes '.encode(), result.stderr)
            self.assertEqual(result.stdout,b'')
        source, ledger = package(survivor='readonly')
        result = invoke(source, ledger)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(json.loads(result.stdout)['historicalOrigins'][0]['closure'],'keep_worker')
        result = invoke(source, ledger, 'check-concurrency')
        self.assertEqual(result.returncode,0,result.stderr)
        self.assertEqual(json.loads(result.stdout)['isolatedFollowups'][0]['followup'],'recovery')
        # Sharing a mutable input still falls outside isolated surviving waits.
        source = source.replace('Observe text. Match when (true).','Observe text. Match when ((the value of "Seed") equals (0)).')
        result = invoke(source, ledger, 'check-concurrency')
        self.assertIn(b'scope-lifetime-unproved',result.stderr)

    def test_all_composed_native_reports_retain_the_same_conditional_history(self):
        source, ledger = package()
        expected = json.loads(invoke(source, ledger).stdout)['historicalOrigins']
        for stage in ['check-concurrency', 'check-timing', 'check-fault-routes', 'check-exceptional-availability', 'check-reconciliation-links', 'check-reconciliation-availability']:
            result = invoke(source, ledger, stage)
            self.assertEqual(result.returncode, 0, result.stderr)
            report = json.loads(result.stdout)
            self.assertIs(report['executable'],False)
            self.assertEqual(report['historicalOrigins'], expected)
            self.assertIn('occurrence-bound-ordinary-closure-and-snapshot-enforcement', report['remaining'])

    def test_format_reparse_keeps_ir_and_history_with_no_new_private_namespace(self):
        source, ledger = package()
        before = json.loads(invoke(source, ledger, 'lower').stdout)
        formatted = json.loads(invoke(source, ledger, 'format').stdout)
        after_source, after_ledger = formatted['source'], json.loads(formatted['companion'])
        self.assertEqual(json.loads(invoke(after_source, after_ledger,'lower').stdout),before)
        self.assertEqual(json.loads(invoke(source,ledger).stdout),json.loads(invoke(after_source,after_ledger).stdout))
        # Generic parent source still cannot read a child's private cell.
        ledger['declarations'].append({'id':'private_read','kind':'expression','scope':'root','name':'Private read'})
        source = source.replace('End process.', 'Let "Private read" mean (the value of "Local" in "Worker" in "Snapshot example") as whole number with parameters empty. End process.')
        result = invoke(source,ledger,'lower')
        self.assertIn(b'reference-missing',result.stderr)
        self.assertEqual(result.stdout,b'')

    def test_native_historical_report_is_bounded_before_any_output_is_written(self):
        source, ledger = large_package()
        result = invoke(source,ledger,'lower')
        self.assertEqual(result.returncode,0,result.stderr)
        result = invoke(source,ledger)
        self.assertNotEqual(result.returncode,0)
        self.assertEqual(result.stdout,b'')
        self.assertIn(b'report-budget',result.stderr)


if __name__ == '__main__':
    unittest.main()
