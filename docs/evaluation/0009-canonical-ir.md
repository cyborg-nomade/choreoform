<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Design evaluation: Canonical, versioned IR

**Status:** Complete<br>
**Framework revision:** `eeddfb43547ac766334e6700d7fb02ef33bf8dde`<br>
**Plan frozen:** 2026-09-03<br>
**Evaluation owner:** Proposal author

## Frozen plan

This is a single-candidate admissibility review, not a ranking. The candidate
is a representation-neutral typed graph serialized as JSON, with stable local
IDs, exact version binding, and separate presentation metadata. ADR-0009 will
compare architectural alternatives without inventing comparable prototype
scores for unimplemented alternatives.

- **Question:** Can this IR preserve ADR-0008's distinctions and provide a
  testable serialization and evolution boundary for the remaining Phase 1 work?
- **Authority:** Project Owner; no acceptance or merge is implied by this work.
- **In scope:** Definition records, graph references, semantic identity,
  canonical bytes, version admission, opaque semantic dialect boundaries,
  presentation separation, and runtime linkage requirements.
- **Out of scope:** A user-facing grammar, visual vocabulary, full type and
  expression semantics, engine/checkpoint format, migration of live instances,
  and a production validator or implementation-language choice.
- **Corpus:** All forty adopted scenarios at `eeddfb4`.
- **Benchmarks:** RP-01 (administration, Level 1), RP-03 (commerce/logistics,
  Level 3), RP-08 (security operations, Level 4).
- **Roles:** Domain reader, process author, operator, and tool implementer.
  Structured text must expose every semantic field; no spatial-only meaning.
- **Common tasks:** Encode benchmark excerpts; inspect references and revision
  dependencies; reorder maps; change a guard or protection field; edit only
  annotations; reject duplicate keys, unknown versions, unknown core fields,
  and dangling references; retain all child identities when input scope grows.
- **Perturbations:** Late/duplicate observation, partial failure, concurrent
  cancellation, decision-relevant revision, long-running definition change,
  large fan-out, unknown semantic dialect, and corrupt revision digest.
- **Evidence:** ADR, wire specification, structural schema, labeled benchmark
  excerpts, repeatable serialization checks, all-scenario gap table, and risks.
- **Limits:** One proposal author and one machine; no independent scoring,
  user study, executable process semantics, or text/visual round-trip evidence.
- **Selection rule:** No weights or total. Gate failures or conditional results
  remain explicit; only a working design proposal may advance while they remain.

The plan was recorded before implementing the schema, fixtures, or scoring.
Candidate artifacts will be identified by the commits in the proposal PR.

## Candidate and evaluators

| Candidate | Revision | Maturity | Evaluator and limits |
| --- | --- | --- | --- |
| ID-addressed JSON graph | ADR-0009 and artifacts in this PR | Structural schema plus tested wire excerpts | Proposal author; not independent; no representative-user study or second score profile |

The schema, [specification](../ir/definition-v0.1.md), and
[example notes](../ir/examples.md) form one candidate package. All scores below
are scoped to that package, not to an imagined executable engine. No material
disagreement has been independently collected; absence of disagreement is not
consensus.

## Corpus traceability

Each scenario is **Partial**: the table names its structural representation and
the remaining material evidence/semantic gap. Merely naming a policy slot does
not establish scenario support. The shared type/expression/policy dialect gap
applies to every row as well as each specific gap below.

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

| Gate | Result | Evidence | Closure evidence, owner, and due condition |
| --- | --- | --- | --- |
| G1 — Corpus accountability | Conditional | All forty scenarios mapped; three labeled excerpts preserve selected pressures, not complete benchmark invariants | Phase 1 author must provide complete RP-01, RP-03, and RP-08 representations before final IR/conformance approval |
| G2 — Semantic accountability | Conditional | Closed core shapes, explicit references, joins, versions and identity; policy/type/expression meaning remains illustrative | Language/type-policy owners must supply accepted dialect contracts and operational examples before planning/execution is admitted |
| G3 — Cross-form parity | Conditional | One semantic projection is specified; no textual/visual frontend or actual round trip exists | Text/visual owners must produce equivalent full benchmarks and repeatable round trips before Phase 1 exit |
| G4 — Protected and accessible meaning | Conditional | Sensitivity, purpose, access and policy references are semantic; no spatial-only core field | Policy and frontend owners must demonstrate complete policy interpretation and protected accessible editing before executable/Studio conformance |
| G5 — Reproducible comparison | Pass | Exact baseline, schema, fixtures, pinned libraries, checksum rules, executable wire checks and disclosed gaps | Independent reproduction remains desirable; does not close other gates |

This review does not alter ADR-0007's gates. A structural working proposal may
advance for owner discussion, but these results do not justify selecting a
final executable or cross-form design. This distinction deliberately carries
forward the correction made during PR #10's review.

## Observed tasks and perturbations

