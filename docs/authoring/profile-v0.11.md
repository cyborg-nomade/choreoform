<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Candidate A authoring profile 0.11.0

**Status:** Accepted under [ADR-0055](../decisions/0055-request-retention-authoring.md)
and owner delegation; effective on parent/main integration. Declarations and
conditional proof grant no actual value/storage/authority/handoff or execution.

Inherit [source 0.10](profile-v0.10.md) at baseline
`30b3f228097692d5e63db418f570b9470f8acd2d`, except below. Header is
`Choreoform authoring "0.11.0".`, companion version `0.10.0` / profile `0.11.0`.
Use the [closed schema](../../schemas/authoring/bindings-0.10.schema.json) and
[versioned grammar](candidate-a-v0.11.ebnf). Every earlier grammar/schema/core /
wire/resource/hash/report domain stays fixed. No new resource kind is introduced.

## Original request clause

A nested scope may add one clause after its interface/history clause and before
its contents:

```
Retain the original request from "Request Payment" and record in "Original request".
```

Resolve the first reference normally as a node, the second as a local data cell.
Both may use qualified references containing `in`, separated by `and record in`.
Root process has no clause. Lower every scope's required retainedRequests map to
empty or this one result/origin pair. Do not insert an initializer, value or private
endpoint exception. Nullable faultInput and wait capture stay intact. Raw core
rules require the exact origin capability input type, readonly uninitialized local
storage, designated direct reconciliation ownership and exact capturing entry.
History-handler and request-retention clauses cannot override their incompatible
owner/handler barriers, even though the grammar can express both.

Select exact [core 0.8](../dialects/core-retained-request-v0.8.md),
`sha256:edc8195ba0c31536ab2cdc95089e8aecc3ff402489639b4435277f0a38e7b603`,
on [IR 0.4](../ir/definition-v0.4.md). Libraries require semantics and all eight
core artifacts; CLI embeds only known core/semantics bytes. Actual source/profile /
provider/type/history and other resources stay explicit and exact, including unused
declarations. Keep normal visibility and all type/provider/readonly/creator /
source/access/provenance/invalidation refusals.

Known whole programs retain 16,384 tokens / 1,000,000 parser visits, shared 100,000
lowering/dependency work and 1 MiB transport bounds. Unknown 1.0/fragments keep
legacy quotas. Package framing uses source 0.11 explicitly. Formatting/reparse /
coordinated aliases preserve within-profile IDs and IR; version/core/wire promotion
changes semantic hashes. Semantic retainedRequests paths map to the clause's
original UTF-8 byte span; public declaration/generated span inventories and
fault-input mapping remain unchanged.

## Native disclosure

All eleven checking/advisory views add declaredRetainedRequests only when nonempty.
Each row contains exact revision/scope/origin/result and host requirements. These
are declarations, including unvisited sites, not activated storage. The origin
capability input and result data type are exactly equal in that validated revision;
the SDK exposes the type. Reports avoid repeating full schemas.

Existing ownedLocalFollowups rows conditionally add originalRequest with origin /
result; conditionalOwnedLocalChecks remains separate from ordinary coverage.
Final reconciliation availability adds nonempty retainedRequestScopes, preserving
prior owned proof and leaving those nodes outside legacy checked sets and inside
legacy uncovered sets. No admission is rerun for output. Empty additions remain
omitted on all older/default reports. All existing evidence fields remain.

Static row accounting reserves inherited output before new declarations, uses
bounded serialization, and retains a complete final 1 MiB writer bound before
stdout. Overflow returns report-budget without partial output or truncation.
Catalogue sharing and declarations never share live values or current grants.

Actual original acceptance/value revisions/digest, exact occurrence/request/key /
effect/attempt correlation, fresh immutable owned storage, complete authenticated
provenance, current source/target/management grants, accepted atomic handoff,
cancellation/closure cutoff and all-settlement remain live premises. No new work
or ordinary resumption is implied. Committed RP-01 remains source 0.7/IR 0.1.
General tool-managed near-English/protected accessible visual editing, complete
validator/benchmarks and real host/browser/user/independent/phase gates remain open.
