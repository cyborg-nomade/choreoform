<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0008: Define the core process semantics as explicit state transitions

**Status:** Proposed<br>
**Date:** 2026-09-02<br>
**Decider:** Project owner

## Context

Phase 1 needs a semantic model before Choreoform can choose a canonical IR,
textual grammar, visual notation, or execution architecture. The model must give
the [Roadmap's](../../ROADMAP.md) control flow, data, actors, time, errors,
cancellation, and side effects one coherent meaning. It must also account for
the [representative process corpus](../representative-processes/README.md)
without importing one domain's workflow vocabulary into the language core.

The [working glossary](../../GLOSSARY.md) adopted by
[ADR-0005](0005-pre-semantic-working-glossary.md) separates process definitions,
plans, process instances, activities, events, state, and capabilities but
deliberately leaves their formal relationships open. The corpus adds harder
constraints: concurrent work can complete partially; late observations can
contradict local expectations; data and decisions have revisions; actors have
bounded authority; instance scope can expand; and external action may succeed
even when its caller cannot observe the result.

Several tempting simplifications are therefore unsafe. A flowchart alone does
not define races or partial joins. Mutable variables alone do not explain which
revision authorized a decision. Exception handling alone cannot represent an
unknown external outcome. Cancellation cannot erase work already performed,
and compensation cannot make history say an effect never happened.

This proposal defines the semantic commitments shared by every representation
and execution target. It does not select syntax, a serialized IR, storage,
transport, or a visual vocabulary. Its evidence profile and corpus mapping are
recorded in the
[semantic-core evaluation](../evaluation/0008-core-process-semantics.md).

## Decision criteria

The semantic model should, in priority order:

1. assign testable meaning to every accepted construct, including invalidity,
   concurrency, nondeterminism, and observable outcomes;
2. make effects, authority, uncertainty, sensitive data, and incomplete
   obligations explicit;
3. preserve one canonical meaning across textual and visual forms without
   depending on layout or source order;
4. use a small set of orthogonal concepts that compose across the adopted
   corpus rather than adding one feature per domain;
5. support local reasoning, hierarchy, dynamic cardinality, and long-running
   instances without requiring a fully expanded authoring graph;
6. retain enough identity, causality, and revision information to explain and
   safely recover an instance; and
7. remain independent of a particular runtime, persistence model, adapter,
   provider, and deployment topology.

## Decision

Define a Choreoform process instance as a **hierarchical labeled transition
system with explicit control obligations, typed versioned data, scoped work,
and append-only observations**.

The semantic unit is an atomic transition from one valid instance
configuration to another. A transition consumes one or more enabled
obligations or accepted observations, evaluates pure rules against a declared
data snapshot, creates data revisions or new obligations, and appends a history
record. It either commits completely or has no internal semantic effect.

Execution targets may use interpreters, generated code, databases, queues, or
distributed workers, but their observable behavior conforms only when it can be
explained as a permitted sequence of these transitions.

### Definitions and instance configurations

A process definition declares:

- a stable definition identity and immutable revision identity;
- typed inputs, outputs, data cells, and immutable artifact references;
- a tree of lexical scopes;
- activity, control, wait, and completion declarations within those scopes;
- pure expressions, guards, invariants, and data transformations;
- participant requirements and authority constraints;
- capability requirements and effect policies; and
- named outcomes, faults, cancellation boundaries, and handlers.

A running instance is bound to one exact definition and derived plan. Editing a
definition never changes an existing instance implicitly. At a semantic step,
an instance configuration contains:

| Component | Meaning |
| --- | --- |
| Definition binding | Exact definition and plan identities used by the instance |
| Scope tree | Active scope occurrences and their parent/child relationships |
| Control obligations | Work that is eligible, active, waiting, or required before a scope can settle |
| Data store | Current typed values and immutable prior revisions visible to each scope |
| Work and effect records | Activity executions, attempts, timers, actor assignments, and capability effects |
| Accepted observations | Correlated inputs available for a semantic transition |
| History | Append-only records of committed transitions and relevant rejected or stale observations |

An **obligation** says what remains semantically outstanding; it is not a claim
that a worker thread is currently running. This distinction permits durable
waits, bounded concurrency, pause and resume, and external work without making
scheduler behavior part of process meaning.

Each runtime occurrence has a stable identity. This includes scope
occurrences, activity executions, attempts, dynamic items, timers, decisions,
data and artifact revisions, observations, and effects. Repetition creates new
occurrence identities instead of reusing an earlier execution record.

### Atomic transitions, ordering, and nondeterminism

The enabled-transition relation is determined only by the bound definition and
the committed instance configuration. A conforming runtime may select any
enabled transition unless the definition establishes a priority or ordering
rule. Consequently:

- causal dependencies impose a partial order, not an invented total order;
- concurrent transitions may commit in either order when their declared reads,
  writes, and obligations do not conflict;
- conflicting transitions are serialized atomically and the loser is
  reevaluated against the new configuration;
- source order, diagram position, connector routing, and wall-clock timestamp
  do not break a race unless a semantic rule explicitly refers to them; and
- every permitted choice among multiple enabled transitions is declared
  nondeterminism and is recorded when resolved.

An observation receives an engine acceptance position when it enters the
instance. That position provides a reproducible local order for racing timers,
messages, cancellations, and completions without claiming to reconstruct an
unknown order in the outside world. Provider timestamps and provenance remain
data that a rule may inspect; they do not silently replace acceptance order.

Given the same definition, initial inputs, accepted observation sequence, and
recorded nondeterministic choices, the semantic transition sequence and
resulting configuration are deterministic.

### Control flow

The core has a small set of control relationships. Textual and visual designs
may provide convenience forms only when they normalize to these relationships
without changing meaning.

| Relationship | Semantic meaning |
| --- | --- |
| Sequence | Completion with a named outcome enables a successor obligation |
| Decision | Pure guards over one snapshot select exactly one eligible branch; zero or multiple eligible branches are faults unless an explicit otherwise/default rule resolves them |
| Parallel split | One transition creates a declared set of child obligations |
| Parallel join | A declared completion predicate over identified child outcomes enables continuation; unselected or unfinished children remain visible and must be settled or transferred |
| Wait | Continuation requires a correlated accepted observation or timer occurrence |
| Repeat | A new body scope occurrence is created while a pure continuation rule holds |
| Dynamic fan-out | A stable, finite collection revision creates one identified child scope per item, subject to runtime concurrency limits |
| Finish | A scope reaches a named outcome only when its completion predicate holds and no unaccounted obligation remains |

A join never infers completion from the absence of work. Its predicate names
which child outcomes are sufficient and what happens to other children.
Runtime concurrency limits affect scheduling, not which child scope occurrences
exist or what their outcomes mean.

Dynamic work is definition-bounded. A definition may instantiate declared
activity or scope templates from versioned data, including an actor-authorized
generic task whose instructions are data. Runtime input may not inject new
control semantics or undeclared capabilities into an instance.

### Data and state

Process data is typed and scoped. A data cell has a stable identity and a
sequence of immutable revisions; its current value is the latest revision
visible in the configuration. Inputs, outputs, artifacts, observations, and
decision evidence likewise identify the revision used.

Internal computation is pure. A transition declares its read set, evaluates
guards and transformations against one consistent snapshot, validates types and
invariants, and atomically publishes its write set. Concurrent writes to the
same cell conflict unless a declared commutative merge operation applies.
There is no implicit last-write-wins rule.

Artifact content may remain in an external repository, but semantic references
include stable identity, revision, integrity information where required, and
provenance. Correcting information creates a new revision and never rewrites
history. Decisions and effects bind to the exact data and artifact revisions
they used. A definition may declare invalidation dependencies so a new revision
withdraws the sufficiency of affected approvals or work without deleting their
records.

Data declarations carry protection metadata sufficient to express sensitivity,
purpose, and permitted participant or capability access. This metadata is
semantic: validation and execution must not silently widen access. The concrete
type system, information-flow analysis, redaction scheme, and policy language
are deferred.

### Actors and human work

A **participant** is an identified person, software agent, organization, or
device that may satisfy a declared actor requirement. A requirement is a typed
predicate over role, authority, relationship, separation of duties, and other
definition-visible attributes; it is not a hard-coded account or provider.

Human and delegated work uses an explicit lifecycle: offered, assigned,
accepted, completed, refused, withdrawn, or expired. Allocation policy may
choose among eligible participants, but it cannot weaken the requirement.
Completion is an attributable observation correlated to one activity execution
and its input revisions. The required authority is checked again when the
completion is accepted, not assumed from assignment alone.

Human judgment is external input, not a result invented by the engine. A timer,
model recommendation, assignment, or delivered message cannot manufacture a
human decision. Overrides and waivers are distinct decisions that record actor,
authority, reason, scope, affected revisions, and any expiry.

Participants do not mutate process state directly. Their observations become
accepted inputs to atomic transitions, which apply the definition's rules and
record both accepted and rejected stale outcomes.

### Time

Time enters semantics through explicit clock and calendar dependencies. A timer
record identifies its purpose, basis, due instant or calculation inputs, time
zone, calendar and policy revision, and lifecycle. Registering, rescheduling,
pausing, resuming, cancelling, and firing a timer are observable transitions.

A timer becomes eligible at or after its due condition according to its named
clock; eligibility does not itself make a transition commit. A firing races
with other observations through the normal atomic ordering rule. Rescheduling
creates a new timer revision and retains the old basis in history.

Pausing time is never global or implicit. A pause names the timers or measured
intervals affected, its authority and reason, and its start and end. Deadlines
may warn, escalate, fail technical work, or return control to an actor, but they
cannot silently create a business or expert decision.

Calendar algorithms, clock precision, clock trust, and policy-specific deadline
calculation are capabilities or later specifications, not hard-coded core
rules.

### Outcomes, faults, and retries

The model distinguishes five conditions that must not be collapsed:

| Condition | Meaning |
| --- | --- |
| Named business outcome | A valid domain-relevant result handled by ordinary control flow |
| Activity fault | A typed failure raised while performing declared work |
| Effect outcome unknown | An external request may or may not have taken effect |
| Cancellation | A scoped request to stop or settle work, independent of fault handling |
| Engine fault | The runtime cannot continue conformantly because of an internal or resource failure |

Faults carry a type, source occurrence, cause, and relevant data revision. A
fault propagates to the nearest lexically enclosing matching handler. Entering
a handler does not erase completed work. An unhandled activity fault fails its
scope; the parent definition determines whether that outcome propagates,
triggers sibling cancellation, or is handled as data.

A retry is a policy-driven creation of a new attempt for the same activity
execution. Attempts have distinct identities, while their logical work and any
effect identity remain stable where the policy requires deduplication. Retry
eligibility, limit, backoff, and reconciliation preconditions are explicit and
version-bound. Retrying never rewinds data or history.

Definition errors, including an unbound capability, unsatisfied type, statically
provable decision ambiguity, invalid join, or unsafe retry policy, prevent
planning and produce a specified validation diagnostic. A decision conflict
that depends on runtime data raises the decision fault defined above; neither
kind is a business outcome.

### Cancellation and compensation

Cancellation is a monotonic, scoped request. Once accepted for a scope
occurrence it prevents new ordinary work in that scope, propagates to active
children according to declared boundaries, and asks cancellable in-flight work
to stop. It does not imply that a person, device, provider, or already-committed
effect stopped.

Each in-flight child must eventually be classified as completed, failed,
cancelled, or outcome unknown. A scope reaches its cancelled outcome only when
all its obligations are settled, explicitly transferred to another scope, or
recorded as outstanding follow-up owned outside the closing scope. Late
observations remain admissible for reconciliation even after ordinary work has
stopped.

Compensation is new, explicit work linked to a prior confirmed or possibly
completed effect. It has its own authority, effects, attempts, failures, and
observable outcome. Compensation never deletes the original effect or
guarantees restoration. Reusable compensation orchestration is deferred, but a
conforming definition can express it with ordinary scoped control and effect
semantics.

### Capabilities and side effects

Pure transitions are the only way to change internal process data. Every
interaction that may observe or change the outside world crosses a declared,
typed **capability** boundary. Binding a capability to a participant, adapter,
or provider is planning or deployment configuration and cannot change its
declared observable contract.

An effect record separates logical effect identity from technical attempts and
uses at least these observable conditions: planned, requested, acknowledged,
succeeded, rejected, cancelled, and outcome unknown. A capability contract
declares:

- input and output types and permitted data access;
- required authority and protection constraints;
- which observations constitute success, rejection, or reconciliation;
- whether and how a stable idempotency identity is honored;
- whether an attempt is read-only, reversible only by another effect, or
  irreversible for process purposes; and
- retry, timeout, cancellation, and reconciliation requirements.

No provider-independent exactly-once guarantee is implied. After an ambiguous
attempt, another attempt is permitted only when the contract provides a stable
idempotency rule or a reconciliation transition establishes that retry is
safe. Otherwise the process must retain an unknown outcome and route it to
explicit handling.

External facts are immutable observations. Duplicate or stale delivery may be
recorded, but an observation identity can affect process state at most once.
Provider state progression must not regress unless the capability contract
defines that transition. Notification delivery, assignment, requested action,
and confirmed real-world outcome remain separate facts.

### History, recovery, and conformance

Every committed semantic transition appends an observable history record with
its instance and occurrence identities, accepted inputs, transition kind,
causal predecessors, data revisions read and written, actor or engine
authority, recorded choice, effect correlations, and known time information.
Rejected stale or duplicate observations that matter to explanation are also
retained without reapplying their state change.

The semantic requirement is an append-only explanatory history, not event
sourcing. A runtime may store snapshots, journals, relational rows, or another
representation if recovery preserves the last committed configuration and the
required history. Recovery must use the bound definition and plan. Work whose
external outcome cannot be derived after a crash becomes outcome unknown; it is
not guessed successful or failed.

An implementation conforms for a process definition when:

1. it rejects definitions that violate the model's static validity rules;
2. every observed instance step corresponds to an enabled semantic transition;
3. it introduces no behavior outside the model's declared nondeterminism;
4. it preserves required identities, revision bindings, obligations, and
   protection constraints; and
5. its outputs and explanatory history distinguish all outcomes required by
   the definition.

## Options considered

| Option | Advantages | Costs and risks | Outcome |
| --- | --- | --- | --- |
| **Hierarchical transition system with explicit obligations** | One operational account for control, data, races, failure, cancellation, and effects; supports hierarchy and dynamic cardinality; representation-neutral | More semantic machinery than a flow graph; formal rules and conformance cases still need to be written | Adopt |
| Petri-net-style token-flow core | Mature concurrency intuition and analysis; visual affinity | Data, authority, time, effect uncertainty, hierarchy, and cancellation need substantial added semantics; token counts can obscure business obligations | Reject as the complete core; retain as a possible analysis projection |
| Structured process algebra only | Strong composition and local reasoning; textual precision | Dynamic graphs, external observations, actor work, and visual mapping need additional operational state; unfamiliar algebra may burden authors | Reject as the user-facing semantic foundation; retain as a formalization technique |
| BPMN execution semantics and metamodel | Broad industry vocabulary and tooling precedent | Large inherited surface, ambiguous or optional execution corners, and early commitment to another notation and metamodel | Reject |
| Runtime-defined orchestration callbacks | Direct implementation path and host-language flexibility | Meaning depends on one engine; weak static analysis and visual parity; effects and nondeterminism can remain hidden | Reject |

The selected model can be formalized with structural operational semantics and
projected into graphs for analysis. Those are implementation and specification
techniques, not competing sources of meaning.

## Consequences

- Textual, visual, IR, planning, and engine proposals gain a common semantic
  target and must preserve its identities and distinctions.
- Concurrency is explicit but scheduler-independent; definitions cannot depend
  on accidental source, layout, or wall-clock ordering.
- Revision-bound decisions, partial fan-out, dynamic scope, human authority,
  timers, and late observations can be expressed without domain-specific core
  constructs.
- External effect uncertainty and outstanding obligations remain visible,
  improving recovery and auditability but requiring more states than a simple
  succeeded/failed API.
- Cancellation and compensation cannot promise fictional rollback. Authors and
  operators must handle completed, in-flight, and unknown work explicitly.
- Protection metadata becomes semantic even though its concrete policy and
  enforcement language remain later work.
- Implementations must preserve an explanatory history, but are free to choose
  persistence and scheduling architectures.
- The model is still a paper design with worked corpus evidence. Formal
  transition rules, executable examples, notation studies, and independent
  implementations may expose a need for a superseding decision.

## Confirmation

This decision is implemented and remains respected when:

- the canonical IR can represent every configuration component and stable
  identity above without encoding syntax- or layout-only meaning;
- the textual and visual forms map every semantic distinction and identify
  representation-only metadata;
- operational rules and conformance tests cover sequence, ambiguous choice,
  parallel completion, dynamic fan-out, racing observations, atomic data
  conflict, authority checks, timer revision, fault propagation, retry,
  scoped cancellation, compensation, duplicate delivery, and unknown effects;
- at least three representative processes are expressed in both forms and all
  forty adopted corpus scenarios retain a traceable semantic account; and
- two independent engines given the same definition, inputs, observation
  acceptance sequence, and recorded choices produce equivalent outcomes and
  required history.

## Open questions and deferred choices

The following choices are intentionally not made by this ADR:

1. the concrete type, expression, name, import, parameter, and composition
   systems;
2. canonical IR schema, serialization, identity encoding, and version rules;
3. textual grammar, surface convenience constructs, and formatting;
4. visual vocabulary, layout metadata, interaction design, and complexity
   management;
5. the formal notation used to publish transition rules and proofs;
6. persistence, event sourcing, transactions, scheduling, leases, and
   distributed delivery architecture;
7. calendar algorithms, clock-trust policy, retry/backoff algorithms, and
   resource-allocation policy;
8. participant identity, authentication, authorization-policy, redaction, and
   information-flow enforcement mechanisms;
9. capability discovery, adapter protocol, deployment permissions, and
   sandboxing;
10. reusable compensation constructs beyond the core rule that compensation is
    explicit linked work; and
11. migration of a running instance to another definition or plan. Until
    separately decided, it is prohibited rather than implicit.

Open review questions for this proposal are:

- Is the obligation model sufficiently small to serve both human-readable
  notation and formal transition rules, or should activity, wait, and effect
  obligations have separate core kinds?
- Should acceptance position be the mandatory race-resolution baseline, or
  should definitions be required to declare a resolver for every potentially
  consequential race?
- Is semantic protection metadata feasible in Phase 1 before the authorization
  and type systems are chosen, and what minimum fields must the first IR retain?
- Which join predicates can be admitted while keeping validation and visual
  explanation tractable?

## Acceptance and action items

This proposal becomes effective only after Project Owner approval and merge.

1. [ ] Obtain Project Owner approval.
2. [ ] Resolve or explicitly defer the open review questions.
3. [ ] Change this ADR's status to Accepted and record the approval.
4. [ ] Mark the linked Roadmap deliverable complete.
5. [ ] Convert the semantic commitments into formal rules and conformance
   examples alongside the canonical IR work.
6. [ ] Re-evaluate the model with executable and cross-form evidence before the
   Phase 1 exit criteria are declared complete.
