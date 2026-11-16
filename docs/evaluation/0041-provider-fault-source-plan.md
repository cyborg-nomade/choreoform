<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Provider fault-source metadata: frozen contract/reader plan

**Date:** 2026-11-06<br>
**Baseline:** `983de13c294db11bef13d14c11c5dff15d6fd9e8`, draft #46 after #53<br>
**Authority:** ADR-0018; actual arbitrary retry-name acceptance in the RP-01 audit

Use a dedicated stacked dependency PR. Define ADR-0041 and one independently
pinned provider-observation fault-source resource/profile, with a bounded typed
metadata reader. Do not reinterpret old provider/core/source bytes or silently
bind this resource to existing retry policies. Follow the earlier clock-reader /
later policy-integration separation. Actual policy/use-site binding needs its own
explicit supported core/source contract after this dependency.

Closed resource shape: `{kind:"provider-fault-source",version:"0.1.0",id:Name,
profile:Ref,provider:Ref,emissions:Map<Name,Emission>}`. Emission is exactly
`{observations:Set<Name>,evidence:"definitive-no-effect"|"unresolved"}`.
Verify exact resource/profile/provider bytes from the explicit bounded registry.
Require the known profile and provider-contract 0.1.0; arbitrary hashed bytes,
unknown kinds/versions/profiles/fields/flags and supplied trust/safety claims refuse.
Require nonempty bounded emissions and observation sets. Resolve every observation
name against the provider's actual declared observation table, not its spelling.

Retain typed provider observation-state metadata in the existing immutable provider
reader. Definitive-no-effect mappings may name only declared rejected/cancelled/
no-effect observations; unresolved mappings only unknown. Succeeded/acknowledged,
missing observations and mixed/wrong evidence classes refuse. One observation may
emit at most one named fault in this profile; unmapped observations emit none.
Exact Unicode names are not normalized or treated as substring aliases. Source
metadata is not fault acceptance, retry eligibility or an effect-settlement fact.

Expose immutable private-constructor typed source/emission/state views and explicit
remaining definition/host requirements. Preserve existing provider reader behavior,
shape/refusal codes, all source/IR/revision and current package/report evidence.
Cache only verified immutable metadata, never actual observations/authority.
Charge provider state-index/source mapping/copy work under existing cumulative
resource budgets; use the current transport/registry limits, no quota increase.
Profile normative bytes are frozen under a new digest, never self-hashed in-file.

Tests: exact pins and typed/cache views; missing/corrupt/profile/provider/type
resources; unknown fields/versions/flags/trust bits; success/acknowledgement and
unknown/no-effect misclassification; unknown names, empty/ambiguous maps; preserved
old provider negatives; work/collection bounds and no literal diagnostic leakage.
An independent Python oracle checks strict profile/snapshot/hash/shape and state-
mapping vectors, without claiming provider truth or independent host behavior.
Run native/Wasm/Python/final-head CI, separately review completed diff, audit all
comments/CodeRabbit summaries and merge only the passing exact signed-off head.

Then implement explicit definition/use-site/retry binding under its own versioned
contract and source support, requiring exact actual capability/provider identity
and preserving stable-key/no-effect/correlation/authority/cancellation/closure
premises. Continue late-observation/data/continuation and full validator work.
This metadata slice does not close C, any scenario, the parent draft or Phase 1.
