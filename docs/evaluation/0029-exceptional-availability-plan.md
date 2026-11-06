<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Conditional handler-body availability: frozen plan

**Date:** 2026-11-06<br>
**Baseline:** `a20049d51a15052a8457a66c320660e623c9a131` (PR #38)<br>
**Authority:** ADR-0018 delegation; ADR-0013 E2/P4 and ADR-0028 route coverage.

Publish ADR-0029 and a separately named conditional exceptional-availability
stage after fault routing. Analyze only handler targets with discovered routing
contexts; never invent a context for uncalled handlers. Derive an explicit entry
assumption set from the handler's lexical ancestors' declared input cells, whose
types/ownership are already checked. These are occurrence-creation input contracts,
not observed values, current revision/permission facts or live-owner evidence.

Use no prior work result, ordinary-path local initializer/write, sibling storage,
fault payload or earlier handler-local value as a guaranteed ancestor entry value.
This deliberately conservative lower bound can refuse valid fault-site-specific
snapshots until a supported proof/profile supplies them. A body proof is conditional
on successful activation and live owning ancestor occurrences with their bound
input revisions; activation failure is not proof that the handler body ran.

Reuse ordinary initialization/availability checking for each selected handler
body and its ordinary children, with fresh locals on every entry. Own initializers
use the pre-entry ancestor/input snapshot and keep the existing no-peer ordering
rule. Nested selected handlers are checked separately with their own minimal
ancestor-input assumptions; an ordinary use of the same template cannot stand in
for the handler context. Retain exact static/source metadata dependencies and
existing conservative parallel/repeat/output rules. Report entry assumptions,
conditional checked nodes/scopes and remaining coverage distinctly from prior
ordinary and routing coverage. Refuse missing availability with a fixed exceptional
code and the actual failing declaration path; return no partial proof or execution
permission. Fault-control protection, invalidation, current authority, concurrent
handler occurrence/lifetime and actual snapshots remain separate obligations.

Before implementation reproduce the existing repeated full-data scan bottleneck:
many visits to an empty child plus unrelated uninitialized storage exhaust the
old availability budget despite requiring no local initialization. Index immutable
initializer IDs by declaration scope once per analysis. Reuse that private bounded
index in control and protection, replacing per-occurrence all-data scans while
preserving fresh locals, input/initializer order and work charges for actual
index/visit/copy work. Do not cache runtime state, authorization or initializer
values. Reuse one graph/budget for the new handler-entry checks rather than
rebuilding the whole graph per handler.

Tests: minimal declared-input assumptions; local literal initialization and no
peer reads; rejection of owner initializers/work results absent explicit input
contracts; distinct ordinary/handler contexts; nested handler assumptions; uncalled
coverage; transitive access metadata availability; useful value-free failure paths;
the repeated-empty-scope/unrelated-storage before/after regression; source/schema/
revision/format/report cycles in both profiles with current-host caveats. No core,
source/schema version, old snapshot/fixture or third-party dependency changes.
Run relevant native/Wasm/artifact/stage checks, self-review separately, record
delegated ADR acceptance and require final-head CI/expected-head merge. Continue
exceptional protection/invalidation/lifetime and remaining Phase 1 work afterward.
