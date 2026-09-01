<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-02: Customer-support SLA escalation

| Attribute | Value |
| --- | --- |
| Domain | Service operations |
| Complexity | Level 2 — Coordinated |
| Primary participants | Customer, support agent, specialist, duty manager, support systems |
| Typical duration | Minutes to several days |
| Automation mix | Human investigation coordinated by event- and timer-driven software |

## Purpose and corpus role

Resolve a customer issue while making response and resolution commitments
visible and escalating work before they are breached. This case stresses
business calendars, queue ownership, multiple clocks, reassignment, and events
that may arrive late, duplicated, or out of order.

## Scope and assumptions

The case covers one support ticket from accepted intake through resolved,
cancelled, merged, or administratively closed. Contract-specific service
levels are configuration, not hard-coded facts. Customer communication and
technical remediation are represented as capabilities rather than product
implementations.

## Participants

- **Customer:** reports the problem, supplies information, and confirms or
  disputes resolution.
- **Support agent:** owns communication and first-line diagnosis.
- **Specialist:** performs domain-specific investigation or remediation.
- **Duty manager:** handles urgent escalation and exceptions.
- **Support systems:** classify, route, measure, notify, and record events.

## Trigger and preconditions

An accepted ticket records channel, customer, entitlement, reported impact,
description, and received time. Classification derives an initial priority and
the applicable service calendar and targets.

## Information and state

- Ticket identity, conversation, attachments, entitlement, priority and every
  priority change with its reason.
- Queue, assignee, response and resolution targets, clock status, escalation
  level, dependencies, and linked or duplicate tickets.
- Customer-visible status kept distinct from internal work status.

## Main success path

1. The ticket is classified and routed to an eligible queue.
2. An agent acknowledges it before the first-response target.
3. The agent resolves it or requests specialist work while retaining ownership
   of customer communication.
4. The customer accepts the resolution, or a defined confirmation period
   expires without further evidence of failure.
5. The ticket closes with service-level results and a resolution summary.

## Alternatives and failures

- Approaching deadlines send warnings and escalate assignment; a breach is
  recorded even if later work succeeds.
- A priority change recalculates future obligations according to policy but
  never erases time already elapsed or a breach already incurred.
- Waiting for customer information may pause only the clocks the contract
  permits; the request and pause reason remain visible.
- A duplicate ticket links to a controlling instance without losing messages
  that arrived on the duplicate.
- Reopening during the confirmation period resumes the existing instance;
  later recurrence may create a linked new instance.
- Notification failure is retried or surfaced without undoing assignment.

## Time, concurrency, and scale

Response, update, and resolution clocks may use different business calendars.
Investigation, specialist work, and customer communication can overlap. A new
customer message, reassignment, and a timer may race. Queues may contain
thousands of tickets and need fair, priority-aware allocation without changing
the semantics of any one instance.

## Capabilities and effects

- **Classify and route:** propose priority and eligible queue with confidence
  or a reason human review is required.
- **Request human work:** assign to an eligible participant and record
  acceptance, transfer, completion, or refusal.
- **Schedule business deadline:** calculate and observe timers against a named,
  versioned calendar.
- **Send customer communication:** record intent and delivery outcome while
  preventing accidental duplicate messages.

## Invariants and protections

- A clock pause requires an allowed reason, actor, start time, and eventual end
  or review.
- Reassignment never creates an interval in which ownership is untraceable.
- Late or duplicate transport messages do not duplicate customer-visible
  effects or reverse a later state silently.
- Service-level reporting is derived from retained facts and policy versions,
  not overwritten counters.

## Observable outcomes

- Agent and customer views consistently communicate ownership and next action
  while respecting internal-data boundaries.
- Operations can explain every target, pause, escalation, breach, and priority
  change.
- A process designer can see which work continues during a paused clock.

## Acceptance scenarios

### RP-02-A — Resolve before targets

- **Given** a correctly routed ticket with active response and resolution
  clocks,
- **when** an agent acknowledges and resolves it before both targets,
- **then** it closes without a breach and records both measured durations.

### RP-02-B — Warning and breach remain distinct

- **Given** an unacknowledged high-priority ticket,
- **when** its warning time and then response deadline pass,
- **then** escalation occurs at the warning and an immutable breach is recorded
  at the deadline even if an agent responds later.

### RP-02-C — Late message after reassignment

- **Given** a ticket transferred from one queue to another,
- **when** a delayed acceptance event from the old queue arrives,
- **then** the event is retained but does not restore the obsolete assignment.

### RP-02-D — Allowed customer wait

- **Given** a ticket awaiting information from the customer,
- **when** an authorized agent starts an allowed pause and the customer replies,
- **then** only the permitted clocks pause and resume, with the interval
  explainable from history.

## Semantic pressures exposed

- How are business-calendar timers defined, revised, paused, and explained?
- How are queues, offers, assignments, ownership, and delegation distinguished?
- How does the model resolve races and stale events deterministically?
- Can one view hide internal details without acquiring different semantics?

## Out of scope

- Workforce forecasting, customer identity verification, incident management,
  knowledge-base authoring, and the technical steps of product remediation.

## Sources and inspiration

- Synthetic composite informed by common service-management patterns rather
  than a particular service-management standard or contract.
