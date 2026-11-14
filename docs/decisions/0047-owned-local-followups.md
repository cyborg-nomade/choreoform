<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0047: Check isolated fresh-local follow-ups with retained protection provenance

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-07<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

ADR-0045/0046 gives raw/readable exact terminal capture, but storage in a surviving
body remains outside ADR-0039's data-free isolation proof. Relaxing only its storage
check would omit initialization, source/control information flow and invalidation.
The [frozen plan](../evaluation/0047-owned-local-followups-plan.md), `afed903`, and
[provenance refinement](../evaluation/0047-owned-local-provenance-refinement.md),
`cdac368`, require a separate conditional proof. No wire/core/source artifact bytes
or actual host lifetime guarantees are changed by this analysis increment.

## Decision

Add a distinct private-constructor `OwnedLocalReport` to composed concurrency
results. Retain the existing `IsolatedFollowup` type and data-free behavior. A
covered synchronous invoke/repeat may obtain the new proof only when its reconcile
closure names its direct child follow-up with no nested scopes, no ports and
terminal closure. No independent declared creator may target that follow-up.
Its entry is an explicit core-0.5 capturing wait. Every declared node, including
unvisited ones, is a flat timerless wait, compute, decision or finish. Complete
node reads/writes are confined to that follow-up's own cells. Require a reachable
finish. Ordinary source/type/provider/visibility checks still apply to every use.
Activities/effects, parallel/nested work, foreign writes/ancestor reads and ordinary
parent resumption remain outside this subset.

Group candidate entries and check zero inherited values with fresh local
initialization using the existing bounded control engine. Reuse protected flow /
access/dependency analysis for those entries, then require strict conditional local
invalidation. Never promote advisory requirements into a passed report. Keep all
ordinary coverage/evidence intact and name conditional coverage separately. Share
immutable templates by body while every actual creator occurrence still requires
its own fresh storage and occurrence-bound handoff.

### Zero inherited values does not erase protection provenance

The initial unpublished draft's empty protection context was incorrect. A local
capture may depend on the restricted decision that created its owner or on source /
control influences inside partially completed owner work. Retain a bounded union
of those immutable definition-bound label references in the already composed
ordinary protection report, including partial child influence and provider/clock
roles. Enable collection only when explicit captures exist; old profiles retain
previous work/behavior. Do not rerun static/ordinary validation to reconstruct it.

For conditional entry, convert cell and ordinary historical influences to a distinct
private protection-provenance origin. It supplies the definition's protection label,
never a current cell value, permission cache, ordinary-completion claim or fabricated
historical receipt. Preserve provider envelopes as separate origins. Propagate
these labels through local initialization, capture, computations and branches;
refuse a wider destination even when its direct capture source agrees with it.

Require live P6 retention of authenticated occurrence-bound source/control protection
provenance through handoff and disposal. The report names required owner/cell or
provider-node/role declarations and the exact definition revision, but supplies no
actual receipt. Immutable provenance labels cannot initialize a cell or satisfy an
ancestor read. Actual current access, authority, source/correlation, atomic single
acceptance, cancellation/closure cutoffs, subscriptions and all-obligation settlement
remain explicit host requirements. Observable-channel/general exceptional protections
are still open; these internal conditional checks do not close them.

### Disclosure and bounds

The optional typed report includes creator/body/closure/follow-up/local-cell/capture
relations, conditional coverage, envelope-comparison counts and access/invalidation /
provenance requirements. All fields are immutable; constructors stay private.
Native concurrency and every later composed view add `ownedLocalFollowups` and
`conditionalOwnedLocalChecks` only when present. Omit both on earlier/data-free
programs. Bound accumulated rows and final combined 1 MiB JSON before stdout.
All indexing, proof walks, retained contexts, template copies and rows use existing
stage work allowances; grouped entry checks use one cumulative allowance per stage.
No quota refill, invented occurrence or live authority cache is introduced.

## Alternatives and consequences

| Option | Benefit | Cost or limitation | Decision |
| --- | --- | --- | --- |
| Separate isolated fresh-local conditional checks | Sound inspectable storage/dataflow subset and honest residual host premises | Adds grouped control/protection/invalidation evidence | Adopt |
| Remove the old storage prohibition alone | Small patch | Omits initialization, information flow and invalidation | Reject |
| Empty protection context because values are fresh | Simple entry model | Can erase creator/owner source/control restrictions | Reject; refined before publication |
| Call surviving bodies ordinary-reachable or retain ancestor values implicitly | Reuses broader reports | Invents activation, snapshots and lifetime | Reject |
| Implement host runtime ownership in the validator | Appears to settle live storage | Validation has no execution, observation or host authority | Defer actual engine work to its phase |

A full RP-01 promoted copy can now capture a fresh local and pass the conditional
subset; a local-read/branch/compute test also passes. The original guards and real
nominal payload identities remain. This is no actual late result, reopened path,
fault-data port, owner allocation or executable scenario. Keep the committed
source-0.7/core-0.4/IR-0.1 package unchanged until separate adoption after remaining
fault-data and permitted continuation/cutoff/settlement work. General retention,
complete validator/benchmarks, plain-English/tool-managed/visual/protected editing
and actual user/host/browser/independent evidence remain open. Parent #46 stays
draft/incomplete and no scenario or phase gate closes. See
[evidence](../evaluation/0047-owned-local-followups.md) for tests, independent finite
Python checks and separate completed-diff automated self-review; no individual
owner or independent human approval is claimed.
