<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0077: Refine fixed direct all-join completion provenance

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-11<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

Signed [plan](../evaluation/0077-capture-inherited-context-plan.md) `0a04032`
precedes probes, after #91 passed final-head CI and merged. The
[pre-correction critique](../evaluation/0077-capture-inherited-context-audit.md)
records the actual full strict/advisory refusal: private authorization-branch
invocation output `authorize_call_value` stays live at `capture_activity`.
Three baseline groups distinguish synchronous completion, fixed all versus partial
any join, and an unvisited nonlocal writer. ADR-0037 deliberately leaves raw fork
summaries live, but the IR contract's direct all predicate requires every fixed
member to have a terminal outcome. Validated control prevents unrelated ordinary
join activation.

## Decision

Expand only the conservative historical-proof subset at a paired direct nonempty
`all` predicate for a fixed `split`. Keep a budgeted pre-child live snapshot and
refine the exact member-scope origins at its join. A cell must belong to that exact
completed child, have no declared nonlocal writer (including unvisited descendants,
handlers/followups and port writes), and not already be live in the inherited split
snapshot. Reuse ADR-0037's conditional ordinary closure/immutable snapshot premise,
original protection labels and typed boundary/site/scope/closure evidence.

This supersedes ADR-0037's limitation only for that proved all-join boundary. Raw
split summaries, dynamic fanout, any/threshold/composite joins, arbitrary descendant
cells, nonlocal writers and inherited live template influences stay unproved/live.
Protection-only analysis and all source/core/IR/policy meanings, public diagnostic
privacy, budgets and host requirements remain fixed. Existing contracts already
supply terminal member and ordinary closure semantics; no new dialect or pin is
required for this bounded analysis refinement. Do not drop private influences or
make private cells readable/invalidation-addressable across sibling scopes.

## Alternatives and consequences

| Option | Decision |
| --- | --- |
| Keep every fixed all-join unproved indefinitely | Rejected for this exact-owner/no-writer subset supported by existing terminal semantics |
| Discard private branch influences at split creation | Rejected; children can still be live and partial joins leave unfinished members |
| Freeze every descendant or every true join predicate | Rejected; ignores writers, inherited live occurrences and dynamic/partial membership |
| Exact direct all-join refinement with original labels and budgeted snapshot | Selected, subject to counterexamples/full verification and recorded review |

A historical row is a conditional declaration, never an actual child outcome,
value, owner/receiver grant, accepted effect/compensation or all-obligation settlement.
Current authorization, occurrence/storage/atomic closure snapshots and every
concurrency/lifetime/exceptional/host/browser/user/independent gate remain separate.
Preserve exact previous benchmark evidence before adopting any changed actual
result; do not relabel forty/all four scenarios Supported or close Phase 1 from
this refinement. Parent #46 stays draft. Required tests/final-head CI, separate
automated review/full comment audit and signed expected-head merge precede durable
progress. No Phase 2.

[Implementation evidence](../evaluation/0077-fixed-all-join-history.md) records
indexed membership, retained counterexamples/work bounds, immutable #91 evidence
and precise actual invalidation adoption. Four full checking views and one distinct
advisory pass; later views retain the actual parallel-effects refusal. No source /
IR-version/core/policy/authority extension or host acceptance is supplied.
