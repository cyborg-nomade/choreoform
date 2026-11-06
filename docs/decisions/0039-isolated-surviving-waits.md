<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0039: Check declared isolation of synchronous surviving waits

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

The [pre-change critique](../evaluation/0039-isolated-survivors-audit.md), `ab1e939`,
reproduces the concurrency stage's blanket terminal-only refusal even for an
isolated wait-only follow-up. P6 already permits ordinary outcomes while explicit
reconciliation remains live. Treating every reconcile closure as settled would
hide mutable writers, overlapping effects or unaccepted ownership. The
[frozen plan](../evaluation/0039-isolated-survivors-plan.md), `445b23e`, instead
limits the new proof to declared isolation and retains live P6 premises.

## Decision

For a covered synchronous invoke/repeat, accept a nonterminal body closure only
when its exact direct-child follow-up has no storage, ports or nested scopes,
uses a terminal own closure, and contains only data-free timer-free waits and
finishes. Check every node, including unvisited declarations, and all relevant
scope-management policy read closures. Require entry at a wait and a reachable
wait-to-finish route through the existing acyclic control graph. A route supplies
no guarantee that any observation arrives or that terminal settlement is possible.
Keep terminal-only split/fanout requirements, all-member joins, ordinary shared
storage/conflict checks and all preceding strict stages unchanged.

This proves only absence of declared data/effect mutation by the surviving subset.
It does not prove an actual lifetime, external independence, authority, progress,
effect settlement or safe irreversible race. P6 still requires the occurrence
owner to exist before disposal, authenticated exact correlation, fresh authority,
accepted atomic single-owner handoff, retention of every transitive outstanding
obligation/subscription, late-fact deduplication/contradiction handling and complete
settlement before terminal disposal. Ordinary outcome publication remains blocked
when the live P6 ownership check fails. No host flag or true predicate discharges it.

Expose immutable `IsolatedFollowup` declaration evidence at each creator/body/
closure/follow-up/wait set. All native stages from concurrency onward disclose
`isolatedFollowups` and its outstanding host requirements. No occurrence IDs or
actual receipts are invented. Cache only definition proof templates within one
check, charge index/proof/evidence copying to the same cumulative budget, and
bound native record/final JSON publication before stdout. Do not recursively use
a later composed stage to justify its prerequisite.

## Alternatives and consequences

| Option | Benefit | Cost or risk | Decision |
| --- | --- | --- | --- |
| Explicit isolated subset plus outstanding host premises | Useful conditional ordinary composition without erasing ownership | Very conservative; reports require careful boundary disclosure | Adopt |
| Treat every reconcile closure as terminal | Broad acceptance | Conceals surviving mutation/effects and ownership | Reject |
| Change benchmark closures to terminal | Simplifies checking | Removes required late-fact/unknown-effect meaning | Reject |
| Keep the blanket refusal | Small implementation | Blocks data-independent composition permitted by P6 | Retain for parallel and all other unproved cases |

No source/core/IR meaning or revision changes. General reconciliation concurrency,
irreversible/domain invariants, exceptional protection, authenticated live host
enforcement and complete benchmark/phase evidence remain unresolved. Passing all
existing partial static stages does not admit a process or complete its scenarios.

[Evidence](../evaluation/0039-isolated-survivors.md) records tests and separate
completed-diff automated review. Acceptance is delegated, not individual owner or
independent review. Require final-head CI, complete comment audit and expected-head
signed-off merge into draft #46; no phase exit is granted.
