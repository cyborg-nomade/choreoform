<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Supported clock resource and host boundary: frozen plan

**Date:** 2026-11-05<br>
**Baseline:** `83e3493024a2ad0d9dfa50aa1c22a9c9a8621605` (PR #30)<br>
**Authority:** ADR-0018 delegation; ADR-0013 P5 requires a supported clock proposal.

Specify ADR-0023 and an immutable external clock profile implementing P5's origin,
microsecond interpretation, trust-source and monotonicity requirements. Compare
instance-relative observation coordinates, wall-time coordinates and accepting
arbitrary hashed clock bytes. Recommend the narrow instance-relative profile
first: exact registered origin, explicit accepted integer-microsecond observations,
no ambient machine clock, calendar generation or implicit cross-clock conversion.
Team familiarity does not determine this choice.

Use an exact pinned profile artifact and a closed clock resource pointing to an
exact supported read-only provider contract. Define the provider's input/output/
outcome requirements and the host's authenticated source binding, occurrence and
origin identity, durable revision/observation ordering, replay/reset behavior,
monotonic acceptance and timer/authority/atomicity obligations. A digest or typed
descriptor supplies none of that live enforcement. Distinguish profile decoding
from operational host support and from source/definition timer admission.

Freeze the normative profile bytes separately; do not rewrite accepted core/
semantic snapshots, wire fields or old fixtures. Add a bounded immutable resource
reader and positive/negative digest/profile/source/shape/contract regressions.
Unknown profiles, source contract mismatches, malformed flags and arbitrary bytes
must refuse. Retain existing non-null timer and clock/calendar source-resource
refusals during this first contract/reader slice. Complete timer typing, policy
reads, clock-result information flow and host-obligation integration in a later
dedicated slice before enabling it; do not weaken checks just to admit an example.

Document the exact follow-up dependencies and full benchmark/host evidence gaps
in the roadmap/checkpoint. No production engine, real clock adapter, actual clock
observations or participant study is claimed. Run relevant native/Wasm/oracle
checks, review separately, record delegated acceptance and final-head required CI,
merge at the expected head, and continue Phase 1 without an item approval pause.
