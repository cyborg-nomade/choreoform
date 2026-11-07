<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Provider metadata dependency closure: frozen contract plan

**Date:** 2026-11-06<br>
**Baseline:** `a5e72f0b39293fa9f72b22df4489345c5e63bd13`, draft PR #46<br>
**Authority:** ADR-0018 delegation; measured RP-01 checkpoint and cycle witness.

Create a dedicated stacked dependency PR targeting `codex/rp01-benchmark-package`.
Publish ADR-0035 and a separately pinned core 0.3.0 extension, inheriting exact
core 0.2.0 and core 0.1.0 bytes. Keep old snapshots, wire fields, type/expression/
policy payloads, retry-clock rules and non-executable stage authority unchanged.
Do not merge parent #46 or claim full benchmark readiness from this dependency.
Source profiles remain unchanged here; explicit new authoring support follows
in its own frozen dependency if raw-core evidence validates this choice.

The specific blocker is resultProtection's definition-context recipient metadata:
allowing a capability's provider result to be reused by the same capability adds
a finite reference back to that capability's wrapper. Existing core 0.1/0.2 require
all policy references acyclic, so the proposed broad RP-01 label fails policy-cycle.
Denying reuse or dropping those references/reads is not an acceptable workaround.

For core-0.3 policy records only, classify references obtained from provider result
protection as metadata edges (activity capability policies, timeout clocks and
retry clocks). Validate all recipients, expected kinds, lexical visibility, pins
and protection fields exactly as before. Other policy references remain strong.
Require the strong graph acyclic with existing 64-level depth bound. Derive reads
as the least finite union closure over both edge sets; do not recursively execute
predicates or cache authorization truth. Cyclic components in the combined graph
must consist entirely of core-0.3 records, and their internal strong edges remain
acyclic. Any cyclic legacy/mixed component still refuses. Combined condensation
DAG depth is also at most 64. Missing refs refuse; no metadata edge is discarded.

Implement iterative indexed SCC/condensation, charging vertices/edges/set copies
under existing 10,000-item/1,000,000-work semantic bounds. Retain the old checker
path for definitions without core 0.3. A small pure read-closure helper may be
public for later compiler reuse: typed local/strong/metadata sets and explicit
cycle eligibility, fixed upper budgets, resulting sets/work count only. It proves
no pin/record validity or execution authority; validated callers establish those.
Charge its work against the caller's existing total budget, not an extra budget.
Do not introduce dependencies or recursion tied to arbitrary graph depth.

Test exact core 0.3/inherited pins/missing/corrupt/unknown bytes; raw canonical
RP-01 candidate promoted with coherent synthetic business/clock result envelopes;
legacy refusal of the identical graph; strong policy cycles and mixed cyclic
components; self/mutual metadata closure with real data reads on every member,
transitive actor/protection/capability/effect/wait/clock influences and stale reads;
acyclic compatibility, disconnected/unvisited declarations, lexical wrong kinds,
long/deep/dense graphs and honest budget refusals. Use an independent Python
finite-union oracle for graph vectors. No lowerable authoring or later stage pass
is invented; retain all additional genuine refusals as evidence.

Run workspace/native/Wasm checks and relevant Python oracles, separately review
completed diff, inspect all PR comments including CodeRabbit summaries, record
delegated acceptance honestly and merge only the passing expected head into the
unmerged parent. Then freeze explicit source support and return to the whole
RP-01 package, protection/invalidation/lifetime/proof and all remaining phase gates.
