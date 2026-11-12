<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Provider interference regions: standalone metadata profile 0.1.0

**Contract ID:** `urn:choreoform:interference-profile:provider-regions:0.1.0`

These exact UTF-8 bytes are the normative artifact. Referenced documents/schemas
are explanatory validation aids, not moving normative dependencies. This profile
specifies portable declarations and their static type judgments. Loading them does
not select them in a semantic definition, prove generic independence, canonicalize
a live key or admit execution. Definition binding, new core/source versions and
concurrency proofs require separate contracts. Existing C12/P3 meanings stay fixed.

## Exact identities and closed resource forms

Use strict bounded UTF-8 JSON with duplicate-key/float/non-finite/surrogate refusal
and the existing safe JSON-integer transport. Ref is exactly `{id,revision}`;
revision is `sha256:` and 64 lowercase hex digits, hashing every stored resource
byte without normalization. New resource IDs/Ref IDs/selector field names have
1..256 UTF-8 bytes. Identity includes both ID and revision. Version is exactly
`0.1.0`; no ranges, unknown fields, aliases, default profile or network lookup.

The descriptor resource has exactly these fields:

| Field | Shape and meaning |
| --- | --- |
| `kind` | Literal `external-interference` |
| `version` | Literal `0.1.0` |
| `id` | Resource identity |
| `profile` | Exact Ref to this artifact |
| `providerProfile` | Exact `urn:choreoform:contracts:core-profile:0.12.0` / `sha256:73158731c33084fde009abcc86530926447902609fda9535f04daa7c768c2007` |
| `provider` | Exact Ref to a supported closed P3 provider-contract0.1 resource |
| `coverage` | Literal `exhaustive`, a declared behavior claim requiring independent truth/support evidence |
| `activity` | Footprint or literal `opaque` |
| `terminal` | Footprint or literal `opaque`, for the provider's correlated terminal observation channel |

`providerProfile` fixes the existing P3/V1/V2 interpretation. It does not select
C12 as a definition dialect or bypass C12's inherited-core requirements when C12
is actually selected in a definition. Supply and verify this exact artifact pin.
The provider's entire shape, input/output/observation types, outcomes, result
protection, idempotency and effect class must validate under the existing reader.
No class or protection is inferred from a caption or altered by this supplement.

A Footprint is exactly `{reads:[Region],writes:[Region]}`. Each list has 0..16
regions. A Region is exactly `{namespace:Ref,key:[Selector]}`, with 1..4 selectors.
A non-read-only activity finite footprint must declare at least one write.
Read-only activity and every terminal finite footprint must declare zero writes.
`opaque` carries no finite independence premise; omission of a descriptor also
supplies no premise. These are exhaustive read/observation and write/effect claims,
including every indirect/shared backend, quota, index, account, domain invariant,
logging or other interference relevant to the operation. Honest partial knowledge
must remain opaque, never an empty footprint asserted to silence a checker.

A namespace resource is exactly `{kind:"external-region-namespace",
version:"0.1.0",id,profile,authority:Ref,keyTypes:[K]}`. `profile` is this exact
artifact Ref. Arity is 1..4, K is literal `text` or `integer`. `authority` pins a
nonempty UTF-8 canonicalization behavior/trust contract. Verify supplied exact
bytes, but do not infer support, trust, consent or current identity from integrity.
Metadata decoding does not interpret that contract as host code or register it.
Unsupported actual authority behavior prevents any later independence admission.

## Finite original-request selector type judgment

A Selector is exactly `{path:[Step],type:K}`. Path length is 0..8. Steps are only:

| Exact Step | Required current type | Result type |
| --- | --- | --- |
| `{op:"field",name:Name}` | Record with that exact field | That field's type |
| `{op:"present"}` | Option(T) | T; a later absent runtime value remains unproved |
| `{op:"representation"}` | Named type with supplied exact supported V2 declaration | Its representation, explicitly preserving original protection/provenance |

Start at the exact provider input type. Resolve every nominal dependency, reject
cycles/unknowns and statically validate all invariant expressions, including unused
named declarations, with existing V1/V2 rules. No nominal membership is inferred
from representation shape. At the leaf the exact type must be Text or Integer as
declared by K, and it must equal the corresponding namespace component type.
Empty path is valid only for such a scalar input. Type-check every selector of
both finite channels, even if no actual call/observation uses it. Unknown fields,
wrong steps/arity/types or implicit nominal erasure refuse. Other primitive/decimal/
variant/list leaves, expressions, constants, defaults, concatenation, indexing,
iteration, coercion, normalization callbacks, current-cell and host-fact reads
are unsupported. An opaque channel supplies no selectors to type-check.

This is static metadata, not a request-value API. Eventual evaluation requires an
actually accepted immutable original request, with full V1/V2 value/invariant
admission and permission. Integer leaves use canonical signed-64 decimal strings;
text is exact Unicode with no implicit normalization. Unknown/missing/unavailable
values, absent Options and unsupported nominal/authority implementations leave
independence unproved, never an empty footprint. Selecting a representation does
not erase sensitivity, access, purpose, control dependency or nominal provenance.

