<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# ADR-0004: Adopt a lightweight, repository-native ADR process

**Status:** Accepted<br>
**Date:** 2026-08-31<br>
**Decider:** Project owner

## Context

Choreoform’s first three durable decisions use architecture decision records,
and Governance requires ADRs for changes to semantics, compatibility,
architecture, licensing, governance, and the open/commercial boundary. The
repository does not yet define when a record is required, how it moves from
proposal to acceptance, how numbers are allocated, or how a decision is later
changed.

An undefined process makes decisions depend on oral context and maintainer
memory. An overly elaborate process would create ceremony before Choreoform has
an implementation or a broad maintainer community. The project needs enough
structure to preserve rationale and invite review without turning every code
choice into a design document.

## Decision criteria

The process should:

1. preserve the reasons, alternatives, and consequences of durable choices;
2. fit the branch, pull-request, and approval rules in Governance;
3. tell contributors when an ADR is and is not warranted;
4. keep accepted history stable while allowing decisions to evolve;
5. work with plain Markdown and GitHub, without mandatory tooling;
6. handle parallel proposals, rejected options, and urgent confidential work;
   and
7. remain proportionate for a small, early-stage project.

## Decision

Adopt the process in [the decision-record guide](README.md) and the repository’s
[ADR template](template.md).

Choreoform ADRs will:

- live in `docs/decisions/` as Markdown alongside the system they govern;
- capture one architecturally or institutionally significant decision;
- use monotonically increasing four-digit identifiers that are not reused
  after a proposal pull request is opened;
- use the statuses `Proposed`, `Accepted`, `Rejected`, `Deprecated`, and
  `Superseded`;
- identify their date and decider by governance role;
- include context, decision criteria, the decision, realistic alternatives,
  consequences, confirmation, and action items;
- receive public review and approval under `GOVERNANCE.md`; and
- preserve accepted rationale, using a later linked ADR for material changes.

The record becomes effective when its accepted version is merged. The decider
records approval before merge, after which the branch updates the ADR status,
index, Roadmap, and immediate action items.

The ADR and implementation may share a pull request only when the implementation
is small, reviewable, and safe to discard. Expensive or difficult-to-reverse
implementation waits for an accepted decision. Prototypes may accompany a
proposal as evidence without creating compatibility promises.

Existing ADR-0001 through ADR-0003 already contain the essential information
required by this process and are accepted without retroactive reformatting.

## Options considered

| Option | Durable rationale | Contributor overhead | Tool dependency | Fit now | Outcome |
| --- | --- | --- | --- | --- | --- |
| **Repository-native lightweight ADRs** | Strong | Low–medium | None | High | Adopt |
| Continue the implicit house style | Uneven | Low | None | Medium | Reject: lifecycle and change rules remain unclear |
| Full RFC process for consequential changes | Very strong | High | None | Low | Reject for current scale |
| External decision service or ADR tool | Varies | Medium | Required | Low | Reject: separates decisions from governed source |

The chosen process combines Nygard’s small, immutable records with MADR’s
explicit consideration of options and outcomes. Choreoform adds decision
criteria, confirmation, and acceptance action items because its early choices
need traceable evidence and explicit Project Owner approval.

A full request-for-comments process could provide fixed review windows,
stakeholder sign-offs, and richer discussion artifacts. Those controls are
premature with one Project Owner and no appointed maintainers. They can be
added later if participation or risk makes the lightweight process inadequate.

Dedicated tooling could allocate numbers, validate metadata, generate an
index, and visualize supersession. Plain Markdown is sufficient now; automation
can be introduced after repeated manual errors demonstrate a need.

## Consequences

- Contributors gain one place to learn whether and how to propose a durable
  decision.
- Decision authority remains in Governance rather than being duplicated in the
  ADR process.
- Accepted records remain trustworthy historical evidence instead of silently
  changing with current design.
- The index and status metadata require manual maintenance initially.
- Some pull requests will need an ADR before implementation, adding deliberate
  lead time to costly decisions.
- Gaps in ADR numbering and merged rejected records are permitted when they
  preserve useful history.
- Specifications and guides must still be maintained; ADRs do not replace
  current documentation.

## Confirmation

The decision is implemented when:

- `docs/decisions/README.md` indexes all records and explains their lifecycle;
- `docs/decisions/template.md` is available for new proposals;
- Governance and Contributing link to the process instead of describing it as
  forthcoming; and
- future durable decisions use the documented status and supersession rules.

Automation is not required for initial conformance. The project may later add a
check for unique identifiers, valid statuses, index coverage, and links.

## Sources

- [Michael Nygard, “Documenting Architecture Decisions”](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions)
- [Markdown Architectural Decision Records 4.0](https://adr.github.io/madr/)
- [MADR minimal template](https://github.com/adr/madr/blob/4.0.0/template/adr-template-minimal.md)

## Acceptance and action items

The Project Owner approved this ADR on 2026-08-31. The process becomes
effective when pull request #5 is merged.

1. [x] Obtain Project Owner approval.
2. [x] Change this ADR’s status to Accepted and record the approval.
3. [x] Add the ADR guide, index, and template.
4. [x] Update Governance and Contributing to reference the process.
5. [x] Mark the Roadmap deliverable complete after approval.
6. [ ] Consider automated validation after repeated manual errors or when
   continuous integration is introduced.
