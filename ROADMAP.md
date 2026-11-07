<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Roadmap

This roadmap is organized around evidence and exit criteria rather than dates.
Each phase should produce a usable vertical slice and the decisions needed by
the next phase. Phases may overlap, but their exit criteria should not be
skipped.

**Execution workflow (owner-authorized 2026-11-05):** follow
[ADR-0018](docs/decisions/0018-phase-review-delegation.md). Codex implements,
reviews and merges verified items within the current phase without item-level
approval pauses. At the phase boundary it reports achievements, evidence and
remaining limits, then waits for owner approval before opening the next phase
chat. Keep all exit/evidence criteria intact. The
[Phase 1 progress record](docs/evaluation/phase-1-progress.md) tracks continuation.

## Phase 0 — Groundwork

**Goal:** establish the project’s identity, legal foundation, scope, and way of
making decisions.

### Deliverables

- [x] Initialize the source repository.
- [x] Publish the project Manifest and initial Roadmap.
- [x] Select **Choreoform** as the permanent project and language name in
  [ADR-0001](docs/decisions/0001-project-and-language-name.md).
- [x] Adopt the licensing policy for software, specifications, generated
  artifacts, extensions, and bundles in
  [ADR-0002](docs/decisions/0002-licensing-policy.md).
- [x] Adopt contribution, governance, security, and code-of-conduct policies in
  [ADR-0003](docs/decisions/0003-community-governance.md).
- [x] Adopt the lightweight architecture-decision-record process in
  [ADR-0004](docs/decisions/0004-lightweight-adr-process.md).
- [x] Adopt the working glossary for process, activity, event, state,
  capability, instance, definition, plan, and bundle in
  [ADR-0005](docs/decisions/0005-pre-semantic-working-glossary.md).
- [x] Adopt a corpus of ten representative processes spanning multiple domains
  and complexity levels in
  [ADR-0006](docs/decisions/0006-representative-process-corpus.md).
- [x] Adopt criteria and a repeatable procedure for evaluating competing
  semantic and notation designs in
  [ADR-0007](docs/decisions/0007-design-evaluation-framework.md).

### Exit criteria

- The permanent name and licenses are decided and documented.
- The [legal-review trigger](docs/legal-review/README.md) for the
  open/commercial boundary is documented, and proprietary bundle development
  remains gated on qualified review.
- Representative use cases expose the minimum concepts required of the core.
- Contributors can understand how decisions are proposed, recorded, and
  changed.

## Phase 1 — Semantic core and language prototype

**Goal:** prove that a small, domain-neutral process model can be expressed,
validated, and round-tripped.

### Deliverables

- [x] Define the semantic model for control flow, data, actors, time, errors,
  cancellation, and side effects in
  [ADR-0008](docs/decisions/0008-core-process-semantics.md).
- [x] Specify the structural foundation of a canonical, versioned intermediate
  representation (IR) in
  [ADR-0009](docs/decisions/0009-canonical-versioned-ir.md).
- [x] Select Rust for the initial shared semantic implementation in
  [ADR-0010](docs/decisions/0010-initial-implementation-language.md), based on
  parser, language-server, graph-modeling, runtime, and deployment requirements.
