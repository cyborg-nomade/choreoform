<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0030: Require an owned reachable reconciliation declaration route

**Status:** Accepted under Project Owner delegation; effective upon PR merge<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 delegation

## Context

P3 requires a reachable declared reconciliation path and owned follow-up; P6
distinguishes that surviving boundary from ordinary terminal closure. ADR-0020's
unique wait-site count resolves a declaration reference only. The existing service
fixture passes prior stages with terminal closure and an unattached ordinary
wait, so it supplies neither required declaration relation nor live ownership.

Close this specific link gap with exact references and useful refusals. Preserve
existing partial APIs/fixtures and distinguish declaration reachability from
entry data, current permission, accepted handoff, provider facts and settlement.

## Decision

Add `check_reconciliation_links` after conditional handler availability, with
immutable ReconciliationReport/ReconciliationUse and Candidate/native adapters.
For every declared capability activity, resolve its capability, effect policy,
reconciliation wait policy and uniquely inferred wait node. Require its own
scope's reconcile closure to name a direct child follow-up containing that wait
node, with a potential ordinary route from the follow-up entry to the wait.

Reuse one bounded wait-site index and ordinary graph/reachability index. Share
the same reachability helper with fault routing; it assigns no data values or
predicate truth. Validate every declared activity use, including ordinary-uncalled
ones, without inventing a runtime context. Report those unvisited sites and unused
effect-policy contexts separately. Human-only definitions have no P3 capability
use. Read-only providers retain the closed effect record's reconciliation and
subscription requirements; metadata asserting read-only is not a host proof.

Missing follow-up refuses `reconciliation-followup-unproved` at the scope closure;
wrong wait context refuses `reconciliation-context-unproved` at the effect's
reconciliation reference; unreachable wait refuses `reconciliation-route-unproved`
at that node. These are conservative supported declaration conditions, not a
language-wide prohibition of deeper/delegated ownership that a later explicit
profile can prove. Never pick an owner from policy declaration scope or map order.

Expose exact activity/scope/capability/effect/closure/follow-up/wait references and
host requirements. A shared declaration can serve multiple potential sites, but
does not prove actual wait progress, logical-effect correlation or completion of
every owned obligation. No subscription/obligation set, successful activation,
accepted handoff, current authority, provider truth or atomic settlement is supplied.
Prior ordinary/conditional/routing/timing coverage remains separate and reports
stay non-executable. Native expanded metadata/final output is bounded to 1 MiB.

## Source dependency

Raw IR already expresses a root wait policy used by a wait in its designated child
follow-up. Current source profiles resolve wait-outcome owners lexically, so a
root policy cannot name that private child node. Preserve that behavior for old
profiles. A dedicated source revision must support an explicitly qualified
reconciliation endpoint within the declared follow-up boundary, with matching
wait owner/policy and ordinary lexical/data privacy retained. Do not proxy the
endpoint with a dummy root node or broaden arbitrary descendant references.

This PR provides direct-IR positives and source-boundary refusals. Freeze/review
the new source profile separately before implementing it; retain the explicit
Roadmap item until that follow-up is complete. No core/wire revision is needed
for the existing raw-IR relation, and no source version is silently changed here.

## Alternatives and consequences

| Option | Benefit | Cost/risk | Decision |
| --- | --- | --- | --- |
| Own-scope direct follow-up and reachable unique wait | Exact conservative P3/P6 declaration evidence; fits existing IR | Deeper/delegated ownership and entry/lifetime proofs remain unproved | Adopt |
| Accept any unique ordinary wait | Small | No surviving boundary or owned route | Reject |
| Infer ownership from effect policy scope | Convenient reuse | Activity may live in another scope with incompatible closure | Reject |
| Exempt read-only contracts | Fewer links | Ignores mandatory closed record and outstanding subscriptions/uncertainty | Reject |
| Change the IR or build a runtime now | Can express richer schemes | Premature version/engine work before validating this existing relation | Defer to demonstrated needs/proper phase |

Follow [frozen plan](../evaluation/0030-reconciliation-links-plan.md), `4b60cb6`,
and [evidence](../evaluation/0030-reconciliation-declaration-links.md). The full
validator, source follow-up, exceptional protection/invalidation/lifetime, complete
benchmarks, text/visual/protected editing and real user evidence remain open.

Accepted after separate completed-diff automated self-review under delegation.
229 Rust tests plus five compile-fail doctests, native/Wasm checks and all relevant
artifact/source/stage oracles passed locally. No actionable finding remains;
final-head required CI and expected-head merge remain prerequisites. No independent
reviewer, individual owner review or actual host observation is claimed.

1. [x] Verify direct-IR links, refusal/coverage/performance and source boundaries.
2. [x] Record delegated acceptance after separate review; require final-head CI.
3. [ ] Complete the explicit qualified endpoint source profile next.
4. [ ] Continue remaining Phase 1 lifecycle/validator and aggregate evidence.
