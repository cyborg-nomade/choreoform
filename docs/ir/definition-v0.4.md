<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Definition IR 0.4: explicit original-request retention declarations

**Status:** Accepted under [ADR-0054](../decisions/0054-owned-request-retention.md)
and owner delegation; effective on parent/main integration. Raw declaration and
conditional proof only; no actual retained value/storage, grant or execution.

Inherit [IR 0.3](definition-v0.3.md) at baseline
`54becbad2235e76cd9681a9b59120f261b57f818`, except one required scope field:

```
retainedRequests: { LocalDataID: OriginActivityNodeID }
```

The [closed schema](../../schemas/ir/definition-0.4.schema.json) permits an ID map;
structural projection resolves local data ownership and node roles. Empty means
no retained request. [Core 0.8](../dialects/core-retained-request-v0.8.md) permits
at most one entry, validating the designated owner/follow-up, exact capability
input type, readonly cell and capturing entry. It requires IR 0.4 and all seven
inherited core artifacts. Earlier selected dialects cannot ignore new fields.
All other wire records, nullable faultInput and wait capture stay as in IR 0.3.

Version participates in the canonical definition digest. Decode/structure/hash
checks remain separate from semantics. The unversioned constructor stays IR 0.1;
old schemas reject the new field, and unsupported 0.5 remains closed. The original
illustrative inspector rejects IR 0.4 even with its illustrative dialect. No
source profile automatically selects this wire.

The static report exposes exact declarations; the existing owned conditional
proof seeds only permitted original requests after empty pre-entry access checks.
Ordinary analysis never seeds them. Later reconciliation keeps the prior owned
proof separate, names retained_request_scopes, and leaves those bodies outside
legacy checked sets. Actual original request acceptance/correlation, immutable
owned storage, complete provenance, current authority and atomic handoff/cutoff /
all-settlement remain mandatory host premises. Neither IDs nor digests supply them.
