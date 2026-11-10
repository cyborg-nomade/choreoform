# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Actual declaration-boundary probes, not late observations or process execution."""
from copy import deepcopy
import json
import unittest

from check_closed_provenance import package as isolated
from check_metadata_authoring import invoke
from check_static_contracts import ROOT

# Explicit historical fixture; current adoption is checked by check_rp01_capture_checkpoint.
BASE=ROOT/'docs/benchmarks/rp01/history/source-0.7'


def package():
    m=json.loads((BASE/'manifest.json').read_bytes())
    return ((BASE/'source.choreo').read_text(),json.loads((BASE/'bindings.json').read_bytes()),
            [(r['id'],r['revision'],(ROOT/r['path']).read_bytes()) for r in m['resources']])


def with_local_storage():
    source,ledger=isolated(survivor='readonly')
    ledger['declarations'].append({'id':'recovery_local','kind':'data','scope':'recovery','name':'Local observation'})
    ledger['generated'].append({'id':'recovery_local_initial','owner':'recovery_local','role':'initial','key':None})
    ledger['symbols'].append({'id':'recovery_local_purpose','owner':'recovery_local','role':'purpose','name':'Reimbursement','wire':'reimbursement'})
    data='Keep "Local observation" as truth value under "Expense access". Sensitivity "expense". Purposes ["Reimbursement"]. Participants ["Payment operator"]. Capabilities empty. Invalidate empty. Initially (false). End data.\n'
    assert source.count('Step "Observe".')==1
    return source.replace('Step "Observe".',data+'Step "Observe".',1),ledger,[]


def with_handler():
    source,ledger,resources=package();ledger=deepcopy(ledger)
    ledger['declarations'].append({'id':'provider_faults','kind':'policy','scope':'expense','name':'Provider faults'})
    source=source.replace('faults "Unhandled faults"','faults "Provider faults"',1)
    source=source.replace('Rule for faults "Unhandled faults".','Rule for faults "Provider faults". Handle ["provider unknown" by "Remaining work"]. End policy.\nRule for faults "Unhandled faults".',1)
    return source,ledger,resources


class LateObservationBoundaryEvidence(unittest.TestCase):
    def test_current_wait_has_no_capture_and_new_spelling_refuses_without_output(self):
        candidate=json.loads((BASE/'candidate.ir.json').read_bytes())
        node=candidate['body']['nodes']['payment_observe']
        self.assertEqual(node['writes'],{});self.assertNotIn('result',node)
        source,ledger,resources=package()
        self.assertEqual(source.count('Wait under "Payment reconciliation".'),1)
        source=source.replace('Wait under "Payment reconciliation".','Wait under "Payment reconciliation" and record in "Recorded payment result".')
        result=invoke((source,ledger,resources))
        self.assertEqual(result.returncode,1);self.assertEqual(result.stdout,b'');self.assertIn(b'syntax at ',result.stderr)

    def test_ordinary_expression_cannot_leak_policy_payload_facts(self):
        source,ledger,resources=package()
        old='Let "Another review is needed" mean (the value of "Review needed")'
        self.assertEqual(source.count(old),1)
        result=invoke((source.replace(old,'Let "Another review is needed" mean (the fact "payload")'),ledger,resources),'check')
        self.assertEqual(result.returncode,1);self.assertEqual(result.stdout,b'');self.assertIn(b'policy-fact at ',result.stderr)

    def test_storage_is_not_a_surviving_lifetime_proof(self):
        p=with_local_storage()
        for stage in ['check','check-control','check-invalidation']:
            result=invoke(p,stage);self.assertEqual(result.returncode,0,result.stderr)
            self.assertIs(json.loads(result.stdout)['executable'],False)
        result=invoke(p,'check-concurrency')
        self.assertEqual(result.returncode,1);self.assertEqual(result.stdout,b'')
        self.assertIn(b'scope-lifetime-unproved at /body/nodes/run; source bytes ',result.stderr)

    def test_recovery_cannot_jump_back_into_an_ordinary_parent_path(self):
        source,ledger,resources=package()
        old='On "observed", continue with "Finish Payment recovery".'
        self.assertEqual(source.count(old),1)
        result=invoke((source.replace(old,'On "observed", continue with "Record paid status".'),ledger,resources))
        self.assertEqual(result.returncode,1);self.assertEqual(result.stdout,b'');self.assertEqual(result.stderr.strip(),b'flow-scope at 0..0')

    def test_handler_selection_and_finish_supply_no_fault_data_or_ordinary_result(self):
        p=with_handler();result=invoke(p,'check-fault-routes')
        self.assertEqual(result.returncode,0,result.stderr)
        report=json.loads(result.stdout);self.assertIs(report['executable'],False)
        self.assertEqual(report['checkedHandlerSelections'],[{'fault':'provider unknown','handler':'remaining','owner':'expense','policy':'provider_faults'}])
        candidate=json.loads(invoke(p).stdout);handler=candidate['body']['scopes']['remaining']
        self.assertEqual(handler['inputs'],{});self.assertEqual(handler['outputs'],{})
        finish=candidate['body']['nodes']['remaining_done']
        self.assertEqual((finish['kind'],finish['outcome'],finish['writes']),('finish','complete',{}))
        self.assertEqual(finish['outcomes'],{})
        self.assertFalse(any(f['source']=='remaining_done' for f in candidate['body']['flows'].values()))
        result=invoke(p,'check-reconciliation-availability')
        self.assertEqual(result.returncode,0,result.stderr);self.assertIs(json.loads(result.stdout)['executable'],False)
        # Passing partial judgments do not synthesize a fault port or continue a payment.
        self.assertTrue(json.loads(result.stdout)['remaining'])


if __name__=='__main__':unittest.main()
