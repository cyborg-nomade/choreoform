<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Candidate A authoring profile 0.6.0

**Status:** Accepted under [ADR-0036](../decisions/0036-metadata-core-authoring.md)
and owner delegation; effective on parent/main integration. This is candidate
lowering and supported partial checking, not executable admission.

Inherit [source 0.5](profile-v0.5.md) at draft-parent commit
`ffe0c2cd38327003b0521a9c3a117afb4644d7ee`, except below. Keep the unchanged
[0.3 EBNF](candidate-a-v0.3.ebnf), exact known-header 16,384-token/1,000,000-parser
quotas, legacy/fragment bounds and specialized actual-use wait endpoint rules.
Use header `Choreoform authoring "0.6.0".`, companion version `0.5.0`, profile
`0.6.0` and the [closed schema](../../schemas/authoring/bindings-0.5.schema.json).

## Explicit core and complete dependency derivation

Select core 0.3 at
`urn:choreoform:contracts:core-profile:0.3.0` /
`sha256:30d9fbfe7e308dc8bb41c6c5bb67fc7b2533e2a84985d2342335e697147757e3`.
Libraries must explicitly supply its exact bytes, both inherited core 0.2/0.1
artifacts, semantics and all actual resource bytes. The native CLI embeds the
three supported core snapshots; it infers no provider, clock or host support.
Old source/companion pairs retain their exact cores. A core 0.3 pin in an older
companion or a wrong new pair refuses; no automatic migration is defined.

Only this source profile derives the versioned metadata closure from
[core 0.3](../dialects/core-metadata-v0.3.md): provider-result protection references
for capability, timeout-clock and retry-clock policies are metadata edges.
Every other policy reference remains strong; retain both when they overlap.
Use the shared bounded pure read-closure helper after resolving declarations.
Derive least complete policy reads over all edges, then use the existing creation,
data-access and node dependency derivation. Semantic revalidation independently
checks the generated payloads/reads/references; compiler output grants no trust.

Strong cycles, wrong/private/unsupported references, inherited pin failures and
budget exhaustion still refuse. Keep the dependency pass's existing 100,000-work
allowance: initial graph work plus the helper's charged work share that allowance.
No per-call extra quota or default limit change is introduced. Dense supported
syntax can therefore refuse `work-limit` without returning a truncated candidate.

## Integrity and limits

Strict pairs are 0.2/0.1, 0.3/0.2, 0.4/0.3, 0.5/0.4 and now 0.6/0.5, with each
version spelled as its full `0.N.0`. Keep existing pins, schemas and digests.
The package domain is ASCII `choreoform-authoring-package:0.6.0`, one zero byte
and the existing length framing/exact source/JCS companion bytes.

Promoting a supported acyclic 0.5 package changes its selected core pin and
therefore **changes semantic revision**, even when the remainder of a compatible
body is identical. It also changes source, bindings and package integrity.
Format/reparse and coordinated alias edits within 0.6 retain the same IR/revision
when identities and actual meaning remain unchanged. General migration, identity
editing/export and text/visual parity remain separate deliverables.

The full promoted RP-01 source with explicit coherent fictional business/clock
labels passes supported static/control/protection checks. It still refuses
private-child invalidation context, and all later composed judgments remain
unestablished. The checked-in parent package stays at its existing profile until
explicit integration. No full scenario, current access, real provider/payment,
live ownership, user study or Phase 1 gate is approved by this source version.
See [evidence](../evaluation/0036-metadata-source.md).
