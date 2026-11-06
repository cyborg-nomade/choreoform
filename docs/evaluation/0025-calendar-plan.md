<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Instance calendar construction and origin binding: frozen plan

**Date:** 2026-11-06<br>
**Baseline:** `05424ff687e3ef4cafed983ca6c3fe31a9864087` (PR #32)<br>
**Authority:** ADR-0018 delegation; ADR-0013 P5 and ADR-0023/0024 calendar follow-up.

Define ADR-0025 and a separately pinned profile for explicit finite interval tables
in the implemented instance clock's microsecond coordinates. Use the existing
Calendar wire shape and its zoneRules Ref for a closed supported construction-rule
artifact, with exact profile/clock pins and an explicit instance-relative zone
meaning. Do not reinterpret arbitrary zone-rule bytes, generate civil calendars,
invent next-day windows or change frozen core/clock/semantic artifacts.

Specify the live activation binding: exact definition/timer/clock/calendar/rules
pins, process/clock origin and timer/basis revisions, trusted host construction
approval and durable correlation. The host derives expected origin from its trusted
registration, not the schedule's assertions. Initial construction approval belongs
to the host's configured calendar trust contract; do not substitute pause authority
and accidentally prohibit an absolute deadline whose pause policy denies changes.
Calendar changes still require explicit authorized reschedule and fresh binding.

Implement a bounded immutable metadata reader for this known profile and pure
half-open coordinate membership. Preserve the generic Calendar provenance reader
as metadata-only. Cache shared interval/rule metadata without caching live approval,
domain binding or policy truth. Unknown construction/profile/zone/clock combinations
must refuse. No live receipt, clock observation or executable permission is produced.

Integrate recognized calendar resources into supported source/timer typing and
timing-use metadata with explicit construction/origin host requirements. A supported
mapping contract can be type checked; actual activation binding/enforcement remains
a required host judgment before execution. Keep unknown/opaque calendars refused
and retain unbound retry-clock refusals. Preserve old fixtures, wire versions,
existing APIs and no-calendar behavior.

Tests: exact profile/rule/clock/source bytes and schemas; wrong/missing pins and
trust flags; sorted finite/empty/half-open/extreme coordinate cases; repeated wide
tables; mismatched timer clocks; source grammar/schema/revision/report cycles;
coverage and explicit live-host requirements. Synthetic membership/source tests
are not observed host binding, real clocks or participant evidence. Run required
native/Wasm/artifact/stage checks, self-review separately, record delegated
acceptance, require final-head CI and expected-head merge, then continue Phase 1.
