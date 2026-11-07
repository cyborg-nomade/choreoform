<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Phase 1 execution checkpoint

**Updated:** 2026-11-07<br>
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
The settlement correction now checks/derives receiver management-authority reads,
retains receiver-context visibility and excludes unrelated child settlement
dependencies. It also supplies exact policy-read diagnostic paths and bounded
closure-copy work. The old omission was reproduced before changes. Six new Rust
and three Python groups cover direct/source behavior, inheritance, modes, links,
500 repeated edges and both profile cycles. 201 Rust tests plus five compile-fail
doctests, native/Wasm and relevant artifact/stage checks passed. PR #36 merged as
`ac6edfa6402c67575de4740c83017e2d97d836bb` after delegated review and passing
final-head CI run `42172282464`; CodeRabbit skipped automatic review. Next is the
[validator gap audit and value-expression conformance plan](0028-value-conformance-plan.md).
Actual live
acceptance/ownership, full benchmarks, editing/visual/user evidence remain open.
The consolidated validator/gate audit now distinguishes implemented judgments,
conservative refusals, missing static analyses, live host facts and product/user
evidence. The reusable value-expression corpus starts conformance with 69 exact
vectors (42 values, 16 static refusals, 11 selected faults) in three Rust groups;
read/fact remain invariant-context refusals. Three Python groups check catalogue
integrity rather than independently evaluating semantics. 204 Rust tests plus
five compile-fail doctests, native/Wasm and all relevant artifact/stage oracles
passed; separate delegated self-review found no open issue. Required final-head
CI passed in run `42173865419`; PR #37 merged as
`8da0ac6c7d632bb05a622fe9e6ec9f27728e8563` after recorded delegated review.
CodeRabbit skipped automatic review. Next is the
[frozen P4 handler-selection graph proposal](0028-fault-routing-plan.md), with
lexical precedence, active failed-selection exclusion and bounded conservative
cycle analysis without guessing emission from retry eligibility. No phase gate
or full validator is closed, and no Phase 2 engine work is started.
The fault-routing slice now checks a bounded indexed active-stack graph with
exact lexical keys, per-propagation failed-selection exclusion and cycle-unproved
refusal. It reports abstract route coverage separately from still-uncovered
handler data contexts and concurrent occurrences. Eleven Rust groups and three
Python groups cover roles/precedence/exclusion, cycles, 1,000 uncalled scopes,
63 nested handlers, budget refusal and both-profile cycles. 215 Rust tests plus
five compile-fail doctests, native/Wasm and all relevant artifact/stage oracles
passed. ADR-0028 is accepted after separate delegated self-review; required
final-head CI passed in run `42176546160`; PR #38 merged as
`a20049d51a15052a8457a66c320660e623c9a131` after delegated review. CodeRabbit
skipped automatic review. Next is the
[conditional handler-body availability plan](0029-exceptional-availability-plan.md),
including explicit ancestor-input assumptions and a private initializer scope
index. Keep exceptional protection/invalidation/lifetime, actual host, full
benchmark/conformance and text/visual/user gates open; no phase exit is claimed.
The conditional handler-body slice now derives explicit ancestor-input premises,
checks selected bodies with fresh locals/separate contexts and refuses unavailable
owner-initializer/result/metadata reads without inventing snapshots. A private
initializer scope index removes a reproduced 1,000-visit × 1,000-unrelated-cell
scan quota failure in availability/protection. Seven Rust and three Python groups
cover premises, roles, initialization, nested/uncalled cases, metadata and both
profile cycles. 222 Rust tests plus five compile-fail doctests, native/Wasm and
all relevant artifact/stage oracles passed; ADR-0029 is accepted after delegated
self-review. PR #39 merged as `4d95e3610488ad98383ab9ddadeff36e5a8fca68`
after recorded delegated review and passing final-head CI run `42179630093`;
CodeRabbit skipped automatic review. The lifecycle audit identified the next
concrete dependency: [owned reconciliation declaration links](0030-reconciliation-links-plan.md),
then the explicit qualified child-endpoint source binding. Keep exceptional
fault-control protection/invalidation/lifetime and host-label boundaries, full
validator, benchmarks, general text/visual/protected editing and actual user
evidence open; no phase gate is closed.
The reconciliation declaration slice now requires the activity's own surviving
follow-up and potentially reachable unique wait inside it, with indexed lookups,
exact immutable references and host/entry/lifetime requirements. Seven Rust and
four Python groups cover correct raw IR under both core profiles, wrong/unreachable
owners/routes, incompatible scope reuse, unused/unvisited contexts, 500 repeated
sites and the old source binding boundary. 229 Rust tests plus five compile-fail
doctests, native/Wasm and all relevant artifact/stage oracles passed. ADR-0030 is
accepted after delegated self-review. PR #40 merged as
`9a530b257ed0fdc2d5b891fea905a1e12ff4dd89` after passing final-head CI run
`42203551485`; CodeRabbit skipped automatic review and no unresolved findings
were present. Next implement the [frozen separately versioned qualified reconciliation
endpoint plan](0031-qualified-endpoint-plan.md) and its
endpoint authoring profile. Actual entry/handoff/correlation/multiple-obligation
lifetime, exceptional protection/invalidation, full validator/benchmarks and
text/visual/protected editing/user gates remain open.

The qualified endpoint follow-up now supplies source 0.4.0 / companion 0.3.0,
unchanged core/grammar meaning, designated child wait binding and an owned service
source example. Six Rust and five Python groups cover identity, format/hash/report
cycles, strict profiles/pins, timer/retry compatibility, declaration order and
private/non-designated/nested/wrong-policy boundaries. 235 Rust tests plus five
compile-fail doctests, native/Wasm and all relevant artifact/source/stage oracles
passed. ADR-0031 is accepted after separate delegated self-review. PR #41 merged
as `1c6718c5c2afd23f5e7609ba34f77cafb44a06db` after passing final-head CI run
`42204771363`; CodeRabbit skipped review and there were no unresolved findings.
The P6 audit confirms a no-port surviving boundary, without guaranteed retained
ancestor values. Next is the [frozen conditional entry plan](0032-reconciliation-entry-plan.md);
retain exceptional protection/invalidation and full validator/benchmark/
text/visual/protected editing/actual user gates. No phase exit is claimed.

