<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Ordinary parallel independence: evidence

**Date:** 2026-11-05<br>
**Status:** Accepted under delegated review; final-head CI required before merge<br>
**Decision:** [ADR-0022](../decisions/0022-ordinary-concurrency.md)<br>
**Plan:** [frozen scope](0022-concurrency-plan.md), `6fd6161`<br>
**Baseline:** `6067d9037c6dd77f01e2b7895673c00c9c066027` (PR #29)

## Result and boundaries

The explicit concurrency stage checks a conservative ordinary independence subset
after invalidation/control/protection/static checking. Indexed scope summaries
include creation/access, nested and incoming control dependencies; fresh locals
are removed at their respective scope boundaries while ancestor storage remains
shared. Reused template IDs still create distinct storage occurrences.

Shared read/write or write/write hazards and unproved overlapping external work
refuse. A monotone predicate must imply all member outcomes and the child closure
contract must require terminal disposal. Early joins and surviving reconcile
lifetimes remain unproved rather than being silently treated as settled. Actual
membership sealing, scope disposal, truthful provider effects, authority and
atomic enforcement remain host obligations. Arbitrary/domain invariant proofs,
exceptional/reconciliation concurrency, clocks, full benchmarks and Phase 1 exit
remain open. Reports remain immutable, revision-bound and non-executable.

## Regressions

Twelve new control groups cover fresh reused/fanout locals; shared reads and
disjoint ancestor writes; write/write and read/write hazards despite a guarded
true invariant; initializer-only reads; nested ancestor writes; all/any/threshold/
and/or implications; reconcile lifetimes across all four creators; opaque human
work; unvisited forks; inherited pre-fork control reads; nonvisible completion
contexts; and fanout shared writes. A provider group checks two exact read-only
contracts with fresh results, then compensatable/irreversible variants with new
verified resource digests that still refuse unproved effects. All intended new
refusals pass the previous invalidation stage first.

The [three-group Python oracle](../../tools/check_concurrency.py) checks schema,
JCS/revision and grammar/package/source cycles for the four existing examples,
a two-child split and a split reusing one child template. It checks report
partitions and an early-join refusal after earlier-stage success, plus links/SPDX.
Synthetic examples do not constitute host, browser or participant-study evidence.

## Review and bounds

Separate review found that explicit child node sets alone missed a pre-fork
decision dependency. The optional origin collector now privately retains incoming
scope context unions for the later checker; earlier protection calls still do not
collect this map. Invalidation's public/CLI outputs remain unchanged. Nonvisible
origins refuse rather than being dropped or confused with another fresh local.

Review also replaced quadratic branch comparisons with accumulated read/write
indices and work flags. The same pairwise hazard predicate is checked through
union membership. A 2,200-branch empty workload passes; a dense repeated-footprint
workload passes earlier stages and exhausts the new cumulative budget boundedly.
No wall-clock threshold, unlimited traversal or scalability certification.

Run the [required workflow](../../.github/workflows/rust-portability.yml): workspace
tests/format/native lint, semantic/frontend Wasm lint and release frontend Wasm
build, plus every existing stage oracle and `tools/check_concurrency.py`. Record
delegated acceptance and passing final-head CI before merge, then continue the
[Phase 1 checkpoint](phase-1-progress.md).

Local verification passed 158 Rust tests plus five compile-fail doctests, format,
native lint, semantic/frontend Wasm lint and release frontend Wasm compilation,
and authoring evidence/frontend, static/control/protection/lifecycle/invalidation/
concurrency Python oracles. Completed-diff automated self-review covered lexical
storage boundaries, incoming context union, initializer/nested access, external
classification, all-member implications, lifecycle refusal, coverage, privacy and
work accounting. No open finding remains; no independent review or actual host/
browser execution is claimed by these checks.
