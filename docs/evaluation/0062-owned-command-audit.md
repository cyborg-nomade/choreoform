<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Critique: finite owned commands, compensation and constructed inputs

**Date:** 2026-11-10<br>
**Baseline:** `e1cdfc45d621a09c170eb3c5f1807efe17a0604d`, draft #46 after #76<br>
**Authority:** ADR-0018 / [ADR-0062](../decisions/0062-owned-command-direction.md)<br>
**Frozen audit:** `0d4f884`, [plan](0062-owned-command-audit-plan.md)

This changes tests/documentation only. Production behavior, all accepted source /
core/wire/schema/artifact/hash/report meanings and committed benchmark bytes stay
fixed. Actual SDK/frontend calls check synthetic static declarations and exact
metadata, not actual observations, requests, current authority, storage or handoff.

## Existing semantics distinguishes class, compensation and identity

ADR-0008 permits compensation linked to a confirmed or possibly completed prior
effect; it is new work with its own authority, failures and observable outcome,
never erasure or guaranteed restoration. P3 separately classifies the new capability's
own effect as read-only/compensatable/irreversible and its provider idempotency.
An irreversible compensator is not excluded merely by that class; a compensatable
activity is not automatically compensating another activity. Existing work.policy
compensates names a visible prior capability activity; it is a declaration reference,
not accepted occurrence correlation or proof a compensation is currently justified.

P3 gives new business work/compensation new logical keys and retains keys only for
technical attempts of the same logical effect. Stable-key metadata must match a
supported provider promise, request digest and occurrence identity; it is not an
engine or exactly-once guarantee. ADR-0008 additionally constrains consequential /
irreversible races through appropriate commutativity, invariant or conflict policy.
Pure true conditions and engine acceptance order do not prove business safety.

## Genuine current first refusals

Eight raw SDK groups and one frontend group establish:

| Probe | Actual result | Interpretation |
| --- | --- | --- |
| 24 combinations: three original effect classes × two new command classes × with/without explicit compensation × capturing/pre-capture entry | Static pass, then owned-continuation-query-effect | New class restriction is independent of original class, link and metadata mismatch |
| Read-only new work plus explicit compensation, for all original classes/both layouts | Static pass, then owned-continuation-query-policy | Class loosening alone leaves another real boundary |
| Matched one-attempt new commands with none or stable-key idempotency | Static pass, then query-effect | Matched metadata does not imply conditional command permission or safe dispatch |
| Mismatched key/provider and nonactivity compensation target | capability-effect / fault-source-provider / compensation-target | Genuine earlier failures, not later conditional proof |
| Correct empty-record construction or pure identity wrapper, both layouts | Static pass, then query-input | Current exact original-request syntax is stricter than purity/type correctness |
| Captured integer payload as the new capability input, with exact changed provider/source/input type and integer next-child request seed | Static pass, then query-input | Existing IR represents typed construction/new retained input type; ownership permission remains absent |
| Constructor node omits its capture read, or its result type mismatches capability input | node-reads / activity-input | Existing complete-read/type obligations remain real |
| Unvisited second otherwise supported capability in the body | Static pass, then owned-continuation-body-unproved | One-query report does not cover arbitrary hidden extra work |

The typed constructor changes only the new capability's input and next-child request
type. The original source/retained request remains an empty record; a shared seed
type cannot be assumed across levels. Normalized source capture output remains
integer. Capturing-first layout places that input use after its wait. The early-entry
variant reaches query-input before conditional availability; it supplies no evidence
that an early captured read would pass. Existing real early-read/capture-integrity
regressions remain unchanged and passing.

The frontend uses existing source 0.13 sentences for correct pure constructors and
a read-only compensates link. Moving its policy into the real recovery lexical
scope makes the original source visible; lowering/static checks pass before the
same query-input/query-policy refusals. No extra syntax/wire field or widened
visibility is needed. No accepted benchmark metadata was broadened to obtain a pass.

## Recommended next bounded contract

An explicit core should support supported pure local input expressions and expose
finite owned work separately from old read-only query semantics. Existing activity,
input expression, exact capability/provider/effect/work policy, optional compensates,
entry/flow/capture/request/closure relations represent the concrete shape. No missing
serialized relation has been found. Existing conditional control/protection machinery
already analyzes local reads, fresh initialization, complete policy/source/control
provenance, actual capability recipients and strict invalidation; reuse its bounded
chain/template analyses rather than inventing another validator or live grant model.

Keep exactly one capability activity per intermediate body and its own direct
surviving child. Preserve readonly original accepted request seeds after zero-live
pre-entry access, all-declaration/source coverage checks and passive leaves. Check
constructed input's actual type/complete reads under normal conditional availability
and protection. New command providers/effects should initially require stable-key,
one attempt, no timeout/retry clock and matching supported general source; their
own designated next child retains their actual accepted request, including a type
different from the parent's seed. A compensation link should name the immediate
source, with actual prior eligible-effect/occurrence/authority/conflict acceptance
remaining mandatory live host gates, never a declaration-only claim of rollback.

Initially require non-read-only work to enter at the original source's unique
actual-use timerless capture, with local uninitialized exclusively written target.
Explicit compensators use capture-first entry independently of their own class;
only uncompensated read-only inquiry retains early entry. This is a narrow
serial-after-terminal-capture declaration family, not complete
preterminal/possibly-completed compensation. Keep read-only pre-capture inquiry and
its old restrictions through inheritance; permit new pure inputs only in the new
core with the same availability/provenance/recipient checks. P4 history activation,
preterminal commands, extra capability activities/arbitrary nesting and general
compensation orchestration stay separate later work. All actual contradictory facts,
unresolved subscriptions/transitive effects and closure obligations remain owned.

Current query getters/row fields mean read-only work. Reusing them to label commands
would silently change report meanings. Add a distinct immutable work relation with
site/class/input/optional compensation and explicit new-effect/correlation/conflict /
provider/authority gates, preserving old query rows and ordinary/legacy coverage.
A new logical command never borrows the original source key or is called a retry.
No metadata report should dispatch, accept a gate, initialize actual storage, settle
an original source, return a parent result or claim liveness.

The audit itself changes no production rule. Freeze a separate signed raw plan and
exact core before implementation, then explicit readable/native selection later.
Record any concrete wire/contract gap as a signed refinement before changing scope.
Local validation passed: 522 Rust tests plus six compile-fail doctests, formatting /
native Clippy, all five Wasm Clippy checks, release Wasm portability/frontend builds,
native portability report and all 57 existing Python workflow commands, including
deterministic benchmark regeneration. Separate completed-diff agent review covered
correctness/security/performance/scope/evidence, metadata independence, typed child
seeds and actual first refusals; no actionable issue remains. Required final-head
CI/comment audit/signed expected-head merge remain before integration. Logs use `/private/tmp/owned-command-audit-{focused,workspace,verify}.log`.
Agent self-review is not independent/human or personal owner review. Parent #46
remains draft/incomplete, forty scenarios Partial and G1–G4 conditional; complete
validator/benchmarks/general near-English/protected accessible visual and actual
host/browser/user/independent/phase evidence stay open. No Phase 2 or exit.
