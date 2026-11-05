# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Ordinary parallel source/artifact evidence, not host or invariant certification."""
import json
from pathlib import Path
import tempfile
import unittest

import jsonschema
import authoring_probe as grammar
import check_ir_fixtures as wire
from check_authoring_evidence import check_links
from check_static_contracts import ROOT, EXAMPLES, run, service_resources


def parallel_package(reuse=False):
    text = (EXAMPLES / 'terminal.choreo').read_text(encoding='utf-8')
    ledger = json.loads((EXAMPLES / 'terminal.bindings.json').read_bytes())
    text = text.replace('Start at "Done".', 'Start at "Fork".', 1)
    right = 'Left' if reuse else 'Right'
    outcomes = 'outcome "complete" of "Left"' + ('' if reuse else ', outcome "complete" of "Right"')
    addition = f'''Step "Fork".
      Start together ["left" runs "Left", "right" runs "{right}"] and gather at "Gather".
    End step.
    Step "Gather".
      Gather from "Fork" when all report [{outcomes}]; for unfinished work use "Settle".
      On "ready", continue with "Done".
    End step.
    '''
    for name in ['Left', 'Right']:
        addition += f'''Scope "{name}" within "Terminal example".
          Start at "{name} done". Inputs empty. Outputs empty. Outcomes ["complete"].
          Use cancellation "Stop", faults "Fault handling", closure "Close" and races "Commit order".
          Step "{name} done". Finish as "complete". End step.
        End scope.
        '''
        scope = name.lower()
        ledger['declarations'] += [
            {'id': scope, 'kind': 'scope', 'scope': 'scope_terminal', 'name': name},
            {'id': f'{scope}_done', 'kind': 'node', 'scope': scope, 'name': f'{name} done'},
        ]
        ledger['symbols'].append({'id': f'{scope}_complete', 'owner': scope, 'role': 'outcome', 'name': 'complete', 'wire': 'complete'})
    text = text.replace('End process.', addition + 'End process.')
    ledger['declarations'] += [
        {'id': 'fork', 'kind': 'node', 'scope': 'scope_terminal', 'name': 'Fork'},
        {'id': 'gather', 'kind': 'node', 'scope': 'scope_terminal', 'name': 'Gather'},
    ]
    ledger['symbols'] += [
        {'id': 'branch_left', 'owner': 'fork', 'role': 'branch', 'name': 'left', 'wire': 'left'},
        {'id': 'branch_right', 'owner': 'fork', 'role': 'branch', 'name': 'right', 'wire': 'right'},
        {'id': 'gather_ready', 'owner': 'gather', 'role': 'outcome', 'name': 'ready', 'wire': 'ready'},
    ]
    ledger['generated'].append({'id': 'gather_done', 'owner': 'gather', 'role': 'flow', 'key': 'gather_ready'})
    return text, json.dumps(ledger)


class ConcurrencyEvidence(unittest.TestCase):
    def test_reports_and_source_cycles(self):
        for name in ['terminal', 'review', 'service', 'control', 'parallel', 'reused']:
            with self.subTest(package=name), tempfile.TemporaryDirectory() as directory:
                source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
                if name in ['parallel', 'reused']:
                    text, bindings = parallel_package(name == 'reused')
                    source.write_text(text, encoding='utf-8')
                    companion.write_text(bindings, encoding='utf-8')
                else:
                    source.write_bytes((EXAMPLES / f'{name}.choreo').read_bytes())
                    companion.write_bytes((EXAMPLES / f'{name}.bindings.json').read_bytes())
                grammar.recognize(source.read_text(encoding='utf-8'))
                resources = service_resources() if name == 'service' else []
                lowered = run('lower', source, companion, resources)
                self.assertEqual(lowered.returncode, 0, lowered.stderr)
                candidate = wire.load(lowered.stdout)
                jsonschema.Draft202012Validator(wire.SCHEMA).validate(candidate)
                self.assertEqual(candidate['revision'], wire.revision(candidate))
                checked = run('check-concurrency', source, companion, resources)
                self.assertEqual(checked.returncode, 0, checked.stderr)
                report = json.loads(checked.stdout)
                self.assertEqual(report['status'], 'concurrency-checks-passed')
                self.assertIs(report['executable'], False)
                self.assertEqual(report['revision'], candidate['revision'])
                self.assertIn('domain-and-irreversible-race-invariant-proofs', report['remaining'])
                forks = {key for key, n in candidate['body']['nodes'].items() if n['kind'] in ['split', 'fanout']}
                self.assertEqual(set(report['checkedForks']) | set(report['uncoveredForks']), forks)
                self.assertFalse(set(report['checkedForks']) & set(report['uncoveredForks']))
                self.assertEqual(report['checkedForks'], ['fork'] if name in ['parallel', 'reused'] else [])
                formatted = run('format', source, companion, resources)
                self.assertEqual(formatted.returncode, 0, formatted.stderr)
                package = json.loads(formatted.stdout)
                source.write_text(package['source'], encoding='utf-8')
                companion.write_text(package['companion'], encoding='utf-8')
                self.assertEqual(json.loads(run('lower', source, companion, resources).stdout), candidate)
                self.assertEqual(json.loads(run('check-concurrency', source, companion, resources).stdout), report)

    def test_early_join_passes_earlier_stages_then_refuses_as_unproved(self):
        with tempfile.TemporaryDirectory() as directory:
            text, bindings = parallel_package()
            text = text.replace('when all report', 'when any report')
            source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
            source.write_text(text, encoding='utf-8')
            companion.write_text(bindings, encoding='utf-8')
            grammar.recognize(text)
            earlier = run('check-invalidation', source, companion)
            self.assertEqual(earlier.returncode, 0, earlier.stderr)
            checked = run('check-concurrency', source, companion)
            self.assertNotEqual(checked.returncode, 0)
            self.assertEqual(checked.stdout, b'')
            self.assertIn(b'parallel-lifetime-unproved at /body/nodes/gather', checked.stderr)

    def test_documents_and_licenses(self):
        for relative in ['docs/decisions/0022-ordinary-concurrency.md',
                         'docs/evaluation/0022-concurrency-plan.md',
                         'docs/evaluation/0022-ordinary-concurrency.md']:
            path = ROOT / relative
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', path.read_text(encoding='utf-8'))
            check_links(path)


if __name__ == '__main__':
    unittest.main(verbosity=2)
