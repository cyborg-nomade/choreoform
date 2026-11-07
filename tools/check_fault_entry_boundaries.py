# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Actual tool boundary judgments, not faults, observations or handler runs."""
from copy import deepcopy
import json
import unittest

import check_ir_fixtures as wire
from check_late_observation_boundaries import with_handler
from check_metadata_authoring import invoke
from check_static_contracts import ROOT


def handler_local(port=False):
    s,l,r=with_handler()
    l['declarations'] += [{'id':'fault_type_cell','kind':'data','scope':'remaining','name':'Fault type'},{'id':'fault_type_guard','kind':'expression','scope':'remaining','name':'Recognised fault type'},{'id':'fault_finish','kind':'node','scope':'remaining','name':'Finish handler'}]
    l['symbols'].append({'id':'fault_purpose','owner':'fault_type_cell','role':'purpose','name':'Reimbursement','wire':'reimbursement'})
    data='Keep "Fault type" as text under "Expense access". Sensitivity "expense". Purposes ["Reimbursement"]. Participants ["Payment operator"]. Capabilities empty. Invalidate ["Release remaining owner"]. Initially unavailable. End data. Let "Recognised fault type" mean ((the value of "Fault type") equals ("provider unknown")) as truth value with parameters empty. '
    before='Step "Release remaining owner". Finish as "complete".  End step.'
    after=data+'Step "Release remaining owner". Choose by ["known" when "Recognised fault type"]; otherwise "other". On "known", continue with "Finish handler". On "other", continue with "Finish handler". End step. Step "Finish handler". Finish as "complete". End step.'
    assert s.count(before)==1;s=s.replace(before,after)
    for name in ['known','other']:
        identity='remaining_done_outcome_'+name
        l['symbols'].append({'id':identity,'owner':'remaining_done','role':'outcome','name':name,'wire':name})
        l['generated'].append({'id':'fault_flow_'+name,'owner':'remaining_done','role':'flow','key':identity})
    if port:
        start=s.index('Scope "Remaining work"');end=s.index('End scope.',start);block=s[start:end].replace('Inputs empty.','Inputs ["Fault" uses "Fault type"].')
        s=s[:start]+block+s[end:]
        l['symbols'].append({'id':'fault_input','owner':'remaining','role':'input','name':'Fault','wire':'fault'})
        # Isolate the handler-port gate from the independently forbidden P6 ports.
        s=s.replace('Rule for closure "Preserve remaining work". Reconcile in "Remaining work".','Rule for closure "Preserve remaining work". Fully terminal.')
    return s,l,r


class FaultEntryBoundaries(unittest.TestCase):
    def test_fault_history_cannot_leak_through_ordinary_policy_facts(self):
        p=with_handler()
        for fact in ['fault','cause','sourceOccurrence','inputRevisions']:
            s,l,r=deepcopy(p);old='Let "Another review is needed" mean (the value of "Review needed")';assert s.count(old)==1
            s=s.replace(old,f'Let "Another review is needed" mean (the fact "{fact}")')
            result=invoke((s,l,r),'check');self.assertEqual(result.returncode,1);self.assertEqual(result.stdout,b'');self.assertIn(b'policy-fact at ',result.stderr)

    def test_fresh_handler_local_and_regular_port_do_not_bind_runtime_fault_history(self):
        p=handler_local();result=invoke(p,'check-fault-routes');self.assertEqual(result.returncode,0,result.stderr)
        result=invoke(p,'check-exceptional-availability');self.assertEqual(result.returncode,1);self.assertEqual(result.stdout,b'');self.assertIn(b'exceptional-availability-unproved at /body/nodes/remaining_done',result.stderr)
        p=handler_local(port=True);result=invoke(p,'check');self.assertEqual(result.returncode,1);self.assertEqual(result.stdout,b'');self.assertIn(b'handler-ports at ',result.stderr)

    def test_exact_name_routes_are_not_emission_or_source_receipts(self):
        s,l,r=with_handler();s=s.replace('"provider unknown" by "Remaining work"','"unrelated declared fault" by "Remaining work"')
        result=invoke((s,l,r),'check-fault-routes');self.assertEqual(result.returncode,0,result.stderr);report=json.loads(result.stdout)
        row,=report['checkedHandlerSelections'];self.assertEqual(row,{'fault':'unrelated declared fault','handler':'remaining','owner':'expense','policy':'provider_faults'})
        self.assertIs(report['executable'],False);self.assertIn('authenticated-fault-source-occurrence-cause-and-input-revisions',report['remaining'])
        self.assertTrue(all(set(row)=={'fault','handler','owner','policy'} for row in report['checkedHandlerSelections']))

    def test_retry_binding_is_not_general_single_attempt_fault_emission_binding(self):
        s,l,r=with_handler();start=s.index('Rule for work "Notice work".');end=s.index('End policy.',start)
        before=s[start:end];assert 'Attempts 1.' in before
        after=before.replace('Retry faults empty.','Retry faults empty. For retries use clock "Expense clock", basis "Notice request", fault source "Payment faults".')
        s=s[:start]+after+s[end:];result=invoke((s,l,r),'lower')
        self.assertEqual(result.returncode,1);self.assertEqual(result.stdout,b'');self.assertIn(b'retry-binding at ',result.stderr)

    def test_old_and_new_wire_schema_scope_records_stay_closed(self):
        import jsonschema
        original=json.loads((ROOT/'docs/benchmarks/rp01/candidate.ir.json').read_bytes())
        for version,schema in [('0.1.0',wire.SCHEMA),('0.2.0',wire.SCHEMA_V2)]:
            d=deepcopy(original);d['version']=version
            if version=='0.2.0':
                for n in d['body']['nodes'].values():
                    if n['kind']=='wait':n['capture']=None
            jsonschema.Draft202012Validator(schema).validate(d)
            for value in [None,'payment_result',{'result':'payment_result'}]:
                changed=deepcopy(d);changed['body']['scopes']['remaining']['faultInput']=value
                with self.assertRaises(jsonschema.ValidationError):jsonschema.Draft202012Validator(schema).validate(changed)


    def test_unresolved_fault_metadata_does_not_make_an_unknown_provider_state_terminal(self):
        from check_terminal_observation_sources import normalized_type
        base=ROOT/'docs/benchmarks/rp01'
        provider=json.loads((base/'payment.provider.json').read_bytes());source=json.loads((base/'payment.fault-source.json').read_bytes())
        unknown=source['emissions']['provider unknown'];self.assertEqual(unknown['evidence'],'unresolved')
        self.assertEqual(set(unknown['observations']),{'unknown'});self.assertEqual(provider['observations']['unknown']['state'],'unknown')
        terminal=normalized_type(provider);self.assertNotIn('unknown',terminal['cases']);self.assertNotIn('provider unknown',terminal['cases'])
        self.assertEqual(set(terminal['cases']['confirmed']['cases']),{'paid','problem'})


if __name__=='__main__':unittest.main()
