<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Design evaluation: Core process semantic model

**Status:** Complete<br>
**Framework revision:** `f62dca5`<br>
**Plan frozen:** 2026-09-02<br>
**Evaluation owner:** Proposal author

This is a single-candidate admissibility review, not a competitive ranking. No
other complete Phase 1 semantic proposal was available at the same maturity.
The Project Owner independently reviewed the proposal and required corrections
to its cross-form gate, dynamic-scope rules, post-closure reconciliation, and
four open design questions before approval. The alternatives in
[ADR-0008](../decisions/0008-core-process-semantics.md) are architectural
families, not scored candidates. This limitation lowers confidence and makes
independent review and executable evidence important follow-up work.

## Decision and scope

- **Question:** Is the hierarchical transition-system model in ADR-0008 a
  sufficiently explicit, economical semantic foundation for Choreoform?
- **In scope:** Control, data, actors, time, faults, cancellation, capabilities,
  side effects, history, and their interactions.
- **Out of scope:** Concrete types and expressions, IR encoding, textual and
  visual notation, runtime architecture, provider protocols, and migration of
  running instances.
- **Target decision ADR:** [ADR-0008](../decisions/0008-core-process-semantics.md).
- **Selection authority:** Project owner.
- **Constraints:** The Manifest, ADR-0005 vocabulary, the adopted corpus, and
  the non-compensable gates in ADR-0007.

## Candidate and maturity

| Candidate | Revision | Maturity | Authors | Declared limits |
| --- | --- | --- | --- | --- |
| HTS — hierarchical transition system with explicit obligations | [ADR-0008 decision](../decisions/0008-core-process-semantics.md) | Paper design with worked mappings | Proposal author | No executable rules, notation, IR, or independent criterion profile yet |

## Roles and activities

| Role | Assumed experience | Activities evaluated | Accessibility needs represented |
| --- | --- | --- | --- |
| Process reader | Understands the corpus domain, not formal methods | Explain outcomes, obligations, and races | Complete structured-text access; no color or geometry dependency |
| Process author | Understands processes and typed data | Map cases, revise inputs, add time and parallel work | Every semantic attribute has a textual representation |
| Operator or debugger | Understands runtime histories | Diagnose stale events, uncertain effects, and cancellation | History and current state remain distinguishable without spatial inspection |
| Engine implementer | Familiar with transition systems | Identify valid steps and conformance obligations | Formal notation must have a prose and structured-data equivalent |

## Frozen evidence plan

- **Corpus revision:** `f62dca5`; all forty adopted acceptance scenarios.
- **Benchmark cases:** RP-01 (Level 1), RP-03 (Level 3), and RP-08 (Level 4),
  spanning administration, commerce/logistics, and security operations.
- **Tasks:** Explain, model, change, diagnose, review, and scale the benchmark
  semantics. Surface-notation authoring and executable round trips are assessed
  only as semantic representability, not as implemented behavior.
- **Perturbations:** Late or duplicate observation, partial failure,
  cancellation during concurrent work, decision-relevant revision, definition
  change during a long-running instance, and large dynamic fan-out.
- **Required artifacts:** ADR-0008, this scenario mapping, gate review, raw
  observations, risk register, and criterion profile.
- **Environment and limits:** Repository-native prose review; no prototype or
  user study; one evaluator.
- **Weights or selection rules:** None.

## Evaluators and independence

| Evaluator | Role or relevant experience | Candidate relationship | Conflict or limitation |
| --- | --- | --- | --- |
| Proposal author | Architecture and semantic analysis | Authored candidate | Not independent; no representative-user study or second reviewer |
| Project Owner | Selection authority and design reviewer | Independent of proposal authorship | Reviewed gates and semantic gaps; did not produce a separate twelve-criterion score profile |

## Corpus traceability

All rows use **Supported** to mean that the paper model names a coherent
semantic account, not that implementation or conformance has been demonstrated.

