<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0025: Define instance calendar construction and activation obligations

**Status:** Accepted under Project Owner delegation; effective upon PR merge<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under Project Owner delegation, ADR-0018

## Context and criteria

P5 calendars contain finite coordinate tables and pinned clock/zone provenance.
ADR-0024 deliberately refuses them without a supported construction/domain
contract. A shared clock-resource hash alone cannot identify a live instance
origin, and an arbitrary uploaded schedule cannot approve its own construction.

Define a narrow implemented mapping with exact provenance, no civil-time guessing,
bounded membership and explicit host activation obligations. Preserve the existing
wire shape, immutable artifacts and generic metadata-reader boundary. Support an
absolute deadline whose pause policy intentionally denies modifications.

## Decision

Adopt [instance calendar intervals 0.1.0](../clocks/instance-calendar-v0.1.md), pinned
as `urn:choreoform:calendar-profile:instance-intervals:0.1.0` at
`sha256:2ab8a6a78f2829ababa2d023f6743ac82d7a31acbfca1fb7a03340b7c929f35f`.
The [exact snapshot](../ir/contracts/sha256-2ab8a6a78f2829ababa2d023f6743ac82d7a31acbfca1fb7a03340b7c929f35f.txt)
separately implements the P5 construction interface without changing core or
clock semantics. Calendar `zone` must be `instance-relative`; `zoneRules` pins a
closed `calendar-rules` artifact whose exact profile and clock references match
the implemented profile and calendar. The timer must use that same clock.

Intervals deliberately represent microsecond windows relative to the registered
instance-clock origin. Keep sorted non-overlapping half-open signed-64 bounds,
adjacency and empty tables. No UTC/business-hour/holiday/time-zone generation,
clock substitution, next-day fallback or horizon extension is inferred. Unknown
profiles, zones, opaque rules, trust flags and pin mismatches refuse.

Add `Resources::instance_calendar` returning shared immutable known construction
metadata. Keep `Resources::calendar` as the generic provenance-only reader. Cache
validated interval/rule metadata by exact pin; cache no approval, live identity
or policy truth. `contains_coordinate` performs binary-indexed mathematical
membership only and has no clock observation, due check or firing authority.

Integrate this known profile into source/static timers and immutable CalendarUse
metadata under each TimerUse: calendar, rule and profile pins with required live
host judgments. The supported mapping contract can now be type checked; a live
binding has not been created. Reports remain non-executable and accept no current
host facts. Unbound retry-clock uses remain refused.

## Host activation boundary

Before activation the host must authenticate construction approval under its
configured calendar trust contract and durably bind exact definition, timer
declaration/occurrence/revision, process instance, clock pin, registered origin,
source-binding identity, calendar/rule/profile pins and basis/input revisions.
Expected origin comes from trusted clock registration, not the schedule's claims.
The profile specifies refusal/recovery rules for stale, wrong-domain or missing
bindings and requires the usual clock/access/authority/cancellation/ownership
checks at firing. A decoded descriptor or matching asserted fields is insufficient.

Initial construction approval belongs to the host's calendar trust contract,
not pauseAuthority. The latter controls requested pause/resume/reschedule changes;
it need not permit pausing in order for an absolute deadline to exist. Calendar
changes require explicit currently authorized reschedule, a new timer revision
and a fresh binding. No runtime binding/approval receipt is manufactured here.

## Options considered

| Option | Benefit | Cost/risk | Decision |
| --- | --- | --- | --- |
| Pinned relative construction and explicit activation obligations | Precise finite mapping, fits P5, preserves origin/host distinction | No civil-time generator or actual host enforcement | Adopt |
| Accept any sorted table with matching clock hash | Small | No supported construction or live origin association | Reject |
| Use pause authority for initial construction approval | Reuses an actor slot | Breaks absolute deadlines whose pause policy denies changes | Reject |
| Add implicit civil conversion/future-window generation | Appears convenient | Invents calendar rules, trust and time-zone semantics | Reject |
| Defer every calendar until a production engine | Small implementation | Prevents useful contract/source/conformance work in Phase 1 | Reject; preserve runtime boundary |

## Consequences and confirmation

Known relative calendar definitions can pass partial static/source/timing stages.
Unknown calendar construction stays refused. Actual construction approval, origin
binding, due/domain provenance and host/atomic enforcement remain mandatory before
execution, together with retry, exceptional/lifecycle/domain proofs and remaining
benchmark, round-trip, protected-accessible editing and user-study gates. Phase 1
remains incomplete; no real host/browser/clock evidence is claimed.

Follow the [frozen plan](../evaluation/0025-calendar-plan.md), committed `d726624`,
and [evidence](../evaluation/0025-instance-calendars.md). Require separate automated
review, passing final-head CI and expected-head merge under ADR-0018 delegation.

Accepted after separate automated self-review under the owner's delegation.
Local verification passed 183 Rust tests plus five compile-fail doctests, native/
Wasm checks and all relevant artifact/stage oracles. Required final-head CI remains
a merge prerequisite. No independent human or individual owner review or actual
host binding/clock/browser observation is claimed.

1. [x] Complete implementation, exact-artifact/membership/source/budget regressions.
2. [x] Record delegated acceptance/review; require final-head CI before merge.
3. [ ] Continue explicit retry-clock and remaining full-validator/phase contracts.
