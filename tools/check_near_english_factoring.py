# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Real source measurements/limits, not new syntax or user comprehension evidence."""
from collections import Counter
from hashlib import sha256
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from check_authoring_evidence import check_links
from check_source_capacity import fixed

ROOT=Path(__file__).resolve().parents[1]
FIXTURES=['docs/benchmarks/rp01/history/source-0.14-before-factoring/source.choreo','docs/benchmarks/rp03/history/source-0.16-before-factoring/source.choreo']
CAPTURE=ROOT/'docs/evaluation/0083-factoring-metrics.json'
BINARY=ROOT/'target/debug/examples/factoring_metrics'


class FactoringMeasurements(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.before=fixed()
        env=os.environ.copy()
        # Local tools are optional: CI installs the pinned toolchain on PATH.
        if (ROOT/'.tools/cargo/bin/cargo').is_file():
            env['CARGO_HOME']=str(ROOT/'.tools/cargo');env['RUSTUP_HOME']=str(ROOT/'.tools/rustup')
            env['PATH']=str(ROOT/'.tools/cargo/bin')+os.pathsep+env['PATH']
        r=subprocess.run(['cargo','run','--quiet','-p','choreoform-authoring-frontend','--example','factoring_metrics','--locked','--',*FIXTURES],cwd=ROOT,env=env,capture_output=True,timeout=60)
        if r.returncode:raise AssertionError(r.stderr.decode())
        cls.raw=r.stdout;cls.reports=json.loads(r.stdout)

    def test_captured_native_measurements_reproduce_without_benchmark_writes(self):
        # Only exact quoted fixture-path labels change; every measured byte stays fixed.
        historical=self.raw
        for name,fixture in zip(['rp01','rp03'],FIXTURES):
            label=json.dumps(fixture).encode();self.assertEqual(historical.count(label),1)
            historical=historical.replace(label,json.dumps('docs/benchmarks/'+name+'/source.choreo').encode(),1)
        self.assertEqual(historical,CAPTURE.read_bytes())
        self.assertEqual([(r['sourceBytes'],r['tokens']) for r in self.reports],[(61211,10223),(576759,84051)])
        self.assertEqual(fixed(),self.before)
        for report in self.reports:
            self.assertEqual(report['sourceDigest'],'sha256:'+sha256((ROOT/report['fixture']).read_bytes()).hexdigest())
            self.assertIn('no usability or lowering-equivalence claim',report['interpretation'])
            for key in ['outerTypes','outerExpressions','dataProtection']:
                value=report[key]
                self.assertLessEqual(value['repeatedBytesUpperBound'],value['bytes'])
                self.assertLessEqual(value['bytes'],report['sourceBytes'])
                self.assertLessEqual(len(value['topRepeated']),8)

    def test_resolved_ir_protection_identity_is_checked_independently_of_spelling(self):
        for name,report,expected in zip(['rp01','rp03'],self.reports,[26,155]):
            body=json.loads((ROOT/FIXTURES[['rp01','rp03'].index(name)]).with_name('candidate.ir.json').read_bytes())['body']
            groups=Counter(json.dumps(d['protection'],sort_keys=True,separators=(',',':')) for d in body['data'].values())
            self.assertEqual(len(body['data']),expected)
            self.assertEqual(len(groups),1)
            self.assertEqual(report['dataProtection']['occurrences'],expected)
        self.assertEqual(self.reports[0]['dataProtection']['repeatedBytesUpperBound'],5808)
        self.assertEqual(self.reports[1]['dataProtection']['repeatedBytesUpperBound'],133364)
        self.assertEqual(self.reports[1]['outerTypes']['repeatedBytesUpperBound'],70621)
        # Lexically different reference spellings/order can still resolve equally.
        self.assertEqual(len(self.reports[0]['dataProtection']['topRepeated']),2)

    def test_measurement_io_work_output_boundaries_fail_without_source_echo(self):
        for args in [[],FIXTURES*2]:
            r=subprocess.run([str(BINARY),*args],cwd=ROOT,capture_output=True,timeout=10)
            self.assertNotEqual(r.returncode,0);self.assertEqual(r.stdout,b'')
        with tempfile.TemporaryDirectory() as directory:
            path=Path(directory)/'input.choreo'
            for raw in [b'\xff',b'"SECRET-MUST-NOT-APPEAR"',b'x'*(1024*1024+1)]:
                path.write_bytes(raw)
                r=subprocess.run([str(BINARY),str(path)],cwd=ROOT,capture_output=True,timeout=10)
                self.assertNotEqual(r.returncode,0);self.assertEqual(r.stdout,b'')
                self.assertNotIn(b'SECRET-MUST-NOT-APPEAR',r.stderr)
        self.assertLessEqual(len(self.raw),65536)

    def test_direction_paper_limits_licensing_and_required_ci(self):
        for name in ['docs/evaluation/0083-near-english-factoring-critique-plan.md','docs/evaluation/0083-near-english-factoring-critique.md','docs/decisions/0083-near-english-factoring-direction.md','docs/decisions/README.md']:
            check_links(ROOT/name)
        self.assertIn('SPDX-License-Identifier: MPL-2.0',CAPTURE.with_name(CAPTURE.name+'.license').read_text())
        self.assertIn('test = true',(ROOT/'crates/authoring-frontend/Cargo.toml').read_text())
        self.assertIn('tools/check_near_english_factoring.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__=='__main__':unittest.main()
