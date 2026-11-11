# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Controlled exact repair/history, not full candidate or host admission."""
from hashlib import sha256
import json
import unittest
import authoring_probe as grammar
from benchmarks.rp01_resources import encoded
from build_rp03_checkpoint import BASE as ROOTBASE,package_artifacts,historical_artifacts
BASE=ROOTBASE/'history/source-0.16-pre-instances'
from check_rp03_package import pre_instance_package as package,historical_package
from check_rp03_symbol_binding import finish_audit
from check_metadata_authoring import invoke
from check_static_contracts import ROOT

HISTORY=ROOTBASE/'history/source-0.14'
ORIGINAL='797d01f0b1a42db5c71de9bc7a054d58e24333798b23e6cde22225dc4cc707ee'


class OrderCurrentCheckpointEvidence(unittest.TestCase):
    def test_all_113_original_generated_files_match_the_independently_frozen_digest(self):
        files=historical_artifacts();self.assertEqual(len(files),113);h=sha256()
        for name in sorted(files):
            raw=(HISTORY/name).read_bytes();self.assertEqual(files[name],raw,name)
            h.update(name.encode()+b'\0'+len(raw).to_bytes(8,'big')+raw)
        self.assertEqual(h.hexdigest(),ORIGINAL)
        expected=set(files)|{'README.md'}
        self.assertEqual({p.relative_to(HISTORY).as_posix() for p in HISTORY.rglob('*') if p.is_file()},expected)

    def test_only_pair_core_source_caption_manifest_and_context_change(self):
        old=historical_artifacts();new=package_artifacts()
        for name,raw in new.items():self.assertEqual((BASE/name).read_bytes(),raw,name)
        self.assertEqual({p.relative_to(BASE).as_posix() for p in BASE.rglob('*') if p.is_file()},set(new)|{'README.md','README.previous.md'})
        for name,raw in old.items():
            if name not in ['source.choreo','bindings.json','manifest.json']:self.assertEqual(new[name],raw,name)
        source=old['source.choreo'].decode().replace('authoring "0.14.0"','authoring "0.16.0"',1).replace('Step "Finish authorization". Finish as "authorization recorded".','Step "Finish authorization". Finish as "authorization-recorded".',1)
        self.assertEqual(source.encode(),new['source.choreo'])
        before=json.loads(old['bindings.json']);after=json.loads(new['bindings.json'])
        before.update(profile='0.16.0',version='0.15.0');before['contracts']['core']=after['contracts']['core'];self.assertEqual(before,after)
        a=json.loads(old['manifest.json']);b=json.loads(new['manifest.json'])
        for field in ['resources','inputArtifacts','scenarios','hostRequirements','declarationCounts']:self.assertEqual(a[field],b[field],field)

    def test_exact_original_refusal_and_corrected_current_refusal_are_distinct(self):
        old=historical_package();x=invoke(old)
        self.assertEqual((x.returncode,x.stdout,x.stderr),(1,b'',b'tokens at 108626..108627\n'))
        p=package();grammar.validate_ledger(p[1]);self.assertEqual(len(grammar.lex(p[0])),77135)
        x=invoke(p);self.assertEqual((x.returncode,x.stdout,x.stderr),(1,b'',b'child-template at 0..0\n'))
        manifest=json.loads((BASE/'manifest.json').read_bytes())
        self.assertEqual(manifest['lowering'],{'result':'refused','code':'child-template','sourceBytes':[0,0],'diagnostic':'child-template at 0..0'})
        self.assertIsNone(manifest['canonicalIR']);self.assertEqual(manifest['semanticStageResults'],[]);self.assertIs(manifest['executable'],False)
        self.assertFalse((BASE/'candidate.ir.json').exists());self.assertFalse((BASE/'reports').exists())
        with self.assertRaisesRegex(ValueError,'oracle state limit'):grammar.recognize(p[0])

    def test_current_package_integrity_and_all_56_direct_finishes_are_consistent(self):
        p=package();expected=grammar.package_digests(p[0].encode(),encoded(p[1]));x=invoke(p,'integrity')
        self.assertEqual(x.returncode,0,x.stderr);self.assertEqual(json.loads(x.stdout),expected)
        self.assertEqual(expected['package'],'sha256:f14598c4e75e1d134a65b9ef85f6a21dd9edd1cca38c7fcb75e4ee0a87a927f5')
        files,total,failures=finish_audit(historical=False);self.assertEqual(total,56);self.assertEqual(failures,[])
        self.assertEqual(files['source.choreo'],p[0].encode())
        self.assertIn('tools/check_rp03_current_checkpoint.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__=='__main__':unittest.main()
