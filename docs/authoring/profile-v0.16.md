<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Candidate A authoring profile 0.16.0

**Status:** Accepted under [ADR-0071](../decisions/0071-compensation-identity-authoring.md)
and owner delegation; effective on parent/main integration.

Inherit [source 0.15](profile-v0.15.md) at raw-contract #85 head
`b40d42454d55d0165a7e3a788b31f96756b324a3` except this explicit pair/core and
purpose-only identity rule. Header `Choreoform authoring "0.16.0".` pairs with
companion 0.15/profile 0.16 and [closed schema](../../schemas/authoring/bindings-0.15.schema.json).
The [grammar](candidate-a-v0.16.ebnf) has unchanged productions. Select exact
[core 0.12](../dialects/core-compensation-identity-v0.12.md) / IR 0.4, semantics and
all eleven exact inherited cores. Every resource/profile is still explicitly verified.

Only `Compensates` has the new identity rule. `none` is null; an unqualified name
uses ordinary lexical lookup. A qualified name spells the **entire** scope path
through this definition's root, for example
`Compensates "Create payable" in "Root" in "Main".`. It resolves directly in that
scope, independently of visibility or a nearer shadowing caption. No ancestor
fallback inside a qualified target, caption scan, partial/foreign path or Unicode
normalization occurs. Every path component has the exact expected declaration kind;
the bound target must be a capability activity after nodes are bound. Invalid
targets refuse at the original reference's UTF-8 byte span before candidate formation.

All other references stay lexical, including private data/results/outcomes/policies /
actors/capabilities/control. The separate actual-use wait-endpoint exception stays
unchanged. Core 0.11's immediate incoming-source finite owned-work proof can reject a
statically resolved target. Static identity grants no eligible prior effect/occurrence,
temporal precedence, values, current authority, new command key, dispatch or settlement.
Own effect class remains independent of compensation purpose. All live host gates
inherited from core 0.12 remain required; compensation never implies restoration.

This known program inherits 131,072 tokens from source 0.15. Source 0.5–0.14 retains
16,384; legacy/fragments 4,096. Preserve 1 MiB source/companion/output, depth 64,
1,000,000 parser work, 100,000 lowering/dependency work and semantic/shared/report
bounds. Full scope paths are bounded by existing depth; each new target-kind check
shares the existing lowering dependency budget. Unknown versions/pairs/pins refuse.
Formatting/reparsing preserves the exact candidate document/revision. Source-package
integrity frames the new profile separately. Old profiles keep their behavior/staging.

The complete temporary RP-03 promotion recognizes all 77,135 tokens in 832,138
native visits and passes the old private compensation reference, then refuses
`symbol-mismatch at 0..0`. No full candidate or semantic report exists. Python
recognition still reaches its unchanged state limit. All committed benchmark bytes
stay fixed. Small valid fixtures pass all eleven checking/advisory views, all
`executable:false`; this is not full benchmark admission or adoption.

Forty scenarios remain Partial, G1–G4 conditional and parent #46 incomplete. Full
validator/full/optional RP-01/03/08, near-English reusable wording/managed identities /
protected accessible editing/export/text-visual cycles and actual users/hosts /
browsers/independent/phase evidence remain open. No execution or Phase 2 follows.
