<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Conditional handler-body availability evidence

**Date:** 2026-11-06<br>
**Plan:** [0029](0029-exceptional-availability-plan.md), `6331fb7`<br>
**Decision:** [ADR-0029](../decisions/0029-conditional-handler-availability.md)

## Judgment and boundary

For routing-discovered handler targets, the new stage records explicit ancestor
input-cell premises and checks body initialization/availability with fresh locals.
It reuses one graph/budget, rechecks a template's exceptional role even if its
ordinary role passed, and gives nested handlers separate minimal premises.
No ordinary work result, owner initializer, peer local, sibling cell or fault
payload is silently credited. Missing availability refuses at the actual node/
data declaration with a fixed exceptional code and no protected values.

Reports separate conditional body coverage, prior ordinary/routing coverage and
current-host obligations. Premises require successful activation and live ancestor
occurrences with bound input revisions; they are not observed facts or authority.
Fault-control protection, actual permission/sufficiency/invalidation, activation
failure/concurrent occurrence lifetime and accountable ownership remain unproved.
The native report has a 1 MiB metadata/output bound and prints no partial report
on budget refusal. No new wire/profile, public runtime evaluator or dependency.

## Performance and regressions

Before changes, 1,000 visits to an empty child plus 1,000 unrelated uninitialized
cells passed static payload checks but exhausted the ordinary availability work
budget (`/private/tmp/exceptional-scan-before.log`). The new private per-scope
initializer index is built once, stores immutable borrowed IDs only and is shared
within each control/protection analysis. The same definition now passes both
ordinary availability and protection without visiting the unused storage scope or
creating access obligations for its cells. This is before/after quota evidence,
not a wall-clock speedup assertion or synthetic host execution.

Seven Rust groups cover that case and conditional ancestor input premises,
owner-initializer/ordinary-role refusal, fresh own/peer initialization, separate
nested handler assumptions, uncalled coverage and transitive management-access
metadata availability. Three Python groups cover both source-profile grammar/
schema/JCS/revision/format/report cycles with explicit input premises and retired
initializer identity; refusal of an unproved owner-initializer snapshot after
routing success; documentation/licenses. No actual input values or current
authority facts are supplied to the CLI. The input-bound and initialized-value
fixtures intentionally describe different processes/premises.

## Verification

Passed 222 Rust tests plus five compile-fail doctests, format/native lint,
semantic/frontend Wasm lint and release frontend Wasm build. All relevant artifact,
authoring/profile/frontend and static/control/protection/lifecycle/invalidation/
concurrency/clock/timing/calendar/retry/settlement/value-conformance/fault-routing
and new exceptional-availability Python oracles passed. Temporary local logs:
`/private/tmp/exceptional-rust.log`, `/private/tmp/exceptional-all-oracles.log`.

Separate completed-diff automated self-review checked input-contract derivation,
no ordinary-result/initializer credit, fresh local/peer initialization, separate
ordinary/handler roles, nested/unvisited coverage, exact metadata dependencies,
scope-index identity/value isolation, declaration diagnostics and report bounds.
Review ensured one budget flows from premise construction through the shared
body graph/index/visits. Existing ordinary regression tests remain in force; the
scope index creates no unused-storage access or runtime cache. No actionable
finding remains. ADR-0029 is accepted under delegation; require final-head CI and
expected-head merge. No independent reviewer, individual owner review or actual
host/browser/process/participant observation is claimed.
