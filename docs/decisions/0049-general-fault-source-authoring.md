<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0049: Name general fault sources once in readable work policies

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-07<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

Core 0.6 now distinguishes general provider emission declarations from retry
eligibility. Source 0.8 still selects core 0.5 and names a source only in the
multiple-attempt retry sentence. Extending it implicitly would change accepted
programs and could confuse noneligible/single-attempt emissions with retry permits.
Follow the [frozen integration plan](../evaluation/0049-general-fault-authoring-plan.md),
`f33c23f`, with every earlier grammar/schema/hash domain unchanged.

## Decision

Publish [source 0.9 / companion 0.8](../authoring/profile-v0.9.md), selecting exact
core 0.6 and IR 0.2 plus all inherited bases. A work policy optionally adds:

```
Recognise faults from "Notice faults".
```

It names a normally visible provider-fault-source resource once, after authority.
Single attempts may omit it and retain null outer source/inner retry fields. Multiple
attempts require it, then use the shorter existing clock/basis sentence:

```
Recognise faults from "Payment faults".
For retries use clock "Expense clock", basis "Payable request".
```

The explicit general source supplies the exact inner retry source too; no source
name is duplicated or inferred from a provider/fault label. Source 0.7/0.8 keep
their previous retry sentence. No resource kind, quota, private visibility exception
or actual-source assumption is introduced. CLI embeds only known semantics/core
artifacts; actual source/profile/provider/type bytes remain explicit.

Derive source protection/read closure from the outer source on the new profile,
without walking the same general/inner metadata twice for multiple attempts.
Keep finite closure, actual-use endpoints, source spans and all inherited semantic
revalidation/refusal boundaries. Known program quotas and legacy fragment/unknown
bounds stay fixed; unknown source sentinel becomes 1.0, not a new supported version.

All ten checking commands plus advisory invalidation requirements disclose nonempty
`declaredFaultSourceUses` from retained static reports, including unvisited declarations,
with revision/site/policy/source/profile/provider and live host requirements. Return
all-name emission/evidence/observation metadata once per exact source pin in
`declaredFaultSourceCatalogues`, separate from eligible retry bindings. Source/profile /
provider identity remains explicit on each use. Do not rerun validation for output.
The shared static-output helper bounds capture/use/catalogue row accumulation and
final combined JSON at 1 MiB before stdout; empty fields remain omitted on earlier
reports. Retain retry, historical, isolated and conditional owned-local views.

## Alternatives and consequences

| Option | Benefit | Cost or limitation | Decision |
| --- | --- | --- | --- |
| One readable general-source sentence and shared retry source | Shorter, explicit source without conflating eligibility | New profile/core pairing and lowering rule | Adopt |
| Repeat the source in general and retry clauses | Direct mapping to duplicate raw fields | More authoring noise and potential disagreement | Reject for this profile; old grammar stays fixed |
| Repeat all emission metadata on every native use | Self-contained rows | Large repeated maps and avoidable report exhaustion | Use exact per-source catalogue sharing |
| Infer source/classification or silently promote old grammar | No new sentence | Changes meaning and cannot prove exact source relation | Reject |

[Evidence](../evaluation/0049-general-fault-source-authoring.md) covers promoted
source copies, raw semantic revalidation, all eleven disclosure views, legacy/hash /
role/visibility/read/budget boundaries and separate completed-diff automated review.
These are declarations, never emitted faults, history values, handler snapshots,
source authentication, current grants or execution. General tool-managed resource /
identity editing, complete plain-English authoring and actual user evaluation remain
open; manual companion bookkeeping is not the ordinary-author product goal.

Keep the committed RP-01 source-0.7/core-0.4/IR-0.1 package and guard/report bytes
unchanged until separate adoption. Next freeze exact typed fault history/source-
control protection and reviewed handler-local entry, then permitted owned
continuation/cutoff/all-settlement. Parent #46 stays draft/incomplete, all forty
scenarios stay Partial, and all full validator/benchmark/visual/protected editing /
actual user/host/browser/independent/phase gates remain open.
