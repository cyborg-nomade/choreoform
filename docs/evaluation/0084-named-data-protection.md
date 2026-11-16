<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Named data protection implementation evidence

**Date:** 2026-11-13<br>
**Authority:** [ADR-0084](../decisions/0084-named-data-protection.md), signed scope `c76073f`<br>
**Publication before compiler:** `f6dfe7e30f7cebf1e0c6fe7c157403abc5fa39d9`

## Actual native source and unchanged meaning

Rust production parsing, managed ledger resolution and bounded lowering support
source 0.17 / companion 0.16 on unchanged C12 / IR 0.4. Define an explicit complete
protection once, reference it on each data declaration and expand exactly. There
are no new semantic policies, resource pins, defaults or permissions. Definition
references bind in their declared scope; each use checks visibility and retains
its complete purpose-symbol wire set and later semantic checks. Unused definitions
also validate. Purpose literals still use each data cell's own managed captions.

The test-only [native transformation](../../crates/authoring-frontend/tests/support/named_protection.rs)
uses actual parser spans on finite fixtures. It is neither a production text
preprocessor nor an IR exporter/editor. Current files and every historical
benchmark are read without modification. No adoption/archive is claimed.

| Actual full source | Current bytes | Named variant bytes | Reduction | Comparison |
| --- | ---: | ---: | ---: | --- |
| RP-01 | 61211 | 56059 | 8.42% | Exact expanded document after explicit C12 baseline promotion |
| RP-03 | 576759 | 448421 | 22.25% | Exact current complete IR/revision |

Both variants actually parse, lower, format/reparse and pass static, control,
protection and strict invalidation checks. RP-01's current core is C11, so the
C12 baseline is explicitly lowered first; its body differs from the current
artifact only in that declared core selection. Its resulting C12 revision is
`sha256:cd0a7b6923ccc8a0afa31a27fca3aff280c3da601089ba12946f2d768fa1a829`.
RP-03 remains exactly
`sha256:a3b8c0f07cfc63d745376c3eb11b67690c98edc3e216d262c1542f00a05798aa`.
Its legitimate refusal remains `parallel-effects-unproved` at
`/body/nodes/start_acceptance`; original guard/receipt/provider meanings remain.
The measured 22.25% exceeds the frozen >=15% target but is not comprehension or
representative-user evidence. RP-08 still has no full lowerable source package.

## Identity, privacy and finite limits

Ten Rust groups cover complete whole-case equivalence; mixed inline/named uses;
local purpose literals; definition after use; explicit ancestor use/private group /
private member refusals; unused malformed/undefined/wrong-kind/duplicate-resolved
members; missing/surplus purpose inventories; explicit rename/reorder/fresh copy /
stale/tombstone inventory; every old source/kind pair and genuinely future version;
format/integrity/unknown annotations; short-source expansion/work refusal.
An inline diagnostic-order regression covers multiple simultaneous defects across
legacy/current/new profiles. Four independent Python groups check exact frozen publication hashes, schema delta,
actual grammar/closed fields, real CLI lowering/all eleven non-executable views /
format/integrity/old pair/refusals/private diagnostics and depth/output bounds.

The same immutable resolved envelope is copied only after charging its JSON-node
work and compact bytes. Definitions plus uses share an expansion byte ceiling of
1 MiB; record insertion has a separate cumulative 1 MiB check without allocating
a serialized buffer. Existing final wire/source/companion 1 MiB, depth64, tokens
131072, parser1M, dependency/lowering100k and semantic/shared bounds stay fixed.
Purpose-symbol lookup uses the ledger's ordered owner/role index, avoiding a scan
of every symbol for every use. Dependency analysis starts with expansion work
already consumed. Short (<100000-byte) sources independently exceed cumulative
byte and work bounds and fail without a candidate. CLI failures emit no partial
stdout or source-literal echo. No permission or live access truth is cached.

Required full final-head native/Wasm/Python checks, separate completed-diff
automated review, complete comments/CodeRabbit audit and exact-head signed merge
remain mandatory before integration. A skipped bot supplies no evidence. Accepted
publication hashes are checked independently; existing accepted artifacts and
benchmark/history/generator/report bytes remain unchanged.

## Still open in Phase 1

Benchmark adoption/archive needs a separate signed scope. Type names, hygienic
service/predicate reuse and C13 source selection remain separate work. General
safe editing/export, protected accessible definition inspection/text-visual cycles,
complete RP-08, semantic exit obligations, physical alias/canonicalization/original
snapshot/revised-surviving-work proof and actual users/hosts/browsers/independent
review remain open. All forty scenarios remain Partial and G1–G4 conditional;
parent #46 stays draft. No execution, full-language/phase completion or Phase 2.
