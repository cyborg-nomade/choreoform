<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Explicit wait-capture authoring and native disclosure evidence

**Date:** 2026-11-07<br>
**Plan:** [frozen dependency scope](0046-wait-capture-authoring-plan.md), `b29db0b`<br>
**Decision:** [ADR-0046](../decisions/0046-wait-capture-authoring.md)

Source 0.8 / companion 0.7 explicitly selects exact core 0.5 and Definition IR 0.2.
The versioned EBNF differs only in its wait production, allowing readable exact
terminal-source/origin/result references. The new companion adds only its closed
terminal-source resource kind plus explicit header/core pins. Every earlier EBNF,
schema, core artifact and source/core/wire/hash domain remains fixed. Bare new-wire
waits have capture:null; old-wire output still omits the field. Genuinely unknown
source sentinel tests now use 0.9; fragments and unknown headers keep legacy bounds.

Lowering resolves resource/node/data roles in normal lexical scope, verifies actual
metadata/profile/provider/type bytes, derives singleton writes and includes source
provider protection references in actual wait reads. The new metadata walk and
policy-read copies charge the same bounded dependency allowance before cloning.
Existing management/policy/control/destination dependencies and finite metadata
closure remain intact. Libraries require all exact core bases; the CLI embeds
known semantics/core bytes only. Declared source equality does not authenticate a
normalizer, activate an origin or prove a live observation/correlation/current grant.

All ten checking commands and advisory invalidation requirements disclose nonempty
`declaredWaitCaptures` directly from their retained typed static report. Every row
includes exact revision/site/source/profile/provider/origin/result and mandatory
host requirements; unvisited sites remain declarations. No second validation or
accepted payload/value/fault/owner/execution handle is produced. Empty captures
omit the key and preserve earlier report structure. Row accumulation and final
combined JSON both bound output at 1 MiB before stdout.

Nine Rust source integration groups check readable capture followed by a branch
through all ten partial stages; exact nullable wire/core selection and retained
composed reports; format/reparse/coordinated alias identity; all old profiles,
wrong pairs/pins/kinds and genuinely unknown/fragment quotas; every missing/corrupt
inherited/source/profile/provider byte; normal visibility, wrong origins and types;
unvisited declaration versus activation; source-only access reads and stale raw
read rejection; and type-equivalent other-provider refusal. One native internal
group verifies final combined output refusal even when its capture row and base
JSON individually fit. Fixtures are static declarations, never observations.

Six independent Python groups use explicit EBNF/schema/hash models and the actual
native adapter. They check all eleven disclosure views and exact revision/pins,
format/alias/package digests, unchanged old wire/report structure, grammar delta /
strict pairs / missing or corrupt source/profile resources, unvisited declarations,
and source-only protection reads from an unselected false predicate branch. An
ordinary synthetic fixture passes all partial checks. A small 130-capture program
with long provider/source IDs is 58,376 bytes / 9,523 measured tokens, below the
existing quota; lowering succeeds, but every checking/advisory command refuses
`report-budget` with empty stdout. No quota increase or truncated report is used.

An actual complete RP-01 copy is explicitly promoted for this test only. Its
captured terminal local preserves real nominal payload identity and passes static
and strict invalidation, but concurrency still refuses `scope-lifetime-unproved`
at `/body/nodes/run_payment`. The checked-in source 0.7/core 0.4/IR 0.1 package,
resources, guard snapshots and reports regenerate unchanged. All forty scenarios
remain Partial; this source support does not complete late-result recovery.

Local verification passed **362 Rust tests plus six compile-fail doctests**,
format/native workspace lint, all five library Wasm lints and release portability /
frontend builds. The unchanged shared portability suite/oracle passes 97 cases,
17 JCS/hash comparisons, three original fixtures/hashes, ten node variants and nine
native CLI checks. Every required Python oracle plus exact benchmark regeneration /
historical probes passes. Wasm compilation is not real browser execution. Transient
logs: `/private/tmp/capture-authoring-{workspace,clippy,wasm,native,oracles,tests,python}.log`.
Final-head CI/comment audit/signed merge remain required.

Separate completed-diff automated self-review checked strict profile/wire/core
selection, old artifact compatibility, reference roles/visibility and qualification,
all inherited pins, derived source/destination reads and stale-read refusal,
metadata versus authority, per-use raw semantic revalidation, cumulative metadata
walk/copy limits and bounded whole-report output before stdout. Reviewed licensing,
fixture/evidence scope and all remaining phase gates. No actionable issue remains.
This is delegated agent self-review, not independent/human or individual owner
approval. Audit every PR thread and CodeRabbit summary before the signed expected-
head merge; a skipped bot review supplies no review evidence.

Next freeze owned-local-storage/lifetime proof before extending the surviving-body
subset. Conditional protection/invalidation, fault data and explicit permitted
continuation/cutoff/settlement still need proofs. General near-English/tool-managed
editing, complete validator/benchmarks, visual/protected accessible editing, actual
user/host/browser and independent evidence remain open. Parent #46 stays draft /
incomplete and Phase 2 does not start.
