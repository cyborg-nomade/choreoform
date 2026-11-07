# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Core-0.3 source evidence; no benchmark, host or phase admission."""
from copy import deepcopy
from hashlib import sha256
import json
from pathlib import Path
import tempfile
import unittest

import authoring_probe as grammar
import check_ir_fixtures as wire
from benchmarks.rp01_resources import types, providers, clock_resources, encoded, pin, record, INTEGER
from benchmarks.rp01_graph import build, foundations, PEOPLE
from benchmarks.rp01_source import Package, Scope
from check_static_contracts import ROOT, run

CORE3 = {'contract': 'urn:choreoform:contracts:core-profile:0.3.0',
         'digest': 'sha256:30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3', 'dialect': 'core'}


def promote(source, ledger):
    ledger = deepcopy(ledger)
    source = source.replace('authoring "0.4.0"', 'authoring "0.6.0"', 1).replace('authoring "0.5.0"', 'authoring "0.6.0"', 1)
    ledger.update(profile='0.6.0', version='0.5.0')
    ledger['contracts']['core'] = CORE3.copy()
    return source, ledger


def full(read=False, payment_policy_revisions=False):
    # Deliberately retain the pre-link-patch fixture for exact missing-coverage
    # and metadata regression evidence; the current package is generated separately.
    _, raw, p = types()
    resources = providers(p, shared=True) | clock_resources(shared=True)
    source, package = build(p, {k: resources[k][0] for k in ['checks', 'payment', 'notice']}, {k: v[2] for k, v in resources.items()}, payment_policy_revisions=payment_policy_revisions)
    source, ledger = promote(source, package.ledger)
    if read:
        source = source.replace('Permit access when ((the fact "purpose") equals ("reimbursement")).', 'Permit access when (((the fact "purpose") equals ("reimbursement")) and ((the value of "Payment due") equals (the value of "Payment due"))).')
    supplied = [(p['id'], p['revision'], raw)] + [(v[2]['id'], v[2]['revision'], v[1]) for v in resources.values()]
    cp = resources['clock'][0]['profile']
    supplied.append((cp['id'], cp['revision'], (ROOT / 'docs/ir/contracts' / (cp['revision'].replace(':', '-') + '.txt')).read_bytes()))
    return source, ledger, supplied


def metadata_graph(count=2, reads=True):
    p = Package()
    root = Scope(p, 'expense', 'Metadata example', None, 'Done', inputs=[('policy', 'Policy input', 'Policy value')])
    foundations(root)
    caps = [f'cap{i}' for i in range(count)]
    provider = {'kind': 'provider-contract', 'version': '0.1.0', 'id': 'urn:test:metadata-source', 'input': record(), 'output': record(), 'outcomes': {'complete': True}, 'resultProtection': {'policy': 'expense_access', 'sensitivity': 'expense', 'purposes': {'reimbursement': True}, 'participants': {k + '_actor': True for k in ['employee','manager','finance','operator','clock']}, 'capabilities': {c: True for c in caps}}, 'observations': {'complete': {'state': 'succeeded', 'result': record(), 'outcome': 'complete'}}, 'idempotency': 'none', 'effectClass': 'read-only'}
    raw = encoded(provider)
    provider_pin = pin(provider['id'], raw)
    p.resource('provider_binding', 'Provider', 'provider', provider_pin)
    for i, cap in enumerate(caps):
        root.capability(cap, f'Consumer {i}', 'Provider', record(), record(), 'Payment operator', 'Effects')
    root.policy('effect', 'effect', 'Effects', 'Effect is read-only. Idempotency is none. Reconcile under "Observations".')
    predicate = '((the value of "Policy value") equals (0))' if reads else '(true)'
    root.policy('wait', 'wait', 'Observations', f'Observe text. Match when {predicate}. Authority "Payment operator". Timer none. On observation outcome "observed" of "Observe". On timer none.')
    root.data('policy_value', 'Policy value', INTEGER, participants=PEOPLE, capabilities=[f'Consumer {i}' for i in range(count)])
    root.step('observe', 'Observe', 'Wait under "Observations".', [('observed', 'Done')])
    root.step('done', 'Done', 'Finish as "complete".')
    source, ledger = promote('Choreoform authoring "0.5.0".\n' + root.render(), p.ledger)
    return source, ledger, [(provider_pin['id'], provider_pin['revision'], raw)]


def invoke(package, stage='lower'):
    source, ledger, resources = package
    with tempfile.TemporaryDirectory() as directory:
        base = Path(directory)
        s, b = base / 'source.choreo', base / 'bindings.json'
        s.write_text(source, encoding='utf-8')
        b.write_bytes(encoded(ledger))
        supplied = []
        for i, (identity, revision, raw) in enumerate(resources):
            path = base / f'resource{i}.json'
            path.write_bytes(raw)
            supplied.append((identity, revision, path))
        return run(stage, s, b, [] if stage == "integrity" else supplied)


