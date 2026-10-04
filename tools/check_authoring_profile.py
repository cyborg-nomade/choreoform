# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Grammar and ledger specification tests, independent of product admission."""

from copy import deepcopy
from hashlib import sha256
import json
from pathlib import Path
import unittest
from unittest.mock import patch

import jsonschema

import authoring_probe as probe
from check_authoring_evidence import check_links

EXAMPLES = probe.ROOT / 'docs/authoring/examples'
SOURCE = (EXAMPLES / 'terminal.choreo').read_text()
RAW_LEDGER = (EXAMPLES / 'terminal.bindings.json').read_bytes()

# Fragments exercise syntax only. Their referenced declarations are deliberately
# absent; acceptance here must never be reported as semantic validity.
CASES = {
    'human': ['Ask "Reviewer" to decide using "Input" under "Work" and record in "Result".',
              'Ask "Reviewer" to decide using "Input" under "Work" and record in nothing.'],
    'request': ['Request "Provider" using "Input" under "Work" and record in nothing.'],
    'compute': ['Set together ["A" to (1), "B" to (the value of "A")].'],
    'invoke': ['Run "Child" with inputs ["request" uses "Input expression"] and outputs ["answer" uses "Answer cell"].',
               'Run "Child" with inputs empty and outputs empty.'],
    'decision': ['Choose by ["approved" when "Allowed", "rejected" when "Denied"]; otherwise "unknown".',
                 'Choose by ["approved" when "Allowed"]; otherwise none.'],
    'split': ['Start together ["finance" runs "Finance", "legal" runs "Legal"] and gather at "Join".'],
    'join': ['Gather from "Split" when all report [outcome "complete" of "Child"]; for unfinished work use "Settle".'],
    'wait': ['Wait under "Observation".'],
    'repeat': ['While "Still needed" is true, repeat "Correction".'],
    'fanout': ['For each item of "Items", using key "Key", run "Child" with item "request"; seal when "Sealed" is true; for changes use "Membership"; gather at "Join".'],
    'finish': ['Finish as "complete".'],
    'join_condition': ['all report [outcome "done" of "Child"]',
                       'any report [outcome "done" of "Child"]',
                       'at least 2 report [outcome "done" of "Child", outcome "refused" of "Other"]',
                       'both (all report [outcome "done" of "Child"]) and (any report [outcome "done" of "Child"])',
                       'either (all report [outcome "done" of "Child"]) or (at least 1 report [outcome "done" of "Child"])'],
    'type': ['truth value', 'text', 'whole number', 'decimal with 18 places',
             'record with empty', 'record with ["a" as text, "b" as whole number]',
             'choice with ["yes" as text, outcome "correction needed" of "Review" as record with empty]',
             'list of optional text', 'optional whole number', 'type "Amount" from "Money contract"'],
    'literal': ['true', 'false', '""', '"héllo"', '-9223372036854775808',
                'decimal -1.20 with 2 places', 'decimal 0 with 0 places',
                'literal of record with ["a" as list of whole number] with value record with ["a" is (list containing [(1), (2)])]',
                'literal of optional text with value absent',
                'literal of optional text with value present ("x")',
                'literal of choice with ["a" as text] with value case "a" containing ("x")',
                'literal of type "Amount" from "Money" with value decimal 1.20 with 2 places',
                'outcome "correction needed" of "Review"',
                'purpose "Expense review" of "Request"'],
    'read': ['the value of "Cell" in "Child" in "Root"'],
    'parameter': ['the parameter "item"'],
    'fact': ['the fact "principal"'],
    'field': ['field "amount" of (the value of "Request")'],
    'record': ['record with empty', 'record with ["a" is (1), "b" is (true)]'],
    'list': ['list of text containing empty', 'list of whole number containing [(1), (2)]'],
    'variant': ['case "yes" of choice with ["yes" as text] containing ("ok")',
                'case outcome "correction needed" of "Review" of choice with [outcome "correction needed" of "Review" as text] containing ("fix")'],
    'absent': ['no value of text'],
    'some': ['some (1)'],
    'exists': ['a value exists in (some (1))'],
    'present': ['the present value of (some (1))'],
    'case_name': ['the case of (the value of "Decision")'],
    'payload': ['the payload of (the value of "Decision") for "yes"',
                'the payload of (the value of "Decision") for outcome "approved" of "Review"'],
    'conditional': ['if (true) then (1) otherwise (2)'],
    'negation': ['not (false)'],
    'binary': [f'(1) {operator} (2)' for operator in
               ('and', 'or', 'equals', 'is less than', 'plus', 'minus', 'times', 'contains')],
    'length': ['the length of (list of text containing empty)'],
    'index': ['item (the value of "Items") at index (0)'],
    'rescale': ['(decimal 1.20 with 2 places) expressed with 3 decimal places'],
    'wrap': ['(decimal 1.20 with 2 places) as named type "Amount" from "Money"'],
    'representation': ['the representation of (the value of "Amount")'],
    'actor_policy': ['Rule for actor "Actor". Permit when (true). End policy.'],
    'protection_policy': ['Rule for protection "Protection". Permit access when ((the fact "purpose") equals (purpose "Expense review" of "Request")). End policy.'],
    'capability_policy': ['Rule for capability "Contract". Provider contract "Provider". End policy.'],
    'effect_policy': [f'Rule for effect "Effect". Effect is {kind}. Idempotency is {key}. Reconcile under "Wait". End policy.'
                      for kind in ('read-only', 'compensatable', 'irreversible') for key in ('none', 'stable-key')],
    'work_policy': ['Rule for work "Work". Instructions "Check the receipt.". Authority "Reviewer". Attempts 2. Delays [1000 microseconds]. Retry faults ["transient"]. Timeout none. Complete when (true). Compensates none. End policy.',
                    'Rule for work "Work". Instructions "Check". Authority "Reviewer". Attempts 1. Delays empty. Retry faults empty. Timeout clock "Clock", basis "Basis", due (1), calendar "Calendar", pause authority "Reviewer". Complete when (true). Compensates "Previous work". End policy.'],
    'faults_policy': ['Rule for faults "Faults". Handle empty. End policy.',
                      'Rule for faults "Faults". Handle ["unavailable" by "Handler", "overflow" by "Overflow handler"]. End policy.'],
    'wait_policy': ['Rule for wait "Wait". Observe text. Match when (true). Authority "Reviewer". Timer none. On observation outcome "observed" of "Wait step". On timer none. End policy.',
                    'Rule for wait "Wait". Observe text. Match when (true). Authority "Reviewer". Timer clock "Clock", basis "Basis", due (1), calendar none, pause authority "Reviewer". On observation outcome "observed" of "Wait step". On timer outcome "expired" of "Wait step". End policy.'],
    'cancel_policy': ['Rule for cancel "Cancel". Authority "Reviewer". For children use "Settle". End policy.'],
    'settlement_policy': ['Rule for settlement "Settle". Unfinished work retain. End policy.',
                          'Rule for settlement "Settle". Unfinished work cancel. End policy.',
                          'Rule for settlement "Settle". Unfinished work transfer to "Root". End policy.'],
    'closure_policy': ['Rule for closure "Close". Fully terminal. End policy.',
                       'Rule for closure "Close". Reconcile in "Followup". End policy.'],
    'race_policy': ['Rule for race "Race". Accept in acceptance order. End policy.',
                    'Rule for race "Race". Guard commits when (true). End policy.'],
    'membership_policy': ['Rule for membership "Changes". For removal use "Settle". Changed items invalidate. End policy.'],
    'data': ['Keep "Request" as text under "Protection". Sensitivity "internal". Purposes ["review"]. Participants ["Reviewer"]. Capabilities empty. Invalidate ["Review"]. Initially unavailable. End data.',
             'Keep "Request" as text under "Protection". Sensitivity "internal". Purposes ["review"]. Participants empty. Capabilities ["Provider"]. Invalidate empty. Initially ("hello"). End data.'],
    'expression_declaration': ['Let "Input" mean (the value of "Request") as text with parameters empty.',
                               'Let "Key" mean (the parameter "item") as text with parameters ["item" as text].'],
    'actor': ['Require actor "Reviewer" under "Actor".'],
    'capability': ['Use capability "Provider" from "Provider resource" with input text and output text, authority "Reviewer" and effects "Effects".'],
    'step': ['Step "Review". Ask "Reviewer" to decide using "Input" under "Work" and record in "Result". Also read ["Audit basis"]. On "approved", continue with "Done". On "correction needed", continue with "Correct". End step.']
}


