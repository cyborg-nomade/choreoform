<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Evaluation plan: Initial textual grammar and parser

**Status:** Complete bounded evaluation; proposal awaiting review<br>
**Framework and corpus revision:** `fe4695fe4c2f3040117d23c98752da4e45a24f0b`<br>
**Plan frozen:** 2026-09-07<br>
**Evaluation owner:** Proposal author; selection authority: Project Owner

## Frozen plan

Evaluate one bounded declaration-oriented textual prototype for exact lowering
to definition IR 0.1.0. Compare JSON-only authoring and a higher-level behavioral
DSL architecturally, without fabricated comparative implementation scores.
The prototype is disposable and the syntax has no released compatibility promise.
Final language selection remains blocked by the existing conditional gates.

Priorities, in order: preserve semantic fields and identities; refuse ambiguous
or lossy input; keep syntax separate from validation and execution; expose source
locations; then assess authoring economy. No weighted total.

The prototype will use explicit versioned text, named declaration sections,
stable IDs, and strict JSON record payloads. All sections, including empty ones,
are explicit. This tests a conservative authoring surface before introducing
expression syntax or control-flow sugar. Alternatives have paper maturity;
the candidate will have prototype maturity. Parser implementation alternatives
are hand-written bounded scanning, parser combinators, and generated grammars;
only the first is planned for this small, replaceable experiment.

Roles: process reader (domain knowledge, no assumed programming experience),
author (explicit ID/reference editing), operator (locating malformed input),
and implementer (lossless transformation). No familiarity advantage is assigned
to any implementation language. Keyboard-readable text is represented; screen
reader, cognitive accessibility, and user comprehension are untested.

