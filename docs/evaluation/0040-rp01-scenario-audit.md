<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-01 original-scenario audit after partial static checks

**Date:** 2026-11-06<br>
**Baseline:** `a89e481cddae40fea18684953d421bcdf0a1447e`<br>
**Plan:** [frozen audit](0040-rp01-scenario-audit-plan.md), `a76939e`<br>
**Evaluator:** Codex under ADR-0018; no independent or individual owner review

## Finding

The complete source graph and exact resources now lower and pass all ten existing
partial static stages. That establishes their documented judgments only. None of
the four original [RP-01 scenarios](../representative-processes/01-expense-reimbursement.md)
is demonstrated end to end. The next useful independent work is finite positive/
negative conformance for the actual guard expressions, followed by explicit
resolution of fault-source/continuation and surviving correlation/data requirements.
Do not declare the complete package done or treat this as an external blocker:
substantial test, contract, validator and authoring work remains independently doable.

## Original requirements and actual boundaries

| Scenario | Actual source evidence | Still missing |
| --- | --- | --- |
| A: ordinary reimbursement | `run_checks`, `manager_review`, `finance_review`, `may_pay`, `payment_commits`, `payment_request_step`, `record_paid`, `run_notice`, `finish_paid`; exact claim/evidence/provider resources and typed protected transfers | Positive/negative evaluation of actual policy/guard snapshots, complete transition/ownership evidence, current authenticated assignments, global business-identity uniqueness and truthful correlated payable confirmation. A stable technical key does not deduplicate separate claim instances |
| B: correction invalidates review | `finance_correction_reason`, `correct_claim`, `publish_correction`, `review_again`; new revision and repeat requested atomically; `may_pay` checks current claim/policy revisions | Finite old/new approval/correction predicate cases, complete invalidation/withdrawal history and occurrence evidence. Distinct business revision strings are claims, not authenticated immutable payload hashes; current host bindings remain required |
| C: timeout and ambiguity | `payment_work` names two attempts, explicit clock/basis and the exact eligible fault; `payment_close`/`payment_wait`/`payment_observe` retain an isolated declared follow-up | Checkable mapping from authenticated provider no-effect evidence to the retry fault, original request/effect/key correlation, accepted ownership and settlement, and explicit behavior after late confirmation/no-effect. No fault-data port, wait-result capture or handler-finish-to-original-activity continuation is supplied |
| D: withdrawal/approval race | `withdraw_request` employee authority, root cancellation, `payment_commits`, original revision dependencies and P6/P7 contract | Finite current-authority/guard cases plus complete conflicting-path/irreversible proof, atomic effective ordering, provider cutoff, stale/losing observation history and pending-owner settlement. Source order and a true guard prove none of this |

The source includes duplicate linking/human resolution, attributable rejection
reasons, correction and notice data. Sequential checks and omitted optional
business-calendar reminders are explicit chosen variants. Current assignment/
reassignment, policy truth and active employee/allocation still depend on supported
authenticated host facts/adapters. Notice is a distinct effect: confirmed payment
is stored first, but an unhandled notification fault has no invented ordinary paid
continuation. The package does not prove protected audience editing or privacy of
all exceptional/observable channels. All four scenarios remain Partial.

## Actual native probes

Temporary copies retained identical bindings/resources and changed only the
specified source clause. Each copy was lowered and checked through all ten native
stages; checked-in benchmark source was unchanged.

| Probe | Exact edit | Actual result |
| --- | --- | --- |
| Missing protective payment predicate | Replace the embedded predicate in `Rule for race "Payment commits"` with `(true)` | Lower and all ten partial stages pass, each non-executable. No domain/irreversible safety is proved |
| Missing manager completion checks | Replace `Complete when (...)` in `Manager review rules` with `(true)` | Lower and all ten partial stages pass, each non-executable. Static typing does not establish attribution/revision/separation semantics of a supplied Boolean |
| Unmapped retry-fault name | Replace `Retry faults ["confirmed no effect"]` with `["timeout means safe"]` | Lower and all ten partial stages pass. An arbitrary name supplies no truthful no-effect classification; remaining host/race gates still block executable admission |
| Attempt to retain late observation | Replace `Wait under "Payment reconciliation".` with `Wait under "Payment reconciliation" and record in "Recorded payment result".` | `syntax at 11313..11316`; no stdout/candidate. This result form is not in the accepted grammar |

Raw local results: `/private/tmp/rp01-scenario-audit.json`. Reproduce by applying
one table edit to a temporary source file and running `lower`, then the ten
commands in `tools/build_rp01_benchmark.py::STAGES`, supplying every exact resource
from the manifest. These are native static observations, not process executions,
authenticated provider facts or observations of an actual unsafe payment.

The current `payment_observe` node has empty writes and no result target. Its
typed wait predicate may inspect an incoming observation, but the initial node
contract does not bind that payload into a cell for later scoped decisions/work.
All fault-handler maps in this fixture are empty. A typed terminal-fact envelope
and an isolated wait-to-finish route therefore cannot be presented as a complete
ambiguous-payment recovery/retry/continuation program. Any new binding or fault/
continuation meaning needs its own explicit ADR/profile, with old bytes preserved.

## Next implementation and verification

First add a test-only finite snapshot harness for actual source-derived guards.
Use the production static checker, exact bound value/type validation and existing
pure parameter/invariant evaluator. Rebind declared read/fact leaves to explicit
immutable parameters only inside the test harness; never expose ambient host IO
or claim authenticated facts. Check the full original and rebound ASTs, preserve
literal data, short-circuit faults and nominal invariants, and keep snapshot
results distinct from policy permission or process transitions. Include old/new
approval, participant/revision separation, missing/invalid evidence and amount/
currency mismatch cases. These can reveal concrete predicate defects before a
public snapshot evaluator or any engine exists. Freeze that implementation scope
before code; keep C/D lifecycle requirements explicit and uncovered.

Then resolve the checkable fault-source and late-observation/continuation contracts
from concrete C requirements, rather than inventing a handler success outcome or
trusting a fault name. Continue full validator and cross-case work. Real host,
browser, independent and representative-user evidence remain additional gates;
do not substitute generated snapshots for them.

Separate completed-audit automated review compared every A–D clause and alternative
against actual source/IR/resource definitions, the whole-package plan, P1/P3/P4/
P5/P6/P7 contracts, all ten report residuals and native probe outputs. The audit
keeps guard typing distinct from proof/permission, separates input values from
observations and preserves the absent wait/fault continuation contracts. No
actionable documentation issue remains. Existing package/regeneration oracles
pass; final-head CI remains required for the draft checkpoint. No phase criterion
is checked off and draft #46 remains incomplete/unmerged.
