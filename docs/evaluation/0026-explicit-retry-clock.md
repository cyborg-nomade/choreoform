<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Explicit retry clock evidence

**Date:** 2026-11-06<br>
**Plan:** [0026](0026-retry-clock-plan.md), `fe26a4f`<br>
**Decision:** [ADR-0026](../decisions/0026-explicit-retry-clock.md)

## Scope

This slice implements the new core/static/protection/timing contract against
direct IR. Existing source 0.2.0 and its frozen core bytes remain unchanged;
the explicit near-English profile follows in a dedicated PR. The CLI exposes
retryUses with the same non-executable boundary; old source returns an empty set.

`tests/timing/retry.rs` adds seven regression groups for exact publication,
mixed per-record dialect selection, strict shape and numeric/binding checks,
base/new artifact availability/integrity, transitive source actor reads and hidden
basis refusal, independent private timeout/retry/activity origins even without
stored results, unvisited coverage, and shared wide source metadata across 700
policies. Delay-zero and signed-64 maximum metadata are preserved; actual overflow
at a fault stamp remains a host judgment. Earlier unbound-retry tests stay intact.

Synthetic source fixtures are lowered first, then direct IR is explicitly
changed/revisioned for the new profile. This is not new source-profile coverage,
actual retry dispatch, observed fault/key/no-effect evidence or independent review.
No scheduler, authenticating host or runtime is implemented.

## Verification

Passed 190 Rust tests plus five compile-fail doctests, format/native lint,
semantic/frontend Wasm lint and release frontend Wasm build. All relevant
artifact integrity, authoring/profile/frontend and static/control/protection/
lifecycle/invalidation/concurrency/clock/timing/calendar Python oracles passed;
the new retry oracle adds three publication/legacy-boundary/document groups.
Logs during implementation: `/private/tmp/retry-rust.log` and
`/private/tmp/retry-oracles.log`; these local logs are not durable CI artifacts.

Separate completed-diff automated self-review checked per-record version
selection, inherited base availability even without a base dialect alias,
strict fields/pins, contextual visibility/dependencies, source-origin ordering
and coexistence, charged delay copies and immutable metadata caching. No open
finding remains. Fixture corrections retained exact new pins after explicit
source rebinding and used the existing duration grammar; no product behavior
was relaxed to pass tests. ADR-0026 is accepted under delegation. Final-head
required CI and expected-head merge remain necessary; no independent review
or real host/clock/fault evidence is claimed.

## Remaining evidence

The new source sentence, full validator/lifecycle/domain judgments, actual trusted
host binding/enforcement, complete benchmark/conformance and text/visual/user
evidence remain required. No gate is closed by descriptor typing alone.
