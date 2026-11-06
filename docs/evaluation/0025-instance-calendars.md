<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Instance calendar construction: evidence

**Date:** 2026-11-06<br>
**Status:** Accepted under delegated review; final-head CI required before merge<br>
**Decision:** [ADR-0025](../decisions/0025-instance-calendars.md)<br>
**Plan:** [frozen scope](0025-calendar-plan.md), `d726624`<br>
**Baseline:** `05424ff687e3ef4cafed983ca6c3fe31a9864087` (PR #32)

## Result and boundary

The known construction profile explicitly interprets finite calendar intervals
in the pinned instance-clock coordinate domain. Closed rule/profile/clock pins
and the exact relative-zone label are validated; unknown/opaque construction
stays refused. The generic Calendar reader remains provenance-only. No previous
core/clock/semantic bytes, wire shape, dependency or old fixture changes.

The resource reader shares immutable validated metadata and offers pure indexed
half-open coordinate membership. It has no observed/authenticated clock or firing
permission. Timer reports expose exact CalendarUse pins and required host
construction/origin binding, current revision and recovery/reschedule judgments.
They create no activation receipt and remain non-executable. Initial host calendar
trust is separate from permission to pause an absolute deadline. Actual host
enforcement, retry timing and the remaining Phase 1 gates stay open.

## Evidence

Four new resource groups cover exact publication and profile/rule/clock identity;
half-open/adjacent/empty/extreme coordinate membership; unknown zones/profiles/
schemas/trust flags and interval order/width; missing/corrupt/opaque rules after
generic metadata reading; and 1,000 repeated reads of a shared 10,000-interval
table. Mathematical membership is not an observed live-domain result.

Four new timer groups cover work/wait CalendarUse metadata and required live
approval/binding; mismatched timer clocks; a deadline that denies pause; and 700
timer policies sharing a 10,000-interval table without live approval caching.
The earlier unknown-calendar regression still refuses, now at the unsupported
construction/zone boundary. No-calendar source behavior stays unchanged apart
from the explicit optional calendar metadata field.

The [four-group Python oracle](../../tools/check_instance_calendar.py) checks
snapshot bytes/digest, grammar/schema/JCS/revision and source/report cycles for
work/wait/empty-table calendars, exact host-requirement references, opaque rules
and mismatched clock refusal, and document/SPDX links. Semantic results come from
Rust; no independent runtime, actual host binding, browser or participant evidence
is claimed by these synthetic cases.

## Reproduce and review

Run workspace tests/format/native lint, semantic/frontend Wasm lint and release
frontend Wasm compilation and every existing artifact/stage oracle plus
`tools/check_instance_calendar.py` in [required CI](../../.github/workflows/rust-portability.yml).
Review exact pin/zone/profile/clock constraints, interval endpoints, cache lifetime,
host/metadata distinction, initial versus modification authority, privacy, coverage
and work bounds separately. Record delegated acceptance and passing final-head CI
before expected-head merge, then continue explicit retry-clock contracts and the
[Phase 1 checkpoint](phase-1-progress.md).

Local verification passed 183 Rust tests plus five compile-fail doctests, format,
native lint, semantic/frontend Wasm lint and release frontend Wasm compilation,
IR/contract integrity and all authoring/static/control/protection/lifecycle/
invalidation/concurrency/clock/timing/calendar Python oracles. Separate automated
self-review confirmed exact immutable pins, closed supported construction,
half-open boundaries, cache scope and work bounds, current-host distinction,
denied-pause deadline support and source/report stability. No open finding remains;
no independent or actual host/browser review is claimed.