| Scenario | Status | Concepts or representation | Evidence or remaining maturity gap |
| --- | --- | --- | --- |
| RP-01-A | Supported | Revision-bound human decision, authority-at-completion check, stable payable effect, attributable history | Formal rule and executable case remain |
| RP-01-B | Supported | Immutable data revisions plus declared approval invalidation dependencies | Change-impact syntax remains |
| RP-01-C | Supported | Unknown effect outcome, stable effect identity, mandatory reconciliation before retry | Capability protocol remains |
| RP-01-D | Supported | Atomic acceptance order resolves the race; stale observation is retained without enabling payment | Race conformance case remains |
| RP-02-A | Supported | Separate timer measurements and named completion outcome | Calendar implementation remains |
| RP-02-B | Supported | Warning and breach are separate timer transitions and immutable observations | Worked timer notation remains |
| RP-02-C | Supported | Assignment revisions reject stale acceptance as a state change while retaining history | Formal stale-event rule remains |
| RP-02-D | Supported | Pause names only affected timers, with actor, reason, and interval | Calendar/pause policy language remains |
| RP-03-A | Supported | Parallel child obligations, per-line dynamic identity, completion predicate, bounded capture effect | Executable join example remains |
| RP-03-B | Supported | Named failure outcome creates a visible void/expiry compensation obligation | Compensation convenience design remains |
| RP-03-C | Supported | Scoped cancellation, racing packed observation, and explicit capture/refund obligations | Provider-specific stopping facts remain external |
| RP-03-D | Supported | Observation identity applies a confirmed effect state change at most once | Deduplication storage remains |
| RP-04-A | Supported | Parallel scoped work, protected data cells, date gate, and separately confirmed custody/effects | Authorization enforcement remains |
| RP-04-B | Supported | Timer and input revisions reschedule dependent work without repeating unaffected completed obligations | Dependency analysis remains |
| RP-04-C | Supported | Revision dependencies invalidate only affected approval/provisioning sufficiency | Static impact declarations remain |
| RP-04-D | Supported | Cancellation settles each parallel child and links explicit disable/recall compensation | Full compensation library remains |
| RP-05-A | Supported | External clinician decision bound to evidence revision and explicit responsibility-acceptance effect | Domain policy remains external |
| RP-05-B | Supported | Timer returns control to human work and cannot manufacture a clinical outcome | Calendar policy remains |
| RP-05-C | Supported | New evidence revision creates a new attributable decision while retaining the former one | View design remains |
| RP-05-D | Supported | Message delivery and responsibility acceptance are distinct effects/observations | Adapter contract remains |
| RP-06-A | Supported | Dynamic per-unit obligations, independent system/physical facts, and completion predicate over disposition | Dynamic-scope IR remains |
| RP-06-B | Supported | Stable item keys retain existing child identities and valid work across collection revisions; new keys add work and removed keys require explicit settlement | IR encoding and executable collection-diff tests remain |
| RP-06-C | Supported | Conflicting observations coexist; invariants prevent digital status from satisfying physical containment | Conflict-resolution policy remains domain data |
| RP-06-D | Supported | Unknown release outcome forces reconciliation before another effect | Capability protocol remains |
| RP-07-A | Supported | Deterministic dynamic fan-out; scheduling limits do not change semantic child identity | Scale prototype remains |
| RP-07-B | Supported | Attempt identity is separate from logical work/effect; unknown result reconciles before retry | Lease/checkpoint providers remain external |
| RP-07-C | Supported | Pause stops selected scheduling; exact definition/artifact binding permits reuse of completed items | Planning rules remain |
| RP-07-D | Supported | Cancellation cannot erase confirmed publication and creates explicit restoration/correction work | Recovery backend remains |
| RP-08-A | Supported | Parallel objectives and work settle before closure; remaining obligations can transfer explicitly | Dynamic-work notation remains |
| RP-08-B | Supported | Versioned scope data and stable item keys instantiate declared templates without invalidating unrelated completed work | Template system and executable tests remain |
| RP-08-C | Supported | Capability authority is checked independently of control eligibility; recommendation is only data | Authorization system remains |
| RP-08-D | Supported | Imported observations preserve provenance and partial order; acceptance order does not invent outside chronology | Reconciliation UX remains |
| RP-09-A | Supported | Parallel decisions bind to one artifact/fact snapshot and a join predicate gates signature | Formal snapshot typing remains |
| RP-09-B | Supported | Declared invalidation dependencies retain unaffected approvals and withdraw affected sufficiency | Impact-analysis tooling remains |
| RP-09-C | Supported | Concurrent decision outcomes remain separate; no last-write-wins; explicit resolver obligation is created | Resolver policy remains |
| RP-09-D | Supported | Unknown signature effect retains package identity and blocks a new consequential request until reconciliation | Provider adapter remains |
| RP-10-A | Supported | One stable cycle/payment effect and a separate entitlement effect; duplicate application is prohibited | Capability details remain |
| RP-10-B | Supported | Attempts share logical effect identity; success cancels remaining timer obligations | Retry policy syntax remains |
| RP-10-C | Supported | Timer and cancellation race through atomic accepted observations and policy-bound guards | Exact policy remains configuration |
| RP-10-D | Supported | Ordinary closure may retain a reconciliation subscription whose late fact creates an explicit outcome; fully terminal instances require a linked new instance | Persistence and intake architecture remain |

