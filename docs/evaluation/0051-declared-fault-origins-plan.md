<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Declared fault origins: frozen prerequisite plan

**Date:** 2026-11-07<br>
**Baseline:** `b0b4771698885e50d18a085e8f3441d6ac670715`, draft #46 after PR #65<br>
**Authority:** ADR-0018; no entry/wire/scenario/phase admission

Before the reviewed handler-entry wire, retain complete incoming declared origin
contexts. Current FaultRoute stores a scope and last active selection; it loses the
full active stack and does not associate exact general-source use sites. Adding a
history initializer directly would risk confusing that abstraction with complete
incoming source/selection evidence. This bounded raw SDK prerequisite is justified
by that inspected implementation dependency, not a change to P4 or existing routing.

Use a dedicated stacked PR and ADR-0051. Add a separate optional FaultOriginReport /
check_fault_origins judgment. Reuse one existing timing/routing pass and its exact
nearest-selection/exclusion/cycle algorithm; do not run admission a second time.
Keep check_fault_routes, its outputs, legacy work costs and all previous artifacts /
wires/core/source versions/digests/reports/package bytes unchanged. No new native
command, source role, entry value or scope field in this prerequisite.

For every activity declaration in each explored scope context, retain declaration
site/scope, ordinary syntactic reachability flag and exact retained core-0.6 general
source use index where available. Match supported-source routes only to that exact
source's declared emission names. Human/unbound/legacy contexts are unsupported
origins, never attributed to another provider from a matching name. Conservatively
explore every known fault name for unsupported activities, including ordinary-
unvisited declarations in explored scopes. Mark activity declarations in unexplored
scopes uncovered; neither flag supplies activation or proof that faults cannot occur.

Keep supported and unsupported origin cases explicit through read-only borrowed
route views: node/scope/source use or unsupported reason, selected handler/fault,
and full active selection stacks before and after the existing selection/exclusion.
Retain complete stacks, not just the top selection. Count declared unhandled origin
cases separately from legacy abstract scope/name counts, without creating events.
No actual emitter, attempt/occurrence, source truth, cause/input revision, protection
snapshot, nearest live handler, initialization/current authority or execution follows.
Engine faults remain uncatchable; an unsupported origin is not a business/engine
classification. Null general source never means no possible faults.

Index site/source/selection metadata once, reuse retained static source rows and
intern stack vectors; avoid copying metadata or strings per route. Charge origin
indexing, every site/name/context comparison, stack intern/copy, row storage and
unhandled counts to the same cumulative routing work allowance. No additional
quota, partial result or row truncation. Optional collection off adds no origin
walks/work/storage to existing legacy checks.

Test exact provider/single-attempt/noneligible routing; same-name other sources and
actual use identity; human/null/legacy/unvisited/unexplored cases; multiple and
nested handlers, failed selection exclusion and complete before/after stacks;
unchanged abstract routes and cycle refusals; sharing/cumulative exhaustion and
zero added legacy work. Add independent finite Python selection/exclusion vectors
consumed by Rust, plus exact current-package/native compatibility and required CI /
SPDX evidence. Run native/Wasm/all Python checks, separate completed-diff automated
self-review, all threads/CodeRabbit summaries, final-head CI and signed expected-head
merge; record the checkpoint.

Then freeze the explicit handler-local entry wire/core contract with all incoming
supported-source contexts and complete source/control provenance, fresh immutable
local initialization, current access and strict invalidation. Unsupported/uncovered
origins must not receive invented history values or coverage. Then readable
source/native entry integration and permitted owned continuation/cutoff/all-settlement.
Current package remains unchanged; parent draft/incomplete, all forty scenarios
Partial, and full validator/benchmark/general authoring/visual/protected editing /
actual host/user/browser/independent/phase gates remain open.
