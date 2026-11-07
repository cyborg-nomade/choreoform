<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0048: Bind declared provider emissions independently of retry eligibility

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-07<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

The [fault-entry audit](../evaluation/0048-fault-entry-contract-audit.md) establishes
that P4 runtime history is not ordinary handler data, abstract routes do not identify
actual emitting activity occurrences and retry binding excludes single-attempt
emission declarations. A provider uncertainty may need reconciliation/handling with
no eligible retry. Follow the [frozen raw-core plan](../evaluation/0048-general-fault-source-plan.md),
`273603d`, before designing typed history or handler entry.

## Decision

Publish exact [core 0.6](../dialects/core-emission-source-v0.6.md), selecting IR 0.2
and inheriting all five prior exact artifacts. Only the versioned work policy gains
required nullable outer `faultSource`. Reuse verified provider-fault-source metadata;
no wire field, source grammar, source profile, quota or actual event is added here.

For one attempt, keep empty eligible faults and null inner retry source/clock/basis;
outer source may be non-null. For multiple attempts, retain every inherited rule
and require exact outer/inner source equality. Null outer source means no general
use binding, never absence of possible faults. Metadata read closure includes source
protection even on unused policies. Every non-null-source activity use, including
unvisited ones, requires capability mode and exact actual provider ID/revision.
Human, type-equivalent other-provider and inconsistent shared-policy uses refuse.

Return immutable private-constructor `DeclaredFaultSourceUse` rows through the
retained static report, with revision/site/work-policy/source/profile/provider and
shared all-name emission/evidence/observation metadata. Include unvisited declared
uses without activation; unused policies yield no use row. Keep all names separate
from retry eligibility and emitted/accepted faults. Share one revision and immutable
source metadata per result under the existing cumulative work budget; empty legacy
paths add zero work.

The conservative fault-name catalogue uses general sources on core 0.6, including
single-attempt/noneligible names. Preserve the previous source path for cores 0.4 /
0.5. Deduplicate pins and keep resource/name/graph work cumulative and bounded.
Scope/name routing still does not authenticate an emitting site, cause, input
revision, actual fault or handler snapshot. Every source/classification/current
access/authority/eligibility/cutoff/selection/settlement requirement remains live.
No actual engine fault becomes catchable by a matching label.

Use a closed explicit known-core list to verify inherited artifacts newest first,
preserving prior verification/refusal order. Shared exact predicates retain retry,
capture and finite metadata family behavior through core 0.6; unknown versions are
not accepted by prefix or numerical comparison. All old artifact bytes and hashes
remain fixed, including old work fields and current package reports.

## Alternatives and consequences

| Option | Benefit | Limitation | Decision |
| --- | --- | --- | --- |
| Separate outer work emission source | Covers all declared provider emission uses without making them retry-eligible | New explicit core payload and actual-use checks | Adopt |
| Reuse inner retry binding for one attempt | Smaller field set | Violates inherited N=1 source/eligibility contract and conflates distinct judgments | Reject |
| Infer source from handler key, type or provider spelling | Short declarations | Cannot establish exact provider/use relation or actual source truth | Reject |
| Add fault-history values/handler ports in this increment | Appears to complete entry | Requires exact history type/protection and reviewed wire/entry initialization | Defer to a separate frozen design |

[Evidence](../evaluation/0048-general-fault-source-binding.md) covers real raw SDK
checks, inherited capture/owned-local/retry stages, source identity and legacy
boundaries, thirteen shared finite vectors, independent Python hash/shape/rule
checks and separate completed-diff automated self-review. These are declaration
judgments, not events, source authentication or independent human approval.

Next explicitly select this core in readable authoring/companion and bounded native
use disclosure. Typed history/source-control protection and handler-local entry
remain a subsequent reviewed extension, followed by permitted owned continuation /
cutoff/all-settlement. Keep the committed source-0.7/core-0.4/IR-0.1 package unchanged
until separate adoption. Parent #46 stays draft/incomplete, all forty scenarios stay
Partial, and full validator/benchmarks, general authoring/visual/protected editing
and actual user/host/browser/independent/phase gates remain open.
