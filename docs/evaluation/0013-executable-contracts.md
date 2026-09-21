<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Evaluation: Initial executable dialect contracts

**Status:** Bounded evaluation complete; owner approved 2026-09-21 in PR #17<br>
**Plan frozen:** 2026-09-19<br>
**Framework/corpus baseline:** `3e5f79fe23bb5b9ce2490d50fe9c8db376fd7d7a`<br>
**Owner:** Proposal author; decision authority: Project Owner

## Frozen plan

Specify a bounded initial contract for types/values, pure expressions and policy
decisions, under ADR-0008/0009/0012. The deliverable is a reviewable semantic
contract and positive/negative evidence, not the Phase 2 engine or the later
complete structural/link/semantic validator. Do not reinterpret illustrative
payloads or mutate frozen snapshots/fixture revisions. Near-English and visual
designs remain separate. Approval must precede treating new contracts as accepted.

Compare a closed declarative profile against embedded host-language execution
and a general-purpose extensible policy VM at paper maturity. Priorities:
precise rejection/transition semantics, no ambient authority/effects, exact
values and dependencies, explicit uncertainty/settlement, then extensibility.
No language-familiarity preference or weighted total. Only the recommended
profile receives a small disposable Rust evidence probe; no comparative
implementation or usability scores will be invented.

Required artifacts: proposed ADR-0013; concrete type/value and expression rules;
closed policy shapes with context and decision/state semantics; integration map
to every existing policy-bearing IR position; positive/negative worked cases;
bounded pure Rust functions checking selected numeric, dependency, authorization,
time, retry and settlement boundaries; explicit coverage gaps. Keep probe APIs
away from execution admission. No IO, network, provider credentials, plugin VM,
new external dependencies or production integration. Resource refusal is an
error, not a business outcome or permission to discard obligations.

Benchmark pressures: reimbursement RP-01 (Level 1), order RP-03 (Level 3), incident
RP-08 (Level 4), from three domains. Retain all forty scenario-level gaps from
ADR-0012; neither paper examples nor unit functions are full process execution.
Tasks: validate values/assignments; infer expression types/dependencies; inspect
authority and data-flow refusal; trace timers, unknown effects, retry safety,
cancellation and ownership through closure. Perturbations: overflow, missing
data, hidden branch reads, revoked authority, stale/duplicate facts, calendar
revision changes, unknown effect outcomes, concurrent cancellation, changed
evidence, long-running version upgrades and large fan-out.

Roles: author/reader inspect meaning without assuming programming experience;
implementer reproduces judgments; operator distinguishes denial, fault, unknown
outcome and pending settlement. No representative users or independent evaluator
are available. Author both proposes and tests. Evidence grades: at most C for
the exact tested pure functions, B for worked contract rules; never infer user
accessibility or a complete runtime from them. Native tests, format/lint, wasm
compile of the probe, and unchanged fixture oracles are required. Browser
execution of the new probe is not claimed.

G1–G4 stay conditional until complete benchmark, validation, cross-form and
protected-accessibility evidence exists. Exact contract publication/binding and
any incompatible core/semantic version changes must be explicit before use.
Freeze this plan in Git before drafting or assessing results.

## Results and evidence boundary

Plan commit: `c52ab36`, preceding the specification/probe work. Candidate revision
is the implementation in this ADR-0013 PR. Same author designed, implemented and
assessed it; no independent human ratings, representative-user study or provider
verification took place. The architecture alternatives have paper maturity; only
the recommended profile has a selected-function probe.

| Area | Evidence | What is not established |
| --- | --- | --- |
| Canonical integers/exact decimal rescale | C: Rust checks endpoints, malformed spellings, range, scale and inexact reduction; round trips across scale pairs | Complete wire decoder, arbitrary precision or domain currency rules |
| Type/dependency/evaluation | C for literal/read/add/if subset: both-branch checking, hidden reads, missing values, overflow and budgets | Other AST forms, full type universe, nominal resource decoding and protection |
| Human completion | C: synthetic current-authority/principal/revision/duplicate checks | Real identity, assignment persistence and trusted observations |
| Retry | C: synthetic fault/budget/timing/cancellation/request/no-effect/idempotency gates | Provider behavior, effect dispatch, durable attempts or actual reconciliation |
| Timer | C: due/pause/revision/half-open interval boundary and overflow checks | Clock trust, correlated events, pause authorization and atomic race commitment |
| Settlement/closure | C: total unique live ownership predicate, terminal obligation/subscription checks | Receiver acceptance, atomic transfer and runtime closure |
| Other policy transitions | B: closed shapes, contexts, tables and counterexamples in C01–C36 | Complete validator/engine, invariant proofs, real information-flow enforcement |
| Corpus narratives | B: RP-01/RP-03/RP-08 boundary traces | Complete graphs, event histories or executed acceptance scenarios |

The [36 case rows](../dialects/cases.md) label B/C individually. Do not promote
an entire policy family to C because one pure predicate runs. Successful lookup
or hashing is not policy enforcement. Current fixtures remain non-executable;
none was rebound to this proposed suite.

## Corpus traceability

The forty rows below are inherited verbatim from ADR-0012's baseline; all remain
Partial. New cases provide limited value/policy evidence, not closure of the
named gaps. RP-01 maps particularly to C13–C20, RP-03 to C16–C20/C26–C28, and
RP-08 to C25/C29/C31/C33–C36. Every row still needs complete accepted contracts
used in full fixtures, comprehensive validation and runtime/cross-form evidence
where relevant. No unsupported scenario was silently removed from the corpus.

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

