<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Provider fault-source metadata evidence

**Date:** 2026-11-06<br>
**Plan:** [frozen scope](0041-provider-fault-source-plan.md), `ffb80a7`<br>
**Decision:** [ADR-0041](../decisions/0041-provider-fault-source-metadata.md)

The new closed profile/resource pins declared observation-to-fault mappings to
an exact provider-contract 0.1.0. Profile identity:
`urn:choreoform:fault-source-profile:provider-observations:0.1.0`.<br>
Exact snapshot revision:
`sha256:f894edd8fb524b4608d1d26791298eee8436bccc9b3101af553efcd03e1179d0`.
The editorial source and new frozen artifact bytes match; earlier snapshots,
provider schemas, core/source pairs and benchmark IR/resources/reports are unchanged.

`Provider::observations()` retains the already-validated closed observation-state
table as typed immutable metadata. The new `Resources::provider_fault_source`
reader verifies exact source/profile/provider/type bytes and returns a cached Arc
of private-constructor `ProviderFaultSource` / `FaultEmission` declarations.
Definitive-no-effect resolves only rejected/cancelled/no-effect declarations;
unresolved only unknown. Succeeded/acknowledged, unknown observation names,
wrong/mixed evidence, empty/ambiguous mappings and trust/safe bits refuse.
Names use exact Unicode comparison. An unknown mapping with a safe-looking fault
name remains unresolved; spelling never proves no effect. Provider decoding keeps
its previous closed shape/state/result/refusal rules and now charges state-index
copies as well. Source caches contain no actual observations or authority.

Six Rust integration groups exercise typed/cache views and explicit remaining
requirements; sixteen mapping vectors (five metadata decodes, eleven refusals);
missing/corrupt profile/provider/type bytes; unsupported pins/fields/versions,
bad provider states/results; unknown without stable key as classification only;
Unicode NFC/NFD distinction and collection limits. One internal group verifies
shared cumulative work exhaustion, no partial source-cache insertion after failure
and charged cached lookup. A failed decode returns no partial mapping report.

Four Python groups independently check exact profile/snapshot/provider hashes,
closed vector identities/counts, provider-state/ambiguity mapping expectations,
licensing and required CI. A native source test supplies valid source/profile
metadata beside the existing package and obtains the identical timing report,
still non-executable: available bytes do not implicitly bind an existing retry.
This oracle checks declared state rules, not actual provider truth or host behavior.

Native verification passed 304 Rust tests plus six compile-fail doctests, format/
workspace lint, all five library Wasm lints, release portability/frontend Wasm
builds and native portability oracle. Logs:
`/private/tmp/fault-source-{workspace,clippy,wasm,native}.log`.
All CI Python oracles and exact current-package regeneration/historical probes
also passed (`/private/tmp/fault-source-oracles.log`). Final-head CI remains
required; the new Python oracle is in required CI.

Separate completed-diff automated self-review checked preservation of old provider
shape/result judgments, exact dependency pins, declared-state versus actual fact
boundaries, ambiguity/class/name rules, immutable cache scope, work/collection
bounds, source/profile publication and unchanged current source/IR/report behavior.
Review corrected the test registry to retain original provider/type bytes instead
of reserializing them, and explicitly asserts the vector's provider pin. No
actionable issue remains. This is delegated automated review, not independent or
individual owner/host review. Audit all comments/CodeRabbit summaries and require
passing final-head CI and expected-head signed-off merge into draft #46.

This reader accepts no observation, emits no fault and authorizes no retry. It
cannot bind itself to work/handlers merely because names or providers match.
Next specify and implement explicit versioned retry/use-site binding, exact actual
provider identity and eligible-source evidence classes while retaining P4 stable-
key/no-effect/correlation/budget/authority/cancellation/closure premises. Then
continue late-observation data/continuation, full validator and benchmark work.
Every scenario and phase gate stays open; the parent remains draft/unmerged.
