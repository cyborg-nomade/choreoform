<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Evaluation: Binding and type foundations

**Status:** Worked paper evaluation complete; owner approved 2026-09-15 in PR #16<br>
**Framework and corpus revision:** `39cebcc2401f12552a034c75a3cba3e075c8ce50`<br>
**Plan frozen:** 2026-09-15<br>
**Owner:** Proposal author; selection authority: Project Owner

## Frozen plan

Question: which binding, type-identity, scope, import, parameter, and composition
rules should constrain the next executable-contract deliverable? Produce a
paper specification and worked judgments, not a product validator or syntax.
The initial parser, IR 0.1.0, and frozen illustrative contracts remain unchanged.

Priorities: stable meaning under edits and dependency changes; explicit data and
authority boundaries; deterministic resolution and checkable type judgments;
composition across domains; then authoring convenience. No weighted score.
The user's Rust/Python experience gives no candidate a preference advantage.

Compare explicit lexical binding and closed, pinned reusable templates against
ambient/dynamic lookup and implicit structural compatibility. Consider a fully
nominal alternative for all types and a more permissive generic/subtyping model.
All alternatives have paper maturity. Only the recommended combination receives
worked cases; do not invent equivalent test or human-study scores.

Roles/tasks: domain reader identifies what a name refers to; author changes a
label, composes a child, or upgrades an import; implementer derives resolution,
type and visibility judgments; operator distinguishes unavailable data from an
optional value and follows revision provenance. No representative users or
independent evaluator are available. All human/accessibility claims remain
unverified. The same author proposes and evaluates; owner review is separate.

Benchmark pressures: RP-01 reimbursement (Level 1), RP-03 order fulfillment
(Level 3), RP-08 incident response (Level 4), from three domains at the revision
above. Supply worked interface/binding fragments, including rejection cases;
they are not complete executable processes. Retain the forty individual
scenario rows and gaps in ADR-0011's evaluation without upgrading their status.

Perturbations: duplicate/ambiguous names; wrong-kind references; shadowing;
same-shape different-domain types; omitted/extra/wrong-type arguments;
output aliasing; sibling access and ambient capture; import identity/digest
mismatch, missing transitive resources and cycles; label versus identity edit;
long-running dependency upgrade; late/duplicate observation, partial failure,
concurrent cancellation, changed evidence and large fan-out. Operational
perturbations receive boundary analysis, not fabricated execution traces.

Required artifacts: proposed ADR-0012, representation-neutral specification,
positive/negative worked cases with rule references, compatibility map to
existing IR and explicit future-wire obligations, gate review and twelve-axis
profile. Each deferred question names a follow-up deliverable. Prose only; no
new package, dependency, wire fields, registry access, runtime, or new parser.
Use repository content as primary evidence; no external technology comparison
is needed. Check Markdown links, scenario accountability, and consistency with
accepted ADRs; run existing regressions to ensure unchanged code still passes.

Evidence is at most B (worked) for proposed semantic judgments. Link/test success
does not make these judgments reproduced execution evidence. G1–G4 remain
conditional; any recommendation is a working foundation, not final language
selection or permission to execute. Owner approval is required before acceptance,
merge, and starting the executable-contract deliverable.

## Worked results and limitations

The plan was committed as `c45d566` before proposal development. Candidate:
the ADR-0012/specification/case revision in this PR. The same author wrote and
assessed it; no independent rating, representative-user study, executable
prototype, runtime trace or performance measurement was performed.

| Task | Worked result | Evidence / limit |
| --- | --- | --- |
| Resolve and rename | Distinguish stable ID, source binding and display; reject ambiguity, shadowing and wrong-kind lookup | B01/B09/B10/B25/B29/B34; paper rules only |
| Check interface types | Exact anonymous shapes, nominal contract-bound identity, explicit option absence | B02–B05/B12/B16/B26–B28/B32/B33; concrete type values still unspecified |
| Compose safely | Complete argument maps, snapshot capture, non-aliased atomic output publication | B03–B07/B13–B15; no implemented transfer or fault injection |
| Reuse and upgrade | Closed templates, explicit exports, verified transitive pins, acyclic imports/calls | B06/B18/B20–B24/B30/B31/B35; linked wire format absent |
| Protect meaning | Data/type compatibility does not grant effect authority or transfer permission | B08/B15/B18/B36; policy enforcement remains open |
| Explain unavailable data | Absence is a typed value; uninitialized reads cannot invent defaults | B05/B19/B33; evaluator fault semantics need implementation |
| Access and author | Friendly names are distinct from tool-managed IDs; no syntax selected | Accessibility and comprehension untested; no human-efficiency score |

