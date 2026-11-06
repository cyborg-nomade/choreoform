<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Explicit retry authoring evidence

**Date:** 2026-11-06<br>
**Plan:** [0027](0027-retry-authoring-plan.md), `474cc35`<br>
**Decision:** [ADR-0027](../decisions/0027-explicit-retry-authoring.md)

Five new Rust groups exercise full new-profile one/multi-attempt cycles,
unchanged identities/tombstones, exact source/companion/core pairings, legacy
grammar/fragment refusal, binding presence and reference roles, transitive
clock-source authority reads, explicit inherited artifact availability and
token bounds in both versioned grammars. Direct-IR retry/source-origin and wide
metadata tests from ADR-0026 remain in force.

Six independent Python groups cover grammar/schema/JCS/revision/integrity and
format/report cycles; strict version/pin pairs; clause requirements; independent
timeout/retry clock references; a coordinated alias edit preserving IR and
permanent rows; and documentation/licenses plus the exact grammar delta. They
use synthetic fixture packages, not observed users, hosts or a production editor.

## Verification

Passed 195 Rust tests plus five compile-fail doctests; format/native lint,
frontend Wasm lint/release build and all relevant artifact, authoring/profile/
frontend and static/control/protection/lifecycle/invalidation/concurrency/clock/
timing/calendar/retry Python oracles. Six new independent Python groups passed.
Two old unknown-version negatives now use 0.4.0 because 0.3.0 is explicitly
supported; the old source files, schema, grammar and expected digest vectors are
unchanged. Local logs: `/private/tmp/author-retry-rust.log`,
`/private/tmp/author-retry-stage-oracles.log` and earlier artifact checks in
`/private/tmp/author-retry-all-oracles.log`; local logs are not durable CI evidence.

Separate completed-diff automated self-review checked grammar cache selection,
strict version/pin pairs and source/companion matching, integrity-domain
compatibility, optional-clause enforcement, reference roles, derived source
dependencies, built-in CLI versus explicit library resources and stage boundaries.
The ledger's shared exact-pin verifier avoids allocating a vector of matches and
also checks inherited base bytes. No actionable finding remains. ADR-0027 is
accepted under delegation, with final-head CI/expected-head merge still required.
No independent reviewer, actual user comprehension or real retry execution is
claimed.

## Remaining work

The clause closes only the explicit source-binding gap. Full validator, exceptional/
lifecycle/domain proofs, trusted host enforcement, complete benchmark/conformance,
general text/visual export/editing, tool-managed ordinary authoring and protected
accessible materials/user evidence remain required before Phase 1 exit.
