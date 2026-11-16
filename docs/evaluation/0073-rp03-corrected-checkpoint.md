<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Corrected RP-03 checkpoint and the next template refusal

**Date:** 2026-11-11<br>
**Baseline:** #87 head `13df5101bd8ba5dc94cdb0ec3ec2fbc84dcd6ee0`, merged
as `946a7e682a9e7784362f1554b0f5798e6996a9cc` before changes/probes<br>
**Frozen plan:** `2b8cbdf`, [scope](0073-rp03-corrected-checkpoint-plan.md)<br>
**Decision:** [ADR-0073](../decisions/0073-rp03-corrected-checkpoint.md)

## Exact controlled changes

The current full source changes only its header from 0.14 to 0.16 and one finish
literal from `authorization recorded` to the already declared `authorization-recorded`.
The companion changes only profile 0.14→0.16, version 0.13→0.15 and selected C11→C12.
Existing source 0.16/companion 0.15/C12/IR 0.4 and all eleven bases are used; no schema,
wire, contract, quota, resolver, symbol, privacy, ownership or host permission changes.
All resources, receipt/guard inputs and rules, scenario A–D/optional scope, wire IDs,
protected labels and host requirements stay fixed. All 113 RP-01 files keep their
prior aggregate digest. The original defect was exact-name inconsistency, not a
reason to normalize punctuation or change business semantics.

The generator now has explicit current/historical selection. Shared RP-01 helpers
are unchanged. Historical generation uses the exact old pair/core/header and old
finish literal; a bounded old source-format witness explicitly requests that old
selection. Current generation records the real later refusal and emits no phantom
candidate or semantic report. Full source/companion/scenario structure stays intact.

## Actual whole-case result

| Evidence | Current result |
| --- | --- |
| Full source | 505,980 UTF-8 bytes; all 77,135 tokens recognized |
| Native parser work | 832,138 actual SDK pattern visits, under unchanged 1,000,000 |
| Pair/core/wire | source 0.16 / companion 0.15 / C12 / intended IR 0.4 |
| Lowering | `child-template at 0..0`, exit 1, empty stdout |
| Candidate/canonical IR/revision | None |
| Full semantic checking/advisory stages | Not run; no candidate exists |
| Independent Python recognition | Unchanged oracle state-limit refusal |
| Source/bindings/package integrity | Native and independent Python agree |
| Scenario status | All four A–D and all forty corpus scenarios remain Partial |

The real SDK and native CLI agree. No alternate direct-IR path, truncated order,
omitted compensation clause, weakened predicate or synthetic host value is used.
The exact current identities are:

- Source: `sha256:43b89e4f755c9a22bd548210fc21a388c5b8b9a7cf49d63579474fdc0eee2b68`.
- Bindings: `sha256:1e29afd35bc79a0ea32a377887b31b8ebef2b1b8f7ba204c845a3c1c06b7b3da`.
- Package: `sha256:f14598c4e75e1d134a65b9ef85f6a21dd9edd1cca38c7fcb75e4ee0a87a927f5`.

These are package identities, not semantic revision, admission or execution evidence.
The companion remains 229,261 pretty bytes / 166,165 JCS bytes. Complete-line lexical
measurement is labeled separately from real whole native grammar recognition;
Python's different state metric is not presented as successful full recognition.

## Exact original history and regression routing

All 113 original generated files are archived byte-for-byte under
`docs/benchmarks/rp03/history/source-0.14/`. Before publication each file was compared
independently against `git show 13df5101bd8ba5dc94cdb0ec3ec2fbc84dcd6ee0:<path>`
and the untouched pre-change copy `/private/tmp/rp03-original-source 014`.
No newly generated expectation served as the only baseline.

Sorted archive paths excluding the new contextual README are framed as UTF-8 path,
NUL, unsigned big-endian eight-byte content length and exact bytes. The 113-file
SHA-256 is `797d01f0b1a42db5c71de9bc7a054d58e24333798b23e6cde22225dc4cc707ee`.
Each historical file and license still matches the original generator behavior.
Historical resource reference paths intentionally name identical unchanged current
resource bytes; archived input-artifact relative paths resolve within the archive.

Three Rust test loaders and Python capacity/compensation/symbol regressions now
select history explicitly. The old source 0.14 quota failure at 108626..108627, old
temporary source 0.15 private reference at 60937..61023, and uncorrected source 0.16
symbol mismatch/companion-only rename witnesses remain real distinct regressions.
The 56-finish audit has one historical mismatch and zero current mismatches. No
test silently turns old refusal evidence into a corrected current source. Other
symbols/control/semantics remain unproved by that limited finish audit.

## Critique and next dependency

The one caption repair moves the full case past its genuine symbol failure to
`child-template`. The generic 0..0 location does not identify which relationship
fails; do not guess a hierarchy repair, broaden visibility, omit a child/call or
relax an accepted template rule. Next freeze a test/docs audit of the actual
creator/body/current scope/declared parent and raw/readable contract requirements.
Use that evidence to choose source restructuring versus a separately versioned
semantic dependency. Later ownership/protection/invalidation/retention/exceptional
contexts, physical/provider races and full benchmark/validator gates remain open.

This checkpoint supplies no actual reservation, shipment, capture/refund eligibility,
event deduplication, immutable storage, live role/current authority, accepted
registration/receiver/cutoff/atomic handoff/all-transitive settlement or conflicting-
evidence review. The existing pure receipt/guard evaluations remain synthetic
declaration evidence, not compiled full-source IR or accepted process traces.
Near-English factoring/reusable wording, managed identity/protected accessible
editing/export/text-visual and real representative users remain explicit Phase 1 work.

## Validation and review

Two real current SDK groups and four Python groups cover exact allowed source /
companion deltas, all original history bytes/identities/resource meanings, current
failure/package integrity, zero current direct-finish mismatches and preserved old
regressions. Local verification passed 582 Rust tests plus six compile-fail
doctests, pinned workspace/five-library Wasm lints, release portability/frontend
Wasm builds, native portability and all 69 Python workflow checks. The broad Rust
run exposed one resource test still expecting the old current-manifest token
refusal; its nominal judgments passed, and its checkpoint assertion now names the
actual current template refusal. A full workspace rerun and final native lint pass.
Separate completed-diff delegated review checked correctness/security/privacy,
performance, scope, exact history and evidence; no remaining actionable scope issue.
Required exact final-head CI/full thread/summary audit precedes signed expected-head
merge. Logs: `/private/tmp/rp03-corrected-{sdk,sdk-history,package,source-history,
symbol-history,current,workspace,verify,final-clippy}.log`. CodeRabbit skips supply no
review evidence; agent review is automated, not independent/human/personal owner
approval. Compiled Wasm is not browser execution.

Parent #46 draft, forty Partial, G1–G4/full/optional/RP-08/validator/near-English /
managed protected accessible text-visual/actual user/host/browser/independent/phase
exits stay open. Independent work remains; no external blocker, Phase 2 or new chat.
