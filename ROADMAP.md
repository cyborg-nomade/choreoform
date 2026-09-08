<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Roadmap

This roadmap is organized around evidence and exit criteria rather than dates.
Each phase should produce a usable vertical slice and the decisions needed by
the next phase. Phases may overlap, but their exit criteria should not be
skipped.

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
- [ ] Design an initial textual grammar and parser. Proposed in
  [ADR-0011](docs/decisions/0011-initial-textual-grammar.md), with a bounded
  [syntax and parser prototype](docs/text/README.md); awaiting owner review.
- [ ] Define names, types, scopes, imports, parameters, and composition.
  This is the proposed next deliverable after PR #14, establishing the binding
  and type foundations needed by the executable contracts below.
- [ ] Specify and obtain approval for executable type, expression, and policy
  dialect contracts replacing the illustrative payloads: exact values and
  operations, purity/dependencies, authority/protection, time, effects,
  cancellation, reconciliation, and settlement. Include worked positive and
  negative semantic cases; a JSON slot or prose label is not an implementation.
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
- [ ] Design an initial visual notation and serialization of layout metadata,
  using the same contracts and full benchmarks as the authoring language.
- [ ] Implement complete structural/link/semantic validation of the accepted
  contracts and stable diagnostics; include negative cases and fail-closed
  handling of unknown dialects and unenforceable policies. Parsing/hashing and
  the existing partial probe are not substitutes.
- [ ] Build text → IR → text and visual → IR → visual round trips.
- [ ] Publish small executable examples and negative examples.
- [ ] Start a language conformance suite.

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

**Sequencing proposal in PR #14:** finish review of the bounded parser, then
address binding/type foundations and executable contracts before advancing to
the near-English and visual authoring designs. Each deliverable still requires
its own review and permission to start. Authoring studies may feed back into the
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
2. Review PR #14 as a low-level parser baseline, not the final authoring language.
   Rust's bounded native/browser confirmation was completed in PR #13.
3. Resolve binding/type foundations and accepted executable dialect contracts,
   then the near-plain-English authoring language and visual notation, as proposed
   in the Phase 1 sequence above. Keep the executable IR completion gate open
   until its validation and full benchmark obligations are met.
4. Expand RP-01, RP-03 and RP-08 from excerpts to complete benchmark processes;
   retain all forty corpus scenarios in the coverage/gap accounting.
