# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Artifact/integration boundaries; raw SDK cases remain Rust evidence."""
from hashlib import sha256
import re
import unittest

from check_static_contracts import ROOT
from check_compensation_references import package, refusal
from check_finite_work_authoring import package as small
from check_source_capacity import promote, fixed
from check_metadata_authoring import invoke

CONTRACT = ROOT/'docs/dialects/core-compensation-identity-v0.12.md'
IDENTITY = 'urn:choreoform:contracts:core-profile:0.12.0'


class CompensationIdentityEvidence(unittest.TestCase):
    def test_exact_artifact_eleven_bases_library_and_registry(self):
        raw=CONTRACT.read_bytes(); revision='sha256:'+sha256(raw).hexdigest()
        self.assertEqual((ROOT/'docs/ir/contracts'/(revision.replace(':','-')+'.txt')).read_bytes(),raw)
        self.assertIn(revision,(ROOT/'crates/semantic-check/src/lib.rs').read_text())
        self.assertIn(revision.replace(':','-'),(ROOT/'docs/ir/contracts/README.md').read_text())
        bases=re.findall(rb'- `(urn:choreoform:contracts:core-profile:[0-9.]+)` / `(sha256:[0-9a-f]{64})`\.',raw)
        self.assertEqual(len(bases),11)
        self.assertEqual({identity.decode() for identity,_ in bases},{f'urn:choreoform:contracts:core-profile:0.{n}.0' for n in range(1,12)})
        for identity,pin in bases:
            artifact=(ROOT/'docs/ir/contracts'/(pin.decode().replace(':','-')+'.txt')).read_bytes()
            self.assertEqual(pin.decode(),'sha256:'+sha256(artifact).hexdigest(),identity)

    def test_old_readable_profile_still_refuses_private_prior_identity_and_new_core(self):
        reference='"Create payable" in "Root" in "Main"'
        refusal(package(reference),'reference-missing',reference)
        s,l,r=promote(small()); l['contracts']['core'].update(contract=IDENTITY,digest='sha256:'+sha256(CONTRACT.read_bytes()).hexdigest())
        result=invoke((s,l,r)); self.assertEqual(result.returncode,1);self.assertEqual(result.stdout,b'')
        self.assertEqual(result.stderr,b'contract-pin at 0..0\n')

    def test_scope_licensing_ci_and_fixed_benchmark_bytes(self):
        before=fixed(); result=invoke(package());self.assertEqual(result.returncode,0,result.stderr)
        self.assertEqual(fixed(),before)
        raw=CONTRACT.read_text()
        self.assertIn('SPDX-License-Identifier: CC-BY-4.0',raw)
        self.assertIn('must still name exactly its immediate incoming source',raw)
        self.assertIn('NEW logical effect and key',raw)
        self.assertIn('tools/check_compensation_identity.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__=='__main__':unittest.main()
