<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Choreoform design-evaluation framework

**Status:** Adopted<br>
**Decision record:** [ADR-0007](../decisions/0007-design-evaluation-framework.md)

This framework compares candidate semantic models, textual languages, visual
notations, canonical representations, and mappings between them. It turns the
[representative process corpus](../representative-processes/README.md) into a
repeatable review method without pretending that design judgment is arithmetic.

Use [the scorecard template](scorecard-template.md) for each comparison. Use the
same frozen plan and benchmark inputs for every candidate in that comparison.

## What the framework answers

The framework helps reviewers answer four different questions without
collapsing them:

1. **Is the candidate admissible?** Five gates protect foundational project
   requirements.
2. **Where is it strong or weak?** Twelve criteria produce a quality profile.
3. **How well do we know?** Evidence strength and confidence qualify each
   assessment.
4. **What should Choreoform decide?** Reviewers synthesize trade-offs and risks;
   the governing ADR makes and explains the decision.

It does not certify correctness, conformance, usability, accessibility,
security, safety, or regulatory fitness. Those claims require purpose-specific
specifications, tests, and representative evaluation beyond this framework.

## Before evaluating: freeze the plan

Record the plan before reviewing candidate results. At minimum it names:

- the exact decision question and what is deliberately outside it;
- candidate revisions and a common maturity level;
- target roles, experience assumptions, and the activities being evaluated;
- the corpus revision and all acceptance scenarios in scope;
- a common benchmark set containing at least one Level 1 or 2 case, one Level 3
  case, and one Level 4 case from at least three domains;
- common perturbations and tasks;
- required artifacts, tools, environments, and time or resource limits;
- evaluators, conflicts of interest, and how disagreements will be handled; and
- any weights or selection rules, justified before results are visible.

Candidates at different maturity levels may be explored, but do not present
their scores as a like-for-like comparison. A paper design can support reasoned
worked examples; prototype behavior, performance, round trips, diagnostics, and
user outcomes require executable or empirical evidence.

## Common evidence package

Each candidate supplies one identifiable package containing:

1. a semantic account of valid, invalid, deterministic, nondeterministic, and
   externally observable behavior;
2. its conceptual vocabulary and relationship to the working glossary;
3. textual and visual notation references with explicit representation-only
   metadata;
4. the canonical mapping and transformation or round-trip rules;
5. a traceability row for every adopted corpus acceptance scenario;
6. full textual and visual representations of the frozen benchmark cases;
7. results for the common tasks and perturbations;
8. diagnostics, histories, diffs, or traces needed to explain outcomes;
9. implementation outline, dependencies, performance assumptions, and known
   limits; and
10. candidate-authored risks and unresolved questions.

An artifact may be prose, formal notation, test, prototype, recording, study,
or measured result. Label simulated, inferred, and observed evidence distinctly.

## Corpus traceability

Map all adopted acceptance scenarios, not only the fully modeled benchmark
cases. Use exactly one primary status and explain every result other than
Supported.

| Status | Meaning |
| --- | --- |
| Supported | The candidate explains how the scenario's outcome and invariants are represented without an identified material gap |
| Partial | The direction is credible, but a named semantic, notational, tooling, or evidence gap remains |
| Outside scope | The proposal deliberately excludes the pressure and explains the boundary and consequence |
| Unknown | The proposal has not yet established an answer |

Traceability is not a feature-count contest. Several scenarios may be handled
by one general concept, and a candidate should prefer coherent composition over
one construct per case. Unsupported results may reveal a justified Choreoform
boundary, but they cannot be hidden.

## Non-compensable gates

Assign Pass, Conditional, or Fail and cite evidence. A conditional result must
name closure evidence and an owner. A candidate cannot be selected as a final
design while any gate remains Conditional or Fail.

### G1 — Corpus accountability

Every adopted scenario is mapped consistently, and partial, outside-scope, and
unknown results have consequences and rationale. Full benchmark representations
preserve their case invariants rather than only drawing the main path.

### G2 — Semantic accountability

The candidate makes enough meaning explicit to distinguish conflicting
interpretations of ordering, concurrency, time, data and state, human work,
failure, cancellation, effects, and observable history. It identifies invalid
definitions and the source of permitted nondeterminism.

### G3 — Cross-form parity

Text and visuals express the same canonical meaning. Any semantic distinction
available in one has a faithful expression or operation in the other.
Formatting and layout metadata are identified as non-semantic unless the
proposal explicitly justifies otherwise. Round trips do not silently invent,
drop, or change meaning.

### G4 — Protected and accessible meaning

Authority, capabilities, sensitive information, external effects, uncertainty,
and irreversible action remain visible at the appropriate boundary. Essential
meaning and operations never depend only on color, shape, position, connector
routing, pointer manipulation, or one sensory mode. A structured textual path
can expose and edit the complete semantic model.

This gate applies notation-level accessibility principles. Product interfaces
must later define and test full requirements, including applicable WCAG success
criteria, keyboard operation, focus, programmatic relationships, contrast,
zoom, reflow, and evaluation with people with disabilities.

### G5 — Reproducible comparison

