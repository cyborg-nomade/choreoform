<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0041: Pin declared provider observation-to-fault evidence

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

The [RP-01 audit](../evaluation/0040-rp01-scenario-audit.md) reproduces that arbitrary
retry fault names pass current partial stages. P3 declares provider observations
and their states; P4 requires an exact accepted eligible fault and actual evidence
before retry. A label such as confirmed no effect supplies neither a provider
emission mapping nor truthful observation/correlation. Existing source/core retry
records have no source-binding field. The [frozen plan](../evaluation/0041-provider-fault-source-plan.md),
`ffb80a7`, separates a new pinned metadata reader from later versioned integration.

## Decision

Publish the independently pinned [provider-observation fault-source profile](../dialects/provider-fault-sources.md)
and bounded typed `Resources::provider_fault_source` reader. A closed resource
pins this profile and one exact provider, and maps fault names to nonempty sets
of its declared observation names with a declared evidence class. Resolve actual
provider observation states, never infer meaning from a fault/observation label.
Retain those typed states in the existing immutable provider reader.

Definitive-no-effect accepts only provider-declared rejected/cancelled/no-effect;
unresolved accepts only unknown. Success and acknowledgement, mismatched classes,
unknown observations, empty or ambiguous maps and supplied trust/safe fields refuse.
One observation maps to at most one fault in this profile. Unmapped observations
emit none through this source. Exact Unicode names have no implicit normalization.
Verify every exact resource/profile/provider/type pin from the explicit registry;
unknown profiles/versions/fields and arbitrary hashed bytes fail closed.

Return immutable private-constructor source/emission metadata and outstanding
definition/host requirements, never an actual observation, accepted fault, retry
permission, liveness/ownership receipt or dispatch capability. Cache only verified
immutable metadata and charge state indexing, mappings and copies within existing
resource budgets. No core/source/provider schema or old definition revision changes.
Registry bytes do not implicitly bind a retry or handler. Later integration needs
an explicit supported definition/core/source binding to the actual provider use.

## Options and consequences

| Option | Benefit | Cost or risk | Decision |
| --- | --- | --- | --- |
| Independent source resource pins existing provider observations | Explicit mapping, immutable provider compatibility, scoped future binding | One extra reference and a subsequent integration step | Adopt |
| Infer no-effect from fault names | Small change | Arbitrary names are not evidence and can misclassify unknown effects | Reject |
| Add fields to old provider/core bytes | Convenient implicit binding | Reinterprets frozen contracts and existing definitions | Reject |
| New provider resource version with embedded emissions | Single combined interface | Changes provider pins and still needs explicit use-site/core support | Viable later alternative, unnecessary for this slice |

The metadata reader may classify unknown for a provider lacking stable-key support;
this does not authorize retry. P4 still requires actual supported stable-key/
identical request identity or definitive correlated no-effect as applicable,
eligible accepted fault, budget/delay, unchanged inputs, fresh authority and the
success/cancellation/ordinary-close cutoff. P3 adapter truth, source/attempt/effect/
request correlation, fault protection/routing and P6 ownership/settlement remain
unproved. No actual provider observation or engine behavior is supplied.

[Evidence](../evaluation/0041-provider-fault-source.md) records exact profile hash,
native/vector/oracle tests and completed-diff automated review. Acceptance is
delegated, not individual owner or independent host review. Require final-head
CI, complete comment audit and signed expected-head merge into draft #46, then
implement explicit versioned use-site binding and remaining C/phase dependencies.
No scenario, parent deliverable or phase gate closes here.
