<!-- SPDX-FileCopyrightText: 2026 Choreoform contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# RP-09: Contract review and approval

| Attribute | Value |
| --- | --- |
| Domain | Legal and procurement operations |
| Complexity | Level 3 — Long-running |
| Primary participants | Requester, counterparty, legal reviewer, security/privacy/finance reviewers, business approver, signer |
| Typical duration | Hours to several months |
| Automation mix | Human negotiation and judgment with document and signature integrations |

## Purpose and corpus role

Move a proposed agreement from intake through specialist review, negotiation,
approval, and signature while ensuring that every decision applies to the same
document and deal facts. This case stresses immutable document versions,
conditionally parallel review, conflicting decisions, negotiated loops,
delegated authority, and edits that invalidate only affected approvals.

This synthetic case is not legal, procurement, privacy, security, or financial
advice and does not encode any organization's approval policy.

## Scope and assumptions

The case covers one agreement request from complete intake through executed,
declined, withdrawn, or expired. Drafting clause content, legal interpretation,
supplier selection, downstream obligations management, and signature law are
external. A policy engine may suggest required reviewers, but authorized people
own exceptions and substantive decisions.

## Participants

- **Requester or business owner:** supplies deal facts, owns business need, and
  coordinates changes.
- **Counterparty:** proposes and negotiates document revisions.
- **Legal reviewer:** assesses legal terms and owns legal disposition.
- **Security, privacy, finance, and other specialists:** review only when
  triggered by deal facts or policy.
- **Business approver and signer:** accept residual risk and bind the
  organization within delegated authority.

## Trigger and preconditions

A review request identifies the parties, agreement type, business owner, value,
term, data and service facts, target date, and an immutable initial document
revision. The counterparty and requested signer must be identifiable.

## Information and state

- Request and document identities, immutable revisions and comparison links,
  structured deal-fact revisions, comments, proposed changes, and negotiation
  provenance.
- Required review set and the rule revision that derived it; assignment,
  decision, conditions, expiry, and delegation per reviewer.
- Approval snapshot, signature package, signer authority, provider effect
  status, and executed artifact identity.

## Main success path

1. Intake validates the document and deal facts and determines required review
   lanes.
2. Independent specialist reviews run in parallel where possible.
3. Required changes are consolidated, negotiated, and recorded as a new
   immutable document revision.
4. Only reviews affected by changed text or facts are renewed under an
   explainable policy.
5. Business approval captures accepted conditions and residual risks against
   one fixed approval snapshot.
6. Authorized signers execute that exact revision, and the signed artifact is
   verified and distributed to permitted repositories.

## Alternatives and failures

- An incomplete request returns for correction without starting a misleading
  review clock.
- A specialist approves with conditions, rejects, requests changes, or declares
  the review not applicable; conditions remain machine-visible obligations.
- Conflicting reviewer requests are escalated for an attributable resolution,
  not resolved by arrival order.
- A counterparty revision triggers a structured impact assessment and
  invalidates affected decisions; unchanged independent approvals may remain.
- Delegation is accepted only if the delegate has sufficient authority at the
  decision time and scope.
- A signature request that expires, is declined, or returns an unknown outcome
  is reconciled before a new package is issued.
- Withdrawal stops future negotiation and signature but preserves the decision
  record and any confidentiality or retention obligations.

## Time, concurrency, and scale

Review lanes can proceed concurrently but converge on a common revision.
Target dates generate reminders and escalation but cannot manufacture approval.
Negotiation may loop many times and span policy or personnel changes. Reviewers
have finite capacity; assignment and delegation affect scheduling without
changing required authority.

## Capabilities and effects

- **Classify required review:** derive an explainable review set from versioned
  facts and policy.
- **Request specialist decision:** bind the assignment and outcome to fixed
  document and fact revisions.
- **Compare document revisions:** identify changes and provenance without
  claiming legal significance automatically.
- **Request signature:** create one controlled package for the approved revision
  and reconcile provider status.
- **Store executed agreement:** preserve integrity, access controls, and a
  durable reference.

## Invariants and protections

- Every approval and signature applies to identified document and deal-fact
  revisions; a material edit cannot inherit them silently.
- Signature cannot be requested until all required decisions and conditions for
  the approval snapshot are satisfied or explicitly accepted by authority.
- A reviewer cannot approve outside current authority through assignment or
  delegation alone.
- Confidential drafts and comments follow participant- and purpose-specific
  access rules.
- The executed artifact is verified against the approved revision.

## Observable outcomes

- Participants can see required reviews, current document revision, conditions,
  conflicts, blockers, expiry, and next owner without exposing all comments.
- History explains why each review was required, retained, invalidated, waived,
  or repeated.
- Operations can distinguish signature requested, viewed, signed, declined,
  expired, provider-unknown, verified, and stored.

## Acceptance scenarios

### RP-09-A — Parallel reviews converge

- **Given** one document revision requiring legal, security, and finance review,
- **when** all three approve and an authorized business approver accepts the
  snapshot,
- **then** only that revision becomes eligible for signature.

### RP-09-B — Revision invalidates affected approval

- **Given** legal and security approved revision 3,
- **when** revision 4 changes only a security-relevant data term,
- **then** security approval is renewed and the retained legal approval remains
  visibly tied to an explainable impact decision.

### RP-09-C — Conflicting requested changes

- **Given** two reviewers request mutually incompatible terms,
- **when** both decisions are recorded,
- **then** neither silently wins and the agreement waits for an authorized,
  attributable resolution.

### RP-09-D — Signature provider times out

- **Given** an approved signature package has been sent,
- **when** provider status becomes unknown after a timeout,
- **then** the process reconciles the existing package before creating another
  legally consequential request.

## Semantic pressures exposed

- How do decisions bind to immutable artifacts and structured facts together?
- How are conditional parallel branches and partial re-review represented?
- How can textual and visual forms reveal change impact and conflicting
  conditions without duplicating document content?
- How are delegated authority and expiring approval evaluated over long runs?

## Out of scope

- Legal interpretation, clause language, supplier selection, electronic-signature
  validity, entity verification, records schedules, and post-signature duties.

## Sources and inspiration

- Synthetic composite based on common document-review pressures; no contract
  template, legal rule, or organization policy is incorporated.
