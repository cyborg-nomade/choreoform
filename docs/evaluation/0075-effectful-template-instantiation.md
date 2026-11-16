<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Effectful caller-local instantiation: bounded critique

**Date:** 2026-11-11<br>
**Frozen plan:** `9149340`, [scope](0075-effectful-template-instantiation-plan.md)<br>
**Dependency:** #89 merged at `28b925b9b1919f029b9231b35aacdead7c399b96` before probes<br>
**Decision:** [ADR-0075](../decisions/0075-effectful-template-instantiation.md)

## Supported bounded relationship

The complete synthetic Main calls A caller and B caller sequentially with typed
record ports. Each caller owns its own service definition and invokes that direct
child with explicit input/output bindings. Each service performs one capability
activity, compares its own result to its own request, and designates a local
reconciliation scope with an exact terminal observation capture and readonly
original-request target. It has its own capability/effect/wait identities and exact
wait endpoint; a fresh fictional provider contract explicitly names both logical
capability recipients. Terminal/fault sources pin that exact provider revision.

All eleven native checking/advisory views pass for all three effect classes
(read-only, compensatable, irreversible), both without and with B's explicit
compensation identity referencing A's activity. This is **66 bounded views**, not
66 full benchmark scenarios. The identity of a prior declaration neither changes
B's own class nor proves an eligible prior occurrence. Compensation/retry/new
logical work and occurrence storage remain separate contracts.

The persisted baseline also passes the real source SDK, format/reparse and
independent raw SDK checks of the lowered body in a fresh envelope. Three Rust
and six Python groups cover the positive relationship and isolated mutations.
No production checker/parser, contract, profile, schema, quota or benchmark /
generator byte changes are included.

## What must be instantiated together

| Relationship | Observed result |
| --- | --- |
| Distinct service parents, complete typed ports, local result/completion rules | Accepted by the bounded static and later partial judgments |
| Shared provider bytes with explicit A/B logical recipient claims | Accepted; not an actual provider approval |
| B borrows A's capability or A's reconciliation wait | Source lower refuses `wait-endpoint-use-unproved` |
| B's wait names A's exported endpoint | Source lower refuses `wait-endpoint-owner` |
| Reconciliation policy names an actor policy | Source lower refuses `wait-endpoint-use-unproved` before any later policy-kind judgment |
| B reads A's private request by exact qualified path | Source lower refuses `reference-qualification` |
| Source capture uses a fault source | Source lower refuses `resource-kind` |
| Capture names a finish node as origin | Static checking refuses `wait-capture-origin-unproved` |
| Original request targets the capture destination | Static checking refuses `retained-request-readonly` |
| Provider narrows recipient claims to A alone | Static checking refuses `information-flow` at A's activity; its result flows into storage also readable by B |
| New resource bytes under an old revision | Lower refuses `resource-digest` |
| Raw input port omitted / valid text expression in record port | `invoke-ports` / `invoke-type` |
| Raw private sibling-cell read | `cell-reference` |
| Raw service reparented alone | Earlier `visibility`; the fixture's retained invalidation references are no longer lexical. This does not replace #89's isolated `child-scope` proof |

Fresh recipient claims change the provider digest and the exact provider pins in
both source descriptors. The test generates mutually consistent fictional bytes;
it does not rewrite a published provider contract or obtain any real consent.
Published benchmark metadata is fixed in this slice.

Invalidation must follow the actual input/control provenance down through both
invocation levels. Seven activity obligations include the service request, caller
request and root request for each service, plus A's exported first result for B's
later activity. Invalidating only the invoking nodes was insufficient. Existing
qualified descendant invalidation syntax expresses the required targets without
opening ordinary private reads. Removing root coverage refuses
`invalidation-coverage` in both source and raw checks. Retained/captured storage
keeps the local exceptional followup invalidation declarations.

## Limits and recommendation

Every accepted report remains `executable:false`. Final reconciliation reports
check no reconciliation nodes and leave the two observe/finish pairs uncovered;
`conditionalOwnedLocalChecks` covers their declared local relationship only.
Capture, retained-request and fault-source rows retain live host requirements.
There is no actual service invocation, accepted effect, captured result, eligible
compensation, original request snapshot, browser/host run or settlement witness.
These ordinary service activities are not evidence for arbitrary work after an
exceptional capture, aggregate ownership or complete full-case validation.

Proceed to a separately signed full RP-03 source-generator adoption plan. Instantiate
bodies by actual caller context, preserve same-parent reuse where valid, and remap
local scope/data/expression/policy/node/flow/symbol identities coherently. Track
which existing provider contracts actually need changed logical recipients;
introduce new explicit synthetic resource revisions only under that adoption plan,
preserving their exact previous bytes/pins in history. Do not invent recipient
aliases or weaken receipt, cancellation, interception, refund, access or completion
predicates. Treat each new full-case refusal at its real stage; a bounded passing
pattern predicts neither complete support nor the next diagnostic.

Preserve scenario/receipt meanings, immutable input provenance and all transitive
obligations. Distinct definitions do not select actual occurrences, authorize new
effects or establish fresh storage/atomic handoff. Runtime current authority,
registration, receiver, cutoff, conflicting evidence and all-obligation settlement
requirements remain open. Near-English factoring and managed identities remain
explicit later deliverables; emitted duplicated source is not their completion.
Parent #46 remains draft, forty scenarios Partial, G1–G4 and every applicable full /
optional/RP-08/validator/user/host/browser/independent phase exit remain open.
