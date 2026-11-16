<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Recorded evidence: representation and trust design comparison

**Date:** 2026-11-16<br>
**Authority:** Signed [scope98](0098-recorded-evidence-contract-plan.md) `d72c286`,
ADR-0097 under owner delegation.<br>
**Stage:** Preparatory comparison; exact normative publication waits for verified
PR112 integration. No new record reader/importer/output or external observation.

RP08-D requires original external identities/provenance/timestamps and only known
causal links after an alternate-channel outage. The [full critique](0097-rp08-coverage-design.md)
keeps that requirement separate from current decision/effect authority, terminal
history, protected audiences and the missing complete package. A new data artifact
must not masquerade as an authenticated event or executable process history.

## Relevant actual baseline

The existing [provider fault-history profile](../dialects/provider-fault-history.md)
is an important precedent: its nominal value can be fabricated and still well-typed.
Its limited claim judgment returns no accepted event. The
[terminal-source profile](../dialects/provider-terminal-observation-sources.md)
similarly separates exact declared types/mappings from actual source/normalizer
trust, original request/effect/attempt/observation correlation, current authority
and accountable settlement. Reuse these distinctions rather than repurposing their
closed resources or assuming any generic foreign decision is a provider fault.

The actual Rust [transport](../../crates/ir-probe-core/src/transport.rs) offers strict
bounded decoding and integer-only canonical JSON. The newer raw-JSON limit API can
reject negative zero and count values/keys without changing old IR admission.
[Resources](../../crates/semantic-check/src/resources.rs) accepts explicit bounded
local bytes and reuses a cumulative native budget. Public
[type descriptor/literal judgments](../../crates/semantic-check/src/types.rs)
validate exact nominal descriptors/values and invariants without host reads. Neither
API authenticates sources. Existing P5 clock metadata/coordinates do not make an
external wall timestamp an accepted current instance-clock observation.

A [pure authoring library](../authoring/native-reversible-v0.1.md) likewise requires
the trusted caller to authorize artifact access before invocation. The real local
prototype enforces its selected complete-artifact session boundary, but supplies
no general redacted audience projection, production identity or DLP. Do not pretend
a new SDK's input Boolean, digest or declared source name can provide those powers.

## Alternatives

| Approach | Useful behavior | Main limitation / decision |
| --- | --- | --- |
| A. Closed inert evidence artifact with explicit profile, original data and claimed relations | Preserves exact input, declared identity/context/time and inspectable partial-order claims; supports deterministic private validation without process-state effects | Select for exact publication. Requires careful stage names, supported payload rules, duplicate/conflict boundaries and later real trusted acceptance |
| B. Opaque log/blob storage alone | Retains source bytes without inventing events | Cannot itself validate identity reuse, supported payload/context, order/reference contradictions or explain source claims. Original bytes remain useful inside A, not its whole model |
| C. General executable event stream imported into live instances | Could reconstruct/accept runtime state and dispatch under a comprehensive host | Requires actual engine/version/state/cutoff/current authority/storage/accepted-history contracts absent here. Reject as this deliverable; it would cross Phase2 and hide unsupplied evidence |

A is a representation stage, not a claim of full RP08-D support. The exact contract
must state what it understands and what remains opaque. A supported literal/raw
payload profile may retain bytes as bytes with no behavioral interpretation;
unknown semantic profiles must not be silently accepted as understood. If typed
payloads are selected, use explicit supported descriptors and actual native value
judgments, retaining all nominal identity/invariants and component budgets.

## Design direction for exact publication

Keep a separately versioned artifact and an immutable native wrapper. The complete
admitted artifact contains explicit stable document/source/record identities,
original payload/evidence provenance, supported profile/type pins, claimed context,
reported time and explicitly asserted relations. It is not a definition, plan,
accepted event, authoring document or runtime-history snapshot. Wire input may not
self-assert an authorized/accepted/executable stage. Reports derive a fixed inert
stage and false execution/authentication/transition claims outside the artifact.

Identity must not depend on list order, captions, Unicode normalization or guessed
occurrence links. Compare compound source/event identities structurally. Distinguish
an equal repeated record from conflicting reuse, and original event identity from
delivery/receipt identity. Define equality exactly, including retained original raw
bytes/pins, so normalization cannot silently discard a signed or conflicting source.
A rejected candidate may be retained only by an explicitly authorized quarantine
outside the admitted wrapper; refusal produces no successful partial artifact.

Relations are source claims, even when well-formed. Define exact local/external
reference boundaries and cycle/conflict behavior. An acyclic asserted-order graph
is not proof of real causality. Incomparable records remain incomparable. Never
create an edge from timestamp, ID, canonical map order or a deterministic topological
display tie-break. Avoid emitting transitive all-pairs order or a unique chronology.
Any future trusted promotion of claims to known history needs a separate exact
acceptance interface and real premises; it is not a method of this inert reader.

Preserve reported/unknown time without reinterpreting it as a P5 clock read or
firing predicate. Exact basis/domain/coordinate and raw original timestamp rules
must be frozen. Different clock bases cannot be compared or silently normalized;
calibration/trust, timestamp correctness and historic order remain unsupplied.
A useful representation may preserve a source's claimed time while granting none
of those facts. It does not authorize a response decision at that time or now.

Source/record/context and protection-like names remain claims. Complete data can
be disclosed only to a trusted authorized complete-artifact reader. Do not output
a “public” summary by deleting a field and treating the result as the same admitted
artifact. Later audience projections must cover content, metadata, graph and timing,
with actual trusted enforcement and stale integrity bindings; redacted reconstruction
and generic declassification remain unsupported. Validation errors never echo source,
principal, evidence text, arbitrary field keys, resource contents or filesystem paths.

Bound raw bytes and encoded/canonical output independently, as well as depth/items,
identity strings, records/sources/links/resources and cumulative structural/graph/
native work. Preflight before hashing/indexing/copying/expansion. Reuse one native
resource/type budget across a whole operation; do not instantiate a fresh allowance
per record. Inherited component limits remain in force. Canonicalization is not a
transport-admission bypass, especially for duplicate keys, floats, negative zero,
unsafe integers, BOM, invalid Unicode and generated output growth.

The implementation can use explicit ID indexes and bounded graph traversal. A
stable display order has no historic acceptance meaning. Publish exact fields,
resource descriptors, bounds/cost accounting, refusal precedence/privacy, immutable
SDK/CLI/bridge/report interfaces and worked vectors before production code; this
comparison deliberately does not substitute provisional shapes for that publication.
Keep the same Rust value/resource rules rather than a JS/Python payload interpreter.

## Evidence and subsequent work

The exact publication should author a finite example with multiple sources, equal
duplicate delivery, incompatible identity reuse, an explicit order claim and
incomparable/tied/unknown reported times. All are synthetic contract data, never
captured incident records or known authenticated principals/events. Negative vectors
must distinguish transport, profile/type/pin, identity/reference/order, cumulative
bounds and stage/authorization confusion. A test-only oracle may check them after
the publication is signed; no future native result may be reported in advance.

This direction preserves all current source/core/IR/provider/native/authoring/view
contracts and965 benchmark bytes. Exact publication depends on verified112; native
implementation depends on this later signed/reviewed/integrated contract. Full
RP08 batch/result/ad hoc-work/parallel effect proof, audience views, general managed
editing, complete validator/diagnostics/conformance, three-process cycles and actual
user/independent/provider/host evidence remain Phase1 work. All40 scenarios Partial,
G1–G4 conditional. No Phase2, standardized notation or phase-completion claim.
