<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Inactive external-interference profile, draft 1

**Identity:** `urn:choreoform:drafts:external-interference:0.1.0-draft.1`<br>
**Status:** inactive design, [ADR-0079](../../decisions/0079-external-interference-draft.md)

This document's exact UTF-8 bytes define the draft profile revision. It is not a
registered core dialect or accepted execution contract. Current C12, P3, source
0.16 and IR0.4 do not consume it. `0.1.0-draft.1` is an exact version, not a range.
Draft changes require another version and new pins; promotion requires a signed
integration plan and new normative artifacts. Schema validity alone is not proof.

## Closed resources

The [descriptor schema](../../../schemas/drafts/external-interference.schema.json)
and [namespace schema](../../../schemas/drafts/external-region-namespace.schema.json)
define exact JSON shapes. Resources use strict bounded UTF-8 JSON, duplicate-key
rejection and exact stored-byte SHA-256 Refs `{id,revision}`. Identity is ID plus
revision, never caption. Unknown fields/versions/profiles are refused. No file,
network or ambient registry lookup is part of the contract.

Descriptor fields are exactly:

| Field | Meaning |
| --- | --- |
| `kind`, `version`, `id` | `external-interference`, `0.1.0-draft.1`, resource identity |
| `profile` | Exact Ref to this draft profile |
| `providerProfile` | Exact C12 Ref; only the existing P3 provider-contract0.1 form is targeted |
| `provider` | Exact provider resource Ref, including unchanged input/output/protection/observations/effect class |
| `coverage` | Literal `exhaustive`: a provider behavior claim, requiring separate supported registration/truth evidence |
| `activity` | Finite footprint or literal `opaque` |
| `terminal` | Finite footprint or `opaque`, for the provider's correlated terminal-observation channel |

A finite footprint is exactly `{reads:[Region],writes:[Region]}`. A Region is
`{namespace:Ref,key:[Selector]}`. Lists have at most 16 reads and 16 writes; each
key has 1..4 components. Duplicated regions may be collapsed after resolution;
they never add authority. A non-read-only activity needs at least one write;
read-only activities and all terminal footprints must have no writes. An effectful
or otherwise unknown observation channel is opaque. Every provider observation
path, indirect/shared service access, quota/account/index/stock invariant and
effect that can interfere must be covered, not only its business result cell.
If honest exhaustive coverage is unavailable, use `opaque` or supply no descriptor.
No accepted provider class, output label or nominal invariant changes here.

A namespace resource has exactly `{kind:"external-region-namespace",
version:"0.1.0-draft.1",id,profile,authority:Ref,keyTypes:[K]}`. `profile` is this
exact draft Ref, `K` is `text` or `integer`, and arity is 1..4. The authority Ref
pins a canonicalization trust/behavior contract, not an identity credential. A
host must independently register and support that exact authority and every used
namespace revision. Namespace resources carry no author-defined normalizer code.
The same ID with a different revision is a different binding; unsupported aliases
and independently registered authorities do not establish physical separation.

## Typed selectors and retained inputs

A Selector is `{path:[Step],type:K}`. The path has 0..8 steps. Only these exist:
`{op:"field",name:Name}`, `{op:"present"}`, `{op:"representation"}`. They mean
exact record lookup, present Option unwrap and explicit nominal representation
selection under V1/V2. A scalar request can use an empty path. Selected leaf type
must exactly equal K and its namespace component type. Integer values are canonical
signed-64 strings; text has exact Unicode spelling, without implicit normalization.
Absent Options, missing fields, unprovided/unsupported nominal resources or
invariants, unsupported leaves/steps and malformed values leave the proof unproved.
There are no constants, E1 expressions, cell/fact/current-input reads, concatenation,
defaults, coercions, list iteration or user code. Selector type-checking occurs
even when another footprint makes comparison trivially conflict-free.

All paths start at the **immutable accepted original request** of an actual
activity occurrence. Type/resource/profile pins and original protection labels
remain attached. A request identity includes definition revision, scope/activity
occurrence, logical effect, input revision vector and digest of the typed accepted
request. Attempt identity is separate. Hashes do not prove valid input, freshness,
authority or confidentiality. A retry must retain the same request, revision,
provider/descriptor and logical key; new inputs mean new logical work. Compensation
is new work with its own request/footprint linked to its eligible original effect.

A terminal channel selects from that original request, including after capture
or handoff. It cannot select from the observed reply or current replacement data.
Correlated terminal observations do not create new business effects. They still
have declared reads, result protection, actual source/correlation and current
receiver-authority requirements. An unrelated read-only query is a new activity.

## Canonical regions and finite judgment

