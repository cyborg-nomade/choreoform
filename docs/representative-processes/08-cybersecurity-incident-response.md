<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-08: Cybersecurity incident response

| Attribute | Value |
| --- | --- |
| Domain | Security operations |
| Complexity | Level 4 — Adaptive |
| Primary participants | Incident commander, analysts, service owners, communications and legal advisers, security tools |
| Typical duration | Minutes to months, including follow-up |
| Automation mix | Dynamic human coordination with guarded automated actions |

## Purpose and corpus role

Coordinate investigation, containment, recovery, and follow-up for a suspected
cybersecurity incident while facts and scope change. This case stresses dynamic
work creation, parallel teams, severity changes, guarded high-impact effects,
evidence handling, confidentiality, decision logs, and recovery that overlaps
ongoing detection.

This is a fictional language-design case, not an incident-response plan or
security advice. Organizations must develop and test procedures appropriate to
their systems, threats, obligations, and authorities.

## Scope and assumptions

The case begins when a report is accepted for assessment and ends when active
response is closed and follow-up obligations are transferred. It does not
prescribe technical commands, severity thresholds, disclosure duties, or
forensic methods. NIST guidance informs the lifecycle pressures, not a claim of
conformance.

## Participants

- **Incident commander:** owns coordination, objectives, cadence, and major
  response decisions.
- **Security analysts:** investigate signals, preserve evidence, and propose or
  perform authorized actions.
- **Service and business owners:** assess impact and execute service-specific
  containment and recovery.
- **Communications, privacy, legal, and leadership advisers:** decide bounded
  notifications and obligations within their authority.
- **Security and operations tools:** provide telemetry and execute explicitly
  authorized effects.

## Trigger and preconditions

An accepted report records source, receipt time, affected identifiers known so
far, observed facts, reporter, confidence, and handling classification. A
suspected incident may be opened before its nature or scope is known.

## Information and state

- Incident identity; hypotheses kept distinct from observations; affected-asset
  and stakeholder scope revisions; severity and rationale; objectives and
  decision log.
- Dynamically created work items, owner and status, dependencies, deadlines,
  evidence references and custody, communications, approvals, and effect
  confirmations.
- Service recovery and monitoring state per asset or capability, not one global
  resolved flag.

## Main success path

1. An authorized responder assesses the report, establishes initial severity,
   and appoints coordination roles.
2. Investigation, evidence preservation, impact analysis, and immediate safe
   mitigations proceed in parallel.
3. The commander approves objectives and bounded containment actions based on
   current facts and risk.
4. Scope, severity, stakeholders, and tasks evolve as evidence arrives; every
   material change records rationale.
5. Affected services recover through validated steps while heightened
   monitoring continues.
6. Active response closes only when exit criteria are met and remediation,
   notification, and learning actions have accountable destinations.

## Alternatives and failures

- Assessment concludes the report is not an incident; retained rationale and
  any already-created obligations are resolved rather than deleted.
- New evidence expands scope across assets or organizations and spawns work
  from a repeatable playbook fragment or an ad hoc authorized task.
- A proposed containment effect could cause greater harm and requires approval,
  simulation, staged execution, or rejection.
- Tool output is delayed, duplicated, contradictory, or compromised; confidence
  and provenance remain visible.
- Loss of the primary coordination system requires a documented alternate
  channel and later reconciliation without fabricating event order.
- Recurrence during recovery can reopen an objective, increase severity, or
  create a linked incident according to an explicit decision.

## Time, concurrency, and scale

Multiple clocks coexist: response cadence, credential or evidence lifetime,
service objectives, contractual notice, and jurisdiction-dependent obligations.
Teams work in parallel, and some containment must race an adversary or ongoing
damage. The graph of affected assets and tasks is discovered incrementally and
may become large. Shift changes require explicit handover of ownership and
context.

## Capabilities and effects

- **Acquire observation:** query telemetry with time range, provenance, and
  confidence rather than treating tool output as unquestionable truth.
- **Preserve evidence:** store a protected reference with custody and access
  history.
- **Execute containment or recovery action:** require scoped authorization,
  stable effect identity, confirmation, and a rollback or contingency plan
  where feasible.
- **Assign dynamic work:** create attributable tasks linked to an objective and
  the facts or decision that justified them.
- **Send restricted communication:** control audience, approved content revision,
  release authority, and delivery evidence.

## Invariants and protections

- High-impact effects cannot occur merely because an untrusted event or model
  recommendation requested them.
- Observations, hypotheses, decisions, and actions remain distinguishable and
  attributable.
- Sensitive incident data follows need-to-know access and must not leak through
  general labels, notifications, or visual summaries.
- Scope or severity changes never erase the state under which an earlier action
  was authorized.
- Closing active response cannot silently discard unfinished remediation,
  disclosure, evidence-retention, or review obligations.

## Observable outcomes

- Responders can see current objectives, ownership, blocked work, recent
  decisions, affected scope, and the confidence of key facts.
- Leadership can receive a bounded summary that shares meaning but not every
  sensitive detail.
- Reviewers can reconstruct why each consequential action was proposed,
  authorized, attempted, confirmed, reversed, or left uncertain.

## Acceptance scenarios

### RP-08-A — Coordinated containment and recovery

- **Given** a confirmed incident with identified affected services,
- **when** authorized parallel containment completes and service owners validate
  staged recovery,
- **then** active response closes only after exit criteria and remaining
  remediation have explicit owners.

### RP-08-B — Scope expands dynamically

- **Given** an investigation scoped to one account,
- **when** verified evidence identifies related assets,
- **then** a new scope revision creates or links appropriate work without
  invalidating unrelated completed evidence gathering.

### RP-08-C — Dangerous automated action is gated

- **Given** a tool recommends isolating a critical service,
- **when** the recommendation lacks the required human authorization,
- **then** no isolation effect occurs and the proposal remains available for an
  accountable decision.

### RP-08-D — Coordination outage and reconciliation

- **Given** responders use an approved alternate channel during a system outage,
- **when** the primary system returns,
- **then** externally recorded decisions and actions are imported with their
  known timestamps and provenance without inventing a total order.

## Semantic pressures exposed

- How can authorized participants add work and scope at runtime without making
  the definition meaningless?
- How are incomplete knowledge, confidence, and conflicting facts represented?
- How can different audiences receive semantically consistent but
  confidentiality-appropriate views?
- How are high-impact capabilities guarded independently of control flow?

## Out of scope

- Technical detection rules, forensic procedures, containment commands,
  severity matrices, notification law, public communications content, and
  business-continuity planning.

## Sources and inspiration

- [NIST SP 800-61 Rev. 3, Incident Response Recommendations and Considerations for Cybersecurity Risk Management](https://csrc.nist.gov/pubs/sp/800/61/r3/final)
- The case reflects preparation, detection, response, recovery, and continuous
  improvement pressures without implementing an organization-specific plan.
