<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-01 whole-package implementation: frozen plan

**Date:** 2026-11-06<br>
**Baseline:** `482adf7c2871e2766e0ac645628a1a2c38a872ce` (PR #45)<br>
**Authority:** ADR-0018 delegation; ADR-0006/0014 and
[current readiness requirements](0034-benchmark-readiness.md).

Create a dedicated `docs/benchmarks/rp01` package, retaining the complete adopted
expense case and all A–D scenarios rather than replacing it with manager review.
Use source 0.4.0 / companion 0.3.0, exact core 0.2.0/base/semantics pins and only
synthetic data/resources. Preserve existing fixtures, contracts and historical
scores. Begin with complete exact types/resources and build the whole graph;
split coherent dependencies into further dedicated PRs if an actual contract or
quota limitation is demonstrated. Do not stop at inventories while claiming a
full package, silently weaken a scenario or invent missing semantics.

Supply request identity/revision/policy, submitter, exact amount/currency, date,
category/purpose/allocation and receipt-or-missing-reason references. Separate
check findings, duplicate resolution, manager/finance decisions/reasons, corrected
request, disposition, payable identity/status and notification evidence. Define
actual nominal invariants for synthetic domain constraints; record what types do
not authenticate. Human result types retain required outer variants; do not assume
a named type is implicitly unwrapped or construct circular resource hashes.

Provide exact policy/duplicate checking, payable processing and notification
provider resources, roles/authority/protection policies, and explicit retry clock/
source/basis resources where multiple attempts are used. Every provider declares
types, result protection, observations/effect/idempotency and host requirements.
Definitive business findings may be successful processing results; an unknown
external effect or transport fault is never relabeled a successful rejection.
No provider returning a safe/approved flag substitutes for process approval,
current authority, request correlation, owned settlement or physical truth.

The chosen first variant may run automated checks sequentially (parallel checks
are optional in the brief) and omit optional business-calendar reminders, clearly
recorded. Include both manager and finance review, correctable and rejected paths,
suspected-duplicate human resolution, explicit repeated correction, new approvals
for changed revisions, payable request and confirmation, ambiguity/retry/follow-up,
notification and cancellation/withdrawal boundaries. No ordinary back edges,
implicit child outputs, timer-generated approvals or handler-finish-to-activity-
outcome mapping. Use exact explicit local invoke transfers/lexical writes where
supported, and keep stronger concurrency/retention/continuation needs visible.

Derive and check the portable companion/generated slots and canonical IR from
the complete source. A package manifest pins every resource/profile/digest and
maps all four scenarios/alternatives to real declaration IDs and repeatable
positive/negative cases. Record actual outcomes for every existing semantic stage,
expected refusal codes/paths and residual host/proof obligations; no assertion
that a lowered candidate is admitted or running. Verify exact source/package/IR
integrity, format/reparse identity, stable IDs, resource/role/revision negatives
and protections. Actual participant/host/browser evidence remains external.

Measure full package bytes/tokens, parser work and semantic/report quota outcomes.
Do not raise default limits or introduce a new grammar/core/wire feature without
a separately frozen justified change/ADR where needed. If the current contract
cannot express a required path, document the concrete graph/requirement and resolve
that dependency explicitly before claiming this whole-package deliverable complete.
Keep all forty scenarios Partial until the full evidence required by the evaluation
framework is present. Run relevant checks and final-head CI, separately self-review,
inspect all PR comments, sign off commits and merge expected heads. Then continue
RP-03/08 and remaining Phase 1 proof/tooling/visual/protected editing/user gates.
