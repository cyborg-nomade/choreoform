<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# External interference: design critique before selection

**Date:** 2026-11-12<br>
**Scope:** signed [plan 79](0079-external-interference-design-plan.md), after #93 merge<br>
**Evidence:** [actual audit](0078-parallel-effects-contract-audit.md); no new runtime evidence

The missing premise is exhaustive provider behavior, not a larger Boolean guard.
Even different warehouse names may address the same physical stock pool, account,
quota or order-wide invariant. Compensation and stable retry keys do not establish
commutativity. Current P3 is closed; accepting an extra field in its existing
version would silently change a pinned contract. Human work remains opaque.

## Packaging options

| Option | Benefits | Costs and failure modes |
| --- | --- | --- |
| Provider contract 0.2 with mandatory interference description | One authoritative document; fewer bindings | Repins unchanged type/result/observation/protection claims and every dependent terminal source; cannot migrate selectively; still needs trust and canonical alias support |
| Separately pinned descriptor bound to exact provider and core profile | Optional migration; old P3 bytes stay immutable; independently inspectable behavior claim | Must reject substitution, conflicting descriptors and unsupported profiles; descriptor integrity is not provider consent; selected descriptor must enter definition revision/dependency closure |
| Infer a region from provider/capability ID or request text | Cheap | Unsound: different IDs/text can alias; omits common infrastructure/domain effects |

The supplementary option is preferable if its own closed version/profile and exact
provider binding are mandatory. It must not be an annotation, an implicit default,
or an author-controlled `independent:true` flag. Production adoption needs a new
core contract and explicit resource binding; this scope only publishes a draft.

## Proof and enforcement options

| Option | Assessment |
| --- | --- |
| Finite read/write regions with typed immutable-request selectors | Checkable conditional model: writes conflict with reads/writes, reads commute with reads. Complete coverage must include indirect/shared effects and observation paths. Canonical mapping remains a separate trusted premise. |
| Runtime conflict admission/locking | Can handle unknown static keys, but requires actual shared atomic admission, external-actor coverage, fencing, uncertainty retention and failure/handoff semantics. A declaration or pure lock simulation cannot supply those guarantees. Defer execution admission to an explicit host implementation scope. |
| Arbitrary symbolic predicates or custom normalizer code | Unbounded proof/trust surface, portability and alias ambiguity. Reject for this first finite contract. |

Prefer static comparison of resolved finite regions as the first contract, with
unresolved input values/aliases staying unproved. The model may compare concrete
synthetic snapshots to test rules; it cannot justify accepting a generic definition
using sample data. Future static checking must prove the regions disjoint for all
admissible overlapping occurrences or report a conditional host obligation without
claiming it was enforced. Do not silently serialize the representative process.

## Required safeguards before selection

- Separate namespace identity from physical identity. Every namespace used in a
  comparison needs a supported, common, exhaustive canonicalization authority.
  Unknown mappings are not fresh domains; different registries do not prove
  separation. Exact canonical domains/typed keys can represent aliases explicitly.
- Keep selectors deliberately small: record field, present option and explicit
  named representation; text/integer leaves only. No expressions, cell lookup,
  string concatenation, defaults or normalization performed by user code. Missing,
  absent, malformed and unsupported values remain unproved, never empty footprints.
- Select from the original accepted request. Actual occurrence/logical effect,
  input revision, descriptor/provider pins and request digest must stay correlated.
  A retry cannot switch those inputs; a changed request is new logical work.
  Cross-revision comparisons need both original snapshots, not reused inequalities.
- A terminal observation is an observation of its original logical effect, not a
  fresh business action. Observation reads must be declared separately and mapped
  from that retained request. Compensation has its own footprint and new identity.
- A definitive effect result does not prove all physical attempts quiescent. Lost
  replies, cutoffs, cancellation requests and expired leases cannot release an
  unresolved claim. Operational exclusion and durable business obligations differ;
  releasing the former never erases reservation/payment/compensation obligations.
- Preserve protection/dependency labels. Public reports must contain no selected
  values, keys, domain names or request digests; a proof result is not a data-release
  grant. Budget input, traversal, comparisons and output; publish no partial result.

The next ADR must settle exact shapes/bounds and these trust/correlation rules,
then exercise them with a pure, explicitly assumed model and closed schema.
Unimplemented host enforcement, accepted nominal invariants, real provider truth,
actual browser/user/independent evidence and the current benchmark refusal remain
visible. This critique supplies neither execution admission nor a Phase 1 exit.
