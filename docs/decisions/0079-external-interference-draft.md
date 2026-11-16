<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0079: Bound external interference with an inactive provider supplement

**Status:** Accepted under Project Owner delegation; draft design only<br>
**Date:** 2026-11-12<br>
**Decider:** Codex under ADR-0018; no personal owner/independent review claimed

## Context and alternatives

The signed [plan](../evaluation/0079-external-interference-design-plan.md) precedes
this work. #93 merged as `30ed6f75ec91954d2a807625a486603318f94bdf` after passing
CI and automated review. Its actual first RP-03 pair lacks an exhaustive external
independence premise. The separately committed
[critique](../evaluation/0079-external-interference-design-critique.md) compares
a provider0.2 repin, an exact-provider supplement and unsound inferred-ID regions;
it also compares static finite reasoning with runtime conflict enforcement.

## Decision

Select the [inactive draft profile](../dialects/drafts/external-interference.md):
an exact-provider/exact-profile supplementary resource with exhaustive or opaque
activity and terminal-channel footprints; exact namespace/authority bindings;
bounded text/integer selectors through record fields, present Options and explicit
nominal representations of immutable accepted original requests. Missing/unknown
values and unsupported aliases remain unproved. A common supported canonicalization
authority is mandatory; differing namespace/provider IDs or strings prove nothing.
Use ordinary write/read and write/write exclusion after canonical resolution.

Select static independence as the eventual proof route. Concrete model comparisons
are conditional examples, not a generic definition proof. Do not admit work using
runtime locks or a descriptor alone. Runtime enforcement would need a separate
atomic admission/fencing/crash/uncertainty/handoff contract and actual host evidence.
Keep request/input revision/logical-effect/attempt/terminal-capture/compensation
identities distinct. An operational exclusion is not a durable business obligation;
quiescence does not undo effects, and lease/cancellation expiry cannot prove it.

Implement only the closed draft schemas/examples and pure assumed model, with
bounded work/input/output and private fixed reports. Nominal invariants, provider
truth, canonical authority support and real host behavior are explicitly assumed
in the model; no production admission API consumes it. Retain current refusal and
all existing resources, profiles, benchmarks, generators and reports byte for byte.

## Consequences and confirmation

The supplement permits selective future migration without repinning unchanged P3
claims. It adds explicit resource/namespace/authority trust and dependency metadata;
the future core must validate unique selection, exact pins/types and unsupported
profiles. A provider0.2 migration remains possible but offers no stronger trust
evidence and needlessly duplicates unchanged claims here. Arbitrary theorem/custom
normalizer code is outside the finite initial scope.

Next: a separately signed production resource/metadata contract scope, before any
concurrency extension or benchmark adoption. New normative artifacts/core versions,
raw/native/Wasm/resource/privacy/counterexample evidence and near-English managed
bindings are explicit dependencies. Generic symbolic disjointness must be proved,
including revised/surviving requests, or remain unproved. This choice may reveal
that legitimate RP-03 providers cannot honestly promise separate regions; do not
change the representative parallel requirement to hide that result.

Exercise exact-byte substitution, selectors/types/options/nominal boundaries,
alias/unknown/overlap/read-only/opaque cases, original versus changed inputs and
retry/terminal correlation; test fail-closed shared budgets and private reports.
Run all required existing checks plus the new oracle, review the completed diff
separately, audit every review summary/thread and merge only with final-head CI.
No skipped bot review is evidence. Parent #46, forty/all four Partial, G1–G4,
full/optional/RP-08/validator/near-English/protected accessible text-visual and real
user/host/browser/independent evidence remain open. No Phase 2.
