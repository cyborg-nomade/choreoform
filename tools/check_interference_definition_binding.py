# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Independent publication/source compatibility evidence; raw selection is SDK-only."""
from copy import deepcopy
from hashlib import sha256
import json
from pathlib import Path
import unittest
from check_authoring_evidence import check_links
from check_metadata_authoring import invoke
from check_source_capacity import fixed
from proofs.parallel_effects import package

ROOT=Path(__file__).resolve().parents[1]
ID='urn:choreoform:contracts:core-profile:0.13.0'
HASH='dd8667135d1c801683d48d8410867b37d5d6f71c7016b8b716016bce92e77428'
ARTIFACT=ROOT/'docs/ir/contracts'/f'sha256-{HASH}.txt'
FIXTURE=ROOT/'crates/semantic-check/tests/interference/parallel-effects.ir.json'


class InterferenceBindingEvidence(unittest.TestCase):
    def test_exact_publication_before_code_and_twelve_inherited_core_pins(self):
        source=(ROOT/'docs/dialects/core-interference-binding-v0.13.md').read_bytes()
        self.assertEqual(source,ARTIFACT.read_bytes())
        self.assertEqual(sha256(source).hexdigest(),HASH)
        self.assertEqual(source.count(b'- `urn:choreoform:contracts:core-profile:'),12)
        self.assertIn(HASH,(ROOT/'crates/semantic-check/src/lib.rs').read_text())
        self.assertIn(HASH,(ROOT/'docs/ir/contracts/README.md').read_text())
        self.assertEqual(sha256((ROOT/'docs/dialects/provider-interference-regions.md').read_bytes()).hexdigest(),'0b10eb4dbb19caace45b93f35dec44aa5f26458d631393932caddb5589210250')

    def test_real_native_c12_lowering_matches_captured_fixture_and_unused_core_does_not_select(self):
        before=fixed();p=package('compensatable','compensatable')
        lower=invoke(p);self.assertEqual(lower.returncode,0,lower.stderr)
        self.assertEqual(lower.stdout,FIXTURE.read_bytes())
        extra=p[0],p[1],[*p[2],(ID,'sha256:'+HASH,ARTIFACT.read_bytes())]
        unchanged=invoke(extra);self.assertEqual(unchanged.returncode,0,unchanged.stderr)
        self.assertEqual(unchanged.stdout,lower.stdout)
        for wrapper in json.loads(lower.stdout)['body']['policies'].values():
            if wrapper['body']['kind']=='capability':self.assertNotIn('interference',wrapper['body'])
        for candidate in [p,extra]:
            r=invoke(candidate,'check-concurrency');self.assertEqual(r.returncode,1);self.assertEqual(r.stdout,b'')
            self.assertTrue(r.stderr.startswith(b'parallel-effects-unproved at /body/nodes/fork;'))
        self.assertEqual(fixed(),before)

    def test_current_source_and_companion_cannot_select_new_core_even_when_supplied(self):
        p=package('compensatable','compensatable');ledger=deepcopy(p[1])
        ledger['contracts']['core'].update(contract=ID,digest='sha256:'+HASH)
        extras=[*p[2],(ID,'sha256:'+HASH,ARTIFACT.read_bytes())]
        for stage in ['lower','check','check-concurrency']:
            r=invoke((p[0],ledger,extras),stage);self.assertEqual(r.returncode,1);self.assertEqual(r.stdout,b'')
            self.assertTrue(r.stderr.startswith(b'contract-pin'),r.stderr)

    def test_documentation_licensing_and_required_ci_command(self):
        for name in ['docs/decisions/0081-interference-definition-binding.md','docs/evaluation/0081-interference-binding-critique.md','docs/evaluation/0081-interference-binding-publication.md','docs/evaluation/0081-interference-definition-binding.md','docs/decisions/README.md','crates/semantic-check/tests/interference/README.md']:
            check_links(ROOT/name)
        self.assertIn('SPDX-License-Identifier: MPL-2.0',FIXTURE.with_name(FIXTURE.name+'.license').read_text())
        self.assertIn('SPDX-License-Identifier: CC-BY-4.0',ARTIFACT.read_text())
        self.assertIn('tools/check_interference_definition_binding.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__=='__main__':unittest.main()
