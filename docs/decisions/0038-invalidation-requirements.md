<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0038: Derive advisory ordinary invalidation requirements

**Status:** Accepted under Project Owner delegation; effective on parent/main integration<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

Full RP-01 now reaches a genuine missing invalidation declaration. The strict
checker reports its first missing cell path. Guessing every possible link would
change behavior unnecessarily; repeatedly patching first errors obscures the
complete ordinary dependency requirement. The [frozen plan](../evaluation/0038-invalidation-requirements-plan.md),
`62fd323`, requires an advisory view before changing benchmark declarations.

## Decision

Add a separate `InvalidationRequirements` type, derivation function, Candidate
method and native `invalidation-requirements` command. Share private immutable
analysis with the strict checker. Require identical resource/static/control/
protection prerequisites and ancestry checks, preserving historical provenance
conditions. Derive all ordinary live site/cell obligations and their exact
required-minus-declared missing set. Refuse unproved contexts or exhausted budgets
entirely; return no guessed partial list. Do not invent entries for unvisited sites.

No public coverage bypass, conversion to a strict report, implicit edit or new
syntax/core/IR pin is introduced. Existing strict and composed checks retain their
first-error ordering and coverage requirement. Advisory output always identifies
itself as non-executable requirements, with revision, input assumptions,
historical conditions, unvisited sites and residual gates. Even an empty missing
set requires a fresh strict check at the final definition revision. Missing-pair
copies consume the existing work budget; native historical and final JSON output
is bounded before publication. No values or current authorization are returned.

## Alternatives and consequences

| Option | Benefit | Cost or risk | Decision |
| --- | --- | --- | --- |
| Separate advisory type sharing private analysis | Complete reviewable ordinary requirements with a strict type boundary | Additional API and bounded output | Adopt |
| Public ignore-coverage flag on strict checker | Small API change | Can misrepresent incomplete analysis as passed | Reject |
| Add every visible site to every cell | Simple source patch | Unnecessary behavioral changes and obscured dependencies | Reject |
| Repeated first-error edits only | No API addition | Hides the full requirement set and complicates review | Reject |

Requirements are conservative within the existing ordinary analysis, not a claim
of minimal causal links or complete exceptional/non-data lifecycle semantics.
Reports do not authorize execution, prove actual closure or replace later
concurrency, host, benchmark or phase evidence. The dependency does not alter
RP-01 source declarations. Return to its exact missing links after integration.

[Evidence](../evaluation/0038-invalidation-requirements.md) records tests and the
separate completed-diff automated review. Acceptance is under delegation, not
individual owner or independent review. Require final-head CI, comment audit and
expected-head signed-off merge into draft #46; the parent remains unmerged.