- [x] Complete ADR-0010's bounded native/browser confirmation in a separate PR
  before substantial parser work: strict IR admission and canonical revisions,
  typed variants/references, native/Wasm parity, explicit host boundaries, and
  repeatable builds with a pinned stable toolchain and locked dependencies.
  [Probe and evidence](docs/evaluation/0010-rust-portability.md) approved by the
  Project Owner on 2026-09-04, effective upon merge of
  [PR #13](https://github.com/cyborg-nomade/choreoform/pull/13).
  Complete structural/semantic validation and ADR-0009's conditional gates remain open.
- [x] Design an initial textual grammar and parser. Accepted as a bounded prototype in
  [ADR-0011](docs/decisions/0011-initial-textual-grammar.md), with a bounded
  [syntax and parser prototype](docs/text/README.md); Project Owner approved
  2026-09-08, effective upon merge of
  [PR #14](https://github.com/cyborg-nomade/choreoform/pull/14).
- [x] Define names, types, scopes, imports, parameters, and composition.
  Accepted in [ADR-0012](docs/decisions/0012-binding-type-foundations.md), with
  [rules and worked cases](docs/bindings/README.md); Project Owner approved
  2026-09-15, effective upon merge of
  [PR #16](https://github.com/cyborg-nomade/choreoform/pull/16).
  This establishes the binding and type foundations needed by the executable
  contracts below, not an implemented module system or executable type dialect.
- [x] Specify and obtain approval for executable type, expression, and policy
  dialect contracts for the existing dialect slots: exact values and
  operations, purity/dependencies, authority/protection, time, effects,
  cancellation, reconciliation, and settlement. Include worked positive and
  negative semantic cases; a JSON slot or prose label is not an implementation.
  Accepted in [ADR-0013](docs/decisions/0013-executable-dialect-contracts.md), with
  [contract rules and bounded evidence](docs/dialects/README.md); Project Owner
  approved 2026-09-21, effective upon merge of
  [PR #17](https://github.com/cyborg-nomade/choreoform/pull/17).
  Illustrative payloads remain historical and non-executable; they are not
  migrated or reinterpreted. The subset probe is not the complete validator or
  reference engine.
- [ ] Design and evaluate the **near-plain-English authoring language** as a
  dedicated ADR/PR. Aim for the broadest practical accessibility with a precise,
  deterministic grammar (controlled English), not unrestricted natural-language
  interpretation. Ordinary authoring must not require JSON records or manual
  wire-format bookkeeping. Preserve explicit meaning, stable identity and policy
  visibility through lowering; reject ambiguity rather than guessing intent.
  Compare worked alternatives on complete RP-01, RP-03 and RP-08 processes and
  representative non-programmer comprehension, authoring, correction and review
  tasks. Document accessibility evidence and unresolved gaps. PR #14's low-level
  declaration prototype does **not** satisfy this item or settle final syntax.
  In progress: [ADR-0014](docs/decisions/0014-near-english-authoring.md) accepts
  Candidate A as a provisional working direction, with the
  [authoring comparison](docs/authoring/README.md); Project Owner approved
  2026-09-22, effective upon merge of [PR #18](https://github.com/cyborg-nomade/choreoform/pull/18).
  This is not completion. Full lowerable benchmarks, grammar/binding-package evidence
  and representative-user studies remain required; this item stays unchecked.
  The grammar/package step is accepted provisionally in
  [ADR-0015](docs/decisions/0015-authoring-grammar-bindings.md): complete Candidate A
  grammar and portable binding-package specifications with executable checks;
  Project Owner approved 2026-10-04, effective upon merge of
  [PR #19](https://github.com/cyborg-nomade/choreoform/pull/19).
  The [bounded Rust frontend slice](docs/evaluation/0015-rust-authoring-frontend.md)
  implements exact source/ledger consistency and supported candidate IR lowering;
  Project Owner approved 2026-11-02 with recommendations, effective upon merge of
  [PR #20](https://github.com/cyborg-nomade/choreoform/pull/20).
  This is **not** full source/semantic admission or general IR export: external
  clock/calendar contract/host support, complete validation/confusable
  diagnostics, identity-preserving export/editing, full source/IR benchmarks and
  matched user-study materials/results remain explicit subsequent work within
  this item. It stays unchecked; coordinate semantic admission with the open
  executable-IR/validator gates before authorizing the next slice.
  [ADR-0016](docs/decisions/0016-static-contract-checks.md) accepts supported artifact
  readers and explicit static checks with source integration; Project Owner
  approved 2026-11-02, effective upon merge of
  [PR #21](https://github.com/cyborg-nomade/choreoform/pull/21).
  [Evidence and residual obligations](docs/evaluation/0016-static-contract-checks.md)
  preserve the distinction from full-definition admission; clocks/timers remain
  unsupported. Neither this authoring item nor the complete validator is closed.
  The bounded [ordinary control/initialization stage](docs/decisions/0017-control-availability-checks.md)
  is accepted: Project Owner approved PR #22 on 2026-11-05, effective upon merge.
  It checks ordinary flow rules, conservative definite reads/
  outputs and explicit root-input/coverage reporting. Its
  [evidence and follow-up priority](docs/evaluation/0017-control-availability-checks.md)
  retain parallel/exceptional availability, control-sensitive information flow/
  access, race/invalidation/reconciliation, supported clocks/host and complete
  benchmark/conformance evidence as open work. It does not close this item,
  complete validation or the executable-IR/Phase 1 gates.
  The [ordinary protection analysis](docs/decisions/0019-protected-ordinary-transfers.md)
  adds control-origin and transfer constraints with explicit access obligations;
  actual access/host enforcement, exceptional/observable channels and race/
  invalidation proofs remain open. Its
  [initial access follow-up](docs/evaluation/0019-initial-access-evidence.md)
  checks ordinary input/initializer policy dependencies conservatively; the
  [completion follow-up](docs/evaluation/0019-completion-access-evidence.md)
  declares output metadata dependencies at finish nodes. These do not establish
  actual runtime access or close the full validator. See its
  [evidence](docs/evaluation/0019-protected-ordinary-transfers.md).
  [Lifecycle use-site validation](docs/decisions/0020-lifecycle-policy-links.md)
  checks actual handler/follow-up child contexts and conservative unique
  reconciliation wait definition sites. It does not prove live ownership,
  receiver acceptance, runtime correlation or settlement. These and the remaining
  race/invalidation, clock, benchmark and phase gates stay open.
  [Creation footprints](docs/evaluation/0019-creation-access-evidence.md) expose
  ancestor initializer/access dependencies on all four creator kinds before
  conflict analysis, without treating fresh locals as shared caller storage.
  [Ordinary invalidation declarations](docs/decisions/0021-ordinary-invalidation.md)
  check conservative pre-result dependency links for visited decisions/work/waits,
  with explicit uncovered sites and unsupported context refusal. Runtime
  withdrawal, non-data/exceptional invalidation and race safety remain open.
  [Ordinary parallel independence](docs/decisions/0022-ordinary-concurrency.md)
  checks fresh-local/shared-ancestor footprints and conservative external/lifetime
  boundaries. Full domain/invariant proofs, early/reconcile concurrency, actual
  host enforcement and aggregate gates remain open.
  [ADR-0023](docs/decisions/0023-clock-resource-contract.md) specifies a pinned
  instance-microsecond clock profile and metadata reader, with explicit trusted
  source/origin/ordering/host requirements. Non-null timers stay unsupported until
  timer typing, clock-source flow, wait outcomes, calendar-domain binding, retry
  clock selection and host-obligation integration are completed. This proposal
  does not supply a real clock adapter or close the time/validator gate.
  [ADR-0024](docs/decisions/0024-timer-contract-checks.md) integrates supported
  non-calendar timer types/dependencies, exact wait outcomes and pre-write clock
  origins, with an explicit non-executable timing report of host requirements.
  Calendar-origin and retry-clock bindings, due/domain provenance and actual
  source/clock/authority/atomic enforcement remain open; no time gate is closed.
  [ADR-0025](docs/decisions/0025-instance-calendars.md) defines a separately pinned
  relative-calendar construction profile, indexed coordinate membership and exact
  source/IR/timing-use references. Actual host construction approval, origin
  activation, reschedule and clock enforcement remain required; unbound retries
  and the remaining aggregate gates stay open.
  [ADR-0026](docs/decisions/0026-explicit-retry-clock.md) adds a separately pinned
  core revision with explicit retry clock/basis fields and protected timing-use
  metadata. [ADR-0027](docs/decisions/0027-explicit-retry-authoring.md) supplies
  source 0.3.0 / companion 0.2.0 and the explicit English retry sentence, with
  grammar/schema/hash/IR/report cycles. Old source retains its old core and
  unbound-retry refusal. This is not complete tool-managed authoring or user evidence.
  Actual fault/domain correlation, unchanged revisions, effect evidence and
  atomic authorized dispatch remain host requirements, not static approval.
  The [settlement dependency correction](docs/evaluation/0019-settlement-authority-dependencies.md)
  closes receiver management-authority reads into transfer policy/node footprints
  and supplies precise read-mismatch paths. Actual receiver acceptance/live
  ownership, exceptional routing and the complete validator still require evidence.
  [ADR-0028](docs/decisions/0028-conservative-fault-routing.md) adds an indexed
  conservative handler-selection graph with lexical/active exclusion rules and
  explicit cycle-unproved refusal. Actual exceptional data, concurrent occurrence
  lifetime and runtime dispatch/ownership remain separate open judgments.
  [ADR-0029](docs/decisions/0029-conditional-handler-availability.md) adds handler
  body availability under explicit ancestor-input premises and fresh locals,
  without crediting an ordinary-path result as a fault snapshot. Exceptional
  fault-control protection/invalidation, actual activation and lifetime remain
  open. A private initializer scope index removes unrelated-storage scans.
  [ADR-0030](docs/decisions/0030-reconciliation-declaration-links.md) checks an
  actual activity scope's surviving follow-up and reachable unique wait declaration.
  Direct-IR positive cases do not supply actual accepted ownership or entry data.
  [ADR-0031](docs/decisions/0031-qualified-wait-endpoints.md) and source 0.4.0
  supply qualified wait-outcome endpoint binding inside the designated direct
  reconciliation child, preserving old lexical/data privacy without dummy root
  nodes. This closes that source binding dependency only; actual follow-up entry,
  protection, handoff, correlation and multiple-obligation lifetime stay open.
  [ADR-0034](docs/decisions/0034-benchmark-authoring-profile.md) adds bounded
  source 0.5.0 whole-program recognition and actual-use follow-up endpoints;
  it does not close live ownership/lifetime or full benchmark evidence.
  [ADR-0035](docs/decisions/0035-provider-metadata-read-closure.md) specifies
  versioned finite provider-metadata read closure with strong/legacy cycle refusal.
  [ADR-0036](docs/decisions/0036-metadata-core-authoring.md) adds explicit source
  0.6 support for that core; full source adoption/benchmark/proof gates remain open.
  [ADR-0037](docs/decisions/0037-closed-ordinary-provenance.md) proves a bounded
  historical-origin subset for ordinary invalidation while retaining mutable
  ancestors, protection and independent lifetime/host obligations.
  [ADR-0038](docs/decisions/0038-invalidation-requirements.md) derives advisory
  required/missing links without bypassing strict coverage. [ADR-0039](docs/decisions/0039-isolated-surviving-waits.md)
  checks a synchronous isolated wait-only follow-up subset under explicit live
  P6 premises, preserving parallel terminal requirements. The full RP-01 draft now
  passes ten partial static stages, but its [original-scenario audit](docs/evaluation/0040-rp01-scenario-audit.md)
  retains guard conformance, fault-source classification, late-observation capture/
  continuation and actual ownership/race/host evidence as open work. This does not
  close full benchmark, near-English authoring, complete validation or phase gates.
  [ADR-0040](docs/decisions/0040-finite-guard-snapshot-conformance.md) adds 44 finite
  source-derived guard snapshots and corrects missing payment policy-revision
  comparisons. These test-only predicate results are neither authenticated facts
  nor process transitions and do not complete any A–D scenario.
  [ADR-0041](docs/decisions/0041-provider-fault-source-metadata.md) and
  [ADR-0042](docs/decisions/0042-explicit-retry-fault-source-binding.md) pin declared
  provider fault evidence and exact retry-use identity. [ADR-0043](docs/decisions/0043-retry-fault-source-authoring.md)
  adds explicit source 0.7 authoring and bounded disclosure. The
  [RP-01 adoption checkpoint](docs/evaluation/0043-rp01-adoption.md) preserves all
  prior guards/IDs and keeps actual fault truth, late-result capture, owned
  continuation and all scenario/phase requirements open.
  [ADR-0044](docs/decisions/0044-provider-terminal-observation-metadata.md) pins
  terminal source/type/protection metadata. [ADR-0045](docs/decisions/0045-explicit-timerless-wait-capture.md)
  adds raw IR 0.2/core 0.5 timerless capture with exact source/origin/result and
  conditional initialization. Explicit readable source support and bounded native
  disclosure are supplied by [ADR-0046](docs/decisions/0046-wait-capture-authoring.md)
  for source 0.8/companion 0.7. General plain-English/tool-managed editing, owned
  recovery storage, fault data/continuation,
  cutoff/settlement and actual host/scenario evidence remain open.
  [ADR-0047](docs/decisions/0047-owned-local-followups.md) separately checks an
  isolated fresh-local surviving subset, including initialization, retained source /
  control protection provenance and strict conditional invalidation. It does not
  allocate host storage or provide actual P6 receipts. General retention, fault
  data, permitted continuation/cutoff/settlement and full scenario evidence stay open.
  The [fault-entry audit](docs/evaluation/0048-fault-entry-contract-audit.md)
  distinguishes abstract routes/retry metadata from authenticated typed handler
  data. General emission binding, explicit versioned entry and permitted owned
  continuation/cutoff/settlement remain dependencies before benchmark adoption.
  [ADR-0048](docs/decisions/0048-general-provider-fault-source-binding.md) binds
  declared provider emissions independently of retry count/eligibility in raw core
  0.6. Readable source/native selection and typed handler data/entry remain explicit
  next dependencies, with no actual emitted-fault or host receipt implied.
  [ADR-0049](docs/decisions/0049-general-fault-source-authoring.md) explicitly
  selects that core with source 0.9/companion 0.8, one readable source sentence and
  bounded shared native catalogues. Typed history/source-control protection and
  handler-local entry remain next; no authenticated fault or scenario exit follows.
  [ADR-0050](docs/decisions/0050-provider-fault-history-metadata.md) specifies
  exact nominal history/source metadata and limited untrusted claim checks. It
  supplies no authenticated fault or handler entry; complete source/control
  provenance, actual incoming contexts and the reviewed entry wire remain open.
  [ADR-0051](docs/decisions/0051-declared-fault-origin-contexts.md) retains
  declared activity/source relations and complete handler stacks in a separate
  bounded SDK report. Unsupported/uncovered origins stay explicit; no actual
  event, fault-control protection snapshot or handler initializer is supplied.
  [ADR-0032](docs/decisions/0032-conditional-reconciliation-availability.md) adds
  conditional follow-up body availability with zero inherited values and fresh
  locals. It does not prove ancestor retention, live activation/storage/current
  permission, exceptional protection/invalidation or accepted settlement.
- [ ] Design an initial visual notation and serialization of layout metadata,
  using the same contracts and full benchmarks as the authoring language.
- [ ] Implement complete structural/link/semantic validation of the accepted
  contracts and stable diagnostics; include negative cases and fail-closed
  handling of unknown dialects and unenforceable policies. Parsing/hashing and
  the existing partial probe are not substitutes.
  [ADR-0033](docs/decisions/0033-semantic-source-locations.md) maps known semantic
  declaration paths to original UTF-8 source spans without guessing global/resource
  locations. Exact field/token precision, full catalogue and complete validator
  coverage remain open; declaration locations do not close this item.
- [ ] Build text → IR → text and visual → IR → visual round trips.
- [ ] Publish small executable examples and negative examples.
- [x] Start a language conformance suite. The
  [value-expression corpus](docs/conformance/README.md) begins with 69 exact
  value/static-refusal/evaluation-fault vectors against production judgments.
  This marks the start only; complete policy/process/benchmark and cross-form
  conformance remains part of the open aggregate gate below. See the
  [consolidated validator gap audit](docs/evaluation/0028-validator-gap-audit.md).
  The [current benchmark readiness audit](docs/evaluation/0034-benchmark-readiness.md)
  retains complete RP-01/03/08 responsibilities, explicit no-output/interface
  boundaries and [all forty scenario gaps](docs/evaluation/0034-corpus-accountability.md).
  All forty remain Partial; the next package target is full RP-01.

### Executable IR completion gate (still open)

- [ ] Complete the executable IR contract with accepted type, expression, and
  policy dialects, semantic validation, and complete benchmark evidence;
  structural approval does not close the conditional gates or Phase 1 exit.

This is an aggregate completion gate, not a finished or silently skipped task.
It was separated from structural approval in PR #11. Its prerequisites are the
binding/type foundations, accepted executable dialects, complete validator,
full benchmark examples, and conformance/round-trip evidence listed above.
ADR-0009's G1–G4 remain conditional, including protected accessible editing and
text/visual parity. The Phase 1 author supplies the evidence and the Project
Owner approves closure. The Phase 2 reference engine remains a separate delivery;
an executable contract here requires precise semantics and checkable evidence,
not a claim that a production engine already exists.

**Sequence approved in PR #14:** after the bounded parser,
address binding/type foundations and executable contracts before advancing to
the near-English and visual authoring designs. Each deliverable still requires
its own recorded review; permission to start within the phase is now delegated
under ADR-0018. Authoring studies may feed back into the
contracts; no final language selection or compatibility commitment is made
while the gates remain conditional.

### Exit criteria

- At least three representative processes can be expressed in both forms.
- Round-tripping preserves semantics and produces stable output.
- Invalid constructs fail with actionable diagnostics.
- Every accepted core construct has written operational semantics and tests.
- Near-plain-English authoring has been evaluated with representative users;
  raw JSON/declaration syntax alone does not meet the authoring-language goal.

## Phase 2 — Reference execution engine

**Goal:** execute the semantic core locally with observable, repeatable
behavior.

### Deliverables

- [ ] Implement parse, validate, normalize, plan, and execute stages behind
  explicit interfaces.
- [ ] Build a reference interpreter before optimizing code generation.
- [ ] Define the execution-state and event-history models.
- [ ] Support inputs, outputs, branching, parallel work, timers, retries,
  cancellation, and compensation at the level promised by the language.
- [ ] Add a capability/adapter interface for external side effects.
- [ ] Add structured logs, traces, breakpoints, and deterministic replay where
  feasible.
- [ ] Define sandboxing, permissions, secrets, and resource-limit policies.
- [ ] Prototype one generated-code or deployable-artifact backend.
- [ ] Test crash recovery and version compatibility.

### Exit criteria

- The conformance examples execute consistently in the reference engine.
- A process can be inspected, paused, resumed, cancelled, and diagnosed.
- External effects are explicit and testable with substitute adapters.
- Execution history explains each state transition.

## Phase 3 — Studio vertical slice

**Goal:** make text and visuals effective, synchronized editing experiences.

### Deliverables

- [ ] Choose the studio delivery architecture (web, desktop, or shared core
  with multiple shells) through an architecture decision record.
- [ ] Implement a textual editor with syntax highlighting, completion,
  diagnostics, navigation, formatting, and refactoring foundations.
- [ ] Implement a visual canvas for creating, connecting, configuring, and
  grouping process elements.
- [ ] Synchronize both editors through the canonical model without semantic
  drift.
- [ ] Add simulation, step execution, breakpoints, state inspection, and an
  execution timeline.
- [ ] Add diff and review experiences suitable for source control.
- [ ] Test keyboard access, screen-reader semantics, and large-process
  navigation.

### Exit criteria

- A user can build and debug the same representative process using either
  editor and switch between them at any time.
- Edits remain stable under source control and collaboration.
- Usability tests validate both a first-time workflow and an expert workflow.

## Phase 4 — Extension and packaging ecosystem

**Goal:** make integrations and reusable process packages safe and portable.

### Deliverables

- [ ] Publish an SDK for capabilities, adapters, domain types, and tooling
  extensions.
- [ ] Specify the bundle/package format, manifest, dependencies, configuration,
  and lifecycle hooks.
- [ ] Define semantic versioning and compatibility negotiation for language,
  engine, studio, extensions, and bundles.
- [ ] Add package signing, provenance, permission declarations, and offline
  verification.
- [ ] Build a local package manager and registry protocol.
- [ ] Publish free reference extensions and example bundles.
- [ ] Add certification and compatibility-test tooling.

### Exit criteria

- A third party can create, test, package, and distribute an extension using
  only public documentation and tools.
- Packages declare their capabilities and fail safely when requirements are not
  met.
- Compatibility is verified automatically before installation or execution.

## Phase 5 — Commercial bundle pilots

**Goal:** validate that curated proprietary bundles create customer value while
remaining clean consumers of the open platform.

### Deliverables

- [ ] Complete qualified legal review of the implemented open/commercial
  boundary before creating proprietary bundle material or accepting a paid
  bundle pilot.
- [ ] Select two sharply defined pilot segments rather than attempting every
  business size and type at once.
- [ ] Research each segment’s jobs, constraints, regulations, integrations, and
  measurable outcomes.
- [ ] Create a bundle lifecycle: configure, validate, simulate, deploy, update,
  migrate, support, and remove.
- [ ] Build anonymized test fixtures and acceptance scenarios.
- [ ] Define pricing, licensing, support, update, and end-of-life policies.
- [ ] Run pilots and measure time-to-value, completion quality, intervention
  rate, and upgrade safety.
- [ ] Keep proprietary source, data, and release infrastructure in separate
  access-controlled repositories.

### Exit criteria

- At least one bundle operates on an unmodified public release of the engine
  and studio.
- Pilot users achieve a documented improvement over their prior process.
- Bundle installation and removal do not compromise platform security or data
  portability.

## Phase 6 — Stable platform releases

**Goal:** turn validated components into a dependable, supportable platform.

### Deliverables

- [ ] Publish versioned language, IR, engine, studio, and SDK releases.
- [ ] Establish deprecation, migration, and long-term-support policies.
- [ ] Add performance, scale, reliability, and security release gates.
- [ ] Complete threat modeling and independent security review.
- [ ] Publish operations, backup, recovery, and incident-response guidance.
- [ ] Automate release provenance, artifacts, compatibility matrices, and
  upgrade tests.
- [ ] Define project governance for long-term stewardship.

### Exit criteria

- The platform has stable compatibility guarantees and tested migrations.
- Releases are reproducible, signed, documented, and supported.
- Production deployments meet published reliability and security targets.

## Cross-cutting work

These concerns begin early and continue through every phase:

- **Semantics and conformance:** executable specifications and cross-version
  tests.
- **Security and privacy:** least privilege, explicit effects, secrets handling,
  isolation, auditability, and data minimization.
- **Accessibility:** textual and visual workflows that do not depend on a single
  mode of perception or input.
- **Observability:** explainable design-time and runtime behavior.
- **Compatibility:** versioned artifacts, migrations, and no silent semantic
  changes.
- **Documentation:** concepts, tutorials, references, architecture decisions,
  and operational guidance developed alongside features.
- **Community:** public decision-making and extension points that do not favor
  proprietary bundles.

## Immediate next decisions

The first implementation work should wait only on decisions that would be
expensive to reverse:

1. Reserve the required Choreoform package namespaces, domains, and public
   handles once their target ecosystems are selected.
2. The low-level parser baseline was approved in PR #14, not the final authoring
   language. Rust's bounded native/browser confirmation was completed in PR #13.
3. Resolve binding/type foundations and accepted executable dialect contracts,
   then the near-plain-English authoring language and visual notation, as approved
   in the Phase 1 sequence above. Keep the executable IR completion gate open
   until its validation and full benchmark obligations are met.
4. Expand RP-01, RP-03 and RP-08 from excerpts to complete benchmark processes;
   retain all forty corpus scenarios in the coverage/gap accounting.
