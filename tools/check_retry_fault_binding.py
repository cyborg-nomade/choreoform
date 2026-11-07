# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Independent core-0.4 shape/use rules; no fault acceptance or retry execution."""
from copy import deepcopy
from hashlib import sha256
import json
import re
from pathlib import Path
import tempfile
import unittest

from check_static_contracts import ROOT,run
from check_metadata_authoring import invoke
from benchmarks.rp01_faults import historical_v6

PIN='0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3'
BASE=ROOT/'docs/benchmarks/rp01'


def check_binding(retry,emissions,provider,site,idempotency='stable-key'):
    """Check the new shape/use rules atop prevalidated refs/source metadata."""
    if set(retry)!={'maxAttempts','delays','faults','clock','basis','faultSource'}: return 'fields'
    attempts=retry['maxAttempts']
    if type(attempts) is not int or not 1<=attempts<=1000: return 'retry-attempts'
    if len(retry['delays'])!=attempts-1: return 'retry-delays'
    if any(not re.fullmatch(r'0|[1-9][0-9]*',d) or int(d)>2**63-1 for d in retry['delays']): return 'retry-delay'
    if any(v is not True for v in retry['faults'].values()): return 'set'
    if attempts==1:
        if retry['faultSource'] is not None or retry['faults']: return 'retry-fault-source-unused'
        if retry['clock'] is not None or retry['basis'] is not None: return 'retry-binding-unused'
        return None
    if not retry['faults'] or retry['faultSource'] is None: return 'retry-fault-source-required'
    for name in retry['faults']:
        if name not in emissions: return 'retry-fault-source-name'
        if emissions[name]=='unresolved' and idempotency!='stable-key': return 'retry-fault-evidence-unproved'
    if retry['clock'] is None or retry['basis'] is None: return 'retry-binding-required'
    if site is not None:
        if site['mode']!='capability': return 'retry-fault-source-context-unproved'
        if site['provider']!=provider: return 'retry-fault-source-provider'
    return None


class RetryFaultBindingEvidence(unittest.TestCase):
    def test_exact_core_publication_inherited_pins_and_known_implementation(self):
        raw=(ROOT/f'docs/ir/contracts/sha256-{PIN}.txt').read_bytes()
        self.assertEqual(sha256(raw).hexdigest(),PIN)
        self.assertEqual(raw,(ROOT/'docs/dialects/core-fault-source-v0.4.md').read_bytes())
        self.assertIn(b'urn:choreoform:contracts:core-profile:0.4.0',raw)
        for old in ['26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58','5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623','30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3','f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0']:
            self.assertEqual(sha256((ROOT/f'docs/ir/contracts/sha256-{old}.txt').read_bytes()).hexdigest(),old)
            self.assertIn(old.encode(),raw)
        self.assertIn(PIN,(ROOT/'crates/semantic-check/src/lib.rs').read_text())

    def test_independent_closed_shape_names_evidence_and_exact_use_identity(self):
        provider={'id':'urn:test:provider','revision':'sha256:'+'0'*64}
        retry={'maxAttempts':2,'delays':['0'],'faults':{'no effect':True},'clock':provider,'basis':'basis','faultSource':provider}
        source={'no effect':'definitive-no-effect','unknown':'unresolved'}
        site={'mode':'capability','provider':provider}
        self.assertIsNone(check_binding(retry,source,provider,site))
        self.assertIsNone(check_binding(retry,source,provider,None)) # unused declaration, no actual use claim
        self.assertIsNone(check_binding(retry,source,provider,site,'none')) # still needs actual definitive evidence
        cases=[('missing','fields'),('null','retry-fault-source-required'),('empty','retry-fault-source-required'),('name','retry-fault-source-name'),('clock','retry-binding-required'),('unused','retry-fault-source-unused'),('human','retry-fault-source-context-unproved'),('provider_id','retry-fault-source-provider'),('provider_revision','retry-fault-source-provider'),('unknown_no_key','retry-fault-evidence-unproved')]
        for mutation,expected in cases:
            r,s=deepcopy(retry),deepcopy(site);key='stable-key'
            if mutation=='missing': del r['faultSource']
            elif mutation=='null': r['faultSource']=None
            elif mutation=='empty': r['faults']={}
            elif mutation=='name': r['faults']={'arbitrary':True}
            elif mutation=='clock': r['clock']=None
            elif mutation=='unused': r.update(maxAttempts=1,delays=[],clock=None,basis=None)
            elif mutation=='human': s['mode']='human'
            elif mutation=='provider_id': s['provider']['id']='urn:test:other'
            elif mutation=='provider_revision': s['provider']['revision']='sha256:'+'1'*64
            else: r['faults']={'unknown':True};key='none'
            self.assertEqual(check_binding(r,source,provider,s,key),expected,mutation)
        r=deepcopy(retry);r['faults']={'unknown':True}
        self.assertIsNone(check_binding(r,source,provider,site))

    def test_existing_source_core_pair_and_reports_do_not_implicitly_upgrade(self):
        ledger=json.loads((BASE/'bindings.json').read_bytes())
        source,ledger=historical_v6((BASE/'source.choreo').read_text(),ledger)
        resources=[(r['id'],r['revision'],(ROOT/r['path']).read_bytes()) for r in json.loads((BASE/'manifest.json').read_bytes())['resources']]
        self.assertEqual((ledger['profile'],ledger['version']),('0.6.0','0.5.0'))
        self.assertEqual(ledger['contracts']['core']['contract'],'urn:choreoform:contracts:core-profile:0.3.0')
        candidate=json.loads(invoke((source,ledger,resources)).stdout)
        self.assertTrue(all('faultSource' not in p['body']['retry'] for p in candidate['body']['policies'].values() if p['body']['kind']=='work'))
        old='For retries use clock "Expense clock", basis "Payable request".'
        self.assertEqual(source.count(old),1)
        source=source.replace(old,old[:-1]+', fault source "Undeclared source".')
        m=json.loads((BASE/'manifest.json').read_bytes())
        supplied=[(r['id'],r['revision'],ROOT/r['path']) for r in m['resources']]
        with tempfile.TemporaryDirectory() as directory:
            p=Path(directory)/'old-profile.choreo';p.write_text(source)
            b=Path(directory)/'legacy-bindings.json';b.write_text(json.dumps(ledger))
            result=run('lower',p,b,supplied)
        self.assertNotEqual(result.returncode,0);self.assertIn(b'syntax',result.stderr);self.assertEqual(result.stdout,b'')

    def test_licensing_registry_ci_and_no_claim_of_host_or_scenario_execution(self):
        self.assertIn('SPDX-License-Identifier: CC-BY-4.0',(ROOT/'docs/dialects/core-fault-source-v0.4.md').read_text())
        self.assertIn('Explicit retry fault-source binding',(ROOT/'docs/ir/contracts/README.md').read_text())
        self.assertIn('tools/check_retry_fault_binding.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())
        m=json.loads((BASE/'manifest.json').read_bytes());self.assertIs(m['executable'],False)
        self.assertTrue(all(s['status']=='Partial' for s in m['scenarios']))


if __name__=='__main__': unittest.main()
