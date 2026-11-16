<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0034: Bound full-benchmark authoring and actual-use wait endpoints

**Status:** Accepted under Project Owner delegation; effective upon integration
into main through the parent benchmark PR<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 owner delegation

## Context

The [frozen RP-01 package](../evaluation/0034-rp01-package-plan.md) exceeds source
0.4's 4,096-token limit: its whole draft has 8,945 tokens and needs 101,388 native
recognizer pattern visits. A separate small witness shows another source gap:
root-visible capability and effect/wait policies serve an activity inside a child,
whose own direct follow-up owns the wait. Raw core 0.2 can declare that shape;
source 0.4 only exports the policy scope's own designated follow-up. Neither gap
justifies silently changing old profiles or exposing all private descendants.

## Decision

Adopt [source 0.5.0 / companion 0.4.0](../authoring/profile-v0.5.md), reusing core
0.2 and the unchanged 0.3 EBNF. Only the known explicit program prefix selects
16,384 tokens and 1,000,000 native parser pattern visits. Keep legacy/fragment
quotas and all other bounds. Expose read-only syntax token/pattern counters.

Extend only missing wait observation/timer owner lookup through explicitly fully
qualified descendant scope chains. After binding steps, require an actual wait
using the exact policy, then either the old policy-scope direct reconcile-child
relation or a nonempty set of actual declared capability-use owners all designating
that exact direct follow-up. Index uses once; retain ordinary successes and wrong
kind/qualification errors. No generic private-data or outcome visibility changes.
Strict pairs/pins and versioned package hash domains remain; unchanged semantic
IR has unchanged revision across promotion.

## Alternatives and consequences

| Option | Benefit | Cost/risk | Decision |
| --- | --- | --- | --- |
| Explicit bounded source version and specialized actual-use binding | Preserves old refusals and semantic pins; admits measured draft declarations | Adds profile maintenance; higher bounded parser cost | Adopt |
| Increase all default quotas and descendant visibility | Fewer version branches | Changes old limits/privacy; unmeasured general behavior | Reject |
| Split full benchmark into unrelated toy packages | Fits legacy parser | Loses whole-case composition and evidence | Reject |
| Move all capability declarations into child scopes | Avoids ancestor policy case | Root protection references lose required lexical visibility | Reject |
| Redesign core ownership/lifetime now | Could address later proof gaps | Not necessary for source declaration binding; requires separate semantic evidence | Defer |

Compilation still does not prove host authorization, live ownership, safe surviving
storage or settlement. The isolated called-child witness still refuses its ordinary
lifetime proof; full RP-01 still owes static and later-stage evidence. No scenario,
G1–G4 gate, user-study or Phase 1 exit is upgraded. No production runtime is added.

The [frozen plan](../evaluation/0034-benchmark-profile-plan.md), `7dc428a`, scopes
this stacked dependency separately from draft PR #46. [Evidence](../evaluation/0034-benchmark-profile.md)
records focused cases, full checks and separate delegated automated self-review.
No individual owner or independent review is claimed. Passing final-head CI and
expected-head merge remain required; main effectiveness waits for parent integration.
