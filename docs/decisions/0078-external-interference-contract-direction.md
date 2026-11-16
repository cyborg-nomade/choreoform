<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0078: Design explicit external interference evidence before parallel acceptance

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-11<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

Signed [plan](../evaluation/0078-parallel-effects-contract-audit-plan.md) `84dd75d`
precedes probes, after #92's passing final-head CI/review/signed merge. The
[actual audit](../evaluation/0078-parallel-effects-contract-audit.md) captures
accept_b versus accumulated accept_a at start_acceptance: shared original-order
read, distinct result writes, and both effects/observations flags true. Data
independence passes before the external boundary refuses. A guarded-true variant
has the same result. Derived read-only delivery summaries pass that isolated
predicate; complete bounded fresh source controls distinguish read-only/silent
peers from mixed effectful/opaque work without relabeling the benchmark.

ADR-0022 intentionally rejects unsupported external overlap. Current closed P3
contracts declare class/keys/types/results/observations/protection, not exhaustive
external interference regions. P7 Boolean guards and distinct logical IDs cannot
prove implementations commute or preserve a domain invariant.

## Decision

Keep every current production meaning and benchmark byte unchanged in this audit.
Select a **new separately signed external-interference contract design/prototype**
as the next dependency, followed by its justified implementation scope. Prefer a
portable supplementary resource bound to an exact provider revision as the design
starting point; compare its trust/version/metadata consequences against a new
provider-contract version before fixing fields or readers. Do not silently extend
C12 or invent a provider-ID namespace/alias.

The design must specify exhaustive read/observation and write/effect boundaries,
canonical namespace/authority/alias semantics, bounded typed accepted-request
selection, unknown/opaque refusal, exact conflict/proof scope, input revision/cohort /
logical effect/attempt/capture correlation, conditional evidence and required real
provider/normalizer/host support/enforcement. The audit's explicit requirements
are acceptance criteria for that next design. Distinct input strings within one
revision cannot justify cross-revision independence without actual correlation.
No pin or descriptor is actual provider consent, execution authorization or a host
outcome. Current protection/storage/invariant/lifetime/ownership gates remain.

## Alternatives and consequences

| Option | Decision |
| --- | --- |
| Treat distinct IDs/result cells/compensation/idempotency/guarded true as safety | Rejected by existing semantics and actual controls |
| Relabel providers read-only or remove observations | Rejected: changes truthful operation meaning |
| Serialize the representative graph to silence the check | Rejected as a workaround for its original parallel requirement |
| Leave legitimate refusal while designing explicit versioned interference premises/proof | Selected; honest partial progress with a concrete next contract dependency |
| General arbitrary domain-invariant theorem solver | Not selected; first scope must be finite, explicit and checkable |

No new external-region/source syntax/resource reader/policy/profile/core/IR-version
implementation occurs here. Private test-only bounded observation supplies no
release API/diagnostic/state. Two SDK/four Python groups record genuine declaration
results and limits. Run all required final-head checks, separate automated review,
complete comment audit and signed expected-head CI merge before progress update.
No independent or personal owner review is claimed. Parent #46 draft, forty/all
four Partial, G1–G4/full/optional/RP-08/validator/near-English/managed protected
accessible text-visual/actual users/hosts/browsers/independent/phase exits remain
open. No Phase 2.