| Task | Observed result | Interpretation and limit |
| --- | --- | --- |
| Encode RP-01 excerpt | Review, confirmed/unknown payment and reconciliation wait have distinct IDs and flows | Selected distinctions survive encoding; omitted finance/correction paths remain a material gap |
| Encode RP-03 excerpt | Two child templates and a reciprocal join are explicit | Structural fork/merge is inspectable; shipping and compensation behavior untested |
| Encode RP-08 excerpt | Per-item child data binding, immutable key input and seal are explicit | Scope-growth contract is representable; no dynamic instance is executed |
| Change map insertion order / JSON indentation | All three fixtures retain canonical bytes and digest | Serialization result, not graph-isomorphism proof |
| Add unknown editor annotation | Digest unchanged | Annotation separation tested; editor preservation/accessibility not implemented |
| Change sensitivity or policy reference | Digest changes | Protection is inside the semantic boundary; no enforcement proof |
| Change expression payload or definition ID | Digest changes | Semantic dependencies and identity are revision-bound |
| Reverse dialect argument array | Digest changes | No accidental sorting of ordered semantic content |
| Malformed JSON and negative graph mutations | Rejected by the fixture harness | Selected transport/shape/link rules only, not complete validation |
| Editorial source edits / changed contract artifact | Frozen local bindings retain the original revisions; altered snapshot bytes and unknown identity/digest pairs are rejected | Integrity and local registry evidence, not authenticity or executable dialect support |
| Unknown declared dialect | May be retained as data, but has no executable implementation here | Inert preservation is distinct from semantic admission |
| Cancellation, retries, late success, large fanout | Policy and identity slots inspected only | No operational trace, timing, performance or recovery result claimed |

Run `uv run tools/check_ir_fixtures.py` as described in the example notes.
Verification uses `jsonschema==4.25.1` and `rfc8785==0.1.4`; it needs no provider
accounts, network reference resolution, or chosen product runtime.

Observed on 2026-09-03: all 12 test groups pass using Python 3.12.13 and uv
0.11.14 on the local macOS host. Groups include three fixture checks, selected
positive interface/computation checks, and malformed-input/graph mutations.
These counts describe this harness, not language conformance coverage.

## Single-evaluator criterion profile

Fitness uses the accepted 0–4 scale, evidence A–D, and confidence Low/Medium/High.
The C grades below cover only reproduced wire properties; they do not upgrade
the entire candidate to executable semantics.

| Criterion | Fitness | Evidence | Confidence | Rationale |
| --- | ---: | :---: | :---: | --- |
| S1 — Corpus adequacy | 2 | B | Low | All pressures have structural slots; full models and dialects missing |
| S2 — Precision and analyzability | 2 | C | Medium | Wire/references are testable; semantic-contract validation is not |
| S3 — Economy and orthogonality | 3 | B | Medium | Scope, data, policy, node and flow maps separate concerns without editor/engine coupling |
| S4 — Composition and scale | 2 | B | Low | Templates and explicit item binding stay compact; imports/general composition and runtime scale untested |
| S5 — Effects, failure, and protection | 2 | C | Low | Hash retains protection and policy references; their behavior/enforcement is still a gap |
| S6 — Identity, evolution, and interoperability | 3 | C | Medium | Stable IDs, explicit hash projection and exact admission; no live migration is permitted |
| H1 — Cognitive fit by role and task | 1 | A | Low | JSON is a tool interchange format, not evidence of usable author notation |
| H2 — Visibility and changeability | 2 | B | Low | ID links expose dependencies but impose navigation overhead without tooling |
| T1 — Textual effectiveness | 2 | A | Low | Durable JSON review possible; no language syntax, formatter or source-ID ergonomics |
| V1 — Visual effectiveness | 1 | A | Low | Visual vocabulary and usability evidence absent |
| V2 — Visual complexity management | 2 | A | Low | Templates avoid mandatory expansion; no tested navigation/filtering |
| X1 — Cross-form and engineering integrity | 2 | C | Low | Serialization and chosen invalid-input checks reproduce; frontend round trips and independent engine behavior absent |

## Risks and sensitivity

| Risk | Impact | Reversibility | Mitigation / evidence |
| --- | --- | --- | --- |
| Dialect slots become a semantic escape hatch | High | Moderate before consumers exist | Require complete immutable contracts; unknown is inert; never equate shape validation with executable validity |
| Seal-before-join blocks a needed adaptive process | High | High while Proposed | Work incremental RP-06-B/RP-08-B traces with owner before acceptance |
| Stable IDs burden textual authors | Medium | Moderate | Prototype explicit versus sidecar identity preservation in grammar work; no syntax chosen here |
| JCS hash mistaken for semantic equivalence or authorization | High | Moderate | Precisely name projection; retain policy fields; separate signature and full-file checks |
| Exact-version admission creates unnecessary upgrade churn | Medium | High before release | Review compatibility matrix using actual consumers, not assumptions |
| Graph/schema/harness drift | High | High before release | Keep machine schema and prose in one PR; add missing validator phases later; retain negative tests |
| Single-author review misses errors | High | High while Proposed | Project Owner review and independent reproduction; do not raise confidence by counting fields |

The recommendation is sensitive to whether this structural boundary can be
approved before executable dialects. If the owner requires a complete runnable
IR now, the correct outcome is to continue specification alongside the type
system, not to relabel these excerpts or Conditional gates as complete.

## Recommendation and follow-through

Advance ADR-0009 for review as a **working structural IR proposal**, not final
executable-language selection. Resolve its four review questions before
acceptance; retain conditional gates and explicit follow-up owners.

- [ ] Record Project Owner review and any disagreement.
- [ ] Resolve structural-versus-executable scope, version policy, hashing and seal.
- [ ] Record acceptance only after approval; Roadmap remains unchecked.
- [ ] Provide accepted dialects and complete benchmark models.
- [ ] Implement semantic validation, frontend round trips and independent tests.
