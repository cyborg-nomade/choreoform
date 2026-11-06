<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Phase 1 validator and evidence gap audit

**Date:** 2026-11-06<br>
**Baseline:** `ac6edfa6402c67575de4740c83017e2d97d836bb` (PR #36)<br>
**Authority:** ADR-0018 delegated review; [frozen plan](0028-value-conformance-plan.md).

## Conclusion

The code now checks substantially more than the original structural probe, but
there is no complete semantic-admission API. Each report deliberately lists
residual obligations, and unvisited declarations are not assigned invented entry
contexts. Passing final-head CI proves its recorded checks, not complete language
conformance. Phase 1 and ADR-0009 G1–G4 remain open. The Phase 2 reference engine
is a separate deliverable; Phase 1 still owes precise operational rules and
checkable positive/negative examples without claiming a running process host.

This audit consolidates current code and accepted-contract/evidence records; it
does not independently re-execute all forty corpus scenarios or upgrade historical
paper evidence. The settlement receiver dependency omission found during the
audit was reproduced and fixed in PR #36, with exact read paths and work charges.
The current change begins [value-expression conformance](../conformance/README.md).

## Judgment inventory

| Area | Implemented evidence | Remaining judgment/evidence |
| --- | --- | --- |
| Transport, revision, record links | Strict bounded transport, exact bytes/pins, canonical revision, structural roles and selected lexical links; IR/source negative tests | A unified complete-validator contract and stable field/source diagnostics; inert unknown preservation must remain distinct from admission |
| V1/V2 types and literal values | Closed descriptors, exact integer/decimal encodings, containers/options/variants, nominal resource identity/invariant checking | Full benchmark type use and conformance breadth; static type equality supplies no domain/business proof |
| E1 pure value operations | Full static AST checking; private invariant evaluator; new exact value/fault vectors | No public general process evaluator; operator catalogue coverage is not all possible cases |
| E2 process snapshots | Exact full-branch read sets, declared parameters, conservative initialization/creator footprints; tiny earlier synthetic probe judgments | Actual bound snapshot/revision consumption, full read permission/current facts and atomic assignment publication; do not treat a caller value map as authorization |
| E3 policy facts/P1 authority | Closed typed fact contexts and actor requirement references/dependencies | Current authenticated principal/assignment/observation/occurrence/revision facts and acceptance state; facts supplied to a pure function would still be assertions |
| P2 information flow/access | Ordinary control-sensitive labels, initializer/input/output/provider origins, transitive policy metadata, explicit access obligations | Exceptional/control-observable channels, richer safe flow proofs and protected editing; actual current purpose/authority/capability enforcement |
| P3 capabilities/effects | Exact provider metadata/observations, types/effect classes, read dependencies and reconciliation wait-site links | Actual provider implementation/trust, request/correlation/key/no-effect proofs, compensation/late-success transitions and ownership; metadata assertions are not those facts |
| P4 retry/fault routing | Core 0.2.0 explicit clock/basis, source 0.3.0 clause, distinct protected sources, immutable retry uses; local handler child/port links | Missing static nearest-handler/selection-invocation acyclicity judgment; no handler entry availability/protection contexts. Accepted fault stamps, original request/input/basis, budget and effect safety remain live obligations |
| P5 waits/time | Exact wait outcomes, known clock/source/calendar pins, relative interval construction/membership and timing uses | Actual registered instance domain/origin, trusted observations/construction, due/basis provenance, authenticated matching, pause/change authorization, replay and atomic fire/cancel |
| P6 lifetime/settlement | Actual-use handler/follow-up links, unique effect wait declaration, ancestor receiver links and management-authority reads; conservative ordinary terminal closure rule | Live occurrence liveness, exact obligation/subscription sets, accepted atomic transfer, cancellation propagation, surviving reconciliation and full exceptional ownership proofs |
| P7 parallel/races | Bottom-up scope footprints, fresh-local separation, incoming origins, conservative shared/effect/lifetime refusals and ordinary data invalidation coverage | Supported proof of conflicting guarded/irreversible paths, non-data/exceptional invalidation, keyed membership/seal transitions and actual runtime reevaluation. A Boolean invariant alone cannot certify safety |
| Coverage/composition | Ordinary root/call coverage, root input assumptions, unvisited sites/scopes reported, bounded graph and metadata work | Complete exceptional/reconciliation entry contexts and combined validation/refusal accounting; do not turn a partial report into an executable plan |

Source references: [semantic-check](../../crates/semantic-check/src/lib.rs),
[policy checks](../../crates/semantic-check/src/definition/policies.rs),
[lifecycle links](../../crates/semantic-check/src/definition/lifecycle.rs),
[control](../../crates/semantic-check/src/control.rs),
[protection](../../crates/semantic-check/src/protection.rs),
[invalidation](../../crates/semantic-check/src/invalidation.rs),
[concurrency](../../crates/semantic-check/src/concurrency.rs),
[timing](../../crates/semantic-check/src/timing.rs),
[source frontend](../../crates/authoring-frontend/README.md), and
[accepted dialect rules](../dialects/README.md). Later pinned clock/calendar/retry
profiles supplement these rules; earlier frozen artifacts are not reinterpreted.

The subsequent [ADR-0028 routing slice](../decisions/0028-conservative-fault-routing.md)
adds a bounded active-stack overapproximation for P4 selection/cycle checking.
It partially addresses the missing graph judgment above; exceptional entry data,
concurrent fault/handler occurrences and runtime ownership remain unproved. The
baseline matrix is not retroactively relabeled complete.
The subsequent [ADR-0029 body slice](../decisions/0029-conditional-handler-availability.md)
checks availability under explicit ancestor input contracts. It does not supply
fault-site snapshots, exceptional information-flow/invalidation or actual
activation/lifetime facts. Its scope index removes repeated full-data scans.
The subsequent [ADR-0030 declaration slice](../decisions/0030-reconciliation-declaration-links.md)
checks direct follow-up ownership/entry-to-wait links per capability activity.
Actual handoff, entry data and multi-obligation lifetime remain unproved. The
qualified child-endpoint dependency is supplied by
[source 0.4.0 / ADR-0031](../decisions/0031-qualified-wait-endpoints.md), with
ordinary lexical/data privacy retained. This does not supply the live or entry
judgments above.

## Distinct kinds of remaining work

Missing static analysis is implementation work: P4 handler-selection/invocation
graphs, exceptional entry contexts, full conflict/invalidation coverage and unified
diagnostic/accounting remain concrete obligations. Conservative `unproved` refusals
are safe current behavior, not new language-wide bans or proof that all valid
processes are supported. Where the accepted contract lacks enough information to
prove a case, propose an explicit supported contract/profile or keep the refusal;
do not insert a safe flag, implicit source or untrusted assertion as proof.

Live host requirements are different: current principal/purpose, truthful provider
facts, trusted clock registration and atomic ownership cannot be manufactured by
static analysis. Phase 1 can specify and test their contracts with labeled synthetic
cases; the actual target must enforce them before execution. Do not move a reference
process engine into this phase merely to relabel the partial reports executable.

Evidence/product gaps are also independent. The existing review/service/control
fixtures and reusable test builders are small synthetic packages, not complete
RP-01/RP-03/RP-08 representations. All forty historical Partial rows remain
accountable until full packages and exact scenario outcomes replace those gaps.
Formatting an original package is not arbitrary-IR export or a general editor.
There is no initial visual notation/layout implementation, full visual/IR cycle,
protected accessible editing product or representative-user observations.

## Gate and exit accounting

| Gate/criterion | Current disposition | Required next evidence |
| --- | --- | --- |
| G1 corpus accountability | Conditional; historical 0 Supported / 40 Partial retained | Complete three benchmark packages and all forty explicit scenario/gap mappings |
| G2 semantic accountability | Conditional; accepted closed contracts and expanding executable judgments | Remaining validator/fault/lifecycle/proof obligations plus complete operational positive/negative examples; actual hosts must still satisfy their boundaries |
| G3 text/visual parity | Conditional | Full benchmark text/IR/text and visual/IR/visual cycles; identify representation-only layout |
| G4 protected accessible meaning | Conditional | Complete policy interpretation and protected accessible editing/materials; actual representative-user evidence |
| G5 reproducibility | Earlier structural-scope pass retained | Keep current exact revisions/artifacts/commands and disclose self-review limits; no independent reproduction is invented |
| Phase 1 exit | Incomplete | Three processes in both forms, stable semantic round trips, actionable invalidity diagnostics, operational semantics/tests for every accepted construct and representative-user authoring evaluation |

The [Roadmap](../../ROADMAP.md), [ADR-0009 evidence](0009-canonical-ir.md) and
[ADR-0007 gates](../decisions/0007-design-evaluation-framework.md) remain governing.
Only the owner approves the phase report/closure. Delegated PR self-review is
recorded honestly; it is not independent human/bot review. Prepare and finish
independent implementation/material work before treating missing participants or
independent observations as an external blocker. No recruitment/message is sent
without explicit authorization.

## Next dependency order

1. Land the exact value-expression vectors and this consolidated audit; keep
   process read/fact contexts explicitly separate from invariant evaluation.
2. Freeze a P4 handler-selection/invocation judgment proposal with explicit fault
   identities, lexical precedence, failed-selection exclusion and cycle cases.
   Determine which graph edges the existing contract can prove and which require
   an explicit emission/profile extension; do not guess from retry-eligible faults.
3. Complete exceptional/reconciliation entry and lifetime/protection/invalidation
   judgments and unified static/refusal/source-diagnostic accounting.
4. Complete the three benchmark packages/all forty mappings and grow conformance
   against the verified semantics; include the conservative refusals honestly.
5. Finish identity-preserving text export/editing, visual notation/layout/parity,
   protected accessible materials and actual user evaluation; audit all exit gates.

This order is dependency guidance under existing delegation, not a new approval
pause, roadmap deferral or Phase 2 authorization. Narrow correctness findings may
be fixed first with frozen plans and regression evidence, as in PR #36.
