<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0005: Establish a pre-semantic working glossary

**Status:** Accepted<br>
**Date:** 2026-08-31<br>
**Decider:** Project owner

## Context

The Manifest and Roadmap already use *process*, *definition*, *plan*,
*instance*, *activity*, *event*, *state*, *capability*, and *bundle*. These
terms carry different meanings in business-process modeling, workflow engines,
distributed systems, programming languages, and ordinary business speech.

The most consequential ambiguity is the word *process*: it can describe an
abstract pattern, an authored file, a deployed executable, an operating-system
process, or one running case. Similar ambiguity exists between an activity and
one attempt to execute it, an event and the message carrying it, state and
history, or a capability and its provider.

Phase 0 needs consistent language for representative use cases and design
evaluation. Phase 1 must still be free to define formal semantics based on
evidence. The glossary therefore needs to establish useful conceptual
boundaries without masquerading as a normative language specification.

## Decision criteria

The vocabulary should:

1. distinguish authored, derived, runtime, and distribution artifacts;
2. remain domain-neutral and applicable to people and software participants;
3. support equivalent textual and visual representations;
4. separate facts, intent, work, state, history, and external effects;
5. support inspectable execution and safe evolution;
6. preserve the public interface between the open foundation and proprietary
   bundles;
7. align with useful industry conventions without adopting another metamodel;
   and
8. leave formal syntax and operational semantics to later decisions.

## Decision

Adopt [GLOSSARY.md](../../GLOSSARY.md) as Choreoform’s working, conceptual, and
non-normative vocabulary for Phase 0 and early Phase 1 discussion.

The glossary establishes four layers:

| Layer | Concept | Boundary |
| --- | --- | --- |
| Authoring | Process definition | Reviewed source of process intent |
| Preparation | Plan | Validated representation derived for execution |
| Runtime | Process instance | One enactment with identity, state, and history |
| Distribution | Bundle | Package of definitions and related material |

Use *process* as the broad conceptual term. Use *process definition* and
*process instance* whenever an authored artifact and a runtime enactment could
be confused. Apply the same definition/runtime distinction to an *activity*
and an *activity execution*.

Treat an event as an immutable record expressing a recognized occurrence, not
as a request to perform work and not as its transport message. Keep current
state distinct from execution history without choosing event sourcing or
another persistence model.

Treat a capability as a typed contract for something a participant or
environment can provide. Keep it distinct from both the process activity that
requires it and the adapter or provider that fulfills it.

Treat a plan as derived rather than authored and a bundle as a distribution
unit rather than a semantic or runtime unit.

The glossary does not select syntax, canonical-IR encoding, lifecycle enums,
event delivery guarantees, state persistence, capability protocol, or bundle
format. A later normative specification or accepted ADR may refine or supersede
a working definition. Editorial clarification may use routine review; a change
to a conceptual boundary requires an ADR under ADR-0004.

## Options considered

| Option | Immediate clarity | Semantic flexibility | External coupling | Outcome |
| --- | --- | --- | --- | --- |
| **Project-specific working glossary** | High | High when explicitly non-normative | Low | Adopt |
| Defer all definitions to Phase 1 | Low | Highest | None | Reject: use cases and design criteria remain ambiguous |
| Adopt BPMN terminology and metamodel | High for BPMN users | Low | High | Reject: narrows the language before evidence |
| Use ordinary dictionary meanings only | Low–medium | Medium | Low | Reject: runtime and artifact boundaries remain unclear |

The project-specific glossary can align selectively with standards while
preserving Choreoform’s goals. In particular, the event definition follows the
fact-oriented direction of CloudEvents, and the activity definition is
compatible with W3C PROV’s time-extended work concept, but neither standard is
made a Choreoform dependency.

Importing BPMN wholesale would provide mature terminology and notation. It
would also import distinctions, execution assumptions, and graphical concepts
before representative use cases establish whether they fit a general
information-process language.

Deferring terminology would avoid premature choices but make the next Roadmap
deliverables compare examples using unstable words. Marking the glossary as
working and non-normative captures the value without claiming formal semantics.

## Consequences

- Contributors can describe representative processes and competing designs
  using consistent conceptual layers.
- Documentation must qualify *definition*, *plan*, and *instance* where the
  shorter word would be ambiguous.
- Phase 1 can formalize or change the concepts through explicit decisions
  rather than silently drifting terminology.
- Some adjacent communities use *event*, *activity*, *workflow*, or *process*
  differently; Choreoform documentation must state its own meaning.
- The glossary creates no compatibility promise for source syntax, serialized
  artifacts, or runtime APIs.
- New core terms will need the same boundary-focused treatment as the language
  expands.

## Confirmation

The decision is implemented when:

- all nine terms named by the Roadmap have explicit entries;
- the glossary distinguishes definition, plan, instance, and bundle;
- one example uses the complete vocabulary without contradiction;
- README and Contributing make the glossary discoverable; and
- the decision index includes this ADR.

Future semantic ADRs and specifications should link or explicitly refine the
glossary rather than reuse a core term with an unstated meaning.

## Sources

- [CloudEvents specification](https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/spec.md)
- [CloudEvents primer](https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/primer.md)
- [W3C PROV Data Model](https://www.w3.org/TR/prov-dm/)
- [OMG Business Process Model and Notation 2.0.2](https://www.omg.org/spec/BPMN/2.0.2/PDF)
- [Open Workflow Specification](https://github.com/open-workflow-specification/specification)

## Acceptance and action items

The Project Owner approved this ADR on 2026-08-31. The working glossary becomes
effective when pull request #6 is merged.

1. [x] Obtain Project Owner approval.
2. [x] Change this ADR’s status to Accepted and record the approval.
3. [x] Publish the working glossary and boundary example.
4. [x] Add the ADR to the decision index and link the glossary from contributor
   entry points.
5. [x] Mark the Roadmap deliverable complete after approval.
6. [x] Revisit definitions as the formal semantic model is accepted in
   [ADR-0008](0008-core-process-semantics.md).
