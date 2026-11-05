<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0020: Validate lifecycle policies at their definition use sites

**Status:** Accepted under Project Owner delegation; effective upon PR merge<br>
**Date:** 2026-11-05<br>
**Decider:** Codex under Project Owner delegation, ADR-0018

## Context

Static payload checking validates structural policy targets relative to the
policy's declaration scope. Reusing that policy in a child scope can incorrectly
make a sibling or the child itself look like its handler/follow-up. Accepted
semantics require direct children of the scope actually handling the fault or
closing. Empty-handler and terminal policies remain legitimately reusable.

ADR-0013 also requires an effect reconciliation wait policy to have a declared
wait node owning its outcome names. A well-typed unattached wait record cannot
supply that site. The current wire policy reference identifies a wait policy,
not an explicit wait-node owner, so multiple possible sites need a conservative
resolution boundary. Definition identity alone never identifies a live owner or
subscription occurrence.

## Decision criteria

1. Enforce accepted lexical lifecycle boundaries at actual scope uses.
2. Resolve non-node wait references without guessing a definition owner.
3. Preserve valid policy reuse and distinguish unsupported inference from a
   language-wide prohibition.
4. Retain explicit runtime ownership/authority/correlation obligations.

## Decision

Extend static definition checking with use-site lifecycle links. Every scope's
fault-handler target and reconcile-closure follow-up must be its own direct
lexical child. Validate all declared scopes, including ordinary-uncalled ones.
Inherited policies with no structural target remain valid; do not require all
policies to be declared in the scope using them.

Index declared wait nodes by their referenced wait policy after node/outcome
checking. For an effect reconciliation reference, require exactly one candidate
wait definition site. Zero candidates refuses `reconciliation-owner-missing`;
multiple candidates refuses `reconciliation-owner-unproved`. This is a
conservative supported inference rule, not a semantic ban on every possible
reuse pattern. Ordinary wait-policy reuse without a non-node effect reference
remains valid in IR; the authoring profile retains its existing explicit-owner
binding restrictions. No first-match or traversal-order selection is allowed.

Keep the existing wire, grammar, exact resource pins and report boundaries. Bind
errors to actual scope-use or effect-reference paths. The index and use-site
traversal consume the existing cumulative static budget. No runtime callback,
host trust flag or mutable admission credential is introduced.

## Options considered

| Option | Advantages | Costs and risks | Outcome |
| --- | --- | --- | --- |
| Check actual use sites and infer only a unique declared wait site | Enforces current closed rules without a wire migration | Ambiguous valid designs can refuse as unproved | Adopt |
| Check only policy declaration context / wait payload shape | Smallest implementation | Can name an impossible child or omit the required wait site | Reject |
| Pick the first matching wait node | Accepts ambiguous inputs | Depends on ordering and invents ownership meaning | Reject |
| Add an explicit owner field in a versioned dialect | Supports richer reuse without inference | Requires separately specified resource/version/source/export migration | Future option when required; do not smuggle it into annotations |

## Consequences

Malformed lifecycle links refuse earlier, and the same check applies to syntax-
lowered and direct IR candidates. Programs relying on an ambiguous non-node
reference must use distinct wait policies/sites or await a supported explicit-
owner contract. Existing unique-site examples remain stable.

A declared definition site is not proof of a live scope, subscription/occurrence
identity, receiver acceptance, capability/principal authority, exact correlation,
atomic settlement, cancellation safety or reconciliation completion. Those remain
host/full-validator obligations. Race/invalidation, clocks, complete benchmarks,
stable source diagnostics, text/visual parity, actual user evaluation and Phase 1
exit remain open. No independent human review or runtime observation is implied.

## Confirmation

Follow the [preimplementation plan](../evaluation/0020-lifecycle-link-plan.md),
committed in `8135d26`. [Evidence](../evaluation/0020-lifecycle-policy-links.md)
includes valid inherited policies, wrong actual contexts, uncalled scopes,
missing/ambiguous effect wait sites, ordinary wait reuse, source cycles, bounded
indexing and native/Wasm/oracle checks.

## Acceptance and action items

Accepted under the owner's 2026-11-05 delegation after a separate automated
self-review of scope contexts, ambiguity, policy reuse, work bounds, diagnostics
and ownership claims. This is not independent human review or owner review of
this individual ADR. Final-head required CI remains a merge prerequisite.

1. [x] Complete implementation, tests and separate automated self-review.
2. [x] Record delegated acceptance; verify final-head required CI before merge.
3. [ ] Continue the remaining lifecycle/race/invalidation, clock and phase gates.
