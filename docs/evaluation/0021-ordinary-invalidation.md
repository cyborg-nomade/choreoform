<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Ordinary invalidation declarations: evidence

**Date:** 2026-11-05<br>
**Status:** Accepted under delegated review; final-head CI required before merge<br>
**Decision:** [ADR-0021](../decisions/0021-ordinary-invalidation.md)<br>
**Plan:** [frozen scope](0021-invalidation-plan.md), `f40f824`<br>
**Baseline:** `14cdfa0147e9c0b0f324e02b29a057dbd0dd689e` (PR #28)

## Result

`check_invalidation` / `Candidate::check_invalidation` / CLI `check-invalidation`
check conservative ordinary dependency coverage after all earlier stages. A
visited decision, human/capability activity or wait requires invalidation links
from visible pre-result cell origins. Branch and repeated-call contexts union.
Missing links refuse at the actual data invalidation set. Nonvisible child-local
completion origins refuse explicitly as unproved, without inventing a link.

The immutable report binds the revision and exposes sorted site/cell obligations
and checked/uncovered consequential sites. The CLI distinguishes those sets from
earlier ordinary control coverage, retains required launch inputs, and says
`executable:false`. Old stages retain their contracts and do not collect this
extra dependency map. No syntax, wire, frozen resource or dependency migration.

This checks declared links only. It does not validate a live revision, withdraw or
cancel work, erase decisions/effects, undo effects, grant authority, bind storage
occurrences or prove a race safe. Provider observations/non-data changes and
exceptional paths still require explicit rules/enforcement. Cross-context value/
occurrence provenance needs more precise analysis before the unsupported cases
can pass. Full invalidation, complete validation and Phase 1 gates remain open.

## Regression evidence

Seven new Rust groups cover direct dependencies without an immutability guess,
control and both branch origins after a merge, work-policy reads, repeated child
contexts, nonvisible completion-origin refusal, unvisited consequential sites,
and a consumed old result that remains a dependency even when the site replaces
it. Earlier protection checks pass before every intended new refusal; paths exist
in the definition and diagnostics do not echo the literal marker.

The [four-group Python oracle](../../tools/check_invalidation.py) independently
checks schema/JCS/revision and source format/reparse cycles for terminal, review,
service and branch packages, then checks Rust CLI reports. It confirms the human
and capability examples need two input links and do not spuriously invalidate
themselves from their own new result. The original service wait remains uncovered;
an explicitly ordinary-connected variant needs its prior work result and both
input/control dependencies. Removing links passes earlier stages, then refuses
without payload leakage. Documentation links and SPDX notices are checked.

Data-access dependency closure remains part of the earlier static/protection
stages; their existing conservative availability and policy-identity refusals
remain in force. The positive work-policy test uses a completion predicate rather
than claiming that an earlier-refused access-policy configuration passes. Neither
the Python artifact oracle nor these synthetic cases are an independent runtime.

## Verification and review

Run the [required workflow](../../.github/workflows/rust-portability.yml), including
workspace tests/format/lint, semantic/frontend Wasm lint, release frontend Wasm
compilation and all existing stage oracles plus `tools/check_invalidation.py`.
Review pre-result ordering, retained old-result reads, branch/repeated context
union, lexical refusal, current-fact/ownership limits, privacy and cumulative work
accounting separately before recording delegated acceptance and final-head CI.
Continue concurrency/lifecycle/clock work after merge; do not stop at item review.

Local verification passed 145 Rust tests and five compile-fail doctests, format,
native lint, semantic/frontend Wasm lint and release frontend Wasm compilation,
and authoring evidence/frontend, static/control/protection/lifecycle/invalidation
Python checks. Separate automated self-review confirmed pre-result collection
without dropping consumed old results, complete ordinary branch/context unions,
explicit lexical refusal and coverage sets, old-stage behavior, privacy and the
budget charges for added visits/member copies. All intended new refusals follow
earlier-stage success; no independent or runtime review is claimed.
