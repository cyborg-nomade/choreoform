<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0023: Define a pinned instance-clock resource and host boundary

**Status:** Accepted under Project Owner delegation; effective upon PR merge<br>
**Date:** 2026-11-05<br>
**Decider:** Codex under Project Owner delegation, ADR-0018

## Context and criteria

ADR-0013 P5 requires an implemented immutable clock contract specifying origin,
microsecond interpretation, trust source and monotonicity, but does not prescribe
a provider format. Current source/static stages refuse non-null timers and clock/
calendar source resources. A matching hash of arbitrary bytes is insufficient.

Define a precise first resource profile while preserving those admission limits.
Use explicit accepted observations, exact identity/version binding and existing
provider metadata. Avoid ambient machine time, hidden civil-time conventions,
new dependencies and false host-support claims. Project requirements determine
the choice; owner/team familiarity supplies no candidate advantage.

## Decision

Adopt the separately pinned [instance microsecond observation profile](../clocks/instance-microseconds-v0.1.md).
Its exact [snapshot](../ir/contracts/sha256-8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9.txt)
has contract ID `urn:choreoform:clock-profile:instance-microseconds:0.1.0` and revision
`sha256:8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9`.
It is an external implementation of P5's interface, not a rewrite of frozen core
or semantic snapshots.

A closed clock resource pins this exact profile and a read-only provider contract
with empty-record input, Integer output, exactly `tick` outcome and idempotency
`none`. The profile fixes origin `0` at explicit instance-clock registration,
microsecond coordinates and accepted nondecreasing observations. Domain identity
includes instance, clock resource pin and origin event. No UTC mapping, time-zone
conversion, interpolation, network/machine-clock read or cross-domain comparison
is implicit. Persist source/origin/observation identity and acceptance state across
restart; unknown or untrusted source facts cannot advance time.

The host must implement and authenticate the exact source/domain binding and
truthful coordinate mapping, retain durable ordering/replay state, and enforce
current authority, revision, pause/reschedule/calendar, cancellation/ownership and
atomic commit rules. Resource hashes and decoded metadata prove none of those
live facts. Provider result protection still needs definition-context validation.
No trust/safe flag or authentication scheme is invented.

Add bounded `Resources::clock` returning private immutable `Clock` metadata with
its own pin, profile pin, source pin and source contract. Verify all three artifacts
and closed supported shapes/types before returning it. Unknown profiles, missing/
corrupt pins, arbitrary source bytes and mismatched source contracts refuse. It
performs no IO, clock observation, dispatch or policy authorization.

Keep all existing non-null-timer and clock/calendar source-resource refusals in
this contract/reader slice. Before enabling them, a dedicated follow-up must check
timer typing and full policy/access dependencies, clock-result/control information
flow, exact wait outcomes and calendar/domain links, and explicit host obligations.
Retry delay clock/basis selection must be specified, never inferred from another
timer or scheduler; use a reviewed versioned extension if existing bindings cannot
express it. This ADR and reader do not constitute source or definition admission.

Calendar construction must bind the exact schedule pin to the registered instance
domain/origin; a matching clock resource pin alone does not prove that live
association. The follow-up must specify and enforce that binding before enabling
calendars under this instance-relative profile.

## Options considered

| Option | Requirements fit | Costs/limits | Decision |
| --- | --- | --- | --- |
| Explicit instance-relative microsecond observation profile | Precise origin/order/source, fits P5 and existing provider metadata | No civil-time mapping or actual host clock; domain binding still mandatory | Adopt first |
| Wall-time coordinate profile first | Can express civil deadlines directly | Requires explicit origin/scale/discontinuity/conversion/trust rules and evidence beyond current contracts | Later dedicated supported profile if required |
| Support both immediately | Broader notation | Duplicates unproved host/domain work and risks implicit conversion | Defer |
| Accept any matching clock hash / ambient host time | Small implementation | No implemented semantics, trust or monotonicity proof | Reject |

## Consequences

Clock metadata now has an exact implemented interpretation and immutable
provenance. Resource verification uses existing registry/transport/work limits
and provider cache, with no new dependency or mutable admission credential.
Existing fixtures, wire/core versions, old snapshots and stage outputs remain
unchanged. A reader can decode a descriptor without enabling a single timer.

Actual source/host enforcement, timer integration, calendar construction, retry
clock bindings, race/lifecycle/invalidation proofs, complete benchmark/conformance,
source/visual parity, user studies and Phase 1 exit remain open. Synthetic resource
tests are not real clock observations, host/browser execution or independent review.

## Confirmation and action items

Follow [the plan](../evaluation/0023-clock-contract-plan.md), committed `b670f50`,
and [evidence](../evaluation/0023-clock-resource-contract.md). Required CI and
separate delegated automated review precede expected-head merge.

Accepted after separate automated self-review under the owner's 2026-11-05
delegation. Local verification passed 162 Rust tests plus five compile-fail
doctests, native/Wasm checks and all relevant artifact/stage oracles. Required
final-head CI remains a merge prerequisite. No independent human or individual
owner review, real clock observations or supported host is claimed.

1. [x] Complete contract/reader and positive/negative integrity/source tests.
2. [x] Record automated self-review and delegated acceptance; require final-head CI before merge.
3. [ ] Complete typed timer/clock-flow/host-obligation integration before enabling timers.
4. [ ] Complete benchmark and actual host/phase evidence; do not infer gate closure.
