<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Explicit timerless wait capture: frozen raw-contract plan

**Date:** 2026-11-07<br>
**Baseline:** `f9f5bdc2b7c00756fddc9514b486bcc18fc4c872`, draft #46 after #58<br>
**Authority:** ADR-0018; audit #57 identifies wire/source/protection/lifetime gaps

Use a dedicated stacked dependency PR. Record ADR-0045 with evaluated wire versus
policy-table options, then publish Definition IR 0.2 and core profile 0.5 exact
artifact bytes. The new wire wait record adds required nullable
capture:{source:Ref,origin:NodeID,result:DataID}|null; every other core record remains
unchanged. Source pins exact terminal-observation metadata; origin names the actual
capability activity whose provider ID/revision must equal that source's provider.
Result names a normally visible exact-typed destination cell. No private/sibling
visibility exception or implicit occurrence/correlation/value is introduced.

IR 0.1 wait shape and every old fixture/schema/pin/revision stay unchanged. Decode
both exact known wire versions explicitly with version-specific closed wait fields;
unknown wire sentinel moves from 0.2 to genuinely unsupported 0.3. Add an explicit
versioned in-memory constructor while the existing constructor remains fixed 0.1.
Canonical semantic projection already includes format/version/kind/body; preserve
that framing and all old hashes. Add typed ArtifactRef/capture projection with
correct NodeId/DataId roles, source-ref shape/digest and reference checks. Publish
new closed schema/prose; verify schema/native/Python alignment, all ten variants,
missing/unknown fields, cross-version refusals and unchanged old projections.
No structural report becomes a semantic or executable admission.

Core 0.5 inherits exact 0.4/0.3/0.2/0.1 artifacts and finite metadata semantics.
Require its use on IR 0.2; old policies on IR 0.2 may have only null capture.
Every non-null capture requires core-0.5 wait policy, timer absent, exact normalized
source type == wait observationType == destination type, visible capability-origin
activity and exact source/provider identity (including ordinary-unvisited nodes).
Human/wrong-kind/wrong-provider/unknown-source/unsupported-profile contexts refuse.
Preserve all old retry/work/metadata contracts; no source/companion versions change.

Derive/verify complete node writes (exact captured cell) and read dependencies:
wait match/authority, scope management, source provider protection references and
destination protection. Source capture fields are per-use node metadata, so their
context is the actual wait scope; do not pull private values into a shared ancestor
policy or guess retained original input values. Original occurrence/request/key /
input correlation stays an explicit host premise, not an ambient value read.

Require P2 source-envelope -> destination transfer with no implicit declassification,
including actual control dependencies and access obligations. Do not label by the
destination alone. Enforce source/target intrinsic binding even for unvisited
captures; ordinary protection/availability reports retain their coverage boundaries.
Timerless sole onObservation completion initializes the result only on accepted
observation; cancellation, duplicates and unmatched/nonterminal observations never
cause a write or ordinary completion. Existing ordinary write-based availability
must prove post-wait reads and reject pre-wait reads. Add exact invalidation/read /
writer-index participation without treating a metadata origin link as control flow.

Expose bounded immutable revision/site/source/profile/provider/origin/result
static declaration evidence with explicit acceptance/correlation/current authority /
protection/cutoff/ownership requirements. No actual observation/value, accepted
fault, retry eligibility, owner, execution handle or implicit parent resume is
returned. Keep P6 data-free survivor and zero-inherited-value limits; a capture in
a stored recovery still refuses the unresolved lifetime subset. Ordinary same-owner
capture can be the positive foundation; it does not complete C/D reconciliation.
Native authoring/report integration follows in a separately explicit source profile.

Tests: raw IR-0.2/core-0.5 ordinary capability -> wait capture -> read/branch with
real exact terminal metadata, complete static/control/protection/invalidation and
composed checks where supported; full RP-01 promoted-copy static binding plus
honest stored-recovery lifetime refusal. Cover every pin/base, missing/null/unknown
fields, old-wire/old-core refusal, exact origin/provider identity even unvisited,
nominal type/protection mismatch, no declassification/control laundering, timer
branch refusal, read-before-capture and stale read/write/invalidation declarations,
wrong/private references, mixed finite/legacy cycles and cumulative work/depth/report
bounds. Keep actual capture tests distinct from host acceptance/execution evidence.
Add independent versioned schema/hash/binding/type/flow corpus/oracles and CI.

Run final-head native/Wasm/all Python checks, separately review completed diff,
audit all comments/CodeRabbit summaries and sign off every authored/merge commit.
Merge only the exact passing head into draft #46. Then freeze explicit source/
companion support and bounded native disclosure; later prove owned local storage /
lifetime and explicit fault data/continuation/cutoff/settlement before benchmark
adoption. Current source 0.7/core 0.4/IR 0.1 package remains unchanged. No phase gate,
complete scenario, actual host/user/browser evidence or Phase 2 work is claimed.
