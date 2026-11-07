<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Isolated surviving waits: frozen audit and implementation plan

**Date:** 2026-11-06<br>
**Baseline:** `b95b2be93c14626adfc248fdf3d6a30235ea8343`, draft #46 link checkpoint<br>
**Authority:** ADR-0018; actual `scope-lifetime-unproved` at `run_checks`

Use a dedicated stacked dependency PR. Before implementation, record a critique
with actual native isolated-read-only, mutable-writer and shared-read/non-wait
counterexamples, and audit P6/P7 contracts. Do not change benchmark closures or
label surviving effects settled. If existing semantics permit a conservative
ordinary-continuation independence subset, record ADR-0039 and implement only it.

Candidate subset: synchronous invoke/repeat bodies with an explicitly owned,
direct-child reconciliation boundary. Every declaration in that follow-up must
be a data-independent, timer-free wait or finish, with no storage, ports, nested
children, external/human activity or ordinary composition. Its own closure must
be terminal and all scope-management/wait/finish dependencies must be data-free.
Require a reachable wait-to-finish route; index all declarations, including
unvisited hazards, under existing cumulative budgets. Do not use policy names,
true guards, caller context guesses or supplied safe flags as proof.

This proves only no declared data/effect mutation by that surviving subset, under
existing P6 fresh authority/owned handoff/closure enforcement. Preserve terminal-
only prerequisites for split/fanout, early joins and all ordinary conflict checks.
Actual occurrence identity, authenticated exact correlation, accepted atomic
handoff, retained transitive obligations, deduplication/contradiction handling and
all-obligation settlement before terminal disposal remain required host premises.
No guarantee of completion/progress, general reconciliation safety or planning.

Expose immutable revision-bound typed conditional evidence for each covered
creator/body/follow-up/wait set, not an actual live owner or completion receipt.
All native composed views from concurrency onward must disclose that evidence
and its outstanding host requirements under the existing bounded publication.
Do not invoke later composed stages recursively to justify their prerequisite.
Old terminal cases and source/core/IR identities remain unchanged.

Tests must contrast exact allowed invoke/repeat against shared reads/writes,
local storage/initializers, nested scopes, unvisited unsafe declarations, opaque
activity, nonterminal follow-up, missing wait/route, timers, split/fanout and early
join cases. Preserve historical label/live-ancestor negatives; a previously
refused isolated case may pass only with new conditional evidence. Exercise
reused templates, work/report bounds, native format/reparse identity and all
composed disclosure. Regenerate actual RP-01 stage evidence and retain the next
real refusal without weakening unrelated contracts. Run native/Wasm/Python and
final-head CI, separately review the complete diff, audit every comment including
CodeRabbit summaries, and merge the passing exact head with explicit sign-off.
Return to draft #46 and remaining whole-package/phase work; no gate closes here.
