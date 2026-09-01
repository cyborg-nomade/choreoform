<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-NN: Process name

| Attribute | Value |
| --- | --- |
| Domain | Domain or operating context |
| Complexity | Level 1–4 from the corpus index |
| Primary participants | People, software, organizations, or devices |
| Typical duration | Useful range, not a runtime guarantee |
| Automation mix | Human, software, device, or mixed |

## Purpose and corpus role

Explain the intended outcome and the distinct language-design pressure this
case contributes. State that the case is synthetic when domain readers could
mistake it for operating guidance.

## Scope and assumptions

Define the boundary, important simplifications, jurisdiction or policy
dependencies, and what is deliberately treated as an external capability.

## Participants

- **Participant:** responsibility, authority, or constraint.

## Trigger and preconditions

State what may start a process instance, required inputs, and facts that must
already hold. Distinguish a request or command from a recorded event.

## Information and state

- Information whose identity, provenance, version, sensitivity, or retention
  matters.
- Lifecycle conditions and pending obligations the instance must remember.

## Main success path

1. Describe a recognizable end-to-end outcome without implying syntax.

## Alternatives and failures

- Describe rejected, missing, duplicate, late, unavailable, partially
  completed, corrected, cancelled, and recovered paths that matter.

## Time, concurrency, and scale

Describe deadlines and calendars; work that may proceed together or race;
ordering constraints; volume and resource limits; and pause/resume behavior.

## Capabilities and effects

- **Capability intent:** observable effect and relevant uncertainty. Do not
  choose a provider or adapter unless that choice is essential to the case.

## Invariants and protections

- State a condition that must remain true across every path, especially around
  authority, privacy, safety, money, physical resources, and duplicate effects.

## Observable outcomes

- State what an operator, participant, auditor, or calling system must be able
  to distinguish or explain.

## Acceptance scenarios

### RP-NN-A — Short scenario name

- **Given** relevant initial state,
- **when** an occurrence or decision happens,
- **then** state and observable effects have a specific outcome.

Include at least three scenarios. Cover one ordinary completion, one
alternative or failure, and one timing, concurrency, recovery, or change case.

## Semantic pressures exposed

- Phrase unresolved requirements as questions for future semantic and notation
  proposals. Avoid prescribing a construct or graphical shape.

## Out of scope

- Identify adjacent work the example does not claim to model.

## Sources and inspiration

- Link primary or authoritative material used to check the pressure. Explain
  deviations when readers might otherwise infer compliance.
