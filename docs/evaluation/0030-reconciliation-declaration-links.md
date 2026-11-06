<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Owned reconciliation declaration evidence

**Date:** 2026-11-06<br>
**Plan:** [0030](0030-reconciliation-links-plan.md), `4b60cb6`<br>
**Decision:** [ADR-0030](../decisions/0030-reconciliation-declaration-links.md)

## Finding and judgment

The unchanged service example passes earlier stages with a unique ordinary wait,
terminal closure and no surviving follow-up. The new stage refuses this at the
owning scope closure. A direct-IR package with a declared child follow-up, correct
wait ownership and potential entry route passes the declaration judgment.

The checker indexes wait sites and ordinary reachability once, sharing the latter
helper with fault routing. It checks each declared capability activity's actual
scope rather than inferring from effect policy scope. Immutable use references,
ordinary-unvisited sites, unused effect contexts and host requirements retain
the distinction from live ownership and data-entry proof. Native output is bounded.

## Regressions and source boundary

Seven Rust groups cover prior-stage false confidence; exact direct-IR links under
both core profiles; ordinary/nested/unreachable wait refusals; a shared effect
policy in an incompatible activity scope; unused effect contexts; 500 additional
ordinary-unvisited activity declarations sharing the indexes; and human-only
definitions. Read-only provider metadata does not bypass required reconciliation.
The follow-up wait remains data-uncovered in prior reports: the link check does
not invent its activation snapshot, current authority or accepted ownership.

Four Python groups check both old source profiles' honest missing-follow-up
refusal after prior success, non-executable human/terminal report cycles, current
grammar/schema admission but lexical refusal of a qualified child endpoint, and
documentation/licenses. The owned source candidate is not reported as lowered:
the separately versioned endpoint-binding profile is explicitly the next task.
No dummy root wait, private data visibility change, old fixture edit or automatic
source migration is used to satisfy the tests.

## Verification and limits

Passed 229 Rust tests plus five compile-fail doctests, format/native lint,
semantic/frontend Wasm lint and release frontend Wasm build. All relevant artifact,
authoring/profile/frontend and static/control/protection/lifecycle/invalidation/
concurrency/clock/timing/calendar/retry/settlement/value/fault/exceptional and new
reconciliation Python oracles passed, including four new groups. Temporary logs:
`/private/tmp/reconciliation-rust.log`, `/private/tmp/reconciliation-all-oracles.log`.

Separate completed-diff automated self-review checked actual activity scope,
direct follow-up/wait context, potential reachability versus data proof, unique
indexed lookup, unvisited/unused distinctions, both core profiles, read-only
contracts, stable paths, bounded reports and unchanged old source/privacy rules.
The shared reachability helper preserves the fault-stage regression evidence.
No actionable finding remains. ADR-0030 is accepted under delegation; require
final-head CI/expected-head merge. No independent reviewer, individual owner
review or actual host/browser/process/user observation is claimed.

Actual follow-up creation before disposal, entry availability/protection,
authorized accepted atomic handoff, exact request/key/observation correlation,
durable late-fact handling, multiple-obligation wait progress and terminal disposal
remain mandatory host/lifecycle judgments. Declaration reachability does not prove
them. Full validator/benchmark/conformance/cross-form/protected editing/user gates
remain open; no Phase 2 engine is introduced.
