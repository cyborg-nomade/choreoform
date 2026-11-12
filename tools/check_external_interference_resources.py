# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Independent artifact/schema/evidence checks, not a metadata/host evaluator."""
from copy import deepcopy
from hashlib import sha256
import json
from pathlib import Path
import unittest
import jsonschema
from check_authoring_evidence import check_links
from check_metadata_authoring import invoke
from check_source_capacity import fixed
from proofs.parallel_effects import package

ROOT=Path(__file__).resolve().parents[1]
ID='urn:choreoform:interference-profile:provider-regions:0.1.0'
DIGEST='0b10eb4dbb19caace45b93f35dec44aa5f26458d631393932caddb5589210250'
PROFILE={'id':ID,'revision':'sha256:'+DIGEST}
FOLDER=ROOT/'docs/dialects/interference-examples'
SOURCE=ROOT/'docs/dialects/provider-interference-regions.md'
ARTIFACT=ROOT/'docs/ir/contracts'/('sha256-'+DIGEST+'.txt')
SCHEMAS={name:jsonschema.Draft202012Validator(json.loads((ROOT/'schemas/resources'/f'{name}-0.1.schema.json').read_bytes())) for name in ['external-interference','external-region-namespace']}


def digest(raw):return 'sha256:'+sha256(raw).hexdigest()


class InterferenceResourceEvidence(unittest.TestCase):
    def test_exact_signed_profile_publication_and_immutable_draft(self):
        self.assertEqual(SOURCE.read_bytes(),ARTIFACT.read_bytes())
        self.assertEqual(digest(ARTIFACT.read_bytes()),PROFILE['revision'])
        self.assertIn(DIGEST,(ROOT/'docs/ir/contracts/README.md').read_text())
        self.assertIn(DIGEST,(ROOT/'crates/semantic-check/src/resources/interference.rs').read_text())
        draft=ROOT/'docs/dialects/drafts/external-interference.md'
        self.assertEqual(digest(draft.read_bytes()),'sha256:4e37d47a5c61f520dba3db704cbe19b7ad0e92b00a56c46edcf91a1e30cb0dfc')

    def test_schemas_closed_shapes_and_stable_example_dependency_pins(self):
        for schema in SCHEMAS.values():jsonschema.Draft202012Validator.check_schema(schema.schema)
        for filename,kind in [('descriptor.json','external-interference'),('namespace.json','external-region-namespace')]:
            doc=json.loads((FOLDER/filename).read_bytes());SCHEMAS[kind].validate(doc)
            self.assertEqual(doc['profile'],PROFILE)
            for change in [lambda d:d.update(extra=True),lambda d:d.update(version='0.1.0-draft.1')]:
                bad=deepcopy(doc);change(bad)
                self.assertFalse(SCHEMAS[kind].is_valid(bad))
        descriptor=json.loads((FOLDER/'descriptor.json').read_bytes());ns=json.loads((FOLDER/'namespace.json').read_bytes())
        provider=ROOT/'docs/dialects/drafts/external-interference-examples/provider-a.json'
        self.assertEqual(descriptor['provider']['revision'],digest(provider.read_bytes()))
        self.assertEqual(descriptor['provider']['id'],json.loads(provider.read_bytes())['id'])
        for channel in ['activity','terminal']:
            for regions in descriptor[channel].values():
                for region in regions:self.assertEqual(region['namespace'],{'id':ns['id'],'revision':digest((FOLDER/'namespace.json').read_bytes())})
        self.assertEqual(ns['authority']['revision'],digest((FOLDER/'synthetic-authority.txt').read_bytes()))

    def test_unused_resource_bytes_are_not_current_core_selection_or_independence(self):
        before=fixed()
        original=package('compensatable','compensatable')
        extras=[]
        for identity,path in [(ID,ARTIFACT),('urn:test:standalone:descriptor',FOLDER/'descriptor.json'),('urn:test:standalone:namespace',FOLDER/'namespace.json'),('urn:test:standalone:authority',FOLDER/'synthetic-authority.txt')]:
            raw=path.read_bytes();extras.append((identity,digest(raw),raw))
        augmented=original[0],original[1],[*original[2],*extras]
        lowered=[invoke(candidate) for candidate in [original,augmented]]
        for result in lowered:self.assertEqual(result.returncode,0,result.stderr)
        self.assertEqual(json.loads(lowered[0].stdout),json.loads(lowered[1].stdout))
        for candidate in [original,augmented]:
            checked=invoke(candidate,'check');self.assertEqual(checked.returncode,0,checked.stderr)
            self.assertIs(json.loads(checked.stdout)['executable'],False)
            r=invoke(candidate,'check-concurrency');self.assertEqual(r.returncode,1);self.assertEqual(r.stdout,b'')
            self.assertTrue(r.stderr.startswith(b'parallel-effects-unproved at /body/nodes/fork;'))
        self.assertEqual(fixed(),before)
        # Those unused bytes were NOT selected or consumed as interference
        # declarations by C12. Real reader evidence is the Rust SDK test suite.

    def test_artifact_and_example_licensing_links_and_ci_command(self):
        for name in ['docs/decisions/0080-external-interference-resources.md','docs/evaluation/0080-external-interference-publication.md','docs/decisions/README.md','docs/dialects/interference-examples/README.md']:
            check_links(ROOT/name)
        for path in [*FOLDER.glob('*.json'),*(ROOT/'schemas/resources').glob('*interference*-0.1.schema.json'),ROOT/'schemas/resources/external-region-namespace-0.1.schema.json']:
            self.assertIn('SPDX-License-Identifier: MPL-2.0',path.with_name(path.name+'.license').read_text())
        self.assertIn('SPDX-License-Identifier: CC-BY-4.0',ARTIFACT.read_text())
        self.assertIn('tools/check_external_interference_resources.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())



if __name__=='__main__':unittest.main()