Case identifiers refer to [the worked judgments](../bindings/cases.md). Their
outcomes are deductions from the proposed rules, not observed program results.
The alternatives in ADR-0012 receive architectural trade-off analysis only,
not fabricated parallel scorecards.

## Corpus traceability

The following forty rows are retained verbatim from the baseline ADR-0011
evaluation. They remain **Partial**: this foundation neither implements their
operational contracts nor upgrades the earlier evidence. Additional evidence
here concerns only bindings/interfaces: RP-01 B01–B08, RP-03 B09–B16, RP-08
B17–B24 and the cross-cutting cases B25–B36. Every row additionally needs the
proposed type/value contracts, validator and versioned mappings where applicable.
The three interface fragments are not full benchmark representations.

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

| Gate | Result | Closure evidence and owner |
| --- | --- | --- |
| G1 Corpus accountability | Conditional | Forty explicit inherited gaps; Phase 1 author must supply complete RP-01/RP-03/RP-08 processes, reviewed by owner before final language selection |
| G2 Semantic accountability | Conditional | Rules and counterexamples at B strength; executable-contract/validation author must provide exact values, operations, policies, diagnostics and tests before admission |
| G3 Cross-form parity | Conditional | No new source/visual forms or module wire mapping; frontend/linkage authors must demonstrate lossless equivalent complete benchmarks before Phase 1 exit |
| G4 Protected accessible meaning | Conditional | Explicit no-authority-by-type rule only; policy/frontend authors must demonstrate enforceable protection and representative accessible editing |
| G5 Reproducible comparison | Pass at paper level only | Frozen plan, fixed baseline, numbered rules and 36 worked judgments permit review; no independent or executable confirmation implied |

No final language selection is permissible with these gates conditional.

## Single-author criterion profile

No weighted total; untested dimensions remain unscored. Ratings concern the
recommended combination only, and do not establish that it outperforms alternatives.

| Criterion | Fitness | Evidence | Confidence | Rationale |
| --- | --- | --- | --- | --- |
| S1 Corpus adequacy | 2 | B | Low | Three interface fragments; forty operational gaps remain |
| S2 Precision/analyzability | 2 | B | Medium | Finite lookup/equality/interface rules; concrete contracts and diagnostics absent |
| S3 Economy/orthogonality | 3 | B | Low | Separates identity, type, availability and permission; authoring cost untested |
| S4 Composition/scale | 2 | B | Low | Closed reusable boundaries and acyclic linking; no implementation or scale evidence |
| S5 Effects/failure/protection | 2 | B | Low | No implicit authority or rollback; enforcement and settlement still deferred |
| S6 Identity/evolution | 2 | B | Medium | Pins, stable IDs and nominal revisions explicit; linked provenance encoding open |
| H1 Cognitive fit | Unscored | A | Low | No representative participants |
| H2 Visibility/changeability | 2 | B | Low | Explicit collisions and boundaries; qualification burden unknown |
| T1 Textual effectiveness | Unscored | A | Low | No authoring grammar selected |
| V1 Visual effectiveness | Unscored | A | Low | No visual candidate |
| V2 Visual complexity | Unscored | A | Low | No visual editing study |
| X1 Cross-form/engineering integrity | 2 | B | Low | Compatibility constraints stated; linkage/round trips not implemented |

No second evaluator, calibration session or scoring disagreement exists yet;
absence of objections is not agreement. Owner review may change these judgments.

## Risks and recommendation