A supported common authority maps every used `(namespace Ref, typed key tuple)`
to a canonical physical `(domain, typed key tuple)`. This mapping must exhaustively
cover aliases across providers/namespaces and shared infrastructure, be stable for
the retained work's lifetime, and certify distinct canonical regions non-aliasing.
Canonical domains come from that common authority, not the author. Identity mapping
is permitted only with its registered injectivity/non-alias evidence. Different
namespace IDs, raw strings, resource digests or authorities are never sufficient.
If any key lacks a supported mapping, the judgment is unproved. Do not combine
separate authorities by assuming their domains are disjoint. Model tables are
finite assumptions for examples, not an implementation of that real trust service.

For resolved footprints A and B, overlap means exact canonical-region equality.
The conditional independence rule is:

`W(A) ∩ (R(B) ∪ W(B)) = ∅` and `W(B) ∩ R(A) = ∅`.

Read/read overlap is allowed. Return `disjoint-in-model`, `overlap-in-model` or
`unproved`; none is an executable admission or general domain-invariant theorem.
Compare the actual two original snapshots even across revisions/cohorts; never
reuse an inequality established only within one revision. A generic definition
requires a static proof for **all** admissible overlapping occurrences, including
earlier surviving work. Sample snapshots are not that proof. If unresolved, retain
the existing refusal. Storage/control/protection/P6 lifetime/irreversible invariants
and all other stage gates remain independently required.

## Runtime boundary and uncertainty

This draft selects static finite region reasoning. It does **not** select runtime
locking as an alternative route to admission. A future runtime option requires its
own contract/implementation/evidence: shared atomic compare-and-admit across every
relevant actor/provider, fencing against stale holders, durable claims before
dispatch, correlation, crash recovery and no gap during authorized atomic handoff.
Independent actors outside such enforcement remain an unresolved premise.

Requested, acknowledged, timed-out, ambiguous or unknown attempts retain possible
interference. A cancellation request, elapsed cutoff, lease expiry or logical
success does not prove every physical attempt quiescent. Actual correlated provider
evidence must prove no further conflicting access, covering late/duplicate attempts,
before an operational exclusion can be released. Reconciliation must remain possible
under the exact original request and its declared observation footprint; neither
a cycle of claims nor ownership expiry may erase uncertainty. Atomic ownership
handoff retains every transitive pending attempt, capture, observation and business
obligation, and validates current receiver support/protection/authority. Unsupported
handoff refuses; no descriptor creates a receiver or settles an obligation.

Operational exclusion is distinct from durable reservation/payment/compensation
obligations. Quiescence can end an operation's access interval without undoing its
lasting effect. Existing P3/P4/P6 settlement/closure obligations persist. A pin,
model comparison, nominal predicate or provider result alone cannot dispose of them.

## Bounds, privacy and prototype

Input and referenced resources total at most 1 MiB per model comparison; JSON depth
64, work 100000 shared units, each Ref/name/text/key component at most 256 UTF-8
bytes, at most 64 canonical table entries and 32 namespaces. A result is at most
1024 bytes and contains only fixed status/remaining labels, `executable:false`.
No partial success/output on budget exhaustion; it returns fixed `unproved`.
Diagnostics never echo requests/keys/domains/hashes or library exception text.

The [pure Python prototype](../../../tools/proofs/external_interference.py) checks
this selector/shape/alias/conflict subset over explicitly synthetic model inputs.
It does not implement full V1/V2 invariant admission: named representation access
requires exact resource bytes but their invariants are **assumed already validated**.
Its result also assumes valid P3 resources, truthful coverage and a registered
canonicalization authority. These assumptions are always reported; they are not
caller-supplied success flags or a production trust API. There is no executable
monitor, actor authority, host/browser run or actual process observation here.

## Production integration boundary

Separate signed scopes must (1) publish a new versioned normative profile/core
artifact, complete closed Rust resource/namespace readers and Ref/type validation,
(2) make explicit descriptor selection semantic definition metadata with exact
dependency closure/revision and bounded raw/native/Wasm refusals, (3) prove finite
generic original-request region separation through accepted source construction
and nominal invariants, or preserve `parallel-effects-unproved`, and (4) add managed
near-English bindings with matching source/raw positive/negative evidence. Conflicting
descriptors for one provider cannot be silently selected or merged. Omission keeps
old conservative behavior; C12 bytes/meaning remain fixed. A new contract must not
retroactively reinterpret old resources. Actual provider/authority/host registration,
truth, browser execution and user/independent evaluation are separate evidence.
Technical pins belong in managed bindings, not ordinary author-facing sentences.
