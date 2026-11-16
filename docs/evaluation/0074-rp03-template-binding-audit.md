<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-03 template parent and caller-context critique

**Date:** 2026-11-11<br>
**Baseline:** #88 head `66884dc27e04de67634839bf3f9fd42ebb1b3632`, merged
as `94adbe95c9770090153d86e9489566b105ff2352` before probes<br>
**Frozen plan:** `9bca249`, [scope](0074-rp03-template-binding-audit-plan.md)<br>
**Decision:** [ADR-0074](../decisions/0074-rp03-template-instantiation-direction.md)

## Actual first failure and accepted rule

The private test-only recorder captures the real complete frontend failure:

| Field | Actual value |
| --- | --- |
| Creator node | `authorize_call` |
| Creator kind | `invoke` |
| Caller lexical scope | `accept_payment` (Authorize branch) |
| Requested body | `authorize_service` (Authorize order payment occurrence) |
| Declared body parent | `order` (Order fulfillment) |
| Native result | `child-template at 0..0`, empty stdout |

`Lower::check_shape` compares body.parent with node.scope for invocation/repeat /
fanout, and checks split children similarly. Raw `Check::child` enforces the same
relationship and emits `child-scope`. The [IR 0.1 contract](../ir/definition-v0.1.md)
defines invoke/repeat/fanout bodies as child scope templates, one connected lexical
parent tree, explicit local ports and definition-bounded occurrence creation.
Split/repeat bodies have no ports and fanout's initial profile has only its item
input; general composition needs an explicit version rather than implicit binding.
[ADR-0008](../decisions/0008-core-process-semantics.md) separates definition templates
from runtime occurrences and preserves unaccounted obligations.

Lexical visibility of a scope name or a global same-definition ID is not the child
creation judgment. Core 0.12 changes only compensation activity identity; it neither
licenses cross-parent invocation nor selects an actual occurrence/owner. This is
not a frontend-only omission. A parser-only bypass would yield a raw-invalid body.

## Bounded real SDK/source evidence

Five SDK groups use complete data-free synthetic declarations plus the unchanged
full current package. The bounded fixture has Main invoking A/B and B invoking its
own C, with exact complete outcomes/policies/resources. No benchmark was truncated
into an allegedly accepted order; no full-case IR was generated through another path.

| Witness | Actual result |
| --- | --- |
| Direct child calls with exact ports/outcomes | Static and later partial checking pass |
| Same A body reused by two creators in Main | Raw later partial checking passes; not actual occurrence/storage/ownership evidence |
| B invokes A, declared under Main | Readable name resolves, lower refuses `child-template` |
| Equivalent isolated raw body under C11 and C12 | Positive direct-child baseline, then `child-scope` for the same cross-parent target |
| Missing/wrong-map raw body ID | Earlier `structure` refusal |
| Missing/wrong-kind readable body | `reference-missing` / `reference-kind` |
| Correct fully qualified direct child C from B | Lower succeeds |
| Main attempts a qualified private grandchild C | Earlier `reference-missing`; generic descendant lookup remains closed |

The qualified direct-child witness was corrected from an initially mistaken expected
refusal to actual acceptance; a separate nonvisible-grandchild witness records the
real privacy refusal. No production behavior changed to satisfy the tests. Four
Python/native groups independently check finite declaration relationships, direct /
same-parent reuse through all eleven native views, distinct binding failures and
unchanged full-case refusal. Passing bounded views are `executable:false`.

The recorder is private `cfg(test)` state, bounded to one tuple/count per thread.
It records declaration IDs/parent identity only; no runtime/public debug surface,
source-literal diagnostic, grant, actual owner or new occurrence is exposed. Both
existing runtime checks keep their original codes/spans/lookup/protection behavior.

## Complete finite declaration inventory, limited interpretation

The Python audit captures actual generated action/declaration metadata and verifies
every generated current artifact still equals committed bytes. This finite fixture
uses unqualified direct `Run` bodies; the model explicitly checks that spelling,
resolves the declared lexical scope name and compares the declared parent. It is
not a general parser or full validator. Tuple-based lexer output is handled exactly,
so all three split declarations/seven child relationships are counted.

There are 26 direct invocations: 23 have different declared body parent and caller
scope. The three matching ones are `capture_call`, `notify_call` and `run_release`.
The mismatches cover reservation/authorization branches, preparation and delivery
branches, normal release/void, and surviving settlement commands. Every mismatched
service body is declared under `order`. `release_a_service` and `release_b_service`
each have two different caller contexts (`release_normal` and `order_settlement`),
so choosing one parent record alone cannot satisfy both call sites.

The three splits contain seven child declarations with matching parent scopes;
there are no repeat/fanout actions in this finite package. Matching that relation
does not establish their ports/join/control/protection/lifecycle judgments, nor any
actual activation or later semantic result. Full current source still refuses at
the first template check; no candidate or semantic-stage report exists.

## Recommendation and material consequences

Prefer explicit call-site service-body instantiation under existing child and port
contracts. Reuse within one declared parent remains available; context-specific
definition subtrees need stable managed identities, explicit input/output bindings
and the correct lexical environment. First freeze a bounded **effectful** feasibility
slice before changing the whole generator. The current data-free witnesses do not
prove effectful cloned bodies, policies or recipient envelopes will pass.

Check capability/effect/wait-policy identity together: current effect policies name
an exact reconciliation policy whose endpoint has an actual owner. Multiple body
instances can require distinct compatible policies/endpoints and protection labels;
do not silently reuse or duplicate IDs, change provider pins, invent a wildcard
recipient or declassify values. Each declared revision must match supplied bytes.
Preserve the separation of definition IDs, occurrence IDs and logical effect/key /
attempts; compensation remains new work bound to an actually eligible prior effect.

Changing a lexical parent can change captured ancestor data, local storage,
creator/owner provenance, closure/fault/cancel/race policy context and all-transitive
ownership. Keep explicit typed ports and immutable accepted input/capture storage;
do not allow a child or caller to finish with unaccounted obligations. Preserve all
original business guards, positive stop/interception/refund eligibility and actual
current authority/receiver/cutoff/atomic handoff/settlement premises. A hierarchy
rewrite is not automatically semantics-neutral or a new supported scenario.

If requirements cannot be expressed through explicit instantiation, a general
reusable-template/module/environment contract needs its own version and ADR. The
present evidence does not justify changing core 0.12 or weakening a parent check.
Near-English reusable formulations, managed identity/protected accessible editing /
export/text-visual and actual users remain explicit Phase 1 deliverables; raw scope
duplication alone is not a usability solution.

## Verification and remaining gates

Local verification passed 587 Rust tests plus six compile-fail doctests, pinned
workspace/five-library Wasm lint, release portability/frontend Wasm builds, native
portability and all 70 Python workflow checks. Separate completed-diff delegated
review checked correctness, security/privacy, bounded work, scope and evidence;
no remaining actionable audit-scope issue. Required exact final-head CI/full unresolved
thread/summary audit precedes signed expected-head merge. Logs:
`/private/tmp/rp03-template-audit-{targeted,python,workspace,verify}.log`. A skipped CodeRabbit review supplies no
evidence; agent review is automated, not independent/human/personal owner approval.
Compiled Wasm is not browser execution.

Every current/historical RP-01/RP-03 source/companion/resource/guard/scenario /
manifest/generator byte stays fixed. Parent #46 draft, forty Partial, G1–G4/full /
optional/RP-08/validator/near-English/managed protected accessible text-visual /
actual user/host/browser/independent/phase exits remain open. No live authority,
execution/settlement, external blocker or Phase 2 follows; independent work remains.
