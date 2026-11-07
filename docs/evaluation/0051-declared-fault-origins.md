<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Declared fault origins and complete handler-context evidence

**Date:** 2026-11-07<br>
**Plan:** [frozen prerequisite scope](0051-declared-fault-origins-plan.md), `d5c227d`<br>
**Decision:** [ADR-0051](../decisions/0051-declared-fault-origin-contexts.md)

The separate raw SDK check_fault_origins returns a FaultOriginReport with the
original FaultReport and borrowed declared site/source/selection views. It reuses
one timing/routing pass and its exact nearest-selection, local-before-escape,
failed-selection exclusion and cycle judgment. No admission is rerun for disclosure.
Legacy check_fault_routes constructs no origin index/context storage, keeps original
budget accounting and returns unchanged scope/name routes and counts. No wire/core /
source/native command, default report or artifact changes.

All activity declarations have read-only site views, including unsupported, unhandled,
ordinary-unvisited and unexplored cases. Exact retained core-0.6 general use indices
supply source/profile/provider identity; human and null/legacy general-source contexts
remain unsupported. Match supported routes only to their exact source's declared
names. Unsupported activities conservatively consider every known name, including
unvisited declarations in explored scopes. Scope exploration and ordinary syntactic
node reachability are separate flags, neither implying activation or current values.
Activity declarations in unexplored scopes remain uncovered. Null source or absent
route is never proof that no fault can occur.

For handled declared cases, retain complete active selection stacks before and
after selection, not merely the previous top handler. Nested failures exclude all
escaped selections before selecting an outer handler. Per-route source metadata /
strings are not copied: sites/selections/source uses are indexed once and stack
vectors are interned. Indexing, each site/name/context comparison, cached stack
lookup, vector/row storage and unhandled counters share the existing cumulative
routing allowance. Failure cannot return partial evidence or truncate coverage.
Separate declared unhandled and unsupported-unhandled counts do not replace legacy
abstract scope/name counts or represent observed faults.

Five Rust integration groups exercise promoted full-package SDK judgments: exact
provider use/single-attempt and noneligible names; same-name different-provider
sources without reassignment; unvisited local-result activity and unexplored scope
flags; nested provider activities with before [first,second] and after [outer]
stacks, preserving failed-selection exclusion; and old retry binding remaining
unsupported as a general origin plus unchanged cycle refusal. They compare old
abstract routes/counts directly against the retained nested report. Fixture-only
local result cells avoid introducing uncovered ancestor writers into ordinary
invalidation. Existing multi-name cyclic-handler refusals stay intact rather than
being weakened to fit a test.

Four internal groups check all nine finite selection/exclusion vectors through the
actual routing algorithm; no origin storage/index when optional collection is off;
shared stack lookup/copy exhaustion; and cumulative deduplicated row/unhandled
accounting without a new allowance. Three Python groups use an independent
ancestry-based declarative boundary model over the same vectors and preserve exact
legacy native routes/candidate bytes plus CI/licensing. These finite declarations
are not generated observations, real source faults or complete activation proofs.

Local verification passes **409 Rust tests plus six compile-fail doctests**, format,
workspace native lint, all five library Wasm lints and release portability/frontend
builds. The native oracle passes 97 structural cases, 17 JCS/hash comparisons, three
original fixtures/hashes, ten node variants and nine CLI operations. Every required
Python command and exact benchmark regeneration/historical probe passes. Wasm
compilation supplies no actual browser execution. Transient logs:
`/private/tmp/fault-origins-{workspace,clippy,wasm,native,oracles,focused,unit,python}.log`.
Required final-head CI, comment audit and signed expected-head merge remain pending.

Separate completed-diff delegated automated self-review checked exact static-source
use indexing and report ownership, unsupported/unvisited/unexplored classifications,
full before/after contexts and unchanged selection/exclusion/cycles, shared cumulative
budgets and no partial result, no metadata copies/rerun or added legacy origin work,
old hashes/reports/contracts, licensing and finite evidence scope. No actionable
issue remains. This is agent self-review, not independent/human or personal owner
approval; inspect all threads and CodeRabbit summaries. Skipped bot review supplies
no review evidence.

No actual emitter, occurrence/attempt/cause/input/effect correlation, source truth,
complete source/control envelope, current access or fresh history initialization
follows. Unsupported/uncovered origins cannot receive invented values or handler
coverage. Next freeze explicit handler-local entry wire/core plus complete incoming
supported-source/control/provenance contexts, fresh immutable local initialization,
conditional access/flow and strict invalidation. Then readable entry authoring and
permitted owned continuation/cutoff/all-settlement. Current RP-01 source 0.7/core
0.4/IR 0.1 remains unchanged; parent #46 draft/incomplete, all forty scenarios Partial,
and all complete validator/benchmark/general authoring/visual/protected editing /
actual host/user/browser/independent/phase gates open.
