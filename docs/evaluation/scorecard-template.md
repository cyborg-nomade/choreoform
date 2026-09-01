<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Design evaluation: Decision question

**Status:** Planned | In progress | Complete<br>
**Framework revision:** Commit or release<br>
**Plan frozen:** YYYY-MM-DD<br>
**Evaluation owner:** Name or role

Copy this template once per comparison, not once per candidate. Keep the raw
observations and independent reviewer scores available beside the synthesis.
Delete instructional text that does not belong in the completed record.

## Decision and scope

- **Question:** Exact decision this comparison informs.
- **In scope:** Semantic, textual, visual, canonical, tooling, or other layers.
- **Out of scope:** Deferred questions and consequences.
- **Target decision ADR:** Link when available.
- **Selection authority:** Governance role.
- **Constraints:** Compatibility, schedule, implementation, licensing, safety,
  or other fixed boundaries.

## Candidates and maturity

| Candidate | Revision | Maturity | Authors | Declared limits |
| --- | --- | --- | --- | --- |
| A | Link or identifier | Paper / worked / prototype / evaluated | ... | ... |
| B | Link or identifier | ... | ... | ... |

Explain any maturity mismatch. Do not present unlike evidence as a like-for-like
ranking.

## Roles and activities

| Role | Assumed experience | Activities evaluated | Accessibility needs represented |
| --- | --- | --- | --- |
| Process reader | ... | Comprehension, review | ... |
| Process author | ... | Authoring, change | ... |
| Operator or debugger | ... | Diagnosis, history inspection | ... |

## Frozen evidence plan

- **Corpus revision:** Commit or release.
- **Acceptance scenarios:** All adopted scenarios, with exclusions forbidden
  unless the decision itself explicitly changes corpus scope.
- **Benchmark cases:** At least one Level 1/2, one Level 3, and one Level 4 case
  across at least three domains.
- **Task briefs:** Links or embedded immutable revisions.
- **Perturbations:** Late/duplicate event; partial failure; concurrent
  cancellation; decision-relevant revision; long-running version change; large
  fan-out; additional decision-specific cases.
- **Required artifacts:** Semantic account, both notations, mappings, diffs,
  diagnostics, prototype, study, or other evidence.
- **Environment and limits:** Tools, versions, hardware, time, assistance, and
  resource budgets.
- **Weights or selection rules:** None by default. If used, rationale and values
  fixed before results are reviewed.

## Evaluators and independence

| Evaluator | Role or relevant experience | Candidate relationship | Conflict or limitation |
| --- | --- | --- | --- |
| ... | ... | ... | ... |

Describe independent scoring, calibration, facilitation, and how representative
users were recruited or why they were not involved.

## Corpus traceability

Repeat this row for every adopted acceptance scenario. Use Supported, Partial,
Outside scope, or Unknown.

| Scenario | Candidate | Status | Concepts or representation | Evidence | Gap or consequence |
| --- | --- | --- | --- | --- | --- |
| RP-01-A | A | ... | ... | Link | ... |

Summarize counts only after retaining the scenario-level table.

| Candidate | Supported | Partial | Outside scope | Unknown | Material interactions or notes |
| --- | ---: | ---: | ---: | ---: | --- |
| A | ... | ... | ... | ... | ... |

## Gate review

Use Pass, Conditional, or Fail. A Conditional result names closure evidence and
an owner. Final selection stops until every gate passes.

| Gate | Candidate | Result | Evidence and rationale | Closure evidence, owner, and due condition |
| --- | --- | --- | --- | --- |
| G1 Corpus accountability | A | ... | ... | ... |
| G2 Semantic accountability | A | ... | ... | ... |
| G3 Cross-form parity | A | ... | ... | ... |
| G4 Protected and accessible meaning | A | ... | ... | ... |
| G5 Reproducible comparison | A | ... | ... | ... |

## Raw task observations

Record observations before scores. Separate what happened from interpretation.