Recommend the bounded foundation, not implementation admission or final notation.
Dominant benefits are explicit stable binding and independent type/authority
checks. Dominant costs are conservative exact equality and explicit interfaces.
Implicit shadowing or structural adaptation would be more convenient in some
cases, but could hide meaning changes. Representative authoring studies or a
worked domain case requiring safe generics could justify a later revision.

The absence of a linked wire format is intentional, not resolved by prose:
before implementing imports, review versioned modules, canonical linked identity,
provenance, stable ID allocation and all reference rewrites. Exact scalar and
policy operations remain the next separately authorized deliverable. Unknown
contracts continue to fail closed; no present fixture becomes executable.

Verification for this prose-only change checks repository links, unique worked
case IDs, all forty inherited scenario rows, unchanged accepted artifacts and
existing regressions. Those checks do not execute the proposed rules and do not
raise their evidence above B. No source code, dependencies or frozen contracts
are changed. No new browser run is needed or claimed for unchanged code.

Observed on 2026-09-15: 32 existing Rust unit/integration tests and three
compile-fail doc tests passed; all 16 Python wire tests passed; the text oracle
passed three complete IR/schema/JCS comparisons, three stable export cycles
and seven CLI refusals. Reproduce with the pinned local environment documented
in [the text guide](../text/README.md#running-and-testing):

```sh
cargo test --workspace --locked --offline
.tools/ir-check/bin/python tools/check_ir_fixtures.py
.tools/ir-check/bin/python tools/check_text_prototype.py
git diff --check
```

A local documentation check verified the seven changed/new Markdown files'
license notices, all 48 relative link targets, exactly B01–B36 in case-table
order, and forty scenario rows identical to the baseline evaluation. The ADR
remains Proposed and the Roadmap item unchecked. Link-target existence is not
external-link or rendered-accessibility validation.

## Follow-through

- [x] Obtain Project Owner review of the four ADR questions; record decisions.
- [ ] On approval, update ADR/index/Roadmap and merge this deliverable only.
- [ ] Convert B01–B36 into executable conformance cases with accepted contracts.
- [ ] Review versioned source/module/linkage mappings before implementation.
- [ ] Carry all conditional gates and representative-user studies forward.

Pause before starting executable dialect work.

## PR #16 review follow-up

Reviewed head `2a0a580` across all review threads, submitted review bodies and
top-level comments; pagination was exhausted. One unresolved actionable finding
was present, from CodeRabbit; the submitted review repeats that same finding.

- **Import digest ambiguity: valid, fixed.** M1 previously left the relationship
  between `revision` and the verified digest implicit. It now defines the pin
  itself as the expected SHA-256 digest, requires recomputation using an explicitly
  supported exact-format rule, and distinguishes IR semantic-projection bytes
  from exact contract-snapshot bytes. Unknown future module hash rules fail
  closed until approved; no second unstated digest or new module encoding is
  introduced. B30 now identifies the actual comparison and gives worked
  reformatting, semantic-tampering, snapshot-byte and unsupported-format checks.
- **Rejected findings: none.** Generic bot suggestions to generate tests or use
  its CLI are optional workflow offers, not additional defect reports; no new
  tool or unrelated implementation was added.

Verification: all 16 existing Python wire tests passed, including canonical
projection invariance, semantic changes, frozen snapshot integrity and corrupt
contract refusal. The independent text oracle passed three IR/schema/JCS
comparisons, three stable export cycles and seven CLI refusals. Relative link
targets, B01–B36 uniqueness/order, all forty unchanged inherited scenario rows,
and `git diff --check` passed. This follow-up changes prose only; no new linker
execution, Rust/browser result or higher evidence grade is claimed. The ADR
remains Proposed, with all previously conditional gates open.

## Owner acceptance — 2026-09-15

The Project Owner approved the ADR and PR with all four recommended decisions.
The acceptance metadata, index and Roadmap now record the completed definition
deliverable; PR #16's merge makes acceptance effective. Earlier Proposed/unchecked
statements above describe the historical verification and review stages.
Approval does not raise the evidence grade or close any conditional gate.
Executable contracts, module/linkage formats, validation and representative
authoring studies remain deferred. Pause before the next Roadmap deliverable.