| Candidate | Supported | Partial | Outside scope | Unknown | Material interactions or notes |
| --- | ---: | ---: | ---: | ---: | --- |
| HTS | 40 | 0 | 0 | 0 | Support is reasoned paper evidence; every scenario still needs executable conformance evidence |

## Gate review

| Gate | Result | Evidence and rationale | Follow-up evidence |
| --- | --- | --- | --- |
| G1 — Corpus accountability | Pass | Every adopted scenario is mapped above, including failure, race, revision, and recovery paths | Convert mappings into conformance cases |
| G2 — Semantic accountability | Pass | ADR-0008 defines configuration, atomic step, invalidity, ordering, nondeterminism, data change, actor work, fault, cancellation, and effect observability | Publish formal transition rules and counterexamples |
| G3 — Cross-form parity | Conditional | The semantic model is representation-neutral, but no textual or visual notation or round trip exists, so parity has not been demonstrated | IR, textual, and visual owners must provide equivalent benchmark representations and passing round trips before the Phase 1 exit review |
| G4 — Protected and accessible meaning | Pass | Authority, a minimum protection envelope, capability boundaries, uncertainty, and irreversible action are semantic attributes; no core rule depends on color, shape, position, pointing, or spatial reading | Policy enforcement and notation accessibility require separate evidence |
| G5 — Reproducible comparison | Pass | Revision, scope, scenario rows, assumptions, observations, limitations, risks, and Project Owner corrections are repository-visible | Independent reproduction and executable evidence are still needed for confidence |

## Raw task and perturbation observations

| Task or perturbation | Observation | Interpretation | Remaining evidence |
| --- | --- | --- | --- |
| Comprehend RP-01 | Possible terminal outcomes, correction obligations, approval revisions, and an unknown payable outcome are distinct | Small cases remain explainable despite richer effect semantics | Reader study |
| Model RP-03 | Per-line child scopes, a join predicate, and linked compensation obligations account for split fulfillment and cancellation | Parallel and effect semantics compose without an order-specific construct | Executable model |
| Model RP-08 | Versioned scope collections instantiate declared work templates; high-impact actions still require capability authority | Dynamic response need not permit arbitrary runtime code injection | Large worked graph |
| Change decision-relevant data | A new data revision atomically withdraws the sufficiency of only decisions named by invalidation dependencies | History and selective reuse are compatible | Dependency-language prototype |
| Diagnose ambiguous choice | Zero or multiple true guards fault unless an explicit otherwise rule resolves the decision | Accidental branch order is not semantic | Diagnostic prototype |
| Late or duplicate observation | Stable observation identity and acceptance position retain the fact but prevent a second state change or silent regression | Delivery behavior is separated from fact semantics | Reordered-event tests |
| Partial failure | Successful siblings remain completed while fault policy creates cancellation or recovery obligations for the rest | Failure does not fictionalize rollback | Nested-scope tests |
| Concurrent cancellation | The cancellation request and completion observation serialize atomically; each in-flight child must settle | Outcome depends on a recorded race, not scheduler folklore | Model checking |
| Long-running definition change | The instance remains bound to its exact definition and plan; migration is prohibited until specified | Safe default avoids silent semantic change | Migration ADR |
| Large fan-out | One declared template and a versioned finite collection define child identity; a runtime limit controls scheduling only | Authoring need not draw a million nodes | Scale prototype and visual study |
| Representation projection | Scopes, obligations, labeled transitions, and attribute tables form a finite structured inventory with stable IDs | Text and visuals have a common semantic target | Actual round-trip implementation |

## Single-evaluator criterion assessment

Fitness uses 0–4, evidence uses A–D, and confidence is Low, Medium, or High.
Scores reflect the semantic-model scope, not the quality of deferred notations
or implementations.

