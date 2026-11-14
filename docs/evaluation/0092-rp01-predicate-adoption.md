<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Current RP-01 typed pure receipt-test adoption

**Date:** 2026-11-14<br>
**Status:** implementation locally verified; final-head CI, separate review and integration pending.<br>
**Authority:** [ADR-0018](../decisions/0018-phase-review-delegation.md), signed
[scope92](0092-rp01-predicate-adoption-plan.md)9fd9a6b and
[ADR-0092](../decisions/0092-rp01-pure-test-adoption.md); no personal owner or independent review.

PR #106 integrated asc34b9403b4dab89fa050ce81603463f7b73193a8 after successful
CI43578324862 ate6bbeaff096afc93b23edb5f49b9f6ec506f861c, separate automated
COMMENT review5903318099 and a fresh full-body/thread/head audit. The CodeRabbit
skip supplied no review evidence. All constituent commits/merge carry sign-off.

Archive23507904f88b148fcb8db57815547d37f464a1a2 precedes producer/current-consumer
changes. All59 exact current files total641730 bytes, with original README and a
separate new note. [Inventory](0092-before-predicates-inventory.json), full prior173
RP-01 files, all732 RP-03 files and905-file combined epoch stay strictly pinned.
The new archive adds60 files; current working counts are233 RP-01 /965 combined.

The finite model producer reproduces the complete previously verified experiment
exactly: **54172bytes /9145 native tokens /113796 parser visits**, from55453 /
9455 /114301 (2.31% fewer source bytes). Definitions/types/uses are included;
parser visits are not elapsed performance or user-readability results. Its
companion value matches the experiment. Legacy mode reproduces all59 generated
source18 files, including the archived README, raw-byte exactly.

Implementation reads no frozen source fixture as its program implementation. It
uses the actual model's complete payment predicate and declared input descriptors,
checks their canonical boundaries recorded in ADR-0092, verifies exact reconstruction
of both original use sites and prints a closed pure subset. Read captions use a
direct identity map; literal values/types stay opaque. New source identities are
reserved across declarations/symbols/generated/retired/definition IDs. Every caller
principal/current-policy/approval/capture/readonly-original-request/tag/fallback
guard stays exact. Native Rust supplies actual source/package/type/IR proof.

Only the four [current-file digests](0092-rp01-adoption-digests.json) change: source,
bindings, manifest and README. Other55 current files, complete IR/revision/wire,
all11 reports, immutable resources/inputs/receipts/guards and existing guard
candidate pins remain byte-identical. There is no second core promotion.

| Pin | Exact value |
| --- | --- |
| Semantic revision | `sha256:cd0a7b6923ccc8a0afa31a27fca3aff280c3da601089ba12946f2d768fa1a829` |
| Canonical wire | `sha256:d96b6398e6856a1e34916046a6ca61ee0e09339f59c5c3664c524a5b43a801de` |
| Source | `sha256:5cb3e96f78c5eae478aee4861c8e548e6203ceaef4301fbb305a60977a7e791a` |
| Canonical companion | `sha256:d4e273f65f19cb981645d62af2dafa7ed8e9da02595b06861c75efacfa7ce6c3` |
| Joint package | `sha256:de9a319aeab50a1d0ab703498a152862a990163cd8f99a7c389d00536d1bf829` |

All11 current checking/advisory views must retain exact non-executable results and
native format/reparse must preserve IR while package integrity changes. Earlier
source18 feature/parameter-stage proofs select the complete explicit archive;
the new profile's call preflight can expose ordinary static defects at lowering,
as already accepted in ADR-0090. Raw native measurements replay unchanged under
their original fixture paths. Full old113/173/605/778/905-file epoch digests remain
strict through explicit reconstruction, not dropped identities or rewritten captures.

All local688 Rust tests +6 compile-fail tests, native workspace lint, five Wasm
library lints, both release Wasm builds, native portability and all87 Python CI
commands pass. The new negative test initially expected a source-type assertion
error for a text Instruction; ordinary native field typing rejects that invalid
expanded body first with `expression-type`. The test now asserts that real private
refusal; otherwise-valid wrong-input contracts remain covered by the native source
assertion suite. Separate review, final-head CI/full comment audit and expected-head
signed merge remain required before integration. All732 RP-03 files, including source19 and its external-
region refusal, stay exact. No runtime work/storage/authority is shared. Full
RP-08/work reuse/protected accessible inspection/editing/export/text-visual, actual
users/hosts/browsers/independent review and every full semantic/region/G1–G4/
scenario/Phase1 criterion remain open. Parent46 stays draft; no Phase2.
