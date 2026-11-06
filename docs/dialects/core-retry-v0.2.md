<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Core profile 0.2.0: explicit retry clock and basis

Contract identity: `urn:choreoform:contracts:core-profile:0.2.0`.
This document is a separately pinned normative extension of core profile 0.1.0,
`urn:choreoform:contracts:core-profile:0.1.0` at
`sha256:26eb773b4444f5af7c4ec3406f46ab7da95e87457bf7b0c2bfbea36ea3bbcb58`.
Its exact base artifact must be available and verified together with this one.
All base type, expression, policy and lifecycle meanings remain in force except
the following closed retry-record replacement. Envelope and semantic-contract
versions do not change. Records select their dialect explicitly; a 0.1.0 work
policy keeps its old record even when a 0.2.0 alias also exists in the definition.

## Retry record

A work policy in this profile uses exactly:

`{maxAttempts:N,delays:List<Duration>,faults:Set<Name>,clock:null|Ref,basis:null|ID}`.

N remains a JSON integer 1..1000 counting the initial attempt. Delays has exactly
N-1 canonical nonnegative signed-64 microsecond coefficient strings, with delay
i-1 before attempt i+1. Fault names remain exact. When N is 1, clock and basis
must both be null. When N exceeds 1, both must be non-null: clock pins the clock
resource and basis references a visible data cell whose revisions participate
in the activation identity. There is no default clock, basis, jitter, backoff,
implicit calendar or pause/reschedule permission. Zero delay still requires the
explicit binding and all current judgments.

The implemented clock contract is instance-microseconds 0.1.0 at
`urn:choreoform:clock-profile:instance-microseconds:0.1.0`,
`sha256:8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9`.
Unknown clock profiles remain unsupported. Retry and work-timeout clocks may
differ; neither supplies the other's binding and their integer coordinates
must never be compared or converted implicitly.

The retry basis read and transitive protection/actor/capability policy reads of
the retry-clock source belong to work.reads. Retry-clock observation is a distinct
protected pre-result influence on activity scheduling, results and successors,
including work that stores no result. Its source must not replace a work-timeout
or activity-provider influence. A pin establishes immutable metadata integrity;
it establishes no truthful observation, live domain, current access or authority.

## Live judgments before retry dispatch

The trusted host durably binds exact definition revision, work policy, activity
occurrence/logical effect, original request digest and input revision, attempt
number, retry clock/source pins and trusted registration binding, process
instance, origin event and basis revisions. Expected domain and origin come from
trusted registration, not from an untrusted fault or uploaded descriptor.

The preceding eligible fault must have been accepted and correlated to that
exact attempt. At fault acceptance the host must accept a current authenticated
observation in the registered retry-clock domain and retain its microsecond stamp
with the fault; a provider's asserted wall time is insufficient. Due is this
stamp plus the declared delay for the next attempt, using checked signed-64
addition. Overflow refuses scheduling and enters owned fault/reconciliation
handling. Delay is elapsed only on accepted monotonic observations in that same
domain. A zero delay does not bypass fault, source, basis or authority checks.

At dispatch, original request/input and bound basis revisions must still match;
changed inputs require a new activity occurrence and logical effect. Recheck
current work authority, target human/capability authority, source protection
and access, eligible fault, attempt budget and cancellation/ordinary closure.
Preserve every base P3/P4 effect judgment: unresolved effects require stable-key
support with identical request digest or definitive correlated no-effect evidence.
Succeeded effects never retry. Definitive rejected/cancelled attempts retry only
when the exact fault and confirmed no-effect evidence satisfy the base rule.
Timing evidence alone never proves no effect or idempotency.

Dispatch/attempt increment, cancellation, late success, settlement and ownership
must be serialized atomically and recovered durably without resetting the
attempt budget, clock origin, original identity or due stamp. Contradictory/late
facts retain their history and route to owned reconciliation. Ordinary closure
must not accidentally restart abandoned work. No new clock rescheduling API,
calendar gating, fault-data port or engine implementation is defined here.

Static checks expose exact references and these host obligations only. They
accept no trusted-looking boolean or synthetic receipt as proof of live facts,
return no executable permission and do not close Phase 1 exit gates. A new source
profile must explicitly express the retry binding; old source/contract bytes
must not silently acquire the fields or a timeout/scheduler-clock default.