class MetadataAuthoringEvidence(unittest.TestCase):
    def test_full_source_grammar_schema_static_control_protection_and_remaining_refusal(self):
        package = full()
        source, ledger, _ = package
        self.assertEqual(len(grammar.lex(source)), 8945)
        grammar.recognize(source)
        grammar.validate_ledger(ledger)
        for stage in ['check', 'check-control', 'check-protection']:
            result = invoke(package, stage)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertIs(json.loads(result.stdout)['executable'], False)
        result = invoke(package, 'check-invalidation')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b'invalidation-coverage at /body/data/disposition/invalidates; source bytes ', result.stderr)

    def test_all_actual_metadata_reads_are_derived_in_self_and_mutual_recipient_components(self):
        for count in [1, 2]:
            package = metadata_graph(count)
            result = invoke(package, 'check')
            self.assertEqual(result.returncode, 0, result.stderr)
            raw = json.loads(invoke(package).stdout)['body']
            for i in range(count):
                self.assertEqual(raw['policies'][f'cap{i}_provider']['body']['reads'], {'policy_value': True})
            self.assertEqual(raw['policies']['effect']['body']['reads'], {'policy_value': True})
        # Real protected clock-source and activity metadata retain their shared
        # root input read; compiler output is independently semantically rechecked.
        package = full(read=True)
        result = invoke(package, 'check')
        self.assertEqual(result.returncode, 0, result.stderr)
        raw = json.loads(invoke(package).stdout)['body']
        for policy in ['payment_work', 'checks_capability_provider', 'payment_capability_provider', 'notice_capability_provider']:
            self.assertIn('payment_due', raw['policies'][policy]['body']['reads'])

    def test_format_reparse_alias_and_independent_hash_revision_cycles(self):
        package = metadata_graph()
        result = invoke(package)
        self.assertEqual(result.returncode, 0, result.stderr)
        candidate = wire.load(result.stdout)
        self.assertEqual(candidate['revision'], wire.revision(candidate))
        formatted = json.loads(invoke(package, 'format').stdout)
        pair = (formatted['source'], json.loads(formatted['companion']), package[2])
        self.assertEqual(json.loads(invoke(pair).stdout), candidate)
        actual = json.loads(invoke(package, 'integrity').stdout)
        self.assertEqual(actual, grammar.package_digests(package[0].encode(), encoded(package[1])))
        source, ledger, resources = deepcopy(package)
        source = source.replace('Policy value', '方針値')
        next(d for d in ledger['declarations'] if d['id'] == 'policy_value')['name'] = '方針値'
        self.assertEqual(json.loads(invoke((source, ledger, resources)).stdout), candidate)

    def test_legacy_cycles_new_pairs_and_private_or_wrong_metadata_still_refuse(self):
        source, ledger, resources = metadata_graph()
        old = deepcopy(ledger)
        old.update(profile='0.5.0', version='0.4.0')
        old['contracts']['core'].update(contract='urn:choreoform:contracts:core-profile:0.2.0', digest='sha256:5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623')
        result = invoke((source.replace('"0.6.0"', '"0.5.0"', 1), old, resources))
        self.assertIn(b'policy-cycle', result.stderr)
        for version, core, code in [('0.4.0', CORE3, b'ledger-profile'), ('0.5.0', old['contracts']['core'], b'contract-pin')]:
            l = deepcopy(ledger)
            l['version'], l['contracts']['core'] = version, core
            result = invoke((source, l, resources))
            self.assertIn(code, result.stderr)
        for bad in ['operator_actor', 'missing_policy']:
            p = json.loads(resources[0][2])
            p['resultProtection']['policy'] = bad
            raw = encoded(p)
            r = (p['id'], 'sha256:' + sha256(raw).hexdigest(), raw)
            l = deepcopy(ledger)
            l['resources'][0]['digest'] = r[1]
            result = invoke((source, l, [r]), 'check')
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(result.stdout, b'')

    def test_provider_metadata_cannot_export_a_private_capability(self):
        source, ledger, resources = metadata_graph(count=1)
        ledger['declarations'].append({'id': 'private_cap', 'kind': 'capability', 'scope': 'remaining', 'name': 'Private consumer'})
        ledger['generated'].append({'id': 'private_cap_provider', 'owner': 'private_cap', 'role': 'provider-policy', 'key': None})
        source = source.replace('Step "Release remaining owner".', 'Use capability "Private consumer" from "Provider" with input record with empty and output record with empty, authority "Payment operator" and effects "Effects". Step "Release remaining owner".')
        provider = json.loads(resources[0][2])
        provider['resultProtection']['capabilities']['private_cap'] = True
        raw = encoded(provider)
        resource = (provider['id'], 'sha256:' + sha256(raw).hexdigest(), raw)
        ledger['resources'][0]['digest'] = resource[1]
        result = invoke((source, ledger, [resource]), 'check')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b'visibility', result.stderr)
        self.assertEqual(result.stdout, b'')

    def test_closed_dependency_budget_refuses_instead_of_dropping_dense_metadata(self):
        package = metadata_graph(count=180, reads=False)
        self.assertLess(len(grammar.lex(package[0])), 16384)
        grammar.recognize(package[0])
        grammar.validate_ledger(package[1])
        result = invoke(package)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(result.stdout, b'')
        self.assertIn(b'work-limit', result.stderr)


if __name__ == '__main__':
    unittest.main()
