<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Clock resource contract: evidence

**Date:** 2026-11-05<br>
**Status:** Accepted under delegated review; final-head CI required before merge<br>
**Decision:** [ADR-0023](../decisions/0023-clock-resource-contract.md)<br>
**Plan:** [frozen scope](0023-clock-contract-plan.md), `b670f50`<br>
**Baseline:** `83e3493024a2ad0d9dfa50aa1c22a9c9a8621605` (PR #30)

## Result and boundary

The first external clock profile fixes explicit instance-origin identity,
microsecond coordinates, authenticated source binding and durable nondecreasing
accepted observation rules. Its [normative source](../clocks/instance-microseconds-v0.1.md)
and [snapshot](../ir/contracts/sha256-8769ccd8eda0a11074662ad1d36aa843f1f951e15640e51beb30d1ab4b43e9c9.txt)
are byte-identical at the published digest. Old core/semantic artifacts and fixture
revisions are unchanged. This does not introduce a wall-time or implicit machine
clock, calendar generation, signature protocol, source authentication or engine.

The immutable resource reader verifies the clock, exact known profile and exact
read-only Integer tick provider. It preserves source result-protection metadata
for later definition-context validation. No callback/IO/time read or authority
grant occurs. Existing non-null-timer and source clock/calendar refusals remain.

## Tests

Four new Rust groups cover exact profile/source/pin binding and immutable getters;
unknown versions/IDs/profiles/shapes/extra trust flags; input/output/outcome/effect/
idempotency/observation mismatches; and missing/corrupt clock/profile/source bytes
or arbitrary correctly hashed clock/source data. Source provider caching never
substitutes for the clock-specific contract checks. Existing generic registry/
transport/depth/work bounds remain in force.

The [Python artifact oracle](../../tools/check_clock_contract.py) checks the
published snapshot's exact bytes/digest and documentation/licensing links, and
confirms that supplying the new metadata format does not enable source lowering.
Semantic reader results come from Rust tests, not an independent Python runtime.

## Required follow-up before timer support

1. Type due/basis/pause authority, close their policy/access reads and handle both
   exact wait outcomes. Preserve explicit clock/calendar/domain pins.
2. Propagate clock-source result protection through timed outcomes/dispatch and
   record real current-source/domain/authority/atomicity host obligations.
3. Specify retry delay clock/basis binding, using a versioned extension if needed;
   do not borrow another timer's clock implicitly.
   Bind calendar construction to the registered instance domain/origin; matching
   resource pins alone cannot establish the live schedule association.
4. Test positive/negative source/IR timer packages and conformance traces, then
   actual supported-host behavior before claiming execution support. Complete
   benchmarks, source/visual parity, user evidence and Phase 1 gates remain open.

Run workspace tests/format/native lint, semantic/frontend Wasm lint and release
frontend Wasm build, every existing stage oracle and this artifact oracle in
[required CI](../../.github/workflows/rust-portability.yml). Review closed fields,
exact immutable pins, source constraints, metadata/host distinction and unchanged
admission limits separately; record delegated acceptance and final-head CI before
merge, then continue Phase 1.

Local verification passed 162 Rust tests plus five compile-fail doctests, format,
native lint, semantic/frontend Wasm lint and release frontend Wasm compilation,
IR fixture/contract integrity and all authoring/static/control/protection/lifecycle/
invalidation/concurrency/clock Python oracles. Separate completed-diff automated
self-review checked closed fields, exact profile/source identity, source type/
effect/outcome constraints, immutable metadata/cache boundaries, registry/work
limits, unchanged admission and the explicit calendar/retry/host follow-ups.
No open finding remains and no independent or actual host/browser review is claimed.
