<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-03 parallel external effects: actual contract boundary

**Date:** 2026-11-11<br>
**Frozen plan:** `84dd75d`, [scope](0078-parallel-effects-contract-audit-plan.md)<br>
**Dependency:** #92 passed CI `43048189364`, review `5861133254`, then signed merge
`a9b5d11acdc2e7ff7a6b7e9209895ae27ba34262` before probes<br>
**Decision:** [ADR-0078](../decisions/0078-external-interference-contract-direction.md)

## Actual first comparison

A private cfg(test) observer at the existing predicate records bounded declaration
IDs/flags/footprints, not physical operations. Both the real whole raw SDK case and
an isolated guarded-true policy variant pass strict invalidation then refuse
`parallel-effects-unproved` at `/body/nodes/start_acceptance`. The actual tuple is:

| Field | Current child | Accumulated previous children |
| --- | --- | --- |
| Lexical branch | `accept_b` | exactly `accept_a` (one prior branch) |
| Shared reads | `original_order` | `original_order` |
| Shared writes | `reserve_b_record` | `reserve_a_record` |
| Effects flag | true | true |
| Observations flag | true | true |

Local write/write and read/write independence passes first. The external-work
predicate then rejects the flags. This is a possible interference boundary, not
an observed physical collision. The failed pair is the two reservation branches,
not an assumption based on the payment/capture caption. Each invokes its own typed
reservation service using a distinct capability/provider binding with compensatable
class and stable technical keys. Its local recovery declaration also contains a
terminal-evidence wait. All-declaration summaries retain unvisited possible work;
those declarations are not accepted provider events or execution traces.

The root has `order_commits` in guarded mode, checking distinct line IDs and matching
order/line totals. Each reservation service also uses that policy; their caller
branches retain the ordinary acceptance-order policy. None supplies an external
independence proof. The raw guarded-true variant keeps the same actual first tuple;
it is a negative control, not a proposed benchmark correction.

## Finite summary inventory and evidence levels

| Scope family | Branches | Effects / observations | Shared writes |
| --- | --- | --- | --- |
| Acceptance | A, B, payment | true / true for all three | own reservation or authorization record |
| Preparation | A, B | true / true | own packing/label/handover records |
| Delivery | A, B | false / true | own delivery record |

Preparation branches have seven shared reads each; delivery has thirteen each,
including retained incoming control/data dependencies. The exact inventory is
exercised by the real `Summary::build` over the full candidate after strict
invalidation. The isolated delivery-summary predicate passes its existing
read-only-observation boundary. That is not a claim that the full process or later
lifetime/domain/concurrency stages pass: the actual whole case stops at acceptance.

Four Python/native groups use complete source0.16/companion0.15/C12 fixtures and
fresh coherent fictional provider bytes/pins. Source, provider class/idempotency,
protected local result, work completion, local wait endpoint, typed human result
and terminal scope declarations are validated, not forged summary flags.

| Bounded complete source bodies | Actual ordinary concurrency result |
| --- | --- |
| Two read-only capabilities | Passes this conditional boundary |
| Two observation-only waits | Passes this boundary |
| One compensatable or irreversible capability, silent peer | Passes this boundary; no blanket non-read-only ban |
| Compensatable + read-only; read-only + irreversible | `parallel-effects-unproved` |
| Irreversible + compensatable; two compensatable capabilities | Same refusal |
| Human + wait; human + read-only capability | Same opaque-work refusal |
| Every negative above with guarded-true policy | Same refusal after five earlier checking/advisory views pass |

The supported controls retain `executable:false` and the unresolved domain/host
requirements. Negative composed reconciliation checks preserve the earlier
concurrency refusal. Format/reparse/integrity and exact resource pins are checked.
These fixtures are separate fictional declarations, never relabeled benchmark
providers or actual human/provider work. Two SDK groups exercise the actual whole
comparison/guard control and finite real summary inventory. A helper predicate
control is explicitly not a complete process/host witness.

## Why current declarations cannot supply the missing premise

