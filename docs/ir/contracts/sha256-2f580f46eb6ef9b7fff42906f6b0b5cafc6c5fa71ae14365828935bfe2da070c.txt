<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Provider fault-history metadata profile, version 0.1.0

**Contract ID:** `urn:choreoform:fault-history-profile:provider-observations:0.1.0`

This independently pinned profile defines a declared history type and source mapping,
not an authenticated event, accepted fault, definition binding or handler entry.
It changes no existing wire, core, policy, source grammar, retry or lifecycle rule.
Every normative byte is frozen in a separately supplied artifact; changed bytes
require a new explicit version. No URI permits discovery or network access.

## Exact resource and nominal type

```
{kind:"provider-fault-history",version:"0.1.0",id:Name,profile:Ref,source:Ref}
```

Ref is exactly `{id:Name,revision:"sha256:" + 64 lowercase hexadecimal digits}`.
Verify exact resource/profile/source/source-profile/provider and transitive type
bytes in the bounded explicit registry. Resource ID equals its supplied identity;
profile equals this implemented artifact. Source must be an exact supported
provider-fault-source under the provider-observations 0.1.0 profile. Unknown fields,
kinds, versions, profiles or arbitrary hashed profile bytes refuse.

The history type is exactly named type `Provider fault history` in artifact
`urn:choreoform:types:provider-fault-history:0.1.0` at
`sha256:a2e177ce1dd4bad5b9cd68b74e6da7749ec2b04b68a39dae1bd72729e2aae7a7`. Verify and decode that fixed artifact, not a caller-selected
shape-equivalent nominal type. Its closed representation is:

```
{type:Text,
 sourceOccurrence:{definitionRevision:Text,instance:Text,scopeOccurrence:Text,
                   site:Text,occurrence:Text,attempt:Integer},
 cause:{observation:Text,record:Text},
 inputRevisions:List<{cell:Text,revision:Text}>}
```

Integer uses the inherited canonical signed-64 string encoding. Type names an exact
source emission; cause.observation names a provider observation mapped by that emission.
Cause.record is an opaque original evidence/history reference, not embedded inputs,
results, an authorization token or proof that evidence exists. Source/provider pins
remain external descriptor identity and must match any future actual-use binding.
Definition revision, instance/scope/node occurrence, attempt and cell revisions are
claims to authenticate against the actual engine/host event; IDs alone prove none.
Input cell revision strings are opaque revision identities, not necessarily artifact
SHA-256 digests. Empty input lists are permitted for genuinely input-free work.

The nominal invariant is always true: ordinary literal admission checks representation
only. A fabricated well-formed value can have this type. Nominal identity, a digest,
metadata decoding and the limited claim check below never attest that a fault occurred.
No constructor or Boolean field can turn this value into an authenticated entry.

## Limited claim judgment

The SDK may check a caller-supplied claim's exact nominal representation, membership
of type/observation in this descriptor's source mapping, nonempty identity/reference
strings within the existing Name bound, canonical definition-revision digest syntax,
attempt 1..1000, and unique input cell IDs. Invalid claims refuse; success returns
unit only. Charge all repeated work to the existing cumulative resource budget,
including cached descriptor lookup and repeated claims. No actual event/entry report
is produced, no definition or host cell is read, and no policy is activated.

Evidence classification remains the exact mapped source metadata. In particular,
provider unknown stays unresolved even if a fault label sounds like success or
no-effect; successful/acknowledged observations cannot enter through this profile.
The claim check does not prove source truth, real attempt/input/request/key/logical-
effect correlation, actual fault class, engine-fault exclusion, current authority,
nearest handler selection or active occurrence. No ordinary value can supply those.

## Protection and entry remain separate

Expose the source provider's declared resultProtection as a lower bound only, not
the complete history envelope. A future entry must additionally retain and combine
all protection from source inputs, actual fault existence and control dependencies,
and creator/owner contexts. Zero inherited values never implies zero provenance.
Missing or incompatible provenance blocks admission; the descriptor cannot discard
it, infer public data or authorize declassification/current access.

P4 history is immutable once actually recorded. A future explicit handler-local
entry contract must verify actual source activity/occurrence/provider/fault/cause /
input revisions and exact nearest handler, freshly initialize an owned read-only
cell, forbid ordinary impersonating writers, and check complete conditional access,
protection, availability and strict invalidation. Existing scope records remain
closed and handlers still have no ordinary ports or readable fault facts. Unsupported
human, unbound and engine origins do not inherit provider history from a label.

Retries retain every P4 evidence/budget/delay/original-input/current-authority/cutoff
premise. Cancellation is monotonic; this metadata cannot reopen a closed activity,
resume its parent, authorize new surviving work, transfer ownership, settle obligations
or permit terminal disposal. Actual authenticated host/adapter execution, source /
control retention, owned handoff and all-obligation settlement remain mandatory.
The reader has no IO, acceptance, authentication, routing, dispatch or execution API.
