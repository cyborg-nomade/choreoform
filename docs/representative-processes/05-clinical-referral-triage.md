<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-05: Clinical referral triage

| Attribute | Value |
| --- | --- |
| Domain | Healthcare coordination |
| Complexity | Level 4 — Adaptive |
| Primary participants | Referrer, patient, triage clinician, booking team, receiving service |
| Typical duration | Minutes for urgent escalation to several weeks for routine coordination |
| Automation mix | Clinician-controlled decisions supported by coordination software |

## Purpose and corpus role

Route a referral to a safe, appropriate next action while handling incomplete
information and time-sensitive concern. This case exists to test human judgment,
confidentiality, prioritization, accountable overrides, safety escalation, and
the difference between coordination state and clinical truth.

This is a fictional requirements example, not a clinical pathway, triage
protocol, medical device specification, or medical advice. A real process must
be designed and governed by qualified professionals under applicable law and
local policy.

## Scope and assumptions

The case begins when a receiving service records a referral and ends with an
accepted routing disposition, return to the referrer, documented transfer, or
explicit closure. Diagnosis, treatment, emergency instructions, clinical
thresholds, and jurisdictional access rules are outside the model. Software may
surface facts or deadlines but does not replace the authorized clinician.

## Participants

- **Referrer:** supplies the referral, evidence, and requested service.
- **Patient or authorized representative:** supplies permitted information and
  preferences and receives appropriate communications.
- **Triage clinician:** evaluates the referral and owns clinical priority and
  routing decisions.
- **Booking team:** offers and coordinates an approved next step.
- **Receiving or alternate service:** accepts a transfer of responsibility.

## Trigger and preconditions

A referral-received event identifies the patient through an approved reference,
the referrer, requested service, reason, available evidence, consent or other
lawful basis as required, and receipt time. Identity uncertainty or an immediate
safety concern is handled visibly, not normalized away.

## Information and state

- Referral and patient-reference identity, provenance and version of documents,
  access restrictions, contact preferences, and relevant communication history.
- Administrative completeness kept distinct from clinical sufficiency.
- Current owner, triage priority and rationale, requested information,
  deadlines, routing decision, transfer acceptance, and unresolved safety flag.

## Main success path

1. Intake checks identity, permitted scope, and administrative completeness.
2. An authorized clinician reviews the referral and records priority, rationale,
   and a routing disposition.
3. If more information is needed safely, a bounded request is sent and tracked.
4. The chosen service explicitly accepts responsibility or the case returns for
   another authorized decision.
5. The patient and referrer receive permitted next-step information, and the
   coordination instance closes with an attributable disposition.

## Alternatives and failures

- A safety concern triggers immediate human escalation while normal intake work
  is suspended or continues only where safe; the example does not define the
  clinical response.
- Missing administrative information may be corrected without erasing the
  original referral or its receipt time.
- A request for clinical information has an owner and deadline; non-response
  returns to clinician judgment rather than automatic rejection.
- The requested service may redirect or decline, but responsibility does not
  disappear between services: transfer requires explicit acceptance.
- New evidence can cause re-triage; the new decision is tied to a new evidence
  set and does not rewrite the earlier rationale.
- Incorrect recipient or failed communication is contained, reported, and
  corrected according to local privacy and safety procedures.

## Time, concurrency, and scale

Deadlines depend on clinician-set priority and local calendars and may change
after new evidence. Information gathering and safe administrative preparation
may overlap, but routing cannot outrun required judgment. Urgent work competes
with routine queues, so allocation must surface priority without starving or
silently losing older referrals.

## Capabilities and effects

- **Request clinical decision:** present the authorized clinician with the
  evidence revision, relevant history, and deadline.
- **Request information:** send a minimum-necessary, attributable request and
  correlate the response.
- **Transfer responsibility:** request and record explicit acceptance by a
  service; sending a message alone is insufficient.
- **Communicate next step:** disclose only permitted information to a verified
  recipient and record delivery status separately from understanding.

## Invariants and protections

- Automation does not invent, upgrade, downgrade, or override a clinical
  decision in this example.
- Every priority and routing decision identifies its clinician, time, rationale,
  and evidence revision.
- No referral is closed as transferred until the receiving responsibility is
  accepted under the applicable policy.
- Access and disclosure are purpose-limited, and especially restricted
  information is not copied into general task metadata.
- A timer cannot silently convert missing information into a clinical outcome.

## Observable outcomes

- Authorized participants can distinguish administrative wait, clinician wait,
  information wait, transfer pending, accepted routing, returned referral, and
  unresolved safety escalation.
- History explains which evidence and priority applied at each decision.
- Operational measurement can avoid exposing clinical content and can separate
  time under different owners.

## Acceptance scenarios

### RP-05-A — Accepted routine routing

- **Given** a complete referral with no recorded urgent concern,
- **when** an authorized clinician records a routine disposition and the
  receiving service accepts it,
- **then** the next step is communicated and the instance closes with the
  evidence revision and responsibility transfer recorded.

### RP-05-B — Missing information times out

- **Given** a clinician requests additional information with a review date,
- **when** no response arrives by that date,
- **then** the case returns to an authorized clinician with its original receipt
  time intact rather than being automatically rejected.

### RP-05-C — New evidence changes priority

- **Given** a referral already assigned a priority,
- **when** verified new evidence arrives,
- **then** re-triage records a new evidence revision and decision while
  retaining the former decision and its effects.

### RP-05-D — Transfer message is not acceptance

- **Given** a clinician routes the referral to another service,
- **when** the transfer message is delivered but no acceptance is received,
- **then** responsibility remains visibly pending and the case cannot close as
  transferred.

## Semantic pressures exposed

- How are expert decisions represented without making the engine their author?
- How can views minimize sensitive data while preserving semantic identity?
- How are changing evidence, priorities, and deadlines related historically?
- How does a notation make responsibility gaps and safety escalations hard to
  overlook?

## Out of scope

- Diagnosis, treatment, clinical thresholds, emergency response instructions,
  capacity allocation rules, billing, and the full patient record.

## Sources and inspiration

- [NHS England, National elective access policy](https://www.england.nhs.uk/long-read/national-elective-access-policy/)
- [NHS England, Advice and Guidance](https://www.england.nhs.uk/elective-care/best-practice-solutions/advice-and-guidance/)
- These sources inform coordination pressures only; the synthetic case does not
  implement or claim conformance with NHS policy.
