<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Candidate A authoring profile 0.12.0

**Status:** Accepted under [ADR-0058](../decisions/0058-readonly-continuation-authoring.md)
and owner delegation; effective on parent/main integration. Declarations and
conditional proof supply no actual request, value, storage, authority or execution.

Inherit [source 0.11](profile-v0.11.md) at baseline
`e38f7caf674416a00a718f4ca77940883e086b58`, except below. Header is
`Choreoform authoring "0.12.0".`, companion version `0.11.0` / profile `0.12.0`.
Use the [closed schema](../../schemas/authoring/bindings-0.11.schema.json) and
[versioned grammar](candidate-a-v0.12.ebnf). All grammar productions are inherited
unchanged; semantic selection is explicit. No new sentence, resource kind, value,
receipt or authority syntax is introduced. Earlier profiles and artifacts stay fixed.

Select exact [core 0.9](../dialects/core-owned-readonly-v0.9.md),
`sha256:5faaf24217bcc5644a9ce5975b2c6ad7074d0197d39a73ab648d6cb3a9ced87f`,
and existing [IR 0.4](../ir/definition-v0.4.md). Libraries require semantics plus all
nine core artifacts; CLI embeds only known semantics/core bytes. Supported exact
source/profile/provider/type/history metadata remains explicit. Every required
artifact, including unused resource declarations, retains its normal checks.

Existing nested scopes and sentences express the finite read-only family:

```
Retain the original request from "Submit payment" and record in "Original request".
Let "Lookup input" mean (the value of "Original request") as record with empty with parameters empty.
Step "Check status". Request "Read-only status service" using "Lookup input" under "Lookup work" and record in "Status". On "done", continue with "Finish recovery". End step.
```

This fragment needs the normal matching declarations, exact types/provider/effect /
general source, capturing entry and designated surviving child. A caption or
read-only label does not establish those contracts. Core 0.9 requires complete
conditional source/input/access/control/provider provenance and explicit query
capability recipients, including when no result cell is present. One attempt,
no clock/timer/compensation, exact readonly input and every raw barrier remain.
Old flat owned proof, ordinary coverage and legacy sets keep their meanings.

Whole programs retain 16,384 tokens / 1,000,000 parser visits, shared 100,000
lowering/dependency work, depth 64 and 1 MiB transport. Unknown headers/fragments
retain earlier quotas. Package framing names source 0.12 explicitly. Formatting /
reparse/coordinated aliases preserve within-profile identity. Original UTF-8 private
history/request clause spans and public span inventories remain unchanged.

## Native disclosure

All eleven checking/advisory views preserve inherited static declarations.
The six views whose SDK chain includes concurrency add nonempty ownedContinuations
(creator/owner), one shared ownedContinuationScopes catalogue (rootOwner/owner /
scope/incoming closure/source/request/query/child/conditionalSource), and separate
conditionalOwnedContinuationChecks. It contains revision, conditional nodes/scopes,
transfers, access/provenance/invalidation, remaining obligations and declared gates.
The gate rows retain exact kind/owner/source/receiver/management actor and mandatory
host requirements; no gate is represented as accepted. Static request relations in
that same revision identify the already-checked exact input type without copying
type schemas into each row. Scope closure means its source owner's incoming policy.

Final reconciliation adds nonempty continuationScopes separately from
retainedRequestScopes and legacy checked sets. Conditional nodes remain legacy
uncovered. Earlier views disclose declarations without inventing conditional proof.
Empty fields remain omitted; all older/default report bytes remain fixed.

Reuse the SDK report without another admission pass. Reserve inherited output
before accumulating bounded creator/scope/gate/access/provenance rows, deduplicate
scope relations across reused creators, and retain the final complete 1 MiB writer
bound before stdout. Overflow returns report-budget without partial output.

Reports contain no actual event/request/value/storage/grant/receipt/ledger and
dispatch nothing. Current permissions, open/noncancelled registration cutoff,
exact occurrence/request/input/value revisions/digest/effect/key/attempt,
authenticated provenance, fresh immutable storage, receiver acceptance and atomic
single-owner handoff remain live host premises. Failed acceptance leaves the old
owner accountable. Terminal disposal requires all actually activated transitive
effects/subscriptions/children settled. Finite shape proves no eventual settlement.

Committed RP-01 remains source 0.7/core 0.4/IR 0.1, all forty scenarios Partial,
G1–G4 conditional. General command/compensation and richer ownership/fault contexts,
complete validator/benchmarks, general tool-managed near-English/protected accessible
visual editing and actual host/browser/user/independent/phase evidence stay open.
