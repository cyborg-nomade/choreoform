<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0024: Type supported timers and expose their host obligations

**Status:** Accepted under Project Owner delegation; effective upon PR merge<br>
**Date:** 2026-11-05<br>
**Decider:** Codex under Project Owner delegation, ADR-0018

## Context and criteria

ADR-0023 supplies a pinned clock metadata profile, but its reader alone cannot
justify timer support. P5 also requires typed due/basis/authority dependencies,
exact wait outcomes, protected clock observations and live host enforcement.
A clock can influence whether work publishes a result, not only which later
branch runs. Multiple source kinds at one node must retain distinct origins.

Implement those static/ordinary judgments with honest runtime boundaries. Avoid
ambient timer facts, arbitrary trust flags, implicit retry-clock selection and
unproved instance/calendar associations. Preserve existing immutable artifacts,
source syntax and no-timer behavior, and make unsupported cases explicit.

## Decision

Recognize only ADR-0023's exact implemented clock profile in source resources and
definition timers. All resources remain explicitly supplied and verified. Type
the closed Timer record: due is an Integer process expression with no policy-only
facts/parameters, basis is visible data, and pauseAuthority is an actor requirement.
Include due/basis reads and transitive pause/source protection/participant/
capability policy reads in the owning policy closure and node declarations.
Source lowering derives the same closure without manual `Also read` bookkeeping.

For waits, onTimer exists exactly when timer exists, differs from onObservation,
and both names comprise the exact node outcome set. Ordinary flow checking
requires corresponding successors. Work timeout observations remain subject to
P1/P4/P5 fault/ownership rules; this stage does not invent a success-on-timeout
outcome, settle unknown work or prove exceptional routing.

Keep clock and activity provider origins in separate typed roles. Add clock
origins before checking result writes whose eligibility depends on timing, retain
both through ordinary completion/merges and preserve them without result cells.
Validate source protection references in definition context; actual source/result
access and current authority remain live host judgments. Use cached provider
objects and borrowed node lookups so absent source kinds allocate no origin keys.
Cache validated provider protection references by source pin and lexical scope,
charging copied members. This avoids repeatedly validating wide immutable source
envelopes while preventing a child visibility proof from escaping to its parent.
No predicate truth or current authority is cached.
Classify waits as external observations in conservative concurrency summaries:
opaque/non-read-only parallel work cannot ignore a concurrent wait merely because
the wait publishes no cell. This fixes an observation-overlap omission.

Add explicit `check_timing` / `Candidate::check_timing` / CLI `check-timing` after
prior analyses. Return private immutable revision-bound TimerUse records with
site, policy, timeout/timer role, clock/source pins, basis, pause authority and
required host judgments. Expose checked/unvisited timed sites and ordinary
coverage/input assumptions, always `executable:false`. No clock observations or
current facts are inputs to this interface; it cannot establish elapsed time,
authenticate a source, authorize dispatch or commit a timer fire.
Integer typing alone does not prove that the due/basis calculation belongs to
the selected registered clock domain. That provenance/domain judgment is an
explicit host requirement; do not infer a cross-clock conversion from numeric
type equality.

## Supported boundaries and follow-ups

Non-null calendars remain refused as `calendar-domain-binding-unproved` after
finite-table/pin validation and clock-pin matching. Matching clock resource pins
do not bind a schedule to a registered instance origin. Calendar source-resource
lowering remains unsupported pending that explicit construction/binding contract.

Attempts beyond one lack an explicit retry clock/basis in the accepted payload.
The timing stage refuses active uses as `retry-clock-basis-unproved`, even with
zero delays or a work timeout. It never borrows that timeout's clock or scheduler
time. Specify a reviewed versioned extension if current fields cannot express the
binding. Unvisited retry/timer sites remain uncovered, not silently proved.

Host requirements include authenticated source/instance/domain/origin binding,
truthful microseconds, durable monotonic/replay/observation identity, exact timer/
occurrence/revision correlation, current source-result and basis access, pause
authority, overflow/reschedule/pause checks, and atomic fire versus cancellation
and owned settlement. Provider-record policy facts require the actual protected
observation context, not invented data-cell names. No descriptor or requirement
list supplies real host support. Timed-work exceptional/lifecycle proofs remain.

## Options considered

| Option | Benefit | Cost/risk | Decision |
| --- | --- | --- | --- |
| Typed supported timers, clock-origin flow and explicit runtime obligations | Useful precise subset without false execution admission | Calendar/retry and host/exceptional judgments remain | Adopt |
| Enable timers after resource hashing only | Small implementation | Misses types, dependencies, information flow and live trust | Reject |
| Add clock labels after result writes / share one provider origin per node | Less code | Can lose source labels or miss timing-dependent publication | Reject |
| Treat calendar pin matches or work timeout as missing domain/retry bindings | Broad apparent coverage | Invents origin and clock-selection meaning | Reject |
| Implement a production timer engine now | Would provide runtime operations | Phase 2 scope; cannot substitute for this contract work | Defer |

## Consequences

Supported non-calendar, single-attempt timer definitions can pass explicit partial
stages. Unknown clock bytes now fail typed resource decoding; malformed timer
records fail closed-field checking rather than a blanket unsupported code. No
wire/core/semantic snapshot, grammar, dependency or old fixture changes. Earlier
stages remain non-executable; full validator/host, domain invariants, exceptional
flow, calendars/retries, benchmarks/conformance, source/visual parity, actual user
studies and Phase 1 exit remain open. No real clock/browser/host evidence is implied.

## Confirmation and action items

Follow the [frozen plan](../evaluation/0024-timer-integration-plan.md), committed
`36630bb`, and [evidence](../evaluation/0024-timer-contract-checks.md). Review the
completed diff separately; require final-head CI before expected-head merge.

Accepted after separate automated self-review under the owner's 2026-11-05
delegation. Local verification passed 175 Rust tests plus five compile-fail
doctests, native/Wasm checks and all relevant artifact/stage oracles. Review
retained pre-write clock origins, isolated source roles, counted wait observations
and added scoped immutable metadata caching with visibility/budget regressions.
Required final-head CI remains a merge prerequisite. No independent human or
individual owner review, actual clock/host execution or phase closure is claimed.

1. [x] Complete tests/source cycles, native/Wasm checks and separate automated review.
2. [x] Record delegated acceptance; verify final-head required CI before merge.
3. [ ] Specify calendar-domain and retry-clock bindings and complete remaining gates.
