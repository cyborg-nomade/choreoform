<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Closed ordinary provenance evidence

**Date:** 2026-11-06<br>
**Plan:** [frozen plan](0037-closed-provenance-plan.md), `a3fc112`<br>
**Pre-change critique:** [actual baseline witnesses](0037-closed-provenance-audit.md),
`1996854`<br>
**Decision:** [ADR-0037](../decisions/0037-closed-ordinary-provenance.md)

The analysis now distinguishes live cell inputs from historical private influences
at synchronous invoke/repeat completion. Classification requires the cell's exact
owning body scope and no declared nonlocal writer; every validated declaration is
indexed, including unvisited descendants/handlers/follow-ups and output bindings.
An already-live caller origin is not upgraded by reuse of its template. Split/
fan-out summaries and unproved descendant origins stay live. Mutable ancestor,
copy/access-policy and provider influences remain; historical labels are never
declassified. Only ordinary invalidation collection uses the new classification.
Protection-only output and the source/IR/core revisions stay unchanged.

`InvalidationReport::historical_origins()` exposes typed declaration evidence.
Each record names the consuming `site`, source `cell`, ordinary `boundary`, source
`scope`, its `closure` and a fixed conditional premise. When `context_scope()` is
present, `site` is the creator passing the influence into that destination scope;
the native record includes `contextScope`. It does not conflate source/destination
scope or invent occurrence IDs. Scope contexts used by concurrency omit only the
historical origins and record that substitution explicitly. No conflict, lifetime
or current authorization rule is weakened.

All seven native views from invalidation through reconciliation availability carry
`historicalOrigins` and the residual occurrence/closure/snapshot enforcement
requirement. A shared serializer bounds origin records and final JSON to 1 MiB
using the existing LimitedWriter before stdout publication. No values, instructions,
current authority, actual completion receipts or executable capability is returned.

Seven additional Rust groups contrast exact call/owner/closure metadata and live
ancestor/output obligations; unvisited descendant writers and forged write sets;
read-only surviving closure with separate lifetime refusal; unchanged historical
control protection; fork/fan-out and inherited-live-template refusal; repeated calls
with distinct conditional boundaries; and dense historical work exhaustion.
Two older unproved-context tests now exercise the proved subset plus an actual
mutable-writer negative rather than silently deleting the old requirement.

Five Python groups exercise real source invoke/repeat success, missing ancestor
and surviving-writer refusals, unchanged separate lifetime refusal, all seven
composed native report views, format/reparse/revision identity and private namespace
refusal. A grammar-valid 100-cell/80-site fixture reaches report-budget with no
stdout; its thousands of historical records are not truncated into success.
These are native static checks on synthetic declarations, not process executions.

Actual RP-01 regeneration retains canonical revision
`sha256:559d82dbee340a13cbb25b61fcaa6fbb588563dad2d4a5e4812721edc909896b`
and the existing source/package/resource/IR digests. Its protection-only reports
are unchanged. The next actual refusal is now
`invalidation-coverage at /body/data/disposition/invalidates; source bytes 49280..49607`.
The mutable ancestor was retained, revealing a genuine missing package declaration.
The manifest/README/host boundary and exact assertions record this result; later
composed stages remain unestablished. No full benchmark or scenario is admitted.

Local checks passed 277 Rust tests plus five compile-fail doctests, format/native
workspace lint, all five library Wasm lints, release portability/frontend Wasm
builds, native report, historical draft probes and all CI Python oracles. Logs:
`/private/tmp/closed-provenance-tests.log`, `/private/tmp/closed-provenance-native.log`,
`/private/tmp/closed-provenance-wasm.log`, `/private/tmp/closed-provenance-report.log`,
`/private/tmp/closed-provenance-oracles.log`. The final head must also pass CI.

Separate completed-diff automated self-review checked core closure/value-copy
premises, exact owner scope, complete writer inventories, inherited live contexts,
mutable ancestor lineage, preserved labels, repeated/fork contexts, scope-entry
report evidence, conditional host obligations and existing work/report budgets.
It added explicit scope-context records after a concurrency regression exposed
that evidence need, preserving the live-writer counterexample. No actionable issue
remains. Acceptance is delegated automated review, not independent or individual
owner review. Inspect all comments and merge only a passing expected head into
the unmerged parent, with an explicit merge sign-off body. The
[earlier merge-message exceptions](0037-merge-signoff-correction.md) are recorded
honestly; no published history is rewritten.

Next return to the full package's missing invalidation declarations and remaining
lifetime/proof dependencies. Derive precise obligations before adding links; do
not invalidate every scope/control node merely to silence a check. All forty
scenarios remain Partial. Complete validator/benchmarks, text/visual/protected
editing, independent host/browser/actual-user evidence and Phase 1 exit stay open.
