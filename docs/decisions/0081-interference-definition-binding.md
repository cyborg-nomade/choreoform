<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0081: Select provider interference explicitly in a new core wrapper

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-12<br>
**Decider:** Codex under ADR-0018; no personal owner or independent review claimed

## Context and alternatives

Signed [scope81](../evaluation/0081-interference-definition-binding-plan.md)
`aa4d58f` precedes work after #95 merge `78f5d706756642932c57c33672903a7b9ccdcacc`,
CI `43233888435` and automated COMMENT review/full audit `5875818566`.
The separately committed [critique](../evaluation/0081-interference-binding-critique.md)
compares a new wire manifest, capability wrapper, effect policy and unused registry
or annotation. Only explicit semantic selection can bind the new metadata.

## Decision

Publish/sign [core0.13.0](../dialects/core-interference-binding-v0.13.md) before
reader implementation. Keep IR0.4's existing dialect-body extension boundary;
require all twelve inherited core artifacts plus the exact standalone profile.
Only the capability-contract policy body gains mandatory `interference:null|Ref`.
Null supplies no premise; non-null selects an exact descriptor bound to the exact
wrapper provider. All wrappers of one exact provider, including unused wrappers,
must consistently select null or the same Ref. Mixed or conflicting selection
refuses. Different provider revisions remain distinct. No implicit selection or
P3 provider-resource mutation is permitted.

Validate the full supported typed resource closure, preserving each wrapper's
lexical provider/protection rules. The root Ref and content-addressed dependencies
enter semantic definition identity. Expose immutable declared per-wrapper metadata
tied to that checked revision, retaining explicit remaining proof/trust/authority /
correlation/lifetime/settlement obligations. Keep old source report bytes unchanged.
No live value evaluator, canonicalizer, independence Boolean or runtime admission
is added; even complete new-core effectful controls must retain the current
parallel-effects-unproved result. All prior core meanings and current/historical
benchmark/generator/report bytes remain exact.

## Consequences and confirmation

A new wire manifest would change the closed IR body and add avoidable structural
migration. The wrapper already names the provider and is a versioned dialect slot.
An effect-policy slot is less precise when policies/providers are shared; annotations
and unused resource bytes have no semantic selection identity. The nullable field
adds raw migration work but eliminates hidden default/first-choice behavior.

Cache selection once; use indexed exact-provider consistency and existing shared
metadata/budget mechanisms. Verify complete definition/resource graphs, all-null /
consistent/foreign/conflicting/mixed/unused selections, transitive substitution and
revision changes, old/new version refusals, inheritance, privacy and budget limits.
Run required final-head native/Wasm/Python checks, separate automated completed-diff
review/full review-comment audit and signed expected-head merge. Bot skips supply
no review evidence; no human/GitHub self-approval is claimed.

Next: a separately signed generic-separation critique/proof scope, including actual
original-request construction, canonical alias assumptions, revised cohorts and
earlier surviving work. Prove the finite supported case or preserve the refusal;
sample inequalities or manufactured external evidence cannot replace proof. Managed
near-English source/companion selection and source/raw equivalence need their own
versioned scope. All unproved Phase 1/actual external-evidence gates and draft #46
remain open; no Phase 2.
