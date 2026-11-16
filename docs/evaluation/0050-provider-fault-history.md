<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Provider fault-history metadata and claim evidence

**Date:** 2026-11-07<br>
**Plan:** [frozen metadata scope](0050-provider-fault-history-plan.md), `b9449d0`<br>
**Decision:** [ADR-0050](../decisions/0050-provider-fault-history-metadata.md)

The exact profile revision is
`sha256:2f580f46eb6ef9b7fff42906f6b0b5cafc6c5fa71ae14365828935bfe2da070c`;
the exact fixed nominal type artifact revision is
`sha256:a2e177ce1dd4bad5b9cd68b74e6da7749ec2b04b68a39dae1bd72729e2aae7a7`.
Both are separately supplied immutable bytes. A closed provider-fault-history
resource pins the profile and an exact existing provider-fault-source. The SDK
verifies descriptor/profile/source/source-profile/provider and all transitive
nominal bytes, retaining immutable metadata without actual-use/entry authority.

The fixed nominal representation has P4's four history fields, exact occurrence
claim components, an opaque original cause reference and input cell revision
identities. Its invariant is true intentionally: ordinary nominal representation
admission cannot establish that any fault occurred. A separate limited claim check
verifies exact fault/observation source membership, identity/reference bounds,
definition revision syntax, attempts and input-cell uniqueness. It returns unit,
not an event, initialized cell, grant or admission report. Invented but well-formed
sites pass this deliberately limited judgment; actual site/event/source/evidence /
input and handler correlation remains unproved. No host observation is generated.

The source provider's declared resultProtection is exposed as a lower bound only.
Actual entry must retain/combine source-input, fault-existence/control and creator /
owner provenance, current access and management authority. Zero inherited values
never erases labels. Unknown evidence remains unresolved; no success/no-effect
classification follows from a convenient fault spelling. Actual event records,
request/key/effect correlation, live activation, exact nearest-handler selection,
fresh read-only entry, conditional initialization/access/flow and strict invalidation
are separate gates. Cancellation, parent resumption, continuation, ownership and
settlement rules are unchanged.

Descriptors cache by exact pin. Aliases share source metadata and fixed nominal type,
not actual occurrences or receipts. Existing cumulative budgets charge cached lookups,
repeated claims and each input row. A final construction failure cannot publish a
partial descriptor. Already verified shared type metadata can remain cached, without
conveying history evidence. No authoring resource kind, native output field, scope
wire, core profile, retry eligibility or quota is added or silently changed.

Seven Rust integration groups check the real full-package source/provider metadata,
fixed nominal/protection lower bound, every missing/corrupt transitive resource,
closed kind/version/profile/source identities and nonleaking diagnostics, alias
sharing, all twenty finite claim cases, ordinary fabricated-value limitations and
closed representation boundaries. Two internal groups check final construction /
cached lookup exhaustion, no failed descriptor publication, repeated claim charges
and proportional per-input work using the original allowance.

Four Python groups independently check exact profile/type snapshot hashes and the
complete representation; all twenty claim vectors against a limited representation /
mapping model; unchanged six native current-package stages with extra unused exact
metadata; and CI/licensing/protection boundaries. Rust consumes the same vectors
through actual SDK judgments. The model assumes prevalidated source metadata and
proves no source truth or process behavior. Existing handler fact/ordinary-port /
closed-wire refusals remain in the required fault-entry boundary suite.

Local verification passes **400 Rust tests plus six compile-fail doctests**, format,
workspace native lint, all five library Wasm lints and release portability/frontend
builds. The native oracle passes 97 structural cases, 17 JCS/hash comparisons, three
original fixtures/hashes, ten node variants and nine CLI operations. Every required
Python workflow command and exact benchmark regeneration/historical probe passes.
Wasm compilation is not actual browser execution. Transient logs:
`/private/tmp/fault-history-{workspace,clippy,wasm,native,oracles,focused,budget,python}.log`.
Required final-head CI/comment audit/signed expected-head merge remain outstanding.

Separate completed-diff automated self-review checked exact profile/type/source
identity and transitive resource closure; literal/claim/actual-event distinctions;
fixed record and claim boundary behavior; provider lower bound versus complete
source/control provenance; immutable alias sharing and cumulative/final construction
budgets; old contract/wire/grammar/digest/report compatibility; licensing, CI and
finite evidence scope. No actionable issue remains. This is delegated agent
self-review, not independent/human or individual owner approval. Inspect every
thread and CodeRabbit summary; a skipped bot review supplies no review evidence.

Next freeze reviewed handler-local entry on a new explicit wire, complete incoming
emitting-site/source/control contexts and conditional fresh initialization/access /
strict invalidation. Fail closed on unsupported human/unbound/engine origins. Then
readable authoring and permitted owned continuation/cutoff/all-settlement. Current
RP-01 source 0.7/core 0.4/IR 0.1 stays unchanged; parent #46 remains draft/incomplete,
all forty scenarios Partial, and all actual host/user/browser/independent, complete
validator/benchmark/general authoring/visual/protected editing and phase gates open.
