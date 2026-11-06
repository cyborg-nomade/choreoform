<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0032: Check surviving follow-up bodies without inherited values

**Status:** Accepted under Project Owner delegation; effective upon PR merge<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

Core P6 creates a no-port reconciliation child before ordinary owner disposal
and keeps it as a separate surviving ownership boundary. Declaration links and
source 0.4.0 now express this relation, but do not prove body data availability.
The creation trigger is first obligation registration, not successful ordinary
completion. Neither ordinary results nor continuing retention of an ancestor's
input/initializer values follow from that rule. Handler live-ancestor input
premises are inappropriate for a boundary that survives ordinary closure.

## Decision

Add `check_reconciliation_availability` after declaration links. For each actual
scope's reconcile closure use, check the designated follow-up with an empty
inherited cell set, fresh local initializers and existing ordinary child rules.
No ports are supplied in the pinned initial profile. Check declared contexts
conditionally, including those with no ordinary activation; do not invent an
occurrence or obligation. An ordinary or handler proof of the same template
cannot replace this separate context.

Use a shared bounded graph, initializer index and budget. Parameterize the private
entry checker's availability diagnostic; preserve existing handler codes and emit
`reconciliation-availability-unproved` at the actual failing node/data declaration.
Other static/control/budget errors retain their current conservative meanings.

Expose immutable owner/closure/follow-up references, host requirements, conditional
checked nodes/scopes and total uncovered nodes separately from prior ordinary/
handler coverage. The native report bounds expanded metadata and final output to
1 MiB. This proves a data-availability body judgment only, conditional on successful
activation, required storage lifetime and accepted execution of its steps. It
does not prove initial/current permission, actual values/obligations, observable
fault/timing channels, correlation, concurrent occurrences or owned settlement.

## Options and consequences

| Option | Benefit | Cost/risk | Decision |
| --- | --- | --- | --- |
| Zero inherited values, fresh local body proof | Conservative useful subset; no retention/snapshot guess | Refuses richer valid ancestor contexts until separately proved | Adopt |
| Assume all ancestor inputs/initializers | More permissive | Surviving boundary lacks retained live-value evidence | Reject |
| Reuse ordinary completion/result state | Convenient | Follow-up can activate earlier, and have different lifetime | Reject |
| Add retention/snapshot profile now | More precision | Needs a demonstrated full contract with provenance/permission/lifetime evidence | Defer |
| Execute a process host for evidence | Could observe concrete executions | Phase 2 engine is separate; observations alone do not prove all paths | Keep separate |

No language-wide ban on ancestor access is imposed; the new refusal is unproved
under this supported subset. Hosts must still preserve required storage, recheck
authority and accept exact atomic ownership/correlation/settlement operations.
Writing a previously unavailable ancestor cell does not authorize access or
prove retained storage. Full exceptional/follow-up information flow/invalidation/
concurrency, validator, benchmarks, cross-form/protected editing and actual users
remain open. No source/core/wire version changes or new dependencies are needed.

Follow [frozen plan](../evaluation/0032-reconciliation-entry-plan.md), `e44b192`.
[Evidence](../evaluation/0032-reconciliation-availability.md) records 242 Rust
tests plus five compile-fail doctests, native/Wasm and all relevant artifact/source/
stage checks. Separate completed-diff automated self-review found no remaining
actionable issue. Accepted under delegation; require passing final-head CI and
expected-head merge. No independent or individual owner review is claimed.
