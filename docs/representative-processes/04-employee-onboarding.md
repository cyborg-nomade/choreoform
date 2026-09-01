<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-04: Employee onboarding

| Attribute | Value |
| --- | --- |
| Domain | People and IT operations |
| Complexity | Level 3 — Long-running |
| Primary participants | New starter, hiring manager, people operations, IT, facilities, identity and payroll systems |
| Typical duration | Several days to several months |
| Automation mix | Parallel human and software work with physical delivery |

## Purpose and corpus role

Prepare a new starter to work safely and productively on an agreed date while
granting only appropriate access. This case stresses long lead times,
dependencies among parallel preparations, sensitive information, delegation,
physical assets, and changes or cancellation after work has begun.

## Scope and assumptions

The case begins after an authorized hiring decision and covers pre-start setup
through the end of an initial onboarding checklist. Recruitment and later
offboarding are separate processes. Required checks, records, contracts, and
access rules depend on employer, role, location, and law; this synthetic case
is not employment, immigration, tax, privacy, or security advice.

## Participants

- **New starter:** supplies permitted information, completes required actions,
  and acknowledges policies.
- **Hiring manager:** confirms role, start date, work location, equipment, and
  justified access.
- **People operations:** coordinates employment records and readiness.
- **IT and facilities:** provision identity, access, devices, workspace, and
  physical credentials.
- **Identity, payroll, learning, and delivery systems:** perform bounded
  external capabilities and report outcomes.

## Trigger and preconditions

An authorized onboarding request identifies the person, role revision, manager,
legal employer, work location, start date, and approved source record. Minimum
data and authority to begin pre-start processing must be present.

## Information and state

- Person and request identities kept distinct; role, location, manager, start
  date, and required-access revisions.
- Sensitive identity, payroll, accommodation, and screening information with
  purpose-specific visibility rather than one shared case payload.
- Per-workstream readiness, dependencies, owners, assets, credentials, pending
  acknowledgements, and revocation obligations.

## Main success path

1. People operations validates the request and gathers only required data.
2. In parallel, IT prepares identity and equipment, facilities prepares access
   or workspace, payroll prepares employment setup, and the manager prepares a
   role plan.
3. Dependent access waits for its prerequisites and required approvals.
4. Near the start date, readiness is assessed and exceptions are escalated.
5. On or after verified start, credentials are activated and equipment is
   received.
6. Required acknowledgements, training, and manager check-in complete the
   onboarding instance.

## Alternatives and failures

- Missing or inconsistent information returns only the relevant workstream for
  correction while independent safe work may continue.
- A start-date change reschedules date-dependent work without recreating
  completed assets or activating access early.
- A role or location change invalidates affected approvals and creates explicit
  adjust, revoke, or replace work.
- A cancelled hire prevents activation and compensates completed provisioning,
  including device recall and account disablement.
- Equipment delivery failure can trigger a temporary approved alternative.
- A prerequisite that cannot finish in time produces a visible readiness
  exception for an authorized decision, not an automatic unsafe waiver.

## Time, concurrency, and scale

Preparation fans out across teams with different calendars and lead times.
Some branches can finish in any order; activation is gated by date, verified
status, and policy. Deadlines are recalculated after an authorized start-date
change. Seasonal hiring may create many instances sharing scarce device stock,
trainers, or facilities capacity.

## Capabilities and effects

- **Collect restricted information:** request a purpose-limited data set and
  expose it only to authorized consumers.
- **Provision identity and access:** create, modify, activate, disable, or
  reconcile accounts under an approved role revision.
- **Allocate and deliver asset:** reserve a physical item and record custody.
- **Assign human work or learning:** establish ownership, due date, and
  attributable completion.
- **Assess readiness:** derive an explainable view without silently treating
  unknown work as complete.

## Invariants and protections

- Access is justified by the effective role revision and is not activated
  before all mandatory gates.
- A cancelled or materially changed request leaves no forgotten account,
  credential, shipment, or asset obligation.
- Sensitive fields are minimized and segmented; participants see only what
  their task requires.
- Manual overrides identify the actor, authority, reason, scope, and expiry.
- An edit never silently changes obligations for work already performed.

## Observable outcomes

- The starter and manager can see relevant readiness without seeing protected
  internal details.
- Coordinators can distinguish not started, active, blocked, waived, failed,
  compensated, and complete for each workstream.
- Audit history relates every account and asset effect to a request revision
  and authorization.

## Acceptance scenarios

### RP-04-A — Ready on start date

- **Given** an authorized request whose independent preparations complete,
- **when** the verified start date arrives,
- **then** gated credentials activate, custody is recorded, and onboarding
  proceeds without exposing restricted data across workstreams.

### RP-04-B — Start date moves

- **Given** equipment is delivered but credentials are scheduled for a future
  start,
- **when** an authorized change moves the date later,
- **then** completed safe work remains complete, activation and reminders move,
  and the reason and old date remain in history.

### RP-04-C — Role change invalidates access

- **Given** access was approved for one role revision,
- **when** the manager submits a materially different role before start,
- **then** affected approval and provisioning are no longer sufficient and
  explicit adjustment or revocation work is tracked.

### RP-04-D — Cancellation compensates parallel work

- **Given** an identity, device shipment, and facility credential are at
  different completion points,
- **when** the hire is cancelled,
- **then** activation is prevented and each completed or uncertain effect has
  an independently visible compensation outcome.

## Semantic pressures exposed

- How does a definition describe parallel work with different data visibility?
- How do changes revise only affected obligations in a running instance?
- How are physical assets and scarce resources represented without embedding
  an inventory system?
- How are gates, waivers, and expiring overrides made visually obvious?

## Out of scope

- Candidate selection, employment-contract negotiation, detailed statutory
  checks, performance management, and full offboarding.

## Sources and inspiration

- [UK Government, Simplifying the onboarding process for new starters](https://www.gov.uk/government/publications/simplifying-the-onboarding-process-for-new-starters-communication-resources)
- The case is a synthetic coordination example and does not implement the
  employment requirements of any jurisdiction.
