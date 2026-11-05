<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Ordinary invalidation declarations: frozen implementation plan

**Date:** 2026-11-05<br>
**Baseline:** `14cdfa0147e9c0b0f324e02b29a057dbd0dd689e` (PR #28)<br>
**Authority:** ADR-0018 delegation; accepted ADR-0013 P7 invalidation rules.

Add an explicit ordinary invalidation check after protection/control/static
checks. Collect a conservative pre-result cell dependency set at visited decision,
activity (human/capability) and wait declarations using ordinary control-origin
propagation plus declared/access reads. Union repeated contexts. Do not include
a site's own newly produced result simply because it writes that cell. Provider
observations and other non-data changes remain explicit runtime/full-validator
obligations, not invented data invalidation links.

Require each visible dependency cell's `invalidates` set to name the dependent
site. No declaration makes data immutable; do not assume a current input can
never be revised. If a dependency cannot be represented in the dependent node's
lexical context, refuse as unproved rather than ignoring it or inventing a link.
This conservative origin approximation can refuse valid programs requiring more
precise value/occurrence provenance. Record that analysis boundary in ADR-0021.

Return immutable revision-bound evidence, checked and uncovered consequential
sites, and sorted dependency obligations. Keep earlier stage contracts stable:
collect extra dependencies only for the new explicit stage. Source/CLI integration
must retain `executable:false`. No syntax/wire/resource/dependency migration.

Budget all added traversal, merges, saved context members, link membership and
visibility work. Tests must show earlier stages pass before the new refusal,
positive direct/control/access dependencies, repeated contexts, branch unions,
own-result exclusion, unvisited sites, cross-context refusal, stable diagnostics,
source format/reparse/revision cycles and native/Wasm parity of compilation.
Run required checks, review separately, record delegated acceptance and verify
final-head CI before expected-head merge. Continue concurrency/lifecycle/clocks;
no full invalidation/runtime, benchmark or aggregate Phase 1 gate closes here.
