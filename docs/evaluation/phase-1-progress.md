<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Phase 1 execution checkpoint

**Updated:** 2026-11-05<br>
**Phase:** Semantic core and language prototype — incomplete<br>
**Workflow:** [ADR-0018](../decisions/0018-phase-review-delegation.md)

## Verified baseline

PR #21 supplies exact resource readers and the static type/expression/policy
stage. PR #22 supplies conservative ordinary flow/initialization checking; the
owner approved it on 2026-11-05. Its three review findings were fixed in
`c53e4d4`, approval recorded in `d414e97`. Required CI passed at the final head;
PR #22 merged as `158207452781a3488d6c2e84a288346374da69bd`. No executable admission, clocks, engine or full benchmark
claim follows from either partial report.

## Execution order and remaining evidence

1. [x] Finalize PR #22; record the delegated workflow in its own PR.
2. Implement control-sensitive information-flow checks and explicit access
   obligations, retaining the distinction between static proof and host authority.
3. Complete concurrency/invalidation/reconciliation and exceptional-flow
   judgments, with an explicit supported-clock contract/host boundary proposal.
4. Complete structural/link/semantic validation and actionable stable diagnostics;
   close supported/unsupported cases explicitly rather than relabeling a subset.
5. Expand RP-01/03/08 into complete source/IR benchmark packages and conformance
   cases; keep all forty corpus scenarios in coverage/gap accounting.
6. Implement identity-preserving general text/IR export/editing and full round
   trips, plus the initial visual notation/layout format and visual/IR cycles.
7. Supply accessible matched authoring/visual materials and representative-user
   evaluation. Actual participant observations remain necessary; generated or
   agent-only assessments cannot close this gate.
8. Audit ADR-0009 G1–G4 and every Phase 1 exit criterion. Report achievements,
   evidence and open limits to the owner, then stop for approval before Phase 2.

Items may be split into coherent PRs or reordered where verified dependencies
justify it. Update the record on each merge rather than asking to start each item.

PR #23 merged the delegated workflow as
`384e3add214bbf1be7aecbf520a9c027d563898a` after recorded automated self-review
and passing final-head required CI. CodeRabbit automatic review was unavailable;
no independent bot/human review is claimed. The next active slice is the
[protected ordinary-transfer plan](0019-protection-plan.md), committed before code.
PR #24 merged as `ecc29c965191504b18bd98c75d6d9f64f44afe3d` after automated self-
review (including fixes for destination access-policy dependencies and comparison
work accounting) and passing final-head CI. CodeRabbit skipped automatic review;
no independent review is claimed. The current follow-up is the
[initial access dependency plan](0019-initial-access-plan.md). No phase-level gate
is closed by these partial reports.

## Honest exit accounting

The near-English authoring, complete validator, text/visual round trips, executable
examples, conformance and aggregate executable-IR gates remain open. So do complete
RP-01/03/08 evidence, protected accessible editing, visual parity and user studies.
The Phase 2 reference engine is separate. Phase 1 has not reached its report/
approval boundary, and no next-phase chat is authorized yet.
