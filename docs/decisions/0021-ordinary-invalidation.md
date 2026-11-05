<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0021: Check conservative ordinary invalidation declarations

**Status:** Accepted under Project Owner delegation; effective upon PR merge<br>
**Date:** 2026-11-05<br>
**Decider:** Codex under Project Owner delegation, ADR-0018

## Context and criteria

ADR-0013 P7 requires invalidation coverage for decision/effect-relevant changed
inputs. The static checker verifies invalidation target references but not that
every consumed dependency declares the target. A prior decision can also carry a
dependency into later work through control flow. Checking only a site's explicit
input expression would miss that dependency. Current control/protection stages
are deliberately partial; they do not execute withdrawal or prove current facts.

Require a bounded, explicit stage that preserves those earlier contracts, refuses
missing coverage and unsupported context inference, and separates definition
links from runtime sufficiency. Do not infer immutable data, guess a live storage
occurrence, or permit a Boolean to certify an irreversible race.

## Decision

Add `check_invalidation` and source/CLI `check-invalidation` after the existing
checks. Optionally collect cell origins at each ordinary-visited decision,
activity (human/capability) and wait before its own result is produced. Use the
existing monotone control-origin approximation and full declared/access reads;
retain branch unions and repeated-call context unions. Do not add a site's newly
produced result to its pre-result dependency set. Earlier protection calls do not
collect or retain this additional map.

For every dependency visible in the site's lexical scope, require its
`data.invalidates` set to name the site. Missing coverage refuses at that actual
set with `invalidation-coverage`. A child-local completion origin that cannot
name a legal invalidation link to later parent/sibling work refuses
`invalidation-context-unproved`; do not drop it or fabricate a link. This is a
conservative supported-analysis boundary. Valid programs can need more precise
value/occurrence provenance, not merely more declarations.

Return private immutable report fields, revision binding, sorted site/cell
obligations, checked consequential sites and ordinary-uncovered consequential
sites. The CLI also preserves earlier ordinary control coverage and launch-input
assumptions and always says `executable:false`. Definition IDs identify template
obligations; the host must bind actual consumed revisions/occurrences. No values,
authority grant, dispatch interface or executable credential are returned.

The approximation includes conservative surplus reads and inherited creation/
completion contexts and does not prune constant guards or release dependencies
at a merge. Data-policy configurations refused by earlier availability/protection
checks stay refused; this stage cannot bypass them. Provider observations and
other non-data changes are not invented data cells and remain explicit residual
obligations. No wire, grammar, resource version or dependency changes.

## Options considered

| Option | Benefit | Cost/risk | Decision |
| --- | --- | --- | --- |
| Conservative ordinary origins, explicit coverage and unproved context refusal | Implements useful P7 links without claiming runtime authority | Can reject valid programs; exceptional and occurrence reasoning remains | Adopt |
| Only explicit input reads | Small implementation | Misses control and policy dependencies | Reject |
| Silently omit nonvisible/local origins or pick a live occurrence | Accepts more examples | Loses dependencies or invents runtime identity | Reject |
| Treat all cells as immutable or invalidation as automatic effect undo | Appears to simplify lifecycle | Contradicts revisions and accepted P7 semantics | Reject |
| Full path/occurrence-sensitive invalidation proof first | More precise acceptance | Requires broader lifecycle/concurrency model and evidence | Follow-up; retain honest partial stages |

## Consequences and limits

Missing links fail before later planning. Adding links does not erase decisions,
undo effects, cancel active work, withdraw consent or authenticate renewed work.
Those actions require fresh occurrences, retained history, live authority and
atomic host enforcement. Parallel races (especially irreversible effects),
exceptional paths, non-data revisions, reconciliation and supported clocks remain
open, as do complete benchmark/conformance, source/visual parity, actual user
studies and Phase 1 exit. This ADR does not close the full invalidation contract.

Existing 1,000,000-unit per-analysis budgets and depth/transport/resource bounds
cover added context-member copies, unions, declaration indexing, membership and
lexical visibility walks. No state-dependent live policy evaluation occurs.

## Confirmation and action items

Follow the [plan](../evaluation/0021-invalidation-plan.md), committed `f40f824`, and
the [evidence](../evaluation/0021-ordinary-invalidation.md). Review the completed
diff separately and require final-head CI before expected-head merge.

Accepted after separate automated self-review under the owner's 2026-11-05
delegation. Local verification passed 145 Rust tests plus five compile-fail
doctests, native/Wasm checks and relevant stage oracles. No independent human
review or owner review of this individual ADR is claimed. Required final-head CI
remains a merge prerequisite.

1. [x] Complete implementation, regressions, source cycles and native/Wasm checks.
2. [x] Record automated self-review and delegated acceptance before merge.
3. [ ] Continue concurrency, lifecycle/clock and aggregate Phase 1 gates.
