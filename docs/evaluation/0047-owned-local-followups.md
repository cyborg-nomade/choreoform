<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Conditional owned-local follow-up evidence

**Date:** 2026-11-07<br>
**Plan:** [frozen subset](0047-owned-local-followups-plan.md), `afed903`;
[provenance refinement](0047-owned-local-provenance-refinement.md), `cdac368`<br>
**Decision:** [ADR-0047](../decisions/0047-owned-local-followups.md)

The new optional private-constructor report is distinct from existing data-free
`IsolatedFollowup`. A covered synchronous invoke/repeat must own a direct portless,
childless, terminal follow-up whose entry is explicit timerless capture. Every
node, including unvisited ones, is wait/compute/decision/finish with complete reads /
writes confined to that follow-up's own fresh locals. All waits are timerless;
no independent declared creator may target the follow-up. Require a reachable
finish. All original static source/provider/type/protection/visibility checks remain.

Candidates share immutable body templates and one bounded graph. Grouped conditional
control analysis starts with zero inherited values and fresh local initialization.
Grouped protected flow/access/dependency analysis and strict invalidation run
before returning a report. Ordinary covered/uncovered sets and historical/data-free
views remain unchanged; conditional coverage is named separately. Advisory ordinary
requirements cannot satisfy missing conditional invalidation.

Initial unpublished diff review found that empty protection origins would erase
creator/owner control provenance. Before implementing the correction, refinement
`cdac368` froze retained label contexts and an explicit P6 provenance premise.
Ordinary protection now retains bounded unions of owner entry/internal source and
control influence, including partial child influence and distinct provider/clock
roles, only when explicit captures exist. It reuses the composed report rather
than rerunning admission. Legacy collection stays off and previous work is unchanged.

Conditional entries convert cell/historical influences to a distinct immutable
protection-provenance origin. It names the definition's label, never an inherited
value, cached grant, ordinary completion or invented runtime receipt. Provider
envelopes remain distinct. These origins propagate through initialization, capture,
computation and branch publication; actual current access remains required. They
cannot satisfy reads or initializers and are not mutable ancestor data dependencies.
The host must retain authenticated occurrence-bound source/control provenance
through handoff/disposal; the report does not establish that retention occurred.

Typed evidence binds the definition revision, creator/body/closure/follow-up /
local-cell/capture declarations, conditional coverage, required accesses, strict
invalidation and required owner/cell or provider-node/role provenance. Templates
share no live occurrences, values or permissions. Native concurrency and all five
later views disclose nonempty `ownedLocalFollowups` and
`conditionalOwnedLocalChecks`; older/data-free reports gain no keys. All new rows
and final combined output remain bounded before stdout.

Nine Rust integration groups cover actual full RP-01 nominal capture followed by
local read/branch/compute; separate ordinary/conditional coverage; strict missing
conditional invalidation; read-before-capture and ancestor dependency refusal;
source/control laundering through a literal; unvisited foreign writes, nested
scopes, noncapture storage and activities; independent unvisited creators; repeated
covered creators sharing immutable metadata; and fresh literal versus ancestor-read
initializers. Repeated-creator/initializer fixtures explicitly add derived ordinary
links then recheck strict coverage at their final revision; no advisory report is
promoted. The adjacent ancestor-storage lifetime case still refuses.

Two internal groups check cumulative index/cached-template row budgets and retained
provenance walk/copy exhaustion before partial rows are returned, plus disabled
legacy collection. Five Python groups exercise actual readable fixtures and all
composed disclosures, a restricted creator/public-capture regression, six independent
finite isolation/initialization/protection/invalidation vectors, shared templates /
bounded output, full promoted RP-01 and unchanged original package reports. The
finite model assumes its prevalidated small fixture; it is not a full validator or
engine, and its sets are static judgments rather than values or generated events.

The creator-control regression passes ordinary invalidation, then refuses
`control-information-flow` at the public capture despite matching direct source /
destination labels. A 130-creator/130-cell fixture is 55,475 source bytes / 9,451
measured tokens; lowering and ordinary invalidation succeed, but every concurrency /
later native view refuses `report-budget` with empty stdout. No larger quota or
truncated JSON is used. All template/provenance/index/copy work remains charged
under existing cumulative stage allowances.

The full RP-01 promoted copy now passes the narrow conditional fresh-local subset,
including real nominal terminal payloads and original guards. This supplies no
actual late result, allocated storage, fault-data port, reopened parent path,
accepted handoff/cutoff or settled obligation. Checked-in source 0.7/core 0.4/IR 0.1
package, resources, guard snapshots and all existing reports regenerate unchanged.
Every wire/core/source artifact byte stays fixed. All forty scenarios remain Partial.

Local verification passes **373 Rust tests plus six compile-fail doctests** (the
full workspace plus the final added provenance-budget test), format/native lint,
all five library Wasm lints and release portability/frontend builds. The native
oracle passes 97 structural cases, 17 JCS/hash comparisons, three unchanged original
fixtures/hashes, ten node variants and nine CLI operations. Every required Python
oracle and exact benchmark regeneration/historical probes passes. Wasm compilation
is not actual browser execution. Transient logs:
`/private/tmp/owned-local-{workspace,clippy,wasm,native,oracles,tests,python,budget,provenance-budget}.log`.
Final-head CI/comment audit/signed merge remain required.

Separate completed-diff automated self-review checked storage scope and all
unvisited hazards, zero inherited values versus immutable protection provenance,
creator context sharing and source-role identity, fresh initialization, internal
source/control transfer and access dependencies, strict conditional invalidation,
ordinary compatibility, grouped cumulative budgets/template sharing, bounded
native output, immutable report authority boundaries, licensing and evidence scope.
The empty-origin defect was corrected and has a native/independent regression;
no actionable issue remains. This is delegated agent self-review, not independent
or individual owner approval. Audit all threads and CodeRabbit summaries before
signed expected-head merge; a skipped bot supplies no review evidence.

Next resolve explicit fault data and permitted owned continuation/cutoff/all-
settlement before benchmark adoption. Actual live storage/source/correlation /
current authority/provenance/atomic acceptance/settlement, complete validator and
benchmarks, general plain-English/tool-managed/visual/protected accessible editing,
actual users/host/browser and independent review remain open. Parent #46 stays
draft/incomplete; no scenario/phase exit or Phase 2 follows.