Benchmarks: the frozen RP-01 reimbursement (Level 1), RP-03 order (Level 3), and
RP-08 incident (Level 4) IR excerpts from three domains. These remain excerpts,
not complete acceptance scenarios. All forty corpus scenarios remain in scope
for accountability; inherit the individually recorded representations and gaps
in [ADR-0009's evaluation](0009-canonical-ir.md#corpus-traceability), with no
upgrade from Partial merely because a parser can carry a record.

Tasks: export each excerpt; parse and lower back to identical IR and canonical
revision; preserve unknown annotations and ordered arrays; preserve original
source including comments; inspect source spans; edit a policy, rename an ID,
reorder declarations, and edit only comments. Reject malformed UTF-8, duplicate
sections/IDs/JSON keys, unsupported source version, missing sections, malformed
delimiters, float tokens, out-of-range integers, and excessive size/depth.

Operational perturbations (late/duplicate observation, partial failure,
concurrent cancellation, revised evidence, long-running version change, large
fan-out) are inspected only for representation loss. No execution, scheduling,
performance, security-enforcement, or recovery claim will follow from parsing.

Required artifacts: proposed ADR; exact grammar and lowering specification;
Rust source tree distinct from IR; original-source preservation and byte spans;
three source fixtures; reproducible positive/negative tests; native CLI; wasm
compile check. No new third-party dependency is planned. Environment: pinned
Rust 1.98.1 and existing Cargo.lock, one local macOS host, no user study or
independent evaluator. Bound input and lowered wire to 1 MiB and JSON depth to
the existing transport limit; fail-fast syntax diagnostics, no editor recovery
or incremental parsing in this experiment. No wall-clock performance target.

The proposal author both implements and evaluates: evidence cannot exceed C
for reproduced mechanical properties or B for worked mappings. Owner review
is required separately; absence of other ratings is not consensus.

## Observed results

Observed on 2026-09-07, with Rust 1.98.1 on the local macOS host. The frozen plan
was committed as `ca9651e` before implementation. Candidate revision is the
implementation commit in this proposal PR; no independent reviewer has rated it.

| Task | Observation | Limit |
| --- | --- | --- |
| Encode excerpts | Three checked-in text files lower to complete original IR values and identical frozen semantic revisions | Excerpts only, not complete benchmark processes |
| Repeat export | IR → normalized text → IR → normalized text is stable for all three | Does not preserve source-only comments through IR |
| Preserve source | Borrowed original source retains comments and whitespace; item and record spans slice the expected declarations | No editing, nested field spans, incremental parsing, or recovery |
| Map identity | Comment edits preserve semantic revision but change source digest; both identities available for source maps | Persistence/consumer freshness enforcement not implemented |
| Change/reorder | Policy edits and ID changes alter revision; item/record reordering does not | Renamed dangling references are a validator responsibility |
| Preserve payloads | Unknown annotations, Unicode spelling, arrays and escaped delimiters survive | No policy interpretation or access enforcement |
| Refuse ambiguity | Exact lexical diagnostic tests reject duplicate keys, exponent/fraction tokens, invalid surrogates and unsafe integers | Payload-level diagnostic span, not exact inner token |
| Resource refusal | Source size, payload depth, and post-lowering envelope depth checked; truncated inputs and arbitrary single bytes do not panic | Bounded regression corpus, not a fuzzing campaign or performance study |
| Native CLI | Three independent IR/schema/JCS comparisons, three stable export cycles and six refusal cases pass | Test oracle is a separate implementation, not an independent human evaluator |
| Portability | New library compiles/lints for wasm32-unknown-unknown | No browser execution of this parser |
| Human authoring | Exported excerpts contain 291, 332 and 396 lines including license comments | Descriptive counts, not a comprehension measure; verbose baseline remains a risk |
| Operational perturbations | Existing fields are retained without semantic alteration | No late-event, cancellation, fault, version-migration or large-fan-out execution |

## Corpus traceability

The following forty rows retain the baseline IR representations and gaps
verbatim from ADR-0009's frozen evaluation. For this text candidate, generic
record transport preserves these slots, but **only the three excerpt files are
mechanically exercised**. Each row remains Partial, and each remaining gap
below still applies. This inheritance is representation accountability, not a
claim of forty executed textual scenarios. Source-specific additional gaps
for every row are human authoring effectiveness, complete benchmark syntax,
semantic validation, and equivalent visual editing.

| Scenario | Status | Candidate representation | Remaining gap |
| --- | --- | --- | --- |
| RP-01-A | Partial | Human activity → payment capability → finish; snapshot/protection fields | Finance flow and executable payment/authority contracts |
| RP-01-B | Partial | Data invalidates review node, with immutable definition and runtime revision links | Correction flow and data-impact semantics |
| RP-01-C | Partial | Unknown payment outcome routes to correlated wait policy | Executable idempotency and reconciliation contract |
| RP-01-D | Partial | Scope race/cancellation policies; no implicit flow ordering | Withdraw/approve race trace and policy validation |
| RP-02-A | Partial | Wait policy carries clock/calendar requirements; named outcomes | Timer dialect and complete support-ticket example |
| RP-02-B | Partial | Distinct timer policy references and immutable runtime-history linkage | Warning/deadline traces; no clock evaluator |
| RP-02-C | Partial | Work policy, actor requirement, assignment/observation linkage | Stale assignment revision checks |
| RP-02-D | Partial | Scope and wait policies retain explicit pause authority/basis | Pause/calendar semantics and examples |
| RP-03-A | Partial | Explicit stock/payment scopes, paired join and stable branch keys | Shipment, capture, delivery and failure paths |
| RP-03-B | Partial | Remaining-child and fault policies preserve settlement obligations | Executable void/expiry compensation flow |
| RP-03-C | Partial | Scoped cancellation and explicit consequential-race policy | Packed/cancel traces and provider facts |
| RP-03-D | Partial | Capability contract/effect policy and observation/effect ID linkage | Duplicate event execution tests |
| RP-04-A | Partial | Protection envelope, parallel scope templates and activation policy | Onboarding graph, date gates and custody effects |
| RP-04-B | Partial | Version-bound timer policy and stable declaration IDs | Rescheduling and selective reuse traces |
| RP-04-C | Partial | Data invalidates dependent work rather than mutating history | Impact-policy contract and safe reprovisioning |
| RP-04-D | Partial | Cancellation/remaining-work policy plus ordinary compensation activities | Full parallel disable/recall handlers |
| RP-05-A | Partial | Human actor requirement; decision input revision; capability boundary | Clinician domain policies and responsibility acceptance |
| RP-05-B | Partial | Timer is a wait policy, not a manufactured human outcome | Executable timeout-to-human flow |
| RP-05-C | Partial | Read/declaration links preserve prior immutable runtime revisions | Revised-evidence and renewed-decision trace |
| RP-05-D | Partial | Separate capability results and observation correlations | Delivery versus responsibility contract |
| RP-06-A | Partial | Per-item data binding, named outcomes and scoped join | Full unit disposition/closure model |
| RP-06-B | Partial | Stable item key, explicit item cell, change policy and seal | Keyed add/remove/retain execution and genealogy evidence |
| RP-06-C | Partial | Typed observation/policy slots and invariant requirements | Conflict policy distinguishing digital and physical evidence |
| RP-06-D | Partial | Effect/reconciliation policy independent of control success | Provider and physical-status reconciliation trace |
| RP-07-A | Partial | Single fanout template, explicit item key and body binding | Million-item resource/identity tests |
| RP-07-B | Partial | Execution/attempt/effect IDs separate in runtime linkage contract | Lease/retry/recovery semantics and checkpoint schema |
| RP-07-C | Partial | Exact definition/plan binding; pause policy and immutable item identity | Pause/resume and artifact-reuse execution |
| RP-07-D | Partial | Compensation is linked work, not definition/history erasure | Publication recovery model and implementation |
| RP-08-A | Partial | Closure policy retains owned follow-up; explicit child join | Complete objective/transfer process |
| RP-08-B | Partial | Incident excerpt binds each asset to a keyed child and explicit seal | Incremental membership traces and all adaptive paths |
| RP-08-C | Partial | Human authorization and independently gated isolation capability | Trusted authority/policy validation; no executable isolation |
| RP-08-D | Partial | Runtime observation IDs, provenance and causal predecessor links | Import/reconciliation schema and offline-outage traces |
| RP-09-A | Partial | Artifact/type snapshots, parallel child scopes and monotone join | Full review/signature graph and evidence contract |
| RP-09-B | Partial | Explicit invalidates set and stable data/work IDs | Selective impact decision semantics |
| RP-09-C | Partial | Distinct outcome flows plus resolver/actor policy references | Conflicting-term resolution flow and authority test |
| RP-09-D | Partial | Immutable capability contract, stable effect and unknown-result policy | Existing-package reconciliation example |
| RP-10-A | Partial | Separate capability/effect records and runtime revision bindings | Renewal/payment/entitlement full process |
| RP-10-B | Partial | Work/effect policies separate retry attempts from logical effect | Retry cancellation and customer-update trace |
| RP-10-C | Partial | Explicit timer, cancellation and consequential-race policies | Policy-based ordering and duplicate-charge tests |
| RP-10-D | Partial | Closure policy distinguishes reconciliation subscription from terminality | Concrete late-success transition and linked-instance records |

| Supported | Partial | Outside scope | Unknown |
| ---: | ---: | ---: | ---: |
| 0 | 40 | 0 | 0 |

## Gate review

| Gate | Result | Evidence and closure |
| --- | --- | --- |
| G1 Corpus accountability | Conditional | Forty traceable inherited rows, three exact excerpt lowerings. Phase 1 author must supply complete RP-01/RP-03/RP-08 representations before final language selection. |
| G2 Semantic accountability | Conditional | Explicit fields retained; syntax is not admission. Type/policy/expression owners must supply accepted executable contracts and validation before planning/execution. |
| G3 Cross-form parity | Conditional | Text/IR projection tested; visual notation and comment-preserving cross-form edits absent. Frontend owners must provide complete equivalent benchmarks and repeated editing round trips before Phase 1 exit. |
| G4 Protected and accessible meaning | Conditional | Protection/annotations retained; no enforcement or accessibility study. Policy/frontend owners must demonstrate protected accessible editing before Studio/executable conformance. |
| G5 Reproducible comparison | Pass for bounded candidate | Plan frozen before implementation; pinned toolchain and lock; checked-in fixtures, tests and separate Python oracle. No comparative usability study or independent rating implied. |

These gates do not authorize final language selection. The recommendation is
only to accept or reject a replaceable working prototype.

## Single-author criterion profile

No aggregate score. Scores cover this bounded package; untested dimensions are
left unscored rather than treated as success or assigned a fictitious failure.

| Criterion | Fitness | Evidence | Confidence | Rationale |
| --- | --- | --- | --- | --- |
| S1 Corpus adequacy | 2 | B | Low | Partial inherited mappings; excerpts only |
| S2 Precision and analyzability | 2 | C | Medium | Exact outer grammar and lossless lowering; semantic validation absent |
| S3 Economy and orthogonality | 2 | B | Low | Few outer constructs but verbose JSON interiors |
| S4 Composition and scale | 2 | B | Low | Existing scope/reference slots retained; authoring and large-scale editing untested |
| S5 Effects, failure, protection | 2 | C | Medium | Complete fixture values retained; no executable contract proof |
| S6 Identity and evolution | 3 | C | Medium | Stable IDs, exact version, source/semantic identity separation and export refusal tested within this profile |
| H1 Cognitive fit | Unscored | A | Low | No representative readers/authors evaluated |
| H2 Visibility and changeability | 2 | B | Low | IDs exposed and spans available; verbosity and coarse errors remain |
| T1 Textual effectiveness | 2 | C | Low | Mechanical authoring surface works; human editing efficacy unknown |
| V1 Visual effectiveness | Unscored | A | Low | No visual candidate |
| V2 Visual complexity management | Unscored | A | Low | No visual candidate |
| X1 Cross-form/engineering integrity | 2 | C | Medium | Text/IR equality and wasm compilation; no visual or browser execution evidence |

The same author designed and assessed the candidate. No second score profile,
calibration session, or disagreement has been collected. Absence of independent
objections is not consensus; owner review may change the recommendation.
Paper alternatives are compared in ADR-0011, without invented equivalent scores.

## Reproduction

Run the commands in [the syntax guide](../text/README.md#running-and-testing).
Observed passes:

- `cargo test --workspace --locked`: 14 new parser integration tests, 7 existing
  Rust unit tests (including the 89-case shared suite), and 3 compile-fail tests.
- `cargo fmt --all -- --check` and native all-target Clippy with warnings denied.
- Wasm library Clippy with warnings denied for the new parser.
- Existing Python wire checker: 14 test groups.
- Existing portability oracle: 89 results, 15 JCS/hash comparisons, 7 CLI checks.
- New text oracle: 3 complete IR/schema/JCS comparisons, 3 stable export cycles,
  6 CLI refusals.

No third-party packages changed. Cargo.lock adds one local workspace member;
its SHA-256 is
`166a9cf5471183e8d6fd9fe0e754da02eaf63a258977519a97bebc19ae71fd35`.
The existing hash-locked Python requirements are unchanged. Browser proof for
the prior portability probe remains historical; no new browser result is claimed.

## Risks and recommendation

Recommend this as an explicit, discardable baseline, with owner review of the
verbosity trade-off. The likely authoring risk is that JSON interiors impose too
much low-level work; mitigation is a representative-user comparison with richer
worked syntax before a compatibility commitment. Mixing outer and JSON
punctuation is another review concern. Both remain reversible because no released
source compatibility or execution contract is created.

Revisit parser machinery if grammar growth demands recovery or incremental
editing. Revisit source syntax with full benchmarks and the visual design.
Retaining probe-core reuse is temporary; a production component split belongs
with later validation work. No user-familiarity argument was used.

No accepted IR semantics or corpus scope changes. Do not start the next
Roadmap item until the owner authorizes it.

## PR #14 review follow-up — 2026-09-08

Reviewed all GitHub review threads, submitted review bodies, and top-level
comments at head `50cf3b4`. Pagination was exhausted: one unresolved inline
thread, by CodeRabbit, and its separate docstring coverage warning.

- **Generated-source size classification: valid, fixed.** A compact valid IR
  with 200,000 zero-valued annotation array entries fits the input limit but its
  pretty source exceeds 1 MiB. A regression test first reproduced the erroneous
  `unrepresentable text export: source size limit` result. Export now checks the
  generated size before internal parsing and reports `source size limit` with
  the supplied IR's span. The Python oracle independently checks the fixture's
  structural validity, nonzero CLI exit, exact diagnostic, and empty stdout.
- **Inline suggested input-size patch: not applied.** Input is already bounded
  by strict transport; a second input-length check does not fix generated-output
  expansion. The fix follows the finding's actual cause instead.
- **Blanket 80% docstring threshold: not adopted as a merge requirement.** No
  repository policy specifies that threshold. Added useful API documentation for
  source binding/source access/item spans and the oracle helpers; did not add
  repetitive test/helper prose merely to satisfy an external percentage.

The updated local suite has 15 parser integration tests and the text oracle has
7 refusal cases. No wire semantics, dependency versions, or frozen revisions
change. The output check classifies resource refusal; it does not claim streaming
or an allocation cap during pretty-print generation.

The owner's near-plain-English direction is a review requirement added after
the original frozen evaluation. It is recorded in ADR-0011 and the Roadmap as a
required separate authoring-language deliverable with representative-user tasks,
not evidence this prototype has met that goal. No scores have been upgraded.

Roadmap history (`1a70b29`, PR #11 structural acceptance) confirms that executable
completion was explicitly deferred, not delivered. Its previous linear placement
obscured dependencies on later contracts, validation and benchmark work. The
revised Roadmap retains the exact unchecked obligation as an aggregate gate,
names its prerequisite deliverables and closure authority, and proposes
binding/type foundations followed by executable dialect contracts before
authoring/visual design. This does not check off or implement the missing work.
