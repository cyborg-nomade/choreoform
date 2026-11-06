# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Independent declared-state mapping oracle; never trusted fault acceptance."""
from copy import deepcopy
from hashlib import sha256
import json
from pathlib import Path
import tempfile
import unittest

import check_ir_fixtures as wire
from check_static_contracts import ROOT,run

CORPUS=ROOT/'crates/semantic-check/tests/conformance/provider-fault-sources.json'
BASE=ROOT/'docs/benchmarks/rp01'
DOC=ROOT/'docs/dialects/provider-fault-sources.md'


def classify(emissions,extras,provider):
    if extras: return {'code':'fields'}
    if not emissions: return {'code':'fault-source-emissions'}
    used=set()
    allowed={'definitive-no-effect':{'rejected','cancelled','no-effect'},'unresolved':{'unknown'}}
    for fault,e in sorted(emissions.items()):
        if set(e)!={'observations','evidence'}: return {'code':'fields'}
        if e['evidence'] not in allowed: return {'code':'fault-source-evidence'}
        if any(flag is not True for flag in e['observations'].values()): return {'code':'set'}
        if not e['observations']: return {'code':'fault-source-observations'}
        for name in sorted(e['observations']):
            if name not in provider['observations']: return {'code':'fault-source-observation'}
            if provider['observations'][name]['state'] not in allowed[e['evidence']]: return {'code':'fault-source-evidence'}
            if name in used: return {'code':'fault-source-ambiguous'}
            used.add(name)
    return {'accepted':True}


class ProviderFaultSourceEvidence(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.corpus=wire.load(CORPUS.read_bytes())
        cls.provider=wire.load((BASE/'payment.provider.json').read_bytes())

    def test_exact_profile_snapshot_pins_closed_catalogue_and_original_provider_bytes(self):
        v=self.corpus
        self.assertEqual(set(v),{'format','version','profile','provider','cases'})
        self.assertEqual((v['format'],v['version']),('choreoform-provider-fault-source-vectors','0.1.0'))
        self.assertEqual(v['profile']['id'],'urn:choreoform:fault-source-profile:provider-observations:0.1.0')
        raw=DOC.read_bytes();revision='sha256:'+sha256(raw).hexdigest()
        self.assertEqual(v['profile']['revision'],revision)
        self.assertEqual((ROOT/'docs/ir/contracts'/(revision.replace(':','-')+'.txt')).read_bytes(),raw)
        reader=(ROOT/'crates/semantic-check/src/resources/fault_source.rs').read_text()
        self.assertIn(revision,reader)
        self.assertEqual(v['provider'],{'id':self.provider['id'],'revision':'sha256:'+sha256((BASE/'payment.provider.json').read_bytes()).hexdigest()})
        self.assertEqual(len(v['cases']),16)
        self.assertEqual(len({c['id'] for c in v['cases']}),16)

    def test_independent_state_and_ambiguity_oracle_matches_every_vector(self):
        accepted=0
        for c in self.corpus['cases']:
            self.assertEqual(set(c),{'id','emissions','extras','expect'})
            result=classify(c['emissions'],c['extras'],self.provider)
            self.assertEqual(result,c['expect'],c['id'])
            accepted+=bool(result.get('accepted'))
        self.assertEqual(accepted,5)
        # Observation state, not a safe-looking fault identity, determines class.
        misleading=next(c for c in self.corpus['cases'] if c['id']=='misleading_name_stays_unresolved')
        self.assertEqual(misleading['emissions']['confirmed no effect']['evidence'],'unresolved')
        mutated=deepcopy(misleading['emissions'])
        mutated['confirmed no effect']['evidence']='definitive-no-effect'
        self.assertEqual(classify(mutated,{},self.provider),{'code':'fault-source-evidence'})

    def test_supplied_mapping_metadata_does_not_implicitly_bind_existing_retry_policies(self):
        m=json.loads((BASE/'manifest.json').read_bytes())
        supplied=[(r['id'],r['revision'],ROOT/r['path']) for r in m['resources']]
        before=run('check-timing',BASE/'source.choreo',BASE/'bindings.json',supplied)
        self.assertEqual(before.returncode,0,before.stderr)
        with tempfile.TemporaryDirectory() as directory:
            source=Path(directory)/'source.json'
            doc={'kind':'provider-fault-source','version':'0.1.0','id':'urn:test:extra-fault-source','profile':self.corpus['profile'],'provider':self.corpus['provider'],'emissions':{'confirmed no effect':{'observations':{'no-effect':True},'evidence':'definitive-no-effect'}}}
            raw=(json.dumps(doc)+'\n').encode();source.write_bytes(raw)
            profile=self.corpus['profile'];profile_path=ROOT/'docs/ir/contracts'/(profile['revision'].replace(':','-')+'.txt')
            supplied += [(doc['id'],'sha256:'+sha256(raw).hexdigest(),source),(profile['id'],profile['revision'],profile_path)]
            after=run('check-timing',BASE/'source.choreo',BASE/'bindings.json',supplied)
        self.assertEqual(after.returncode,0,after.stderr)
        self.assertEqual(json.loads(after.stdout),json.loads(before.stdout))
        self.assertIs(json.loads(after.stdout)['executable'],False)

    def test_licensing_registry_and_required_ci_keep_the_metadata_boundary_visible(self):
        self.assertEqual(Path(str(CORPUS)+'.license').read_text(),'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n')
        self.assertIn('SPDX-License-Identifier: CC-BY-4.0',DOC.read_text())
        self.assertIn('Provider-observation fault-source',(ROOT/'docs/ir/contracts/README.md').read_text())
        self.assertIn('tools/check_provider_fault_sources.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__=='__main__': unittest.main()