The reconciliation-body stage now checks each actual reconcile-closure follow-up
with zero inherited cells, fresh locals and one shared graph/index/budget, keeping
ordinary/handler/conditional body coverage separate. Seven Rust and four Python
groups cover parent-value refusals, local initialization, role isolation, nested/
unused contexts, 401 entries and owned source cycles. 242 Rust tests plus five
compile-fail doctests, native/Wasm and all relevant artifact/source/stage oracles
passed. ADR-0032 is accepted after separate delegated self-review. PR #42 merged
as `b6dbb9f6c16b68afdc8a9928d7ac68f8bfe90852` after passing final-head CI run
`42205941353`; CodeRabbit skipped review and there were no unresolved findings.
Next is the [frozen source diagnostic plan](0033-source-diagnostics-plan.md),
using existing immutable source maps. Retain exceptional/follow-up
protection/invalidation/concurrency, richer retained snapshots, actual host and
full validator/benchmark/text/visual/protected editing/user gates. No phase exit.

The source diagnostic utility now maps existing IR record paths to immutable
original UTF-8 declaration/generated spans; all ten native semantic commands share
the formatter, preserving code/path prefixes without copying source literals.
Four Rust and five Python groups cover actual static/control/lifecycle errors,
generated records, wrong/missing/global paths and multibyte format/reparse cycles.
246 Rust tests plus five compile-fail doctests, native/Wasm and all relevant
artifact/source/stage oracles passed. ADR-0033 is accepted after separate delegated
self-review. PR #43 merged as `fa27c712eaabd9b218a8dc699d353f6934836797`
after passing final-head CI run `42206977773`; CodeRabbit skipped review and there
were no unresolved findings. Next is the [frozen static-path correction](0033-static-paths-plan.md)
for still-bare scope/data/actor/capability static failures. Full
semantic/proof/benchmark/text/visual/protected editing and actual user gates remain
open; declaration locations do not close full diagnostics or phase exit.

The static-path correction reproduced bare source errors, then added selected
scope/input, actor, data and capability record/field paths without changing codes
or capability short-circuit order. Five Rust and three Python groups cover exact
raw-IR pointers, both core/all source profiles, native source spans, unused/private
metadata, ancestry and global revision behavior. 251 Rust tests plus five compile-
fail doctests, native/Wasm and all relevant artifact/source/stage checks passed.
Separate delegated review found no actionable issue. PR #44 merged as
`dc529b0a9ce639601661fc0833702786ec663d24` after passing final-head CI run
`42208252170`; CodeRabbit skipped review and there were no unresolved findings.
Next is the [frozen full-benchmark readiness audit](0034-benchmark-readiness-plan.md)
to choose the next substantial implementation slice. The forty historical scenarios and full
validator/proof/text/visual/protected editing/user gates remain open.

The full benchmark readiness audit now traces all twelve RP-01/03/08 scenarios
and their whole-case alternatives to current capabilities and concrete gaps.
It verifies no-output split/repeat/fan-out restrictions and separates result
aggregation, fault continuation, surviving data and import/protected-view needs
from isolated checks. A current table retains all forty unique scenarios as
Partial. Local link/anchor/SPDX/count checks and all six existing authoring-evidence
groups passed; separate delegated document review found no actionable issue.
PR #45 merged as `482adf7c2871e2766e0ac645628a1a2c38a872ce` after passing
final-head CI run `42209667673`; CodeRabbit skipped review and there were no
unresolved findings. Next implement the [frozen full RP-01 package plan](0034-rp01-package-plan.md),
measuring actual source/quota and stage results
and retaining all four scenarios/alternatives and explicit unsupported boundaries.
Any necessary contract extension gets its own ADR/version/evidence; no toy review
is relabeled a full reimbursement package. Full phase gates remain open.

