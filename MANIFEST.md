# Manifest

## Purpose

Organizations run on processes, but those processes are commonly scattered
across prose, diagrams, spreadsheets, scripts, application settings, and human
memory. The same process is repeatedly translated between people who design it,
people who implement it, and systems that execute it. Meaning is lost at every
translation.

Choreoform aims to make an information process a single, precise artifact that
people can understand, software can execute, and tools can safely transform.

## The objective

Build a coherent product family with four parts:

1. **A general process language** with equivalent textual and visual forms. It
   should describe information processes without assuming a particular
   industry, vendor, user interface, or deployment platform.
2. **An execution engine** that validates a process definition and turns it
   into an executable plan, generated program, or target-specific artifact.
3. **A studio** in which visual and textual editing are two synchronized views
   of the same process, supported by IDE features such as diagnostics,
   navigation, simulation, and debugging.
4. **Process bundles** containing useful, tested, and supported processes for
   different types and sizes of business.

The first three products form an open foundation. The fourth is a commercial
product line built on that foundation.

## What “general” means

The language should model the recurring elements of information processes:

- events and triggers;
- activities, decisions, and parallel work;
- information, state, and transformations;
- people, software agents, roles, and responsibilities;
- resources, capabilities, and external systems;
- time, deadlines, retries, cancellation, and compensation;
- rules, invariants, permissions, and failure paths;
- composition, reuse, configuration, and versioning.

“General” does not mean encoding every domain concept in the core language.
The core should remain small and precise; domain vocabulary and integrations
belong in typed extensions, libraries, and bundles.

## One model, two representations

Text and diagrams must not become competing sources of truth. Both
representations should map losslessly to a canonical process model, subject to
explicit rules about visual layout metadata and textual formatting.

A user should be able to:

1. edit a process as text;
2. see the corresponding diagram update;
3. edit the diagram;
4. return to meaningful, stable text; and
5. execute the same semantics throughout that round trip.

If a construct cannot be represented faithfully in both primary forms, it is
not yet part of the language.

## Execution is semantics, not diagram animation

The engine must give every accepted construct defined runtime meaning. A
process that validates should have predictable behavior for inputs, outputs,
state changes, concurrency, failures, and cancellation.

The execution pipeline should keep concerns separate:

```text
visual or text source
        ↓
parse and validate
        ↓
canonical process model
        ↓
normalize and plan
        ↓
interpreter, generated code, or target adapter
        ↓
observable execution
```

The canonical model is the contract between language tooling and execution
backends. It should be specified and testable independently of any one engine
implementation.

## Principles

### Precise before broad

A small language with executable, testable semantics is more valuable than a
large notation with ambiguous behavior.

### Human-legible and machine-checkable

Processes should be readable by their participants while remaining strict
enough for validation, analysis, and execution.

### Progressive disclosure

Simple processes should look simple. Advanced concerns—distribution,
compensation, security policy, deployment, and optimization—should be
available without overwhelming the common case.

### Determinism where promised

The language should make nondeterminism, concurrency, time, and external side
effects visible. Tools must not imply stronger guarantees than a target runtime
can provide.

### Safe evolution

Language versions, extensions, bundles, and execution targets need explicit
compatibility rules. A saved process should never silently change meaning.

### Inspectable execution

Users should be able to explain what ran, why a path was selected, which data
was used, what changed, and how a failed process can be resumed or compensated.

### Portable core, explicit integrations

Core semantics should not depend on a vendor. Interactions with databases,
queues, APIs, AI models, and business applications should use explicit,
replaceable capabilities.

### Free foundation

The language specification, canonical model, engine, studio, conformance
suite, and public extension interfaces will be free software. Their development
should be possible in public without access to proprietary bundles.

## Open and commercial boundary

Commercial process bundles may be proprietary, but they must consume the same
documented public interfaces available to everyone. They must not be required
to build, test, or operate the open foundation.

The intended boundary is:

| Open foundation | Commercial products |
| --- | --- |
| Language specification and schemas | Domain-specific process definitions |
| Parser, validator, and canonical model | Curated configurations and content |
| Execution engine and public adapters | Bundle-specific integrations where licensing permits |
| Studio and language tooling | Commercial packaging, support, and service agreements |
| SDK, bundle format, and compatibility tools | Certified business process bundles |
| Conformance and reference test suites | Proprietary bundle test data and know-how |

Licenses must preserve this boundary without restricting independent users from
writing, sharing, or selling their own processes and bundles.

## Non-goals for the first releases

- Replacing general-purpose programming languages.
- Encoding every business domain in the language core.
- Building a distributed workflow platform before local semantics are proven.
- Standardizing a visual notation before usability testing.
- Promising automatic optimization or AI-generated correctness.
- Creating proprietary advantages by withholding essential runtime interfaces.

## Measures of progress

The project is succeeding when it can demonstrate that:

- one non-trivial process round-trips between text and visuals without semantic
  loss;
- independent implementations can interpret the canonical model consistently;
- the engine can execute, observe, pause, resume, and diagnose representative
  processes;
- a new integration can be added through a stable public interface;
- the studio makes both first-time modeling and expert textual editing
  effective; and
- a proprietary bundle can be installed and run without private modifications
  to the open foundation.

## Invitation

This project begins with questions, not a finished notation. Early work should
favor executable examples, falsifiable semantics, and documented decisions.
Names, syntax, and architecture may change; the commitment to a precise,
portable, inspectable, and free foundation should not.
