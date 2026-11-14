<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Current RP-03 typed pure-test adoption

**Date:** 2026-11-14<br>
**Status:** implementation locally verified; final-head CI, separate review and integration pending.<br>
**Authority:** [ADR-0018](../decisions/0018-phase-review-delegation.md), signed
[scope91](0091-rp03-predicate-adoption-plan.md) `373f5b1` and
[ADR-0091](../decisions/0091-rp03-pure-test-adoption.md); no personal owner or independent review.

PR #105 supplies the separately contracted source19 compiler. Its final head
b7057abda1a472a3a80df413f7557ec4ddcf1760 passed CI43476905305, separate automated
COMMENT review5895660433 and fresh full comment/thread/head audit before signed
merge63988a63dc826dff6776251dbe747d947e17b2aa. The CodeRabbit skip supplies no
review evidence. Every commit and merge carries the required sign-off.

Signed archive `dddceb238e8424171fd71b80b70dbd132cd97847` precedes producer/current
consumer changes. Its126 exact files total3887465 bytes; a separate archive note
adds one file. [Inventory](0091-before-predicates-inventory.json) and prior complete
605-file RP-03 /778-file combined trees remain strictly pinned. The new archive
adds127 files; none of the prior historical files or173 RP-01 files changes.

The new finite model producer reproduces the complete previously tested source19
experimental source exactly: **291309bytes /41306 native tokens /386333 parser
visits**, down from410752 /66783 /840430 (29.08% fewer source bytes). Complete
definitions/types/uses are included; parser visits are not elapsed performance or
user-readability results. The canonical companion value matches the experiment.

The producer reads no frozen source fixture. It reconstructs existing complete
case ASTs, checks full instruction/receipt representation/nominal pins, validates
the two exact balanced templates and restores each original AST before rendering.
Its legacy mode reproduces all125 generated prior source18 files raw-byte exactly.
Current production selects new syntax explicitly; default/historical producer
paths retain their old behavior. The native Rust frontend supplies binding/type/
IR proof, not the Python renderer. No compiler/contract/quota changes are needed.

Only the four [current-file digests](0091-rp03-adoption-digests.json) change: source,
bindings, manifest and README. The other122 current files, complete canonical IR,
five reports, immutable resources/nominal invariants, receipt/guard vectors and
every semantic/request/work/capability/endpoint/capture/compensation identity stay
byte-identical. New managed identities belong only to source tests/inputs/type.

| Pin | Exact value |
| --- | --- |
| Semantic revision | `sha256:a3b8c0f07cfc63d745376c3eb11b67690c98edc3e216d262c1542f00a05798aa` |
| Canonical wire | `sha256:6550ac457539457b6e48a452d17d95060db1a2f36fbcf74f8b3c7c1e7c11cfb4` |
| Source | `sha256:b6101a0f870a94b30e9e484a2b0571d5ab7840cf6ba5e32d24e971baa1ba0375` |
| Canonical companion | `sha256:141235570474a05d6bdf70d33d145533d0718b9b64dc89b0020b02b0092737c0` |
| Joint package | `sha256:7803000c6eba666edb1b5fa93a176897abd5166a8b96a7945659d54b9d31807d` |

All55 cases,23 distinct nominal results/invariants and25 work occurrences remain
separate. The two pure tests have four explicit inputs; no execution/storage/
authority is shared. Four ordinary checks and advisory invalidation retain success.
Six views from concurrency onward retain `parallel-effects-unproved` at
`/body/nodes/start_acceptance`, with actual new source span58784..58971.

Historical proofs explicitly load their full source18 archive. The native metrics
oracle replays complete exact archived packages under their original relative
fixture names in a temporary tree, preserving the raw captured report. Whole-tree
proofs reconstruct their exact605/778-file epoch and retain original digests.
Current-package tests continue to use the actual new source, all11 outcomes,
format/reparse/integrity and meaningful invalid call/type/identity mutations.
Frozen source19 experimental artifacts and older raw evidence remain unchanged.

Full local validation passes688 Rust tests plus6 compile-fail tests, native lint,
five Wasm library lints, both release Wasm builds, native portability and all86
Python commands. The initial full Python run found two current-audit assertions
with old source spans; both now assert the exact new span58784..58971, with refusal
code/path unchanged, and all affected/remaining checks pass. The initial new
negative test expected a diagnostic name that is not used by the checker; it now
asserts the actual `expression-type` code, and all five adoption groups pass.
Final-head CI, separate automated review, full comment audit and expected-head
signed merge remain required before integration.
RP-01 stays source18; its separate archive-first adoption follows. Full RP-08,
work/reuse/protected managed inspection/editing/export/text-visual, actual users/
hosts/browsers/independent review and every full semantic/region/G1–G4/scenario/
Phase1 exit criterion remain open. Parent46 stays draft; no Phase2.
