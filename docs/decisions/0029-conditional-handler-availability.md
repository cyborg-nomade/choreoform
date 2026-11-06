<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0029: Check handler-body availability under explicit input contracts

**Status:** Accepted under Project Owner delegation; effective upon PR merge<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 delegation

## Context and criteria

ADR-0028 discovers potential handler contexts without supplying initialized or
authorized data. Ordinary-path state/result evidence cannot be reused as a fault
snapshot. The core has no fault-data port, emission bound or supported fault-site
snapshot proof. Supply a useful conservative body judgment with honest premises,
fresh locals, bounded work and unchanged prior APIs/contracts.

Repeated scope checks also scan every declared cell, even when a visited empty
scope has no initializers. The pre-change regression demonstrates an unrelated
storage × invocation product exhausting the work quota. New handler entries would
multiply that bottleneck, so index the immutable metadata before adding contexts.

## Decision

Add `check_exceptional_availability` after fault routing, with immutable
ExceptionalReport and Candidate/native adapters. For each routing-discovered
handler target, derive a minimal entry premise from its lexical ancestors' declared
input cells. Scope ownership/types are already checked, and successful occurrence
creation must bind these inputs. Record the exact assumed cell set by handler;
do not claim the live occurrence, value, input revision, authority or activation
has been observed. Uncalled handler declarations receive no invented context.

Check the body conditional on successful handler activation and live owning
ancestor occurrences with the required bound input revisions. Use no owner
initializer, ordinary write/result, sibling data, prior handler local or fault
record as guaranteed entry data. This input-only lower bound intentionally can
refuse valid richer fault-site snapshots as unproved. An initialized cell and a
declared input are different process definitions; tests of both are not an
automatic migration or a proof that an initializer is globally unsafe.

Reuse ordinary initialization/availability rules for the handler body and its
ordinary children. Own initializers use the pre-entry ancestor/input snapshot;
peers cannot make each other available by lexical order. Fresh locals are
recreated at each entry. Nested selected handlers get independent minimal input
premises. An ordinary proof of a shared template cannot replace the handler proof.
One shared graph, initializer index and work budget serve the new body checks.

Return conditional checked nodes/scopes, handler premises and overall remaining
node coverage separately from prior ordinary/routing coverage. Availability
failures use `exceptional-availability-unproved` at the actual failing declaration;
other conservative control/budget refusals remain unchanged. No partial proof,
current host fact or executable capability is returned. Native expanded entry
metadata/final output is bounded, as for the routing adapter.

Index initializer IDs privately by declaration scope once in control and
protection. Store borrowed immutable IDs, not initializer values, runtime state
or policy truth. Charge index construction and actual per-visit local/member work;
preserve input handling, pre-entry snapshot and fresh-local rules. Scope lookups
replace full-table scans. Avoid allocating an Origin::Cell just to compare an
initializer's own ID with an existing borrowed cell origin.

## Options considered

| Option | Benefit | Cost/risk | Decision |
| --- | --- | --- | --- |
| Explicit ancestor-input premises and separate body proof | Safe simple lower bound, useful contexts, no invented fault snapshot | Over-refuses owner initializers/results without richer proof | Adopt |
| Reuse ordinary available-state union or assume all initialized cells | Appears permissive | Fault timing/activation/context can differ; silently credits unproved evidence | Reject |
| Treat an author/host boolean as snapshot authorization | Easy API | Converts a claim into authority without provenance | Reject |
| Add a fault-site/emission/activation profile now | Potential precision | Requires durable new source/host semantics and full evidence | Future explicit precision option |
| Keep full-table scans for every new context | Minimal edit | Unrelated storage causes quota refusal and multiplicative work | Replace with private immutable scope index |

## Consequences and confirmation

Presence/type checking under stated premises is useful but is not fault-control
information flow, current read/write permission, sufficient decision evidence or
actual snapshot binding. Concurrent fault/handler occurrence lifetime, activation
failure routing, reconciliation entry/ownership, invalidation and atomic runtime
enforcement remain open. No handler ports or fault record bindings, contract/source
versions, old artifacts/examples or dependencies change. Phase 1 remains incomplete.

Follow [frozen plan](../evaluation/0029-exceptional-availability-plan.md), `6331fb7`,
and [evidence](../evaluation/0029-conditional-handler-availability.md). Separate
automated review and final-head CI/expected-head merge are required under ADR-0018;
no individual owner, independent reviewer or actual host/user observation is claimed.

Accepted after separate completed-diff automated self-review under delegation.
Local verification passed 222 Rust tests plus five compile-fail doctests,
native/Wasm checks and all relevant artifact/source/stage oracles. Review carried
the same budget from premise derivation into the shared body graph/index/visits;
current-host and fault-control limits remain explicit. No actionable finding
remains. Final-head required CI and expected-head merge remain prerequisites.

1. [x] Finish conditional-context/index regressions and relevant native/Wasm checks.
2. [x] Record delegated acceptance after separate review; require final-head CI.
3. [ ] Continue exceptional control-protection/invalidation/lifetime and remaining
   full-validator/benchmark/cross-form/user evidence before Phase 1 report.