class GrammarTests(unittest.TestCase):
    def test_malformed_grammar_is_refused(self):
        for grammar in ('a = unknown ;', 'a = "x" ; a = "y" ;',
                        'a = "x"', 'a = ("x" ;', 'a = | "x" ;'):
            with patch.object(Path, 'read_text', return_value=grammar), self.assertRaises(ValueError):
                probe.load_grammar()

    def test_complete_program_and_order_independent_presentation(self):
        probe.recognize(SOURCE)
        probe.recognize(SOURCE.replace('  ', '\t').replace('\n', '\r\n'))
        actor = '  Require actor "Reviewer" under "Reviewer requirement".\n'
        probe.recognize(SOURCE.replace(actor, '').replace('End process.', actor + 'End process.'))
        nested = 'Scope "Child" within "Terminal example".' + SOURCE.split('Process "Terminal example".', 1)[1].replace('End process.', 'End scope.')
        probe.recognize(SOURCE.replace('End process.', nested + 'End process.'))

    def test_closed_vocabulary_fragments(self):
        rules = probe.load_grammar()
        for rule, examples in CASES.items():
            for source in examples:
                with self.subTest(rule=rule, source=source):
                    probe.recognize(source, rule, rules)
        self.assertEqual(len(CASES), 52)

    def test_unknown_missing_reordered_and_trailing_clauses(self):
        for source in (SOURCE + 'Pay it when ready.', SOURCE.replace('Inputs empty.', ''),
                       SOURCE.replace('Start at "Done".', 'Begin at "Done".'),
                       SOURCE.replace('End step.', 'End rule.'),
                       SOURCE.replace('Outcomes ["complete"].', 'Outcomes [].'),
                       SOURCE.replace('Process "Terminal example".', 'Process "".'),
                       SOURCE.replace('"0.2.0"', '"0.3.0"'),
                       SOURCE.replace('Inputs empty.\n  Outputs empty.', 'Outputs empty.\n  Inputs empty.')):
            with self.subTest(source=source), self.assertRaises(ValueError):
                probe.recognize(source)
        for source, rule in [('1.0', 'literal'), ('01', 'literal'), ('-0', 'literal'),
                              ('true and false', 'expression'), ('any report empty', 'join_condition'),
                              ('Set together empty.', 'compute'), ('Rule for work "W". Retry safely. End policy.', 'policy')]:
            with self.subTest(source=source), self.assertRaises(ValueError):
                probe.recognize(source, rule)

    def test_quotes_comments_and_exact_unicode(self):
        for source in ('"héllo # ."', '"quote \\" and slash \\\\"', '"é"', '"é"'):
            probe.recognize(source, 'literal')
        self.assertNotEqual(probe.lex('"é"'), probe.lex('"é"'))
        probe.recognize('true # do not parse this\n', 'literal')
        for source in ('"a\\n"', '"a\nb"', '"\x7f"', '"\u0085"', '"\u2028"', '"\u2029"', '"unterminated', '\ufefftrue', '1with'):
            with self.assertRaises(ValueError):
                probe.recognize(source, 'literal')

    def test_exact_decimal_and_bounds(self):
        self.assertEqual(probe.exact_decimal('-1.20', 2), '-120')
        self.assertEqual(probe.exact_decimal('0', 0), '0')
        self.assertEqual(probe.exact_decimal('-9223372036854775808', 0), '-9223372036854775808')
        for scale in (True, 1.0):
            with self.assertRaises(ValueError):
                probe.exact_decimal('1.0', scale)
        for spelling, scale in [('1.2', 2), ('-0.00', 2), ('1e2', 0), ('01.2', 1),
                                ('9223372036854775808', 0), ('0.0', 0), ('0', 19), ('0', -1)]:
            with self.assertRaises(ValueError):
                probe.exact_decimal(spelling, scale)

    def test_oracle_resource_limits(self):
        for source in ('#' + 'x' * probe.MAX_BYTES, '(' * 65 + 'true' + ')' * 65,
                       'true ' * (probe.MAX_TOKENS + 1)):
            with self.assertRaises(ValueError):
                probe.recognize(source, 'expression')
        with patch.object(probe, 'MAX_STATES', 10), self.assertRaises(ValueError):
            probe.recognize(SOURCE)