| Criterion | Fitness | Evidence | Confidence | Rationale |
| --- | ---: | :---: | :---: | --- |
| S1 — Corpus adequacy | 3 | B | Medium | All forty scenarios have a coherent mapping using a small shared vocabulary |
| S2 — Precision and analyzability | 3 | B | Medium | Atomic transitions, snapshots, invalidity, ordering, and declared nondeterminism are testable, but formal rules are absent |
| S3 — Economy and orthogonality | 3 | B | Medium | Scopes, obligations, revisions, observations, and effects compose across concerns with limited special cases |
| S4 — Composition and scale | 3 | B | Medium | Hierarchy and definition-bounded dynamic fan-out support locality and cardinality; module semantics are deferred |
| S5 — Effects, failure, and protection | 4 | B | Medium | Unknown outcomes, authority, retries, cancellation, compensation, and access constraints are explicit and non-fictional |
| S6 — Identity, evolution, and interoperability | 2 | B | Medium | Stable binding and identities are strong, but IR versioning and migration are deliberately deferred |
| H1 — Cognitive fit by role and task | 2 | A | Low | The concept count appears manageable, but no representative-user evidence exists |
| H2 — Visibility and changeability | 2 | B | Low | Obligations and dependencies support explanation, but actual views and change tooling are absent |
| T1 — Textual effectiveness | 2 | A | Low | The semantic inventory is structurally textual, but no grammar, formatter, or diff evidence exists |
| V1 — Visual effectiveness | 2 | A | Low | The model has graph and hierarchy projections, but no tested visual vocabulary exists |
| V2 — Visual complexity management | 2 | A | Low | Hierarchy and dynamic templates avoid mandatory expansion; navigation and filtering remain untested |
| X1 — Cross-form and engineering integrity | 2 | B | Low | One canonical transition relation is a strong synchronization boundary, but no IR or round trip exists |

No second criterion profile is available to reconcile. The Project Owner's
independent review identified the gate and semantic gaps recorded in this
revision; their resolution does not substitute for representative-user or
implementation evidence.

## Risks, unknowns, and reversibility

| Risk or unknown | Likelihood or uncertainty | Impact | Reversibility | Mitigation or next evidence |
| --- | --- | --- | --- | --- |
| Obligations become too abstract for authors | Medium uncertainty | High | Medium before syntax | Prototype RP-01, RP-03, and RP-08 in both forms and test comprehension |
| Atomic acceptance order hides a needed domain race policy | Medium | High | Medium | Enforce the accepted rule that consequential races require commutativity, invariant protection, or explicit policy; add counterexamples |
| Monotone join predicates remain difficult to validate or visualize | Medium | Medium–high | Medium | Encode only the accepted all, any, threshold, named-outcome, and monotone-composition subset in the first IR and test explanations |
| Protection metadata outruns the Phase 1 type and policy systems | Medium | High | High if enforcement silently widens access | Preserve the accepted minimum protection envelope in the IR and require targets to fail closed |
| History requirements impose excessive runtime cost | Medium uncertainty | Medium | Medium | Separate required semantic fields from optional telemetry and benchmark snapshots/journals |
| Definition-bounded dynamic work is too restrictive for adaptive cases | Low–medium | Medium | Medium | Work RP-08 ad hoc-response examples before freezing the grammar |
| Single-author paper evidence misses contradictions | High | High | High before implementation | Obtain Project Owner and independent contributor review; add model-based tests |

## Synthesis and recommendation

- **Gate result:** G1, G2, G4, and G5 pass at paper semantic-model scope. G3 is
  Conditional until equivalent textual and visual benchmark representations
  and round trips exist. Accepting ADR-0008 establishes a working semantic
  foundation; it does not select a final cross-form design or satisfy the
  Phase 1 exit criteria.
- **Dominant strengths:** One atomic-transition account spans concurrency,
  revisions, human authority, timers, failure, cancellation, and uncertain
  effects. The corpus mapping does not require domain-specific core features.
- **Dominant weaknesses:** Evidence is worked prose from one evaluator. Type,
  composition, IR, notation, persistence, and migration decisions remain open.
- **Decision sensitivity:** The recommendation would change if formalization
  shows obligations cannot express joins locally, if notation prototypes need
  hidden semantics, or if representative cases require runtime-injected control
  behavior rather than declared templates.
- **Recommendation:** Accept ADR-0008 as the working Phase 1 semantic
  foundation after incorporating the Project Owner's review resolutions. Keep
  G3 Conditional and re-evaluate the model before the Phase 1 exit review.
- **What would change this recommendation:** A smaller model that accounts for
  the same forty scenarios with clearer operational rules, or a worked
  counterexample showing irreconcilable ambiguity or inaccessible meaning.

## Framework calibration findings

The framework can review a layer-specific candidate, but future scorecards
should state whether G3 and G4 assess the candidate's own artifacts or its
constraints on downstream notations. T1, V1, V2, and X1 scores for a proposal
limited to semantics mainly measure design compatibility and should not be
compared with scores backed by working editors. No framework change is
proposed until another use shows whether this distinction recurs.

## Decision follow-through

- [x] Link the accepted outcome of ADR-0008.
- [x] Obtain at least one independent review and preserve its required corrections.
- [ ] Convert accepted claims into formal rules and conformance tests.
- [ ] Re-run cross-form and accessibility gates for textual and visual designs.
- [ ] Record later framework calibration without rewriting this evaluation.
