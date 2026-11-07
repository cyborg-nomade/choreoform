# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Independent declared-state/type oracle, never an observation acceptance test."""
from hashlib import sha256
import json
from pathlib import Path
import tempfile
import unittest

from check_static_contracts import ROOT,run

PIN='a224fc2b8318af2ff8dfc9955b55af03f4d5b7bd0de719de0384421698d20ead'
PROFILE={'id':'urn:choreoform:observation-source-profile:provider-terminal-facts:0.1.0','revision':'sha256:'+PIN}
BASE=ROOT/'docs/benchmarks/rp01'
CORPUS=ROOT/'crates/semantic-check/tests/conformance/terminal-observation-sources.json'
TAGS={'succeeded':'confirmed','rejected':'rejected','cancelled':'cancelled','no-effect':'noeffect'}
NONTERMINAL={'acknowledged','unknown'}
CORRELATION={'kind':'record','fields':{k:{'kind':'text'} for k in ['logical effect','request digest','observation']}}


def classify(states):
    if any(state not in set(TAGS)|NONTERMINAL for state in states.values()):return {'code':'provider-observation'}
    if 'succeeded' not in states.values():return {'code':'provider-outcomes'}
    terminal={name:TAGS[state] for name,state in states.items() if state in TAGS}
    return {'terminal':terminal,'cases':sorted(set(terminal.values()))}


def normalized_type(provider):
    states={name:o['state'] for name,o in provider['observations'].items()}
    result=classify(states)
    if 'code' in result:raise ValueError(result['code'])
    return {'kind':'variant','cases':{tag:provider['output'] if tag=='confirmed' else CORRELATION for tag in result['cases']}}


class TerminalObservationSourceEvidence(unittest.TestCase):
    def test_exact_profile_registry_licensing_and_required_checks(self):
        doc=ROOT/'docs/dialects/provider-terminal-observation-sources.md'
        raw=(ROOT/f'docs/ir/contracts/sha256-{PIN}.txt').read_bytes()
        self.assertEqual(sha256(raw).hexdigest(),PIN);self.assertEqual(raw,doc.read_bytes())
        self.assertIn(PROFILE['id'].encode(),raw)
        self.assertIn(PIN,(ROOT/'crates/semantic-check/src/resources/terminal_source.rs').read_text())
        self.assertIn('SPDX-License-Identifier: CC-BY-4.0',doc.read_text())
        self.assertEqual(Path(str(CORPUS)+'.license').read_text(),'SPDX-FileCopyrightText: 2026 Choreoform contributors\nSPDX-License-Identifier: MPL-2.0\n')
        self.assertIn('Provider terminal-observation source',(ROOT/'docs/ir/contracts/README.md').read_text())
        self.assertIn('tools/check_terminal_observation_sources.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())

    def test_twelve_independent_state_name_and_refusal_vectors(self):
        corpus=json.loads(CORPUS.read_bytes())
        self.assertEqual(corpus['profile'],PROFILE);self.assertEqual(len(corpus['cases']),12)
        self.assertEqual(len({c['id'] for c in corpus['cases']}),12)
        accepted=0
        for c in corpus['cases']:
            self.assertEqual(classify(c['states']),c['expect'],c['id'])
            accepted+='terminal' in c['expect']
        self.assertEqual(accepted,7)
        self.assertEqual(classify({'unknown':'succeeded','paid':'unknown'}),{'terminal':{'unknown':'confirmed'},'cases':['confirmed']})

    def test_exact_real_provider_type_matches_declared_benchmark_shape_without_binding(self):
        provider=json.loads((BASE/'payment.provider.json').read_bytes())
        candidate=json.loads((BASE/'candidate.ir.json').read_bytes())
        expected=normalized_type(provider)
        self.assertEqual(expected,candidate['body']['policies']['payment_wait']['body']['observationType'])
        self.assertEqual(expected['cases']['confirmed'],provider['output'])
        self.assertEqual(set(expected['cases']),{'confirmed','rejected','cancelled','noeffect'})
        self.assertEqual(expected['cases']['confirmed']['cases']['paid']['contract']['revision'],'sha256:'+sha256((BASE/'types.json').read_bytes()).hexdigest())
        self.assertNotIn('unknown',classify({n:d['state'] for n,d in provider['observations'].items()})['terminal'])
        self.assertTrue(provider['resultProtection']['purposes'])
        # This comparison does not bind a source or authenticate this wait/provider.
        self.assertNotIn('source',candidate['body']['policies']['payment_wait']['body'])

    def test_unrelated_valid_metadata_does_not_change_any_current_composed_report(self):
        m=json.loads((BASE/'manifest.json').read_bytes())
        supplied=[(r['id'],r['revision'],ROOT/r['path']) for r in m['resources']]
        provider=next({'id':r['id'],'revision':r['revision']} for r in m['resources'] if r['id']=='urn:choreoform:benchmarks:rp01:payment:0.1.0')
        source={'kind':'provider-terminal-observation-source','version':'0.1.0','id':'urn:test:unbound-terminal-source','profile':PROFILE,'provider':provider}
        with tempfile.TemporaryDirectory() as directory:
            p=Path(directory)/'source.json';raw=json.dumps(source).encode();p.write_bytes(raw)
            extra=[(source['id'],'sha256:'+sha256(raw).hexdigest(),p),(PROFILE['id'],PROFILE['revision'],ROOT/f'docs/ir/contracts/sha256-{PIN}.txt')]
            for stage in ['check-timing','check-fault-routes','check-exceptional-availability','check-reconciliation-links','check-reconciliation-availability']:
                before=run(stage,BASE/'source.choreo',BASE/'bindings.json',supplied)
                after=run(stage,BASE/'source.choreo',BASE/'bindings.json',supplied+extra)
                self.assertEqual(before.returncode,0,before.stderr);self.assertEqual(after.returncode,0,after.stderr)
                self.assertEqual(after.stdout,before.stdout);self.assertIs(json.loads(after.stdout)['executable'],False)
        # Reader tests validate this metadata separately. Extra registry bytes are inert.
        self.assertTrue(all(s['status']=='Partial' for s in m['scenarios']))


if __name__=='__main__':unittest.main()
