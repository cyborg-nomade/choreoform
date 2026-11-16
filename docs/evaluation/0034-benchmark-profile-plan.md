<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Benchmark authoring profile: frozen dependency plan

**Date:** 2026-11-06<br>
**Baseline:** `def1d7e299f66aa12442181bbfc90b6443a99a07` (draft PR #46)<br>
**Authority:** ADR-0018 delegation; actual RP-01 profile refusals under plan `8304be4`.

The RP-01 whole-source draft measures 52,055 bytes / 8,945 tokens. Current source
0.4 correctly refuses at its unchanged 4,096-token guard. A separate 1,097-token
syntax/schema-valid witness correctly refuses reference-missing: a root capability
must stay visible so root data can name its permitted access, while its activity
can live in a child whose own surviving follow-up owns the wait. Existing raw IR
lexical/lifecycle relations allow that declaration shape; source 0.4's endpoint
exception only considers the wait policy's own scope closure. Reproduce both with
`tools/build_rp01_benchmark.py --check --probe-profile`; do not claim draft lowering.

Publish ADR-0034, source 0.5.0 / companion 0.4.0, retaining exact core 0.2.0/base
and unchanged 0.3 EBNF. This dependency is a dedicated stacked PR targeting draft
branch `codex/rp01-benchmark-package`; keep draft #46 unmerged until integration,
whole-package evidence and final review are complete. Delegated acceptance does
not imply an individual owner or independent review, nor public/main availability
before the parent integrates.

Only the explicit supported 0.5 program header selects up to 16,384 lexical tokens
and 1,000,000 parser work units. Keep prior profiles/fragments at 4,096/100,000;
unknown headers never select arbitrary quotas. Preserve 1 MiB input, existing
depth limits, 64-level lower/IR/resource/ledger constraints and all other budgets.
Record actual parser work/token counts through read-only Syntax metadata to
verify the measured need. If a further concrete bound fails, record evidence and
revise this plan explicitly before changing it; do not silently widen defaults.

Extend only wait observation/timer outcome binding after ordinary missing lookup:
an explicitly fully qualified descendant endpoint may resolve through its exact
scope chain. Preserve successful ordinary lookup and wrong-kind/qualification
errors. After steps, require an actual wait using this exact policy and either
the old own-scope designated direct follow-up relation or the actual declared
capability activity-use relation: capability -> effect -> this wait policy, and
each relevant activity owner's actual reconcile closure -> this direct child.
Require at least one relevant use for the new relation and reject incompatible
owners. Build one indexed policy-to-owner-use map; no per-endpoint full-node scan.
Do not expose data, expressions, arbitrary nodes or generic descendant namespaces.

Keep declaration-order staging and exact source spans. New body fields/pins are
not invented. Source 0.4 retains its narrower binding and old refusal; ordinary
lexical waits retain old behavior. Frame package integrity with 0.5.0; strict pair/
pin/schema matching and explicit resource bytes remain. Promotion of an otherwise
identical supported 0.4 package keeps semantic IR/revision because core meaning
is unchanged. Verify raw-IR support and actual new-source lowering, not just syntax.

Tests: legacy quotas/fragments/unknown versions; exact new cap/work limits and
full RP-01 parse/lower; minimal actual-use endpoint, multiple compatible/incompatible
sites, unused policy, wrong/ancestor/nested owner, forged namespace, generic data/
outcome privacy, observation/timer owner and declaration order; all source pairs,
hash/format/alias identities and no implicit resource/host support. The isolated
witness remains a declaration test, not a full benchmark result. Do not upgrade
forty-scenario status, exceptional/protection/lifetime proof, visual/editing/user
evidence or phase gates from the compiler result.

Run native/Wasm and all relevant artifact/source/stage checks at final head;
separately review, inspect all comments, sign off commits and merge expected heads.
Then return to draft #46, adopt the explicit new profile only after acceptance,
derive real canonical IR/stage/refusal evidence, resolve further genuine package
issues and complete its review before readiness/merge. No Phase 2 engine work.
