<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Core profile 0.13.0: explicit provider interference selection

**Contract ID:** `urn:choreoform:contracts:core-profile:0.13.0`

These exact UTF-8 bytes form the normative artifact. Require all twelve exact
inherited cores, preserving their meanings except the explicitly revised P3
capability-contract wrapper below:

- `urn:choreoform:contracts:core-profile:0.12.0` / `sha256:73158731c33084fde009abcc86530926447902609fda9535f04daa7c768c2007`.
- `urn:choreoform:contracts:core-profile:0.11.0` / `sha256:246bf3835eef25f8adf76bce39e6b887715e098761e675f821bf745a8111d29e`.
- `urn:choreoform:contracts:core-profile:0.10.0` / `sha256:74c3d5bfe2a880259bc13f521127aa1dd84977ae9e6f83b339e3a0629964e60f`.
- `urn:choreoform:contracts:core-profile:0.9.0` / `sha256:5faaf24217bcc5644a9ce5975b2c6ad7074d0197d39a73ab648d6cb3a9ced87f`.
- `urn:choreoform:contracts:core-profile:0.8.0` / `sha256:edc8195ba0c31536ab2cdc95089e8aecc3ff402489639b4435277f0a38e7b603`.
- `urn:choreoform:contracts:core-profile:0.7.0` / `sha256:e286e54441b3d785ae823eef359c12d64fb8405829c70709227282e5af62f54b`.
- `urn:choreoform:contracts:core-profile:0.6.0` / `sha256:27bce3fde9a0393abb6bf760a8588af9268de65069b73f4cb946144f5d30cb85`.
- `urn:choreoform:contracts:core-profile:0.5.0` / `sha256:50007ac05e5e71df91bb9997a40a06c7a8993df04b1a673421e4a987014299d7`.
- `urn:choreoform:contracts:core-profile:0.4.0` / `sha256:0a80458ee083a987da47f90e5a1ef2cb107a0380b8736f0d734b345a4eec24b3`.
- `urn:choreoform:contracts:core-profile:0.3.0` / `sha256:30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3`.
- `urn:choreoform:contracts:core-profile:0.2.0` / `sha256:5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623`.
- `urn:choreoform:contracts:core-profile:0.1.0` / `sha256:26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58`.

Also require `urn:choreoform:interference-profile:provider-regions:0.1.0` /
`sha256:0b10eb4dbb19caace45b93f35dec44aa5f26458d631393932caddb5589210250`, even
when every selection is null. Use existing IR0.4 with uniform exact selected core
aliases. Unknown/mixed aliases, wrong pins, missing bases or unsupported wire
versions refuse. C12 and every older contract/source/schema/report byte meaning
stay fixed. This is raw semantic selection, not new source or execution admission.

## Closed P3 capability-contract wrapper

Only under this core, the capability-contract policy dialect body is exactly:

`{kind:"capability",reads:Set<ID>,provider:Ref,interference:null|Ref}`.

All common policy scope/dialect/reads/protection/visibility/provider rules remain
mandatory. `interference` is required, never silently defaulted or inferred. Null
supplies no external interference premise. A non-null Ref selects one exact
standalone descriptor whose provider Ref must exactly equal this wrapper's
provider, including revision. Validate the complete descriptor/namespace/authority /
provider/type/nominal/profile resource graph through the standalone profile; no
old artifact is mutated or reinterpreted. Descriptor providerProfile fixes C12's
unchanged P3/V1/V2 interpretation; it does not select a second definition dialect.
No new field is added to P3 provider-contract0.1 resource bytes or other policies.

Every capability-contract wrapper in the whole definition, including unused ones,
must select the same null or the same exact descriptor Ref for the same exact
provider Ref. Mixed null/non-null or different descriptor revisions/IDs refuse;
there is no implicit propagation, arbitrary first choice or footprint merging.
Different provider revisions remain distinct bindings. A registry may store many
versions, but only explicit consistent wrapper selections have semantic meaning.
Each wrapper still validates the provider's protection references in its lexical
scope. Sharing immutable resource metadata across scopes grants no private data,
current actor authority, cross-scope expression reads or actual activity instance.

## Semantic revision and dependency closure

The wrapper field is in the canonical semantic definition body. Selecting or
changing a descriptor Ref changes its JCS bytes and definition revision. Descriptor
bytes commit to exact profile/provider/namespace Refs; namespace bytes commit to
exact authority/profile Refs; provider types commit to nominal resource Refs.
Resolve and verify every supported typed dependency with existing bounded resource
readers. Missing, substituted, malformed, unknown or unsupported graph nodes refuse.
Rebinding a dependency needs newly pinned enclosing resources and a new definition
revision, never mutation behind an old pin. Follow only contract-defined Ref slots,
not arbitrary ref-looking data literals, text links, network content or host code.
Opaque authority bytes receive integrity checking only; their actual trust/support /
physical non-alias meaning remains an explicit host prerequisite.

Static evidence may expose per-wrapper policy/scope/descriptor metadata tied to
the checked definition revision, including unused selections. Such a declaration
is not an actual call, accepted request, runtime footprint, canonical mapping or
independence certificate. Keep original provider/protection/provenance labels and
all underlying resource obligations. Actual selectors still start from accepted
immutable original requests; a type-checked present-Option path does not prove
presence. All actual occurrence/logical-effect/attempt/input-revision/cohort /
capture/compensation correlation, surviving uncertainty/obligations, current
receiver authority and atomic transitive ownership/settlement requirements persist.

## No concurrency extension or live authority

This core supplies explicit metadata binding only. Existing ordinary concurrency,
local storage, dataflow, invalidation, protection, timing, exceptional/owned work,
reconciliation and all inherited conditional proofs remain unchanged. In particular,
a complete effectful fork must still refuse parallel-effects-unproved even with
finite selected descriptors. Neither descriptor coherence, distinct sample keys,
guarded true, compensation nor idempotency establishes generic external separation.
No canonicalizer, live request evaluator, host callbacks, runtime locks, dispatch,
accepted event, execution/independence Boolean or authority grant is introduced.

A separately justified generic proof must address all admissible overlapping
original requests, including revised cohorts and earlier surviving work. If honest
provider claims or proof are unavailable, preserve refusal and finish independent
Phase 1 work. Do not serialize the representative graph or invent external evidence
to claim closure. Actual provider/canonical authority/host truth and support remain
independent prerequisites; runtime conflict enforcement is not selected here.

## Bounded implementation and compatibility

Cache core selection once per definition; reuse immutable metadata caches and the
existing cumulative semantic-work budget, never reset per wrapper or dependency.
Charge each selection/consistency lookup/use and use indexed exact-provider pins.
Borrow type paths and share provider/namespace structures; do not clone whole
provider types per wrapper/selector. Apply existing registry/type/depth/work/report
bounds and standalone field/footprint/key/path limits. Failed/inconsistent parents
publish no successful Report. Error codes reveal no payload/authority text or
selected request values. No old public source diagnostic or report serialization
changes. New raw metadata getters remain non-executable declaration evidence.

Readable/native source selection requires its own explicit source/companion version,
managed near-English bindings and source/raw positive/negative equivalence evidence.
Source0.16/companion0.15 remain C12, including future-version/unknown-field refusal.
Old IR0.4 definitions retain their old exact dialect meanings. No current or
historical benchmark/generator/resource/report bytes are adopted or repinned here.
Complete validation/benchmarks/near-English/protected accessible text-visual and
actual user/host/browser/independent evidence remain open where unproved. No Phase 2.
