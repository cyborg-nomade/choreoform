<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Architecture decision records

Architecture decision records (ADRs) explain why Choreoform made durable,
consequential choices. They preserve the context, alternatives, decision, and
consequences for future contributors. In Choreoform, “architecture” includes
institutionally significant decisions such as licensing and governance, not
only software structure.

ADRs complement current documentation: an ADR explains why a choice was made,
while specifications, guides, and code describe the system as it exists now.

## Decision index

| ADR | Status | Decision |
| --- | --- | --- |
| [0001](0001-project-and-language-name.md) | Accepted | Name the project and language Choreoform |
| [0002](0002-licensing-policy.md) | Accepted | License the open foundation without capturing user processes |
| [0003](0003-community-governance.md) | Accepted | Establish lightweight community governance and contribution policy |
| [0004](0004-lightweight-adr-process.md) | Accepted | Adopt a lightweight, repository-native ADR process |
| [0005](0005-pre-semantic-working-glossary.md) | Accepted | Establish a pre-semantic working glossary |
| [0006](0006-representative-process-corpus.md) | Accepted | Adopt a representative process corpus |
| [0007](0007-design-evaluation-framework.md) | Accepted | Evaluate semantic and notation designs with evidence profiles |
| [0008](0008-core-process-semantics.md) | Accepted | Define the core process semantics as explicit state transitions |

## When an ADR is required

Write an ADR before committing the project to a decision that is expensive to
reverse, constrains later work, or needs durable rationale. Examples include:

- language semantics, syntax foundations, compatibility, or versioning;
- canonical formats, public interfaces, extension boundaries, or execution
  guarantees;
- component boundaries, foundational dependencies, deployment models, or
  security architecture;
- project-wide privacy, reliability, accessibility, or release policy; and
- licensing, governance, or the open/commercial boundary.

An ADR is normally unnecessary for routine implementation, local refactoring,
editorial changes, temporary experiments, or choices that are both inexpensive
and easy to replace. When uncertain, ask in the relevant issue or pull request.
A short ADR is cheaper than repeatedly reconstructing a consequential choice.

Never put secrets, personal data, embargoed vulnerability details, or
confidential Code of Conduct information in an ADR.

## Creating a record

1. Search this index, open issues, and pull requests for an existing decision.
2. Keep the record to one decision. Split independently reversible choices.
3. Copy [the template](template.md).
4. Assign the next four-digit number and a concise kebab-case filename:
   `NNNN-short-decision-title.md`.
5. Set the status to `Proposed`, use the proposal date, and name the decider by
   governance role.
6. Explain the forces and decision criteria before evaluating realistic
   alternatives. Include material disadvantages and unresolved risks.
7. Open a pull request and link relevant issues, prototypes, evidence, and
   Roadmap items.

Numbers increase monotonically and are never reused after a proposal pull
request is opened; gaps are acceptable. If concurrent proposals select the
same number, the later one must be renumbered before merge. A number conveys
identity, not priority.

The ADR and implementation may share a pull request when the implementation is
small, reviewable, and safe to discard. Expensive or difficult-to-reverse work
should wait for a separate decision pull request to be accepted. Prototypes are
welcome as evidence when they do not create a compatibility commitment.

## Review and decision

The authority to decide follows [GOVERNANCE.md](../../GOVERNANCE.md). Anyone may
propose or review an ADR. The author should actively seek input from affected
contributors and summarize material objections; silence is not consent.

There is no fixed review period. The decider allows time proportionate to the
decision’s reach, reversibility, urgency, and participation. The Project Owner
must approve Roadmap deliverables and the durable or cross-cutting decisions
identified by Governance.

When the decider approves a proposal:

1. record the approval in the pull request;
2. change the status to `Accepted` and record the approval in the ADR;
3. update the index and any Roadmap item in the same branch;
4. verify that immediate action items are complete or explicitly deferred; and
5. merge the pull request, which makes the decision effective.

A proposal that is not selected may be closed without merge. Mark it `Rejected`
and merge it only when preserving the evaluated alternative and rejection
rationale will help future decisions.

## Statuses

| Status | Meaning |
| --- | --- |
| `Proposed` | Under review and not binding. |
| `Accepted` | Approved and effective once merged. |
| `Rejected` | Considered and explicitly not selected. |
| `Deprecated` | Retained for history but no longer recommended; no single replacement necessarily exists. |
| `Superseded` | Replaced in whole or in part by a linked later ADR. |

## Changing a decision

Accepted ADRs are historical records, not living specifications. Do not rewrite
their original context, choice, or consequences to match hindsight. Minor
corrections, clearer links, action-item updates, and status metadata may be
edited without changing the decision’s meaning.

To reverse or materially change an accepted decision:

1. create a new ADR describing the changed context and prior decision;
2. follow the normal review and approval process;
3. after acceptance, mark the old ADR `Superseded` and link both records; and
4. update current specifications and guides separately.

Use `Deprecated` when a decision is being retired without one clear
replacement. A superseding ADR need not copy the entire old record.

## Urgent and confidential decisions

Urgent safety or security work may be performed before a public ADR when delay
would increase harm. If the result creates a durable decision, document it as
soon as disclosure is safe, omitting protected details. Operational incident
handling and Code of Conduct enforcement remain governed by their confidential
processes.

## Writing guidance

- Write for a future contributor who did not witness the discussion.
- State observable constraints and distinguish evidence from assumptions.
- Use active, decisive language in the Decision section.
- Present the strongest realistic alternatives fairly.
- Record positive, negative, and neutral consequences.
- Link sources instead of copying transient discussion into the record.
- Prefer a short, complete record; add detail only when it preserves important
  evidence or trade-offs.

This process draws on Michael Nygard’s original lightweight ADR format and the
Markdown Architectural Decision Records (MADR) project, while retaining the
house style established by Choreoform’s first records.

## Sources

- [Michael Nygard, “Documenting Architecture Decisions”](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions)
- [Markdown Architectural Decision Records](https://adr.github.io/madr/)