RP-01 implementation is in progress under frozen plan `8304be4` on
`codex/rp01-benchmark-package`. The generated draft has 52,055 source bytes and
8,945 measured tokens, nine scopes and explicit request/check/review/correction/
payment/notice/follow-up paths, exact synthetic resources and four scenario maps.
It is **not lowerable/admitted yet**, with no canonical IR or semantic stage claim.
The strict current source 0.4 frontend reproduces `tokens`; a separate syntax/
schema-valid 1,097-token witness reproduces `reference-missing` for an ancestor
policy naming the actual capability activity's child-owned follow-up endpoint.
Four targeted Rust groups verify exact nominal/provider/clock metadata and input
artifact integrity without host authority; native workspace lint passed. A
provider observation-key collision found by those checks was fixed. Draft
generation/probes are reproducible with `tools/build_rp01_benchmark.py --check --probe-profile`.
Next keep this package as an unmerged draft and freeze a separately versioned,
bounded authoring profile for actual-use endpoint binding and the measured token/
parser quota need. Preserve old profiles/limits and general data privacy; then
return to full lowering, semantic/refusal evidence and all RP-01 alternatives.
Draft [PR #46](https://github.com/cyborg-nomade/choreoform/pull/46) tracks head
`def1d7e299f66aa12442181bbfc90b6443a99a07`; it is not ready for merge. The
active dependency branch is `codex/benchmark-authoring-profile`, following the
[frozen source 0.5 plan](0034-benchmark-profile-plan.md), stacked on that draft.
Source/IR, full validator/proof, benchmark, visual/protected editing and actual
user gates remain open. No phase report or Phase 2 boundary has been reached.

Source 0.5.0 / companion 0.4.0 is implemented on the stacked dependency branch
under ADR-0034, retaining core 0.2 and older limits/privacy. It provides closed
whole-program quotas and actual-use follow-up endpoint binding. Explicit promoted
copies of the full RP-01 draft lower with 8,945 tokens / 101,388 native pattern
visits, but its static permission mismatch and called surviving-scope lifetime
proof remain separate. Three additional Rust and seven Python groups pass;
258 Rust tests plus five compile-fail doctests, native/Wasm and all CI Python
checks passed. Separate delegated review corrected budget documentation and
added exact token-cap evidence. See [dependency evidence](0034-benchmark-profile.md).
Require final-head CI/comment audit/expected-head merge into the unmerged parent
before explicit benchmark adoption. No complete scenario or phase gate is closed.

Dependency [PR #47](https://github.com/cyborg-nomade/choreoform/pull/47) merged
into draft #46 as `3ba30c1fc7a2a4781c3f7db990922a63f0070669` after final-head
CI `42243608148`. Delegated self-review comment `6688576876` records the
separate review and no unresolved findings; CodeRabbit skipped the non-default
base and supplies no independent evidence. ADR-0034 becomes effective on main
only with parent integration. The active branch is again
`codex/rp01-benchmark-package`; next follow the
[frozen integration continuation](0034-rp01-integration-plan.md) to adopt the
profile explicitly, fix the contradictory synthetic provider capability envelope,
derive canonical IR and retain actual semantic-stage results/refusals. The
parent remains a draft and no complete benchmark/phase gate is closed.

The [RP-01 candidate checkpoint](0034-rp01-checkpoint.md) now derives canonical
IR under explicit source 0.5 / companion 0.4 and records all ten real native stage
refusals in its manifest. The initial information-flow mismatch remains. Testing
the intended common provider capability label revealed a genuine core-0.2
`policy-cycle`, so the package-only fix is not adopted. Five new Python groups
reproduce both refusals, generation/hashes/formatting, negative pins/roles and four
scenario mappings; 258 Rust tests plus five doctests, workspace lint and all CI
Python oracles pass. Active draft #46 remains unmerged. Next freeze a separately
versioned finite provider-metadata read-closure contract while preserving legacy
cycle refusal and all real dependency reads; then add explicit authoring support
and return to remaining package/proof issues. No complete benchmark or phase exit.

The active stacked dependency branch is `codex/provider-metadata-closure`, following
[frozen plan](0035-provider-metadata-plan.md), `52596b0`. ADR-0035/core 0.3
provides finite provider metadata read closure while preserving strong acyclicity,
legacy cycle refusal, exact visibility/pins and complete read sets. Nine new Rust
groups and twelve Python-recomputed graph vectors cover real metadata/clock reads,
large iterative graphs and budget/depth boundaries. Raw RP-01 with coherent labels
passes protection but still refuses private-child invalidation context. See
[evidence](0035-provider-metadata.md). Local 267 Rust tests plus five doctests,
native/Wasm and all Python checks pass; final-head CI/comment audit/expected-head
merge are still required. Source profiles and parent candidate remain unchanged.
Next freeze explicit source support before adopting core 0.3 in draft #46, then
resolve actual invalidation/lifetime dependencies. No benchmark or phase gate closes.

[PR #48](https://github.com/cyborg-nomade/choreoform/pull/48) merged into draft
#46 as `ffe0c2cd38327003b0521a9c3a117afb4644d7ee` after final-head CI
`42247517504`. Delegated self-review comment `6689168425` records the separate
review and no unresolved findings; CodeRabbit skipped the non-default base and
supplies no independent evidence. ADR-0035 remains pending parent/main integration.
The active dependency is now `codex/metadata-core-authoring`, following the
[frozen source-0.6 plan](0036-metadata-source-plan.md). Add explicit core-0.3
source support without changing prior pairs, quotas or hashes, then return to
actual RP-01 adoption and remaining invalidation/lifetime/proof work. Draft #46
remains unmerged and all phase gates remain open.

Source 0.6 / companion 0.5 now explicitly selects core 0.3 and both inherited
snapshots on the source dependency branch. Old pairs/quotas/hashes stay intact.
Three additional Rust and six Python groups cover actual whole-source and
self/mutual metadata lowering/reads, strict resources and revision changes,
format/alias/hash cycles, privacy and dense work refusal. Promoted RP-01 copies
pass static/control/protection but still refuse private-child invalidation context.
270 Rust tests plus five doctests, native/Wasm and all Python checks pass; see
[evidence](0036-metadata-source.md). Separate delegated review removes unused
legacy reverse-index construction on the new path. Final-head CI/comment audit/
expected-head merge remain required before parent adoption. Draft #46 stays
unmerged; invalidation/lifetime/full proof/benchmark and all phase gates remain open.

[PR #49](https://github.com/cyborg-nomade/choreoform/pull/49) merged into draft
#46 as `c759741741922cb08f43793d7ed3f7645ca16f29` after final-head CI
`42250149390`. Delegated self-review comment `6689541091` records the separate
review and no unresolved findings; CodeRabbit skipped the non-default base and
supplies no independent evidence. ADR-0036 remains pending parent/main integration.
The active branch is again `codex/rp01-benchmark-package`, following the
[frozen core-0.3 adoption continuation](0036-rp01-adoption-plan.md). Adopt source
0.6 and coherent business/clock labels explicitly, preserving actual old-profile
regression fixtures, then derive real stage evidence and freeze the next genuine
proof dependency. Draft #46 remains unmerged; no scenario or phase gate closes.

The actual [RP-01 adoption checkpoint](0036-rp01-adoption.md) now uses source 0.6 /
companion 0.5 / core 0.3 with coherent fictional business/clock recipient labels.
Freshly derived IR and all ten stage results are bound in the manifest: static,
control and protection pass, while invalidation and later prerequisites refuse
`invalidation-context-unproved` at `/body/nodes/check_rejection`. No later stage
or full scenario is admitted. Historical source-0.5/narrow-resource and 0.4 endpoint
fixtures remain independently reproducible. Local 270 Rust tests plus five
doctests, lint and all Python checks pass; draft #46 still needs its final-head
checks and full deliverable completion. Next audit closed-child provenance and
mutable-survivor counterexamples before changing invalidation/lifetime analyses.
All forty scenarios, full benchmark/text/visual/protected editing/user gates and
Phase 1 exit remain open. The parent is still unmerged.

Draft #46's adoption head `3f67ab3bf54ed0e564f27dc8d6a0690b12b9f620` passed
final-head CI `42251674305`; it remains unmerged because whole-package obligations
are not complete. The active branch is `codex/closed-output-provenance`, following
[frozen plan](0037-closed-provenance-plan.md), `a3fc112`. The
[pre-change critique](0037-closed-provenance-audit.md) records five actual native
invoke/repeat/read-only-survivor/mutable-writer/missing-ancestor witnesses and
recommends only exact-scope, no-descendant-writer historical origins at synchronous
ordinary completion. Mutable ancestor reads must remain; partial fork summaries
and unproved descendants are excluded. The checker is still unchanged at this
audit checkpoint. Next implement ADR-0037 with counterexample/budget/report tests,
then rerun the actual full package and retain its next real refusal. No phase gate.

ADR-0037's exact-scope/no-descendant-writer proof is implemented on the closed-
provenance branch after the committed critique. Seven additional Rust and five
Python groups cover live ancestor retention, mutable/unvisited writers, protected
historical control, repeated/fork contexts, scope-entry evidence and work/report
bounds. The full RP-01 source/IR identity and protection reports remain unchanged;
its next real refusal is missing `/body/data/disposition/invalidates` coverage,
not private context. See [evidence](0037-closed-provenance.md). Local 277 Rust tests
plus five doctests, native/Wasm and all Python checks pass; final-head CI/comment
audit/expected-head merge remain required. Subsequent merge bodies will explicitly
include sign-off trailers; [three earlier GitHub default-message exceptions](0037-merge-signoff-correction.md)
are recorded without rewriting published hashes. Next derive precise missing
package invalidation obligations, then continue lifetime/proof work. Draft #46
remains unmerged and no complete scenario or phase gate closes.

[PR #50](https://github.com/cyborg-nomade/choreoform/pull/50) merged into draft
#46 as `a8fd751f33385cd65d96f5987489b2aead598018` after final-head CI
`42260886484`. Delegated self-review comment `6691092446` records no unresolved
findings; CodeRabbit skipped the non-default base and supplies no independent
evidence. The merge message has an explicit sign-off trailer. ADR-0037 remains
pending parent/main integration. The active dependency is now
`codex/invalidation-requirements`, following the
[frozen advisory requirements plan](0038-invalidation-requirements-plan.md).
Derive exact required/missing links while keeping all strict checks strict, then
return to source declarations and remaining lifetime/proof work. Draft #46 remains
unmerged and all scenarios/phase gates remain open.

ADR-0038 advisory requirements are implemented after the frozen plan. Six Rust
and six Python groups plus a compile-fail type-boundary test distinguish complete
requirements from passed coverage, retain strict error precedence and refuse
unproved contexts/resources/budgets. Actual unchanged RP-01 derives 84 required /
72 missing links and 88 conditional historical records. Strict invalidation still
refuses disposition coverage; see [evidence](0038-invalidation-requirements.md).
Local 283 Rust tests plus six doctests, native/Wasm and all Python checks pass.
Final-head CI, comment audit and signed expected-head merge into draft #46 remain
required. Then add precise source links and continue lifetime/proof dependencies;
the parent remains unmerged and every scenario/phase gate stays open.

[PR #51](https://github.com/cyborg-nomade/choreoform/pull/51) merged into draft
#46 as `365c2b24e0b6a763b9488801c955da3127c92120` after final-head CI
`42264580138`. Review comment `6694481064` records delegated automated review and
the complete empty-finding comment/thread audit; CodeRabbit skipped the stacked
target and supplies no independent evidence. The merge is explicitly signed off.
ADR-0038 remains pending parent/main integration. Active work is the
[precise RP-01 source-link integration](0038-rp01-links-plan.md) on the parent.
Add the exact measured links, then retain and audit the next actual proof gap.
Draft #46 remains unmerged; no scenario or phase gate closes.

The [precise link integration checkpoint](0038-rp01-links.md) adds only the 72
measured missing pairs, preserving every old link/ID/resource and all other body
fields. The native package now passes static/control/protection/invalidation;
advisory missing is zero (84 required, 88 conditional historical records). All six
later composed stages refuse `scope-lifetime-unproved` at `run_checks`, original
source bytes 40983..41175. Local 283 Rust tests plus six doctests, all Python
oracles and exact generation/profile probes pass after retaining explicit old
missing-link counterexamples. Draft #46 still requires final-head CI and remains
unmerged/incomplete. Next freeze and audit called surviving reconciliation lifetime
without weakening its closure policy. All scenarios/phase gates remain open.

Draft #46's precise-link head `b95b2be93c14626adfc248fdf3d6a30235ea8343`
passed required final-head CI `42287364292`. It remains draft/unmerged. The active
stacked dependency is `codex/isolated-surviving-waits`, following
[frozen plan](0039-isolated-survivors-plan.md), `445b23e`, and the
[pre-change critique](0039-isolated-survivors-audit.md). Five actual native witnesses
distinguish isolated waits from mutable/shared-read/storage/route cases. Next prove
only declared isolation under explicit outstanding P6 host premises; keep all
parallel and general reconciliation/progress/phase gates open.

ADR-0039's isolated synchronous follow-up subset is implemented after its critique.
Seven Rust control groups, one budget group and six Python groups preserve shared/
mutable/storage/nested/timed/parallel negatives and expose conditional P6 host
premises in all six composed native reports. Full RP-01 now passes all ten existing
partial static stages with three isolated-follow-up declarations; source/IR/
resource identity is unchanged. See [evidence](0039-isolated-survivors.md). Local
291 Rust tests plus six doctests, native/Wasm and all Python checks pass.
Final-head CI/comment audit/signed expected-head merge into draft #46 remain
required. Then audit original full A–D conformance/negative/contract dependencies;
partial stage success does not complete the benchmark or any scenario. The parent
remains draft/unmerged, all forty scenarios Partial, and all phase gates open.

[PR #52](https://github.com/cyborg-nomade/choreoform/pull/52) merged into draft
#46 as `a89e481cddae40fea18684953d421bcdf0a1447e` after required final-head CI
`42289283886` and delegated review/comment audit `6694910027`. CodeRabbit skipped
the stacked target and supplies no independent evidence; the merge is explicitly
signed off. ADR-0039 remains pending parent/main integration. Active work is the
[original RP-01 scenario conformance audit](0040-rp01-scenario-audit-plan.md),
which must distinguish ten passing partial stages from A–D/domain/host evidence.
The parent remains draft/unmerged; every scenario and phase gate stays open.

The [original A–D audit](0040-rp01-scenario-audit.md) distinguishes ten passing
partial stages from missing guard/transition/host evidence. Three actual weakened-
predicate/fault-name temporary copies still pass partial checks without executable
authority; attempted wait-result capture refuses syntax. No unsafe payment or
actual process execution is claimed. Next freeze finite source-derived guard
snapshot conformance using existing pure production judgments in a test-only
harness, then resolve concrete fault-source/late-observation/continuation needs.
Substantial independent work remains; this is not an external blocker or phase
report. Draft #46 stays unmerged and all forty scenarios remain Partial.

The scenario audit checkpoint is `80037c65c4f681d39199f120153fb11ec60f49cb`
on draft #46. Active dependency: `codex/rp01-guard-conformance`, following the
[frozen finite-snapshot plan](0040-rp01-guard-plan.md). Use exact original ASTs,
production static/value/pure judgments and explicit immutable synthetic bindings;
introduce no public evaluation/authority/runtime API or scenario completion claim.
The parent remains draft/unmerged and every phase gate stays open.

The finite snapshot harness has reproduced `commit_wrong_policy`: the actual
payment commit predicate returns true for otherwise well-typed evidence with a
wrong policy revision, while the retained expected result is false. No unsafe
execution/reachability claim follows. Before source changes, the
[precise correction plan](0040-rp01-policy-guard-fix-plan.md) freezes three explicit
policy-revision equality clauses and symmetric negatives, preserving old fixture
profiles and all other semantic fields. This source correction is part of the
dedicated guard-conformance deliverable; no phase gate closes.

ADR-0040's test-only finite guard harness now passes 44 source-derived predicate
vectors (41 Boolean values, one evaluation fault, two binding refusals), mapped
only to A–D fragments. Six new Rust groups exercise the catalogue/boundaries and
four Python groups check independent accountability. The recorded payment policy
gap is corrected by three explicit equality clauses; the six old clauses and all
other body fields/IDs/resources remain. Old profile fixtures retain their guard.
New package semantic revision is `sha256:d0bca38fb470cd0485b9a1cc22767182e6276669ca9ca131ac45d7af268269f3`.
All ten partial stages still pass. See [evidence](0040-rp01-guard-snapshots.md).
Local 297 Rust tests plus six doctests, native/Wasm and all Python checks pass;
final-head CI/comment audit/signed expected-head integration remain required. No
snapshot is an observation, permission, transition or completed scenario. Continue
concrete fault-source/late-observation/continuation work; no phase gate closes.

Draft #46's original-scenario audit head `80037c65c4f681d39199f120153fb11ec60f49cb`
passed required final-head CI `42290223948`. It remains draft/unmerged. Current
guard-conformance work is on its dedicated dependency branch and still requires
its own final-head review/CI/merge. ADR-0040 is pending parent/main integration.

[PR #53](https://github.com/cyborg-nomade/choreoform/pull/53) merged into draft
#46 as `983de13c294db11bef13d14c11c5dff15d6fd9e8` after final-head CI
`42292305622` and delegated review/comment audit `6695368575`. CodeRabbit skipped
the stacked target and supplies no independent evidence; the merge is explicitly
signed off. ADR-0040 remains pending parent/main integration. Active dependency:
`codex/provider-fault-source`, following the
[frozen metadata contract/reader plan](0041-provider-fault-source-plan.md).
First pin declared provider observation-to-fault evidence, then design explicit
versioned actual-use retry/source binding. No source/core reinterpretation or
actual fault/permission/settlement claim follows. Draft #46 remains unmerged and
all scenarios/phase gates remain open.

ADR-0041's independent provider-observation fault-source profile/reader is
implemented on its dedicated branch. Six Rust integration groups, one budget
group and sixteen vectors distinguish declared unknown/no-effect states and
reject success/acknowledgement, aliases, ambiguous maps and trust claims. Four
Python groups check independent mapping/hash accountability and identical native
timing behavior when extra mapping bytes are supplied: no implicit retry binding.
See [evidence](0041-provider-fault-source.md). Local 304 Rust tests plus six doctests
and native/Wasm/all Python checks pass; final-head CI/comment audit/signed exact
merge remain required. Then design explicit versioned actual-use binding; metadata
does not establish a fault, retry permission, scenario or phase exit.

[PR #54](https://github.com/cyborg-nomade/choreoform/pull/54) merged into draft
#46 as `5f442637c82cceba5c718f52f748368489076b16` after final-head CI
`42295158876` and delegated review/comment audit `6695872106`. CodeRabbit skipped
the stacked target and supplies no independent evidence; the merge is explicitly
signed off. ADR-0041 remains pending parent/main integration. Active dependency:
`codex/retry-fault-source-binding`, following the
[frozen explicit raw-core binding plan](0042-retry-fault-binding-plan.md).
Core 0.4 will bind declared source/provider/eligible-fault evidence explicitly;
current source 0.6/core 0.3 remains unchanged until separate authoring integration.
All actual acceptance/permission/settlement, scenario and phase gates stay open.

ADR-0042 raw core-0.4 binding is implemented after the frozen plan/supplement.
Ten Rust integration groups plus one copied-evidence budget group check exact
source/provider/eligible-class relationships, unvisited/human/unused contexts,
metadata reads and mixed/legacy behavior. Bound source names, including noneligible
faults, enter conservative routing without claiming actual emission; graph/resource
work shares one allowance. Typed RetryUse evidence remains revision-bound and
non-executable. See [evidence](0042-retry-fault-binding.md). Local 315 Rust tests
plus six doctests and native/Wasm/all Python checks pass; final-head CI/comment
audit/signed exact merge remain required. Current source 0.6/core 0.3 and package bytes
stay unchanged pending a separately versioned authoring/disclosure step. All
scenario/phase and actual host/data/continuation gates remain open.

[PR #55](https://github.com/cyborg-nomade/choreoform/pull/55) merged into draft
#46 as `f6cffd2a88635e84b39b49261d984be11e5ac263` after final-head CI
`42300892278` and delegated review/comment audit `6696903303`. CodeRabbit skipped
the stacked target and supplies no independent evidence; the merge is explicitly
signed off. ADR-0042 remains pending parent/main integration. Active dependency:
`codex/retry-fault-authoring`, following the
[frozen authoring/disclosure plan](0043-retry-fault-authoring-plan.md).
Select raw core 0.4 only through explicit new source 0.7/companion 0.6, retain old
profiles/package bytes, then separately adopt the source in RP-01. All actual
host/data/continuation, scenario and phase gates remain open.

ADR-0043 source 0.7/companion 0.6 is implemented after frozen plan `eb16001`, with
explicit fault-source clauses/core 0.4, complete source metadata reads and bounded
typed disclosure through all five stages from timing onward. Five Rust/seven Python
groups cover actual full-source/pin/identity/context/read/compatibility/hash/output
cases. Local 320 Rust + six doctests and native/Wasm/all Python checks pass; every
large-report path refuses without stdout. See [evidence](0043-retry-fault-authoring.md).
Final-head CI/comment audit/signed exact merge remain required. Current checked-in
RP-01 bytes remain unchanged pending separate adoption. No actual fault, retry
permission, continuation, scenario or phase gate is established.

[PR #56](https://github.com/cyborg-nomade/choreoform/pull/56) merged into draft
#46 as `bb6ff8a8a61b4f37927786261f50eedea946630b` after final-head CI
`42302583387` and delegated review/comment audit `6697116702`. CodeRabbit skipped
the stacked target and supplies no independent evidence; the merge is explicitly
signed off. ADR-0043 remains pending parent/main integration. Active parent work
follows the [frozen RP-01 source adoption plan](0043-rp01-adoption-plan.md).
Add exact synthetic payment-fault-source bytes and regenerate real partial reports;
keep all old IDs, guards, provider/type/clock/input bytes and snapshot expectations.
Parent remains draft/unmerged; lifecycle/data/continuation and all phase gates stay open.

RP-01 now explicitly adopts source 0.7/companion 0.6/core 0.4 and the exact
synthetic payment-fault-source mapping. All ten partial stages pass at revision
`sha256:97e5c346442d5b051e40bc91d2c3014be1bd990688324be846739c5d35a265c4`;
source is 56,874 bytes / 9,470 tokens. Five views disclose the exact declared retry
binding, and routing counts ten abstract unhandled cases, not observed faults.
Only guard-catalogue revision/digest changes; all 44 cases/46 values/outcomes and
all old graph IDs/guards/links/provider/type/clock/input bytes remain. The explicit
historical source-0.6 counterpart retains its exact old revision and negatives.
See [checkpoint evidence](0043-rp01-adoption.md). Local 320 Rust + six doctests,
native/Wasm/all Python/exact regeneration probes pass; final-head parent CI/comment
audit remain required. Parent stays draft/incomplete/unmerged. Next audit/freeze
wait-result/fault-data and owned continuation needs before adding any fields or
syntax. All forty scenarios and phase gates stay unresolved.

Draft #46 adoption checkpoint `a92e472e5f026474277aa8dd25ef32cc0b8ae386` passed
required final-head CI `42355306461`. Parent comment audit found no unresolved
threads or actionable summary finding; CodeRabbit skipped the draft and supplies
no evidence. Delegated checkpoint review is comment `6704458877`. The parent remains
draft/incomplete/unmerged. Active dependency `codex/late-observation-contract-audit`
follows the [frozen lifecycle audit plan](0044-late-observation-audit-plan.md):
resolve explicit wait capture versus fault data/owned continuation requirements
before adding a versioned field or grammar. All scenario/phase gates remain open.

The [late-observation/continuation audit](0044-late-observation-contract-audit.md)
records actual capture-syntax, ordinary-fact leakage, local recovery storage,
cross-scope flow, handler and raw closed-wire probes. One Rust/five Python groups
preserve the boundaries; no product contract/package changes. Capture needs both
an explicit admitted wire/policy representation and observation source/protection
premises; a type/target label alone is insufficient. Recommended next dependency:
exact provider terminal-observation metadata/reader, followed by separately reviewed
capture, owned-storage and fault-data/continuation contracts. Audit final-head
CI/comment review/signed merge remain required. No scenario/phase gate closes.

[PR #57](https://github.com/cyborg-nomade/choreoform/pull/57) merged into draft
#46 as `dca191306dc748bea0725cb163756a0f49084fc0` after final-head CI
`42355914677` and delegated review/comment audit `6704545336`. CodeRabbit skipped
the stacked target and supplies no evidence; merge is explicitly signed off.
The parent stays draft/incomplete/unmerged. Active dependency:
`codex/provider-terminal-observations`, following the
[frozen metadata reader plan](0044-terminal-observation-source-plan.md).
Establish exact declared terminal source/type/protection premises without binding
existing waits or changing source/core/wire/package semantics. Capture, owned
storage, fault data/continuation, all scenario and phase gates remain open.

ADR-0044 terminal-observation metadata is implemented under the frozen plan and
sharing refinement. Exact provider states derive only present terminal tags/type
and source protection; unknown/acknowledged labels stay nonterminal. Seven Rust /
three internal budget groups plus twelve vectors and four Python groups cover
pins, nominal identity, classification, sharing/copy bounds and no implicit binding.
225 distinct source aliases share one exact provider/profile schema. Local 331
Rust + six doctests and native/Wasm/all Python checks pass; current package bytes
and reports regenerate unchanged. See [evidence](0044-terminal-observation-source.md).
Final-head CI/comment audit/signed merge remain required. Next explicit capture
representation/source protection/availability, then owned storage/fault data /
continuation; all scenario and phase gates remain open.

[PR #58](https://github.com/cyborg-nomade/choreoform/pull/58) merged into draft
#46 as `f9f5bdc2b7c00756fddc9514b486bcc18fc4c872` after final-head CI
`42357073750` and delegated review/comment audit `6704719084`. CodeRabbit skipped
the stacked target and supplies no review evidence; merge is explicitly signed
off. ADR-0044 remains pending parent/main integration. Active dependency
`codex/timerless-wait-capture` follows the
[frozen raw capture/wire/core plan](0045-wait-capture-plan.md): explicit IR 0.2
wait capture with source/origin/result and core 0.5 checks, initially timerless,
without changing current source/package or proving owned recovery storage. All
scenario/phase and actual host/fault-data/continuation requirements remain open.


ADR-0045 raw wait capture is implemented under the frozen plan. IR 0.2/core 0.5
add explicit timerless source/origin/result declarations with exact provider/type,
normal visibility, complete writes/access dependencies and source/control P2 flow.
Five wire/fifteen semantic/one budget Rust groups and seventeen shared vectors /
four Python groups cover the boundaries; local 352 Rust plus six doctests and
native/Wasm/all Python checks pass. Immutable equality caching lets 421 captures
of a 2,500-field type pass the unchanged allowance without caching live authority.
A raw full-RP-01 captured-local copy passes static/invalidation but still refuses
stored-recovery lifetime, as required. Current source/package bytes stay unchanged;
see [evidence](0045-wait-capture.md). Final-head CI/comment audit/signed merge remain
required. Next freeze explicit source/companion and bounded native disclosure,
then owned local storage/fault-data/continuation proofs. No phase/scenario gate closes.


[PR #59](https://github.com/cyborg-nomade/choreoform/pull/59) merged into draft
#46 as `c65cd8faf97d43ba0fd6b095b3bf04452c1e0955` after required final-head CI
`42362368784` and delegated self-review/comment audit `6705405325`. CodeRabbit
skipped the nondefault target and supplies no evidence; no actionable threads or
summary findings existed. Merge explicitly carries the sign-off. ADR-0045 remains
pending parent/main integration. Active dependency `codex/wait-capture-authoring`
follows the [frozen source/native plan](0046-wait-capture-authoring-plan.md).
Current package and every scenario/phase gate remain unchanged/open; owned storage,
fault data and permitted continuation/cutoff/settlement remain later proofs.


ADR-0046 explicit source 0.8/companion 0.7 capture is implemented under the frozen
plan, selecting raw IR 0.2/core 0.5. Readable source/origin/result references derive
writes and complete actual-use source/destination access reads. Eleven bounded
native checking/advisory views disclose only declarations from retained static
reports, including unvisited sites, with all live host requirements. Nine Rust
integration/one final-output group and six Python groups cover version/pin/role /
visibility/type/read/alias/hash/disclosure boundaries. A 130-capture/9,523-token
source lowers, then all report views refuse overlarge output before stdout.
Local 362 Rust plus six doctests and all native/Wasm/Python checks pass; full RP-01
promoted-copy capture still refuses stored-recovery lifetime and checked-in package
regenerates unchanged. See [evidence](0046-wait-capture-authoring.md). Final-head
CI/comment audit/signed merge remain required. Next owned local-storage/lifetime
proof, then explicit fault data and owned continuation/cutoff/settlement; all
scenario, complete authoring/validator, external and phase gates stay open.


[PR #60](https://github.com/cyborg-nomade/choreoform/pull/60) merged into draft
#46 as `9a4a39f7bf9fa1080dca697d5f66b94fad3e86cf` after final-head CI
`42363432443` and delegated review/comment audit `6705563487`. No actionable
threads/summary findings existed; CodeRabbit skipped and supplies no review
evidence. Merge is explicitly signed off; ADR-0046 remains pending parent/main
integration. Active dependency `codex/owned-local-followups` follows the
[frozen conditional local-storage plan](0047-owned-local-followups-plan.md).
Require zero inherited values, complete conditional protection/invalidation and
bounded separate evidence before widening the current lifetime refusal. No actual
ownership, fault data, continuation or phase/scenario gate is closed.


Initial owned-local draft review found that zero inherited values must not erase
creator/owner source and control protection provenance. The
[frozen refinement](0047-owned-local-provenance-refinement.md) requires bounded
retained label contexts and an explicit P6 provenance-retention premise, separate
from current values/permissions and ordinary historical completion. Implementation
remains unpublished; no new lifetime or phase claim is made yet.


ADR-0047 conditional fresh-local follow-ups are implemented under the frozen plan
and provenance refinement. Separate immutable evidence requires direct portless /
childless terminal bodies, explicit capturing entry, flat timerless local-only
nodes and no independent creator. Grouped zero-inherited-value initialization,
retained creator/owner protection provenance, internal access/flow and strict local
invalidation checks precede disclosure. The host must still provide actual storage,
authenticated provenance/correlation, current access, atomic handoff/cutoff and all-
settlement. Nine Rust integration/two budget groups and five Python groups cover
boundaries and six independent finite vectors; a restricted creator/public capture
now refuses, and 130 creators/130 cells refuse oversized reports before stdout.
Local 373 Rust plus six doctests and all native/Wasm/Python checks pass. Full RP-01
promoted fresh-local capture now has conditional evidence; the adjacent ancestor
storage case refuses and checked-in package bytes regenerate unchanged. See
[evidence](0047-owned-local-followups.md). Final-head CI/comment audit/signed merge
remain required. Next explicit fault data and permitted owned continuation/cutoff /
settlement, then separate adoption; every scenario/phase/external gate stays open.


[PR #61](https://github.com/cyborg-nomade/choreoform/pull/61) merged into draft
#46 as `805d507836c3ba23243339522f63f699f075ab7a` after required final-head CI
`42365524284` and delegated review/comment audit `6705837334`. No actionable
threads or summary findings existed; CodeRabbit skipped and supplies no review
evidence. Merge explicitly carries the sign-off; ADR-0047 remains pending parent /
main integration. Active dependency `codex/fault-entry-contract-audit` follows the
[frozen fault-entry/emission audit](0048-fault-entry-audit-plan.md) before designing
fault-data representation. Actual storage/provenance/authority/handoff/cutoff /
settlement and all scenario/phase gates remain open; parent stays draft/unmerged.


The [fault-entry/emission audit](0048-fault-entry-contract-audit.md) records actual
ordinary-fact, unavailable-local, handler-port, exact-name abstraction, single-
attempt source and closed-wire refusals. One Rust/six Python groups preserve them;
unknown provider evidence remains nonterminal. No contract/package changes.
Recommended next dependency: explicit provider emission-source binding independent
of retry eligibility in a new core, followed by reviewed typed history/source /
protection and handler-local entry representation with conditional initialization /
strict invalidation. Owned continuation/cutoff/all-settlement stays separate.
Final-head CI/comment audit/signed merge remain required; all phase gates stay open.


[PR #62](https://github.com/cyborg-nomade/choreoform/pull/62) merged into draft
#46 as `a9b08627a3f9db54b69b70fc47228ec4d0918307` after required final-head CI
`42366303371` and delegated review/comment audit `6705961928`. CodeRabbit skipped
and supplies no review evidence; no actionable threads/summary findings existed.
Merge explicitly carries the sign-off. Active dependency
`codex/general-fault-source-binding` follows the
[frozen raw core plan](0048-general-fault-source-plan.md): independent declared
provider emission sources, with no fault history/data/receipt or current authority.
Parent stays draft/incomplete; all actual host/scenario/phase gates remain open.


ADR-0048 raw core 0.6 is implemented under the frozen plan. Required nullable outer
work faultSource declares provider emissions independently of attempts/eligibility;
N=1 keeps inherited empty/null retry fields and N>1 requires exact source agreement.
All actual/unvisited uses verify exact capability/provider identity, while unused
policies still verify metadata/read closure but yield no use row. Static rows share
immutable metadata/revision and carry all actual source/authority/selection/cutoff /
settlement requirements. Eight Rust integration/one budget groups plus thirteen
shared vectors/four Python groups cover boundaries; local 383 Rust plus six doctests
and native/Wasm/all Python checks pass. Current package regenerates unchanged;
see [evidence](0048-general-fault-source-binding.md). Final-head CI/comment audit /
signed merge remain required. Next explicit readable source/native selection, then
typed history/handler entry and owned continuation/cutoff/settlement. Parent and
all scenario/phase/external gates remain draft/open.


[PR #63](https://github.com/cyborg-nomade/choreoform/pull/63) merged into draft
#46 as `af62713386b4b5c6d437fa4ad87bee43f4e4b7f5` after required final-head CI
`42393751009` and delegated review/comment audit `6710263812`. No actionable
threads/summary findings existed; CodeRabbit skipped and supplies no review evidence.
Merge explicitly carries the sign-off; ADR-0048 remains pending parent/main
integration. Active dependency `codex/general-fault-source-authoring` follows the
[frozen readable source/native plan](0049-general-fault-authoring-plan.md).
Current package, typed history/handler entry, continuation/cutoff/settlement and
all actual source/host/scenario/phase gates remain unchanged/open.

ADR-0049 source 0.9 / companion 0.8 integration is implemented under the frozen
plan. Readable work policies name general sources once, reuse exact pins for retry
and derive complete source protection reads. All eleven native checking views
return retained use rows and per-source catalogues within shared/final output
bounds. Eight Rust and six Python groups cover version/resource/provider/context /
read/compatibility/sharing/budget boundaries. Local 391 Rust plus six doctests and
all native/Wasm/Python checks pass; current package bytes remain unchanged. See
[evidence](0049-general-fault-source-authoring.md). Final-head CI/comment audit /
signed merge remain required. Next exact typed history/source-control protection,
then reviewed handler-local entry and permitted owned continuation/cutoff/settlement.
Parent #46 and all scenario/phase/external gates remain draft/open.

[PR #64](https://github.com/cyborg-nomade/choreoform/pull/64) merged into draft
#46 as `6c1d51b1b71c6463950b8aab328bfc8686ac0eb1` after required final-head CI
`42396716472` and delegated review/comment audit `6710703021`. No actionable
threads or summary findings existed; CodeRabbit skipped and supplies no review
evidence. Merge explicitly carries the sign-off; ADR-0049 remains pending parent /
main integration. Active dependency `codex/provider-fault-history` follows the
[frozen metadata plan](0050-provider-fault-history-plan.md), keeping declaration /
claim shape distinct from authenticated occurrence/history or handler entry.
Parent and every scenario/phase/external gate remain draft/open.

## Honest exit accounting

The near-English authoring, complete validator, text/visual round trips, executable
examples, conformance and aggregate executable-IR gates remain open. So do complete
RP-01/03/08 evidence, protected accessible editing, visual parity and user studies.
The Phase 2 reference engine is separate. Phase 1 has not reached its report/
approval boundary, and no next-phase chat is authorized yet.
