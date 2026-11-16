<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0044: Derive exact provider terminal-observation metadata

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-07<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

The [late-observation audit](../evaluation/0044-late-observation-contract-audit.md)
finds that wait payload type declarations do not establish source/protection
provenance, and the current closed IR cannot silently gain a capture field.
Before designing retained accepted-payload values, establish exactly which
provider observations are terminal, their normalized types and their source
protection. Neither a safe-looking name nor a target cell label supplies this.
Follow the [frozen metadata plan](../evaluation/0044-terminal-observation-source-plan.md),
`141bd8b`, refined before the sharing change in `53681cb`.

## Decision

Publish the separately pinned
[provider terminal-observation source profile](../dialects/provider-terminal-observation-sources.md).
Its exact closed resource is kind/version/id/profile/provider. Verify all exact
source/profile/provider/type bytes. Derive names/states/tags from validated provider
metadata: succeeded -> confirmed with exact provider output; rejected/cancelled/
no-effect -> their terminal tags with the existing correlation record. Unknown and
acknowledged are nonterminal regardless of spelling. Derive a Variant containing
only present terminal cases; several success observations share confirmed.
Preserve nominal pins and the provider's exact declared resultProtection for every
case. Reject author-supplied type/protection/state/trust overrides.

Expose immutable private-constructor `ProviderTerminalSource` metadata with exact
pins, provider contract, terminal declarations, normalized type and source envelope.
Cache sources by exact pin with shared Arc; share one derived name/type schema per
exact profile/provider pair to avoid copying large types for source aliases. Charge
terminal-name and every nested type/key/nominal copy against the existing cumulative
resource allowance/depth guard.
Repeated cached lookup does not create another copy or fresh allowance.

## Options and consequences

| Option | Benefit | Limitation | Decision |
| --- | --- | --- | --- |
| Independent exact metadata resource and reader | Checkable declared type/source/protection premise without old-wire changes | Subsequent explicit wait-use/capture integration still required | Adopt |
| Infer source from a wait type or destination envelope | Smaller declaration | Cannot establish provider identity/provenance; risks declassification | Reject |
| Accept user-declared normalized safe/terminal flags | Easy adapter integration | Names/flags do not prove original provider state or host truth | Reject |
| Add capture and implicit ordinary resume immediately | Appears to complete C | Leaves closed-wire, timer, lifetime and cancellation semantics unresolved | Reject |

Source/provider metadata is deliberately not a definition binding, observation or
permission. Actual normalizer/provider support, validated payload/outcome, exact
original effect/attempt/occurrence/request/key/input/observation correlation,
current authority/protection, acceptance order/cutoffs, duplicate/contradiction
history and owned all-obligation settlement remain live requirements. A succeeded
business-problem output does not prove physical payment. Terminal classification
is truthful only when supported actual provider evidence is accepted.

No provider/core/source/wire/schema/package meaning changes. Current native reports
remain identical with unrelated valid new metadata supplied. No wait capture,
fault port, timer initialization, local-storage lifetime, handler result conversion,
ordinary continuation or execution handle is introduced. Next separately decide
explicit capture representation and source/protection/availability, then owned
storage and C/D continuation. All scenario/phase and actual external gates remain.

[Evidence](../evaluation/0044-terminal-observation-source.md) records exact real
nominal provider cases, twelve independent vectors, budgets and native/Wasm/oracles
plus completed-diff automated review. Acceptance is delegated, not individual owner
or independent review. Require final-head CI, all comments/CodeRabbit audit and
signed expected-head merge into draft #46, which remains incomplete/unmerged.
