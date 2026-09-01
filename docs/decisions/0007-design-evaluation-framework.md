<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0007: Evaluate semantic and notation designs with evidence profiles

**Status:** Accepted<br>
**Date:** 2026-09-01<br>
**Decider:** Project owner

## Context

Phase 1 will require consequential choices about Choreoform's semantic model,
canonical representation, textual language, and visual notation. Those choices
interact: a visually attractive design may obscure ambiguous execution
semantics, while a formally strong design may be unnecessarily difficult to
read, change, review, or express without sight.

ADR-0006 established ten representative processes as shared requirements
evidence. It deliberately did not define how competing designs should be
compared. Without a method agreed before candidate designs are scored, reviewers
could choose favorable examples, change weights after seeing results, collapse
incomparable qualities into one total, or let strength in one area compensate
for a foundational failure in another.

Established bodies of work address parts of the problem. Workflow Patterns
provides structured coverage questions; Cognitive Dimensions examines the
usability of notations for different activities; the Physics of Notations
addresses cognitively effective visual vocabularies; ISO/IEC 25010 supplies a
general product-quality model; and WCAG supplies technology-neutral,
test-oriented accessibility requirements. None is by itself a selection method
for a dual-form executable process language.

## Decision criteria

The evaluation method should:

1. make semantic adequacy, cross-form equivalence, safety, and accessibility
   non-negotiable;
2. compare every candidate against the same corpus evidence and user tasks;
3. keep criterion fitness separate from evidence strength and reviewer
   confidence;
4. expose trade-offs rather than hide them in one aggregate score;
5. work for early paper proposals and become stricter as prototypes and user
   evidence become available;
6. resist cherry-picking, hindsight weighting, and undocumented exceptions;
7. remain light enough to use in repository-native design review; and
8. produce traceable artifacts that can support an ADR without replacing
   judgment.

## Decision

Adopt the
[Choreoform design-evaluation framework](../evaluation/README.md) for comparing
competing semantic, textual, visual, canonical-model, and cross-form designs.

The framework has four parts:

1. **A frozen evaluation plan.** Before scoring, reviewers record the decision
   question, candidate scope and maturity, target roles and tasks, corpus
   revision, common benchmark cases and perturbations, required evidence, and
   any criterion weights.
2. **Five non-compensable gates.** Corpus accountability, explicit semantics,
   cross-form parity, protected boundaries and accessibility, and reproducible
   evidence must pass. A conditional pass identifies closure work; a candidate
   cannot be selected as final while a gate remains conditional.
3. **A twelve-criterion profile.** Reviewers independently assess semantic,
   human, textual, visual, cross-form, and engineering qualities on an anchored
   0–4 fitness scale. They record evidence strength and confidence separately.
   The profile has no default weighted total.
4. **A repeatable comparison procedure.** Every candidate maps all corpus
   acceptance scenarios, fully works the same stratified benchmark cases,
   performs the same comprehension, authoring, change, diagnosis, and
   round-trip tasks, and receives an explicit risk and trade-off synthesis.

The five gates are:

| Gate | Required outcome |
| --- | --- |
| G1 — Corpus accountability | Every adopted acceptance scenario is mapped as supported, partial, outside scope, or unknown; every non-supported result is explained |
| G2 — Semantic accountability | Meaning, invalidity, nondeterminism, state change, and observable effects are explicit enough to distinguish conflicting interpretations |
| G3 — Cross-form parity | Text and visuals can express the same semantic distinctions through one declared canonical meaning; representation-only metadata is identified |
| G4 — Protected and accessible meaning | Authority, sensitive information, and external effects remain explicit; no essential meaning depends only on color, geometry, pointing, or one sensory mode |
| G5 — Reproducible comparison | Candidate revision, inputs, artifacts, tools, results, assumptions, and unresolved failures are identifiable and available for review |

The twelve scored criteria are:

- **S1 Corpus adequacy** — fitness across the adopted processes and acceptance
  scenarios, including interactions rather than isolated feature claims.
- **S2 Precision and analyzability** — clarity of valid and invalid meaning,
  observable behavior, diagnostics, and feasible static or dynamic analysis.
- **S3 Economy and orthogonality** — a small coherent core with limited overlap,
  exceptions, accidental concepts, and equivalent ways to say the same thing.
- **S4 Composition and scale** — support for abstraction, reuse, locality,
  parameterization, dynamic cardinality, and navigation of large definitions.
- **S5 Effects, failure, and protection** — explicit capabilities, authority,
  uncertainty, retry, cancellation, compensation, privacy, and recovery.
- **S6 Identity, evolution, and interoperability** — stable identity,
  versioning, migration, serialization, extension, and long-running-instance
  behavior without silent semantic change.
- **H1 Cognitive fit by role and task** — fitness for learning, reading,
  authoring, review, operation, and debugging by relevant people.
- **H2 Visibility and changeability** — discoverable dependencies, local
  reasoning, manageable viscosity, useful abstraction, and predictable edits.
