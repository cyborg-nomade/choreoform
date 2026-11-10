# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Conditional declared isolation, never actual ownership or process execution."""
import json
from pathlib import Path
import tempfile
import unittest

from check_closed_provenance import package, invoke
from benchmarks.rp01_graph import foundations
from benchmarks.rp01_source import Package, Scope, CORE3
from benchmarks.rp01_resources import clock_resources
from check_static_contracts import ROOT, run

STAGES = ['check-concurrency','check-timing','check-fault-routes',
          'check-exceptional-availability','check-reconciliation-links',
          'check-reconciliation-availability']
# Explicit historical fixture; current adoption is checked by check_rp01_capture_checkpoint.
BASE=ROOT/'docs/benchmarks/rp01/history/source-0.7'


def dense(calls=230,waits=100):
    p=Package()
    p.ledger.update(profile='0.6.0',version='0.5.0',root='root',definition='isolated_report_budget')
    p.ledger['contracts']['core']=CORE3.copy()
    root=Scope(p,'root','Isolated report',None,'Call 0')
    foundations(root)
    for i in range(calls):
        root.step(f'call{i}',f'Call {i}','Run "Worker" with inputs empty and outputs empty.', [('complete',f'Call {i+1}' if i+1<calls else 'Done')])
    root.step('done','Done','Finish as "complete".')
    worker=Scope(p,'worker','Worker',root,'Worker done',closure='Keep worker')
    worker.policy('keep','closure','Keep worker','Reconcile in "Recovery".')
    worker.step('worker_done','Worker done','Finish as "complete".')
    recovery=Scope(p,'recovery','Recovery',worker,'Await 0')
    for i in range(waits):
        recovery.policy(f'wait_policy{i}','wait',f'Observation {i}',f'Observe text. Match when (true). Authority "Payment operator". Timer none. On observation outcome "observed" of "Await {i}". On timer none.')
        recovery.step('wait_'+('x'*38)+f'{i:04}',f'Await {i}',f'Wait under "Observation {i}".', [('observed',f'Await {i+1}' if i+1<waits else 'Recovery done')])
    recovery.step('recovery_done','Recovery done','Finish as "complete".')
    worker.child(recovery)
    root.child(worker)
    return 'Choreoform authoring "0.6.0".\n'+root.render(),p.ledger