## Canonicalization and conditional independence contract

A separately supported common authority must map every used `(namespace Ref,
typed key tuple)` to a canonical physical `(domain,typed key tuple)`, exhaustively
covering cross-provider/namespace aliases and shared infrastructure. It must
certify distinct canonical regions non-aliasing and retain stable meanings while
old work survives. Domains come from that authority, not author captions. Distinct
provider IDs, namespace pins, raw strings, nominal predicates or authorities do
not prove physical separation. Unknown/unregistered/ambiguous mappings remain
unproved; separate registries cannot silently be assumed disjoint. No normalizer
code, canonicalization table or live mapping API is defined by this reader profile.

For fully resolved finite footprints, writes conflict with another footprint's
reads or writes: `W(A) intersect (R(B) union W(B))` and `W(B) intersect R(A)` must
both be empty. Read/read overlap is allowed. This conditional mathematical rule
does not itself provide the canonicalization premise or a generic proof. A generic
definition must establish disjointness for all admissible overlapping occurrences,
including earlier surviving work and revised inputs. Concrete example values or
assumed mapping tables cannot supply that theorem. Human/unsupported work stays
opaque. Storage/control/protection/availability/invalidation, irreversible/domain
invariants and all lifetime/ownership gates remain independent requirements.

## Correlation, uncertainty and surviving obligations

Every evaluated footprint must retain exact definition/descriptor/provider/type /
namespace/authority revisions, scope/activity occurrence, logical effect, input
revision vector and digest of the typed accepted original request. The logical
effect and actual occurrence cannot be renamed to substitute another request.
Attempt identity is separate; technical retries retain the same request/revisions/
logical key. New business inputs require new actual activity occurrence and logical
effect, not reuse of the old attempt budget. Compare both original snapshots
across input revisions/cohorts; an inequality within one revision cannot be reused
against earlier unresolved work.

Terminal-channel selectors start at the exact retained original activity request,
including after capture or handoff, never a reply payload/current replacement
cell. Actual source, event, attempt, request and receiver correlation/authority /
protection still require evidence. A correlated terminal observation is not new
business work; an unrelated query is a new activity. Compensation is new logical
work with its own request/footprint/key and an independently eligible prior effect.

Requested/acknowledged/timed-out/ambiguous/unknown attempts retain possible external
interference. Cancellation requests, cutoffs, lease expiry or one logical success
do not prove every physical attempt quiescent. Actual correlated provider evidence
must cover late/duplicate attempts and no further conflicting access. No descriptor
grants cancellation, dispatch, settlement or permission to erase uncertainty.
Reconciliation must remain possible under the original request and its observation
footprint. Durable reservation/payment/compensation obligations survive operational
quiescence: releasing an operational exclusion never undoes a lasting effect.

Ownership handoff must be authorized and atomic with no gap/loss, retaining every
transitive unresolved attempt, capture, subscription and business obligation, and
checking current receiver support, protection and authority. No pin or contract
creates a receiver or proves accepted transfer. Runtime locking is not an alternate
admission route here; such a future option requires its own durable atomic global
admission/fencing/crash-recovery/external-actor/uncertainty/handoff implementation
and real host evidence. Existing P3/P4/P6 settlement/closure rules remain mandatory.

## Portable metadata implementation and bounded failure

Use the existing SDK registry bounds: at most 256 supplied resources, each at most
1 MiB and total at most 8 MiB, JSON/type depth64 and one shared cumulative 1000000
semantic-work budget, never reset per descriptor/namespace/selector. The fixed
footprint/key/path/name bounds above apply before allocation/traversal. Charge
metadata elements, Ref verification and path/type/nominal work; use shared immutable
decoded caches. Walk only the selected type path without cloning a whole provider
type for each selector. Failure publishes no partially decoded successful parent;
already fully validated child metadata may remain cached. Repeated failures must
not bypass validation or clear the cumulative budget.

Expose only immutable decoded resource metadata and explicit remaining obligations,
not independence/execution Booleans or live accepted-request/actor/host state.
Error codes contain no resource payloads, selected keys, request values, authority
contract text or exception details. Future value-derived results must retain input
protection labels and authorized disclosure rules, including their result status.
No new public source/core diagnostic family or process report is activated here.

This production metadata profile deliberately uses existing SDK quotas rather
than draft79's tighter disposable comparison limits (100000 work/1MiB total model
input/1024-byte fixed result). No comparison result is produced here; existing SDK
quota meanings are unchanged. Draft79 remains immutable, inactive model evidence.
Every new resource version/profile binding is explicit; no draft resource or old
provider/core artifact is reinterpreted. A registry may store multiple versions;
loading is not selection. Future semantic definition binding must select exactly
one descriptor per exact provider, include its transitive dependency closure in
the semantic revision, and reject conflicting selections/old unsupported versions.
Managed near-English bindings and source/raw equivalence require separate versions.

Integrity/static metadata never proves provider truth, actual canonical mappings,
host/browser execution, current actor permission, user/independent evaluation or
any Phase 1 exit. Complete generic validation, representative text/visual cases
and actual external evidence remain separate work. This profile starts no Phase 2.
