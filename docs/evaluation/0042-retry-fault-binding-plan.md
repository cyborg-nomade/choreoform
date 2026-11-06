<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Explicit retry fault-source binding: frozen raw-core plan

**Date:** 2026-11-06<br>
**Baseline:** `5f442637c82cceba5c718f52f748368489076b16`, draft #46 after #54<br>
**Authority:** ADR-0018; ADR-0041 metadata is deliberately not a definition binding

Use a dedicated stacked dependency PR. Define ADR-0042 and an explicitly pinned
core-profile 0.4.0 inheriting exact 0.3/0.2/0.1 bytes. Add required nullable
`faultSource:Ref` only to this version's work retry record, retaining clock/basis
and all other fields. Do not alter old snapshots, provider resources, old core/
source behavior or wire structure. Current authoring profile 0.6 still selects
core 0.3; a later separately planned source/companion version must select core 0.4.

In core 0.4, maxAttempts 1 requires null clock/basis/faultSource and empty faults;
multiple attempts require nonempty eligible faults, exact source/profile/provider
bytes and non-null clock/basis/faultSource. Every eligible fault must resolve in
the source emissions. Definitive-no-effect is only a declared evidence class;
unresolved eligible faults require the pinned provider's declared stable-key
support in this conservative profile. Actual support/evidence/correlation still
remain P3/P4 host gates, not parser permission.

At every declared multi-attempt activity use, including ordinary-unvisited sites,
require capability mode and exact source provider identity/revision equal to the
actual capability provider. Type equality or matching labels alone is insufficient.
Human/mismatched provider contexts refuse as unproved. Unused policies may have
validated source declarations but no actual-use/activation claim. Preserve work/
target authority intersection, result/protection and all preceding strict checks.

Include source-provider protection references as metadata edges in exact policy
read closure. Core 0.3/0.4 use the same finite metadata semantics; mixed legacy
cyclic components/strong cycles remain refused. Require exact inherited artifacts
and preserve old-core diagnostics/unknown-contract failure. No supplied source
read sets, safe flags or source registry discovery substitute for derivation.

Expose immutable typed revision-bound source/provider/profile/eligible-fault/
observation/class declaration evidence through the library timing RetryUse, with
explicit actual acceptance/current authority/stable-key-or-no-effect/request/
attempt/cancellation/closure/ownership requirements. Old-core RetryUse has no
source binding, never implied proof. Do not accept a fault/observation or implement
dispatch/retry/runtime continuation. Native source/report integration follows with
the future authoring profile; current package source/IR/reports stay unchanged.

Tests: raw core-0.4 full-package positive and fresh revision; all exact inherited/
source/profile/provider pins; missing/unused fields and eligible names; wrong
provider despite same types/labels, human and unvisited contexts; unknown without
stable key, no-effect without stable key as declared mapping only; unused policies;
source metadata read closure and old/mixed dialect cases; immutable report/budgets.
Keep unknown version sentinels genuinely unknown rather than dropping negatives.
Add independent artifact/schema/accountability oracle and required CI checks.

Run native/Wasm/all Python and final-head CI, separate completed-diff automated
review, inspect all comments/CodeRabbit summaries and sign off authored/merge
commits. Merge only the passing exact head into draft #46. Then explicitly version
authoring/source/report integration before adopting the resource in RP-01. Continue
late-observation/data/continuation and full validator work; no scenario/phase exit.