class IsolatedSurvivorsEvidence(unittest.TestCase):
    def test_isolated_invoke_repeat_and_all_composed_reports_disclose_host_premises(self):
        for mode in ['invoke','repeat']:
            source,ledger=package(mode,survivor='readonly')
            expected=None
            for stage in STAGES:
                result=invoke(source,ledger,stage)
                self.assertEqual(result.returncode,0,result.stderr)
                report=json.loads(result.stdout)
                self.assertIs(report['executable'],False)
                records=report['isolatedFollowups']
                self.assertEqual(len(records),1)
                p=records[0]
                self.assertEqual((p['creator'],p['body'],p['closure'],p['followup'],p['waits']),('run','worker','keep_worker','recovery',['observe']))
                self.assertIn('fresh-authority-and-accepted-atomic-single-owner-handoff',p['hostRequirements'])
                self.assertIn('retain-all-transitive-obligations-and-subscriptions-until-settled',p['hostRequirements'])
                if expected is None: expected=records
                self.assertEqual(records,expected)

    def test_shared_reads_finish_only_and_parent_writers_keep_their_refusals(self):
        source,ledger=package(survivor='readonly')
        for s in [source.replace('Observe text. Match when (true).','Observe text. Match when ((the value of "Seed") equals (0)).'),source.replace('Start at "Observe".','Start at "Recovery done".')]:
            self.assertEqual(invoke(s,ledger).returncode,0)
            result=invoke(s,ledger,'check-concurrency')
            self.assertIn(b'scope-lifetime-unproved at /body/nodes/run; source bytes ',result.stderr)
            self.assertEqual(result.stdout,b'')
        source,ledger=package(survivor='writer')
        result=invoke(source,ledger,'check-concurrency')
        self.assertIn(b'invalidation-context-unproved',result.stderr)
        self.assertEqual(result.stdout,b'')

    def test_exact_format_and_revision_cycles_preserve_conditional_evidence(self):
        source,ledger=package(survivor='readonly')
        old=json.loads(invoke(source,ledger,'lower').stdout)
        report=json.loads(invoke(source,ledger,'check-concurrency').stdout)
        formatted=json.loads(invoke(source,ledger,'format').stdout)
        s,l=formatted['source'],json.loads(formatted['companion'])
        self.assertEqual(json.loads(invoke(s,l,'lower').stdout),old)
        self.assertEqual(json.loads(invoke(s,l,'check-concurrency').stdout),report)

    def test_real_rp01_retains_three_owned_followups_without_host_or_scenario_admission(self):
        m=json.loads((BASE/'manifest.json').read_bytes())
        supplied=[(r['id'],r['revision'],ROOT/r['path']) for r in m['resources']]
        for stage in STAGES:
            result=run(stage,BASE/'source.choreo',BASE/'bindings.json',supplied)
            self.assertEqual(result.returncode,0,result.stderr)
            report=json.loads(result.stdout)
            self.assertIs(report['executable'],False)
            records=report['isolatedFollowups']
            self.assertEqual({r['creator'] for r in records},{'run_checks','run_payment','run_notice'})
            self.assertTrue(all(r['followup']==r['body'].replace('_service','_recovery') for r in records))
        self.assertTrue(all(s['status']=='Partial' for s in m['scenarios']))

    def test_reused_templates_charge_and_bound_native_evidence_before_stdout(self):
        source,ledger=dense()
        lower=invoke(source,ledger,'lower')
        self.assertEqual(lower.returncode,0,lower.stderr)
        result=invoke(source,ledger,'check-concurrency')
        self.assertEqual(result.stdout,b'')
        self.assertIn(b'report-budget',result.stderr)
        self.assertNotEqual(result.returncode,0)

    def test_a_valid_timed_followup_still_refuses_the_timer_free_subset(self):
        source,ledger=package(survivor='readonly')
        resources=clock_resources()
        clock=resources['clock'][2]
        ledger['declarations'].append({'id':'clock','kind':'resource','scope':'root','name':'Expense clock'})
        ledger['resources'].append({'binding':'clock','kind':'clock','contract':clock['id'],'digest':clock['revision']})
        target='"Observe" in "Recovery" in "Worker" in "Snapshot example"'
        source=source.replace('Timer none.','Timer clock "Expense clock", basis "Seed", due (0), calendar none, pause authority "Keep the deadline fixed".').replace('On timer none.','On timer outcome "elapsed" of '+target+'.')
        source=source.replace('Wait under "Observe policy". On "observed", continue with "Recovery done".','Wait under "Observe policy". On "observed", continue with "Recovery done". On "elapsed", continue with "Recovery done".')
        ledger['symbols'].append({'id':'observe_outcome_elapsed','owner':'observe','role':'outcome','name':'elapsed','wire':'elapsed'})
        ledger['generated'].append({'id':'observe_flow_elapsed','owner':'observe','role':'flow','key':'observe_outcome_elapsed'})
        with tempfile.TemporaryDirectory() as directory:
            s,b=Path(directory)/'timed.choreo',Path(directory)/'bindings.json'
            s.write_text(source)
            b.write_text(json.dumps(ledger))
            supplied=[]
            for name,(_,raw,pin) in resources.items():
                path=Path(directory)/(name+'.json')
                path.write_bytes(raw)
                supplied.append((pin['id'],pin['revision'],path))
            profile=resources['clock'][0]['profile']
            supplied.append((profile['id'],profile['revision'],ROOT/'docs/ir/contracts'/(profile['revision'].replace(':','-')+'.txt')))
            prior=run('check-invalidation',s,b,supplied)
            self.assertEqual(prior.returncode,0,prior.stderr)
            result=run('check-concurrency',s,b,supplied)
        self.assertIn(b'scope-lifetime-unproved at /body/nodes/run',result.stderr)
        self.assertEqual(result.stdout,b'')


if __name__=='__main__': unittest.main()
