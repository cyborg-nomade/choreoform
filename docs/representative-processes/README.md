<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Representative process corpus

**Status:** Adopted and non-normative<br>
**Decision record:** [ADR-0006](../decisions/0006-representative-process-corpus.md)

This corpus gives Choreoform design proposals a stable set of realistic
problems to explain. Its ten processes range from a short internal approval to
long-running, multi-party, and safety-sensitive work. They are synthetic
composites: each preserves useful operational tensions without describing a
particular organization or pretending to be a deployable process.

The cases describe evidence, not language features. Terms such as *wait*,
*parallel*, *retry*, or *compensate* name behavior that must be accounted for;
they do not require constructs with those names. See the
[working glossary](../../GLOSSARY.md) for the conceptual vocabulary currently
used by the project.

## Boundaries

The corpus is not Choreoform syntax, a normative specification, a conformance
suite, or operational guidance. The cases omit many jurisdiction-, policy-,
organization-, and product-specific requirements. They must not be copied into
production without domain research, risk assessment, and qualified review.

No example contains real personal, commercial, security, or patient data.
Example identifiers and situations are fictitious. Sources listed in cases
were used to check design pressures; their inclusion does not make a case a
faithful implementation of the source.

## Complexity scale

The level is a navigation aid, not a statement of business importance or risk.

| Level | Typical shape |
| --- | --- |
| 1 — Bounded | Mostly sequential, one organization, limited branching, and a short lifetime |
| 2 — Coordinated | Meaningful branching, a timer or integration, or limited concurrent work |
| 3 — Long-running | Multiple systems or roles, parallel work, durable waits, and explicit recovery |
| 4 — Adaptive | Dynamic scope or response, competing deadlines, and safety-, security-, or compliance-sensitive judgment |

## Adopted cases

| ID | Process | Domain | Level | Why it is in the corpus |
| --- | --- | --- | --- | --- |
| RP-01 | [Expense reimbursement](01-expense-reimbursement.md) | Small-business administration | 1 | Policy decisions, correction loops, and accountable human approval |
| RP-02 | [Customer-support SLA escalation](02-customer-support-sla-escalation.md) | Service operations | 2 | Queues, timers, reassignment, and late or duplicate events |
| RP-03 | [E-commerce order fulfillment](03-ecommerce-order-fulfillment.md) | Commerce and logistics | 3 | Parallel reservations, partial failure, cancellation, and compensation |
| RP-04 | [Employee onboarding](04-employee-onboarding.md) | People and IT operations | 3 | Long-running human/software work, dependencies, sensitive data, and change |
| RP-05 | [Clinical referral triage](05-clinical-referral-triage.md) | Healthcare coordination | 4 | Human judgment, priority, missing information, privacy, and safety escalation |
| RP-06 | [Manufacturing quality hold and release](06-manufacturing-quality-hold-release.md) | Manufacturing quality | 4 | Physical lots, evidence, quarantine, related scope, and independent sign-off |
| RP-07 | [Data-pipeline backfill](07-data-pipeline-backfill.md) | Data engineering | 3 | High-volume parallelism, checkpoints, rate limits, pause/resume, and idempotency |
| RP-08 | [Cybersecurity incident response](08-cybersecurity-incident-response.md) | Security operations | 4 | Event-driven, dynamically scoped work with containment, evidence, and recovery |
| RP-09 | [Contract review and approval](09-contract-review-approval.md) | Legal and procurement operations | 3 | Document versions, parallel reviewers, conflicting decisions, and invalidated approvals |
| RP-10 | [Subscription renewal and payment recovery](10-subscription-renewal-payment-recovery.md) | Billing operations | 2 | Scheduled attempts, asynchronous payment outcomes, retries, and reinstatement |

## Coverage matrix

`●` means the pressure is central to a case; `○` means it appears but is not
the case's main reason for inclusion. Empty cells are deliberately not forced
into every example.

| ID | Human work | Parallel work | Time / timers | External effects | Retry / recovery | Cancel / compensate | Sensitive / controlled data | Definition or scope change |
| --- | :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| RP-01 | ● |  | ○ | ○ | ○ | ● | ○ | ○ |
| RP-02 | ● | ○ | ● | ● | ● | ○ | ○ | ● |
| RP-03 | ○ | ● | ○ | ● | ● | ● | ○ | ○ |
| RP-04 | ● | ● | ● | ● | ○ | ● | ● | ● |
| RP-05 | ● | ○ | ● | ● | ○ | ○ | ● | ● |
| RP-06 | ● | ● | ● | ● | ● | ● | ● | ● |
| RP-07 | ○ | ● | ○ | ● | ● | ● | ● | ● |
| RP-08 | ● | ● | ● | ● | ● | ● | ● | ● |
| RP-09 | ● | ● | ● | ● | ○ | ● | ● | ● |
| RP-10 | ○ | ○ | ● | ● | ● | ● | ● | ○ |

Together the cases also cover sequence, exclusive and inclusive decisions,
loops, fan-out/fan-in, races between work and deadlines, partial completion,
manual intervention, resource assignment, messages, and explainable history.
This is a coverage inventory, not yet the scoring model for design proposals;
the next Roadmap deliverable defines that evaluation method.

## How to read a case

Every case separates the same kinds of evidence:

- **main success path** establishes a recognizable end-to-end story;
- **alternatives and failures** prevent the happy path from defining the model;
- **time, concurrency, and scale** expose execution pressures hidden by a
  flowchart;
- **capabilities and effects** identify work at trust and integration
  boundaries without selecting adapters;
- **invariants and protections** say what must remain true;
- **observable outcomes** identify what people and systems must be able to
  inspect;
- **acceptance scenarios** provide traceable examples for later designs and
  tests; and
- **semantic pressures** summarize questions a proposal must answer without
  dictating the answer.

Use [the case template](template.md) when proposing a supplementary process.
Prefer a pressure that is absent from the matrix over another domain-flavored
version of an existing happy path.

## Evolution

Clarifications, additional acceptance scenarios, better sources, and
supplementary cases use normal review. A proposal should say which gap it
fills, what it duplicates, and whether it changes the matrix.

Replacing or removing an adopted case, materially narrowing the coverage
model, or making a case normative requires an ADR under
[ADR-0006](../decisions/0006-representative-process-corpus.md). Future formal
specifications should reference scenario IDs when they rely on corpus evidence.
