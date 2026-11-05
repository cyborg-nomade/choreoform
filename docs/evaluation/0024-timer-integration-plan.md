<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Timer typing, clock flow and host obligations: frozen plan

**Date:** 2026-11-05<br>
**Baseline:** `47084ce231abd51d6e3c0194785673347756d6c0` (PR #31)<br>
**Authority:** ADR-0018 delegation; ADR-0013 P5 and ADR-0023 clock integration.

Integrate the exact instance-clock profile into supported static/source timer
checking without admitting execution. Validate closed Timer fields, due Integer
process expressions (no invented timer facts), visible basis and pause authority,
clock source protection/authority references and all transitive policy/access
reads. Check the exact observation/timer outcome set, distinct names and presence
rules on waits. Source lowering must derive identical dependencies and recognize
only the implemented pinned clock profile, retaining exact supplied resources.

Propagate clock-source result protection through timed outcomes and before any
result write whose eligibility depends on timing. Use typed provider-origin roles
so an activity result and clock source at the same node cannot overwrite one
another. Retain clock origins even without result publication; do not drop them
at a merge or guess current clock facts. Earlier-stage payload success must precede
new information-flow refusals in regression evidence.

Add an explicit timing-report stage after prior analyses: immutable revision-bound
timer uses with site/policy/clock/source/basis/pause-authority and host obligations,
ordinary coverage and unvisited timed sites, always non-executable. This describes
required live source/domain/origin, revision/authority, monotonic/replay/pause/
reschedule and atomic fire/cancellation enforcement; it does not authenticate or
observe a clock, grant access, dispatch work, or implement the Phase 2 engine.

Keep non-null calendars refused as domain-binding-unproved; a matching clock pin
does not bind a schedule to an instance origin. Keep calendar source-resource
lowering unsupported pending that contract. Retry attempts beyond one lack an
explicit clock/basis binding in the accepted payload: the timing stage must refuse
them as unproved, not borrow a work-timeout clock or infer scheduler timing. Record
these and exceptional/timed-work lifecycle gaps explicitly in ADR-0024, roadmap
and evidence; use reviewed versioned extensions if existing bindings cannot suffice.

Preserve old wire/core/semantic artifacts, fixtures and no-timer behavior. New
positive/negative source/IR tests cover strict fields/types/refs, both wait outcomes,
clock metadata dependencies, private clock sources (including null results and
activity/clock-origin coexistence), known/unknown resource pins, coverage, source
format/reparse/revision stability and retained calendar/retry refusals. Budget new
work, run required native/Wasm/oracle checks, review separately, record delegated
acceptance, require final-head CI and expected-head merge, then continue Phase 1.