class LedgerTests(unittest.TestCase):
    def setUp(self):
        self.ledger = probe.strict_json(RAW_LEDGER)

    def refuses(self, mutate):
        changed = deepcopy(self.ledger)
        mutate(changed)
        with self.assertRaises((ValueError, jsonschema.ValidationError)):
            probe.validate_ledger(changed)

    def test_schema_and_integrity_vector(self):
        probe.validate_ledger(self.ledger)
        jsonschema.Draft202012Validator.check_schema(json.loads((probe.ROOT / 'schemas/authoring/bindings-0.1.schema.json').read_text()))
        digests = probe.package_digests(SOURCE.encode(), RAW_LEDGER)
        self.assertEqual(digests['package'], 'sha256:9d0328a4cd9266221ac31393567bc23ef6c513bfe8bb24be573047e0601b44d5')
        reformatted = json.dumps(self.ledger, ensure_ascii=False, sort_keys=True).encode()
        self.assertEqual(digests, probe.package_digests(SOURCE.encode(), reformatted))
        whitespace = probe.package_digests((SOURCE + '\n').encode(), RAW_LEDGER)
        self.assertEqual(digests['bindings'], whitespace['bindings'])
        self.assertNotEqual(digests['source'], whitespace['source'])
        self.assertNotEqual(digests['package'], whitespace['package'])

    def test_identity_and_symbol_rename_preserve_unknown_metadata(self):
        old = deepcopy(self.ledger)
        changed = probe.rename_ledger(self.ledger, 'node_done', 'Completed')
        changed = probe.rename_ledger(changed, 'sym_complete', 'finished successfully')
        self.assertEqual(self.ledger, old)  # Inputs never partially modified.
        self.assertEqual(changed['annotations'], old['annotations'])
        self.assertEqual(changed['symbols'][0]['wire'], 'complete')
        self.assertEqual(probe.symbol_wire(changed, 'scope_terminal', 'outcome', 'finished successfully'), 'complete')
        with self.assertRaises(ValueError):
            probe.symbol_wire(changed, 'node_done', 'outcome', 'finished successfully')
        self.assertEqual(probe.resolve(changed, 'scope_terminal', 'Completed', 'node'), 'node_done')
        with self.assertRaises(ValueError):
            probe.resolve(changed, 'scope_terminal', 'Done', 'node')
        with self.assertRaises(ValueError):
            probe.rename_ledger(old, 'node_done', 'Reviewer')
        with self.assertRaises(ValueError):
            probe.rename_ledger(old, 'missing', 'Name')
        self.assertEqual(self.ledger, old)

    def test_duplicate_and_retired_identity_refusal(self):
        self.refuses(lambda x: x['retired'].append('node_done'))
        self.refuses(lambda x: x['declarations'].append(deepcopy(x['declarations'][1])))
        self.refuses(lambda x: x.update(definition='node_done'))
        self.refuses(lambda x: x['symbols'][0].update(id='node_done'))
        self.refuses(lambda x: x['declarations'][-1].update(name='Reviewer'))
        self.refuses(lambda x: x.update(extra=True))
        self.refuses(lambda x: x.update(profile='0.1.0'))
        self.refuses(lambda x: x.pop('contracts'))

    def test_lexical_lookup_tree_and_cross_kind_shadowing(self):
        rows = self.ledger['declarations']
        rows.append({'id': 'child', 'kind': 'scope', 'scope': 'scope_terminal', 'name': 'Child'})
        rows.append({'id': 'cell', 'kind': 'data', 'scope': 'child', 'name': 'Private'})
        self.assertEqual(probe.resolve(self.ledger, 'child', 'Reviewer', 'actor'), 'actor_reviewer')
        self.assertEqual(probe.resolve(self.ledger, 'child', 'Private', 'data', ('Child', 'Terminal example')), 'cell')
        for scope, name, kind, qualifiers in [('scope_terminal', 'Private', 'data', ()),
                                               ('child', 'Reviewer', 'node', ()),
                                               ('child', 'Private', 'data', ('Wrong',)),
                                               ('child', 'reviewer', 'actor', ())]:
            with self.assertRaises(ValueError):
                probe.resolve(self.ledger, scope, name, kind, qualifiers)
        self.refuses(lambda x: x['declarations'][-1].update(name='Reviewer'))
        self.refuses(lambda x: x['declarations'][-2].update(scope='child'))
        self.refuses(lambda x: x['declarations'][-1].update(scope=None))
        self.refuses(lambda x: x['declarations'][-1].update(scope='actor_reviewer'))

    def test_generated_slots_use_identity_not_positions(self):
        self.ledger['declarations'].append({'id': 'cell', 'kind': 'data', 'scope': 'scope_terminal', 'name': 'Result'})
        self.ledger['symbols'].append({'id': 'sym_purpose', 'owner': 'cell', 'role': 'purpose', 'name': 'Expense review', 'wire': 'expense_review'})
        self.assertEqual(probe.symbol_wire(self.ledger, 'cell', 'purpose', 'Expense review'), 'expense_review')
        self.ledger['symbols'].append({'id': 'sym_next', 'owner': 'node_done', 'role': 'outcome', 'name': 'correction needed', 'wire': 'correction_needed'})
        self.ledger['generated'] = [
            {'id': 'expr_initial', 'owner': 'cell', 'role': 'initial', 'key': None},
            {'id': 'expr_assignment', 'owner': 'node_done', 'role': 'assignment', 'key': 'cell'},
            {'id': 'flow_next', 'owner': 'node_done', 'role': 'flow', 'key': 'sym_next'}]
        probe.validate_ledger(self.ledger)
        renamed = probe.rename_ledger(self.ledger, 'sym_next', 'needs revision')
        self.assertEqual(renamed['generated'], self.ledger['generated'])
        self.refuses(lambda x: x['generated'].append(dict(x['generated'][0], id='another')))
        self.refuses(lambda x: x['generated'][2].update(key='sym_complete'))
        self.refuses(lambda x: x['generated'][1].update(key='actor_reviewer'))
        self.refuses(lambda x: x['symbols'][-1].update(wire='correction needed'))
        self.refuses(lambda x: x['symbols'].append(dict(x['symbols'][-1], id='another')))

    def test_population_wide_join_aliases_do_not_disguise_wire_collisions(self):
        for identity in ('finance', 'legal'):
            self.ledger['declarations'].append({'id': identity, 'kind': 'scope', 'scope': 'scope_terminal', 'name': identity})
            self.ledger['symbols'].append({'id': 'sym_' + identity, 'owner': identity, 'role': 'outcome', 'name': 'complete', 'wire': 'complete'})
        self.assertEqual(probe.join_tokens(self.ledger, {'finance', 'legal'}, {'sym_finance', 'sym_legal'}), {'complete'})
        with self.assertRaises(ValueError):
            probe.join_tokens(self.ledger, {'finance', 'legal'}, {'sym_finance'})
        with self.assertRaises(ValueError):
            probe.join_tokens(self.ledger, {'finance'}, {'sym_legal'})

    def test_copy_delete_and_recreate_identity_model(self):
        self.ledger['declarations'].append({'id': 'copied_done', 'kind': 'node', 'scope': 'scope_terminal', 'name': 'Done copy'})
        probe.validate_ledger(self.ledger)
        old_node = next(row for row in self.ledger['declarations'] if row['id'] == 'node_done')
        self.ledger['declarations'].remove(old_node)
        self.ledger['retired'].append('node_done')
        self.ledger['declarations'].append(dict(old_node, id='recreated_done'))
        probe.validate_ledger(self.ledger)
        self.assertEqual(probe.resolve(self.ledger, 'scope_terminal', 'Done', 'node'), 'recreated_done')
        self.refuses(lambda x: x['declarations'][-1].update(id='node_done'))

    def test_pins_are_exact_bytes_and_supplied_locally(self):
        raw = b'{"example":"not an admitted provider"}\n'
        digest = 'sha256:' + sha256(raw).hexdigest()
        self.ledger['declarations'].append({'id': 'res', 'kind': 'resource', 'scope': 'scope_terminal', 'name': 'Provider resource'})
        self.ledger['resources'].append({'binding': 'res', 'kind': 'provider', 'contract': 'urn:test:provider', 'digest': digest})
        supplied_contracts = {(pin['contract'], pin['digest']):
                              (probe.ROOT / 'docs/ir/contracts' / (pin['digest'].replace(':', '-') + '.txt')).read_bytes()
                              for pin in self.ledger['contracts'].values()}
        probe.verify_resources(self.ledger, supplied_contracts | {('urn:test:provider', digest): raw})
        for supplied in ({}, supplied_contracts | {('urn:test:provider', digest): raw.rstrip()}):
            with self.assertRaises(ValueError):
                probe.verify_resources(self.ledger, supplied)
        self.refuses(lambda x: x['resources'].clear())
        self.refuses(lambda x: x['resources'].append(dict(x['resources'][0])))
        self.refuses(lambda x: x['contracts']['core'].update(digest='sha256:' + '0' * 64))
        # Matching a digest is intentionally not provider semantic admission.

    def test_strict_companion_transport(self):
        for raw in (b'{"a":1,"a":2}', b'\xef\xbb\xbf{}', b'{"a":1.0}',
                    b'{"a":NaN}', b'{"a":-0}', b'{"a":9007199254740992}',
                    b'{"a":"\\ud800"}', b'{"a":"\xff"}', b'{} trailing',
                    b'[' * 65 + b'0' + b']' * 65):
            with self.subTest(raw=raw), self.assertRaises(ValueError):
                probe.strict_json(raw)

    def test_document_links_and_licenses(self):
        for relative in ('docs/authoring/profile.md', 'docs/authoring/bindings.md',
                         'docs/decisions/0015-authoring-grammar-bindings.md',
                         'docs/evaluation/0015-authoring-grammar-bindings.md'):
            path = probe.ROOT / relative
            text = path.read_text()
            self.assertIn('SPDX-License-Identifier: CC-BY-4.0', text)
            check_links(path)
        for relative in ('docs/authoring/candidate-a.ebnf',
                         'docs/authoring/examples/terminal.choreo',
                         'docs/authoring/examples/terminal.bindings.json.license',
                         'schemas/authoring/bindings-0.1.schema.json.license'):
            self.assertIn('SPDX-License-Identifier: MPL-2.0', (probe.ROOT / relative).read_text())


if __name__ == '__main__':
    unittest.main(verbosity=2)
