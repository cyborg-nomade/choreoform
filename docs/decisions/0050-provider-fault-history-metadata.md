<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0050: Separate provider history claims from authenticated handler entry

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-07<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

Core 0.6/source 0.9 declares general emission sources independently of retry.
P4 specifies immutable runtime history with type, source occurrence, cause and
input revisions, but current scope wires provide no ordinary history binding.
Abstract scope/name routing is not an actual emitting activity or event receipt.
The [fault-entry audit](../evaluation/0048-fault-entry-contract-audit.md) calls for
an exact type/source/protection foundation before introducing readable handler data.
Follow [frozen plan](../evaluation/0050-provider-fault-history-plan.md), `b9449d0`.

## Decision

Publish a separately pinned [provider history metadata profile](../dialects/provider-fault-history.md)
and [fixed nominal type](../dialects/provider-fault-history.types.json).
A closed provider-fault-history descriptor pins the profile and one existing exact
provider-fault-source. Verify all descriptor/profile/source/source-profile/provider /
transitive type bytes using the existing bounded explicit registry. No network or
ambient provider/name matching is added. Keep every old wire/core/source/hash/report
unchanged; this raw SDK metadata has no automatic definition or authoring binding.

The fixed nominal history representation has exactly four P4 fields: type; source
occurrence (definition revision, instance, scope occurrence, node site, activity
occurrence, attempt); original cause observation/reference; and input cell revision
identities. References are opaque claims, not raw inputs/results, grants or evidence
of an actual event. Keep source/provider identity in the exact descriptor, to be
matched by any future entry contract. The nominal invariant is true intentionally:
ordinary representation admission cannot attest that a history occurred.

Provide a distinctly named limited claim check. It checks literal representation,
exact source fault/observation mapping, nonempty bounded identity/reference strings,
canonical definition revision syntax, attempt 1..1000 and unique input cell IDs.
Success returns unit; it creates no receipt, event, entry snapshot or execution
permission. Even invented sites may pass this limited judgment. Evidence classes
remain exact source metadata; provider uncertainty never becomes success/no-effect
because of a convenient label. Actual occurrence/evidence/input/authority checks
remain mandatory and separate.

Expose the provider's declared protection as a lower bound, separately from required
source-input, fault-existence/control and creator/owner provenance. No inherited
values does not imply no provenance. Future entry must retain/combine those labels,
verify actual source contexts and exact nearest handler, initialize a read-only
local cell without impersonating writers, and prove conditional access/availability /
information flow and strict invalidation. Keep old handler ports/facts/wires closed.

Cache immutable descriptors by exact pin; share source metadata and fixed nominal
type across aliases without sharing any live event, grant or occurrence. All cached
lookups, claim checks and input rows consume the same cumulative resource allowance.
Publish only completed descriptor entries. Verified shared type metadata may remain
cached after a later descriptor construction failure; it conveys no history evidence.

## Alternatives and consequences

| Option | Benefit | Limitation / decision |
| --- | --- | --- |
| Exact nominal type plus source descriptor and limited claim check | Stable identity, small closed representation, testable source membership | Adopt; actual event/entry verification is still required |
| Caller-chosen shape-equivalent nominal type | Application flexibility | Reject here: weakens exact interchange identity and complicates later binding |
| Structural text/record value as authenticated history | Short implementation | Reject: any author can fabricate such a value |
| Embed raw inputs/results or a safe/approved flag | Convenient data access | Reject: leaks value/protection and cannot establish source truth or authority |
| Fold handler entry and owned continuation into metadata | Fewer apparent steps | Reject: actual origin/control/initialization and cancellation/lifetime judgments differ |

[Evidence](../evaluation/0050-provider-fault-history.md) covers exact pins/types,
closed resources, all finite claim cases, deliberate ordinary-value limitations,
protection lower bounds, shared caches and cumulative refusal. This is delegated
agent self-review, not independent/human or personal owner approval.

Next review explicit handler-local entry on a new wire, complete incoming actual
emitting-site/source/control contexts and conditional initialization/access/strict
invalidation, then readable authoring and permitted owned continuation/cutoff/all-
settlement. Fail closed on unsupported human/unbound/engine origins. Do not adopt
promoted fixtures silently. Parent #46 stays draft, all forty scenarios Partial,
and all complete validator/benchmark/authoring/visual/protected editing and actual
host/user/browser/independent/phase gates remain open.