- **T1 Textual effectiveness** — readable and writable source, stable formatting
  and diffs, precise diagnostics, navigation, and conventional tooling.
- **V1 Visual effectiveness** — clear symbol-to-concept mapping, perceptual
  discriminability, restrained visual vocabulary, and explicit visual
  rationale.
- **V2 Visual complexity management** — hierarchy, filtering, overview and
  detail, traceable connections, and legibility without layout-dependent
  meaning.
- **X1 Cross-form and engineering integrity** — deterministic transformation,
  round-trip stability, traceability, automation feasibility, performance, and
  maintainable implementation boundaries.

Each criterion receives three independent annotations:

- **fitness:** 0 unacceptable, 1 major weakness, 2 mixed or conditional,
  3 strong, or 4 compelling;
- **evidence strength:** A assertion, B reasoned worked examples, C reproducible
  prototype or tests, or D independent evaluation or representative-user
  evidence; and
- **confidence:** low, medium, or high.

A score is not a measurement with interval-scale precision. Reviewers must give
a short rationale and cite artifacts. Scores separated by more than one point
are discussed; unresolved disagreement remains visible rather than being
silently averaged.

No criterion has a default weight and no overall total is reported. If a
particular decision genuinely needs different priorities, weights and selection
rules must be justified and frozen in the evaluation plan before candidate
results are reviewed. Gate failures can never be outweighed.

The framework informs, but does not make, architecture decisions. The deciding
ADR records the criterion profile, material disagreements, risks, and any
reasoned departure from the result. It must not claim that a high score proves
correctness, accessibility, usability, safety, or universality.

## Options considered

| Option | Advantages | Costs and risks | Outcome |
| --- | --- | --- | --- |
| **Gates plus evidence profiles** | Preserves fatal constraints and trade-offs; usable at several maturity levels; traceable | More review work; does not produce a simple winner automatically | Adopt |
| Single weighted score | Easy ranking and presentation | False precision; compensates fatal weaknesses; weights invite hindsight bias | Reject |
| Corpus pass/fail only | Directly connected to project evidence | Rewards feature coverage but misses quality, usability, economy, and implementation risk | Reject |
| Unstructured expert review | Fast and flexible | Inconsistent evidence, memory bias, and decisions that are difficult to reproduce | Reject |
| Adopt one external framework unchanged | Established terminology and precedent | Each covers only part of a dual-form executable language | Reject |

## Consequences

- Phase 1 proposals must expose semantic and representational weaknesses before
  a preferred design is selected.
- Text and visual forms are treated as peers sharing meaning, not as a source
  language and disposable rendering.
- Accessibility is considered at notation design time; Studio implementation
  will still require complete accessibility requirements and user testing.
- Review artifacts will be larger than an informal opinion but reusable for
  conformance tests, usability studies, and later compatibility decisions.
- Early proposals can be compared with worked examples, but low evidence
  strength and confidence remain visible until prototypes and user studies
  exist.
- The lack of an aggregate total may make close decisions harder to summarize;
  that friction is intentional when trade-offs are real.
- The criteria themselves will need empirical validation and refinement. A
  material change to gates, scoring semantics, or the default no-total rule
  requires an ADR; routine clarification and improved prompts use normal review.

## Confirmation

The decision is implemented when:

- the framework defines the five gates, twelve criteria, anchored fitness
  scores, evidence strengths, confidence labels, and comparison procedure;
- a reusable scorecard template captures the frozen plan, corpus traceability,
  independent reviews, disagreements, risks, and decision synthesis;
- repository entry points link to the framework;
- the first competing Phase 1 design review uses a completed scorecard; and
- the deciding ADR links the scorecard and explains any departure from it.

## Sources

- [Workflow Patterns](https://mitpress.mit.edu/9780262029827/workflow-patterns/)
- [Cognitive Dimensions of Notations resource site](https://www.cl.cam.ac.uk/~afb21/CognitiveDimensions/)
- [Daniel Moody, “The Physics of Notations”](https://doi.org/10.1109/TSE.2009.67)
- [ISO/IEC 25010:2023 product quality model](https://www.iso.org/standard/78176.html)
- [W3C Web Content Accessibility Guidelines 2.2](https://www.w3.org/TR/WCAG22/)
- [OMG Business Process Model and Notation 2.0.2](https://www.omg.org/spec/BPMN/2.0.2/)

## Acceptance and action items

The Project Owner approved this ADR on 2026-09-01. The design-evaluation
framework becomes effective when pull request #8 is merged.

1. [x] Obtain Project Owner approval.
2. [x] Change this ADR's status to Accepted and record the approval.
3. [x] Publish the evaluation framework and scorecard template.
4. [x] Link the framework from contributor entry points and the decision index.
5. [x] Mark the Phase 0 Roadmap deliverable complete after approval.
6. [ ] Use and calibrate the framework during the first Phase 1 design
   comparison.
