<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Evaluation: Near-English authoring candidates

**Status:** In progress; paper comparison prepared, human evaluation not run<br>
**Plan frozen:** 2026-09-22 in `5b5c34210cf5b77faab3213fb464d641884e031f`<br>
**Framework/corpus baseline:** `8a90c092fec740436ad248e88ca864f3ccead4fd`<br>
**Owner/evaluator:** Proposal author; decision authority: Project Owner

## Scope, candidates and independence

Follow the [frozen plan](0014-authoring-plan.md), not a retrospective scoring
rule. The [ADR](../decisions/0014-near-english-authoring.md) asks for a reversible
working direction, not final syntax approval. Author, designer and evaluator are
the same agent. No independent scores, participant results or consensus exist.
Candidate revision is the exact review commit containing this file and the linked
artifacts; Git retains the pre-results plan. Subsequent changes require new
evidence attribution rather than rewriting this comparison as if observed earlier.

| Candidate | Revision/artifact | Maturity | Limitation |
| --- | --- | --- | --- |
| A | [Structured steps](../authoring/syntax.md#3-candidate-a-structured-steps), at this review commit | Worked paper | Full grammar/companion/benchmarks not lowerable |
| B | [Guarded rules](../authoring/syntax.md#4-candidate-b-guarded-rules), at this review commit | Worked paper | Same gaps; local rules only, not an event engine |

The compact JSON baseline and unrestricted English are contextual options only;
neither is assigned a like-for-like score. Existing Rust parser tests concern
that baseline, not A or B. No new product parser or dependency is introduced.

## Corpus traceability

Each row applies **separately to both A and B**, which share the same mapping and
contracts. No scenario is supported merely because it can be paraphrased in
English. All forty remain Partial; outside the three walkthroughs the evidence
is a mapping sketch, not a complete candidate representation. Each still requires
full source/IR, validation and relevant cross-form/runtime evidence.

| Scenario | A status | B status | Proposed representation | Remaining gap / consequence |
| --- | --- | --- | --- | --- |
| RP-01-A | Partial | Partial | Ask manager/finance, request payable, wait for confirmation | Exact types/providers and complete current-revision payment graph |
| RP-01-B | Partial | Partial | Explicit repeat cycle and decision invalidation | Full dependency/availability and renewed-work evidence |
| RP-01-C | Partial | Partial | Separate request and owned reconciliation wait | Exact retry/no-effect provider evidence and lifecycle routing |
| RP-01-D | Partial | Partial | Cancellation rule plus guarded commits | Withdrawal cutoff/invariant across approval and dispatch |
| RP-02-A | Partial | Partial | Distinct acknowledgement/resolution waits and clocks | Full ticket source, duration data and supported time contract |
| RP-02-B | Partial | Partial | Separate warning and breach observations | Immutable breach/history representation and deadline traces |
| RP-02-C | Partial | Partial | Work rule with assignment revision | Delayed old-assignment acceptance tests |
| RP-02-D | Partial | Partial | Explicit timer basis and pause authority | Authorized per-clock pause/resume evidence |
| RP-03-A | Partial | Partial | Start together, keyed shipment fan-out, gather | Line aggregation, exact capture total and item-interface design |
| RP-03-B | Partial | Partial | Explicit void/expiry settlement work | Provider contracts, no-shipment guard and owned failure paths |
| RP-03-C | Partial | Partial | Guarded packing/cancellation and compensation | Proven conflict coverage and physical provider cutoff |
| RP-03-D | Partial | Partial | Correlated wait with duplicate retention | Runtime one-transition evidence, not source wording |
| RP-04-A | Partial | Partial | Parallel preparation, gated activation wait | Full onboarding/custody and protected transfer definition |
| RP-04-B | Partial | Partial | Versioned timer basis and explicit reschedule | Selective preserved work versus moved activation evidence |
| RP-04-C | Partial | Partial | Named invalidation and explicit adjustment work | Safe revocation/reprovisioning contracts and dependency tests |
| RP-04-D | Partial | Partial | Cancellation plus per-effect surviving settlement | Complete parallel compensation graph and uncertainty handling |
| RP-05-A | Partial | Partial | Human disposition and responsibility acceptance | Domain authority/evidence and full transfer graph |
| RP-05-B | Partial | Partial | Timer routes to human work, not automatic rejection | Exact fault/wait routing and original receipt preservation |
| RP-05-C | Partial | Partial | New evidence revision invalidates sufficiency | Complete re-triage and prior-effect history representation |
| RP-05-D | Partial | Partial | Message request distinct from receiving acceptance | Accepted ownership transfer contract and nonclosure trace |
| RP-06-A | Partial | Partial | Per-unit keyed response and explicit closure | Digital/physical confirmation and disposition contracts |
| RP-06-B | Partial | Partial | Keyed additions retain unaffected children | Genealogy evidence, changed-input and seal execution tests |
| RP-06-C | Partial | Partial | Separate facts and human conflict decision | Physical/digital conflict model; digital success not enough |
| RP-06-D | Partial | Partial | Release request separate from reconciliation | Provider/physical evidence and safe retry contract |
| RP-07-A | Partial | Partial | One keyed template and aggregate gate | Large-population limits, result collection and publication model |
| RP-07-B | Partial | Partial | Work attempts separate from logical effects | Lease/checkpoint contracts and unknown-write recovery |
| RP-07-C | Partial | Partial | Pinned instance definition, retained keyed work | General runtime pause/resume is not timer pause; interface missing |
| RP-07-D | Partial | Partial | Compensation as new recorded work | Restoration/publication provider and whole recovery flow |
| RP-08-A | Partial | Partial | Guarded actions and explicit follow-up ownership | Full transfer, exit predicates and protected summaries |
| RP-08-B | Partial | Partial | Keyed finite batches and repeated occurrences | Ad hoc template creation unsupported; batch-reuse design needed |
| RP-08-C | Partial | Partial | Ask commander plus independent capability authority | Enforcement and invariant evidence; recommendation grants nothing |
| RP-08-D | Partial | Partial | Preserve observations, timestamps and known causality | Runtime import schema absent; cannot pretend a wait implements import |
| RP-09-A | Partial | Partial | Parallel snapshot review, gather then signature | Complete revision-bound review/signature contracts |
| RP-09-B | Partial | Partial | Explicit invalidation and attributable impact decision | Supported selective reuse semantics and full source example |
| RP-09-C | Partial | Partial | Multiple conflicting decisions route to human resolution | Complete conflict/result types; no first-rule-wins shortcut |
| RP-09-D | Partial | Partial | Separate signature request and correlated reconciliation | Exact existing-package provider/retry contract |
| RP-10-A | Partial | Partial | One renewal effect and independent receipt work | Entitlement/payment joint state and complete source/IR |
| RP-10-B | Partial | Partial | Changed payment input becomes new occurrence linked to cycle | No changed-request reuse of old retry key; full cycle invariant needed |
| RP-10-C | Partial | Partial | Guarded cancellation versus retry timer | Current authority, request identity and timer revision evidence |
| RP-10-D | Partial | Partial | Ordinary closure with owned reconciliation, or linked later instance | Explicit reinstate/credit/refund decisions and late-success tests |

| Candidate | Supported | Partial | Outside scope | Unknown |
| --- | ---: | ---: | ---: | ---: |
| A | 0 | 40 | 0 | 0 |
| B | 0 | 40 | 0 | 0 |

## Gates

| Gate | A | B | Evidence / closure owner and condition |
| --- | --- | --- | --- |
| G1 Corpus accountability | Conditional | Conditional | All scenarios recorded; Phase 1 author must supply full source/IR benchmarks, not inventories/ellipses, before final selection |
| G2 Semantic accountability | Conditional | Conditional | Closed mapping direction; language/validator authors must specify complete grammar, binding encoding and prove exact lowering/refusal |
| G3 Cross-form parity | Conditional | Conditional | No visual candidate or transformation; frontend authors must demonstrate equivalent full benchmarks and edits before Phase 1 closure |
| G4 Protected accessible meaning | Conditional | Conditional | Visible policy obligations only; owner coordinates representative studies, frontend/security authors demonstrate protected accessible editing |
| G5 Reproducible comparison | Pass at paper scope | Pass at paper scope | Frozen plan, identifiable artifacts and explicit non-results; no executable/user claims |

Neither candidate can be selected as the final language with these conditions.
The Roadmap remains unchecked; a protocol is not representative-user evaluation.

## Raw desk observations (not participant observations)

| Common task | A observation | B observation | Evidence and limit |
| --- | --- | --- | --- |
| Comprehend | Distinct Ask/Request/Wait actions and local On clauses | Same distinctions under repeated When/then | [RP-01](../authoring/benchmarks.md#rp-01--reimbursement-including-corrections-and-uncertainty); reader accuracy unmeasured |
| Author | Explicit action plus one clause per result | Entry rule plus one local result rule per result | Neither creates fewer required policies; complete authoring not performed |
| Change | Outcome target is locally explicit; no inferred next line | Result-rule target equally explicit | Ordinary target/guard edits can be compared on paper; no editor/rename execution |
| Diagnose | Unsupported natural phrases must refuse | Same refusal, plus global-trigger interpretation risk | [Refusal table](../authoring/syntax.md#7-required-diagnostics-not-implemented-messages); messages not implemented |
| Round-trip | Ledger/complete field map proposed | Same ledger and fields | No transformation observed; companion encoding remains open |
| Review | Control wording separates result from a general condition | Uniform When may resemble an event engine | Hypothesis requiring C2/C3/R2 participant tasks, not a finding about users |
| Scale | One fan-out sentence avoids one source block per item | Same template, with a longer entry prefix | Both still need seal/settlement clauses; no large-instance performance claim |
| Access | Text and explicit end markers do not rely on indentation | Same | No screen-reader/keyboard study; text availability alone does not pass G4 |

All six frozen perturbation classes have expected paper outcomes in the
[benchmark table](../authoring/benchmarks.md#common-perturbations-and-paper-outcomes).
These are reasoning results, not runtime traces. The same examples also exposed
limits: calendar generation, exact order aggregation, restricted item interfaces,
ad hoc templates and partial-order import. No silent semantic extension was made.

## Single-evaluator criterion profile

Notation is fitness / evidence / confidence. `Unscored` is deliberately not zero.
All scored judgments are B-strength desk reasoning; no score comes from tests of
the old parser. No totals, averages, weighted ranking or inferred user preference.

| Criterion | A | B | Rationale |
| --- | --- | --- | --- |
| S1 Corpus adequacy | 2 / B / Low | 2 / B / Low | All forty partial; whole-case treatments but no complete lowerable benchmarks |
| S2 Precision/analyzability | 2 / B / Low | 2 / B / Low | Exact names and closed map, but full grammar/validation remain open |
| S3 Economy/orthogonality | 2 / B / Low | 2 / B / Low | Shared finite model; A has explicit verbs, B repeats condition wrappers; both verbose |
| S4 Composition/scale | 2 / B / Low | 2 / B / Low | Local scopes and keyed templates; ports/linkage restrictions remain |
| S5 Effects/failure/protection | 2 / B / Low | 2 / B / Low | Obligations exposed, no real enforcement or guarded-commit proof |
| S6 Identity/evolution | 2 / B / Low | 2 / B / Low | Stable ledger proposal with failure rules, no schema/rename/merge implementation |
| H1 Cognitive fit | Unscored / A / Low | Unscored / A / Low | Zero representative participants |
| H2 Visibility/changeability | 2 / B / Low | 2 / B / Low | Local clauses visible; packaging and policy density need human/tool evidence |
| T1 Textual effectiveness | 2 / B / Low | 2 / B / Low | Worked readable fragments, exact quotes; no complete source grammar or stable formatter |
| V1 Visual effectiveness | Unscored / A / Low | Unscored / A / Low | No visual design in this deliverable |
| V2 Visual complexity | Unscored / A / Low | Unscored / A / Low | No visual/large-process navigation evidence |
| X1 Engineering integrity | 2 / B / Low | 2 / B / Low | Full field obligations mapped; lowering, package identity and round trips unimplemented |

No independent evaluator or disagreement reconciliation was available. Equal
coarse scores do not mean identical usability; they indicate insufficient evidence
to separate candidates confidently. The recommendation is a reversible design
judgment, not a numerical winner or a passed final-selection gate.

## Risks and recommendation

Develop A provisionally because its local action/outcome phrasing states the
control boundary directly. Keep B in matched user tasks. Favor B or revise A if
representative readers/creators show fewer critical errors with B. Revisit quoted
names or companion packaging if predictable plain-editor changes prove too hard.

High-impact risks are hidden policy meaning, assumed English semantics, identity
loss, and unsupported whole-case requirements disguised as named templates.
Mitigation is explicit draft refusal, complete future grammar/contract evidence,
portable identity tests and participant tasks. A readable fragment cannot settle
any of these alone. No compatibility commitment makes this direction cheap to
revise; a production compiler or published language version would change that.

The [study protocol](../authoring/study.md) requires consent, independent task-key
review, complete bounded materials and de-identified results. The Project Owner
must coordinate participants. We have not contacted people or claimed their
experience. Full benchmark materials are also still prerequisites, not completed
by the study plan itself.

## Reproduction and verification

From the repository root, run `python3 tools/check_authoring_evidence.py`.
It checks forty unique corpus IDs/statuses and all ten core kinds across eleven
mapping rows, including separate human and capability activity rows. It also
checks required documents, local Markdown target files/anchors and the unchanged frozen-plan
bytes. It is **not** a grammar, semantics, accessibility or lowering validator.
Existing Rust tests, Clippy/native/Wasm CI and Python wire/text/snapshot checks
are regressions for the unchanged implementation; record actual results below
after running them. No browser execution or speed/memory claim is intended.

Observed locally on 2026-09-22: all six evidence-inventory tests passed, including
missing/duplicate scenario, unknown kind and broken-link mutations. All 40
existing Rust unit/integration tests and three compile-fail doc tests passed;
formatting, native workspace Clippy and contract-probe Wasm Clippy passed.
The 16 Python wire tests, four contract-snapshot integrity groups and text oracle
(three IR/schema/JCS comparisons, three stable exports, seven CLI refusals) passed.
`git diff --check` passed. No existing crate, dependency, fixture or immutable
contract bytes changed. These observations do not promote either paper candidate
to executable or representative-user evidence.

## Follow-through

- [ ] Owner reviews ADR-0014's four questions and provisional scope.
- [ ] Full grammar/binding-package specification and complete source/IR benchmarks.
- [ ] Independent task-material review and coordinated participant study.
- [ ] Reassess evidence/gates before final selection or Roadmap completion.
- [ ] Obtain permission before starting the visual-notation item.
