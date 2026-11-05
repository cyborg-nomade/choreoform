<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Timer contract checks: evidence

**Date:** 2026-11-05<br>
**Status:** Accepted under delegated review; final-head CI required before merge<br>
**Decision:** [ADR-0024](../decisions/0024-timer-contract-checks.md)<br>
**Plan:** [frozen scope](0024-timer-integration-plan.md), `36630bb`<br>
**Baseline:** `47084ce231abd51d6e3c0194785673347756d6c0` (PR #31)

## Result

Static/source checking recognizes only the pinned instance-clock profile, types
closed timer fields and pure Integer due expressions, validates basis/authority
and clock-source protection references, and closes all policy/access dependencies.
Timed waits require the exact distinct observation/timer outcome pair. Source
lowering derives the same dependencies; syntax and old fixtures are unchanged.

Clock source labels enter the ordinary protection context before eligible result
writes. Typed activity/clock roles prevent one provider from replacing another
at a shared node; both survive without a published result. Borrowed lookups avoid
origin-key allocation when a source role is absent. Conservative concurrency now
counts waits as external observations when checking opaque/effectful overlap.

The explicit timing stage returns immutable revision-bound uses and required live
host judgments, checked/unvisited timed sites and ordinary input/coverage facts.
It accepts no clock observations/current host facts and always stays non-executable.
Actual timing eligibility, source authentication/access, domain/origin identity,
revision/authority, pause/replay/reschedule, cancellation/ownership and atomic
commit remain host/full-validator obligations, not proofs supplied by metadata.
Integer due typing also does not prove calculation/domain provenance; the report
explicitly requires the due and basis calculation to belong to the registered
clock domain, without implicit cross-clock numeric conversion.

## Evidence

Thirteen new Rust groups cover work/wait use metadata; clock protection before a
result write; distinct activity/clock origins without result cells; both timed
wait outcomes; closed fields, due types/facts and required reads; exact outcome
presence/names; retry-clock refusal after earlier stages pass; unvisited sites;
clock participant-requirement dependencies; valid-but-unbound calendars and
wrong clock pins; and source derivation/format/reparse/revision stability including
clock-source actor-policy reads without manual `Also read`. Existing concurrency
tests now also cover an opaque worker overlapping a wait.
Review added a private source-pin/lexical-scope reference cache with charged
copies. A 700-policy case reuses one 4,001-purpose source envelope successfully;
a separate case confirms that a valid child reference proof cannot be reused in
its parent. This caches no live policy truth or permission.

The [five-group Python oracle](../../tools/check_timing.py) independently checks
grammar/schema/JCS/revisions and source/report cycles for timed human work and an
ordinary-connected timed service wait. It checks exact host-use metadata, private
clock refusals after earlier-stage success, explicit retry refusal, unchanged
empty timing evidence for the four no-timer examples, and docs/SPDX links. Rust
provides semantic results; no independent runtime or actual clock was observed.

Earlier negative tests retain their intent with more precise typed errors:
arbitrary clock bytes refuse resource transport decoding and malformed Timer
records refuse closed fields. The clock publication oracle now checks recognized
metadata without execution authority. This explicitly supersedes the first
contract/reader slice's blanket source-clock refusal; its historical evidence is
not a claim that all future integration remains unsupported.

## Open boundaries and reproduce

Valid finite calendar tables still refuse without instance-origin construction
binding; source calendar resources remain unsupported. Active multiple attempts
still refuse without explicit retry clock/basis, even with a timeout or zero delay.
These require dedicated contracts/versioned extensions, not inferred defaults.
Exceptional/timed-work lifecycle, whole observable-channel flow, domain/race,
actual host, full benchmarks, export/visual/usability and Phase 1 gates remain open.

Run workspace tests/format/native lint, semantic/frontend Wasm lint and release
frontend Wasm compilation, plus every existing artifact/stage oracle and
`tools/check_timing.py` in [required CI](../../.github/workflows/rust-portability.yml).
Review pre-write ordering, role isolation, context/dependency closure, current-fact
limits, coverage, privacy, budgets and old no-timer behavior separately; record
delegated acceptance and final-head CI before merge and continue Phase 1.

Local verification passed 175 Rust tests plus five compile-fail doctests, format,
native lint, semantic/frontend Wasm lint and release frontend Wasm compilation,
IR fixture/contract integrity and all authoring/static/control/protection/lifecycle/
invalidation/concurrency/clock/timing Python oracles. Separate completed-diff
automated self-review confirmed clock-before-write ordering, distinct source
roles, retained null-result origins, pure due context, exact wait outcomes,
dependency closure, unvisited/retry/calendar boundaries, visibility-aware caching,
bounded member copies and unchanged no-timer evidence. No open finding remains;
no independent human, real clock/host or browser execution evidence is claimed.