| Task or perturbation | Candidate | Role | Observation | Artifact or measure | Unexpected result |
| --- | --- | --- | --- | --- | --- |
| Comprehend | A | ... | ... | Link | ... |
| Author | A | ... | ... | Link | ... |
| Change | A | ... | ... | Link | ... |
| Diagnose | A | ... | ... | Link | ... |
| Round-trip | A | ... | ... | Link | ... |
| Review | A | ... | ... | Link | ... |
| Scale | A | ... | ... | Link | ... |
| Access | A | ... | ... | Link | ... |

## Independent criterion assessments

Fitness uses 0–4; evidence uses A–D; confidence is Low, Medium, or High. Keep
each evaluator's original assessment before reconciliation.

| Criterion | Candidate | Evaluator | Fitness | Evidence | Confidence | Rationale and artifact |
| --- | --- | --- | ---: | :---: | :---: | --- |
| S1 Corpus adequacy | A | ... | ... | ... | ... | ... |
| S2 Precision and analyzability | A | ... | ... | ... | ... | ... |
| S3 Economy and orthogonality | A | ... | ... | ... | ... | ... |
| S4 Composition and scale | A | ... | ... | ... | ... | ... |
| S5 Effects, failure, and protection | A | ... | ... | ... | ... | ... |
| S6 Identity, evolution, and interoperability | A | ... | ... | ... | ... | ... |
| H1 Cognitive fit by role and task | A | ... | ... | ... | ... | ... |
| H2 Visibility and changeability | A | ... | ... | ... | ... | ... |
| T1 Textual effectiveness | A | ... | ... | ... | ... | ... |
| V1 Visual effectiveness | A | ... | ... | ... | ... | ... |
| V2 Visual complexity management | A | ... | ... | ... | ... | ... |
| X1 Cross-form and engineering integrity | A | ... | ... | ... | ... | ... |

Repeat the twelve rows for every candidate and evaluator.

## Disagreements and calibration

Record every fitness difference greater than one point and any disagreement
that could change the decision.

| Criterion and candidate | Assessments | Shared facts | Remaining disagreement | Evidence that could resolve it |
| --- | --- | --- | --- | --- |
| ... | ... | ... | ... | ... |

Do not silently average unresolved scores.

## Comparison profile

Present criterion profiles, not a default total. A range preserves unresolved
reviewer variation.

| Criterion | Candidate A fitness / evidence / confidence | Candidate B fitness / evidence / confidence | Dominant trade-off |
| --- | --- | --- | --- |
| S1 | ... | ... | ... |
| S2 | ... | ... | ... |
| S3 | ... | ... | ... |
| S4 | ... | ... | ... |
| S5 | ... | ... | ... |
| S6 | ... | ... | ... |
| H1 | ... | ... | ... |
| H2 | ... | ... | ... |
| T1 | ... | ... | ... |
| V1 | ... | ... | ... |
| V2 | ... | ... | ... |
| X1 | ... | ... | ... |

If the frozen plan uses weights, show the unweighted profile, calculation,
sensitivity analysis, and every plausible weight change that reverses the
result. Never include a failed gate in a weighted comparison.

## Risks, unknowns, and reversibility

| Candidate | Risk or unknown | Likelihood or uncertainty | Impact | Reversibility | Mitigation or next evidence |
| --- | --- | --- | --- | --- | --- |
| A | ... | ... | ... | ... | ... |

Call out compatibility commitments, ecosystem lock-in, implementation cost,
accessibility risk, security boundaries, and claims that still have only A- or
B-strength evidence.

## Synthesis and recommendation

- **Gate result:** Candidate-by-candidate summary.
- **Dominant strengths:** Qualities supported by the most relevant evidence.
- **Dominant weaknesses:** Material gaps and trade-offs.
- **Decision sensitivity:** What reasonable priority or assumption changes
  would alter the result.
- **Recommendation:** Select, reject, prototype further, combine only if
  coherent, or defer.
- **What would change this recommendation:** Specific new evidence.

The recommendation informs the target ADR; it does not make the decision.

## Decision follow-through

- [ ] Link the deciding ADR and its outcome.
- [ ] Explain any departure from this evaluation.
- [ ] Convert accepted semantic claims into specifications and conformance
  tests.
- [ ] Convert usability and accessibility unknowns into representative studies.
- [ ] Record calibration findings for the framework without rewriting this
  completed evaluation.
