<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Phase 1 execution checkpoint

**Updated:** 2026-11-06<br>
**Phase:** Semantic core and language prototype — incomplete<br>
**Workflow:** [ADR-0018](../decisions/0018-phase-review-delegation.md)

## Verified baseline

PR #21 supplies exact resource readers and the static type/expression/policy
stage. PR #22 supplies conservative ordinary flow/initialization checking; the
owner approved it on 2026-11-05. Its three review findings were fixed in
`c53e4d4`, approval recorded in `d414e97`. Required CI passed at the final head;
PR #22 merged as `158207452781a3488d6c2e84a288346374da69bd`. No executable admission, clocks, engine or full benchmark
claim follows from either partial report.

## Execution order and remaining evidence

1. [x] Finalize PR #22; record the delegated workflow in its own PR.
2. Implement control-sensitive information-flow checks and explicit access
   obligations, retaining the distinction between static proof and host authority.
3. Complete concurrency/invalidation/reconciliation and exceptional-flow
   judgments, with an explicit supported-clock contract/host boundary proposal.
4. Complete structural/link/semantic validation and actionable stable diagnostics;
   close supported/unsupported cases explicitly rather than relabeling a subset.
5. Expand RP-01/03/08 into complete source/IR benchmark packages and conformance
   cases; keep all forty corpus scenarios in coverage/gap accounting.
6. Implement identity-preserving general text/IR export/editing and full round
   trips, plus the initial visual notation/layout format and visual/IR cycles.
7. Supply accessible matched authoring/visual materials and representative-user
   evaluation. Actual participant observations remain necessary; generated or
   agent-only assessments cannot close this gate.
8. Audit ADR-0009 G1–G4 and every Phase 1 exit criterion. Report achievements,
   evidence and open limits to the owner, then stop for approval before Phase 2.

Items may be split into coherent PRs or reordered where verified dependencies
justify it. Update the record on each merge rather than asking to start each item.

