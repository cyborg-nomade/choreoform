# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Independent declaration/pin/refusal audit, never actual requests or host traces."""
from hashlib import sha256
import json
from pathlib import Path
import unittest
from check_authoring_evidence import check_links
from check_metadata_authoring import invoke
from check_rp03_package import package
from check_source_capacity import fixed

ROOT=Path(__file__).resolve().parents[1]
BASE=ROOT/'docs/benchmarks/rp03'


class RequestRegionAudit(unittest.TestCase):
    def test_current_request_provider_terminal_and_type_pins_are_exact(self):
        body=json.loads((BASE/'candidate.ir.json').read_bytes())['body']
        manifest=json.loads((BASE/'manifest.json').read_bytes())
        resources={(r['id'],r['revision']):(ROOT/r['path']).read_bytes() for r in manifest['resources']}
        for key in ['reserve_a','reserve_b']:
            capture=body['nodes'][key+'_observe']['capture']
            self.assertEqual(capture['origin'],key+'_activity')
            pin=capture['source'];raw=resources[pin['id'],pin['revision']]
            self.assertEqual(pin['revision'],'sha256:'+sha256(raw).hexdigest())
            terminal=json.loads(raw);provider=terminal['provider']
            raw=resources[provider['id'],provider['revision']]
            self.assertEqual(provider['revision'],'sha256:'+sha256(raw).hexdigest())
            doc=json.loads(raw)
            self.assertEqual(doc['effectClass'],'compensatable')
            self.assertEqual(doc['input']['fields']['target warehouse'],{'kind':'option','element':{'kind':'text'}})
            self.assertNotIn('interference',doc)
        types=json.loads((BASE/'types.json').read_bytes())
        for line in ['first line','second line']:
            self.assertEqual(types['definitions']['Order']['representation']['fields'][line]['fields']['warehouse'],{'kind':'text'})

    def test_accepted_new_profile_artifacts_remain_byte_identical(self):
        for name,h in [('core-interference-binding-v0.13.md','dd8667135d1c801683d48d8410867b37d5d6f71c7016b8b716016bce92e77428'),('provider-interference-regions.md','0b10eb4dbb19caace45b93f35dec44aa5f26458d631393932caddb5589210250')]:
            source=(ROOT/'docs/dialects'/name).read_bytes()
            self.assertEqual(sha256(source).hexdigest(),h)
            self.assertEqual(source,(ROOT/'docs/ir/contracts'/f'sha256-{h}.txt').read_bytes())

    def test_actual_source_refusal_and_benchmark_bytes_stay_fixed(self):
        before=fixed();r=invoke(package(),'check-concurrency')
        self.assertEqual(r.returncode,1);self.assertEqual(r.stdout,b'')
        self.assertEqual(r.stderr.strip(),b'parallel-effects-unproved at /body/nodes/start_acceptance; source bytes 109844..110031')
        self.assertEqual(fixed(),before)

    def test_direction_links_roadmap_open_gate_and_ci_command(self):
        for name in ['docs/evaluation/0082-request-region-proof-audit-plan.md','docs/evaluation/0082-request-region-proof-audit.md','docs/decisions/0082-request-region-proof-direction.md','docs/decisions/README.md','ROADMAP.md']:
            check_links(ROOT/name)
        roadmap=(ROOT/'ROADMAP.md').read_text()
        self.assertIn('- [ ] Complete the executable IR contract',roadmap)
        self.assertIn('0082-request-region-proof-direction.md',roadmap)
        self.assertIn('tools/check_request_region_audit.py',(ROOT/'.github/workflows/rust-portability.yml').read_text())


if __name__=='__main__':unittest.main()
