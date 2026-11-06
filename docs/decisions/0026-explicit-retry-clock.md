<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0026: Bind retry timing explicitly in a versioned core profile

**Status:** Accepted under Project Owner delegation; effective upon PR merge<br>
**Date:** 2026-11-06<br>
**Decider:** Codex under ADR-0018 delegation

## Context

Core 0.1.0 P4 requires delay to elapse on a pinned basis, but its closed retry
record has no clock or basis fields. Work timeout is optional and independent;
an ambient scheduler clock or zero-delay exception would invent semantics.
ADR-0024 therefore refuses active multi-attempt timing. Preserve frozen bytes,
explicit dependencies, protected observations and the static/live boundary.

## Decision

Adopt [core profile 0.2.0](../dialects/core-retry-v0.2.md), exact snapshot
[`sha256:5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623`](../ir/contracts/sha256-5f3c08648ea19d82251c63d24d15b9639eb28dab9eeb254121520f1d6b9a4623.txt),
as an explicit extension inheriting the exact base artifact. Its only payload
change is required nullable retry clock/basis fields: both null for one attempt,
both explicit for multiple attempts. Record dialect selects the shape; mixed
definitions do not upgrade old work records. Verify both contract artifacts.
All other base judgments remain in force, including retry eligibility/effect
safety and semantic-contract identity.

Check known clock/source metadata, visible basis and transitive source policy
reads. Keep a distinct pre-write retry-clock origin beside timeout and activity
origins, including null results. Expose immutable RetryUse references, delays,
attempt limit and host requirements through check_timing. Old active unbound
retries still refuse; unvisited retries remain uncovered. Reports cannot dispatch.

The live host must correlate the preceding eligible fault and accepted stamp
in the registered retry-clock domain, add the delay with overflow checks, retain
original request/input and basis revisions, and recheck work/target/source
authority, effect evidence, budget, cancellation, closure and owned settlement.
Different retry/timeout clocks are independent; no coordinate comparison is
implied. A static descriptor cannot authenticate any of these live facts.

## Alternatives and trade-offs

| Option | Benefit | Cost/risk | Decision |
| --- | --- | --- | --- |
| New core profile with explicit fields | Precise portable binding; old meaning preserved | Versioned payload/source evolution and explicit base artifact | Adopt |
| Borrow timeout or scheduler clock | Small implementation | Optional timeout, missing basis/domain, implicit authority | Reject |
| Annotation or provider boolean declares binding safe | Avoids schema change | Converts untrusted claims into semantic authority | Reject |
| Keep every retry unsupported | Conservative | Leaves an actionable contract gap indefinitely | Reject; keep honest live obligations |

Implementation language remains Rust on requirements; familiarity supplies no
candidate advantage. No new dependencies or engine are needed.

## Consequences and follow-up

First deliver core/static/protection/timing and direct-IR evidence in this PR.
Then publish a new near-English source profile with an explicit clock/basis
sentence and identity-preserving companion/source cycles. Source 0.2.0 continues
to lower old core records until that dedicated follow-up; the CLI's new retryUses
field is empty for them. No silent migration, wire-bookkeeping workaround or
per-item owner approval is introduced.

Follow [the frozen plan](../evaluation/0026-retry-clock-plan.md), `fe26a4f`.
Separate automated review, relevant native/Wasm/artifact checks and final-head CI
are merge requirements. Live host, exceptional/lifecycle/domain proof, complete
validator, benchmarks, round trips, accessible editing, user studies and Phase 1
exit remain open. No real host/fault/clock observations are claimed.

Accepted under delegated automated self-review after the completed diff was
checked separately for strict version/field handling, exact artifact integrity,
contextual visibility, source-origin independence, bounded metadata/report work
and honest runtime boundaries. Local checks passed 190 Rust tests plus five
compile-fail doctests, native/Wasm lint/builds and relevant artifact/source/stage
oracles. No independent human/bot approval or actual host execution is claimed.

1. [x] Verify implementation and evidence; record delegated acceptance.
2. [x] Require passing final-head CI and expected-head merge after review.
3. [ ] Implement the explicit near-English source profile follow-up.
4. [ ] Continue remaining Phase 1 contracts and aggregate evidence.
