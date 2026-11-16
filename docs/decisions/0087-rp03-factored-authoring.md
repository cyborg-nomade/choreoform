<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0087: Adopt factored RP-03 source with an exact pre-adoption archive

**Status:** Accepted under Project Owner delegation; effective on integration<br>
**Date:** 2026-11-13<br>
**Decider:** Codex under ADR-0018; no personal owner or independent approval

## Context and decision

Signed [scope87](../evaluation/0087-rp03-factored-authoring-plan.md) `d2e301f`
follows PR #101 merge `fe16b0e1264933a8f61b885e78c1dbf2e444e89c`, final-head
CI `43411597910` at `1cbaee542d158b3b7f5a074ade2e2d7ab04a5efe`, automated
COMMENT review `5889683049` and fresh full audit. Archive checkpoint
`b8d73a750e251fa4b21755aeea71e32300d78a86` copies all 126 then-current non-history
files exactly before adoption code. [Archive provenance](../benchmarks/rp03/history/source-0.16-before-factoring/ARCHIVE.md)
records its inventory/digest; original files and all earlier history remain immutable.

Adopt source 0.18 / companion 0.17 on unchanged C12 / IR 0.4 in the actual current
RP-03 fixture producer/package. Define complete protection as **Order information**
with stable source ID `order_data_protection` and its explicit fulfillment purpose
symbol; define the exact common request record as **Order instruction** with stable
source ID `order_instruction_type`. All semantic IDs, per-data purpose identities,
resource pins, policy/participant/capability sets and logical work occurrences stay
fixed. A changed requested protection causes the finite fixture producer to refuse,
not silently drop fields. Only exact complete instruction descriptors use the name.

This producer renders its own explicit finite fixture parameters. Native Rust still
parses, binds, lowers and checks the result; the test-only span transformation is
not deployed as an editor/compiler, and no production Python compiler is introduced.
Historical/default producer modes stay explicit and retain their old inline forms.
No general source editing, identity recovery or arbitrary IR export is claimed.

## Evidence and consequences

Actual native generation produces 410752 UTF-8 bytes / 66783 complete-line tokens,
versus 576759 / 84051 before adoption. Source integrity is
`sha256:2e2be8154b94791d5af9e5e77c87895d205e2455af37063e66b98efbcb645695`,
canonical bindings
`sha256:6e0274d9c2fff47cbf1fecdda9fd8920dd5ada6e647a457d34b6e2c0af52cfbe`,
joint package
`sha256:21d5c4c815ea3139e10f35e47cd6509f5a1c0761800881a96810738188ad24a0`.
The complete canonical IR bytes/revision remain exact:
`sha256:a3b8c0f07cfc63d745376c3eb11b67690c98edc3e216d262c1542f00a05798aa`,
wire digest `sha256:6550ac457539457b6e48a452d17d95060db1a2f36fbcf74f8b3c7c1e7c11cfb4`.

All five emitted checking/advisory report files, resource/receipt/guard artifacts
and other generated contents compare byte-identically against the archive. Only
source, companion and manifest differ in the generated finite current package;
the current README/documentation records adoption separately. Actual native
format/reparse preserves the exact candidate. Four checking views plus advisory
pass; six later views still refuse `parallel-effects-unproved` at
`/body/nodes/start_acceptance`, now measured source bytes 72263..72450. New source
spans are actual observations, not copied old positions. Native parser/token/work
and all current/historical consumer/negative-mutation checks must be verified.

Preserving the full old package costs repository space but keeps evidence replayable
without rewriting historical observations. Old factoring metrics retain their
original captured bytes/meaning; consumers must explicitly use this snapshot for
the old baseline. RP-01 stays unchanged and needs separate adoption. A shorter file
is not user accessibility, physical independence or full semantic admission.

Required final-head checks/CI, separate automated completed-diff review, all
comments/CodeRabbit audit and expected-head signed merge precede integration.
RP-03/all scenarios remain Partial, G1–G4 conditional and parent #46 draft. C13
source selection, parameterized receipt/work/predicate reuse, full RP-08, protected
accessible editing/export/text-visual, actual users/hosts/browsers/independent
review and all semantic/region/phase gates remain open; no Phase 2.