## Gates

| Gate | Result | Closure evidence, owner and due condition |
| --- | --- | --- |
| G1 Corpus accountability | Conditional | Phase 1 author must supply complete RP-01/RP-03/RP-08 fixtures and individual scenario evidence before final language/Phase 1 closure |
| G2 Semantic accountability | Conditional | Closed proposed rules and subset checks; contract/validator author must publish accepted artifacts, implement all used judgments and verify hosts before execution admission |
| G3 Cross-form parity | Conditional | No new source/visual grammar; frontend authors must show equivalent full benchmarks and repeated semantic-preserving edits before Phase 1 exit |
| G4 Protected accessible meaning | Conditional | Explicit deny/no-declassification rules, not enforcement evidence; policy/frontend authors must demonstrate protected metadata, trustworthy hosts and representative accessible editing |
| G5 Reproducibility | Pass within bounded scope | Frozen plan, typed probe, exact commands, B/C labeling and unchanged corpus rows; no external provider or human result implied |

These conditions block final language selection and aggregate executable-IR
completion. A working contract decision can be reviewed without claiming those
gates have passed.

## Single-author profile

No weighted score or like-for-like implementation ranking of alternatives.

| Criterion | Fitness | Evidence | Confidence | Rationale |
| --- | --- | --- | --- | --- |
| S1 Corpus adequacy | 2 | B | Low | Three bounded narratives; forty partial scenarios |
| S2 Precision/analyzability | 2 | C for subset / B otherwise | Medium | Closed types/AST/policies, but full validator and host contexts absent |
| S3 Economy/orthogonality | 2 | B | Low | Uniform typed facts and explicit transitions, substantial policy vocabulary |
| S4 Composition/scale | 2 | B | Low | Retains binding/ownership boundaries; no process-scale run |
| S5 Effects/failure/protection | 2 | C for predicates / B otherwise | Medium | Explicit unknown/retry/settlement checks; no genuine enforcement or atomic runtime |
| S6 Identity/evolution | 2 | B | Medium | Specified suite/resource hashes and no rebinding; accepted snapshot not yet published |
| H1 Cognitive fit | Unscored | A | Low | No representative users |
| H2 Visibility/changeability | 2 | B | Low | Policies explicit but verbose; authoring design absent |
| T1 Textual effectiveness | Unscored | A | Low | JSON interchange shapes are not final authoring syntax |
| V1 Visual effectiveness | Unscored | A | Low | No visual candidate |
| V2 Visual complexity | Unscored | A | Low | No visual editing evidence |
| X1 Engineering integrity | 2 | C for probe / B otherwise | Medium | Native tests and wasm compilation; no full dialect decoding/browser parity or cross-form evidence |

No second evaluator or calibration session exists. Absence of disagreement is
not consensus. Owner review of the four ADR questions may change the direction.

## Reproduction and scope of testing

Use the existing pinned Rust 1.98.1 local environment and hash-locked Python
environment described in [the text guide](../text/README.md#running-and-testing).

```sh
cargo fmt --all -- --check
cargo test --workspace --locked --offline
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo clippy -p choreoform-contract-probe --lib --target wasm32-unknown-unknown --locked --offline -- -D warnings
.tools/ir-check/bin/python tools/check_ir_fixtures.py
.tools/ir-check/bin/python tools/check_text_prototype.py
git diff --check
```

Native workspace CI automatically includes the new tests/lint; the workflow
adds a wasm lint for this isolated probe. No shared wire algorithm, parser,
frozen contract, example revision or external dependency changed. Cargo.lock
adds only the local probe package. No new browser execution result is claimed.

Observed locally on 2026-09-19: all 40 Rust unit/integration tests (8 new probe
groups plus 32 existing tests) and three compile-fail doc tests passed. Formatting,
native workspace Clippy and new-probe wasm Clippy passed. All 16 Python wire tests
passed; the text oracle passed three IR/schema/JCS comparisons, three stable
export cycles and seven CLI refusals. No timing or allocation claim is made.

## Risks and recommendation

Recommend review of this bounded working contract, not blanket execution approval.
Exact coefficient limits and no rounding avoid accidental host semantics but may
require extensions for domain calculations. Conservative information flow and
definition-context-specific provider records may block reuse; general weakening
without enforceable policy evidence is not an acceptable workaround. Rich time,
fault-data and race mechanisms remain explicitly unsupported initially. Host
contracts, invariant coverage and lifecycle correctness need substantial further
evidence. The small probe intentionally cannot settle these concerns.

Reconsider the profile if complete benchmarks require otherwise safe behavior
it cannot express, or if independent review finds inconsistent transition rules.
Do not hide such a gap by declaring an unknown policy understood. Revisions to
core IR/semantics must be explicit rather than smuggled into dialect support.

## Follow-through

- [x] Owner review of ADR-0013 and its four recommended decisions.
- [ ] After approval, publish/register the exact accepted snapshot and update
  status/index/Roadmap; retain illustrative artifacts unchanged.
- [ ] Translate every paper rule into validator/conformance tests and full
  benchmarks in separately authorized deliverables.
- [ ] Obtain real host contract/enforcement and representative-user evidence.

Pause before the next Roadmap item.
