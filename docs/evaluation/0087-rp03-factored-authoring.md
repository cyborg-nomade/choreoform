<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-03 factored authoring adoption evidence

**Date:** 2026-11-13<br>
**Authority:** [ADR-0087](../decisions/0087-rp03-factored-authoring.md), signed scope `d2e301f`<br>
**Exact archive before adoption code:** `b8d73a750e251fa4b21755aeea71e32300d78a86`

## Byte preservation and actual production path

All 126 then-current non-history RP-03 files / 4057203 bytes were copied exactly to
[the pre-factoring archive](../benchmarks/rp03/history/source-0.16-before-factoring/ARCHIVE.md).
Its relative-path/NUL/SHA-256/LF inventory digest is
`53e91aa95aad3f0b66b0cb8a951c69db4d5fbcc5f8494cf901ed6586730167ec`.
An additional provenance note is separately pinned. A closed fixture loader checks
all 127 names, inventory and note bytes before replay; no earlier archive is edited.
All 465 other previously tracked benchmark files were verified unchanged before
adoption. The 113 RP-01 files remain exact; their equivalent relative inventory
SHA-256 is `62ebf5abf58d73a682a3f69271ba00521065539451195f0324d357c16f677c9b`.

The real finite RP-03 fixture producer opts into source 0.18 / companion 0.17 on C12 /
IR 0.4. It renders explicit **Order information** protection and **Order instruction**
type descriptions with managed IDs `order_data_protection` and
`order_instruction_type`; per-data purpose identities remain fixed. It refuses a
requested envelope that differs from its complete frozen definition, and freezes
the type description before later caller mutation. Historical/default fixture modes
stay explicit and inline. No production Python compiler, source text substitution,
general editor/exporter, new semantic type or runtime is introduced. Native Rust
parses, resolves, lowers, checks and formats the resulting complete package.

## Actual native observations

| Property | Pre-factoring source 0.16 | Adopted source 0.18 |
| --- | ---: | ---: |
| UTF-8 source bytes | 576759 | 410752 |
| Complete-line / native whole-program tokens | 84051 | 66783 |
| Native recognizer pattern visits | Historical capture retained | 840430 |
| Core / wire | C12 / IR 0.4 | C12 / IR 0.4 |

The actual new source is 28.78% smaller than the original. Domain names/fixture
layout produce 410752 bytes versus 412288 in the earlier test-only generic span
variant. These are measured costs, including definitions/uses; no comprehension or
actual representative-user evidence follows. Native work remains below 1M and
all source/output/token/depth/lowering/semantic quotas stay unchanged.

Actual native lowering and format/reparse preserve **byte-identical canonical IR**:
revision `sha256:a3b8c0f07cfc63d745376c3eb11b67690c98edc3e216d262c1542f00a05798aa`,
wire `sha256:6550ac457539457b6e48a452d17d95060db1a2f36fbcf74f8b3c7c1e7c11cfb4`.
Source digest is
`sha256:2e2be8154b94791d5af9e5e77c87895d205e2455af37063e66b98efbcb645695`,
canonical bindings `sha256:6e0274d9c2fff47cbf1fecdda9fd8920dd5ada6e647a457d34b6e2c0af52cfbe`,
joint package `sha256:21d5c4c815ea3139e10f35e47cd6509f5a1c0761800881a96810738188ad24a0`.

Only four previously current benchmark files differ: source, companion, manifest
and README. All 122 other originals, including the entire candidate, five generated
checking/advisory reports, every resource/guard/receipt/launch value and license,
compare byte-identically. The producer actually regenerated/checks those reports;
old observations are not relabeled as new measurements. Four checking plus advisory
pass; six later stages still refuse `parallel-effects-unproved` at
`/body/nodes/start_acceptance`, now actual source bytes 72263..72450. Old spans remain
in the exact archived manifest. There is no new external-independence proof.

The 25 logical capability/effect/wait/service instances, 26 invocations, 7 split
children, 312 required / 0 missing invalidation obligations, 849 historical origins,
45 uncovered / 34 analyzed nodes, 22 receipt cases and 8 guard cases retain meaning.
All report outputs remain non-executable declarations and all scenarios Partial.

## Historical consumers, negative controls and validation

Old feature-equivalence tests explicitly read the signed source 0.16 archive, so
their original byte/revision assertions still test their original baseline. The
immutable ADR-0083 native metric capture remains exact: only its then-current
fixture-path label is normalized during comparison to the archived location; every
measurement byte stays checked. Resolved protection identity also reads the matching
archived candidate. The original invalidation-adoption proof now reads its archived
post-adoption package, preserving its strict unchanged-companion/resource assertion;
new source-only identities are not silently removed from a current comparison.

Current consumers actually parse/check the new files. Native whole-program tests
measure 66783 tokens / 840430 visits, compare source/bindings/package integrity,
exact current/archive candidate bytes and formatting, all first four source/raw
checks, strict/advisory coverage and unchanged concurrency refusal. Four additional
Python groups verify exact archived/other-package bytes and allowed changes, real
format/historical lowering equality, active new-definition mutations and the finite
producer's complete-envelope/type-snapshot safeguards. Mutations require exactly
one real target and prove empty-stdout refusal or changed revision followed by
static refusal; missing substrings cannot become passing no-ops.

Full final-head pinned Rust/compile-fail/native/Wasm/Python checks, separate
completed-diff automated review, all review/thread/CodeRabbit comments, successful
exact-head CI and signed merge remain mandatory before integration. A skipped bot
supplies no review evidence and no human/independent self-approval is claimed.

## Unresolved Phase 1 gates

The package is still a fictional draft with static/conditional declaration evidence.
Actual current values/authority, original accepted snapshot storage, truthful
providers, physical aliases/canonicalization and revised-surviving overlap proof,
receiver/handoff/settlement and host execution remain unproved. RP-01 adoption,
C13 source selection, parameterized receipt/work/predicate reuse, full RP-08,
managed protected accessible inspection/editing/export/text-visual and actual
representative users/browsers/independent evidence stay open. G1–G4 remain
conditional, parent #46 draft and Phase 1 incomplete; no Phase 2/new phase chat.
