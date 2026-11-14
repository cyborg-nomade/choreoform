<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Fixed all-join history and precise RP-03 invalidation

**Date:** 2026-11-11<br>
**Frozen plan:** `0a04032`, [scope](0077-capture-inherited-context-plan.md)<br>
**Pre-correction critique/baseline:** `21a3e76348a696434f2715d01b8d90de13705198`,
[actual tuple and three passing baseline groups](0077-capture-inherited-context-audit.md)<br>
**Decision:** [ADR-0077](../decisions/0077-fixed-all-join-history.md)

## Bounded existing-contract proof

The full baseline identifies `authorize_call_value` in `accept_payment` as a live
private influence at `capture_activity` in `capture_service`. It survives an exact
fixed all-join even though the branch has reported its terminal outcome. The
pre-correction audit is signed before production changes; it does not invent a
current value or misidentify the result as a missing simple invalidation clause.

The analysis now refines the matching direct nonempty all-join of a fixed split.
It retains the pre-child live snapshot and indexes exact member scope IDs. Each
origin is visited once. Only a cell owned by an exact member, with no declared
nonlocal writer and absent from that inherited live snapshot, becomes conditional
historical evidence at the join boundary. Invoke/repeat share the same classification
conditions. Original protection labels, mutable ancestors, source providers and
unproved descendants remain. All declarations, including unvisited writers/port
writes, are indexed before refinement. Join-entry snapshots stay local to each scope analysis and are not reused across calls.

Snapshot storage/lookups, member indexing and origin visits use the existing
cumulative million-work budget. A 512-member/2,048-local-origin positive regression
passes with indexed membership; dense historical consumption still refuses at the
unchanged budget. There is no per-member rescan of the complete origin set or
larger quota. Protection-only output stays identical modulo the changed definition
revision; its analysis does not use the historical refinement.

Seven bounded raw SDK groups cover exact history/boundary/scope/closure, strict versus
advisory evidence, possible unvisited descendant writes, partial/threshold/composite
joins, inherited live template occurrences across a later all-join, mutable ancestor
coverage, original control-protection refusal, unproved deeper descendants and
wide/dense work. The private whole-case tuple group checks current history and an
isolated one-field partial-join negative. Six Python/native groups exercise complete
readable source, all eleven views, format/integrity, actual earlier private-read /
coverage refusals, full-case adoption, exact archives and safe generated-file cleanup.

## Actual full RP-03 adoption, not an advisory promotion

After the refinement, the unchanged #91 source's current advisory actually derives
312 ordinary invalidation obligations, including 278 missing links, plus 849 historical
origin rows. Strict checking correctly reports missing authorization-record coverage.
The finite generator applies only those actual missing links, retaining every old
link. It resolves exact ledger scope/node names and recognizes only its own known
line/block layout, then runs the real complete pipeline again. Missing/invalid
advice aborts construction. This is not a general editor or a passed-check report.

The compiled candidate differs from #91 **only** in the added `data.invalidates`
members and resulting revision. Source/IR code, types, expressions, business guards,
receipts/inputs, actor/capability/effect/wait identities, storage/ports, protection /
provider/source claims and companion bytes otherwise stay fixed. A regression
independently compares the entire compiled body to the exact advised delta and
checks final missing obligations are empty. The current package has 34 analyzed
invalidation nodes and 45 uncovered nodes; the latter are not silently certified.

Current source is 576,759 bytes /84,051 tokens, same source0.16/companion0.15/C12/IR0.4.
Canonical revision:
`sha256:a3b8c0f07cfc63d745376c3eb11b67690c98edc3e216d262c1542f00a05798aa`.
Canonical wire digest:
`sha256:6550ac457539457b6e48a452d17d95060db1a2f36fbcf74f8b3c7c1e7c11cfb4`.
Wire is 883,289 bytes, within its existing bound. Static/control/protection/strict
invalidation pass, and the distinct advisory passes. Five actual reports remain
`executable:false`. The six later checking views actually refuse
`parallel-effects-unproved at /body/nodes/start_acceptance; source bytes 109844..110031`.
No later report, parallel safety or executable/host permission is inferred.

## Exact previous evidence and honest regeneration

All 121 generated #91 files were compared byte-for-byte with signed head
`8e5d1382a0d6964b77a3696046df637544756901` before archival. Their sorted path/NUL /
8-byte-length/content digest is
`c3da997a63a63ef4c19631b5765c7077e32fab288f24f072808f334451e465b5`.
The previous explanatory README is retained separately. The new archive is immutable
input evidence verified against recorded per-file SHA-256 pins, not a claim that
today's refined checker reproduces its old semantic refusal. Reproduce the exact
then-current checker at the signed commit. Current rechecking of that old source
now reaches genuine missing coverage; the historical manifest is not rewritten.

The source0.14 and pre-instance source0.16 generators/archives/refusals remain exact;
all current/historical RP-01 bytes stay fixed. Provider/type/clock/launch/receipt /
guard/scenario meanings and every old resource pin/claim remain. Generated output
cleanup removes only known stale candidate/stage filenames after complete construction;
unknown files are retained so inventory checks can report them, rather than delete work.

## Verification and next dependency

Local verification passes 600 Rust tests plus six compile-fail doctests, format,
native lint, all five library Wasm lints, release portability/frontend Wasm builds,
fresh native portability and all 73 Python commands. Logs are
`/private/tmp/all-join-workspace.log` and `/private/tmp/all-join-verify.log`.
Separate completed-diff automated review/full comment audit and exact-head CI are
required before merge; skipped bot review supplies no independent evidence.
These are synthetic static declarations, not independent or personal owner approval,
actual child outcomes, snapshot storage, values, effect eligibility, clock/receiver /
registration/current authority/cutoff/atomic handoff/all-transitive settlement.
Next audit the exact parallel-effects refusal and existing effect/guard/closure
contract before choosing a compatible proof or separately signed semantic dependency.
Parent #46 draft, forty/all four Partial, G1–G4/full/optional/RP-08/validator /
near-English factoring/managed identities/protected accessible text-visual/actual
users/hosts/browsers/independent/phase exits remain open. No Phase 2.
