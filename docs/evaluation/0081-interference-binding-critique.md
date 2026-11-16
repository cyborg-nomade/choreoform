<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Semantic interference selection: critique before contract

**Date:** 2026-11-12<br>
**Scope:** signed `aa4d58f`, [plan](0081-interference-definition-binding-plan.md)<br>
**Dependency met:** #95 signed merge `78f5d706756642932c57c33672903a7b9ccdcacc`,
CI `43233888435`, automated COMMENT review/fresh full audit `5875818566`

The actual standalone Rust reader validates the declaration graph. A supplied
resource remains unused unless a definition commits to it. Current capability
contract policies are closed dialect payloads `{kind,reads,provider}`; the IR0.4
outer body is independently closed. Selection must change semantic identity and
cannot be an annotation, filename convention or an implicit registry search.

| Binding option | Benefits | Costs and risks |
| --- | --- | --- |
| New definition-level resource-selection manifest | Central list; explicit shared-provider choices | Changes the closed wire body and requires a new IR version, migration and structural tooling; duplicates provider links already in policies |
| Nullable descriptor Ref in capability-contract policy under a new core | Keeps existing IR dialect-body extension boundary; explicit beside exact provider; semantic JCS revision already includes it | Requires a new closed core version and consistent selection across duplicate wrappers; absent fields cannot silently default |
| Effect-policy slot | Close to local effect metadata | Effect policies can be shared; weakens exact provider association and encourages inconsistent bindings for one provider |
| Supplied resource or annotation alone | Minimal syntax | Not semantic identity/selection; ambiguous versions and no exact dependency commitment; reject |

Prefer the capability-contract wrapper under core0.13.0 with a mandatory
`interference:null|Ref`. Null means no premise. All wrappers of the same exact
provider must agree on null or the same exact descriptor Ref, including unused
declarations. Mixing null and a Ref must refuse rather than implicitly applying
one wrapper's choice elsewhere. Different provider revisions remain different
bindings. A selected descriptor must name the exact wrapper provider, not merely
the same ID. Each wrapper still validates provider protection in its lexical
scope; globally shared immutable metadata does not grant private data access.

The root Ref commits to exact descriptor bytes; their typed Ref slots commit to
provider/profile/namespace/authority/nominal dependencies. Validate those exact
supplied dependencies through existing readers, not arbitrary JSON/ref-looking
literal scanning or text-link/network discovery. Corrupt or missing dependencies
refuse. Rebinding any dependency needs a new descriptor pin and definition revision.
The new core itself requires all twelve exact inherited core artifacts and the
standalone profile. C12 and source0.16 remain unchanged and refuse the new field.

Return declared per-wrapper metadata tied to the checked definition revision.
The raw SDK Report can expose an additional immutable getter without changing
existing source report serialization. Do not turn resource coherence into a
concurrency result: even complete new-core effectful fixtures must retain the
existing parallel-effects-unproved refusal. No live request/canonicalizer/host API
or generic symbolic theorem is selected here. Source adoption requires a new
managed near-English version; ordinary authors do not maintain technical hashes.

The later proof critique must consider actual original-request construction,
aliases, revised cohorts and earlier surviving work. Distinct sample keys or one
revision's inequality cannot establish that theorem. Legitimate providers may
not support exhaustive separation. Preserve that refusal, finish independent
Phase 1 work and retain real external evidence gates rather than inventing proof.
