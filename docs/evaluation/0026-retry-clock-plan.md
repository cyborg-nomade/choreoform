<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Explicit retry clock binding: frozen plan

**Date:** 2026-11-06<br>
**Baseline:** `3eaf5b1c8871ea0b0a95601e8d0a4616f574f033` (PR #33)<br>
**Authority:** ADR-0018 delegation; ADR-0013 P3/P5 and ADR-0024 follow-up.

Publish ADR-0026 and a separately pinned core profile 0.2.0 that inherits the
exact 0.1.0 contract and changes only the closed work.retry record: explicit
clock Ref and visible basis data ID are required for multiple attempts; both
are null for one attempt. Preserve old bytes, pins, records and diagnostics.
Old retries remain timing-unproved. Do not use annotations, work timeout clocks,
ambient scheduler time or delay-zero exceptions to supply a missing binding.

Type the explicit clock and basis, close clock-source protection and authority
dependencies, retain a distinct pre-write retry-clock information-flow origin
(including null-result work), and expose exact immutable retry uses in the
non-executable timing report. Multiple clocks at one site must not overwrite
one another. Bound metadata work, reuse immutable source metadata, and keep
visibility checks contextual. Require explicit base-contract bytes for the new
profile's inherited meaning.

Specify live retry requirements: accepted eligible fault stamped in the exact
registered retry-clock domain; checked delay addition; original request/input
and basis revisions; current work/target/source authority; no-effect or stable-key
proofs as already required by P3; durable attempt/cancellation/ownership checks.
No automatic retry after success or closure, no cross-clock comparison, no live
fault/source/key proof, dispatch or admission is manufactured by this stage.

Use one PR for core/static/protection/timing integration and direct-IR tests;
follow with a separate authoring profile revision and plain-language retry clause.
Do not silently alter source 0.2.0 or its grammar/companion semantics. This split
must remain explicit in Roadmap and the checkpoint until the source follow-up
is merged. Tests cover exact publication, old/new/mixed dialect boundaries,
strict fields, missing/wrong/hidden clock and basis, transitive source policy
reads, independent timeout/retry origins, null results, visited/unvisited sites,
bounded repeated metadata, and explicit live-host requirements. Run native/Wasm
and relevant artifact/stage checks; self-review separately; require final-head
CI and expected-head merge. Continue remaining Phase 1 work afterward.
