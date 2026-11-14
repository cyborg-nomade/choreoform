# SPDX-FileCopyrightText: 2026 Choreoform contributors
# SPDX-License-Identifier: MPL-2.0
"""Independent finite-union graph oracle, not policy or execution admission."""
from hashlib import sha256
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
VECTORS = ROOT / 'crates/semantic-check/tests/conformance/policy-reads.json'
PIN = '30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3'


def derive(nodes):
    edges = {p: set(v['strong']) | set(v['metadata']) for p, v in nodes.items()}
    if any(not deps <= nodes.keys() for deps in edges.values()):
        return {'code': 'policy-reference'}
    strong = {p: set(v['strong']) | (set(v['metadata']) if not v['metadata_cycles'] else set()) for p, v in nodes.items()}
    # Repeated finite relation composition, independent of native SCC traversal.
    def transitive(graph):
        reach = {p: set(deps) for p, deps in graph.items()}
        changed = True
        while changed:
            updated = {p: deps | set().union(*(reach[q] for q in deps)) for p, deps in reach.items()}
            changed = updated != reach
            reach = updated
        return reach
    strict_reach, reach = transitive(strong), transitive(edges)
    if any(p in strict_reach[p] or (not v['metadata_cycles'] and p in reach[p]) for p, v in nodes.items()):
        return {'code': 'policy-cycle'}
    def depth(graph):
        pending = dict(graph)
        known = {}
        while pending:
            ready = [p for p, deps in pending.items() if deps <= known.keys()]
            if not ready:
                raise ValueError('oracle condensed a cyclic graph')
            for p in ready:
                known[p] = 1 + max((known[q] for q in pending.pop(p)), default=0)
        return max(known.values(), default=0)
    if depth(strong) > 64:
        return {'code': 'policy-depth'}
    groups = {p: frozenset(q for q in nodes if q == p or (q in reach[p] and p in reach[q])) for p in nodes}
    condensed = {group: {groups[q] for p in group for q in edges[p] if groups[q] != group} for group in set(groups.values())}
    if depth(condensed) > 64:
        return {'code': 'policy-depth'}
    reads = {p: set(v['local']) for p, v in nodes.items()}
    changed = True
    while changed:
        updated = {p: deps | set().union(*(reads[q] for q in edges[p])) for p, deps in reads.items()}
        changed = updated != reads
        reads = updated
    return {'reads': {p: sorted(v) for p, v in reads.items()}}


class PolicyReadEvidence(unittest.TestCase):
    def test_independent_oracle_recomputes_every_native_conformance_vector(self):
        cases = json.loads(VECTORS.read_bytes())
        self.assertGreaterEqual(len(cases), 10)
        for case in cases:
            with self.subTest(case['name']):
                self.assertEqual(derive(case['nodes']), case['expected'])

    def test_exact_new_snapshot_and_both_inherited_pins(self):
        raw = (ROOT / f'docs/ir/contracts/sha256-{PIN}.txt').read_bytes()
        self.assertEqual(sha256(raw).hexdigest(), PIN)
        self.assertEqual(raw, (ROOT / 'docs/dialects/core-metadata-v0.3.md').read_bytes())
        for pin in ['26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58', '5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623']:
            self.assertEqual(sha256((ROOT / f'docs/ir/contracts/sha256-{pin}.txt').read_bytes()).hexdigest(), pin)
            self.assertIn(pin.encode(), raw)

    def test_transitive_union_never_drops_a_cyclic_members_direct_reads(self):
        cases = json.loads(VECTORS.read_bytes())
        case = next(c for c in cases if c['name'] == 'mutual-recipients')
        result = derive(case['nodes'])['reads']
        self.assertEqual(result['a'], ['claim', 'policy'])
        self.assertEqual(result['b'], ['claim', 'policy'])


if __name__ == '__main__':
    unittest.main()
