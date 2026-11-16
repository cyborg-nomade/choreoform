<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0054: Retain exact immutable original requests in owned follow-ups

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-08<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

The bounded fresh-local owned proof can capture a terminal result and retain
source/control labels. It cannot read the original request after parent disposal.
A current ancestor value, initializer or fault-history identifier cannot provide
that value. P6 requires occurrence-owned surviving storage and accepted handoff;
it does not grant arbitrary ancestor access or ordinary resumption. Follow the
[frozen plan](../evaluation/0054-retained-request-plan.md), `d0015f6`.

## Decision

Publish [Definition IR 0.4](../ir/definition-v0.4.md) and exact
[core 0.8](../dialects/core-retained-request-v0.8.md), with all seven inherited
bases. Every scope adds required retainedRequests mapping local data IDs to
origin node IDs; the initial semantic subset permits at most one mapping. Keep
all prior schemas/wires/core/source/hash meanings fixed. Older selected cores
cannot ignore the field; the unversioned constructor and illustrative inspector
retain their prior limits. Unknown wire tests move from newly supported 0.4 to
0.5; this grants no source profile or illustrative contract support.

Only a direct designated reconciliation child may bind its parent's exact
capability activity original input. Require exact input type including named
identity, local readonly uninitialized result, no ports/nested/ordinary creators /
fault-handler reuse, terminal closure and a capturing entry for that exact origin.
All capturing waits in this body share that origin; normal source/provider /
visibility/type checks stay mandatory. These are declarations, not captured values.

Extend the existing conditional fresh-local owned proof. Both synchronous creator
and origin require ordinary coverage. Check write access with empty pre-entry
values before seeding the explicit snapshot, then fresh local initialization and
local wait/compute/decision/finish flow. Preserve all-declared-body restrictions
and strict conditional invalidation. Reuse ordinary source/input/policy/control /
provider provenance through the existing retained-label mechanism and require the
exact activity-provider origin. That provenance never provides current ancestor
values or grants; constant/initializer/branch publication remains protected.
Share the single prior validation chain, templates and immutable typed relations;
charge indexing, copying, flow and publication cumulatively, refusing partial proof.

Static SDK rows disclose revision/scope/origin/result/exact input type and live
premises. Owned rows expose their distinct original-request relation. Later legacy
reconciliation availability recognizes already-proven snapshot scopes and retains
that prior owned proof separately; it excludes their bodies from its legacy sets
and leaves them uncovered there. Re-running a zero-seed legacy check would erase
the explicit initialization, while silently seeding every declaration would bypass
the owned source/access/protection/invalidation proof. No ordinary coverage changes.
Source/companion/native selection follows a separate explicit authoring increment;
all current source/default report/package bytes stay unchanged.

## Alternatives and consequences

Re-reading the parent's current input conflates revisions and relies on disposed
storage. Treating the snapshot as an ordinary initializer or port invents a value
and can bypass pre-entry access. Retaining an arbitrary ancestor graph is much
broader than the concrete original request needed for correlation and new-work
planning. Adopt a single exact request relation first; keep general retention,
dynamic bundles, new effects and continuation separate.

The host must install the one original accepted request before disposal, with
exact definition/instance/source/owner occurrence, input/value revision/digest,
logical effect/key/attempt correlation and authenticated complete provenance.
Retry cannot re-evaluate or replace it; changed business input requires new work.
Current source/target data/management authority, receiver acceptance, atomic
single-owner handoff, cancellation/closure cutoff and all-obligation settlement
remain live premises. Static proof creates no storage, receipt, current grant,
dispatch or disposal right. A handler cannot resume its parent or clear cancellation.

See [evidence](../evaluation/0054-owned-request-retention.md). Next readable retained
request selection, then permitted separately authorized new work inside the surviving
boundary with explicit cutoff/handoff/all-settlement. Full validator/benchmarks,
general tool-managed near-English/protected accessible visual editing and real
host/browser/user/independent gates remain open. Parent #46 stays draft/incomplete;
all forty scenarios Partial, no Phase 2 or phase exit.