[ADR-0022](../decisions/0022-ordinary-concurrency.md) deliberately distinguishes
fresh/local data storage from external effects. Human work is opaque; capabilities
use their exact validated class. Non-read-only/opaque work against another external
observation/work refuses. Guarded Boolean syntax, compensation, idempotency and
separate result cells are expressly not external commutativity/domain proofs.

The current [P3 provider contract](../dialects/policies.md#p3-capability-and-effects)
is closed: kind/version/id, input/output, outcomes, resultProtection, observations,
idempotency and effectClass. All 24 current provider-contract documents (23 operation
providers and clock source) have exactly those fields. No external interference
footprint or canonical physical-resource namespace is declared. P7 guarded predicates
are typed expressions, not theorem certificates or exhaustive provider guarantees.
Different provider/capability IDs and two different warehouse input strings do not
establish that implementations cannot access a common backend/domain invariant.

A no-op source change, true guard, altered effect class or frontend-only bypass
cannot fill that gap. Serializing the original graph would change its parallel
requirement; it is not a correction to the existing independence proof. Current
code/diagnostic behavior is therefore preserved in this audit.

## Requirements for the next contract/proof scope

Design a **versioned, explicit external-interference contract** before extending
acceptance. Prefer a separate portable resource bound to an exact provider revision,
subject to the next design's comparison against a provider-contract version update.
Do not add meanings to existing closed fields or infer an implicit provider-ID region.
The next signed design/prototype scope must settle at least:

1. Exhaustive external reads/observations and writes/effects, canonical namespace /
   authority and alias rules, including unknown/opaque cases. Partial claims cannot
   certify independence. Captures/reconciliation channels retain their source and
   original-effect relationship; human work stays opaque unless separately proved.
2. Bounded typed selection from immutable accepted requests and exact provider/source
   revisions, never arbitrary host code or current-cell reevaluation. Missing/unknown
   keys, optional values, unsupported selectors and unresolved namespace bindings
   must refuse or remain explicitly unproved.
3. The exact independence/conflict rule and its scope. Disjoint declarations need
   declared, enforceable meaning, not a safe Boolean. Local storage, protection,
   domain/irreversible invariants and surviving lifetime remain independent gates.
4. Actual input revisions/cohorts and effect identity. A nominal inequality within
   one input revision does not prove non-overlap across revised concurrent requests.
   New business input is new logical work; retry/idempotency and compensation are
   different relationships. No static declaration selects actual occurrences/values.
5. Provider/normalizer/host support and truthful exhaustive claims, registration and
   canonical mapping, current authority/protection, cancellation/cutoff, uncertain
   effects and all-transitive ownership/atomic handoff/settlement. A pin is integrity,
   not actual provider consent or permission; actual enforcement evidence stays open.
6. Backward-compatible version/pin/profile handling, bounded work/output, native/Wasm /
   source/raw counterexamples, privacy-preserving conditional evidence and an honest
   boundary between static proof, declared host obligations and executable admission.

Do not choose field names/selectors or implement the new reader/meaning in this
audit. The next contract design must be concrete before its implementation plan,
with durable decisions under delegation. No generated claim replaces actual host,
provider, browser, representative-user or independent evidence.

All current/historical benchmark/generator/resource/guard/report bytes remain fixed.
Full source remains 576759 bytes/84051 tokens, revision
`sha256:a3b8c0f07cfc63d745376c3eb11b67690c98edc3e216d262c1542f00a05798aa`.
Five actual reports remain non-executable; six later views retain the exact
acceptance refusal. Forty/all four Partial, G1–G4/parent #46/full/optional/RP-08 /
validator/near-English factoring/managed protected accessible text-visual/actual
users/hosts/browsers/independent/phase exits remain open. No Phase 2.


Local verification passes 602 Rust tests plus six compile-fail doctests, format,
native/all-five-library Wasm lint, release portability/frontend Wasm, fresh native
portability and all 74 Python commands. Logs:
`/private/tmp/parallel-effects-workspace.log` and `/private/tmp/parallel-effects-verify.log`.
Final-head CI, separate automated completed-diff review and full comment audit are
required before the signed parent merge; no skipped bot or personal owner review
is counted as independent evidence.
