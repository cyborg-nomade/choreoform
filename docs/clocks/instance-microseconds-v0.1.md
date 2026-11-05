<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Instance microsecond observation clock, version 0.1.0

**Contract ID:** `urn:choreoform:clock-profile:instance-microseconds:0.1.0`

This external clock profile implements the clock-resource boundary in ADR-0013
P5. It does not change the accepted core-profile 0.1.0 contract or standardize a
machine clock API. This file's exact stored bytes are the profile artifact; a
clock resource pins its ID and SHA-256 revision. No digest inside this file binds
the file to itself. Changed normative bytes require a new explicit binding.

## Resource shape

A clock resource is the closed JSON record:

```
{kind:"clock", version:"0.1.0", id:Name, profile:Ref, source:Ref}
```

`Ref` is exactly `{id:Name,revision:"sha256:" + 64 lowercase hexadecimal digits}`.
`id` must equal the supplied resource identity. Verify the exact clock, profile
and source bytes in an explicit bounded registry; a URI does not permit fetching.
`profile` must identify this exact implemented profile artifact. Unknown profiles,
fields, versions, malformed reference/set flags and arbitrary hashed bytes refuse.

`source` pins a supported provider-contract 0.1.0 with empty-record input, Integer
output, exactly the ordinary outcome `tick`, effectClass `read-only` and idempotency
`none`. Its observations retain the existing provider contract rules: every
succeeded tick carries the exact Integer result type; unknown/rejected/cancelled
states do not carry a successful result or advance the clock. The result's integer
is a canonical signed-64 coefficient string. Provider result protection and its
definition-context references remain subject to the usual access/flow checks.
This describes accepted observations, not permission to dispatch the provider.

## Coordinates and identity

Each process instance explicitly registers one clock-origin event for this clock
resource pin before using its coordinates. The origin instant is `0`; one unit
denotes one elapsed microsecond from that event according to the bound source.
The domain identity includes the process instance, clock resource pin and exact
origin event identity. Distinct origin events or process instances are distinct
domains even if their resource pins and integer values match. Origins cannot be
silently reset or reused. This profile supplies no UTC/civil-time mapping, leap
rule, time-zone conversion, inferred drift correction or cross-domain comparison.

Only explicit accepted source observations advance the known instant. There is
no core call to a local machine clock, interpolation while waiting, permission to
read a network service, or assumption that lack of observations means no time
has elapsed. The source's unit and origin mapping are host-enforced contractual
facts, not properties that a digest can prove. Timer expressions still return
Integer coordinates in this domain; negative due coordinates are representable
but do not make a backwards clock observation acceptable.

## Acceptance and durable ordering

Before acceptance the host authenticates the source binding and verifies the
exact clock/domain, instance, origin and observation identity. An arbitrary
host-supplied name, uploaded descriptor or Boolean flag is not authentication.
The source must be implemented and explicitly trusted for this clock by the host's
execution contract. Do not infer support from successful resource decoding.

Persist origin registration, source binding, accepted observation identities,
instant and engine acceptance positions atomically with dependent transitions.
The known instant initially equals `0`. A new observation must be in signed-64
range and at least the last accepted instant. Different authenticated identities
may report the same instant. A repeated identity does not advance or fire again;
backwards, stale, wrong-domain or unauthenticated facts remain recorded/refused.
Provider timestamps do not override engine acceptance position or cancellation.
This profile does not define a signature scheme or confer trust on provider bytes.

After restart, retain the same domain, origin and last accepted state. Recover
the source mapping and authenticate new observations before advancing; if that
cannot be established, dependent timers remain pending. Do not guess an origin,
substitute a wall clock, classify an unknown effect as failed, or reopen a terminal
instance. A distinct process instance registers its own explicit domain.

## Timers, calendars and host obligations

P5 timer revision, basis/due dependencies, pause authority, non-overlapping exact
pause sums, overflow refusal, reschedule invalidation, current revision checks,
calendar interval gates and atomic cancellation races remain mandatory. A tick
at/after effective due establishes timing eligibility only; it cannot authorize
or commit a transition on its own. Unknown source observations leave timing
pending. Pausing a timer never pauses or resets this clock domain.

A calendar must pin this same clock resource and contain immutable precomputed
intervals for the registered domain. Its provenance/host construction still need
support; zone strings do not create civil-time interpretation. This profile does
not generate intervals, choose a retry clock/basis, or fill another missing policy
binding. Such integration must be explicitly specified and checked.

The host must enforce truthful microsecond coordinates, authenticated provider/
domain binding, current authority/protection, exact occurrence/revision identity,
durable monotonic ordering, replay/reset handling, sealed membership, ownership
and atomic commits. A descriptor reader proves none of these operational facts.
No execution capability, authenticated claim, real clock adapter or safe flag is
part of this resource. Library/CLI timer admission requires the additional typed
policy, information-flow and host-obligation judgments; descriptor support alone
must not remove existing unsupported-timer refusals.
