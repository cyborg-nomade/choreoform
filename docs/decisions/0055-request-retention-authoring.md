<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0055: Author original-request retention through an explicit readable clause

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-08<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

ADR-0054 supplies raw IR 0.4/core 0.8 original-input declarations and separate owned
conditional proof. Current source 0.10 cannot select that wire or express the
binding; native reports cannot disclose it. Follow the [frozen integration plan](../evaluation/0055-request-retention-authoring-plan.md),
`e9af4dc`, preserving all earlier meanings and committed RP-01 evidence.

## Decision

Publish [source 0.11 / companion 0.10](../authoring/profile-v0.11.md), explicitly
selecting existing exact core 0.8/IR 0.4 and all seven inherited bases. Add one
optional nested scope clause after its interface/history clause, before contents:
`Retain the original request from "Request Payment" and record in "Original request".`
The delimiter separates normally qualified references containing `in`. Resolve a
visible node and local data cell; no private endpoint exception, value, initializer
or snapshot is invented. Root has no clause. Every scope lowers an empty required
retainedRequests map, or its one explicit result/origin pair. Keep nullable
faultInput, capture and all raw source/type/readonly/creator/access/protection /
invalidation barriers. Syntax may express both clauses but cannot bypass their
incompatible handler/owner restrictions.

Inherit exactly source 0.10's resource kinds and current quotas. Libraries require
all exact artifacts; CLI embeds only selected known core/semantics bytes. Keep
explicit known-profile membership, unknown/fragment legacy bounds and distinct
0.11 package framing. Formatting and coordinated aliases preserve semantic
identity within this profile. Map retainedRequests paths to original UTF-8 clause
spans without changing existing public declaration/generated span inventories.

All eleven checking/advisory views disclose nonempty declaredRetainedRequests
from retained static rows, including unvisited declarations: exact revision/scope /
origin/result and mandatory host premises. Origin/result identify the exact
validated input-type relation in that revision; the SDK exposes the descriptor.
Avoid repeated schemas and another type serializer. Existing owned follow-up rows
add originalRequest only when present. Final reconciliation discloses nonempty
retainedRequestScopes while keeping its prior owned proof and legacy uncovered
bodies distinct. Reuse the existing chain; no output admission rerun.

Bound static rows cumulatively with inherited output reserved first, and the
complete final 1 MiB JSON before stdout. Reuse the bounded writer for owned rows;
preserve every older/default field when empty. Overflow refuses before partial
stdout, with no truncation or silently dropped declaration. Existing capture/source /
history/retry/historical/isolated/owned evidence remains separate from actual events.

## Alternatives and consequences

Keeping raw-only support prevents normal readable review. Re-reading current
parent data or using an initializer changes original request identity and access
ordering. Repeating full type schemas wastes bounded report space; revision-bound
origin/result relations and the typed SDK give the exact relation without copies.
Implicit migration would change older wire/hash/core contracts; explicit pairing
keeps those meanings fixed.

This is intermediate near-English syntax, not general editing/usability completion.
Actual original accepted values/revisions/digests, occurrence/request/key/effect /
attempt correlation, immutable owned storage, authenticated complete provenance,
current authority and atomic accepted handoff/cutoff/all-settlement remain live
premises. No new work is dispatched and no parent resumes or clears cancellation.
Next permitted separately authorized new work inside the surviving boundary with
explicit cutoff/handoff/all-settlement. General tool-managed English editing,
complete validator/benchmarks, protected accessible visual tooling and real
host/browser/user/independent gates remain open. Parent #46 stays draft/incomplete;
all forty scenarios Partial, no Phase 2 or phase exit. See [evidence](../evaluation/0055-request-retention-authoring.md).