Candidate revision, inputs, examples, tools, results, assumptions, and failures
are identifiable and reviewable. Another contributor can reconstruct a worked
result at the evidence level being claimed. Unknowns remain unknown rather than
being inferred as success.

## Fitness scale

Score each criterion independently. Scores are ordered judgments, not precise
measurements; do not calculate percentage differences between them.

| Score | Anchor |
| --- | --- |
| 0 — Unacceptable | Contradicts the criterion or has a material unmitigated failure |
| 1 — Major weakness | Addresses a narrow happy path but requires foundational redesign or creates severe trade-offs |
| 2 — Mixed or conditional | Credible direction with important gaps, exceptions, complexity, or uncertain trade-offs |
| 3 — Strong | Satisfies the criterion across representative evidence with manageable, explicit trade-offs |
| 4 — Compelling | Satisfies diverse and adversarial evidence unusually well with simple rationale and no material weakness found |

## Evidence strength and confidence

Record these separately for every criterion.

| Grade | Evidence strength |
| --- | --- |
| A — Assertion | Design claim or expert expectation without a worked result |
| B — Worked | Reasoned examples, mappings, or analyses that another reviewer can inspect |
| C — Reproduced | Executable prototype, repeatable test, measured result, or independently reproduced transformation |
| D — Evaluated | Independent review or a study with relevant representative users and recorded method and results |

Evidence grades are not a maturity ladder every criterion must climb in the
same way. Formal semantics may rely on proof and executable counterexamples;
learnability requires people. Record confidence as **Low**, **Medium**, or
**High** based on relevance, coverage, repeatability, and agreement—not merely
the volume of artifacts.

## Criterion profile

### S1 — Corpus adequacy

**Question:** Does the design account coherently for the adopted process
pressures and their interactions?

Look for coverage of ordinary and failure paths, invariants, timing,
concurrency, human judgment, physical and software work, dynamic scope, and
explainable outcomes. Penalize feature-by-feature patching that lacks a general
model.

### S2 — Precision and analyzability

**Question:** Can contributors determine what a definition means, when it is
invalid, and what behavior may be observed?

Look for operational or otherwise testable semantics, deterministic validation,
explicit nondeterminism, useful diagnostics, analyzable properties, and
counterexamples for subtle races or failures.

### S3 — Economy and orthogonality

**Question:** Does a small, coherent set of concepts compose without accidental
overlap or special cases?

Look for minimal core commitments, consistent rules, low redundancy, few
context-sensitive exceptions, and clear justification for convenience forms.
Economy does not mean minimizing characters or symbols at the cost of meaning.

### S4 — Composition and scale

**Question:** Can people define, reuse, parameterize, navigate, and reason about
small and large processes locally?

Look for abstraction boundaries, explicit interfaces, modular validation,
dynamic cardinality, hierarchical views, and behavior that does not require an
enormous fully expanded graph.

### S5 — Effects, failure, and protection

**Question:** Does the design make authority and uncertain or harmful effects
explicit across failure and recovery?

Look for capability boundaries, least privilege, effect identity, retries,
timeouts, outcome uncertainty, cancellation, compensation, manual intervention,
data minimization, and history that never fictionalizes rollback.

### S6 — Identity, evolution, and interoperability

**Question:** Can definitions, instances, artifacts, extensions, and external
facts evolve without silent semantic change?

Look for stable identity, version binding, migration and compatibility policy,
canonical serialization, extension boundaries, durable-instance treatment, and
portable meaning independent of one engine or editor.

### H1 — Cognitive fit by role and task

**Question:** How well does the combined notation support the actual activities
of relevant beginners, expert authors, reviewers, operators, and debuggers?

Assess learning, comprehension, authoring, modification, search, review,
communication, and diagnosis separately. Do not assume one representation or
one notation density fits every role and activity.

### H2 — Visibility and changeability

**Question:** Can people find relevant information and make predictable changes
without excessive work or hidden consequences?

Look for visible dependencies, locality, progressive disclosure, juxtaposable
views, low viscosity, useful consistency, error-proneness, premature commitment,
and the ability to trace a small edit through semantics and artifacts.

### T1 — Textual effectiveness

**Question:** Is the textual form effective as durable, human-authored source?

Look for readable structure, writable conventions, restrained punctuation,
local context, stable formatting, meaningful diffs and merges, precise source
locations, completion and navigation potential, refactorability, and accessible
operation with ordinary text and source-control tools.

### V1 — Visual effectiveness

**Question:** Does the visual vocabulary communicate semantic distinctions
quickly, accurately, and with explicit rationale?

Look for one-to-one concept/symbol discipline where appropriate, perceptual
discriminability, semantic transparency, restrained symbol count, dual coding,
use of the full visual-variable repertoire, and notation choices tested with
target users rather than justified only by familiarity.

### V2 — Visual complexity management

**Question:** Can the visual form retain comprehension as definitions grow and
cross-cutting concerns appear?

Look for meaningful hierarchy, overview and detail, filtering, layers,
consistent navigation, crossing and distance management, legible labels,
traceable off-page relationships, and layout changes that preserve meaning.

