<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Choreoform working glossary

**Status:** Working, conceptual, and non-normative<br>
**Decision record:** [ADR-0005](docs/decisions/0005-pre-semantic-working-glossary.md)

This glossary gives contributors a shared vocabulary before the formal
semantic model exists. It defines conceptual boundaries for discussion,
examples, and early design work; it does not define language syntax, a storage
schema, or final execution semantics.

Phase 1 specifications and later ADRs may refine these meanings. Until then,
use the qualified terms in this document whenever the shorter word would be
ambiguous.

## Concept map

```text
bundle
  └── contains → process definition
                    │
                    ├── text and visual forms share one canonical meaning
                    │
                    └── validate + normalize + bind → plan
                                                       │
                                                       └── start → process instance
                                                                    ├── executes activities
                                                                    ├── observes and records events
                                                                    ├── changes state
                                                                    └── uses capabilities
```

The map separates four layers that should not be used interchangeably:

| Layer | Primary concept | What it represents |
| --- | --- | --- |
| Authoring | Process definition | The behavior and requirements people create and review |
| Preparation | Plan | A validated, derived representation prepared for execution |
| Runtime | Process instance | One identifiable enactment with state and history |
| Distribution | Bundle | A package that delivers definitions and related material |

## Core terms

### Process

A **process** is a coordinated pattern of information-related behavior that
connects occurrences, work, decisions, information, participants, and rules
toward one or more outcomes.

A process may involve people, software, devices, or combinations of them. It
may be long-running, concurrent, nondeterministic, or partially automated. The
word is the broad conceptual term; it does not identify a particular file or a
particular run.

When precision matters, use **process definition** for the authored artifact
and **process instance** for one runtime enactment.

### Definition

A **definition** is an authored, declarative description of allowable
structure, behavior, requirements, and configuration. It describes what may or
should happen rather than recording what did happen in one run.

Within Choreoform, unqualified *definition* normally means **process
definition** when the context is clear. A definition is treated as fixed for a
specific plan or process instance; the later versioning model will determine
how revisions receive identity and compatibility guarantees.

A definition is not a plan, runtime state, execution history, or bundle.

### Process definition

A **process definition** is a definition of a process. Its textual and visual
forms express the same canonical meaning, apart from explicitly non-semantic
presentation metadata such as layout or source formatting.

A process definition declares activities, event relationships, data and state
requirements, control rules, required capabilities, inputs, outputs, and
failure behavior to the extent supported by the language. It may compose or
reference other definitions.

The definition is the reviewed source of intent. Validation, normalization, or
target preparation may derive a plan from it but must not silently change its
meaning.

### Plan

A **plan** is a validated, derived representation of a specific process
definition prepared for a particular kind of execution.

Planning may resolve imports and types, normalize equivalent constructs,
select legal execution strategies, bind configuration, and identify capability
requirements. A plan may be portable or target-specific; that boundary remains
a Phase 1 and Phase 2 design decision.

A plan is not authored as the primary source of truth and is not itself a
running process instance. Multiple plans may preserve the semantics of one
definition for different execution targets.

### Instance

An **instance**, or **process instance**, is one identifiable runtime enactment
of a particular process definition through a particular plan.

An instance has its own inputs, lifecycle, state, event history, activity
executions, outputs, and outcome. Multiple instances may run from the same
definition. An instance may be active, waiting, completed, failed, cancelled,
or in another lifecycle condition that later semantics define.

An instance remains associated with the exact definition and plan from which it
started so that a later edit cannot silently change the meaning of work already
in progress.

### Activity

An **activity** is a named unit of work declared within a process definition.
It consumes time conceptually, even when an implementation completes it
immediately, and it may read information, produce information, change state, or
cause effects through capabilities.

An activity describes work in the model; an **activity execution** is one
runtime occurrence of that work within a process instance. Retries or repeated
paths may produce multiple activity executions for the same declared activity.

Activities may be carried out by people, software, or other participants. An
activity is not a capability: the activity states the work the process calls
for, while a capability states what a participant or environment can provide.

### Event

An **event** is an immutable record expressing that an occurrence was
recognized, together with the context needed to interpret it.

Events represent facts from the process’s perspective: something happened, was
observed, or was recorded. An event may trigger a process instance, make an
activity eligible, record an outcome, or explain a state transition. It does
not itself request that work be performed.

A command, request, timer registration, or signal expresses intent or input; it
may later lead to an event. Transport messages may carry events, but an event
is not defined by its transport. Identity, ordering, delivery, deduplication,
and time semantics remain later design decisions.

