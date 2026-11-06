<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Instance calendar intervals, version 0.1.0

**Contract ID:** `urn:choreoform:calendar-profile:instance-intervals:0.1.0`

This external construction profile implements finite P5 calendar tables for the
instance microsecond observation-clock profile 0.1.0. Exact profile bytes are
separately pinned by identity and SHA-256; there is no self-digest in this file.
The required clock profile is
`urn:choreoform:clock-profile:instance-microseconds:0.1.0` at
`sha256:8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9`.
It does not alter frozen core/clock/semantic contracts or provide a civil-time
calendar generator. Changed normative bytes require a new explicit binding.

## Immutable resources

Retain the P5 closed Calendar resource:

```
{kind:"calendar",version:"0.1.0",id:Name,clock:Ref,
 zone:"instance-relative",zoneRules:Ref,
 intervals:List<{start:Integer,end:Integer}>}
```

The zoneRules Ref pins a closed construction-rule artifact:

```
{kind:"calendar-rules",version:"0.1.0",id:Name,profile:Ref,clock:Ref}
```

Ref is exactly `{id:Name,revision:"sha256:" + 64 lowercase hexadecimal digits}`.
Verify exact calendar, rule, profile and clock/source bytes in the explicitly
supplied bounded registry. Resource IDs must match. The rule profile must be this
exact implemented contract; its clock must equal the calendar clock, whose profile
must be the exact implemented instance-microsecond clock contract. A timer using
the calendar must pin that same clock. Unknown profiles, extra fields/trust flags,
opaque rule bytes, unsupported zones or mismatched pins refuse.

Intervals are deliberate finite origin-relative coordinate windows. Bounds are
canonical signed-64 coefficient strings, in microseconds from registered clock
origin `0`. Require start < end, sorted non-overlapping half-open ranges. Adjacent
ranges are permitted. An empty table contains no eligible coordinate. Negative
bounds are representable; they never authorize a backwards clock observation.

`instance-relative` is an exact coordinate/provenance label, not a time zone or
permission to reinterpret UTC/civil dates. This profile supplies no business-hour,
holiday, daylight-saving, leap, locale or wall-time conversion algorithm. Such
construction needs a separately supported profile/host contract. Never infer the
next day, extend the horizon or add an interval absent from the pinned table.

## Live activation binding

A table is a template for a registered clock domain, not a self-authenticated
claim about a live instance. Before enabling its timer, the host records a durable
activation binding containing at least:

- exact definition revision and timer declaration/occurrence/revision;
- process-instance identity, clock resource pin and registered origin event;
- current clock-source binding identity;
- calendar, construction-rule and construction-profile pins;
- the exact basis/input revision identities used for due calculation;
- trusted host execution-contract identity and construction-approval observation.

The host obtains expected instance/origin/source identity from its trusted clock
registration and execution context, not from the schedule's assertions. It must
authenticate the construction approval under its configured calendar trust
contract and support this exact coordinate profile. Matching hashes, asserted
names, matching fields or a Boolean cannot establish trust or approval. Resource
decoding and coordinate membership do not create this binding.

Initial construction approval belongs to the host's calendar trust contract. Do
not substitute the timer's pause policy: P5 permits an absolute deadline whose
pause/resume/reschedule authority denies changes. Subsequent calendar changes do
require explicit reschedule under the current pauseAuthority, a new timer revision
and a fresh activation binding. Never mutate a running timer's table implicitly.

Require exact binding/current identities at firing together with P5's authenticated
nondecreasing clock observation, effective due, current basis, no active pause,
current access/authority, ownership and atomic cancellation/commit checks. Replay,
wrong-origin, wrong-instance, wrong-clock, stale-timer/basis/calendar or unsupported
construction facts cannot enable a fire. Retain refused and historical records.
Persist/recover the binding across restart; missing trustworthy recovery leaves
the timer pending, without origin reset or guessed calendar replacement.

## Coordinate gate and evidence boundary

For a coordinate x in the registered domain, membership means exactly that one
pinned interval satisfies start <= x < end. Sorted tables allow indexed lookup
without scanning every interval. Membership is only one mathematical guard;
it proves neither that x was observed/authenticated nor that the timer may fire.
Outside all supplied future intervals the timer remains pending indefinitely
unless an explicit authorized reschedule supplies a new pinned calendar.

An implementation may expose immutable decoded metadata and pure membership for
explicit coordinates. Those operations perform no IO, source authentication,
current policy evaluation, live binding, calendar generation or timer dispatch.
Static/source reports must expose the exact rule/profile/calendar references and
required live construction/origin judgments and remain non-executable. Actual
host enforcement and execution admission require supported trusted runtime
boundaries; synthetic resource/membership tests are not observations of them.
