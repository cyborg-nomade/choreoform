# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Versioned quotas/declared-use endpoints, not benchmark or host admission."""
from copy import deepcopy
import json
from pathlib import Path
import re
import tempfile
import unittest

import authoring_probe as grammar
import check_ir_fixtures as wire
from benchmarks.rp01_resources import types, providers
from benchmarks.rp01_graph import endpoint_probe
from check_static_contracts import ROOT, run

BENCHMARK = ROOT / 'docs/benchmarks/rp01'


def resources():
    return [(r['id'], r['revision'], ROOT / r['path']) for r in json.loads((BENCHMARK / 'manifest.json').read_bytes())['resources']]


def promote(text, ledger):
    ledger = deepcopy(ledger)
    ledger.update(profile='0.5.0', version='0.4.0')
    return text.replace('"0.4.0"', '"0.5.0"', 1), ledger


def endpoint(invoked=False):
    _, _, p = types()
    provider, _, provider_pin = providers(p)['checks']
    text, package = endpoint_probe(p, provider, provider_pin)
    text, ledger = promote(text, package.ledger)
    # This isolated declaration witness retains the provider's restrictive
    # capability envelope. It is not the whole RP-01 authorization design.
    text = text.replace('Capabilities ["Check the claim"].', 'Capabilities empty.')
    for name, refs in [
        ('Check request', '"Request Claim checks"'),
        ('Current claim', '"Run worker", "Request Claim checks" in "Claim checks" in "Expense reimbursement"'),
    ]:
        match = re.search(r'Keep "' + name + r'".*?End data\.', text, re.S)
        assert match
        text = text[:match.start()] + match[0].replace('Invalidate empty.', f'Invalidate [{refs}].') + text[match.end():]
    if not invoked:
        text = text.replace('Start at "Run worker".', 'Start at "Finish witness".', 1)
    return text, ledger


def write(directory, text, ledger):
    source, companion = Path(directory) / 'source.choreo', Path(directory) / 'bindings.json'
    source.write_text(text, encoding='utf-8')
    companion.write_text(json.dumps(ledger, ensure_ascii=False), encoding='utf-8')
    return source, companion


def invoke(text, ledger, stage='lower'):
    with tempfile.TemporaryDirectory() as directory:
        source, companion = write(directory, text, ledger)
        return run(stage, source, companion, resources())