### State

**State** is the information about a process instance at a point in its
lifecycle that the engine uses to determine its current condition and legal
future behavior.

State may include control position, process data, pending activities, timers,
capability interactions, cancellation or failure context, and recorded
outcomes. The semantic model will decide which parts are durable, derived, or
externally referenced.

State is not the same as event history. History records relevant occurrences
and decisions over time; state describes the instance’s current condition.
State may be reconstructed from history in some execution models, but this
glossary does not require event sourcing.

### Capability

A **capability** is a named, typed contract for something a process may require
a participant or its environment to do or provide.

A capability expresses intent at the process boundary without selecting a
particular provider. It may be fulfilled by a person, software service, device,
model, or other implementation through a binding or adapter. Examples include
requesting an approval, sending a notification, storing a document, or looking
up an account.

A capability is not an activity or an adapter. An activity may require one or
more capabilities; an adapter is one implementation mechanism that connects a
capability contract to a provider. Permissions, effects, retries, and
idempotency belong to the eventual capability semantics, not this working
definition.

### Bundle

A **bundle** is a versioned, distributable package of one or more process
definitions plus the metadata and related material needed to install,
configure, validate, or operate them.

Depending on the future bundle format, related material may include assets,
configuration schemas, declared dependencies, compatibility constraints,
tests, migrations, documentation, and references to required extensions or
capabilities. A bundle need not be executable without satisfying those
requirements.

A bundle is not the same as a process definition or extension. It may contain
definitions and depend on extensions. Bundles may be free or proprietary under
the licensing boundary in [ADR-0002](docs/decisions/0002-licensing-policy.md),
but proprietary bundles use the same documented public interfaces as free
ones.

## Supporting terms

These terms are included because they clarify the boundaries above. They do not
expand the Phase 0 deliverable into a complete semantic model.

### Activity execution

One runtime occurrence of a declared activity within a process instance. It
has instance-specific inputs, timing, status, attempts, and outcomes. Retry and
attempt identity remain to be specified.

### Adapter

An implementation that connects a capability contract to a particular person,
service, device, protocol, or provider. Adapters belong at an explicit
integration boundary and are replaceable without changing process intent when
their observable behavior satisfies the same contract.

### Canonical model

The representation of a process definition’s semantic meaning shared by the
textual and visual forms. It excludes representation-only details except where
they must be retained for stable round trips. The versioned intermediate
representation described in the Roadmap may encode this model, but its format
has not been selected.

### Execution history

The durable record of events, decisions, activity executions, capability
interactions, and state transitions needed to explain what happened in a
process instance. Whether history is the source of state or an audit projection
is a later architecture decision.

## Boundary example

Consider a purchase-approval process:

- The **process definition** declares how a request is checked, routed for
  approval, accepted, rejected, or escalated.
- A **bundle** may package that definition with configuration schemas,
  documentation, tests, and an accounting-system dependency.
- The engine validates the definition and derives a **plan** for a local or
  hosted execution target.
- Purchase request `PR-1042` starts one **process instance**.
- “Obtain manager approval” is an **activity**; its actual assignment and
  completion are an **activity execution**.
- “Request human approval” is a **capability** whose provider might be the
  Studio inbox, email, or an external task system through an adapter.
- “Manager approval recorded” is an **event**.
- The request data, current waiting point, deadline, and recorded decision are
  part of the instance’s **state**.

## Alignment and independence

The vocabulary is intentionally project-specific but informed by established
work:

- CloudEvents distinguishes an occurrence from the event record that expresses
  it and separates event facts from transport messages. Choreoform follows that
  direction without adopting the CloudEvents wire format as a core requirement.
- W3C PROV treats activities as things that occur over time and act on
  information entities. Choreoform additionally distinguishes a declared
  activity from its runtime executions.
- BPMN provides widely recognized process-modeling terminology, but this
  glossary does not adopt the BPMN metamodel, execution semantics, or graphical
  notation.
- Open Workflow provides a nearby example of a vendor-neutral workflow DSL,
  but Choreoform keeps its vocabulary independent while its semantic scope is
  still being established.

## Sources

- [CloudEvents specification](https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/spec.md)
- [CloudEvents primer](https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/primer.md)
- [W3C PROV Data Model](https://www.w3.org/TR/prov-dm/)
- [OMG Business Process Model and Notation 2.0.2](https://www.omg.org/spec/BPMN/2.0.2/PDF)
- [Open Workflow Specification](https://github.com/open-workflow-specification/specification)
