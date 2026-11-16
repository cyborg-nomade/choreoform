# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Existing source/C11 identity boundaries; no new access or full-case IR."""
import json
import re
import unittest

import authoring_probe as grammar
from check_source_capacity import promote, fixed
from check_finite_work_authoring import package as small
from check_metadata_authoring import invoke


def package(target='none'):
    s, l, r = promote(small())
    addition = f'''Scope "Audit" within "Main".
Start at "Audit done".
Inputs empty. Outputs empty. Outcomes ["complete"].
Use cancellation "Cancel", faults "Faults", closure "Close" and races "Race".
Rule for work "Audit work". Instructions "Synthetic declaration". Authority "Operator". Attempts 1. Delays empty. Retry faults empty. Timeout none. Complete when (true). Compensates {target}. End policy.
Step "Audit done". Finish as "complete". End step.
End scope.
'''
    s = s.replace('End process.', addition+'End process.')
    for identity, kind, scope, name in [
        ('audit', 'scope', 'main', 'Audit'),
        ('audit_done', 'node', 'audit', 'Audit done'),
        ('audit_work', 'policy', 'audit', 'Audit work'),
    ]:
        l['declarations'].append(dict(id=identity, kind=kind, scope=scope, name=name))
    l['symbols'].append(dict(id='audit_outcome_complete', owner='audit', role='outcome', name='complete', wire='complete'))
    return s, l, r


def refusal(p, code, text):
    result = invoke(p)
    if result.returncode != 1 or result.stdout != b'':
        raise AssertionError((result.returncode, result.stdout, result.stderr))
    match = re.fullmatch(rb'([a-z-]+) at (\d+)\.\.(\d+)\n', result.stderr)
    if not match:
        raise AssertionError(result.stderr)
    actual, start, end = match.groups()
    if actual.decode() != code or p[0].encode()[int(start):int(end)] != text.encode():
        raise AssertionError((result.stderr, p[0].encode()[int(start):int(end)], text))


class CompensationReferenceEvidence(unittest.TestCase):
    def test_unused_sibling_rule_has_a_real_positive_baseline(self):
        p = package(); grammar.recognize(p[0]); grammar.validate_ledger(p[1])
        result = invoke(p, 'check'); self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIsNone(json.loads(invoke(p).stdout)['body']['policies']['audit_work']['body']['compensates'])

    def test_qualified_and_unqualified_sibling_or_foreign_activity_are_not_visible(self):
        for reference in ['"Create payable"', '"Create payable" in "Root" in "Main"', '"Create payable" in "Foreign process"']:
            refusal(package(reference), 'reference-missing', reference)

    def test_visible_ancestor_and_same_scope_identity_differ_from_wrong_qualification(self):
        s, l, r = promote(small())
        good = '"Create payable" in "Root" in "Main"'
        p = s.replace('Compensates "Create payable".', 'Compensates '+good+'.'), l, r
        self.assertEqual(invoke(p, 'check').returncode, 0)
        wrong = '"Create payable" in "Main"'
        refusal((s.replace('Compensates "Create payable".', 'Compensates '+wrong+'.'), l, r), 'reference-qualification', wrong)
        # Same-site syntax resolves; eligible *prior dynamic occurrence* is not
        # established by accepting a declaration whose site is also its user.
        s = s.replace('Compensates "Create payable".', 'Compensates "Adjust account".')
        self.assertEqual(invoke((s,l,r), 'check').returncode, 0)
        result = invoke((s,l,r), 'check-concurrency')
        self.assertEqual(result.returncode, 1)
        self.assertIn(b'owned-continuation-compensation-source', result.stderr)

    def test_missing_wrong_kind_duplicate_name_private_data_and_outcome_stay_closed(self):
        refusal(package('"Missing"'), 'reference-missing', '"Missing"')
        # A visible node resolves syntactically but the real SDK rejects its kind.
        p = package('"Main done"'); self.assertEqual(invoke(p).returncode, 0)
        result = invoke(p, 'check'); self.assertEqual(result.returncode, 1)
        self.assertIn(b'compensation-target', result.stderr)
        refusal(package('"Operator"'), 'reference-kind', '"Operator"')
        s,l,r = package(); l['declarations'].append(dict(id='audit_duplicate',kind='node',scope='audit',name='Audit done'))
        result = invoke((s,l,r)); self.assertEqual(result.returncode,1); self.assertEqual(result.stdout,b'')
        self.assertEqual(result.stderr,b'binding-duplicate at 0..0\n')
        s,l,r = package(); s = s.replace('Complete when (true). Compensates none. End policy.\nStep "Audit done"', 'Complete when ((the value of "Origin value" in "Root" in "Main") equals (0)). Compensates none. End policy.\nStep "Audit done"')
        refusal((s,l,r), 'reference-missing', '"Origin value" in "Root" in "Main"')
        s,l,r = package(); s = s.replace('Complete when (true). Compensates none. End policy.\nStep "Audit done"', 'Complete when ((outcome "done" of "Create payable" in "Root" in "Main") equals (outcome "done" of "Create payable" in "Root" in "Main")). Compensates none. End policy.\nStep "Audit done"')
        grammar.recognize(s)  # Valid syntax, independently of binding refusal.
        refusal((s,l,r), 'reference-missing', '"Create payable" in "Root" in "Main"')

    def test_all_committed_benchmark_bytes_are_fixed(self):
        before=fixed(); result=invoke(package('"Create payable" in "Root" in "Main"'))
        self.assertEqual(result.returncode,1); self.assertEqual(fixed(),before)


if __name__ == '__main__': unittest.main()
