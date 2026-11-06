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


class ClosedProvenanceEvidence(unittest.TestCase):
    def test_baseline_context_refusals_and_complete_mutable_input_reads(self):
        for mode, survivor, omit in [('invoke', None, False), ('repeat', None, False), ('invoke', 'readonly', False), ('invoke', 'writer', False), ('invoke', None, True)]:
            with self.subTest(mode=mode, survivor=survivor, omit=omit):
                source, ledger = package(mode, survivor, omit)
                result = invoke(source, ledger, 'check-protection')
                self.assertEqual(result.returncode, 0, result.stderr)
                raw = json.loads(invoke(source, ledger, 'lower').stdout)['body']
                self.assertIn('seed', raw['nodes']['run']['reads'])
                result = invoke(source, ledger)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(b'invalidation-context-unproved at /body/nodes/review', result.stderr)
                self.assertEqual(result.stdout, b'')


if __name__ == '__main__':
    unittest.main()
