<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Actual original-request region proof: prerequisite critique and evidence

**Date:** 2026-11-12<br>
**Frozen scope:** `2026bd7`, [plan](0082-request-region-proof-audit-plan.md)<br>
**Dependency:** #96 signed merge `c0ab0c0b66874490f932780ad1ccbab7b85975ec`,
CI `43242066147`, automated COMMENT review/fresh full audit `5876581681`

## Actual first pair and request paths

The actual whole case still compares accept_b against accept_a at start_acceptance
and refuses parallel-effects-unproved after shared storage independence passes.
[Audit78](0078-parallel-effects-contract-audit.md) records the exact bounded tuple.
Core13 adds selection metadata, not a new concurrency judgment; the
[derived whole raw test](0081-interference-definition-binding.md) preserves it.

| Declaration | Reservation A | Reservation B |
| --- | --- | --- |
| Actual caller | reserve_a_call in accept_a | reserve_b_call in accept_b |
| Input expression | reserve_a_call_instruction | reserve_b_call_instruction |
| Declared common read | original_order | original_order |
| target warehouse AST | some(first line.warehouse of unwrapNamed(original_order)) | some(second line.warehouse of unwrapNamed(original_order)) |
| Invoke child / port cell | reserve_a_service / reserve_a_request | reserve_b_service / reserve_b_request |
| Activity input expression | reserve_a_input reads reserve_a_request | reserve_b_input reads reserve_b_request |
| Actual activity | reserve_a_activity | reserve_b_activity |
| Recovery declaration | reserve_a_recovery | reserve_b_recovery |
| Typed original-request cell | reserve_a_retained from reserve_a_activity | reserve_b_retained from reserve_b_activity |
| Terminal capture origin | reserve_a_observe captures reserve_a_activity | reserve_b_observe captures reserve_b_activity |

The real compiled AST/port/type/retention/capture identities are checked directly.
They do not select actual occurrence IDs or prove the two reads used one accepted
revision snapshot. Actual activity reads include both its local request and the
original-order dependency. The retained-request declaration and terminal origin
name original work; a current replacement cell cannot establish its old request.
Neither metadata getter returns a live accepted request or physical key.

## Five real SDK groups: mathematical limits, not event evidence

The audit uses unchanged current types/provider/launch/candidate bytes and real
V1/V2 literal judgments. It adds no fake state, accepted event or host Boolean.

1. Check actual request construction, invoke-port binding and local input ASTs.
2. Check original activity retention/capture declarations and exact request types.
3. Validate two synthetic Order values under the actual nominal invariant: each
   has different first/second warehouse strings. Swap those strings in a new
   mathematical revision: old first now equals revised second. This refutes the
   universal **input-type** cross-revision disequality proposition.
4. Validate Order labels `DEPOT-A` and `depot-a`: raw strings differ, while a purely
   hypothetical ASCII case-fold interpretation maps them together. No such
   normalizer is registered/selected here. The actual type is Text with nonempty /
   raw inequality constraints, not a canonical physical warehouse identity type.
5. Validate original and changed complete reservation provider-input records:
   both type-check, yet differ in request/order revision and target warehouse.
   Input type correctness cannot make changed data the same original request,
   a technical retry or a correlated old capture.

These are mathematical counterexamples/declared links. They are **not** accepted
business edits, valid actor permissions, actual input revision vectors, dispatches,
provider consent, simultaneous active work or a trace of permitted execution.
The current fixture's fictional warehouse strings also supply no physical mapping.
The [inactive pure model](../dialects/drafts/external-interference.md) independently
shows the conditional alias/read/write rule; its tables remain assumptions.

## What the accepted lifecycle rules actually establish

The immutable [C1 artifact](../ir/contracts/sha256-26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58.txt)
P3/P4 retain the original request, input revision vector and logical effect across
technical attempts; changed business input is new work. P6 stops ordinary work in
a cancelled scope, preserves uncertainty and requires actual live authorized
receiver acceptance/atomic ownership transfer before disposal. Reconciliation
and ordinary closure differ; terminal instances do not reopen. These constraints
must not be replaced with a mathematical type-validity test.

P7 says a new data revision withdraws dependent work's sufficiency, retains prior
records/effects, requests withdrawal/cancellation for active work and gives renewed
work fresh occurrences. Invalidation does not undo effects. The immutable
[semantic contract](../ir/contracts/sha256-0d353f015c758acd70f01bba74724981932915947f54b135a58d3554f6411141.txt)
likewise separates cancellation request, unknown outcome, live reconciliation and
terminal disposal. [Core11](../dialects/core-owned-work-v0.11.md) additionally keeps
actual occurrence-bound P6 registration, exact source/request/value/key/attempt
correlation, immutable original storage, current authority and all-transitive
settlement as mandatory live premises.

Therefore the value permutation above is not a proof that these two concrete
activities may overlap. A future proof must establish which old/new occurrences
are admissibly overlapping under the full lifecycle and source policies, then
exclude interference for every such pair, or prove the relevant overlap cannot
occur. Current static metadata and nominal warehouse inequality supply neither
that complete admissibility theorem nor an actual physical canonicalization rule.
Accountable ownership/strict invalidation alone must not be called quiescence or
region exclusion. Current whole invalidation coverage remains 312/0 missing;
external concurrency still correctly refuses. No accepted lifecycle is weakened.

## Options and recommended sequencing

| Direction | Assessment |
| --- | --- |
| Treat different IDs/strings/one-revision invariant as proof | Reject: actual type counterexamples and unknown physical aliases |
| Implement a supported finite canonical authority contract plus a complete original-request/cohort/lifetime theorem | Potential future proof route; requires new exact contract/support, typed provenance and all admissibly overlapping revisions. Not provided by current opaque authority metadata. |
| Add runtime conflict admission/fencing/claim retention | Different contract and real host enforcement problem; not selected by ADR79/80/81 and not implemented by an assumed table or lease flag |
| Preserve refusal, finish independent Phase 1 authoring work while keeping explicit proof prerequisites | Recommend next: genuine near-English factoring critique/evaluation, with semantic gates visibly open |

Do not manufacture another descriptor or change providers to make the benchmark
pass. Before any future concurrency extension, freeze a separate supported
canonicalization/provenance/admissibility proof design and pin its actual normative
meaning. Prove the finite supported case or retain refusal. This audit does not
claim that no sound future proof exists; it identifies what the current premises
cannot establish.

The independent near-English deliverable is already required in Phase 1. The
576759-byte/84051-token RP-03 source and repeated service/protection/receipt clauses
are a concrete accessibility bottleneck. Prioritize its critique and controlled
English factoring options, identity/protection preservation and source/raw / full
RP-01/03/08/user/visual evidence plan. A larger token quota or new metadata-only
core cannot satisfy that requirement. Do not defer it to Phase 2 or claim current
source is accessible because it parses. This sequencing does not close the open
executable-IR/validator/benchmark/concurrency gates.

No production meaning, source/version/schema, accepted artifact, current/historical
benchmark/resource/guard/generator/report bytes change in this audit. Forty/all
four Partial, G1–G4, full/optional/RP-08/complete validator/protected accessible
text-visual/actual users/hosts/browsers/independent/phase exits remain open.
No actual external blocker is reported while independent work remains. No Phase 2.

## Verification checkpoint

The five SDK groups pass. Required full pinned workspace/compile-fail/native/Wasm /
release/native-portability/Python checks and separate completed-diff automated
review are pending. Fresh full unresolved review/thread/CodeRabbit audit and
final-head CI precede signed expected-head merge; no skipped bot or personal
owner/independent/human approval is claimed.