### X1 — Cross-form and engineering integrity

**Question:** Can the design be implemented and synchronized without drift,
fragile heuristics, or unreasonable cost?

Look for deterministic canonical mapping, stable round trips, source/visual
traceability, incremental validation, error recovery, performance and storage
feasibility, maintainable component boundaries, test strategy, and room for
free-software implementations and independent extensions.

## Common tasks and perturbations

Run the same tasks against every candidate at the maturity level being claimed:

1. **Comprehend:** explain the possible outcomes, pending obligations, and one
   subtle failure path from text alone and from visuals alone.
2. **Author:** create the same benchmark process from a concise case brief in
   each form.
3. **Change:** add a deadline, revise an approval-bound artifact, and change a
   parallel branch so reviewers can inspect viscosity and semantic impact.
4. **Diagnose:** locate and explain an invalid definition, a race, an ambiguous
   external outcome, and a stale or duplicate event.
5. **Round-trip:** transform text to canonical model to visual form and back,
   then repeat after changes from both editors.
6. **Review:** compare revisions, identify semantic changes, and distinguish
   them from formatting or layout movement.
7. **Scale:** navigate a parameterized or dynamically expanded process without
   requiring every runtime item on the authoring canvas.
8. **Access:** perform equivalent comprehension and editing without color,
   precise pointer gestures, or spatial inspection alone.

Freeze the exact briefs and expected observations in the scorecard. Include at
least these perturbation classes: late or duplicate event, partial failure,
cancellation during concurrent work, decision-relevant revision, definition
version change during a long-running instance, and a large fan-out.

## Review and synthesis procedure

1. Confirm plan, candidate maturity, artifacts, and evaluator conflicts.
2. Map every corpus acceptance scenario before scoring.
3. Evaluate gates. Stop final selection for any Conditional or Fail result, but
   retain useful partial analysis.
4. Run the frozen benchmark tasks and record raw observations.
5. Have at least two reviewers score independently when feasible. A single
   reviewer must label the limitation.
6. Discuss criterion differences greater than one point. Preserve unresolved
   scores and reasons; do not silently average them.
7. Summarize dominant strengths, weaknesses, irreversible risks, closure work,
   and sensitivity to any weights.
8. Make the decision through the normal ADR process. Link the scorecard and
   explain judgment, especially any departure from the profile.

## Interpreting a profile

Do not report a default total, average, percentage, or winner. Compare criterion
profiles, evidence grades, confidence, gates, and risks. A design with several
3/B/Medium assessments may be more decision-ready than one with a speculative
4/A/Low and unresolved gates.

If an evaluation plan uses weights, report the unweighted profile beside the
weighted result and test whether reasonable weight changes reverse the outcome.
No weight can compensate for a failed gate.

Treat close profiles as evidence that prototyping or user study is more useful
than finer scoring. Record what observation would change the decision.

## Maintaining the framework

Routine changes may clarify prompts, add examples, or improve source links. A
material change to gate meaning, the fitness or evidence scales, the required
corpus accountability, or the default prohibition on aggregate totals requires
an ADR.

Calibrate the framework after each substantive use. Record criteria that were
ambiguous, duplicated, impractical, or unable to distinguish candidates. Do not
rewrite a completed scorecard when the framework changes; retain the framework
revision it used.

## Foundations and limits

This framework adapts, rather than adopts wholesale:

- Workflow Patterns as a precedent for evaluating language support against
  recurring control, data, resource, interaction, and exception pressures;
- Cognitive Dimensions of Notations for activity-sensitive qualities such as
  viscosity, visibility, hidden dependencies, and premature commitment;
- the Physics of Notations for visual discriminability, semantic transparency,
  graphic economy, complexity management, and cognitive fit;
- ISO/IEC 25010's use of explicit quality characteristics in requirements and
  evaluation; and
- WCAG 2.2's testable, technology-neutral approach and requirements for text
  alternatives, programmatic relationships, non-color meaning, keyboard
  operation, and distinguishability.

These sources overlap and sometimes trade off. They do not prove that the
twelve criteria are complete, independent, or equally important. Choreoform
must validate the framework through actual design comparisons and studies with
the people expected to use the language and Studio.

## Sources

- [Workflow Patterns](https://mitpress.mit.edu/9780262029827/workflow-patterns/)
- [Cognitive Dimensions of Notations resource site](https://www.cl.cam.ac.uk/~afb21/CognitiveDimensions/)
- [Thomas Green and Alan Blackwell, “Cognitive Dimensions of Information Artefacts: a tutorial”](https://www.cl.cam.ac.uk/~afb21/CognitiveDimensions/CDtutorial.pdf)
- [Daniel Moody, “The Physics of Notations”](https://doi.org/10.1109/TSE.2009.67)
- [ISO/IEC 25010:2023 product quality model](https://www.iso.org/standard/78176.html)
- [W3C Web Content Accessibility Guidelines 2.2](https://www.w3.org/TR/WCAG22/)
- [OMG Business Process Model and Notation 2.0.2](https://www.omg.org/spec/BPMN/2.0.2/)