class BenchmarkProfileEvidence(unittest.TestCase):
    def test_full_source_profile_grammar_schema_ir_hash_and_format_cycles(self):
        text, ledger = promote((BENCHMARK / 'source.choreo').read_text(encoding='utf-8'), json.loads((BENCHMARK / 'bindings.json').read_bytes()))
        self.assertEqual(len(grammar.lex(text)), 8945)
        self.assertGreater(grammar.recognize(text), grammar.MAX_STATES)
        grammar.validate_ledger(ledger)
        with tempfile.TemporaryDirectory() as directory:
            source, companion = write(directory, text, ledger)
            result = run('lower', source, companion, resources())
            self.assertEqual(result.returncode, 0, result.stderr)
            candidate = wire.load(result.stdout)
            self.assertEqual(candidate['revision'], wire.revision(candidate))
            self.assertEqual(len(candidate['body']['scopes']), 9)
            self.assertEqual(json.loads(run('integrity', source, companion).stdout), grammar.package_digests(source.read_bytes(), companion.read_bytes()))
            formatted = json.loads(run('format', source, companion, resources()).stdout)
            source.write_text(formatted['source'], encoding='utf-8')
            companion.write_text(formatted['companion'], encoding='utf-8')
            self.assertEqual(json.loads(run('lower', source, companion, resources()).stdout), candidate)
            # The full draft still owes its independent static/protection fixes;
            # compiling it does not establish reimbursement or permission.
            self.assertIsNotNone(candidate['revision'])

    def test_actual_declared_use_can_bind_an_ancestor_policy_to_a_child_followup(self):
        text, ledger = endpoint()
        result = invoke(text, ledger, 'check-reconciliation-availability')
        self.assertEqual(result.returncode, 0, result.stderr)
        report = json.loads(result.stdout)
        self.assertIs(report['executable'], False)
        links = invoke(text, ledger, 'check-reconciliation-links')
        self.assertEqual(links.returncode, 0, links.stderr)
        self.assertIn('checks_request_step', json.loads(links.stdout)['ordinaryUnvisitedReconciliationSites'])
        self.assertIn('checks_observe', report['checkedReconciliationNodes'])
        raw = json.loads(invoke(text, ledger).stdout)['body']
        self.assertEqual(raw['capabilities']['checks_capability']['scope'], 'expense')
        self.assertEqual(raw['policies']['checks_wait']['scope'], 'expense')
        self.assertEqual(raw['nodes']['checks_request_step']['scope'], 'checks_service')
        self.assertEqual(raw['nodes']['checks_observe']['scope'], 'checks_recovery')
        # Existing conservative live-scope composition still refuses a called
        # surviving child; the source exception does not close that proof gap.
        called, l = endpoint(invoked=True)
        result = invoke(called, l, 'check-reconciliation-links')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b'scope-lifetime-unproved', result.stderr)

    def test_unused_incompatible_and_wrong_endpoint_namespaces_refuse(self):
        base, ledger = endpoint()
        qualified = '"Observe Claim checks" in "Claim checks recovery" in "Claim checks" in "Expense reimbursement"'
        for text, code in [
            (base.replace('Request "Check the claim" using ', 'Ask "Payment operator" to decide using '), 'wait-endpoint-use-unproved'),
            (base.replace(qualified, '"Run worker" in "Claim checks recovery" in "Claim checks" in "Expense reimbursement"'), 'reference-qualification'),
            (base.replace(qualified, '"Observe Claim checks" in "Expense types"'), 'reference-kind'),
            (base.replace('in "Expense reimbursement". On timer', 'in "Wrong root". On timer'), 'reference-qualification'),
        ]:
            result = invoke(text, ledger)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(code.encode(), result.stderr)
            self.assertEqual(result.stdout, b'')
        other = '''Rule for work "Other work". Instructions "Isolated declaration". Authority "Payment operator". Attempts 1. Delays empty. Retry faults empty. Timeout none. Complete when (true). Compensates none. End policy.
Scope "Other owner" within "Expense reimbursement". Start at "Other request". Inputs empty. Outputs empty. Outcomes ["complete"]. Use cancellation "Stop work", faults "Unhandled faults", closure "Ordinary close" and races "Acceptance order".
Step "Other request". Request "Check the claim" using "Claim input" under "Other work" and record in "Root check finding".
On "clear", continue with "Other done". On "correction", continue with "Other done". On "duplicate", continue with "Other done". On "rejected", continue with "Other done". End step.
Step "Other done". Finish as "complete". End step. End scope.
'''
        l = deepcopy(ledger)
        l['declarations'] += [
            {'id': 'other_work', 'kind': 'policy', 'scope': 'expense', 'name': 'Other work'},
            {'id': 'other_owner', 'kind': 'scope', 'scope': 'expense', 'name': 'Other owner'},
            {'id': 'other_request', 'kind': 'node', 'scope': 'other_owner', 'name': 'Other request'},
            {'id': 'other_done', 'kind': 'node', 'scope': 'other_owner', 'name': 'Other done'},
        ]
        l['symbols'].append({'id': 'other_complete', 'owner': 'other_owner', 'role': 'outcome', 'name': 'complete', 'wire': 'complete'})
        for tag in ['clear', 'correction', 'duplicate', 'rejected']:
            symbol = 'other_' + tag
            l['symbols'].append({'id': symbol, 'owner': 'other_request', 'role': 'outcome', 'name': tag, 'wire': tag})
            l['generated'].append({'id': symbol + '_flow', 'owner': 'other_request', 'role': 'flow', 'key': symbol})
        result = invoke(base.replace('End process.', other + 'End process.'), l)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b'wait-endpoint-use-unproved', result.stderr)

    def test_generic_private_data_and_outcome_references_do_not_gain_visibility(self):
        base, ledger = endpoint()
        for name, expression, ty in [
            ('Private data', 'the value of "Check request" in "Claim checks" in "Expense reimbursement"', 'type "Claim" from "Expense types"'),
            ('Private outcome', 'outcome "observed" of "Observe Claim checks" in "Claim checks recovery" in "Claim checks" in "Expense reimbursement"', 'text'),
        ]:
            l = deepcopy(ledger)
            l['declarations'].append({'id': 'private_expression', 'kind': 'expression', 'scope': 'expense', 'name': name})
            source = base.replace('End process.', f'Let "{name}" mean ({expression}) as {ty} with parameters empty. End process.')
            result = invoke(source, l)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b'reference-missing', result.stderr)

    def test_declaration_order_alias_identity_and_many_same_owner_sites(self):
        base, ledger = endpoint()
        original = json.loads(invoke(base, ledger).stdout)
        block = re.search(r'Rule for wait "Claim checks reconciliation"\..*?End policy\.', base, re.S)[0]
        reordered = base.replace(block, '').replace('End process.', block + ' End process.')
        self.assertEqual(json.loads(invoke(reordered, ledger).stdout), original)
        renamed = base.replace('Claim checks recovery', '照合')
        l = deepcopy(ledger)
        for row in l['declarations']:
            row['name'] = row['name'].replace('Claim checks recovery', '照合')
        self.assertEqual(json.loads(invoke(renamed, l).stdout), original)
        l = deepcopy(ledger)
        additions = []
        for i in range(200):
            node = f'extra{i}'
            l['declarations'].append({'id': node, 'kind': 'node', 'scope': 'checks_service', 'name': f'Extra check {i}'})
            flows = []
            for tag in ['clear', 'correction', 'duplicate', 'rejected']:
                symbol = f'{node}_{tag}'
                l['symbols'].append({'id': symbol, 'owner': node, 'role': 'outcome', 'name': tag, 'wire': tag})
                l['generated'].append({'id': symbol + '_flow', 'owner': node, 'role': 'flow', 'key': symbol})
                flows.append(f'On "{tag}", continue with "Finish Claim checks".')
            additions.append(f'Step "Extra check {i}". Request "Check the claim" using "Claim checks input" under "Claim checks work" and record in "Check outcome". ' + ' '.join(flows) + ' End step.')
        wide = base.replace('Step "Finish Claim checks".', '\n'.join(additions) + '\nStep "Finish Claim checks".')
        self.assertLess(len(grammar.lex(wide)), 16384)
        result = invoke(wide, l)
        self.assertEqual(result.returncode, 0, result.stderr)
        nodes = json.loads(result.stdout)['body']['nodes']
        self.assertEqual(sum(n['kind'] == 'activity' and n['mode'] == 'capability' for n in nodes.values()), 201)

    def test_timer_and_observation_require_the_same_actual_wait_owner(self):
        base, ledger = endpoint()
        manifest = json.loads((BENCHMARK / 'manifest.json').read_bytes())
        clock = next(r for r in manifest['resources'] if r['path'].endswith('/clock.json'))
        ledger['declarations'] += [
            {'id': 'probe_clock', 'kind': 'resource', 'scope': 'expense', 'name': 'Test clock'},
            {'id': 'probe_basis', 'kind': 'data', 'scope': 'expense', 'name': 'Clock basis'},
        ]
        ledger['resources'].append({'binding': 'probe_clock', 'kind': 'clock', 'contract': clock['id'], 'digest': clock['revision']})
        ledger['symbols'] += [
            {'id': 'probe_expiry', 'owner': 'checks_observe', 'role': 'outcome', 'name': 'expired', 'wire': 'expired'},
            {'id': 'probe_basis_purpose', 'owner': 'probe_basis', 'role': 'purpose', 'name': 'Reimbursement', 'wire': 'reimbursement'},
        ]
        ledger['generated'] += [
            {'id': 'probe_expiry_flow', 'owner': 'checks_observe', 'role': 'flow', 'key': 'probe_expiry'},
            {'id': 'probe_basis_initial', 'owner': 'probe_basis', 'role': 'initial', 'key': None},
        ]
        qualified = '"Observe Claim checks" in "Claim checks recovery" in "Claim checks" in "Expense reimbursement"'
        text = base.replace('Timer none.', 'Timer clock "Test clock", basis "Clock basis", due (10), calendar none, pause authority "Payment operator".')
        text = text.replace('On timer none.', f'On timer outcome "expired" of {qualified}.')
        text = text.replace('On "observed", continue with "Finish Claim checks recovery".', 'On "observed", continue with "Finish Claim checks recovery". On "expired", continue with "Finish Claim checks recovery".')
        text = text.replace('End process.', '''Keep "Clock basis" as whole number under "Expense access". Sensitivity "expense". Purposes ["Reimbursement"]. Participants ["Payment operator"]. Capabilities empty. Invalidate empty. Initially (0). End data. End process.''')
        result = invoke(text, ledger)
        self.assertEqual(result.returncode, 0, result.stderr)
        policy = json.loads(result.stdout)['body']['policies']['checks_wait']['body']
        self.assertEqual(policy['onObservation'], 'observed')
        self.assertEqual(policy['onTimer'], 'expired')
        changed = text.replace(f'On timer outcome "expired" of {qualified}.', 'On timer outcome "complete" of "Run worker".')
        result = invoke(changed, ledger)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b'wait-outcome-owner', result.stderr)

    def test_deeper_or_nonwait_target_is_not_a_designated_direct_child_endpoint(self):
        base, ledger = endpoint()
        scope = re.search(r'Scope "Claim checks recovery" within "Claim checks"\..*?End scope\.', base, re.S)[0]
        observe = re.search(r'Step "Observe Claim checks"\..*?End step\.', scope, re.S)[0]
        inner_observe = observe.replace('continue with "Finish Claim checks recovery"', 'continue with "Finish inner"')
        inner = '''Scope "Inner endpoint" within "Claim checks recovery". Start at "Observe Claim checks". Inputs empty. Outputs empty. Outcomes ["complete"]. Use cancellation "Stop work", faults "Unhandled faults", closure "Ordinary close" and races "Acceptance order".
''' + inner_observe + '\nStep "Finish inner". Finish as "complete". End step. End scope.'
        replacement = scope.replace(observe, '').replace('Start at "Observe Claim checks".', 'Start at "Finish Claim checks recovery".').replace('End scope.', inner + ' End scope.')
        text = base.replace(scope, replacement).replace('of "Observe Claim checks" in "Claim checks recovery"', 'of "Observe Claim checks" in "Inner endpoint" in "Claim checks recovery"')
        l = deepcopy(ledger)
        next(d for d in l['declarations'] if d['id'] == 'checks_observe')['scope'] = 'inner'
        l['declarations'] += [
            {'id': 'inner', 'kind': 'scope', 'scope': 'checks_recovery', 'name': 'Inner endpoint'},
            {'id': 'inner_done', 'kind': 'node', 'scope': 'inner', 'name': 'Finish inner'},
        ]
        l['symbols'].append({'id': 'inner_complete', 'owner': 'inner', 'role': 'outcome', 'name': 'complete', 'wire': 'complete'})
        result = invoke(text, l)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b'wait-endpoint-use-unproved', result.stderr)
        l = deepcopy(ledger)
        l['symbols'].append({'id': 'fake_finish_outcome', 'owner': 'checks_recovered', 'role': 'outcome', 'name': 'observed', 'wire': 'observed'})
        text = base.replace('of "Observe Claim checks" in', 'of "Finish Claim checks recovery" in')
        result = invoke(text, l)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b'wait-endpoint-owner', result.stderr)


if __name__ == '__main__':
    unittest.main()