PR #23 merged the delegated workflow as
`384e3add214bbf1be7aecbf520a9c027d563898a` after recorded automated self-review
and passing final-head required CI. CodeRabbit automatic review was unavailable;
no independent bot/human review is claimed. The next active slice is the
[protected ordinary-transfer plan](0019-protection-plan.md), committed before code.
PR #24 merged as `ecc29c965191504b18bd98c75d6d9f64f44afe3d` after automated self-
review (including fixes for destination access-policy dependencies and comparison
work accounting) and passing final-head CI. CodeRabbit skipped automatic review;
no independent review is claimed. The current follow-up is the
[initial access dependency plan](0019-initial-access-plan.md). No phase-level gate
is closed by these partial reports.
That follow-up is implemented with six control groups, one source lowering/
reparse regression and a fourth protection-oracle group. Local verification
(125 Rust tests plus five doctests, lint, Wasm and stage oracles) and recorded
automated self-review passed. PR #25 merged at
`89d648266702d13746ffd4a7b9c04c5998994b59` after final-head required CI and recorded
delegated review; no unresolved findings were present and CodeRabbit skipped
automatic review. The [completion dependency plan](0019-completion-access-plan.md)
is the next active task, followed by lifecycle/race/invalidation and clock work.
The completion fix is implemented with three regression groups and stronger source
report-cycle evidence. Automated review added private immutable metadata caching
and complete visit/member work charges. Local verification passed 128 Rust tests
plus five doctests, native/Wasm checks and stage oracles. Recorded automated review
is complete. PR #26 merged as `be697012c15374ca05082a235d15cd16bd442e45` after
recorded delegated review and passing final-head CI; CodeRabbit automatic review
was skipped and no independent review is claimed. The next active dependency is
[lifecycle policy use-site validation](0020-lifecycle-link-plan.md), before
conservative race/invalidation judgments and explicit supported-clock contracts.
The lifecycle slice is implemented and accepted under delegated automated review:
actual-use fault/closure child links and missing/ambiguous effect wait-site checks,
with valid ordinary wait reuse preserved. Local verification passed 133 Rust tests
and five compile-fail doctests, native/Wasm checks and relevant Python oracles.
PR #27 merged as `20a94148a09ee9e5d75ddbe2d4c1c9aa4f50dec7` after recorded
automated review and passing final-head CI run `42110269705`. CodeRabbit skipped
automatic review; no independent review is claimed. No runtime ownership proof
or aggregate gate closure follows. Before race/invalidation analysis, implement
the [scope-creation footprint plan](0019-creation-access-plan.md) so initializer
ancestor reads cannot disappear from creator read declarations.
That fix now derives and checks per-scope footprints for invoke/repeat/split/
fanout, including transitive initializer/access dependencies and excluding fresh
local storage. Five new regression groups cover direct IR and source cycles.
Separate automated self-review passed; local verification passed 138 Rust tests
plus five compile-fail doctests, native/Wasm checks and the relevant stage oracles.
PR #28 merged as `14cdfa0147e9c0b0f324e02b29a057dbd0dd689e` after recorded
delegated review and passing final-head CI run `42111580262`. CodeRabbit skipped
automatic review; no independent review is claimed. The next slice is
[ordinary invalidation declaration coverage](0021-invalidation-plan.md), followed
by conservative conflict judgments and explicit lifecycle/clock host boundaries.
The ordinary invalidation slice now checks visible pre-result dependency links,
unions control/branch/repeated contexts, reports unvisited consequential sites and
refuses nonrepresentable cross-context origins. ADR-0021 is accepted under
delegation after separate automated self-review; 145 Rust tests plus five
compile-fail doctests, native/Wasm checks and relevant Python oracles passed
locally. PR #29 merged as `6067d9037c6dd77f01e2b7895673c00c9c066027` after
recorded automated review and passing final-head CI run `42113299497`. CodeRabbit
skipped automatic review; no independent review is claimed. No runtime
invalidation, race-safety or phase-exit claim follows. Next is the
[ordinary parallel independence plan](0022-concurrency-plan.md), followed by
explicit lifecycle/clock host contracts and remaining full-validator evidence.
The concurrency slice now indexes scope footprints, keeps fresh local storage
distinct, retains pre-fork control dependencies, and refuses unproved shared/
external/lifetime hazards. Review fixed incoming-context loss and quadratic pair
scans, with explicit nonvisible-context refusal and 2,200-branch/dense-budget
regressions. ADR-0022 is accepted under delegated automated review; 158 Rust tests
plus five compile-fail doctests, native/Wasm checks and all relevant stage oracles
passed locally. PR #30 merged as `83e3493024a2ad0d9dfa50aa1c22a9c9a8621605`
after recorded automated review and passing final-head CI run `42115779980`.
CodeRabbit skipped automatic review; no independent review is claimed. Domain
invariants, runtime/exceptional/reconciliation concurrency and phase gates stay open.
The next slice is the [supported clock resource/host boundary proposal](0023-clock-contract-plan.md).
Timer typing, clock-source information flow and host-obligation integration must
follow before source/definition timers can be enabled; no host support is inferred.
The clock contract/reader slice now pins a separate immutable instance-microsecond
profile and validates exact typed source metadata. Existing timer/source-resource
refusals remain. ADR-0023 is accepted under delegated automated review; 162 Rust
tests plus five compile-fail doctests, native/Wasm checks, artifact integrity and
all relevant stage oracles passed locally. PR #31 merged as
`47084ce231abd51d6e3c0194785673347756d6c0` after recorded delegated review and
passing final-head CI run `42118063627`. CodeRabbit skipped automatic review; no
independent review is claimed. The next slice is
[timer typing/clock flow/host obligations](0024-timer-integration-plan.md).
Calendar-domain and retry-clock bindings remain explicit subsequent contracts;
no actual clock support or phase gate closure is claimed.
The timer integration now checks strict fields/dependencies and exact wait
outcomes, retains distinct pre-write clock/activity origins and exposes immutable
timing uses with current-domain/source/authority/atomicity host requirements.
Review added wait-observation conflict coverage and source-pin/scope metadata
caching with 700-policy and visibility-isolation regressions. ADR-0024 is accepted
under delegation; 175 Rust tests plus five compile-fail doctests, native/Wasm and
all relevant stage/artifact oracles passed locally. Final-head CI and the dedicated
PR #32 merged as `05424ff687e3ef4cafed983ca6c3fe31a9864087` after recorded
delegated review and passing final-head CI run `42123357994`. CodeRabbit skipped
automatic review; no independent review is claimed. The next slice is the
[calendar construction/origin-binding plan](0025-calendar-plan.md), then explicit
retry-clock contracts (versioned where necessary) and remaining full-validator/
lifecycle, benchmark and authoring/visual/user evidence. Phase 1 remains incomplete.
The calendar slice now recognizes exact pinned relative construction rules,
shares immutable wide tables and exposes explicit live activation requirements.
Initial host calendar trust is separate from permission to pause a deadline.
ADR-0025 is accepted under delegated automated review; 183 Rust tests plus five
compile-fail doctests, native/Wasm checks and all relevant stage/artifact oracles
passed locally. PR #33 merged as `3eaf5b1c8871ea0b0a95601e8d0a4616f574f033`
after recorded delegated review and passing final-head CI run `42168666093`.
CodeRabbit skipped automatic review; no independent review is claimed. Next is
the [versioned retry-clock/basis plan](0026-retry-clock-plan.md): core/static/
timing integration followed by a separate near-English authoring revision.
No implicit timeout/scheduler clock or phase exit follows from calendar metadata.
The core 0.2.0 retry slice now requires explicit clock/basis fields per record
dialect, retains distinct pre-write source origins and exposes immutable RetryUse
metadata/live obligations. Seven new Rust and three Python groups cover version,
integrity, visibility, dependencies, independent clocks/null results, coverage and
bounded sharing. ADR-0026 is accepted under delegated automated self-review;
190 Rust tests plus five compile-fail doctests, native/Wasm and relevant stage/
artifact oracles passed locally. PR #34 merged as
`0eb2b77f0258b8fbe8bc7ec1c23b61cae48a9e43` after recorded delegated review and
passing final-head CI run `42169819399`; CodeRabbit skipped automatic review.
Next is the [separately versioned near-English source plan](0027-retry-authoring-plan.md).
Old source/core remain unchanged. No independent review, live retry dispatch or
phase gate closure is claimed.
The authoring follow-up now supplies source 0.3.0 / companion 0.2.0 and an explicit
English retry binding with exact core 0.2.0 selection. Old grammar/schema/examples
and package digests remain intact; source/companion mismatches refuse. Five Rust
and six independent Python groups cover full cycles, identities, version/field/
reference refusals, dependencies, separate clocks, alias edits and bounds.
195 Rust tests plus five compile-fail doctests, native/Wasm and all relevant
artifact/stage oracles passed. ADR-0027 is accepted after delegated automated
self-review. PR #35 merged as `6170c615bda47e985bc03e018e2d9da1e81ecc54`
after recorded delegated review and passing final-head CI run `42170950887`;
CodeRabbit skipped automatic review. The remaining-validator audit identified
[omitted settlement receiver-authority dependencies](0019-settlement-access-plan.md)
as the next concrete fix. Keep full lifecycle/validator, benchmarks, general
text/visual export/editing and actual user evidence open; no phase gate closes.

## Honest exit accounting

The near-English authoring, complete validator, text/visual round trips, executable
examples, conformance and aggregate executable-IR gates remain open. So do complete
RP-01/03/08 evidence, protected accessible editing, visual parity and user studies.
The Phase 2 reference engine is separate. Phase 1 has not reached its report/
approval boundary, and no next-phase chat is authorized yet.
